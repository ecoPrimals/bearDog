// SPDX-License-Identifier: AGPL-3.0-only

//! FIDO2 Device Discovery
//!
//! Discovers FIDO2/CTAP2 security keys on the system.
//!
//! # Pure Rust Implementation
//!
//! Uses `beardog-hid` for 100% Pure Rust HID access (no C dependencies).
//!
//! # Evolution History
//!
//! - **Jan 25, 2026**: Evolved from hidapi (C library) to beardog-hid (Pure Rust)
//!   - Eliminated C dependency
//!   - Direct /dev/hidraw access on Linux
//!   - ecoBin compliant

use super::types::{Fido2Capabilities, Fido2DeviceInfo, Fido2Transport};
use beardog_errors::BearDogError;
use std::path::PathBuf;
use tracing::{debug, info};

#[cfg(feature = "fido2")]
use beardog_hid::{discover, types::is_fido2_device, HidDeviceInfo};

/// Discover all FIDO2 devices on the system (Pure Rust)
///
/// # Example
///
/// ```rust,no_run
/// use beardog_security::hsm::fido2::discover_fido2_devices;
///
/// #[tokio::main]
/// async fn main() {
///     match discover_fido2_devices().await {
///         Ok(devices) => {
///             for device in devices {
///                 println!("Found: {} {}", device.manufacturer, device.product);
///             }
///         }
///         Err(e) => eprintln!("Discovery failed: {}", e),
///     }
/// }
/// ```
pub async fn discover_fido2_devices() -> Result<Vec<Fido2DeviceInfo>, BearDogError> {
    info!("🔍 Discovering FIDO2/CTAP2 security keys (Pure Rust)...");

    #[cfg(not(feature = "fido2"))]
    {
        tracing::warn!("FIDO2 support not enabled (compile with --features fido2)");
        return Ok(Vec::new());
    }

    #[cfg(feature = "fido2")]
    {
        let mut fido2_devices = Vec::new();

        // Discover all HID devices (Pure Rust!)
        let hid_devices = discover().await?;

        debug!(
            "Found {} HID devices, filtering for FIDO2...",
            hid_devices.len()
        );

        // Filter for FIDO2-compatible devices
        for hid_dev in hid_devices {
            if is_fido2_device(hid_dev.vendor_id, hid_dev.product_id) {
                debug!(
                    "Found FIDO2 device: {:04x}:{:04x} - {} {}",
                    hid_dev.vendor_id.0,
                    hid_dev.product_id.0,
                    hid_dev.manufacturer,
                    hid_dev.product
                );

                // Convert to FIDO2 device info
                let fido_info = convert_to_fido2_info(hid_dev).await?;

                info!(
                    "✅ Detected FIDO2 device: {} ({})",
                    fido_info.product, fido_info.manufacturer
                );

                fido2_devices.push(fido_info);
            }
        }

        if fido2_devices.is_empty() {
            info!("No FIDO2 devices found");
        } else {
            info!("Found {} FIDO2 device(s)", fido2_devices.len());
        }

        Ok(fido2_devices)
    }
}

#[cfg(feature = "fido2")]
async fn convert_to_fido2_info(hid_dev: HidDeviceInfo) -> Result<Fido2DeviceInfo, BearDogError> {
    Ok(Fido2DeviceInfo {
        device_path: PathBuf::from(hid_dev.path.clone()),
        vendor_id: hid_dev.vendor_id.0,
        product_id: hid_dev.product_id.0,
        manufacturer: hid_dev.manufacturer.clone(),
        product: hid_dev.product.clone(),
        serial: if hid_dev.serial.is_empty() {
            None
        } else {
            Some(hid_dev.serial.clone())
        },
        aaguid: None,
        firmware_version: None,
        protocol_versions: vec!["FIDO_2_0".to_string()],
        extensions: Vec::new(),
        transport: Fido2Transport::Usb,
        capabilities: probe_capabilities(&hid_dev).await,
    })
}

#[cfg(feature = "fido2")]
async fn probe_capabilities(_hid_dev: &HidDeviceInfo) -> Fido2Capabilities {
    // Assumes default FIDO2 capabilities until CTAP2 getInfo is implemented (Phase 2).
    tracing::info!(
        "FIDO2 capabilities: using assumed defaults (CTAP2 getInfo query pending Phase 2)"
    );
    Fido2Capabilities {
        resident_keys: true,
        user_presence: true,
        user_verification: true,
        hmac_secret: true,
        cred_protect: false,
        max_msg_size: 7609,
        max_cred_count: 25,
        algorithms: vec![-7, -8, -257], // ES256, EdDSA, RS256
        pin_protocols: vec![1, 2],
        max_resident_keys: Some(25),
        max_entropy_size: Some(32),
        supported_algorithms: vec![
            "ES256".to_string(),
            "EdDSA".to_string(),
            "RS256".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires physical FIDO2 device
    async fn test_discover_fido2_devices() {
        let result = discover_fido2_devices().await;
        assert!(result.is_ok());

        let devices = result.unwrap();
        if !devices.is_empty() {
            println!("Found {} FIDO2 device(s):", devices.len());
            for dev in devices {
                println!(
                    "  {} {} (VID:{:04x}, PID:{:04x})",
                    dev.manufacturer, dev.product, dev.vendor_id, dev.product_id
                );
            }
        }
    }
}
