// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capabilities Comprehensive Tests
//!
//! Extensive test coverage for capability types and operations

use crate::canonical::capabilities::*;

#[cfg(test)]
mod capability_type_tests {
    use super::*;

    #[test]
    fn test_all_capability_variants_exist() {
        // Ensure all expected capability types can be created
        let capabilities = [
            CapabilityType::Security,
            CapabilityType::DataStorage,
            CapabilityType::ComputeIntelligence,
            CapabilityType::ServiceMesh,
            CapabilityType::DistributedIntelligence,
            CapabilityType::ContainerOrchestration,
        ];

        assert_eq!(capabilities.len(), 6);
    }

    #[test]
    fn test_capability_clone() {
        let cap = CapabilityType::Security;
        let cloned = cap.clone();

        assert_eq!(format!("{cap:?}"), format!("{:?}", cloned));
    }

    #[test]
    fn test_capability_debug_output() {
        let cap = CapabilityType::Security;
        let debug_str = format!("{cap:?}");

        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Security"));
    }

    #[test]
    fn test_capability_equality() {
        let cap1 = CapabilityType::Security;
        let cap2 = CapabilityType::Security;
        let cap3 = CapabilityType::DataStorage;

        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }
}

#[cfg(test)]
mod capability_collection_tests {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_capability_set_operations() {
        let mut capabilities = HashSet::new();

        capabilities.insert(CapabilityType::Security);
        capabilities.insert(CapabilityType::DataStorage);
        capabilities.insert(CapabilityType::ComputeIntelligence);

        assert_eq!(capabilities.len(), 3);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(capabilities.contains(&CapabilityType::Security));
        assert!(!capabilities.contains(&CapabilityType::Monitoring));
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_capability_set_no_duplicates() {
        let mut capabilities = HashSet::new();

        capabilities.insert(CapabilityType::Security);
        capabilities.insert(CapabilityType::Security);
        capabilities.insert(CapabilityType::Security);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        // Should only have one entry
        assert_eq!(capabilities.len(), 1);
    }

    #[test]
    fn test_capability_vec_operations() {
        let capabilities = [
            CapabilityType::Security,
            CapabilityType::DataStorage,
            CapabilityType::ComputeIntelligence,
        ];

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(capabilities.len(), 3);
        assert_eq!(capabilities[0], CapabilityType::Security);
    }
}

#[cfg(test)]
mod capability_metadata_tests {
    use super::*;

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_beardog_primary_capabilities() {
        // BearDog's primary capability is Security
        let beardog_caps = [CapabilityType::Security];

        assert_eq!(beardog_caps.len(), 1);
        assert_eq!(beardog_caps[0], CapabilityType::Security);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_multi_capability_primal() {
        // Some primals might have multiple capabilities
        let capabilities = [
            CapabilityType::ComputeIntelligence,
            CapabilityType::ServiceMesh,
            CapabilityType::ContainerOrchestration,
        ];

        assert!(!capabilities.is_empty());
    }
}

#[cfg(test)]
mod capability_serialization_tests {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    use super::*;

    #[test]
    fn test_capability_size() {
        use std::mem::size_of;

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let size = size_of::<CapabilityType>();
        // Should be reasonably sized (enum with string data)
        assert!(
            size <= 32,
            "Capability type should be reasonably sized, got {size} bytes"
        );
    }

    #[test]
    fn test_capability_option_size() {
        use std::mem::size_of;

        let size = size_of::<Option<CapabilityType>>();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Option<enum> should still be reasonably sized
        assert!(
            size <= 32,
            "Option<Capability> should be reasonably sized, got {size} bytes"
        );
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
mod capability_pattern_tests {
    use super::*;

    #[test]
    fn test_capability_matching() {
        let cap = CapabilityType::Security;

        let description = match cap {
            CapabilityType::Security => "Provides security services",
            CapabilityType::DataStorage => "Provides storage services",
            CapabilityType::ComputeIntelligence => "Provides compute services",
            CapabilityType::ServiceMesh => "Provides networking services",
            CapabilityType::DistributedIntelligence => "Provides AI services",
            CapabilityType::ContainerOrchestration => "Provides orchestration services",
            _ => "Provides other services",
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
        };

        assert_eq!(description, "Provides security services");
    }

    #[test]
    fn test_capability_filtering() {
        let all_caps = [
            CapabilityType::Security,
            CapabilityType::DataStorage,
            CapabilityType::ComputeIntelligence,
            CapabilityType::ServiceMesh,
        ];

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let n = all_caps
            .iter()
            .filter(|&c| matches!(c, CapabilityType::Security))
            .count();

        assert_eq!(n, 1);
    }
}

#[cfg(test)]
mod capability_integration_tests {
    use super::*;

    #[test]
    fn test_capability_discovery_simulation() {
        // Simulate discovering capabilities
        let discovered = [CapabilityType::Security];

        assert!(!discovered.is_empty());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(discovered[0], CapabilityType::Security);
    }

    #[test]
    fn test_capability_registration_simulation() {
        // Simulate registering capabilities with ecosystem
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let capabilities = vec![CapabilityType::Security];

        // Should be able to announce these
        assert!(!capabilities.is_empty());

        for cap in capabilities {
            assert_eq!(cap, CapabilityType::Security);
        }
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_capability_lookup_simulation() {
        use std::collections::HashMap;

        // Simulate capability registry
        let mut registry = HashMap::new();
        registry.insert(CapabilityType::Security, "beardog");
        registry.insert(CapabilityType::DataStorage, "storage-provider");
        registry.insert(CapabilityType::DistributedIntelligence, "compute-provider");

        // Look up who provides security
        let security_provider = registry.get(&CapabilityType::Security);
        assert_eq!(security_provider, Some(&"beardog"));
    }
}
