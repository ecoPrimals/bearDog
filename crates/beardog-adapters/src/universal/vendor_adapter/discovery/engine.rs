

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

#[derive(Debug, Clone)]
    _config: DiscoveryEngineConfig,

    engine_id: Uuid,
}

#[derive(Debug, Clone)]
            discovery_interval_seconds: 300, // 5 minutes
            discovery_timeout_seconds: 30,
            enable_caching: true,
            cache_ttl_seconds: 600, // 10 minutes
        }
    }

#[derive(Debug, Clone)]
    /// The capability value
    pub capability: CapabilityType,

    /// The connection value
    pub connection: ConnectionSpec,

    /// The quality profile value
    pub quality_profile: QualityProfile,

    /// The resource requirements value
    pub resource_requirements: ResourceRequirements,

    /// Mapping of discovery metadata
    pub discovery_metadata: HashMap<String, serde_json::Value>,

    /// The discovered at value
    pub discovered_at: DateTime<Utc>,

    /// The discovered by value
    pub discovered_by: String,

pub enum ConnectionSpec {

    /// Represents http variant
    Http {
        base_url: String,
        auth: AuthSpec,
        headers: HashMap<String, String>,

    /// Represents grpc variant
    Grpc {
        endpoint: String,
        tls: Option<TlsConfig>,

    /// Represents native library variant
    NativeLibrary {
        library_path: String,
        initialization: LibraryInitSpec,

    /// Represents hardware variant
    Hardware {
        device_path: String,
        interface_type: HardwareInterface,

    /// Represents message queue variant
    MessageQueue {
        broker_url: String,
        topic: String,
        protocol: MessageProtocol,

    Environment { variables: HashMap<String, String> },
    Environment { variables: HashMap<String, String> },
    Environment { variables: HashMap<String, String> },

pub enum AuthSpec {
    /// No none specified
    None,
    /// Represents api key variant
    ApiKey(String,
        password: String,
    /// Represents o auth2 variant
    OAuth2 {
        token_url: String,
        client_id: String,
        client_secret: String,

pub enum MessageProtocol {
    /// Represents mqtt variant
    Mqtt,
    /// Represents amqp variant
    Amqp,
    /// Represents kafka variant
    Kafka,
    /// Represents redis variant
    Redis,

pub struct QualityProfile {
    /// The reliability score value
    pub reliability_score: f64,
    /// The availability percentage value
    pub availability_percentage: f64,
    pub performance_rating: u8,
    /// Number of security_rating
    pub security_rating: u8,}
    pub security_rating: u8,}
    pub security_rating: u8,}

impl Default for QualityProfile {
            reliability_score: 0.8,
            availability_percentage: 99.0,
            performance_rating: 5,
            security_rating: 5,

pub struct ResourceRequirements {
    /// Number of min_cpu_cores
    pub min_cpu_cores: u32,
    /// Number of min_memory_mb
    pub min_memory_mb: u64,
    /// Number of min_disk_space_mb
    pub min_disk_space_mb: u64,
    pub network_bandwidth_mbps: u32,}

impl Default for ResourceRequirements {
            min_cpu_cores: 1,
            min_memory_mb: 512,
            min_disk_space_mb: 1024,
            network_bandwidth_mbps: 10,}

impl VendorDiscoveryEngine {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: DiscoveryEngineConfig) -> Result<Self, BearDogError> {
        let engine_id = Uuid::new_v4();
        tracing::info!("🔍 Creating Vendor Discovery Engine: {}", engine_id);
        let mut engine = Self {
            strategies: Vec::new(config,
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

/// Add Strategy operation.
    pub fn add_strategy(&mut self, strategy: Box<dyn DiscoveryStrategy>) {
        tracing::info!("📋 Adding discovery strategy: {}", strategy.strategy_name());
        self.strategies.push(strategy);

/// Discover All Capabilities operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_all_capabilities(&self) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
        let mut all_capabilities = Vec::new();
        for strategy in &self.strategies {
            match strategy.discover_capabilities() {
                Ok(mut capabilities) => {
                    tracing::info!(
                        "🔍 Strategy "{}" discovered {} capabilities",
                        strategy.strategy_name(),
                        capabilities.len()
                    );
                    all_capabilities.append(&mut capabilities);
                }
                Err(e) => {
                    tracing::warn!("⚠️ Strategy "{}" failed: {:?}", strategy.strategy_name({}",
            all_capabilities.len(&CapabilityType,
    ) -> Result<Vec<DiscoveredCapability>, BearDogError>> {
        let mut matching_capabilities = Vec::new();
            if strategy.can_discover(capability) {
                match strategy.discover_capabilities() {
                    Ok(capabilities) => {
                        let matching: Vec<DiscoveredCapability> = capabilities
                            .into_iter()
                            .filter(|cap| &cap.capability == capability)
                            .collect();
                        tracing::info!(
                            "🎯 Strategy "{}" found {} matching capabilities for {:?}",
                            strategy.strategy_name(),
                            matching.len(),
                            capability
                        );
                        matching_capabilities.extend(matching);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "⚠️ Strategy "{}" failed for capability {:?}: {:?}",
                            capability,
                            e
        Ok(self.engine_id,
            total_strategies: self.strategies.len(),
            strategy_names: self
                .strategies
                .iter(Uuid,
    /// Number of total_strategies
    pub total_strategies: usize,
    /// Name of the strategys
    pub strategy_names: Vec<String>,
