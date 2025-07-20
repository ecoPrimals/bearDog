//! Android device management and communication

use crate::error::DeployError;
use anyhow::Result;
use console::style;
use serde::{Deserialize, Serialize};
use std::{process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    time::sleep,
};
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub serial: String,
    pub manufacturer: String,
    pub model: String,
    pub android_version: String,
    pub api_level: u32,
    pub is_pixel: bool,
    pub has_grapheneos: bool,
    pub has_strongbox: bool,
}

pub struct DeviceManager;

impl DeviceManager {
    pub fn new() -> Self {
        Self
    }

    /// Check for connected Android device and get its info
    pub async fn check_device(&self) -> Result<DeviceInfo> {
        info!("📱 Checking for connected Android devices...");

        // Get list of connected devices
        let devices = self.get_connected_devices().await?;

        if devices.is_empty() {
            return Err(DeployError::DeviceNotFound.into());
        }

        if devices.len() > 1 {
            warn!("Multiple devices connected: {:?}", devices);
            return Err(DeployError::MultipleDevices.into());
        }

        let serial = &devices[0];
        let device_info = self.get_device_info(serial).await?;

        info!(
            "✅ Device detected: {} {} ({})",
            device_info.manufacturer, device_info.model, device_info.serial
        );

        Ok(device_info)
    }

    async fn get_connected_devices(&self) -> Result<Vec<String>> {
        let output = Command::new("adb")
            .args(["devices"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                DeployError::device_command_failed(format!("Failed to run adb devices: {e}"))
            })?;

        if !output.status.success() {
            return Err(DeployError::device_command_failed("adb devices command failed").into());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let devices: Vec<String> = output_str
            .lines()
            .skip(1) // Skip header line
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 && parts[1] == "device" {
                    Some(parts[0].to_string())
                } else {
                    None
                }
            })
            .collect();

        debug!("Connected devices: {:?}", devices);
        Ok(devices)
    }

    async fn get_device_info(&self, serial: &str) -> Result<DeviceInfo> {
        // Get device properties
        let manufacturer = self
            .get_device_property(serial, "ro.product.manufacturer")
            .await?;
        let model = self.get_device_property(serial, "ro.product.model").await?;
        let android_version = self
            .get_device_property(serial, "ro.build.version.release")
            .await?;
        let api_level_str = self
            .get_device_property(serial, "ro.build.version.sdk")
            .await?;
        let build_fingerprint = self
            .get_device_property(serial, "ro.build.fingerprint")
            .await?;

        let api_level = api_level_str.parse::<u32>().unwrap_or(0);
        let is_pixel = manufacturer.to_lowercase() == "google";
        let has_grapheneos = build_fingerprint.contains("GrapheneOS");

        // Check for StrongBox support (requires API 28+)
        let has_strongbox = api_level >= 28 && is_pixel;

        Ok(DeviceInfo {
            serial: serial.to_string(),
            manufacturer,
            model,
            android_version,
            api_level,
            is_pixel,
            has_grapheneos,
            has_strongbox,
        })
    }

    async fn get_device_property(&self, serial: &str, property: &str) -> Result<String> {
        let output = Command::new("adb")
            .args(["-s", serial, "shell", "getprop", property])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                DeployError::device_command_failed(format!(
                    "Failed to get property {property}: {e}"
                ))
            })?;

        if !output.status.success() {
            return Err(DeployError::device_command_failed(format!(
                "Failed to get property {property}"
            ))
            .into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Deploy the built application to the device
    pub async fn deploy_app(&self, build_type: &str) -> Result<()> {
        info!("📲 Deploying BearDog to device...");

        let device = self.check_device().await?;

        // Determine library path
        let lib_name = "libpixel8_native_app.so";
        let local_path = format!("target/aarch64-linux-android/{build_type}/examples/{lib_name}");

        // Check if binary exists
        if !std::path::Path::new(&local_path).exists() {
            return Err(DeployError::deployment_failed(format!(
                "Binary not found: {local_path}. Run build first."
            ))
            .into());
        }

        // Push to device
        info!("📤 Pushing binary to device...");
        let output = Command::new("adb")
            .args([
                "-s",
                &device.serial,
                "push",
                &local_path,
                "/data/local/tmp/",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DeployError::deployment_failed(format!("Failed to push binary: {e}")))?;

        if !output.status.success() {
            return Err(DeployError::deployment_failed(format!(
                "Failed to push binary: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
            .into());
        }

        // Set executable permissions
        info!("🔧 Setting executable permissions...");
        let output = Command::new("adb")
            .args([
                "-s",
                &device.serial,
                "shell",
                "chmod",
                "755",
                &format!("/data/local/tmp/{lib_name}"),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                DeployError::deployment_failed(format!("Failed to set permissions: {e}"))
            })?;

        if !output.status.success() {
            return Err(
                DeployError::deployment_failed("Failed to set executable permissions").into(),
            );
        }

        info!("✅ Application deployed successfully to {}", device.model);
        Ok(())
    }

    /// Run the deployed application on the device
    pub async fn run_app(&self, _args: Vec<String>) -> Result<()> {
        let device = self.check_device().await?;

        info!(
            "🚀 Running BearDog on {} {}...",
            device.manufacturer, device.model
        );

        // Try to run the application
        let lib_name = "libpixel8_native_app.so";
        let device_path = format!("/data/local/tmp/{lib_name}");

        let mut cmd = Command::new("adb");
        cmd.args([
            "-s",
            &device.serial,
            "shell",
            &format!("cd /data/local/tmp && /system/bin/linker64 {device_path}"),
        ]);

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| DeployError::device_command_failed(format!("Failed to run app: {e}")))?;

        if !output.status.success() {
            warn!("App execution failed, trying alternative method...");

            // Try alternative execution method
            let alt_output = Command::new("adb")
                .args([
                    "-s",
                    &device.serial,
                    "shell",
                    &format!("LD_LIBRARY_PATH=/data/local/tmp {device_path}"),
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
                .map_err(|e| {
                    DeployError::device_command_failed(format!(
                        "Failed to run app (alternative): {e}"
                    ))
                })?;

            if !alt_output.status.success() {
                return Err(DeployError::device_command_failed(format!(
                    "App execution failed: {}",
                    String::from_utf8_lossy(&alt_output.stderr)
                ))
                .into());
            }
        }

        info!("✅ Application started successfully");
        Ok(())
    }

    /// Monitor application logs
    pub async fn monitor_logs(&self, filter: &str, follow: bool) -> Result<()> {
        let device = self.check_device().await?;

        info!(
            "📊 Monitoring logs from {} {} (filter: '{}')",
            device.manufacturer, device.model, filter
        );

        if !follow {
            // Clear and show recent logs
            let _ = Command::new("adb")
                .args(["-s", &device.serial, "logcat", "-c"])
                .output()
                .await;

            sleep(Duration::from_millis(500)).await;
        }

        // Start logcat with filtering
        let mut cmd = Command::new("adb");
        cmd.args(["-s", &device.serial, "logcat"]);

        if !follow {
            cmd.arg("-d"); // Dump and exit
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            DeployError::device_command_failed(format!("Failed to start logcat: {e}"))
        })?;

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            // Apply filter
            if line.to_lowercase().contains(&filter.to_lowercase())
                || line.contains("BearDog")
                || line.contains("StrongBox")
                || line.contains("HSM")
            {
                println!("{}", style(line).cyan());
            }
        }

        let _ = child.wait().await;
        Ok(())
    }
}
