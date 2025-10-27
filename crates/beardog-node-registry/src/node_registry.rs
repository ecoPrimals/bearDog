// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;

pub mod bootstrap;

pub use bootstrap::{
    types::{BootstrapConfig, BootstrapStats, DiscoveredNode},
    BootstrapManager,
};
/// Core functionality
/// Core functionality
pub mod core;
pub mod federation;
pub mod phonebook;
pub mod trust;
pub mod types;

pub use bootstrap::{BootstrapConfig, BootstrapManager};
pub use core::{BearDogNodeRegistry, HealthStatus, LocalRegistryInfo, NodeRegistryHealthStatus};
pub use federation::{
    FederatedNodeInfo, FederationHealthStatus, FederationManager, FederationManagerStatus,
    NodeSearchCriteria,};

pub use phonebook::{
    NodeDiscoveryCriteria, PhonebookEntry, PhonebookService, PhonebookStatus,
    ServiceDiscoveryCriteria,
};
pub use trust::{TrustManager, TrustMetrics};
pub use types::{
    NodeInfo, TrustLevel, RegistryConfig, RegistryStatistics, ServiceHealthStatus,
    PhonebookConfig, FederationConfig, TrustPropagationConfig, TrustStore,
    TrustRelationship, ServiceAdvertisement, DistributedRegistryInfo,
    NodeType, NodeTypeRegistry, node_types,
};

pub use core::BearDogNodeRegistry as InMemoryNodeRegistry;
use beardog_errors::BearDogError;

/// Create Default Registry operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates default_registry
pub async fn create_default_registry() -> Result<BearDogNodeRegistry, BearDogError> {
    BearDogNodeRegistry::new_default()
}

/// Create Registry operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates registry
pub async fn create_registry(config: RegistryConfig) -> Result<BearDogNodeRegistry, BearDogError> {
    Ok(BearDogNodeRegistry::new(config.enable_registry))
}

pub async fn create_phonebook_service(config: PhonebookConfig) -> Result<PhonebookService, BearDogError> {

    Ok(PhonebookService::new(PhonebookConfig {
        enable_phonebook_service: config.enable_phonebook_service,
        federation: FederationConfig {
            enable_federation: false,
            ..Default::default()
        },
        ..Default::default()
    }))
}

/// Create Federation Test Registry operation.
pub fn create_federation_test_registry() -> Result<BearDogNodeRegistry, BearDogError> {
    Ok(BearDogNodeRegistry::new(true))
}

/// Create Phonebook Test Registry operation.
pub fn create_phonebook_test_registry() -> Result<PhonebookService, BearDogError> {
    Ok(PhonebookService::new(PhonebookConfig::default()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[tokio::test]
    async fn test_create_test_registry() {
        let registry = create_federation_test_registry()?;
        let stats = registry.get_statistics();
        assert_eq!(stats.total_nodes, 0);

        let node_info = NodeInfo {
            node_id: "test_node".to_string(),
            capabilities: vec!["test".to_string()],
            endpoint: "https://test.example.com:8843".to_string(),
            last_seen: SystemTime::now(),
            metadata: std::collections::HashMap::with_capacity(16),
            display_name: "Test Node".to_string(),
            node_type: "test_node".to_string(),
            network_address: "192.168.1.100".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        registry.add_node(node_info)?;
        let stats = registry.get_statistics();
        assert_eq!(stats.total_nodes, 1);

        let retrieved_node = registry.get_node("test_node")?;
        assert!(retrieved_node.is_some());
        if let Some(node) = retrieved_node {
            assert_eq!(node.node_id, "test_node");
        }
    }

    #[tokio::test]
    async fn test_federation_registry_creation() {
        let registry = create_federation_test_registry()?;
        let federation_status = registry.get_federation_status();
        assert!(federation_status.is_some());
    }

    #[tokio::test]
    async fn test_phonebook_service_creation() {
        let phonebook = create_phonebook_test_registry()?;
        let stats = phonebook.get_statistics();
        assert_eq!(stats.registered_nodes, 0);
        assert!(stats.active);
    }

    #[tokio::test]
    async fn test_phonebook_registry_creation() {
        let registry = create_phonebook_test_registry()?;
        let phonebook_status = registry.get_phonebook_status();
        assert!(phonebook_status.is_some());
    }

    #[tokio::test]
    async fn test_node_trust_management() {
        let registry = create_federation_test_registry()?;
        // Test trust management functionality - pending full implementation
        // Verify registry was created successfully
        assert!(registry.health_check().is_ok());
    }

    #[tokio::test]
    async fn test_registry_health_check() {
        let registry = create_federation_test_registry()?;
        let health_status = registry.health_check()?;
        assert_eq!(health_status.overall_status, HealthStatus::Healthy);
        assert_eq!(health_status.local_nodes, 0);
        assert!(health_status.federation_status.is_none());
        assert!(health_status.phonebook_status.is_none());
    }
}
