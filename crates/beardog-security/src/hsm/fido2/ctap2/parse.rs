// SPDX-License-Identifier: AGPL-3.0-only

//! CBOR parsing for CTAP2 responses (e.g. `GetInfo`).

use super::types::Ctap2DeviceInfo;
use beardog_errors::BearDogError;
use ciborium::Value as CborValue;
use std::collections::BTreeMap;
use tracing::debug;

/// Parse the CBOR payload of a successful CTAP2 `GetInfo` response (after status byte).
///
/// # Errors
///
/// Returns an error if the CBOR is malformed or not a valid `GetInfo` map.
pub fn parse_get_info_cbor(response_bytes: &[u8]) -> Result<Ctap2DeviceInfo, BearDogError> {
    let cbor_value: CborValue = ciborium::from_reader(response_bytes)
        .map_err(|e| BearDogError::system(format!("CBOR parse error: {e}")))?;

    let CborValue::Map(map) = cbor_value else {
        return Err(BearDogError::system(
            "GetInfo response not a CBOR map".to_string(),
        ));
    };

    let mut info = Ctap2DeviceInfo {
        versions: Vec::new(),
        extensions: Vec::new(),
        aaguid: Vec::new(),
        options: BTreeMap::new(),
        max_msg_size: None,
        pin_protocols: None,
        max_credential_count_in_list: None,
        max_credential_id_length: None,
        transports: None,
        algorithms: None,
        max_serialized_large_blob_array: None,
        force_pin_change: None,
        min_pin_length: None,
        firmware_version: None,
        max_cred_blob_length: None,
        max_rpids_for_set_min_pin_length: None,
        preferred_platform_uv_attempts: None,
        uv_modality: None,
        certifications: None,
        remaining_discoverable_credentials: None,
        vendor_prototype_config_commands: None,
    };

    for (key, value) in map {
        let key_int: Option<i128> = match key {
            CborValue::Integer(n) => Some(n.into()),
            _ => None,
        };

        match key_int {
            Some(1) => {
                // versions
                if let CborValue::Array(arr) = value {
                    for v in arr {
                        if let CborValue::Text(s) = v {
                            info.versions.push(s);
                        }
                    }
                }
            }
            Some(2) => {
                // extensions
                if let CborValue::Array(arr) = value {
                    for v in arr {
                        if let CborValue::Text(s) = v {
                            info.extensions.push(s);
                        }
                    }
                }
            }
            Some(3) => {
                // aaguid
                if let CborValue::Bytes(b) = value {
                    info.aaguid = b;
                }
            }
            Some(4) => {
                // options
                if let CborValue::Map(opts) = value {
                    for (opt_key, opt_val) in opts {
                        if let (CborValue::Text(key_str), CborValue::Bool(val_bool)) =
                            (opt_key, opt_val)
                        {
                            info.options.insert(key_str, val_bool);
                        }
                    }
                }
            }
            Some(5) => {
                // maxMsgSize
                if let CborValue::Integer(n) = value {
                    info.max_msg_size = Some(n.try_into().unwrap_or(0));
                }
            }
            Some(6) => {
                // pinProtocols
                if let CborValue::Array(arr) = value {
                    let protocols: Vec<u64> = arr
                        .iter()
                        .filter_map(|v| {
                            if let CborValue::Integer(n) = v {
                                (*n).try_into().ok()
                            } else {
                                None
                            }
                        })
                        .collect();
                    info.pin_protocols = Some(protocols);
                }
            }
            _ => {
                debug!("Unhandled GetInfo field: {:?}", key);
            }
        }
    }

    Ok(info)
}
