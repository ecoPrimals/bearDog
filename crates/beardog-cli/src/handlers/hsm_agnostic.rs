// SPDX-License-Identifier: AGPL-3.0-only

//! Universal HSM discovery via `beardog-tunnel` (no hardcoded vendor paths).

use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::types::HsmTier;
use beardog_tunnel::{DiscoveredHsm, DiscoveryEngine};

/// Convert discovered HSM to CLI-friendly format
#[derive(Debug, Clone)]
pub struct CliHsmInfo {
    /// Stable short id (hash-derived) for CLI references
    pub id: String,
    /// Display name (`vendor` + `model`)
    pub name: String,
    /// Vendor or manufacturer label from discovery
    pub vendor: String,
    /// Model name from discovery
    pub model: String,
    /// Tier label: Hardware, Software, Mobile, Cloud, …
    pub tier: String,
    /// High-level interface kind (USB, TPM, Software, …)
    pub hsm_type: String,
    /// Endpoint or device path detail (reserved for diagnostics; also in `interface_detail`)
    pub path: String,
    /// Single-line description of interface and path
    pub interface_detail: String,
}

impl From<DiscoveredHsm> for CliHsmInfo {
    fn from(hsm: DiscoveredHsm) -> Self {
        let tier_str = match hsm.assigned_tier {
            HsmTier::Hardware => "Hardware",
            HsmTier::Software => "Software",
            HsmTier::Cloud => "Cloud",
            HsmTier::Mobile => "Mobile",
            _ => "Unknown",
        };

        let (hsm_type, path) = match hsm.interface_type {
            beardog_tunnel::HsmInterfaceType::CloudKms { provider, .. } => {
                ("CloudKMS".to_string(), provider)
            }
            beardog_tunnel::HsmInterfaceType::NetworkHsm { endpoint, .. } => {
                ("NetworkHSM".to_string(), endpoint)
            }
            beardog_tunnel::HsmInterfaceType::UsbHsm { device_id } => {
                ("USB Token".to_string(), device_id)
            }
            beardog_tunnel::HsmInterfaceType::SoftwareHsm { implementation } => {
                ("Software".to_string(), implementation)
            }
            beardog_tunnel::HsmInterfaceType::Tpm { version } => ("TPM".to_string(), version),
            beardog_tunnel::HsmInterfaceType::SmartCard { reader } => {
                ("Smart Card".to_string(), reader)
            }
            beardog_tunnel::HsmInterfaceType::MobileHsm { platform, .. } => {
                ("Mobile HSM".to_string(), platform)
            }
            beardog_tunnel::HsmInterfaceType::CustomApi { api_type, endpoint } => {
                (api_type, endpoint)
            }
        };

        // Generate deterministic unique ID based on HSM properties
        // This ensures discovery is reproducible and testable
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(hsm.vendor.as_bytes());
        hasher.update(hsm.model.as_bytes());
        hasher.update(hsm_type.as_bytes());
        hasher.update(path.as_bytes());
        let hash = hasher.finalize();

        let id = format!(
            "{}-{:x}",
            hsm.vendor.to_lowercase(),
            u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
        );

        Self {
            id,
            name: format!("{} {}", hsm.vendor, hsm.model),
            vendor: hsm.vendor.clone(),
            model: hsm.model,
            tier: tier_str.to_string(),
            hsm_type: hsm_type.clone(),
            path: path.clone(),
            interface_detail: format!("{hsm_type} via {path}"),
        }
    }
}

/// Discover all available HSMs using universal discovery
///
/// # Errors
///
/// Returns an error if the discovery engine cannot be initialized.
pub async fn discover_all_hsms() -> Result<Vec<CliHsmInfo>, BearDogError> {
    let engine = DiscoveryEngine::new()?;

    let mut all_hsms = Vec::new();

    // Discover PKCS#11 HSMs (any vendor)
    if let Ok(pkcs11_hsms) = engine.discover_pkcs11_hsms() {
        all_hsms.extend(pkcs11_hsms);
    }

    // Discover USB HSMs (any FIDO2/CTAP2 device)
    if let Ok(usb_hsms) = engine.discover_usb_hsms() {
        all_hsms.extend(usb_hsms);
    }

    // Discover Mobile HSMs (StrongBox, Secure Enclave)
    if let Ok(mobile_hsms) = engine.discover_mobile_hsms() {
        all_hsms.extend(mobile_hsms);
    }

    // Discover Software HSMs
    if let Ok(software_hsms) = engine.discover_software_hsms() {
        all_hsms.extend(software_hsms);
    }

    // Discover TPMs
    if let Ok(tpm_hsms) = engine.discover_tpm_hsms() {
        all_hsms.extend(tpm_hsms);
    }

    // Discover Smart Cards
    if let Ok(smartcard_hsms) = engine.discover_smartcard_hsms() {
        all_hsms.extend(smartcard_hsms);
    }

    // Convert to CLI format
    Ok(all_hsms.into_iter().map(CliHsmInfo::from).collect())
}

/// Select best HSM based on user preference
///
/// # Errors
///
/// Returns an error if no HSMs are available or no HSM matches the preference.
#[allow(
    dead_code,
    reason = "Exposed for tests and future interactive HSM selection."
)]
pub fn select_hsm<'a>(
    hsms: &'a [CliHsmInfo],
    preference: &str,
) -> Result<&'a CliHsmInfo, BearDogError> {
    if hsms.is_empty() {
        return Err(BearDogError::not_found(
            "No HSMs found. Please install a PKCS#11 provider or connect hardware.".to_string(),
        ));
    }

    match preference.to_lowercase().as_str() {
        "hardware" | "hw" => {
            // Prefer hardware HSMs
            hsms.iter()
                .filter(|h| h.tier == "Hardware")
                .max_by_key(|h| {
                    // Prioritize by type: USB Token > TPM > Other; `path` breaks ties deterministically
                    let type_rank = match h.hsm_type.as_str() {
                        "USB Token" => 100,
                        "TPM" => 50,
                        _ => 10,
                    };
                    (type_rank, h.path.as_str())
                })
                .ok_or_else(|| BearDogError::not_found("No hardware HSMs found".to_string()))
        }
        "software" | "sw" => {
            // Software HSMs only
            hsms.iter()
                .find(|h| h.tier == "Software")
                .ok_or_else(|| BearDogError::not_found("No software HSMs found".to_string()))
        }
        "mobile" => {
            // Mobile HSMs only
            hsms.iter()
                .find(|h| h.tier == "Mobile")
                .ok_or_else(|| BearDogError::not_found("No mobile HSMs found".to_string()))
        }
        "auto" => {
            // Smart selection: Hardware > Software > Mobile > Cloud
            hsms.iter()
                .max_by_key(|h| {
                    let tier_rank = match h.tier.as_str() {
                        "Hardware" => 100,
                        "Software" => 50,
                        "Mobile" => 40,
                        "Cloud" => 30,
                        _ => 1,
                    };
                    (tier_rank, h.path.as_str())
                })
                .ok_or_else(|| BearDogError::not_found("No HSMs available".to_string()))
        }
        _ => {
            // Unknown preference - default to auto selection
            hsms.iter()
                .max_by_key(|h| {
                    let tier_rank = match h.tier.as_str() {
                        "Hardware" => 100,
                        "Software" => 50,
                        "Mobile" => 25,
                        _ => 1,
                    };
                    (tier_rank, h.path.as_str())
                })
                .ok_or_else(|| BearDogError::not_found("No HSMs available".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_finds_hsms() {
        // This test will pass if ANY HSM is found (any vendor)
        let result = discover_all_hsms().await;

        // Should succeed (even if empty - no error)
        assert!(result.is_ok(), "Discovery should not error");

        // If any PKCS#11 provider is installed, we should find at least one
        if let Ok(hsms) = result {
            println!("Found {} HSMs", hsms.len());
            for hsm in &hsms {
                println!("  - {} ({})", hsm.name, hsm.tier);
            }
        }
    }

    #[test]
    fn test_select_hsm_auto() {
        let hsms = vec![
            CliHsmInfo {
                id: "sw-1".to_string(),
                name: "SoftHSM2".to_string(),
                vendor: "OpenDNSSEC".to_string(),
                model: "SoftHSM".to_string(),
                tier: "Software".to_string(),
                hsm_type: "PKCS#11".to_string(),
                path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
                interface_detail: "PKCS#11 Software HSM".to_string(),
            },
            CliHsmInfo {
                id: "hw-1".to_string(),
                name: "Solo V2".to_string(),
                vendor: "SoloKeys".to_string(),
                model: "Solo 2".to_string(),
                tier: "Hardware".to_string(),
                hsm_type: "USB Token".to_string(),
                path: "USB".to_string(),
                interface_detail: "FIDO2/CTAP2 Hardware Token".to_string(),
            },
        ];

        let selected = select_hsm(&hsms, "auto").unwrap();

        // Should prefer hardware
        assert_eq!(selected.tier, "Hardware");
        assert_eq!(selected.vendor, "SoloKeys");
    }

    #[test]
    fn test_select_hsm_software() {
        let hsms = vec![CliHsmInfo {
            id: "sw-1".to_string(),
            name: "PKCS#11 Provider".to_string(),
            vendor: "Generic".to_string(),
            model: "Software HSM".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
            path: "discovered-via-pkcs11-scan".to_string(),
            interface_detail: "PKCS#11".to_string(),
        }];

        let selected = select_hsm(&hsms, "software").unwrap();
        assert_eq!(selected.tier, "Software");
    }

    #[test]
    fn test_select_hsm_no_hsms() {
        let hsms: Vec<CliHsmInfo> = vec![];
        let result = select_hsm(&hsms, "auto");
        assert!(result.is_err());
    }

    #[test]
    fn test_select_hsm_hardware_pref_no_hardware_tier() {
        let hsms = vec![CliHsmInfo {
            id: "sw-1".to_string(),
            name: "Soft".to_string(),
            vendor: "V".to_string(),
            model: "M".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
            path: "/x".to_string(),
            interface_detail: "d".to_string(),
        }];
        let err = select_hsm(&hsms, "hardware").expect_err("no hardware tier");
        let msg = format!("{err}");
        assert!(msg.contains("hardware") || msg.contains("found"), "{msg}");
    }

    #[test]
    fn test_select_hsm_mobile_pref_no_mobile_tier() {
        let hsms = vec![CliHsmInfo {
            id: "sw-1".to_string(),
            name: "Soft".to_string(),
            vendor: "V".to_string(),
            model: "M".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
            path: "/x".to_string(),
            interface_detail: "d".to_string(),
        }];
        assert!(select_hsm(&hsms, "mobile").is_err());
    }

    #[test]
    fn test_select_hsm_unknown_preference_falls_back_like_auto() {
        let hsms = vec![
            CliHsmInfo {
                id: "sw-1".to_string(),
                name: "Soft".to_string(),
                vendor: "V".to_string(),
                model: "M".to_string(),
                tier: "Software".to_string(),
                hsm_type: "PKCS#11".to_string(),
                path: "/a".to_string(),
                interface_detail: "d".to_string(),
            },
            CliHsmInfo {
                id: "hw-1".to_string(),
                name: "Hw".to_string(),
                vendor: "W".to_string(),
                model: "H".to_string(),
                tier: "Hardware".to_string(),
                hsm_type: "USB Token".to_string(),
                path: "/b".to_string(),
                interface_detail: "d".to_string(),
            },
        ];
        let picked = select_hsm(&hsms, "not-a-known-preference")
            .expect("unknown preference uses default branch");
        assert_eq!(picked.tier, "Hardware");
    }

    #[test]
    fn test_cli_hsm_info_from_secure_enclave_tier_maps_to_unknown_label() {
        use std::collections::HashMap;

        use beardog_tunnel::tunnel::hsm::types::HsmTier;
        use beardog_tunnel::tunnel::hsm::universal_discovery::AuthenticationMethod;
        use beardog_tunnel::{
            DiscoveredHsm, DiscoveryHsmHealthStatus, HsmConnectionInfo, HsmInterfaceType,
            UniversalHsmCapabilities,
        };

        let hsm = DiscoveredHsm {
            vendor: "Apple".to_string(),
            model: "SecureEnclave".to_string(),
            interface_type: HsmInterfaceType::SoftwareHsm {
                implementation: "test-impl".to_string(),
            },
            connection_info: HsmConnectionInfo {
                endpoint: "ep".to_string(),
                auth_method: AuthenticationMethod::None,
                timeout_ms: 1_000,
                encrypted: false,
                parameters: HashMap::new(),
            },
            capabilities: UniversalHsmCapabilities::default(),
            assigned_tier: HsmTier::SecureEnclave,
            supports_human_entropy: false,
            health_status: DiscoveryHsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
        };

        let info = CliHsmInfo::from(hsm);
        assert_eq!(info.tier, "Unknown");
        assert!(info.interface_detail.contains("Software"));
    }
}
