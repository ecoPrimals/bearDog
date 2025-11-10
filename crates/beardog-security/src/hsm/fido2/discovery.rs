//! FIDO2 Device Discovery
//!
//! Discovers FIDO2/CTAP2 security keys on the system.

use super::types::{Fido2Capabilities, Fido2DeviceInfo, Fido2Transport};
use beardog_errors::BearDogError;
use std::path::PathBuf;
use tracing::{debug, info, warn};

#[cfg(feature = "fido2")]
use hidapi::HidApi;

/// Discover all FIDO2 devices on the system
pub async fn discover_fido2_devices() -> Result<Vec<Fido2DeviceInfo>, BearDogError> {
    info!("🔍 Discovering FIDO2/CTAP2 security keys...");
    
    #[cfg(not(feature = "fido2"))]
    {
        warn!("FIDO2 support not enabled (compile with --features fido2)");
        return Ok(Vec::new());
    }
    
    #[cfg(feature = "fido2")]
    {
        let mut devices = Vec::new();
        
        // Initialize HID API
        let api = HidApi::new().map_err(|e| {
            BearDogError::system(format!("Failed to initialize HID API: {}", e))
        })?;
        
        // Enumerate HID devices
        for device_info in api.device_list() {
            // Check if this is a FIDO device
            // FIDO Usage Page: 0xF1D0, Usage: 0x01
            if is_fido_device(device_info) {
                debug!(
                    "Found potential FIDO2 device: {:04x}:{:04x} - {}",
                    device_info.vendor_id(),
                    device_info.product_id(),
                    device_info.product_string().unwrap_or("Unknown")
                );
                
                if let Ok(fido_info) = probe_fido2_device(&api, device_info).await {
                    info!(
                        "✅ Detected FIDO2 device: {} ({})",
                        fido_info.product, fido_info.manufacturer
                    );
                    devices.push(fido_info);
                }
            }
        }
        
        if devices.is_empty() {
            info!("No FIDO2 devices found");
        } else {
            info!("Found {} FIDO2 device(s)", devices.len());
        }
        
        Ok(devices)
    }
}

#[cfg(feature = "fido2")]
fn is_fido_device(device_info: &hidapi::DeviceInfo) -> bool {
    // FIDO2 devices use HID Usage Page 0xF1D0
    // We can also check for known vendor IDs
    const FIDO_USAGE_PAGE: u16 = 0xF1D0;
    
    // Known FIDO2 vendor IDs
    const SOLOKEYS_VID: u16 = 0x1209;  // SoloKeys
    const YUBICO_VID: u16 = 0x1050;     // YubiKey
    const GOOGLE_VID: u16 = 0x18D1;     // Google Titan
    const ONLYKEY_VID: u16 = 0x1D50;    // OnlyKey
    
    let vid = device_info.vendor_id();
    let usage_page = device_info.usage_page();
    
    // Check usage page (if available)
    if usage_page == FIDO_USAGE_PAGE {
        return true;
    }
    
    // Check known vendor IDs
    matches!(vid, SOLOKEYS_VID | YUBICO_VID | GOOGLE_VID | ONLYKEY_VID)
}

#[cfg(feature = "fido2")]
async fn probe_fido2_device(
    api: &HidApi,
    device_info: &hidapi::DeviceInfo,
) -> Result<Fido2DeviceInfo, BearDogError> {
    // Build device path
    let device_path = match device_info.path().to_str() {
        Ok(path) => PathBuf::from(path),
        Err(_) => return Err(BearDogError::system("Invalid device path encoding".to_string())),
    };
    
    // For now, create basic device info
    // TODO: Query actual CTAP2 capabilities via GetInfo command
    let fido_info = Fido2DeviceInfo {
        device_path,
        vendor_id: device_info.vendor_id(),
        product_id: device_info.product_id(),
        manufacturer: device_info
            .manufacturer_string()
            .unwrap_or("Unknown")
            .to_string(),
        product: device_info
            .product_string()
            .unwrap_or("FIDO2 Security Key")
            .to_string(),
        serial: device_info.serial_number().map(|s| s.to_string()),
        aaguid: None,  // Will be populated by GetInfo
        firmware_version: None,
        protocol_versions: vec!["FIDO_2_0".to_string()],
        extensions: Vec::new(),
        transport: Fido2Transport::Usb,
        capabilities: Fido2Capabilities::default(),
    };
    
    Ok(fido_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fido2_discovery() {
        // This will only find devices if hardware is connected
        let result = discover_fido2_devices().await;
        assert!(result.is_ok(), "Discovery should not fail");
        
        let devices = result.unwrap();
        println!("Found {} FIDO2 devices", devices.len());
        
        for device in devices {
            println!("  - {} by {}", device.product, device.manufacturer);
            println!("    Path: {:?}", device.device_path);
            println!("    VID:PID: {:04x}:{:04x}", device.vendor_id, device.product_id);
        }
    }
}

