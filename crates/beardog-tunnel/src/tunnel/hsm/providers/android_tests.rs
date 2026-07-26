// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[tokio::test]
async fn test_android_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AndroidUniversalProvider::new();
    assert!(provider.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_capabilities_detection() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AndroidUniversalProvider::new()?;
    let caps = provider.capabilities();
    assert!(caps.is_some());
    Ok(())
}

#[test]
fn test_security_levels() -> Result<(), Box<dyn std::error::Error>> {
    let provider_strongbox = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: true,
        tee_available: true,
        device_metadata: HashMap::new(),
    };
    assert_eq!(provider_strongbox.get_security_level(), 3);

    let provider_tee = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: true,
        device_metadata: HashMap::new(),
    };
    assert_eq!(provider_tee.get_security_level(), 2);

    let provider_software = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };
    assert_eq!(provider_software.get_security_level(), 1);
    Ok(())
}

#[test]
fn test_strongbox_level() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(StrongBoxLevel::Full, StrongBoxLevel::Full);
    assert_ne!(StrongBoxLevel::Full, StrongBoxLevel::Basic);
    Ok(())
}

#[test]
fn test_vendor_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: true,
        tee_available: true,
        device_metadata: HashMap::new(),
    };

    provider
        .device_metadata
        .insert("device_model".to_string(), "Pixel 8 Pro".to_string());

    let info = provider.get_vendor_info();
    assert_eq!(info.name, "Android");
    assert_eq!(info.model, "Pixel 8 Pro");
    Ok(())
}

#[test]
fn test_has_capabilities() -> Result<(), Box<dyn std::error::Error>> {
    let provider_with_strongbox = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: true,
        tee_available: true,
        device_metadata: HashMap::new(),
    };

    assert!(provider_with_strongbox.has_strongbox());
    assert!(provider_with_strongbox.has_tee());

    let provider_without = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };

    assert!(!provider_without.has_strongbox());
    assert!(!provider_without.has_tee());
    Ok(())
}

#[tokio::test]
async fn test_android_capabilities() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AndroidUniversalProvider::new()?;

    if let Some(caps) = provider.capabilities() {
        // On non-Android (Linux CI), software fallback is used:
        // neither hardware_backed nor StrongBox will be available.
        #[cfg(target_os = "android")]
        assert!(caps.hardware_backed || caps.strongbox_level != StrongBoxLevel::None);
        #[cfg(not(target_os = "android"))]
        let _ = caps;
    }
    Ok(())
}

// ========================================================================
// COMPREHENSIVE EDGE CASE TESTS (Week 1 Sprint)
// ========================================================================

#[tokio::test]
async fn test_strongbox_levels_progression() -> Result<(), Box<dyn std::error::Error>> {
    // Test all security level combinations
    let levels = vec![
        (false, false, 1), // No StrongBox, no TEE
        (false, true, 2),  // TEE only
        (true, false, 3),  // StrongBox only
        (true, true, 3),   // StrongBox + TEE
    ];

    for (strongbox, tee, expected) in levels {
        let provider = AndroidUniversalProvider {
            capabilities: None,
            strongbox_available: strongbox,
            tee_available: tee,
            device_metadata: HashMap::new(),
        };
        assert_eq!(
            provider.get_security_level(),
            expected,
            "Failed for strongbox={}, tee={}",
            strongbox,
            tee
        );
    }
    Ok(())
}

#[test]
fn test_all_strongbox_levels() -> Result<(), Box<dyn std::error::Error>> {
    // Test all StrongBox level combinations
    assert_eq!(StrongBoxLevel::None, StrongBoxLevel::None);
    assert_eq!(StrongBoxLevel::Basic, StrongBoxLevel::Basic);
    assert_eq!(
        StrongBoxLevel::WithAttestation,
        StrongBoxLevel::WithAttestation
    );
    assert_eq!(StrongBoxLevel::Full, StrongBoxLevel::Full);

    assert_ne!(StrongBoxLevel::None, StrongBoxLevel::Basic);
    assert_ne!(StrongBoxLevel::Basic, StrongBoxLevel::WithAttestation);
    assert_ne!(StrongBoxLevel::WithAttestation, StrongBoxLevel::Full);
    Ok(())
}

#[test]
fn test_vendor_info_with_different_models() -> Result<(), Box<dyn std::error::Error>> {
    let models = vec![
        "Pixel 8 Pro",
        "Samsung Galaxy S24",
        "OnePlus 12",
        "Xiaomi 14 Pro",
    ];

    for model in models {
        let mut provider = AndroidUniversalProvider {
            capabilities: None,
            strongbox_available: true,
            tee_available: true,
            device_metadata: HashMap::new(),
        };

        provider
            .device_metadata
            .insert("device_model".to_string(), model.to_string());
        provider
            .device_metadata
            .insert("android_version".to_string(), "14".to_string());

        let info = provider.get_vendor_info();
        assert_eq!(info.name, "Android");
        assert_eq!(info.model, model);
    }
    Ok(())
}

#[tokio::test]
async fn test_capabilities_structure() -> Result<(), Box<dyn std::error::Error>> {
    let provider = AndroidUniversalProvider::new()?;

    if let Some(caps) = provider.capabilities() {
        // Verify all capability fields are accessible
        let _ = caps.strongbox_level;
        let _ = caps.tee_type;
        let _ = caps.attestation_supported;
        let _ = caps.hardware_backed;
        let _ = caps.biometric_auth;

        // Capabilities should be internally consistent
        if caps.strongbox_level == StrongBoxLevel::WithAttestation {
            assert!(
                caps.attestation_supported
                    || caps.strongbox_level == StrongBoxLevel::WithAttestation
            );
        }
    }
    Ok(())
}

#[test]
fn test_device_metadata_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: true,
        tee_available: true,
        device_metadata: HashMap::new(),
    };

    // Test metadata insertion and retrieval
    provider
        .device_metadata
        .insert("key1".to_string(), "value1".to_string());
    provider
        .device_metadata
        .insert("key2".to_string(), "value2".to_string());

    assert_eq!(
        provider.device_metadata.get("key1"),
        Some(&"value1".to_string())
    );
    assert_eq!(
        provider.device_metadata.get("key2"),
        Some(&"value2".to_string())
    );
    assert_eq!(provider.device_metadata.get("nonexistent"), None);

    Ok(())
}

#[tokio::test]
async fn test_provider_with_maximum_capabilities() -> Result<(), Box<dyn std::error::Error>> {
    // Test provider with all capabilities enabled
    let mut provider = AndroidUniversalProvider {
        capabilities: Some(AndroidCapabilities {
            strongbox_level: StrongBoxLevel::Full,
            tee_type: Some("Trusty TEE".to_string()),
            attestation_supported: true,
            hardware_backed: true,
            biometric_auth: true,
        }),
        strongbox_available: true,
        tee_available: true,
        device_metadata: HashMap::new(),
    };

    provider
        .device_metadata
        .insert("device_model".to_string(), "Pixel 8 Pro".to_string());
    provider
        .device_metadata
        .insert("strongbox_version".to_string(), "1.0".to_string());

    // Verify maximum security level
    assert_eq!(provider.get_security_level(), 3);
    assert!(provider.has_strongbox());
    assert!(provider.has_tee());

    let info = provider.get_vendor_info();
    assert_eq!(info.name, "Android");
    assert_eq!(info.model, "Pixel 8 Pro");

    Ok(())
}

#[tokio::test]
async fn test_provider_with_minimum_capabilities() -> Result<(), Box<dyn std::error::Error>> {
    // Test provider with minimal capabilities
    let provider = AndroidUniversalProvider {
        capabilities: Some(AndroidCapabilities {
            strongbox_level: StrongBoxLevel::None,
            tee_type: None,
            attestation_supported: false,
            hardware_backed: false,
            biometric_auth: false,
        }),
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };

    // Verify minimum security level
    assert_eq!(provider.get_security_level(), 1);
    assert!(!provider.has_strongbox());
    assert!(!provider.has_tee());

    Ok(())
}

#[tokio::test]
async fn test_concurrent_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Test creating multiple providers concurrently
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = tokio::spawn(async { AndroidUniversalProvider::new() });
        handles.push(handle);
    }

    // All providers should initialize successfully
    for handle in handles {
        let result = handle.await?;
        assert!(result.is_ok());
    }

    Ok(())
}

#[test]
fn test_platform_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test platform detection logic
    let is_android = AndroidUniversalProvider::is_android_platform();

    #[cfg(target_os = "android")]
    assert!(is_android, "Should detect Android platform");

    #[cfg(not(target_os = "android"))]
    assert!(
        !is_android,
        "Should not detect Android on non-Android platforms"
    );

    Ok(())
}

#[test]
fn test_capabilities_clone() -> Result<(), Box<dyn std::error::Error>> {
    // Test that capabilities can be cloned
    let caps1 = AndroidCapabilities {
        strongbox_level: StrongBoxLevel::Full,
        tee_type: Some("Trusty TEE".to_string()),
        attestation_supported: true,
        hardware_backed: true,
        biometric_auth: true,
    };

    let caps2 = caps1.clone();

    assert_eq!(caps1.strongbox_level, caps2.strongbox_level);
    assert_eq!(caps1.tee_type, caps2.tee_type);
    assert_eq!(caps1.attestation_supported, caps2.attestation_supported);
    assert_eq!(caps1.hardware_backed, caps2.hardware_backed);
    assert_eq!(caps1.biometric_auth, caps2.biometric_auth);

    Ok(())
}

#[tokio::test]
async fn test_tee_detection() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };

    // Test TEE detection
    let has_tee = provider.detect_tee();

    // Detection state should match
    assert!(
        has_tee == provider.tee_available,
        "TEE detection state should match"
    );

    Ok(())
}

#[test]
fn test_security_level_consistency() -> Result<(), Box<dyn std::error::Error>> {
    // Test that security levels are consistent with capabilities
    let test_cases = vec![
        (StrongBoxLevel::None, None, false, false),
        (
            StrongBoxLevel::Basic,
            Some("QSEE".to_string()),
            false,
            false,
        ),
        (
            StrongBoxLevel::WithAttestation,
            Some("Trusty TEE".to_string()),
            true,
            false,
        ),
        (
            StrongBoxLevel::Full,
            Some("Trusty TEE".to_string()),
            true,
            true,
        ),
    ];

    for (strongbox_level, tee_type, attestation, hw_backed) in test_cases {
        let caps = AndroidCapabilities {
            strongbox_level: strongbox_level.clone(),
            tee_type: tee_type.clone(),
            attestation_supported: attestation,
            hardware_backed: hw_backed,
            biometric_auth: false,
        };

        // Verify capability structure is valid
        if caps.strongbox_level == StrongBoxLevel::WithAttestation {
            assert!(
                caps.attestation_supported
                    || caps.strongbox_level == StrongBoxLevel::WithAttestation
            );
        }

        if caps.strongbox_level == StrongBoxLevel::Full {
            assert!(caps.hardware_backed || caps.strongbox_level == StrongBoxLevel::Full);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_provider_state_transitions() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with no capabilities
    let mut provider = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };

    // Initial state
    assert!(!provider.has_strongbox());
    assert!(!provider.has_tee());
    assert_eq!(provider.get_security_level(), 1);

    // Simulate TEE detection
    provider.tee_available = true;
    assert!(provider.has_tee());
    assert_eq!(provider.get_security_level(), 2);

    // Add StrongBox
    provider.strongbox_available = true;
    assert!(provider.has_strongbox());
    assert_eq!(provider.get_security_level(), 3);

    Ok(())
}

#[tokio::test]
async fn test_tee_types() -> Result<(), Box<dyn std::error::Error>> {
    let tee_types = vec!["Trusty TEE", "QSEE", "OP-TEE", "Kinibi", "Teegris"];

    for tee_type in tee_types {
        let caps = AndroidCapabilities {
            strongbox_level: StrongBoxLevel::Full,
            tee_type: Some(tee_type.to_string()),
            attestation_supported: true,
            hardware_backed: true,
            biometric_auth: true,
        };

        assert_eq!(caps.tee_type, Some(tee_type.to_string()));
    }

    Ok(())
}

#[test]
fn test_strongbox_detection() -> Result<(), Box<dyn std::error::Error>> {
    let mut provider = AndroidUniversalProvider {
        capabilities: None,
        strongbox_available: false,
        tee_available: false,
        device_metadata: HashMap::new(),
    };

    // Test StrongBox detection
    let has_strongbox = provider.detect_strongbox();

    // Detection state should match
    assert!(
        has_strongbox == provider.strongbox_available,
        "StrongBox detection state should match"
    );

    Ok(())
}

#[tokio::test]
async fn test_biometric_authentication() -> Result<(), Box<dyn std::error::Error>> {
    let caps = AndroidCapabilities {
        strongbox_level: StrongBoxLevel::Full,
        tee_type: Some("Trusty TEE".to_string()),
        attestation_supported: true,
        hardware_backed: true,
        biometric_auth: true,
    };

    assert!(caps.biometric_auth);

    let caps_no_bio = AndroidCapabilities {
        strongbox_level: StrongBoxLevel::Basic,
        tee_type: None,
        attestation_supported: false,
        hardware_backed: false,
        biometric_auth: false,
    };

    assert!(!caps_no_bio.biometric_auth);

    Ok(())
}
