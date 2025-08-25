// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Node Registry Module
///
/// This module provides a fully decentralized node registry system for BearDog
/// that supports federation, phonebook services, and peer-to-peer networking.
/// ## Architecture
/// The node registry is built with a modular architecture:
/// - **Core Registry** (`core.rs`) - Main registry implementation
/// - **Types** (`types.rs`) - Data structures and configuration
/// - **Trust Management** (`trust.rs`) - Trust relationships and propagation
/// - **Federation** (`federation.rs`) - Registry-to-registry communication
/// - **Phonebook Service** (`phonebook.rs`) - Discovery service
/// - **Bootstrap** (`bootstrap.rs`) - Initial node discovery and setup
/// ## Usage
/// ```rust
/// use beardog::node_registry::{BearDogNodeRegistry, RegistryConfig, node_types};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a decentralized registry
/// let config = RegistryConfig {
///     node_type: node_types::SECURITY.to_string(),
///     federation: FederationConfig {
///         enable_federation: true,
///         max_federated_registries: 10,
///         ..Default::default()
///     },
///     phonebook: PhonebookConfig {
///         enable_phonebook_service: false,
///     ..Default::default()
/// };
/// let registry = BearDogNodeRegistry::new(config).await?;
/// // Bootstrap with initial nodes
/// registry.bootstrap().await?;
/// // Register a node
/// let node_info = NodeInfo::new(
///     "node1".to_string(),
///     vec![0u8; 32], // 32-byte Ed25519 public key
///     "https://node1.example.com:8843".to_string(),
/// );
/// registry.add_node(node_info).await?;
/// # Ok(())
/// # }
/// ```

// Public modules
pub mod bootstrap;
// Import the modular bootstrap components
pub use bootstrap::{
    types::{BootstrapConfig, BootstrapStats, DiscoveredNode},
    BootstrapManager,
};
pub mod core;
pub mod federation;
pub mod phonebook;
pub mod trust;
pub mod types;
// Re-export main types for convenience
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
// Re-export for backwards compatibility};


pub use core::BearDogNodeRegistry as InMemoryNodeRegistry;
use crate::BearDogResult;
/// Create a new BearDog node registry with default configuration
pub async fn create_default_registry() -> BearDogResult<BearDogNodeRegistry> {
    BearDogNodeRegistry::new_default().await
}
/// Create a new BearDog node registry with custom configuration
pub async fn create_registry(config: RegistryConfig) -> BearDogResult<BearDogNodeRegistry> {
    BearDogNodeRegistry::new(config).await
/// Create a phonebook service with default configuration}


pub async fn create_phonebook_service() -> BearDogResult<PhonebookService> {
    let config = PhonebookConfig {
        enabled: true,
        ..Default::default()
    };
    PhonebookService::new(config).await
/// Create a phonebook service with custom configuration
pub async fn create_phonebook_service_with_config(
    config: PhonebookConfig,
) -> BearDogResult<PhonebookService> {
/// Utility function to create a basic node registry for testing
#[cfg(test)]}


pub async fn create_test_registry() -> BearDogResult<BearDogNodeRegistry> {
    let config = RegistryConfig {
        max_nodes: 100,
        federation: FederationConfig {
            enable_federation: false,
            ..Default::default()
        },
        phonebook: PhonebookConfig {
            enable_phonebook_service: false,
/// Utility function to create a federation-enabled registry for testing
pub async fn create_federation_test_registry() -> BearDogResult<BearDogNodeRegistry> {
            enable_federation: true,
            max_federated_registries: 5,
/// Utility function to create a phonebook registry for testing}


pub async fn create_phonebook_test_registry() -> BearDogResult<BearDogNodeRegistry> {
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
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let stats = registry.get_statistics().await;
        assert_eq!(stats.total_nodes, 0);
    }
    async fn test_create_test_registry() {
        let registry = create_test_registry().await.map_err(|e| {
        // Create a test node
        let node_info = NodeInfo {
            node_id: "test_node".to_string(),
            public_key: vec![0u8; 32],
            trust_level: TrustLevel::Basic,
            capabilities: vec!["test".to_string()],
            endpoint: "https://test.example.com:8843".to_string(),
            last_seen: SystemTime::now(),
            metadata: std::collections::HashMap::new(),
            display_name: "Test Node".to_string(),
            node_type: "test_node".to_string(),
            network_address: "192.168.1.100".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };
        // Add the node
        registry.add_node(node_info).await.map_err(|e| {
        // Verify it was added
        assert_eq!(stats.total_nodes, 1);
        // Retrieve the node
        let retrieved_node = registry.get_node("test_node").await.map_err(|e| {
        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.map_err(|e| {
})?.node_id, "test_node");
    async fn test_federation_registry_creation() {
        let registry = create_federation_test_registry().await.map_err(|e| {
        // Check that federation is enabled
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
        // Check that phonebook service is enabled
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
        // Check initial trust level
        let trust_level = registry.get_trust_level("trust_test_node").await.map_err(|e| {
        assert_eq!(trust_level, TrustLevel::Unknown);
        // Update trust level
        registry
            .set_trust_level("trust_test_node", TrustLevel::High)
            .await
            .map_err(|e| {
        // Verify trust level was updated
        let updated_trust_level = registry.get_trust_level("trust_test_node").await.map_err(|e| {
        assert_eq!(updated_trust_level, TrustLevel::High);
        // Check if node is trusted
        let is_trusted = registry.is_trusted_node("trust_test_node").await.map_err(|e| {
        assert!(is_trusted);
    async fn test_registry_health_check() {
        let health_status = registry.health_check().await.map_err(|e| {
        assert_eq!(health_status.overall_status, HealthStatus::Healthy);
        assert_eq!(health_status.local_nodes, 0);
        assert!(health_status.federation_status.is_none());
        assert!(health_status.phonebook_status.is_none());
