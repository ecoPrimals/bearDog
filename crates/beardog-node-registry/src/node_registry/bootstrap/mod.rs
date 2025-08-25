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


/// Bootstrap Module for Node Registry - Complete Modular Architecture
///
/// This module handles the bootstrapping process for the node registry,
/// including discovering and connecting to initial nodes, phonebook services,
/// and federation partners. The module has been completely refactored into
/// focused sub-modules for excellent maintainability and clarity.
/// ## Module Organization
/// - `types` - Type definitions and configuration structures
/// - `discovery` - Node discovery from various sources (phonebook, environment)
/// - `verification` - Node verification and identity validation  
/// - `federation` - Federation network discovery and bootstrapping
/// - `manager` - Main BootstrapManager coordination (legacy file)
/// ## Key Features
/// - **Multi-Source Discovery**: Phonebook services, environment variables, federation
/// - **Comprehensive Verification**: Identity validation, challenge-response auth
/// - **Federation Support**: Cross-registry node discovery and trust establishment
/// - **Regional Preferences**: Smart node selection based on geographic preferences
/// - **Robust Caching**: Verification and federation result caching for performance
/// - **Configurable Trust**: Multi-tier trust assessment and validation

pub mod discovery;
pub mod federation;  
pub mod types;
pub mod verification;
// Re-export commonly used types and services
pub use discovery::NodeDiscovery;
pub use federation::FederationBootstrap;
pub use verification::NodeVerification;
pub use types::{
    BootstrapConfig, BootstrapHealthCheck, BootstrapStats,
    DiscoveredNode, FederationDiscoveryResult, PhonebookDiscoveryResponse,
};
// Import the main BootstrapManager from the original file
// (The original bootstrap.rs file now serves as the manager module)
pub use super::bootstrap::BootstrapManager;
/// Bootstrap service factory for creating focused services
pub struct BootstrapServiceFactory;
impl BootstrapServiceFactory {
    /// Create a complete set of bootstrap services}


    pub fn create_services(config: BootstrapConfig) -> BootstrapServices {
        BootstrapServices {
            discovery: NodeDiscovery::new(config.clone()),
            verification: NodeVerification::new(config.clone()),
            federation: FederationBootstrap::new(config),
        }
    }
}
/// Collection of all bootstrap services for easy management
pub struct BootstrapServices {
    /// Node discovery service
    pub discovery: NodeDiscovery,
    /// Node verification service
    pub verification: NodeVerification,
    /// Federation bootstrap service
    pub federation: FederationBootstrap,}


impl BootstrapServices {
    /// Comprehensive bootstrap operation using all services
    pub async fn comprehensive_bootstrap(&self) -> crate::BearDogResult<Vec<crate::node_registry::types::NodeInfo>> {
        use tracing::{info, warn};
        
        let mut all_nodes = Vec::new();
        // Step 1: Discover from environment
        match self.discovery.discover_from_environment().await {
            Ok(mut env_nodes) => {
                info!("🌍 Environment discovery: {} nodes", env_nodes.len());
                all_nodes.append(&mut env_nodes);
            }
            Err(e) => warn!("Environment discovery failed: {}", e),
        // Step 2: Discover from phonebooks
        match self.discovery.discover_from_phonebooks().await {
            Ok(mut phonebook_nodes) => {
                info!("📞 Phonebook discovery: {} nodes", phonebook_nodes.len());
                all_nodes.append(&mut phonebook_nodes);
            Err(e) => warn!("Phonebook discovery failed: {}", e),
        // Step 3: Discover from federation
        match self.federation.bootstrap_from_federation().await {
            Ok(mut federation_nodes) => {
                info!("🌐 Federation discovery: {} nodes", federation_nodes.len());
                all_nodes.append(&mut federation_nodes);
            Err(e) => warn!("Federation discovery failed: {}", e),
        // Step 4: Filter and deduplicate
        all_nodes = self.discovery.filter_discovered_nodes(all_nodes);
        // Step 5: Verify all discovered nodes
        let mut verified_nodes = Vec::new();
        for node in all_nodes {
            match self.verification.verify_node(&node).await {
                Ok(true) => {
                    info!("✅ Verified node: {}", node.node_id);
                    verified_nodes.push(node);
                }
                Ok(false) => {
                    warn!("❌ Failed to verify node: {}", node.node_id);
                Err(e) => {
                    warn!("⚠️ Verification error for {}: {}", node.node_id, e);
        info!("🎉 Bootstrap complete: {} verified nodes", verified_nodes.len());
        Ok(verified_nodes)
    
    /// Get comprehensive bootstrap statistics
    pub async fn get_comprehensive_stats(&self) -> std::collections::HashMap<String, u32> {
        let mut stats = std::collections::HashMap::new();
        // Discovery stats
        let discovery_stats = self.discovery.get_discovery_stats().await;
        for (key, value) in discovery_stats {
            stats.insert(format!("discovery_{}", key), value);
        // Verification stats
        let verification_stats = self.verification.get_verification_stats();
        for (key, value) in verification_stats {
            stats.insert(format!("verification_{}", key), value);
        // Federation stats
        let federation_stats = self.federation.get_federation_stats();
        for (key, value) in federation_stats {
            stats.insert(format!("federation_{}", key), value);
        stats
} 
