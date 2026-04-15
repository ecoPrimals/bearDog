use super::*;

#[test]
fn test_discoverer_creation() {
    let discoverer = MobileDiscoverer::new();
    assert!(discoverer.is_ok());
}

#[tokio::test]
async fn test_mobile_discovery() {
    let discoverer = MobileDiscoverer::new()?;
    let result = discoverer.discover().await;
    assert!(result.is_ok());
}

#[test]
fn test_platform_detection() {
    let platform = MobileDiscoverer::detect_platform();
    // Platform detection should not fail
    assert!(matches!(platform, MobilePlatform::Ios | MobilePlatform::Android | MobilePlatform::Unknown));
}

#[test]
fn test_ios_secure_enclave_capabilities() {
    let discoverer = MobileDiscoverer::new()?;
    let caps = discoverer.create_ios_secure_enclave_capabilities();
    
    assert_eq!(caps.security.fips_140_2_level, Some(2));
    assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
    assert!(caps.security.attestation);
    assert!(!caps.key_management.key_backup); // Keys cannot leave enclave
}

#[test]
fn test_android_strongbox_capabilities() {
    let discoverer = MobileDiscoverer::new()?;
    let caps = discoverer.create_android_strongbox_capabilities();
    
    assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
    assert!(caps.security.attestation);
    assert!(!caps.key_management.key_backup);
}

#[test]
fn test_samsung_knox_capabilities() {
    let discoverer = MobileDiscoverer::new()?;
    let caps = discoverer.create_samsung_knox_capabilities();
    
    assert_eq!(caps.security.common_criteria_eal, Some(5));
    assert!(caps.compliance.common_criteria);
}

#[test]
fn test_mobile_hsm_types() {
    assert_eq!(MobileHsmType::IosSecureEnclave, MobileHsmType::IosSecureEnclave);
    assert_ne!(MobileHsmType::IosSecureEnclave, MobileHsmType::AndroidStrongBox);
}
