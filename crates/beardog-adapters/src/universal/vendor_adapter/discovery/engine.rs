

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::{

    strategies::{
        CloudDiscoveryStrategy, EnvironmentDiscoveryStrategy, HardwareDiscoveryStrategy,
        NetworkDiscoveryStrategy,
    },
    DiscoveryStrategy,
};

#[derive(Debug)]
pub enum DiscoveryStrategyType {
    Cloud(CloudDiscoveryStrategy),
    Environment(EnvironmentDiscoveryStrategy),
    Hardware(HardwareDiscoveryStrategy),
    Network(NetworkDiscoveryStrategy),
}

pub struct VendorDiscoveryEngine {

    strategies: Vec<DiscoveryStrategyType>,

    _config: DiscoveryEngineConfig,

    engine_id: Uuid,
}

#[derive(Debug, Clone)]

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCapability {

    pub instance_id: Uuid,

    pub capability: CapabilityType,

    pub connection: ConnectionSpec,

    pub quality_profile: QualityProfile,

    pub resource_requirements: ResourceRequirements,

    pub discovery_metadata: HashMap<String, serde_json::Value>,

    pub discovered_at: DateTime<Utc>,

    pub discovered_by: String,

pub enum ConnectionSpec {

    Http {
        base_url: String,
        auth: AuthSpec,
        headers: HashMap<String, String>,

    Grpc {
        endpoint: String,
        tls: Option<TlsConfig>,

    NativeLibrary {
        library_path: String,
        initialization: LibraryInitSpec,

    Hardware {
        device_path: String,
        interface_type: HardwareInterface,

    MessageQueue {
        broker_url: String,
        topic: String,
        protocol: MessageProtocol,

    Environment { variables: HashMap<String, String> },

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

pub enum MessageProtocol {
    Mqtt,
    Amqp,
    Kafka,
    Redis,

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

    pub async fn new(config: DiscoveryEngineConfig) -> Result<Self, BearDogError> {
        let engine_id = Uuid::new_v4();
        tracing::info!("🔍 Creating Vendor Discovery Engine: {}", engine_id);
        let mut engine = Self {
            strategies: Vec::new(),
            _config: config,
            engine_id,
        };

        engine.add_strategy(Box::new(EnvironmentDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(HardwareDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(CloudDiscoveryStrategy::new()));
        engine.add_strategy(Box::new(NetworkDiscoveryStrategy::new()));
        tracing::info!(
            "✅ Registered {} default discovery strategies",
            engine.strategies.len()
        );
        Ok(engine)

    #[must_use] pub const fn engine_id(&self) -> Uuid {
        self.engine_id

    pub fn add_strategy(&mut self, strategy: Box<dyn DiscoveryStrategy>) {
        tracing::info!("📋 Adding discovery strategy: {}", strategy.strategy_name());
        self.strategies.push(strategy);

    pub async fn discover_all_capabilities(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
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

    pub async fn discover_capability(
        &self,
        capability: &CapabilityType,
    ) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
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

    #[must_use] pub fn get_statistics(&self) -> DiscoveryStatistics {
        DiscoveryStatistics {
            engine_id: self.engine_id,
            total_strategies: self.strategies.len(),
            strategy_names: self
                .strategies
                .iter()
                .map(|s| s.strategy_name().to_string())
                .collect(),

pub struct DiscoveryStatistics {
    pub engine_id: Uuid,
    pub total_strategies: usize,
    pub strategy_names: Vec<String>,
