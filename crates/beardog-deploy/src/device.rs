// SPDX-License-Identifier: AGPL-3.0-only

// Device management and deployment system
//
// This module provides comprehensive device management capabilities including
// device discovery, status monitoring, and deployment operations for Android and iOS devices.
// All operations maintain sovereignty compliance and zero hardcoded assumptions.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::time::Duration;

use crate::command_runner::CommandRunner;
#[cfg(not(test))]
use crate::command_runner::SystemCommandRunner;
#[cfg(test)]
use crate::command_runner::mock::MockAdbCommandRunner;
use tracing::{debug, error, info, warn};

/// Device type enumeration
///
/// Categorizes different types of devices that can be managed and deployed to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of device
pub enum DeviceType {
    /// Android device with `StrongBox` support
    AndroidStrongBox,
    /// iOS device with `Secure Enclave`
    IosSecureEnclave,
    /// Hardware HSM device
    HardwareHsm,
    /// Software HSM implementation
    SoftwareHsm,
    /// Unknown device type
    Unknown,
}

/// Device connection status
///
/// Represents the current connection and availability status of a device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceStatus {
    /// Device is reachable and ready for deployment (e.g. listed by discovery or env fallback).
    Available,
    /// Device is currently connected
    Connected,
    /// Device is disconnected
    Disconnected,
    /// Device is in error state
    Error,
}

///
/// capabilities, status, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub id: String,
    /// Human-readable device name
    /// Name of the item
    pub name: String,
    /// Type of device
    /// The device type value
    pub device_type: DeviceType,
    /// Current device status
    /// Current status of the component
    pub status: DeviceStatus,
    /// List of device capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Additional device metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

///
/// Handles device discovery, connection management, and deployment operations
pub struct DeviceManager {
    runner: Box<dyn CommandRunner>,
}

impl std::fmt::Debug for DeviceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceManager").finish_non_exhaustive()
    }
}

impl Default for DeviceManager {
    /// Creates default device manager
    ///
    /// # Returns
    /// Default `DeviceManager` instance
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    fn logcat_follow_timeout() -> Duration {
        std::env::var("BEARDOG_LOGCAT_FOLLOW_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map_or_else(|| Duration::from_secs(300), Duration::from_secs)
    }

    /// Creates a new device manager
    ///
    /// # Returns
    /// A new `DeviceManager` instance
    #[must_use]
    pub fn new() -> Self {
        #[cfg(test)]
        {
            Self::with_command_runner(Box::new(MockAdbCommandRunner::new()))
        }
        #[cfg(not(test))]
        {
            Self::with_command_runner(Box::new(SystemCommandRunner))
        }
    }

    /// Use a custom command runner (e.g. `MockAdbCommandRunner` in tests).
    #[must_use]
    pub fn with_command_runner(runner: Box<dyn CommandRunner>) -> Self {
        Self { runner }
    }

    /// Check for a default available device via capability detection
    ///
    /// This is a simplified interface that returns the first available device found
    /// through runtime discovery. For full device enumeration, use `check_devices()`.
    ///
    /// # Errors
    /// Returns an error if no devices are available.
    pub fn check_device(&self) -> Result<DeviceInfo, BearDogError> {
        info!("🔍 Checking for available device via capability detection...");

        // Try runtime device discovery first
        match self.detect_android_devices() {
            Ok(mut devices) if !devices.is_empty() => {
                info!("✅ Found device via adb: {}", devices[0].name);
                // Safe: we verified non-empty above
                Ok(devices.remove(0))
            }
            Ok(_) | Err(_) => {
                let get = |k: &str| std::env::var(k).ok();
                // Fallback: Check environment for device info
                let device_id =
                    get("BEARDOG_DEVICE_ID").unwrap_or_else(|| "local_fallback".to_string());
                let device_name = get("BEARDOG_DEVICE_NAME")
                    .unwrap_or_else(|| "Local Development Device".to_string());

                warn!("⚠️ No adb devices detected, using environment-configured fallback");

                Ok(DeviceInfo {
                    id: device_id.clone(),
                    name: device_name,
                    device_type: Self::detect_device_type_from_env_with(&get),
                    status: DeviceStatus::Available,
                    capabilities: Self::detect_capabilities_from_env_with(&get),
                    metadata: Self::build_device_metadata_with(&device_id, &get),
                })
            }
        }
    }

    /// Detect device type using a custom env lookup (tests inject a map).
    fn detect_device_type_from_env_with(get: &impl Fn(&str) -> Option<String>) -> DeviceType {
        if get("DEVICE_STRONGBOX_CAPABLE").unwrap_or_default() == "true" {
            DeviceType::AndroidStrongBox
        } else if get("DEVICE_SECURE_ENCLAVE_CAPABLE").unwrap_or_default() == "true" {
            DeviceType::IosSecureEnclave
        } else if get("DEVICE_HARDWARE_HSM_CAPABLE").unwrap_or_default() == "true" {
            DeviceType::HardwareHsm
        } else {
            DeviceType::SoftwareHsm // Safe fallback
        }
    }

    fn detect_capabilities_from_env_with(get: &impl Fn(&str) -> Option<String>) -> Vec<String> {
        let mut capabilities = Vec::new();

        if get("DEVICE_STRONGBOX_CAPABLE").unwrap_or_default() == "true" {
            capabilities.push("strongbox".to_string());
        }
        if get("DEVICE_BIOMETRIC_CAPABLE").unwrap_or_default() == "true" {
            capabilities.push("biometric_auth".to_string());
        }
        if get("DEVICE_SECURE_STORAGE_CAPABLE").unwrap_or_default() == "true" {
            capabilities.push("secure_storage".to_string());
        }

        if capabilities.is_empty() {
            capabilities.push("software_crypto".to_string());
            capabilities.push("basic_auth".to_string());
        }

        capabilities
    }

    fn build_device_metadata_with(
        device_id: &str,
        get: &impl Fn(&str) -> Option<String>,
    ) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("device_id".to_string(), device_id.to_string());
        map.insert(
            "storage_available".to_string(),
            get("DEVICE_STORAGE_BYTES").unwrap_or_else(|| "1073741824".to_string()),
        );
        map.insert(
            "strongbox_supported".to_string(),
            get("DEVICE_STRONGBOX_CAPABLE").unwrap_or_else(|| "false".to_string()),
        );
        map.insert(
            "secure_enclave_supported".to_string(),
            get("DEVICE_SECURE_ENCLAVE_CAPABLE").unwrap_or_else(|| "false".to_string()),
        );
        map.insert(
            "manufacturer".to_string(),
            get("DEVICE_MANUFACTURER").unwrap_or_else(|| "Unknown".to_string()),
        );
        map.insert(
            "model".to_string(),
            get("DEVICE_MODEL").unwrap_or_else(|| "Unknown".to_string()),
        );
        map.insert(
            "os_version".to_string(),
            get("DEVICE_OS_VERSION").unwrap_or_else(|| "Unknown".to_string()),
        );

        map
    }

    /// Check for all available devices via runtime discovery
    ///
    /// # Errors
    /// Returns an error if device detection fails completely.
    pub fn check_devices(&self) -> Result<Vec<DeviceInfo>, BearDogError> {
        info!("🔍 Checking for available devices via runtime discovery...");

        // Try real adb detection first
        match self.detect_android_devices() {
            Ok(devices) if !devices.is_empty() => {
                info!("✅ Found {} device(s) via adb", devices.len());
                Ok(devices)
            }
            Ok(_) | Err(_) => {
                warn!("⚠️ No adb devices found, attempting environment-based detection");

                // Try environment-configured device
                match self.check_device() {
                    Ok(device) => {
                        info!("✅ Using environment-configured device: {}", device.name);
                        Ok(vec![device])
                    }
                    Err(e) => {
                        error!("❌ No devices found via any detection method");
                        Err(BearDogError::system(format!(
                            "No devices found or connected. Tried adb and environment detection: {e}"
                        )))
                    }
                }
            }
        }
    }

    /// Deploy App operation - deploys the application to the device
    ///
    /// # Errors
    /// Returns an error if deployment fails.
    pub fn deploy_app(&self, release: bool) -> Result<(), BearDogError> {
        let build_type = if release { "release" } else { "debug" };
        info!("📲 Deploying {} build to device...", build_type);

        // Determine APK path based on build type
        let apk_path = if release {
            "android/app/build/outputs/apk/release/app-release.apk"
        } else {
            "android/app/build/outputs/apk/debug/app-debug.apk"
        };

        // Check if APK exists
        if !std::path::Path::new(apk_path).exists() {
            error!("APK not found at: {}", apk_path);
            return Err(BearDogError::system(format!(
                "APK not found at: {apk_path}. Build the app first using 'cargo build --release'"
            )));
        }

        // Get first available device
        let devices = self.check_devices()?;
        if devices.is_empty() {
            return Err(BearDogError::system(
                "No devices available for deployment".to_string(),
            ));
        }

        let device_id = &devices[0].id;
        info!("📲 Deploying to device: {}", device_id);

        let output = self
            .runner
            .run("adb", &["-s", device_id, "install", "-r", apk_path])
            .map_err(|e| BearDogError::system(format!("Failed to execute adb install: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Deployment failed: {}", stderr);
            return Err(BearDogError::system(format!("Deployment failed: {stderr}")));
        }

        info!("✅ App deployed successfully to {}", device_id);
        Ok(())
    }

    /// Run App operation - runs the deployed application
    ///
    /// # Errors
    /// Returns an error if app launch fails.
    pub fn run_app(&self, args: &[String]) -> Result<(), BearDogError> {
        info!("🚀 Running app with args: {:?}", args);

        // Get first available device
        let devices = self.check_devices()?;
        if devices.is_empty() {
            return Err(BearDogError::system("No devices available".to_string()));
        }

        let device_id = &devices[0].id;

        // Launch app using adb shell am start
        // Assuming package name is com.beardog.app (should be configurable)
        let package_name =
            std::env::var("BEARDOG_PACKAGE_NAME").unwrap_or_else(|_| "com.beardog.app".to_string());
        let main_activity = format!("{package_name}/MainActivity");

        let mut owned: Vec<String> = vec![
            "-s".to_string(),
            device_id.clone(),
            "shell".to_string(),
            "am".to_string(),
            "start".to_string(),
            "-n".to_string(),
            main_activity,
        ];
        for arg in args {
            owned.push("--es".to_string());
            owned.push("arg".to_string());
            owned.push(arg.clone());
        }
        let argv: Vec<&str> = owned.iter().map(String::as_str).collect();
        let output = self
            .runner
            .run("adb", &argv)
            .map_err(|e| BearDogError::system(format!("Failed to launch app: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("App launch failed: {}", stderr);
            return Err(BearDogError::system(format!("App launch failed: {stderr}")));
        }

        info!("✅ App started successfully on {}", device_id);
        Ok(())
    }

    /// Show Logs operation - displays application logs via logcat
    ///
    /// # Errors
    /// Returns an error if log streaming fails.
    pub fn show_logs(&self, package: &str, follow: bool) -> Result<(), BearDogError> {
        let follow_msg = if follow { " (following)" } else { "" };
        info!("📊 Showing logs for package: {}{}", package, follow_msg);

        // Get first available device
        let devices = self.check_devices()?;
        if devices.is_empty() {
            return Err(BearDogError::system("No devices available".to_string()));
        }

        let device_id = &devices[0].id;

        let _ = self
            .runner
            .run("adb", &["-s", device_id, "logcat", "-c"])
            .ok();

        info!("📊 Starting logcat for {} on device {}", package, device_id);

        if !follow {
            // Snapshot (bounded); do not use `--pid` with a package name (expects PID).
            let output = self
                .runner
                .run("adb", &["-s", device_id, "logcat", "-d", "-t", "200"])
                .map_err(|e| BearDogError::system(format!("Logcat failed: {e}")))?;

            if !output.status.success() {
                return Err(BearDogError::system("Logcat exited with error".to_string()));
            }
            return Ok(());
        }

        info!(
            "📊 Logcat following (max {}s) — set BEARDOG_LOGCAT_FOLLOW_SECS to override",
            Self::logcat_follow_timeout().as_secs()
        );

        match self.runner.run_bounded(
            "adb",
            &["-s", device_id, "logcat"],
            Self::logcat_follow_timeout(),
        ) {
            Ok(out) => {
                if !out.status.success() {
                    return Err(BearDogError::system("Logcat exited with error".to_string()));
                }
                Ok(())
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                info!("logcat follow stopped after timeout");
                Ok(())
            }
            Err(e) => Err(BearDogError::system(format!("Failed to start logcat: {e}"))),
        }
    }

    /// Detect Android Devices operation - Android-specific device detection via adb
    ///
    /// # Errors
    /// Returns an error if adb is not available or device detection fails.
    #[allow(
        dead_code,
        reason = "Public Android adb discovery API for tooling; not yet wired into default deploy flows."
    )]
    pub fn detect_android_devices(&self) -> Result<Vec<DeviceInfo>, BearDogError> {
        debug!("🔍 Detecting Android devices via adb...");

        let output = self.runner.run("adb", &["devices", "-l"]).map_err(|e| {
            error!("Failed to execute adb: {}", e);
            BearDogError::system(format!(
                "Failed to execute adb: {e}. Ensure adb is installed and in PATH"
            ))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("adb command failed: {}", stderr);
            return Err(BearDogError::system(format!(
                "adb command failed: {stderr}"
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        // Parse adb output
        for line in stdout.lines().skip(1) {
            // Skip "List of devices attached" header
            if line.trim().is_empty() {
                continue;
            }

            // Parse line format: "device_id    device   ..."
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                let device_id = parts[0].to_string();

                // Get device properties
                let api_level = self
                    .get_device_property(&device_id, "ro.build.version.sdk")
                    .unwrap_or_else(|_| "unknown".to_string());
                let model = self
                    .get_device_property(&device_id, "ro.product.model")
                    .unwrap_or_else(|_| "Unknown Model".to_string());
                let manufacturer = self
                    .get_device_property(&device_id, "ro.product.manufacturer")
                    .unwrap_or_else(|_| "Unknown".to_string());

                // Check for StrongBox support
                let strongbox_available = self.has_strongbox_support(&device_id);

                let mut metadata = HashMap::new();
                metadata.insert("adb_id".to_string(), device_id.clone());
                metadata.insert("api_level".to_string(), api_level);
                metadata.insert("model".to_string(), model.clone());
                metadata.insert("manufacturer".to_string(), manufacturer);
                metadata.insert(
                    "strongbox_available".to_string(),
                    strongbox_available.to_string(),
                );

                devices.push(DeviceInfo {
                    id: device_id.clone(),
                    name: format!("{model} ({device_id})"),
                    device_type: if strongbox_available {
                        DeviceType::AndroidStrongBox
                    } else {
                        DeviceType::Unknown
                    },
                    status: DeviceStatus::Connected,
                    capabilities: self.detect_device_capabilities(&device_id, strongbox_available),
                    metadata,
                });
            }
        }

        info!("✅ Found {} Android device(s) via adb", devices.len());
        Ok(devices)
    }

    /// Get device property via adb shell getprop
    fn get_device_property(&self, device_id: &str, property: &str) -> Result<String, BearDogError> {
        let output = self
            .runner
            .run("adb", &["-s", device_id, "shell", "getprop", property])
            .map_err(|e| BearDogError::system(format!("Failed to get device property: {e}")))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(BearDogError::system("Failed to read property".to_string()))
        }
    }

    /// Check if device has StrongBox support
    fn has_strongbox_support(&self, device_id: &str) -> bool {
        let output = self
            .runner
            .run("adb", &["-s", device_id, "shell", "pm", "list", "features"]);

        if let Ok(output) = output {
            let features = String::from_utf8_lossy(&output.stdout);
            features.contains("android.hardware.strongbox_keystore")
        } else {
            false
        }
    }

    /// Detect device capabilities
    fn detect_device_capabilities(&self, device_id: &str, has_strongbox: bool) -> Vec<String> {
        let mut capabilities = vec!["android".to_string(), "keystore".to_string()];

        if has_strongbox {
            capabilities.push("strongbox".to_string());
        }

        if let Ok(output) = self
            .runner
            .run("adb", &["-s", device_id, "shell", "pm", "list", "features"])
        {
            let features = String::from_utf8_lossy(&output.stdout);
            if features.contains("android.hardware.fingerprint")
                || features.contains("android.hardware.biometrics")
            {
                capabilities.push("biometric".to_string());
            }
        }

        capabilities
    }

    /// Deploy to Android operation - Android-specific deployment
    ///
    /// # Errors
    /// Returns an error if deployment fails.
    #[allow(
        dead_code,
        reason = "Public APK install hook for Android workflows; callers integrate explicitly."
    )]
    pub fn deploy_to_android(&self, device_id: &str, apk_path: &str) -> Result<(), BearDogError> {
        info!(
            "📲 Deploying to Android device: {} with APK: {}",
            device_id, apk_path
        );

        // Check if APK exists
        if !std::path::Path::new(apk_path).exists() {
            error!("APK not found at: {}", apk_path);
            return Err(BearDogError::system(format!(
                "APK not found at: {apk_path}"
            )));
        }

        info!("📦 Installing APK...");
        let output = self
            .runner
            .run("adb", &["-s", device_id, "install", "-r", "-t", apk_path])
            .map_err(|e| BearDogError::system(format!("Failed to execute adb install: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            error!("Deployment failed: {} {}", stdout, stderr);
            return Err(BearDogError::system(format!(
                "Deployment failed: {stdout} {stderr}"
            )));
        }

        let output_text = String::from_utf8_lossy(&output.stdout);
        if output_text.contains("Success") {
            info!("✅ Successfully deployed to Android device: {}", device_id);
            Ok(())
        } else {
            error!("Deployment output: {}", output_text);
            Err(BearDogError::system(format!(
                "Deployment may have failed: {output_text}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_manager_default_matches_new() {
        assert!(format!("{:?}", DeviceManager::default()).contains("DeviceManager"));
        let _ = DeviceManager::new();
    }

    #[test]
    fn test_device_type_serde_roundtrip() {
        for dt in [
            DeviceType::AndroidStrongBox,
            DeviceType::IosSecureEnclave,
            DeviceType::HardwareHsm,
            DeviceType::SoftwareHsm,
            DeviceType::Unknown,
        ] {
            let json = serde_json::to_string(&dt).unwrap();
            let back: DeviceType = serde_json::from_str(&json).unwrap();
            assert_eq!(dt, back);
        }
    }

    #[test]
    fn test_device_status_serde_roundtrip() {
        for st in [
            DeviceStatus::Available,
            DeviceStatus::Connected,
            DeviceStatus::Disconnected,
            DeviceStatus::Error,
        ] {
            let json = serde_json::to_string(&st).unwrap();
            let back: DeviceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(st, back);
        }
    }

    #[test]
    fn test_device_info_serde_roundtrip() {
        let info = DeviceInfo {
            id: "id-1".to_string(),
            name: "n".to_string(),
            device_type: DeviceType::HardwareHsm,
            status: DeviceStatus::Connected,
            capabilities: vec!["a".to_string()],
            metadata: HashMap::from([("k".to_string(), "v".to_string())]),
        };
        let json = serde_json::to_string(&info).unwrap();
        let back: DeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info.id, back.id);
        assert_eq!(info.capabilities, back.capabilities);
    }

    #[test]
    fn test_detect_device_type_strongbox_env() {
        let mut map = HashMap::new();
        map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "true".to_string());
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(
            DeviceManager::detect_device_type_from_env_with(&get),
            DeviceType::AndroidStrongBox
        );
    }

    #[test]
    fn test_detect_device_type_secure_enclave_env() {
        let mut map = HashMap::new();
        map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "false".to_string());
        map.insert(
            "DEVICE_SECURE_ENCLAVE_CAPABLE".to_string(),
            "true".to_string(),
        );
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(
            DeviceManager::detect_device_type_from_env_with(&get),
            DeviceType::IosSecureEnclave
        );
    }

    #[test]
    fn test_detect_device_type_hardware_hsm_env() {
        let mut map = HashMap::new();
        map.insert(
            "DEVICE_HARDWARE_HSM_CAPABLE".to_string(),
            "true".to_string(),
        );
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(
            DeviceManager::detect_device_type_from_env_with(&get),
            DeviceType::HardwareHsm
        );
    }

    #[test]
    fn test_detect_capabilities_from_env_all_flags() {
        let mut map = HashMap::new();
        map.insert("DEVICE_STRONGBOX_CAPABLE".to_string(), "true".to_string());
        map.insert("DEVICE_BIOMETRIC_CAPABLE".to_string(), "true".to_string());
        map.insert(
            "DEVICE_SECURE_STORAGE_CAPABLE".to_string(),
            "true".to_string(),
        );
        let get = |k: &str| map.get(k).cloned();
        let caps = DeviceManager::detect_capabilities_from_env_with(&get);
        assert!(caps.contains(&"strongbox".to_string()));
        assert!(caps.contains(&"biometric_auth".to_string()));
        assert!(caps.contains(&"secure_storage".to_string()));
    }

    #[test]
    fn test_build_device_metadata_env_overrides() {
        let mut map = HashMap::new();
        map.insert("DEVICE_STORAGE_BYTES".to_string(), "2048".to_string());
        map.insert("DEVICE_MANUFACTURER".to_string(), "Acme".to_string());
        map.insert("DEVICE_MODEL".to_string(), "X1".to_string());
        map.insert("DEVICE_OS_VERSION".to_string(), "14".to_string());
        let get = |k: &str| map.get(k).cloned();
        let m = DeviceManager::build_device_metadata_with("dev-xyz", &get);
        assert_eq!(m["device_id"], "dev-xyz");
        assert_eq!(m["storage_available"], "2048");
        assert_eq!(m["manufacturer"], "Acme");
        assert_eq!(m["model"], "X1");
        assert_eq!(m["os_version"], "14");
    }

    #[test]
    fn test_detect_device_type_from_env_default() {
        let get = |_k: &str| -> Option<String> { None };
        assert_eq!(
            DeviceManager::detect_device_type_from_env_with(&get),
            DeviceType::SoftwareHsm
        );
    }

    #[test]
    fn test_detect_capabilities_from_env_defaults() {
        let get = |_k: &str| -> Option<String> { None };
        let caps = DeviceManager::detect_capabilities_from_env_with(&get);
        assert!(caps.contains(&"software_crypto".to_string()));
        assert!(caps.contains(&"basic_auth".to_string()));
    }

    #[test]
    fn test_build_device_metadata_structure() {
        let metadata =
            DeviceManager::build_device_metadata_with("test-123", &|_k| -> Option<String> { None });
        assert_eq!(metadata["device_id"], "test-123");
        assert!(metadata.contains_key("storage_available"));
        assert!(metadata.contains_key("strongbox_supported"));
        assert!(metadata.contains_key("secure_enclave_supported"));
        assert!(metadata.contains_key("manufacturer"));
        assert!(metadata.contains_key("model"));
        assert!(metadata.contains_key("os_version"));
        assert_eq!(metadata.len(), 7);
    }

    #[test]
    fn test_build_device_metadata_various_ids() {
        for id in [
            "a",
            "device-1",
            "pixel8a_strongbox",
            "long-id-with-many-parts",
        ] {
            let m = DeviceManager::build_device_metadata_with(id, &|_| None);
            assert_eq!(m["device_id"], id);
        }
    }

    #[test]
    fn test_check_device_env_fallback_with_empty_mock() {
        let mgr =
            DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::empty_devices()));
        let d = mgr.check_device().expect("fallback device");
        assert_eq!(d.id, "local_fallback");
    }

    #[test]
    fn test_detect_android_devices_exercises_adb() {
        let mgr = DeviceManager::new();
        // Exercises the adb code path — result depends on environment
        let _ = mgr.detect_android_devices();
    }

    #[test]
    fn test_check_device_always_returns_device() {
        let mgr = DeviceManager::new();
        let device = mgr
            .check_device()
            .expect("should always return a device via fallback");
        assert!(!device.id.is_empty());
        assert!(!device.name.is_empty());
        assert!(
            matches!(
                device.status,
                DeviceStatus::Available | DeviceStatus::Connected
            ),
            "status should be Available or Connected"
        );
    }

    #[test]
    fn test_check_devices_returns_nonempty() {
        let mgr = DeviceManager::new();
        let devices = mgr.check_devices().expect("should return devices");
        assert!(!devices.is_empty());
    }

    #[test]
    fn test_deploy_app_debug_no_apk() {
        let mgr = DeviceManager::new();
        let result = mgr.deploy_app(false);
        assert!(result.is_err());
    }

    #[test]
    fn test_deploy_app_release_no_apk() {
        let mgr = DeviceManager::new();
        let result = mgr.deploy_app(true);
        assert!(result.is_err());
    }

    #[test]
    fn test_deploy_to_android_nonexistent_apk() {
        let mgr = DeviceManager::new();
        let result = mgr.deploy_to_android("dev-0", "/nonexistent.apk");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_app_exercises_path() {
        let mgr = DeviceManager::new();
        let _ = mgr.run_app(&[]);
        let _ = mgr.run_app(&["--verbose".to_string()]);
    }

    #[test]
    fn test_show_logs_exercises_path() {
        let mgr = DeviceManager::new();
        let _ = mgr.show_logs("com.beardog.test", false);
        let _ = mgr.show_logs("com.beardog.test", true);
    }

    #[test]
    fn test_device_type_eq_and_clone() {
        let a = DeviceType::AndroidStrongBox;
        let b = a.clone();
        assert_eq!(a, b);
        assert_ne!(a, DeviceType::IosSecureEnclave);
    }

    #[test]
    fn test_device_status_eq_and_clone() {
        let a = DeviceStatus::Available;
        let b = a.clone();
        assert_eq!(a, b);
        assert_ne!(a, DeviceStatus::Error);
    }

    #[test]
    fn test_device_info_debug_format() {
        let info = DeviceInfo {
            id: "dbg-test".to_string(),
            name: "Debug".to_string(),
            device_type: DeviceType::Unknown,
            status: DeviceStatus::Disconnected,
            capabilities: vec![],
            metadata: HashMap::new(),
        };
        let dbg = format!("{:?}", info);
        assert!(dbg.contains("dbg-test"));
        assert!(dbg.contains("Unknown"));
        assert!(dbg.contains("Disconnected"));
    }

    #[test]
    fn test_detect_device_capabilities_exercises_path() {
        let mgr = DeviceManager::new();
        let caps = mgr.detect_device_capabilities("fake-device", false);
        assert!(caps.contains(&"android".to_string()));
        assert!(caps.contains(&"keystore".to_string()));
        // Without strongbox, should not contain "strongbox"
        assert!(!caps.contains(&"strongbox".to_string()));
    }

    #[test]
    fn test_detect_device_capabilities_with_strongbox() {
        let mgr = DeviceManager::new();
        let caps = mgr.detect_device_capabilities("fake-device", true);
        assert!(caps.contains(&"strongbox".to_string()));
    }

    #[test]
    fn test_has_strongbox_support_exercises_path() {
        let mgr = DeviceManager::new();
        // Will return false without real device, but exercises path
        let _ = mgr.has_strongbox_support("fake-device");
    }

    #[test]
    fn test_get_device_property_exercises_path() {
        let mgr = DeviceManager::new();
        let _ = mgr.get_device_property("fake-device", "ro.build.version.sdk");
    }

    #[test]
    fn test_detect_android_devices_mock_emulator_full_parse() {
        let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        let devices = mgr.detect_android_devices().expect("mock adb");
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id, "emulator-5554");
        assert!(devices[0].metadata.contains_key("api_level"));
        assert_eq!(devices[0].device_type, DeviceType::AndroidStrongBox);
    }

    #[test]
    fn test_check_device_prefers_adb_when_emulator_listed() {
        let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        let d = mgr.check_device().expect("device");
        assert_eq!(d.id, "emulator-5554");
        assert_eq!(d.status, DeviceStatus::Connected);
    }

    #[test]
    fn test_check_devices_via_adb_mock() {
        let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        let v = mgr.check_devices().expect("devices");
        assert_eq!(v.len(), 1);
        assert!(v[0].name.contains("Pixel_Test"));
    }

    #[test]
    fn test_get_device_property_sdk_from_mock() {
        let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        let v = mgr
            .get_device_property("emulator-5554", "ro.build.version.sdk")
            .expect("sdk");
        assert_eq!(v, "33");
    }

    #[test]
    fn test_deploy_to_android_success_with_temp_apk() {
        let mut path = std::env::temp_dir();
        path.push(format!("beardog_deploy_apk_{}.apk", std::process::id()));
        std::fs::write(&path, b"dummy").expect("write apk");
        let mgr = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        let r = mgr.deploy_to_android("emulator-5554", path.to_str().expect("utf8"));
        let _ = std::fs::remove_file(&path);
        assert!(r.is_ok());
    }
}
