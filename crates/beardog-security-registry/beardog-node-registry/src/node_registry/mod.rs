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
    ServiceDiscoveryCriteria,};

pub use trust::{TrustManager, TrustMetrics};
pub use types::{
    NodeInfo, TrustLevel, RegistryConfig, RegistryStatistics, ServiceHealthStatus,
    PhonebookConfig, FederationConfig, TrustPropagationConfig, TrustStore,
    TrustRelationship, ServiceAdvertisement, DistributedRegistryInfo,
    NodeType, NodeTypeRegistry, node_types,};

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
    BearDogNodeRegistry::new(true,
        ..Default::default()
    };
    PhonebookService::new(PhonebookConfig,
) -> Result<PhonebookService, BearDogError> {

#[cfg(100,
        federation: FederationConfig {
            enable_federation: false,
            ..Default::default(PhonebookConfig {
            enable_phonebook_service: false,

/// Create Federation Test Registry operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates federation_test_registry
pub fn create_federation_test_registry(true,
            max_federated_registries: 5,}

/// Create Phonebook Test Registry operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates phonebook_test_registry
pub fn create_phonebook_test_registry(1000,
        node_type: node_types::PHONEBOOK.to_string();
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        let stats = registry.get_statistics();
        assert_eq!(stats.total_nodes, 0);
    }
    fn test_create_test_registry() {
        let registry = create_test_registry().map_err(|e| {

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

        registry.add_node(node_info).map_err(|e| {

        assert_eq!(stats.total_nodes, 1);

        let retrieved_node = registry.get_node("test_node").map_err(|e| {
        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.map_err(|e| {
})?.node_id, "test_node");
    fn test_federation_registry_creation() {
        let registry = create_federation_test_registry().map_err(|e| {

        let federation_status = registry.get_federation_status();
        assert!(federation_status.is_some());
        let status = federation_status.map_err(|e| {
        assert!(status.active);}


    fn test_phonebook_service_creation() {
        let phonebook = create_phonebook_service().map_err(|e| {
        let stats = phonebook.get_statistics();
        assert_eq!(stats.registered_nodes, 0);
        assert!(stats.active);}


    fn test_phonebook_registry_creation() {
        let registry = create_phonebook_test_registry().map_err(|e| {

        let phonebook_status = registry.get_phonebook_status();
        assert!(phonebook_status.is_some());
        let status = phonebook_status.map_err(|e| {}


    fn test_node_trust_management() {
            node_id: "trust_test_node".to_string(),
            capabilities: vec!["trust_test".to_string()],
            endpoint: "https://trust-test.example.com:8843".to_string(),
            display_name: "Trust Test Node".to_string(),
            network_address: "192.168.1.101".to_string(),

        let trust_level = registry.get_trust_level("trust_test_node").map_err(|e| {
        assert_eq!(trust_level, TrustLevel::Unknown);

        registry
            .set_trust_level("trust_test_node", TrustLevel::High)
            .map_err(|e| {

        let updated_trust_level = registry.get_trust_level("trust_test_node").map_err(|e| {
        assert_eq!(updated_trust_level, TrustLevel::High);

        let is_trusted = registry.is_trusted_node("trust_test_node").map_err(|e| {
        assert!(is_trusted);}


    fn test_registry_health_check() {
        let health_status = registry.health_check().map_err(|e| {
        assert_eq!(health_status.overall_status, HealthStatus::Healthy);
        assert_eq!(health_status.local_nodes, 0);
        assert!(health_status.federation_status.is_none());
        assert!(health_status.phonebook_status.is_none());
