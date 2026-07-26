// SPDX-License-Identifier: AGPL-3.0-or-later

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
use beardog_hid::{
    HidDeviceInfo, ProductId, VendorId, discover,
    types::{fido2_manufacturer_name, fido2_product_name, is_fido2_device},
};

/// Discover all FIDO2 devices on the system (Pure Rust)
///
/// # Example
///
/// ```rust,no_run
/// use beardog_security::hsm::fido2::discover_fido2_devices;
///
/// #[tokio::main(flavor = "current_thread")]
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
///
/// # Errors
///
/// Returns [`BearDogError`] when HID discovery or device probing fails (`fido2` feature).
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
    let (caps, aaguid, versions, extensions) = probe_capabilities_live(&hid_dev).await;

    Ok(Fido2DeviceInfo {
        device_path: PathBuf::from(hid_dev.path.clone()),
        vendor_id: hid_dev.vendor_id.0,
        product_id: hid_dev.product_id.0,
        manufacturer: if hid_dev.manufacturer.is_empty() || hid_dev.manufacturer == "Unknown" {
            identify_manufacturer(hid_dev.vendor_id.0)
        } else {
            hid_dev.manufacturer.clone()
        },
        product: if hid_dev.product.is_empty() || hid_dev.product == "Unknown" {
            identify_product(hid_dev.vendor_id.0, hid_dev.product_id.0)
        } else {
            hid_dev.product.clone()
        },
        serial: if hid_dev.serial.is_empty() {
            None
        } else {
            Some(hid_dev.serial.clone())
        },
        aaguid,
        firmware_version: None,
        protocol_versions: versions,
        extensions,
        transport: Fido2Transport::Usb,
        capabilities: caps,
    })
}

/// Probe device capabilities via live CTAP2 `GetInfo`, falling back to defaults
/// if the device cannot be opened or queried.
#[cfg(feature = "fido2")]
async fn probe_capabilities_live(
    hid_dev: &HidDeviceInfo,
) -> (Fido2Capabilities, Option<[u8; 16]>, Vec<String>, Vec<String>) {
    use super::ctap2;

    match beardog_hid::open_device(&hid_dev.path).await {
        Ok(mut device) => match ctap2::ctap2_get_info(&mut device).await {
            Ok(info) => {
                tracing::info!(
                    "FIDO2 capabilities: live CTAP2 GetInfo succeeded (versions: {})",
                    info.versions.join(", ")
                );

                let hmac_secret = info.extensions.iter().any(|e| e == "hmac-secret");
                let cred_protect = info.extensions.iter().any(|e| e == "credProtect");
                let rk = info.options.get("rk").copied().unwrap_or(false);
                let up = info.options.get("up").copied().unwrap_or(true);
                let uv = info.options.get("uv").copied().unwrap_or(false);

                let algorithms: Vec<i32> = info.algorithms.as_ref().map_or_else(
                    || vec![-7],
                    |algs| {
                        algs.iter()
                            .filter_map(|m| {
                                m.get("alg").and_then(|v| {
                                    if let ciborium::Value::Integer(n) = v {
                                        let n: i128 = (*n).into();
                                        i32::try_from(n).ok()
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect()
                    },
                );

                let supported_algorithms: Vec<String> = algorithms
                    .iter()
                    .map(|a| match a {
                        -7 => "ES256".to_string(),
                        -8 => "EdDSA".to_string(),
                        -257 => "RS256".to_string(),
                        other => format!("COSE({other})"),
                    })
                    .collect();

                let pin_protocols: Vec<u8> = info
                    .pin_protocols
                    .as_ref().map_or_else(|| vec![1], |pp| pp.iter().filter_map(|&v| u8::try_from(v).ok()).collect());

                let aaguid = if info.aaguid.len() == 16 {
                    let mut a = [0u8; 16];
                    a.copy_from_slice(&info.aaguid);
                    Some(a)
                } else {
                    None
                };

                #[expect(clippy::cast_possible_truncation, reason = "CTAP2 values fit in usize")]
                let caps = Fido2Capabilities {
                    resident_keys: rk,
                    user_presence: up,
                    user_verification: uv,
                    hmac_secret,
                    cred_protect,
                    max_msg_size: info.max_msg_size.map_or(1024, |v| v as usize),
                    max_cred_count: info
                        .max_credential_count_in_list
                        .map_or(1, |v| v as usize),
                    algorithms,
                    pin_protocols,
                    max_resident_keys: info.remaining_discoverable_credentials.map(|v| v as usize),
                    max_entropy_size: Some(64),
                    supported_algorithms,
                };

                (caps, aaguid, info.versions.clone(), info.extensions)
            }
            Err(e) => {
                tracing::warn!("CTAP2 GetInfo failed, using defaults: {e}");
                (default_capabilities(), None, vec!["FIDO_2_0".to_string()], Vec::new())
            }
        },
        Err(e) => {
            tracing::warn!("Cannot open HID device for probing, using defaults: {e}");
            (default_capabilities(), None, vec!["FIDO_2_0".to_string()], Vec::new())
        }
    }
}

#[cfg(feature = "fido2")]
fn default_capabilities() -> Fido2Capabilities {
    Fido2Capabilities {
        resident_keys: true,
        user_presence: true,
        user_verification: false,
        hmac_secret: false,
        cred_protect: false,
        max_msg_size: 1024,
        max_cred_count: 1,
        algorithms: vec![-7],
        pin_protocols: vec![1],
        max_resident_keys: None,
        max_entropy_size: Some(32),
        supported_algorithms: vec!["ES256".to_string()],
    }
}

/// Identify manufacturer from VID when sysfs doesn't provide it.
#[cfg(feature = "fido2")]
fn identify_manufacturer(vid: u16) -> String {
    fido2_manufacturer_name(VendorId(vid)).map_or_else(|| format!("VID:{vid:04X}"), str::to_string)
}

/// Identify product from VID/PID when sysfs doesn't provide it.
#[cfg(feature = "fido2")]
fn identify_product(vid: u16, pid: u16) -> String {
    fido2_product_name(VendorId(vid), ProductId(pid)).map_or_else(|| format!("PID:{pid:04X}"), str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires physical FIDO2 device"]
    async fn test_discover_fido2_devices() -> Result<(), BearDogError> {
        let devices = discover_fido2_devices().await?;
        if !devices.is_empty() {
            println!("Found {} FIDO2 device(s):", devices.len());
            for dev in devices {
                println!(
                    "  {} {} (VID:{:04x}, PID:{:04x})",
                    dev.manufacturer, dev.product, dev.vendor_id, dev.product_id
                );
            }
        }

        Ok(())
    }
}
