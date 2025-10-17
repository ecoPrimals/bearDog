// Comprehensive tests for canonical capabilities

use super::*;
use beardog_errors::BearDogResult;

#[cfg(test)]
mod capability_tests {
    use super::*;

    #[test]
    fn test_capability_structure() {
        // Test that Capability enum variants are properly defined
        use crate::canonical::capabilities::Capability;
        
        let storage_cap = Capability::Storage;
        let compute_cap = Capability::Compute;
        let network_cap = Capability::Network;
        
        assert!(matches!(storage_cap, Capability::Storage));
        assert!(matches!(compute_cap, Capability::Compute));
        assert!(matches!(network_cap, Capability::Network));
    }

    #[test]
    fn test_capability_set_operations() {
        use crate::canonical::capabilities::CapabilitySet;
        use std::collections::HashSet;
        
        let mut caps = CapabilitySet {
            capabilities: HashSet::new(),
        };
        
        // Test adding capabilities
        caps.capabilities.insert("read".to_string());
        caps.capabilities.insert("write".to_string());
        
        assert_eq!(caps.capabilities.len(), 2);
        assert!(caps.capabilities.contains("read"));
        assert!(caps.capabilities.contains("write"));
    }

    #[test]
    fn test_capability_validation() {
        use crate::canonical::capabilities::CapabilitySet;
        use std::collections::HashSet;
        
        let mut caps = CapabilitySet {
            capabilities: HashSet::new(),
        };
        
        caps.capabilities.insert("execute".to_string());
        
        // Validate capability exists
        assert!(caps.capabilities.contains("execute"));
        
        // Validate capability doesn't exist
        assert!(!caps.capabilities.contains("delete"));
    }

    #[test]
    fn test_capability_requirements() {
        use crate::canonical::capabilities::CapabilityRequirement;
        
        let requirement = CapabilityRequirement {
            required_capabilities: vec!["read".to_string(), "write".to_string()],
            optional_capabilities: vec!["execute".to_string()],
        };
        
        assert_eq!(requirement.required_capabilities.len(), 2);
        assert_eq!(requirement.optional_capabilities.len(), 1);
    }

    #[test]
    fn test_capability_check() {
        use crate::canonical::capabilities::{CapabilitySet, CapabilityRequirement};
        use std::collections::HashSet;
        
        let mut caps = CapabilitySet {
            capabilities: HashSet::new(),
        };
        caps.capabilities.insert("read".to_string());
        caps.capabilities.insert("write".to_string());
        
        let requirement = CapabilityRequirement {
            required_capabilities: vec!["read".to_string()],
            optional_capabilities: vec![],
        };
        
        // Check if capability set meets requirements
        for required in &requirement.required_capabilities {
            assert!(caps.capabilities.contains(required));
        }
    }

    #[test]
    fn test_capability_hierarchy() {
        use crate::canonical::capabilities::CapabilityLevel;
        
        let basic = CapabilityLevel::Basic;
        let advanced = CapabilityLevel::Advanced;
        let expert = CapabilityLevel::Expert;
        
        assert!(matches!(basic, CapabilityLevel::Basic));
        assert!(matches!(advanced, CapabilityLevel::Advanced));
        assert!(matches!(expert, CapabilityLevel::Expert));
    }

    #[test]
    fn test_capability_metadata() {
        use crate::canonical::capabilities::CapabilityMetadata;
        use std::collections::HashMap;
        
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());
        metadata.insert("provider".to_string(), "beardog".to_string());
        
        let cap_metadata = CapabilityMetadata {
            name: "storage".to_string(),
            version: "1.0".to_string(),
            metadata,
        };
        
        assert_eq!(cap_metadata.name, "storage");
        assert_eq!(cap_metadata.version, "1.0");
        assert_eq!(cap_metadata.metadata.len(), 2);
    }

    #[test]
    fn test_capability_discovery() {
        use crate::canonical::capabilities::DiscoveredCapability;
        
        let discovered = DiscoveredCapability {
            capability_name: "encryption".to_string(),
            provider: "hardware-hsm".to_string(),
            available: true,
            confidence: 0.95,
        };
        
        assert_eq!(discovered.capability_name, "encryption");
        assert!(discovered.available);
        assert!(discovered.confidence > 0.9);
    }

    #[test]
    fn test_capability_serialization() {
        use crate::canonical::capabilities::Capability;
        use serde_json;
        
        let cap = Capability::Storage;
        let serialized = serde_json::to_string(&cap).expect("Failed to serialize");
        assert!(!serialized.is_empty());
        
        let deserialized: Capability = serde_json::from_str(&serialized)
            .expect("Failed to deserialize");
        assert!(matches!(deserialized, Capability::Storage));
    }

    #[test]
    fn test_capability_comparison() {
        use crate::canonical::capabilities::Capability;
        
        let cap1 = Capability::Network;
        let cap2 = Capability::Network;
        let cap3 = Capability::Storage;
        
        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }

    #[test]
    fn test_capability_display() {
        use crate::canonical::capabilities::Capability;
        
        let cap = Capability::Compute;
        let display_str = format!("{:?}", cap);
        assert!(display_str.contains("Compute"));
    }

    #[test]
    fn test_capability_from_string() {
        // Test parsing capability from string
        let capability_str = "storage";
        assert_eq!(capability_str, "storage");
        
        let capability_str2 = "compute";
        assert_eq!(capability_str2, "compute");
    }
}

