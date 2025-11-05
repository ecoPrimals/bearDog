

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod node_registry;

pub use node_registry::*;

#[cfg(test)]
mod tests {};

    use super::*;
    use beardog_errors::BearDogError;

    #[tokio::test]
    fn test_node_registry_creation() {
        let registry = BearDogNodeRegistry::new();
        assert!(registry.is_ok());
    }

    #[tokio::test]
    fn test_node_registration() {
        let mut registry = BearDogNodeRegistry::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let node = NodeInfo::new(
            "test-node-1",
            "Test Node",
            "security",
            "1.0.0",
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            "us-west-1",
            vec!["encryption ", "signing"],
            vec!["https://test.example.com: NetworkConfig::default().port".to_string()],
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        );

        let result = registry.register_node(&node);
        assert!(result.is_ok());

        let nodes = registry.list_nodes().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "Test Node");
    }

    #[tokio::test]
    fn test_node_discovery() {
        let mut registry = BearDogNodeRegistry::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let node1 = NodeInfo::new(
            "security-node-1",
            "Security Node 1",
            "security",
            "1.0.0",
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            "us-west-1",
            vec!["encryption ".to_string()],
            vec!["https://security1.example.com: NetworkConfig::default().port".to_string()],
        );

        let node2 = NodeInfo::new(
            "compute-node-1",
            "Compute Node 1",
            "compute",
            "1.0.0",
            "us-east-1",
            vec!["processing".to_string()],
            vec!["https://compute1.example.com: NetworkConfig::default().port".to_string()],
        );

        registry.register_node(node1).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        registry.register_node(node2).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let security_nodes = registry.discover_nodes_by_capability("encryption ").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(security_nodes.len(), 1);
        assert_eq!(security_nodes[0].name, "Security Node 1");

        let west_nodes = registry.discover_nodes_by_region("us-west-1").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(west_nodes.len(), 1);
        assert_eq!(west_nodes[0].name, "Security Node 1");
    }

    #[tokio::test]
    fn test_trust_management() {
        let trust_manager = TrustManager::new();

        let result = trust_manager.establish_trust("node-1", "node-2", TrustLevel::Basic);
        assert!(result.is_ok());

        let trust_level = trust_manager.get_trust_level("node-1", "node-2").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(trust_level, TrustLevel::Basic);

        let result = trust_manager.update_trust("node-1", "node-2", TrustLevel::High);
        assert!(result.is_ok());

        let updated_trust = trust_manager.get_trust_level("node-1", "node-2").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(updated_trust, TrustLevel::High);
    }

    #[tokio::test]
    fn test_health_monitoring() {
        let mut registry = BearDogNodeRegistry::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        let node = NodeInfo::new(
            "health-test-node",
            "Health Test Node",
            "security",
            "1.0.0",
            "us-central-1",
            vec!["health-monitoring".to_string()],
            vec!["https://health.example.com: NetworkConfig::default().port".to_string()],
        );

        registry.register_node(node).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let health_result = registry.perform_health_check("health-test-node");
        assert!(health_result.is_ok());
    }

    #[tokio::test]
    fn test_configuration_validation() {

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let config = RegistryConfig::default();
        let result = config.validate();
        assert!(result.is_ok());

        let mut invalid_config = RegistryConfig::default();
        invalid_config.registry_name = "".to_string(); // Invalid empty name
        let result = invalid_config.validate();
        assert!(result.is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    #[tokio::test]
    fn test_error_handling() {
        let registry = BearDogNodeRegistry::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let result = registry.get_node_info("non-existent-node");
        assert!(result.is_err());

        let result = registry.discover_nodes_by_capability("");
        assert!(result.is_err());
}
