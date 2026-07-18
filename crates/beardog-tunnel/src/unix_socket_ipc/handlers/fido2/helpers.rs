// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared helpers for FIDO2 IPC handlers: device resolution and provider construction.

use tracing::info;

/// Resolve a FIDO2 device path, auto-selecting the first FIDO2 device if none given.
///
/// # Errors
///
/// Returns an error if HID enumeration fails or no FIDO2 device is connected.
pub async fn resolve_device_path(explicit: Option<&str>) -> Result<String, String> {
    if let Some(path) = explicit {
        return Ok(path.to_string());
    }
    let devices = beardog_hid::discover()
        .await
        .map_err(|e| format!("HID enumeration failed: {e}"))?;
    let fido2_device = devices
        .into_iter()
        .find(|d| beardog_hid::types::is_fido2_device(d.vendor_id, d.product_id))
        .ok_or(
            "No FIDO2 device found — connect a USB security key \
             (SoloKey, YubiKey, or other CTAP2 authenticator)"
                .to_string(),
        )?;
    info!(
        path = fido2_device.path.as_str(),
        "Auto-selected FIDO2 device"
    );
    Ok(fido2_device.path)
}

/// Create a `SoloV2Provider` backed by a HID device.
///
/// # Errors
///
/// Returns an error if the device cannot be opened or transport initialization fails.
pub async fn create_ctap2_provider(
    device_path: &str,
    rp_id: &str,
) -> Result<crate::tunnel::hsm::solo_v2::SoloV2Provider, String> {
    use crate::tunnel::hsm::solo_v2::SoloV2Provider;
    use crate::tunnel::hsm::solo_v2::types::{SoloV2Config, SoloV2DeviceInfo};

    let device_info = SoloV2DeviceInfo {
        device_id: device_path.to_string(),
        product_name: "FIDO2 device".to_string(),
        firmware_version: "unknown".to_string(),
        is_connected: true,
        vendor_id: 0,
        product_id: 0,
    };

    let config = SoloV2Config {
        device_id: Some(device_path.to_string()),
        relying_party_id: rp_id.to_string(),
        require_user_presence: true,
        require_user_verification: false,
        user_interaction_timeout: 30,
    };

    SoloV2Provider::with_hid_device_path(device_info, config, device_path)
        .await
        .map_err(|e| format!("Failed to open CTAP2 device at {device_path}: {e}"))
}
