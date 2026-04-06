// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Pkcs11Discoverer;
use super::Pkcs11TokenInfo;
use crate::universal_hsm_discovery::{HsmTier, HsmType, TamperResistance};
use std::path::PathBuf;

#[test]
fn test_discoverer_creation() {
    let discoverer = Pkcs11Discoverer::new();
    assert!(discoverer.is_ok());
}

#[tokio::test]
async fn test_pkcs11_discovery() {
    let discoverer = Pkcs11Discoverer::new().expect("discoverer");
    let result = discoverer.discover().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_library_discovery() {
    let discoverer = Pkcs11Discoverer::new().expect("discoverer");
    let libraries = discoverer.find_pkcs11_libraries().await;
    assert!(libraries.is_ok());
}

#[test]
fn test_token_classification() {
    let discoverer = Pkcs11Discoverer::new().expect("discoverer");

    // Test Thales classification
    let thales_token = Pkcs11TokenInfo {
        library_path: PathBuf::from("/opt/nfast/lib/libcknfast.so"),
        label: "Thales Token".to_string(),
        manufacturer_id: "Thales".to_string(),
        model: "nShield".to_string(),
        serial_number: "123456".to_string(),
        slot_id: 0,
    };
    let (hsm_type, tier, _) = discoverer.classify_token(&thales_token);
    assert_eq!(hsm_type, HsmType::Hardware);
    assert_eq!(tier, HsmTier::Tier1);

    // Test SoftHSM classification
    let softhsm_token = Pkcs11TokenInfo {
        library_path: PathBuf::from("/usr/lib/softhsm/libsofthsm2.so"),
        label: "SoftHSM Token".to_string(),
        manufacturer_id: "SoftHSM Project".to_string(),
        model: "SoftHSM v2".to_string(),
        serial_number: "789012".to_string(),
        slot_id: 0,
    };
    let (hsm_type, tier, _) = discoverer.classify_token(&softhsm_token);
    assert_eq!(hsm_type, HsmType::Software);
    assert_eq!(tier, HsmTier::Tier3);
}

#[test]
fn test_enterprise_hsm_capabilities() {
    let discoverer = Pkcs11Discoverer::new().expect("discoverer");
    let caps = discoverer.create_enterprise_hsm_capabilities();

    assert!(caps.security.fips_140_2_level == Some(3));
    assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
    assert!(caps.key_management.key_backup);
    assert!(caps.advanced_features.quantum_resistant);
    assert!(caps.api_support.pkcs11);
}

#[test]
fn test_custom_library_paths() {
    let mut discoverer = Pkcs11Discoverer::new().expect("discoverer");
    discoverer.add_library_path(PathBuf::from("/custom/path/lib.so"));
    assert_eq!(discoverer.custom_library_paths.len(), 1);
}
