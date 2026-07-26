// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.discover` — enumerate connected FIDO2/CTAP2 devices.

use serde_json::{Value, json};
use tracing::info;

/// Enumerate connected FIDO2/CTAP2 devices.
///
/// # Returns
///
/// ```json
/// {
///   "devices": [
///     {
///       "path": "/dev/hidraw3",
///       "vendor_id": 7504,
///       "product_id": 24705,
///       "product_name": "SoloKeys Solo 2",
///       "fido2": true
///     }
///   ],
///   "count": 1
/// }
/// ```
pub fn handle_fido2_discover(_params: Option<&Value>) -> Result<Value, super::super::HandlerError> {
    #[cfg(feature = "ctap2")]
    {
        let raw_devices = beardog_hid::discover()
            .await
            .map_err(|e| format!("FIDO2 discovery failed: {e}"))?;
        let fido2_devices: Vec<Value> = raw_devices
            .iter()
            .filter(|d| beardog_hid::types::is_fido2_device(d.vendor_id, d.product_id))
            .map(|d| {
                json!({
                    "path": d.path,
                    "vendor_id": d.vendor_id.0,
                    "product_id": d.product_id.0,
                    "manufacturer": d.manufacturer,
                    "product": d.product,
                    "fido2": true,
                })
            })
            .collect();

        let count = fido2_devices.len();
        info!(count, "FIDO2 device discovery complete");

        Ok(json!({
            "devices": fido2_devices,
            "count": count,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        info!("FIDO2 discovery: feature not enabled, returning empty");
        Ok(json!({
            "devices": [],
            "count": 0,
            "note": "FIDO2 feature not enabled — rebuild with --features ctap2",
        }))
    }
}
