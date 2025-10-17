//! Comprehensive tests for CapabilityType enum
//!
//! Tests the core CapabilityType enum and its methods.

use crate::canonical::capabilities::CapabilityType;
use std::collections::{HashMap, HashSet};

#[test]
fn test_capability_type_creation() {
    let cap = CapabilityType::KeyManagement;
    assert_eq!(cap, CapabilityType::KeyManagement);

    let hsm = CapabilityType::HardwareSecurityModule;
    assert_eq!(hsm, CapabilityType::HardwareSecurityModule);
}

#[test]
fn test_capability_type_clone() {
    let cap = CapabilityType::ComputeIntelligence;
    let cap_clone = cap.clone();
    assert_eq!(cap, cap_clone);
}

#[test]
fn test_capability_type_hash() {
    let mut caps = HashSet::new();
    caps.insert(CapabilityType::KeyManagement);
    caps.insert(CapabilityType::ServiceMesh);
    caps.insert(CapabilityType::KeyManagement); // Duplicate

    assert_eq!(caps.len(), 2);
    assert!(caps.contains(&CapabilityType::KeyManagement));
    assert!(caps.contains(&CapabilityType::ServiceMesh));
}

#[test]
fn test_capability_type_name() {
    let kms_name = CapabilityType::KeyManagement.name();
    assert!(!kms_name.is_empty());

    let hsm_name = CapabilityType::HardwareSecurityModule.name();
    assert!(!hsm_name.is_empty());

    let mesh_name = CapabilityType::ServiceMesh.name();
    assert!(!mesh_name.is_empty());

    let compute_name = CapabilityType::ComputeIntelligence.name();
    assert!(!compute_name.is_empty());
}

#[test]
fn test_capability_type_is_vendor() {
    // Test vendor capabilities
    assert!(CapabilityType::KeyManagement.is_vendor_capability());
    assert!(CapabilityType::HardwareSecurityModule.is_vendor_capability());
    assert!(CapabilityType::SecretsManagement.is_vendor_capability());
    assert!(CapabilityType::Authentication.is_vendor_capability());

    // Test primal capabilities are not vendor
    assert!(!CapabilityType::ServiceMesh.is_vendor_capability());
    assert!(!CapabilityType::ComputeIntelligence.is_vendor_capability());
}

#[test]
fn test_capability_type_is_primal() {
    // Test primal capabilities
    assert!(CapabilityType::ServiceMesh.is_primal_capability());
    assert!(CapabilityType::ComputeIntelligence.is_primal_capability());

    // Test vendor capabilities are not primal
    assert!(!CapabilityType::KeyManagement.is_primal_capability());
    assert!(!CapabilityType::HardwareSecurityModule.is_primal_capability());
}

#[test]
fn test_capability_type_as_id() {
    let kms_id = CapabilityType::KeyManagement.as_capability_id();
    assert!(!kms_id.is_empty());

    let hsm_id = CapabilityType::HardwareSecurityModule.as_capability_id();
    assert!(!hsm_id.is_empty());

    // IDs should be unique
    assert_ne!(kms_id, hsm_id);
}

#[test]
fn test_capability_type_display() {
    let cap = CapabilityType::KeyManagement;
    let display_string = format!("{}", cap);
    assert!(!display_string.is_empty());
}

#[test]
fn test_capability_type_from_string() {
    let cap = CapabilityType::ServiceMesh;
    let cap_string: String = cap.into();
    assert!(!cap_string.is_empty());
}

#[test]
fn test_capability_type_serialization() {
    let cap = CapabilityType::KeyManagement;
    let json = serde_json::to_string(&cap).expect("Should serialize");
    assert!(!json.is_empty());

    let deserialized: CapabilityType = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(cap, deserialized);
}

#[test]
fn test_capability_type_all_major_variants() {
    // Ensure all major capability types can be created and are distinct
    let capabilities = vec![
        CapabilityType::KeyManagement,
        CapabilityType::HardwareSecurityModule,
        CapabilityType::SecretsManagement,
        CapabilityType::Authentication,
        CapabilityType::CloudStorage,
        CapabilityType::ServiceMesh,
        CapabilityType::ComputeIntelligence,
        CapabilityType::Monitoring,
        CapabilityType::Logging,
        CapabilityType::Metrics,
    ];

    // All should have unique names
    let names: Vec<String> = capabilities.iter().map(CapabilityType::name).collect();
    let unique_names: HashSet<_> = names.iter().collect();
    assert_eq!(
        names.len(),
        unique_names.len(),
        "All capability names should be unique"
    );
}

#[test]
fn test_capability_discovery_pattern() {
    // Simulate capability discovery pattern
    let mut discovered_capabilities = HashMap::new();

    // Discover multiple capabilities
    discovered_capabilities.insert(
        "kms-provider".to_string(),
        vec![
            CapabilityType::KeyManagement,
            CapabilityType::SecretsManagement,
        ],
    );

    discovered_capabilities.insert(
        "mesh-provider".to_string(),
        vec![CapabilityType::ServiceMesh],
    );

    // Verify discovery
    assert_eq!(discovered_capabilities.len(), 2);
    assert!(discovered_capabilities.contains_key("kms-provider"));
    assert_eq!(
        discovered_capabilities.get("kms-provider").unwrap().len(),
        2
    );
}

#[test]
fn test_capability_filtering() {
    let capabilities = [
        CapabilityType::KeyManagement,          // Vendor
        CapabilityType::ServiceMesh,            // Primal
        CapabilityType::Monitoring,             // Cross-cutting
        CapabilityType::HardwareSecurityModule, // Vendor
    ];

    // Filter for vendor capabilities
    let vendor_caps: Vec<_> = capabilities
        .iter()
        .filter(|c| c.is_vendor_capability())
        .collect();
    assert!(!vendor_caps.is_empty());
    assert_eq!(vendor_caps.len(), 2);

    // Filter for primal capabilities
    let primal_caps: Vec<_> = capabilities
        .iter()
        .filter(|c| c.is_primal_capability())
        .collect();
    assert!(!primal_caps.is_empty());
}

#[test]
fn test_capability_type_equality() {
    let cap1 = CapabilityType::KeyManagement;
    let cap2 = CapabilityType::KeyManagement;
    let cap3 = CapabilityType::ServiceMesh;

    assert_eq!(cap1, cap2);
    assert_ne!(cap1, cap3);
    assert_ne!(cap2, cap3);
}

#[test]
fn test_capability_type_in_hashmap() {
    let mut cap_map = HashMap::new();
    cap_map.insert(CapabilityType::KeyManagement, "KMS Provider");
    cap_map.insert(CapabilityType::ServiceMesh, "Mesh Provider");

    assert_eq!(
        cap_map.get(&CapabilityType::KeyManagement),
        Some(&"KMS Provider")
    );
    assert_eq!(
        cap_map.get(&CapabilityType::ServiceMesh),
        Some(&"Mesh Provider")
    );
    assert_eq!(cap_map.len(), 2);
}

#[test]
fn test_capability_type_multiple_instances() {
    // Test that we can create many instances without issues
    let capabilities: Vec<CapabilityType> = (0..100)
        .map(|i| match i % 5 {
            0 => CapabilityType::KeyManagement,
            1 => CapabilityType::ServiceMesh,
            2 => CapabilityType::HardwareSecurityModule,
            3 => CapabilityType::Monitoring,
            _ => CapabilityType::Logging,
        })
        .collect();

    // Should not panic or have memory issues
    assert_eq!(capabilities.len(), 100);
}

#[test]
fn test_capability_type_debug() {
    let cap = CapabilityType::KeyManagement;
    let debug_string = format!("{:?}", cap);
    assert!(!debug_string.is_empty());
}

#[test]
fn test_vendor_vs_primal_separation() {
    let vendor_caps = vec![
        CapabilityType::KeyManagement,
        CapabilityType::HardwareSecurityModule,
        CapabilityType::SecretsManagement,
        CapabilityType::Authentication,
        CapabilityType::CloudStorage,
        CapabilityType::DatabaseService,
    ];

    let primal_caps = vec![
        CapabilityType::ServiceMesh,
        CapabilityType::ComputeIntelligence,
    ];

    // All vendor caps should be vendor
    for cap in &vendor_caps {
        assert!(
            cap.is_vendor_capability(),
            "{} should be vendor",
            cap.name()
        );
        assert!(
            !cap.is_primal_capability(),
            "{} should not be primal",
            cap.name()
        );
    }

    // All primal caps should be primal
    for cap in &primal_caps {
        assert!(
            cap.is_primal_capability(),
            "{} should be primal",
            cap.name()
        );
        assert!(
            !cap.is_vendor_capability(),
            "{} should not be vendor",
            cap.name()
        );
    }
}

#[test]
fn test_capability_id_format() {
    let cap = CapabilityType::KeyManagement;
    let id = cap.as_capability_id();

    // ID should not be empty and should be a valid format
    assert!(!id.is_empty());
    // Should be lowercase or have some consistent format
    assert!(id.len() > 3, "Capability ID should be meaningful");
}

#[test]
fn test_capability_type_collection_operations() {
    let mut capability_set = HashSet::new();

    // Add various capabilities
    capability_set.insert(CapabilityType::KeyManagement);
    capability_set.insert(CapabilityType::ServiceMesh);
    capability_set.insert(CapabilityType::Monitoring);

    // Test set operations
    assert!(capability_set.contains(&CapabilityType::KeyManagement));
    assert!(capability_set.contains(&CapabilityType::ServiceMesh));
    assert!(!capability_set.contains(&CapabilityType::HardwareSecurityModule));

    capability_set.remove(&CapabilityType::Monitoring);
    assert!(!capability_set.contains(&CapabilityType::Monitoring));
    assert_eq!(capability_set.len(), 2);
}
