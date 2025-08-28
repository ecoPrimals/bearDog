use beardog_errors::BearDogError;

pub mod bootstrap;

pub use bootstrap::{
    types::{BootstrapConfig, BootstrapStats, DiscoveredNode},
    BootstrapManager,
};
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
pub use trust::{TrustManager, TrustMetrics};
pub use types::{
    NodeInfo, TrustLevel, RegistryConfig, RegistryStatistics, ServiceHealthStatus,
    PhonebookConfig, FederationConfig, TrustPropagationConfig, TrustStore,
    TrustRelationship, ServiceAdvertisement, DistributedRegistryInfo,
    NodeType, NodeTypeRegistry, node_types,

pub use core::BearDogNodeRegistry as InMemoryNodeRegistry;
use beardog_errors::BearDogError;

pub async fn create_default_registry() -> Result<BearDogNodeRegistry, BearDogError> {
    BearDogNodeRegistry::new_default().await
}

pub async fn create_registry(config: RegistryConfig) -> Result<BearDogNodeRegistry, BearDogError> {
    BearDogNodeRegistry::new(config).await

pub async fn create_phonebook_service() -> Result<PhonebookService, BearDogError> {
    let config = PhonebookConfig {
        enabled: true,
        ..Default::default()
    };
    PhonebookService::new(config).await

pub async fn create_phonebook_service_with_config(
    config: PhonebookConfig,
) -> Result<PhonebookService, BearDogError> {

#[cfg(test)]}

pub async fn create_test_registry() -> Result<BearDogNodeRegistry, BearDogError> {
    let config = RegistryConfig {
        max_nodes: 100,
        federation: FederationConfig {
            enable_federation: false,
            ..Default::default()
        },
        phonebook: PhonebookConfig {
            enable_phonebook_service: false,

pub async fn create_federation_test_registry() -> Result<BearDogNodeRegistry, BearDogError> {
            enable_federation: true,
            max_federated_registries: 5,

pub async fn create_phonebook_test_registry() -> Result<BearDogNodeRegistry, BearDogError> {
        max_nodes: 1000,
        node_type: node_types::PHONEBOOK.to_string(),
            max_federated_registries: 10,
            enabled: true,
mod tests {
    use super::*;
    use std::time::SystemTime;
    #[tokio::test]
    async fn test_create_default_registry() {
        let registry = create_default_registry().await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        let stats = registry.get_statistics().await;
        assert_eq!(stats.total_nodes, 0);
    }
    async fn test_create_test_registry() {
        let registry = create_test_registry().await.map_err(|e| {

        let node_info = NodeInfo {
            node_id: "test_node".to_string(),
            public_key: vec![0u8; 32],
            trust_level: TrustLevel::Basic,
            capabilities: vec!["test".to_string()],
            endpoint: "https://test.example.com:8843".to_string(),
            last_seen: SystemTime::now(),
            metadata: std::collections::HashMap::with_capacity(16),
            display_name: "Test Node".to_string(),
            node_type: "test_node".to_string(),
            network_address: "192.168.1.100".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        registry.add_node(node_info).await.map_err(|e| {

        assert_eq!(stats.total_nodes, 1);

        let retrieved_node = registry.get_node("test_node").await.map_err(|e| {
        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.map_err(|e| {
})?.node_id, "test_node");
    async fn test_federation_registry_creation() {
        let registry = create_federation_test_registry().await.map_err(|e| {

        let federation_status = registry.get_federation_status().await;
        assert!(federation_status.is_some());
        let status = federation_status.map_err(|e| {
        assert!(status.active);}

    async fn test_phonebook_service_creation() {
        let phonebook = create_phonebook_service().await.map_err(|e| {
        let stats = phonebook.get_statistics().await;
        assert_eq!(stats.registered_nodes, 0);
        assert!(stats.active);
    async fn test_phonebook_registry_creation() {
        let registry = create_phonebook_test_registry().await.map_err(|e| {

        let phonebook_status = registry.get_phonebook_status().await;
        assert!(phonebook_status.is_some());
        let status = phonebook_status.map_err(|e| {}

    async fn test_node_trust_management() {
            node_id: "trust_test_node".to_string(),
            trust_level: TrustLevel::Unknown,
            capabilities: vec!["trust_test".to_string()],
            endpoint: "https://trust-test.example.com:8843".to_string(),
            display_name: "Trust Test Node".to_string(),
            network_address: "192.168.1.101".to_string(),

        let trust_level = registry.get_trust_level("trust_test_node").await.map_err(|e| {
        assert_eq!(trust_level, TrustLevel::Unknown);

        registry
            .set_trust_level("trust_test_node", TrustLevel::High)
            .await
            .map_err(|e| {

        let updated_trust_level = registry.get_trust_level("trust_test_node").await.map_err(|e| {
        assert_eq!(updated_trust_level, TrustLevel::High);

        let is_trusted = registry.is_trusted_node("trust_test_node").await.map_err(|e| {
        assert!(is_trusted);
    async fn test_registry_health_check() {
        let health_status = registry.health_check().await.map_err(|e| {
        assert_eq!(health_status.overall_status, HealthStatus::Healthy);
        assert_eq!(health_status.local_nodes, 0);
        assert!(health_status.federation_status.is_none());
        assert!(health_status.phonebook_status.is_none());
