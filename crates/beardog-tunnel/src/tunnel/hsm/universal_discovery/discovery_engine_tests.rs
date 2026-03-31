// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for HSM discovery engine (extracted from `discovery_engine.rs` for file size compliance).

use super::*;

#[tokio::test]
async fn test_discovery_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
    let engine = DiscoveryEngine::new();
    assert!(engine.is_ok());
    Ok(())
}

#[test]
fn test_pkcs11_discoverer() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = Pkcs11Discoverer::new();
    assert!(discoverer.is_ok());
    Ok(())
}

#[test]
fn test_software_hsm_discovery() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = SoftwareHsmDiscoverer::new()?;
    let result = discoverer.discover();
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_all_discoverers_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let pkcs11 = Pkcs11Discoverer::new();
    let cloud_kms = CloudKmsDiscoverer::new();
    let network = NetworkHsmDiscoverer::new();
    let usb = UsbHsmDiscoverer::new();
    let software = SoftwareHsmDiscoverer::new();
    let mobile = MobileHsmDiscoverer::new();
    let tpm = TpmDiscoverer::new();
    let smartcard = SmartCardDiscoverer::new();

    assert!(pkcs11.is_ok());
    assert!(cloud_kms.is_ok());
    assert!(network.is_ok());
    assert!(usb.is_ok());
    assert!(software.is_ok());
    assert!(mobile.is_ok());
    assert!(tpm.is_ok());
    assert!(smartcard.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_discovery_engine_full_scan() -> Result<(), Box<dyn std::error::Error>> {
    let engine = DiscoveryEngine::new()?;

    let pkcs11_result = engine.discover_pkcs11_hsms();
    let cloud_result = engine.discover_cloud_kms_hsms();
    let network_result = engine.discover_network_hsms();
    let usb_result = engine.discover_usb_hsms();
    let software_result = engine.discover_software_hsms();
    let mobile_result = engine.discover_mobile_hsms();
    let tpm_result = engine.discover_tpm_hsms();
    let smartcard_result = engine.discover_smartcard_hsms();

    assert!(pkcs11_result.is_ok());
    assert!(cloud_result.is_ok());
    assert!(network_result.is_ok());
    assert!(usb_result.is_ok());
    assert!(software_result.is_ok());
    assert!(mobile_result.is_ok());
    assert!(tpm_result.is_ok());
    assert!(smartcard_result.is_ok());

    Ok(())
}

#[test]
fn test_network_scan_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = NetworkScanConfig {
        ip_ranges: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
        timeout_ms: 5000,
        parallel_scans: 10,
    };

    assert_eq!(config.ip_ranges.len(), 2);
    assert_eq!(config.timeout_ms, 5000);
    assert_eq!(config.parallel_scans, 10);

    Ok(())
}

#[test]
fn test_usb_enumeration_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = UsbEnumerationConfig {
        scan_interval_ms: 1000,
        auto_detect: true,
    };

    assert_eq!(config.scan_interval_ms, 1000);
    assert!(config.auto_detect);

    Ok(())
}

#[test]
fn test_tpm_interface_types() -> Result<(), Box<dyn std::error::Error>> {
    let types = [
        TpmInterfaceType::Tpm12,
        TpmInterfaceType::Tpm20,
        TpmInterfaceType::FirmwareTpm,
        TpmInterfaceType::SoftwareTpm,
    ];

    assert_eq!(types.len(), 4);

    Ok(())
}

#[test]
fn test_software_hsm_implementations() -> Result<(), Box<dyn std::error::Error>> {
    let impls = vec![
        SoftwareHsmImplementation::BearDogNative,
        SoftwareHsmImplementation::OpenSsl,
        SoftwareHsmImplementation::SoftHsm,
        SoftwareHsmImplementation::MicrosoftCng,
        SoftwareHsmImplementation::MacOsKeychain,
        SoftwareHsmImplementation::Custom {
            name: "CustomHSM".to_string(),
            path: PathBuf::from("/opt/custom-hsm"),
        },
    ];

    assert_eq!(impls.len(), 6);

    Ok(())
}

#[tokio::test]
async fn test_concurrent_discovery_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = tokio::spawn(async { DiscoveryEngine::new() });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await?;
        assert!(result.is_ok());
    }

    Ok(())
}

#[test]
fn test_pkcs11_search_paths() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = Pkcs11Discoverer::new()?;
    assert!(!discoverer.search_paths.is_empty());
    Ok(())
}

#[test]
fn test_cloud_kms_providers() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = CloudKmsDiscoverer::new()?;
    assert!(!discoverer._enabled_providers.is_empty());
    Ok(())
}

#[test]
fn test_network_hsm_ports() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = NetworkHsmDiscoverer::new()?;
    assert!(!discoverer._common_ports.is_empty());
    Ok(())
}

#[test]
fn test_usb_hsm_vendor_ids() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = UsbHsmDiscoverer::new()?;
    assert!(!discoverer._hsm_vendor_ids.is_empty());
    Ok(())
}

#[test]
fn test_software_hsm_implementations_list() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = SoftwareHsmDiscoverer::new()?;
    assert!(!discoverer.implementations.is_empty());
    Ok(())
}

#[test]
fn test_tpm_interface_types_list() -> Result<(), Box<dyn std::error::Error>> {
    let discoverer = TpmDiscoverer::new()?;
    assert!(!discoverer._interface_types.is_empty());
    Ok(())
}

#[test]
fn test_network_scan_config_clone() -> Result<(), Box<dyn std::error::Error>> {
    const TEST_TIMEOUT_MS: u32 = 3000;
    let config1 = NetworkScanConfig {
        ip_ranges: vec!["192.168.1.0/24".to_string()],
        timeout_ms: TEST_TIMEOUT_MS,
        parallel_scans: 5,
    };

    let config2 = config1.clone();

    assert_eq!(config1.ip_ranges, config2.ip_ranges);
    assert_eq!(config1.timeout_ms, config2.timeout_ms);
    assert_eq!(config1.parallel_scans, config2.parallel_scans);

    Ok(())
}

#[test]
fn test_usb_config_clone() -> Result<(), Box<dyn std::error::Error>> {
    let config1 = UsbEnumerationConfig {
        scan_interval_ms: 2000,
        auto_detect: false,
    };

    let config2 = config1.clone();

    assert_eq!(config1.scan_interval_ms, config2.scan_interval_ms);
    assert_eq!(config1.auto_detect, config2.auto_detect);

    Ok(())
}

#[test]
fn test_custom_software_hsm() -> Result<(), Box<dyn std::error::Error>> {
    let custom = SoftwareHsmImplementation::Custom {
        name: "MyCustomHSM".to_string(),
        path: PathBuf::from("/usr/local/lib/custom-hsm.so"),
    };

    if let SoftwareHsmImplementation::Custom { name, path } = custom {
        assert_eq!(name, "MyCustomHSM");
        assert_eq!(path, PathBuf::from("/usr/local/lib/custom-hsm.so"));
    } else {
        panic!("Expected Custom variant");
    }

    Ok(())
}

#[tokio::test]
async fn test_discovery_engine_repeated_scans() -> Result<(), Box<dyn std::error::Error>> {
    let engine = DiscoveryEngine::new()?;

    for _ in 0..3 {
        let software_result = engine.discover_software_hsms();
        assert!(software_result.is_ok());
    }

    Ok(())
}

#[test]
fn test_network_scan_config_with_empty_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let config = NetworkScanConfig {
        ip_ranges: Vec::new(),
        timeout_ms: 5000,
        parallel_scans: 10,
    };

    assert!(config.ip_ranges.is_empty());

    Ok(())
}

#[test]
fn test_network_scan_config_with_many_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let ranges: Vec<String> = (0..10).map(|i| format!("192.168.{}.0/24", i)).collect();

    let config = NetworkScanConfig {
        ip_ranges: ranges.clone(),
        timeout_ms: 5000,
        parallel_scans: 20,
    };

    assert_eq!(config.ip_ranges.len(), 10);

    Ok(())
}

#[test]
fn test_usb_config_variations() -> Result<(), Box<dyn std::error::Error>> {
    let configs = vec![
        UsbEnumerationConfig {
            scan_interval_ms: 100,
            auto_detect: true,
        },
        UsbEnumerationConfig {
            scan_interval_ms: 5000,
            auto_detect: false,
        },
        UsbEnumerationConfig {
            scan_interval_ms: 0,
            auto_detect: true,
        },
    ];

    for config in configs {
        let _ = config.scan_interval_ms;
        let _ = config.auto_detect;
    }

    Ok(())
}

#[test]
fn test_software_hsm_implementation_variants() {
    let v = vec![
        SoftwareHsmImplementation::BearDogNative,
        SoftwareHsmImplementation::OpenSsl,
        SoftwareHsmImplementation::SoftHsm,
        SoftwareHsmImplementation::MicrosoftCng,
        SoftwareHsmImplementation::MacOsKeychain,
        SoftwareHsmImplementation::Custom {
            name: "x".to_string(),
            path: PathBuf::from("/tmp/lib.so"),
        },
    ];
    for s in v {
        let dbg = format!("{s:?}");
        assert!(!dbg.is_empty());
    }
}

#[test]
fn test_tpm_interface_type_variants() {
    for t in [
        TpmInterfaceType::Tpm12,
        TpmInterfaceType::Tpm20,
        TpmInterfaceType::FirmwareTpm,
        TpmInterfaceType::SoftwareTpm,
    ] {
        let dbg = format!("{t:?}");
        assert!(!dbg.is_empty());
    }
}

#[test]
fn pkcs11_identify_vendor_from_library_branches() {
    let (v, m) = Pkcs11Discoverer::identify_vendor_from_library("libeToken.so");
    assert_eq!(v, "SafeNet");
    assert_eq!(m, "eToken");
    let (v2, m2) = Pkcs11Discoverer::identify_vendor_from_library("libLunaAPI.so");
    assert_eq!(v2, "Thales");
    assert_eq!(m2, "Luna HSM");
    let (v3, _) = Pkcs11Discoverer::identify_vendor_from_library("opensc-pkcs11.so");
    assert_eq!(v3, "OpenSC");
}
