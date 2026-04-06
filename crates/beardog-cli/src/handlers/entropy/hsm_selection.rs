// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM label formatting and tier selection for entropy collection.

use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::universal_discovery::HsmInterfaceType;

use super::types::HsmInfo;

/// Maps a discovered HSM interface to a short CLI label (used by `handle_entropy_collect`).
pub(crate) fn format_hsm_interface_type_label(interface_type: &HsmInterfaceType) -> String {
    match interface_type {
        HsmInterfaceType::Tpm { version } => format!("TPM {version}"),
        HsmInterfaceType::SoftwareHsm { implementation } => {
            format!("Software ({implementation})")
        }
        HsmInterfaceType::MobileHsm { platform, .. } => format!("Mobile ({platform})"),
        HsmInterfaceType::CloudKms { provider, .. } => format!("Cloud ({provider})"),
        HsmInterfaceType::NetworkHsm { endpoint, .. } => format!("Network ({endpoint})"),
        HsmInterfaceType::UsbHsm { device_id } => format!("USB ({device_id})"),
        HsmInterfaceType::SmartCard { reader } => format!("SmartCard ({reader})"),
        HsmInterfaceType::CustomApi { api_type, .. } => format!("Custom ({api_type})"),
    }
}

/// Pick an HSM from a discovered list according to CLI preference (`auto`, `software`, …).
pub(crate) fn select_hsm_by_preference<'a>(
    available_hsms: &'a [HsmInfo],
    device_preference: &str,
) -> Result<&'a HsmInfo, BearDogError> {
    match device_preference.to_lowercase().as_str() {
        "auto" => available_hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .or_else(|| available_hsms.iter().find(|h| h.tier == "Hardware"))
            .or_else(|| available_hsms.iter().find(|h| h.tier == "Software"))
            .ok_or_else(|| BearDogError::not_found("No suitable HSM found".to_string())),
        "software" => available_hsms
            .iter()
            .find(|h| h.tier == "Software")
            .ok_or_else(|| {
                BearDogError::not_found(
                    "No software HSM found (install a PKCS#11 provider)".to_string(),
                )
            }),
        "mobile" => available_hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .ok_or_else(|| {
                BearDogError::not_found(
                    "No mobile HSM found (check Android StrongBox via ADB)".to_string(),
                )
            }),
        "usb" | "hardware" => available_hsms
            .iter()
            .find(|h| h.tier == "Hardware")
            .ok_or_else(|| {
                BearDogError::not_found(
                    "No hardware HSM found (connect any FIDO2/CTAP2 security token)".to_string(),
                )
            }),
        _ => {
            let msg = format!(
                "Unknown device preference: '{device_preference}'. Use: auto, software, mobile, usb, hardware"
            );
            Err(BearDogError::invalid_input(&msg))
        }
    }
}
