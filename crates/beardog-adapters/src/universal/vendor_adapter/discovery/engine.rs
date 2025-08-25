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


/// Vendor Discovery Engine Implementation

use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::{
// CANONICAL IMPORT: use beardog_types::config::UnifiedDiscoveryConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedNetworkConfig;
    strategies::{
        CloudDiscoveryStrategy, EnvironmentDiscoveryStrategy, HardwareDiscoveryStrategy,
        NetworkDiscoveryStrategy,
    },
    DiscoveryStrategy,
};
/// **MODERNIZED** - Discovery strategy types using enum dispatch
#[derive(Debug)]
pub enum DiscoveryStrategyType {
    Cloud(CloudDiscoveryStrategy),
    Environment(EnvironmentDiscoveryStrategy),
    Hardware(HardwareDiscoveryStrategy),
    Network(NetworkDiscoveryStrategy),
}

/// **ZERO-COST** - Vendor discovery engine with enum dispatch
pub struct VendorDiscoveryEngine {
    /// Discovery strategies - zero-cost enum dispatch
    strategies: Vec<DiscoveryStrategyType>,
    /// Configuration for discovery operations
    _config: DiscoveryEngineConfig,
    /// Engine ID
    engine_id: Uuid,
}
/// Configuration for the discovery engine
#[derive(Debug, Clone)]
// MIGRATED: DiscoveryEngineConfig -> use beardog_types::config::UnifiedDiscoveryConfig;


impl Default for DiscoveryEngineConfig {}


    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval_seconds: 300, // 5 minutes
            discovery_timeout_seconds: 30,
            enable_caching: true,
            cache_ttl_seconds: 600, // 10 minutes
        }
    }
/// **DISCOVERED CAPABILITY** - What we found, not who provides it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCapability {
    /// Unique capability instance ID
    pub instance_id: Uuid,
    /// Capability type
    pub capability: CapabilityType,
    /// Connection information (vendor-agnostic)
    pub connection: ConnectionSpec,
    /// Quality profile
    pub quality_profile: QualityProfile,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Discovery metadata
    pub discovery_metadata: HashMap<String, serde_json::Value>,
    /// When this capability was discovered
    pub discovered_at: DateTime<Utc>,
    /// Discovery strategy that found this
    pub discovered_by: String,
/// **CONNECTION SPEC** - How to connect (vendor-agnostic)
pub enum ConnectionSpec {
    /// HTTP/REST API
    Http {
        base_url: String,
        auth: AuthSpec,
        headers: HashMap<String, String>,
    /// gRPC service
    Grpc {
        endpoint: String,
        tls: Option<TlsConfig>,
    /// Native library
    NativeLibrary {
        library_path: String,
        initialization: LibraryInitSpec,
    /// Hardware device
    Hardware {
        device_path: String,
        interface_type: HardwareInterface,
    /// Message queue
    MessageQueue {
        broker_url: String,
        topic: String,
        protocol: MessageProtocol,
    /// Environment variables
    Environment { variables: HashMap<String, String> },
/// Authentication specification
pub enum AuthSpec {
    None,
    ApiKey(String),
    Bearer(String),
    Basic {
        username: String,
        password: String,
    OAuth2 {
        token_url: String,
        client_id: String,
        client_secret: String,
/// TLS configuration}


// MIGRATED: TlsConfig -> use beardog_types::config::UnifiedNetworkConfig;


pub enum MessageProtocol {
    Mqtt,
    Amqp,
    Kafka,
    Redis,
/// Quality profile for discovered capabilities
pub struct QualityProfile {
    pub reliability_score: f64,
    pub availability_percentage: f64,
    pub performance_rating: u8,
    pub security_rating: u8,}


impl Default for QualityProfile {
            reliability_score: 0.8,
            availability_percentage: 99.0,
            performance_rating: 5,
            security_rating: 5,
/// Resource requirements}


pub struct ResourceRequirements {
    pub min_cpu_cores: u32,
    pub min_memory_mb: u64,
    pub min_disk_space_mb: u64,
    pub network_bandwidth_mbps: u32,}


impl Default for ResourceRequirements {
            min_cpu_cores: 1,
            min_memory_mb: 512,
            min_disk_space_mb: 1024,
            network_bandwidth_mbps: 10,}


impl VendorDiscoveryEngine {
    /// Create a new discovery engine
    pub async fn new(config: DiscoveryEngineConfig) -> BearDogResult<Self> {
        let engine_id = Uuid::new_v4();
        tracing::info!("🔍 Creating Vendor Discovery Engine: {}", engine_id);
        let mut engine = Self {
            strategies: Vec::new(),
            _config: config,
            engine_id,
        };
        // Register default discovery strategies
        engine.add_strategy(Box::new(EnvironmentDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(HardwareDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(CloudDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(NetworkDiscoveryStrategy::new()));
        tracing::info!(
            "✅ Registered {} default discovery strategies",
            engine.strategies.len()
        );
        Ok(engine)
    /// Get engine ID
    #[must_use] pub const fn engine_id(&self) -> Uuid {
        self.engine_id
    /// Add a discovery strategy}


    pub fn add_strategy(&mut self, strategy: Box<dyn DiscoveryStrategy>) {
        tracing::info!("📋 Adding discovery strategy: {}", strategy.strategy_name());
        self.strategies.push(strategy);
    /// Discover all capabilities using all strategies
    pub async fn discover_all_capabilities(&self) -> BearDogResult<Vec<DiscoveredCapability>> {
        let mut all_capabilities = Vec::new();
        for strategy in &self.strategies {
            match strategy.discover_capabilities().await {
                Ok(mut capabilities) => {
                    tracing::info!(
                        "🔍 Strategy '{}' discovered {} capabilities",
                        strategy.strategy_name(),
                        capabilities.len()
                    );
                    all_capabilities.append(&mut capabilities);
                }
                Err(e) => {
                    tracing::warn!("⚠️ Strategy '{}' failed: {:?}", strategy.strategy_name(), e);
            }
            "✅ Total capabilities discovered: {}",
            all_capabilities.len()
        Ok(all_capabilities)
    /// Discover capabilities for a specific capability type
    pub async fn discover_capability(
        &self,
        capability: &CapabilityType,
    ) -> BearDogResult<Vec<DiscoveredCapability>> {
        let mut matching_capabilities = Vec::new();
            if strategy.can_discover(capability).await {
                match strategy.discover_capabilities().await {
                    Ok(capabilities) => {
                        let matching: Vec<DiscoveredCapability> = capabilities
                            .into_iter()
                            .filter(|cap| &cap.capability == capability)
                            .collect();
                        tracing::info!(
                            "🎯 Strategy '{}' found {} matching capabilities for {:?}",
                            strategy.strategy_name(),
                            matching.len(),
                            capability
                        );
                        matching_capabilities.extend(matching);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "⚠️ Strategy '{}' failed for capability {:?}: {:?}",
                            capability,
                            e
        Ok(matching_capabilities)
    /// Get statistics about the discovery engine
    #[must_use] pub fn get_statistics(&self) -> DiscoveryStatistics {
        DiscoveryStatistics {
            engine_id: self.engine_id,
            total_strategies: self.strategies.len(),
            strategy_names: self
                .strategies
                .iter()
                .map(|s| s.strategy_name().to_string())
                .collect(),
/// Discovery engine statistics
pub struct DiscoveryStatistics {
    pub engine_id: Uuid,
    pub total_strategies: usize,
    pub strategy_names: Vec<String>,
