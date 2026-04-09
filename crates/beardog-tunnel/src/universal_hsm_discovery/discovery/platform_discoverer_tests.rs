// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_discoverer_creation() {
    let discoverer = PlatformDiscoverer::new();
    assert!(discoverer.is_ok());

    let disc = discoverer.expect("platform discoverer");
    assert!(disc.enable_tpm_probe);
    assert!(disc.enable_tee_probe);
}

#[tokio::test]
async fn test_platform_discovery() {
    let discoverer = PlatformDiscoverer::new()?;
    let result = discoverer.discover().await;
    assert!(result.is_ok());
    
    // Discovery should succeed even if no HSMs are found
    let hsms = result?;
    // The number of HSMs found depends on the platform
    assert!(hsms.len() <= 4); // At most TPM, TEE, and platform-specific
}

#[tokio::test]
async fn test_tpm_capabilities() {
    let discoverer = PlatformDiscoverer::new()?;
    let caps = discoverer.create_tpm_capabilities();
    
    // Verify TPM capabilities
    assert!(caps.key_management.key_generation);
    assert!(caps.key_management.key_storage);
    assert!(caps.security.tamper_resistance == TamperResistance::Tier2);
    assert!(caps.security.fips_140_2_level == Some(2));
    assert!(!caps.human_entropy.supported);
}

#[tokio::test]
async fn test_tee_capabilities() {
    let discoverer = PlatformDiscoverer::new()?;
    let caps = discoverer.create_tee_capabilities();
    
    // Verify TEE capabilities
    assert!(caps.key_management.key_generation);
    assert!(caps.performance.max_operations_per_second > 1000);
    assert!(caps.security.secure_boot);
    assert!(!caps.human_entropy.supported);
}

// ========== Additional Comprehensive Tests ==========

#[tokio::test]
async fn test_tpm_discovery_with_disabled_probe() {
    let mut discoverer = PlatformDiscoverer::new()?;
    discoverer.enable_tpm_probe = false;
    
    let hsms = discoverer.discover().await?;
    let tpm_count = hsms.iter().filter(|h| h.name.contains("TPM")).count();
    assert_eq!(tpm_count, 0, "Should not discover TPM when probe disabled");
}

#[tokio::test]
async fn test_tee_discovery_with_disabled_probe() {
    let mut discoverer = PlatformDiscoverer::new()?;
    discoverer.enable_tee_probe = false;
    
    let hsms = discoverer.discover().await?;
    let tee_count = hsms.iter().filter(|h| h.name.contains("TEE")).count();
    assert_eq!(tee_count, 0, "Should not discover TEE when probe disabled");
}

#[tokio::test]
async fn test_all_hsms_have_valid_timestamps() {
    let discoverer = PlatformDiscoverer::new()?;
    let hsms = discoverer.discover().await?;
    
    for hsm in &hsms {
        assert!(hsm.discovered_at <= Utc::now());
        assert!(!hsm.id.is_empty());
        assert!(!hsm.name.is_empty());
    }
}

#[tokio::test]
async fn test_concurrent_discoveries() {
    use std::sync::Arc;
    let discoverer = Arc::new(PlatformDiscoverer::new()?);
    
    let mut handles = vec![];
    for _ in 0..3 {
        let disc = Arc::clone(&discoverer);
        handles.push(tokio::spawn(async move {
            disc.discover().await
        }));
    }
    
    for handle in handles {
        let result = handle.await?;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_discovery_deterministic() {
    let discoverer = PlatformDiscoverer::new()?;
    
    let hsms1 = discoverer.discover().await?;
    let hsms2 = discoverer.discover().await?;
    
    assert_eq!(hsms1.len(), hsms2.len(), "Discovery should be deterministic");
}

#[test]
fn test_default_implementation() {
    let discoverer = PlatformDiscoverer::default();
    assert!(discoverer.enable_tpm_probe);
    assert!(discoverer.enable_tee_probe);
}

#[test]
fn test_clone_implementation() {
    let discoverer1 = PlatformDiscoverer::new().expect("platform discoverer");
    let discoverer2 = discoverer1.clone();
    
    assert_eq!(discoverer1.enable_tpm_probe, discoverer2.enable_tpm_probe);
    assert_eq!(discoverer1.enable_tee_probe, discoverer2.enable_tee_probe);
}
