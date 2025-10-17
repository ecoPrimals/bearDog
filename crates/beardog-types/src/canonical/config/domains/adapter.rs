//! # Adapter Domain Configuration
//!
//! This module provides unified configuration for all adapter-related functionality,
//! consolidating 20+ scattered adapter configs into a single, maintainable structure.
//!
//! ## Consolidated Configs
//!
//! This module replaces and unifies:
//! - `AdapterConfig` (multiple duplicates across crates)
//! - `PrimalAdapterConfig`, `UniversalAdapterConfig`
//! - `DiscoveryConfig`, `ChainConfig`, `StepConfig`, `RetryConfig`
//! - `OptimizationConfig`, `EntropyCapabilityConfig`
//! - Service mesh and handoff configs
//! - Vendor adapter configs

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::config::r#trait::{validation, BearDogConfig};

/// **UNIFIED ADAPTER CONFIGURATION** - Single source of truth for all adapter functionality
///
/// This configuration consolidates all adapter-related settings into a single,
/// well-organized structure with domain-specific sub-configurations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UnifiedAdapterConfig {
    /// Core adapter settings
    pub core: CoreAdapterConfig,

    /// Discovery and capability detection settings
    pub discovery: DiscoveryConfig,

    /// Chain processing and workflow settings
    pub chain: ChainConfig,

    /// Performance optimization settings
    pub optimization: OptimizationConfig,

    /// Service mesh and handoff settings
    pub service_mesh: ServiceMeshConfig,

    /// Vendor-specific adapter settings
    pub vendor: VendorConfig,

    /// Security settings for adapters
    pub security: AdapterSecurityConfig,

    /// Monitoring and health check settings
    pub monitoring: AdapterMonitoringConfig,
}

/// Core adapter configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreAdapterConfig {
    /// Adapter identifier
    pub adapter_id: String,

    /// Adapter type (primal, universal, capability, etc.)
    pub adapter_type: AdapterType,

    /// Maximum concurrent connections
    pub max_connections: usize,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// Enable adapter registry
    pub registry_enabled: bool,

    /// Adapter metadata
    pub metadata: HashMap<String, String>,
}

/// Discovery and capability detection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryConfig {
    /// Discovery timeout
    pub timeout: Duration,

    /// Maximum discovery attempts
    pub max_attempts: u32,

    /// Discovery interval
    pub discovery_interval: Duration,

    /// Enable capability caching
    pub cache_enabled: bool,

    /// Cache TTL
    pub cache_ttl: Duration,

    /// Discovery endpoints
    pub endpoints: Vec<String>,

    /// Enable predictive discovery
    pub predictive_enabled: bool,
}

/// Chain processing and workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainConfig {
    /// Maximum chain length
    pub max_chain_length: usize,

    /// Chain processing timeout
    pub processing_timeout: Duration,

    /// Step configuration
    pub step: StepConfig,

    /// Retry configuration
    pub retry: RetryConfig,

    /// Enable parallel processing
    pub parallel_enabled: bool,

    /// Maximum parallel workers
    pub max_workers: usize,
}

/// Individual step configuration within chains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepConfig {
    /// Step timeout
    pub timeout: Duration,

    /// Maximum step attempts
    pub max_attempts: u32,

    /// Enable step validation
    pub validation_enabled: bool,

    /// Step metadata
    pub metadata: HashMap<String, String>,
}

/// Retry configuration for failed operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,

    /// Initial retry delay
    pub initial_delay: Duration,

    /// Maximum retry delay
    pub max_delay: Duration,

    /// Backoff multiplier
    pub backoff_multiplier: f64,

    /// Enable exponential backoff
    pub exponential_backoff: bool,

    /// Jitter factor (0.0 to 1.0)
    pub jitter_factor: f64,
}

/// Performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OptimizationConfig {
    /// Enable performance optimization
    pub enabled: bool,

    /// Optimization level (1-5)
    pub level: u8,

    /// Enable SIMD optimizations
    pub simd_enabled: bool,

    /// Enable zero-copy optimizations
    pub zero_copy_enabled: bool,

    /// Buffer size for optimizations
    pub buffer_size: usize,

    /// Enable adaptive optimization
    pub adaptive_enabled: bool,

    /// Optimization metrics collection
    pub metrics_enabled: bool,
}

/// Service mesh and handoff configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ServiceMeshConfig {
    /// Enable service mesh integration
    pub enabled: bool,

    /// Mesh discovery configuration
    pub discovery: MeshDiscoveryConfig,

    /// Handoff retry configuration
    pub handoff_retry: HandoffRetryConfig,

    /// Mesh security configuration
    pub security: MeshSecurityConfig,

    /// Health monitoring configuration
    pub health_monitor: HealthMonitorConfig,
}

/// Mesh discovery specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeshDiscoveryConfig {
    /// Discovery protocol
    pub protocol: String,

    /// Discovery port
    pub port: u16,

    /// Discovery timeout
    pub timeout: Duration,

    /// Enable mTLS for discovery
    pub mtls_enabled: bool,
}

/// Handoff retry specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandoffRetryConfig {
    /// Maximum handoff attempts
    pub max_attempts: u32,

    /// Handoff timeout
    pub timeout: Duration,

    /// Enable circuit breaker
    pub circuit_breaker_enabled: bool,

    /// Circuit breaker threshold
    pub circuit_breaker_threshold: u32,
}

/// Mesh security specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeshSecurityConfig {
    /// Enable TLS
    pub tls_enabled: bool,

    /// TLS version
    pub tls_version: String,

    /// Certificate path
    pub cert_path: Option<String>,

    /// Key path
    pub key_path: Option<String>,

    /// CA path
    pub ca_path: Option<String>,
}

/// Health monitoring specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthMonitorConfig {
    /// Health check interval
    pub check_interval: Duration,

    /// Health check timeout
    pub check_timeout: Duration,

    /// Enable predictive health monitoring
    pub predictive_enabled: bool,

    /// Health check endpoints
    pub endpoints: Vec<String>,
}

/// Vendor-specific adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorConfig {
    /// KMS configuration
    pub kms: KmsConfig,

    /// Cloud provider configurations
    pub cloud_providers: HashMap<String, CloudProviderConfig>,

    /// Enable vendor abstraction
    pub abstraction_enabled: bool,
}

/// KMS (Key Management Service) configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KmsConfig {
    /// KMS provider type
    pub provider: String,

    /// KMS endpoint
    pub endpoint: Option<String>,

    /// KMS region
    pub region: Option<String>,

    /// KMS key ID
    pub key_id: Option<String>,

    /// Enable hardware backing
    pub hardware_backed: bool,
}

/// Cloud provider specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudProviderConfig {
    /// Provider name (AWS, GCP, Azure, etc.)
    pub name: String,

    /// Provider region
    pub region: String,

    /// Authentication configuration
    pub auth: HashMap<String, String>,

    /// Provider-specific settings
    pub settings: HashMap<String, String>,
}

/// Security configuration for adapters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterSecurityConfig {
    /// Enable security features
    pub enabled: bool,

    /// Authentication required
    pub auth_required: bool,

    /// Authorization level
    pub auth_level: AuthLevel,

    /// Enable encryption in transit
    pub encryption_in_transit: bool,

    /// Enable encryption at rest
    pub encryption_at_rest: bool,

    /// Security audit logging
    pub audit_logging: bool,
}

/// Monitoring configuration for adapters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,

    /// Metrics collection interval
    pub metrics_interval: Duration,

    /// Enable performance metrics
    pub performance_metrics: bool,

    /// Enable error metrics
    pub error_metrics: bool,

    /// Enable usage metrics
    pub usage_metrics: bool,

    /// Metrics retention period
    pub retention_period: Duration,
}

/// Adapter type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterType {
    /// Primal-specific adapter
    Primal,
    /// Universal adapter for cross-primal communication
    Universal,
    /// Capability-based adapter
    Capability,
    /// Vendor-specific adapter
    Vendor,
    /// Service mesh adapter
    ServiceMesh,
    /// Custom adapter type
    Custom(String),
}

/// Authorization level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthLevel {
    /// No authentication required
    None,
    /// Basic authentication
    Basic,
    /// Token-based authentication
    Token,
    /// Certificate-based authentication
    Certificate,
    /// Multi-factor authentication
    MultiFactor,
}

impl Default for CoreAdapterConfig {
    fn default() -> Self {
        Self {
            adapter_id: "default-adapter".to_string(),
            adapter_type: AdapterType::Universal,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            registry_enabled: true,
            metadata: HashMap::new(),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            max_attempts: 3,
            discovery_interval: Duration::from_secs(60),
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300),
            endpoints: vec!["http://localhost:8080/discovery".to_string()],
            predictive_enabled: false,
        }
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            max_chain_length: 10,
            processing_timeout: Duration::from_secs(300),
            step: StepConfig::default(),
            retry: RetryConfig::default(),
            parallel_enabled: true,
            max_workers: 4,
        }
    }
}

impl Default for StepConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_attempts: 3,
            validation_enabled: true,
            metadata: HashMap::new(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            exponential_backoff: true,
            jitter_factor: 0.1,
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: 3,
            simd_enabled: true,
            zero_copy_enabled: true,
            buffer_size: 8192,
            adaptive_enabled: true,
            metrics_enabled: true,
        }
    }
}

impl Default for MeshDiscoveryConfig {
    fn default() -> Self {
        Self {
            protocol: "https".to_string(),
            port: 8443,
            timeout: Duration::from_secs(5),
            mtls_enabled: true,
        }
    }
}

impl Default for HandoffRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            timeout: Duration::from_secs(10),
            circuit_breaker_enabled: true,
            circuit_breaker_threshold: 5,
        }
    }
}

impl Default for MeshSecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: true,
            tls_version: "1.3".to_string(),
            cert_path: None,
            key_path: None,
            ca_path: None,
        }
    }
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(5),
            predictive_enabled: false,
            endpoints: vec!["/health".to_string()],
        }
    }
}

impl Default for VendorConfig {
    fn default() -> Self {
        Self {
            kms: KmsConfig::default(),
            cloud_providers: HashMap::new(),
            abstraction_enabled: true,
        }
    }
}

impl Default for KmsConfig {
    fn default() -> Self {
        Self {
            provider: "software".to_string(),
            endpoint: None,
            region: None,
            key_id: None,
            hardware_backed: false,
        }
    }
}

impl Default for AdapterSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_required: true,
            auth_level: AuthLevel::Token,
            encryption_in_transit: true,
            encryption_at_rest: false,
            audit_logging: true,
        }
    }
}

impl Default for AdapterMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            performance_metrics: true,
            error_metrics: true,
            usage_metrics: true,
            retention_period: Duration::from_secs(86400), // 24 hours
        }
    }
}

impl BearDogConfig for UnifiedAdapterConfig {
    fn validate(&self) -> BearDogResult<()> {
        // Validate core configuration
        validation::validate_non_empty_string(&self.core.adapter_id, "adapter_id")?;
        validation::validate_range(self.core.max_connections, 1, 10000, "max_connections")?;

        // Validate discovery configuration
        validation::validate_non_empty_collection(
            &self.discovery.endpoints,
            "discovery_endpoints",
        )?;
        for endpoint in &self.discovery.endpoints {
            validation::validate_url(endpoint, "discovery_endpoint")?;
        }

        // Validate chain configuration
        validation::validate_range(self.chain.max_chain_length, 1, 1000, "max_chain_length")?;

        // Validate optimization configuration
        validation::validate_range(self.optimization.level, 1, 5, "optimization_level")?;

        // Validate retry configuration
        if self.chain.retry.backoff_multiplier <= 1.0 {
            return Err(BearDogError::configuration(
                "Backoff multiplier must be greater than 1.0",
            ));
        }

        validation::validate_range(self.chain.retry.jitter_factor, 0.0, 1.0, "jitter_factor")?;

        // Validate service mesh configuration
        if self.service_mesh.enabled {
            validation::validate_port(self.service_mesh.discovery.port, "mesh_discovery_port")?;
        }

        Ok(())
    }

    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        // Implement field-by-field merge with other taking precedence
        let mut merged = self.clone();

        // Merge core config
        if other.core.adapter_id != "default-adapter" {
            merged.core.adapter_id.clone_from(&other.core.adapter_id);
        }
        merged.core.max_connections = other.core.max_connections;
        merged.core.connection_timeout = other.core.connection_timeout;
        merged.core.registry_enabled = other.core.registry_enabled;

        // Merge discovery config
        if !other.discovery.endpoints.is_empty() {
            merged
                .discovery
                .endpoints
                .clone_from(&other.discovery.endpoints);
        }
        merged.discovery.timeout = other.discovery.timeout;
        merged.discovery.cache_enabled = other.discovery.cache_enabled;

        // Merge other sections...
        merged.optimization = other.optimization.clone();
        merged.security = other.security.clone();
        merged.monitoring = other.monitoring.clone();

        Ok(merged)
    }

    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();

        // Load from environment variables with BEARDOG_ADAPTER_ prefix
        if let Ok(adapter_id) = std::env::var("BEARDOG_ADAPTER_ID") {
            config.core.adapter_id = adapter_id;
        }

        if let Ok(max_conn) = std::env::var("BEARDOG_ADAPTER_MAX_CONNECTIONS") {
            config.core.max_connections = max_conn
                .parse()
                .map_err(|_| BearDogError::configuration("Invalid max_connections value"))?;
        }

        if let Ok(timeout) = std::env::var("BEARDOG_ADAPTER_TIMEOUT_SECS") {
            let secs: u64 = timeout
                .parse()
                .map_err(|_| BearDogError::configuration("Invalid timeout value"))?;
            config.core.connection_timeout = Duration::from_secs(secs);
        }

        if let Ok(endpoints) = std::env::var("BEARDOG_ADAPTER_DISCOVERY_ENDPOINTS") {
            config.discovery.endpoints =
                endpoints.split(',').map(|s| s.trim().to_string()).collect();
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> BearDogResult<String> {
        #[cfg(feature = "config")]
        return toml::to_string_pretty(self).map_err(|e| {
            BearDogError::configuration(&format!("Failed to serialize to TOML: {e}"))
        });

        #[cfg(not(feature = "config"))]
        Err(BearDogError::configuration(
            "TOML support not enabled - enable 'config' feature",
        ))
    }

    fn domain() -> &'static str {
        "adapter"
    }

    fn apply_environment_overrides(&mut self, environment: &str) -> BearDogResult<()> {
        match environment {
            "development" => {
                self.security.auth_required = false;
                self.security.encryption_in_transit = false;
                self.monitoring.enabled = false;
                self.optimization.level = 1;
            }
            "production" => {
                self.security.auth_level = AuthLevel::MultiFactor;
                self.security.encryption_at_rest = true;
                self.optimization.level = 5;
                self.service_mesh.enabled = true;
                self.monitoring.performance_metrics = true;
            }
            "staging" => {
                self.security.auth_level = AuthLevel::Token;
                self.optimization.level = 3;
                self.monitoring.enabled = true;
            }
            _ => {
                // Unknown environment, use defaults
            }
        }
        Ok(())
    }
}

impl UnifiedAdapterConfig {
    /// Create a new adapter configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Create adapter configuration for development environment
    pub fn development() -> Self {
        let mut config = Self::default();
        // SAFETY: Development environment overrides should always be valid.
        // If this fails, it indicates a programming error in apply_environment_overrides.
        config
            .apply_environment_overrides("development")
            .expect("Development environment configuration must be valid");
        config
    }

    /// Create adapter configuration for production environment
    pub fn production() -> Self {
        let mut config = Self::default();
        // SAFETY: Production environment overrides should always be valid.
        // If this fails, it indicates a programming error in apply_environment_overrides.
        config
            .apply_environment_overrides("production")
            .expect("Production environment configuration must be valid");
        config
    }
}

/// Migration utilities for legacy adapter configurations
pub mod migration {
    use super::{DiscoveryConfig, Duration, OptimizationConfig, UnifiedAdapterConfig};

    /// Migrate from legacy `AdapterConfig` to `UnifiedAdapterConfig`
    pub fn migrate_legacy_adapter_config(
        adapter_id: String,
        max_connections: usize,
        timeout_secs: u64,
    ) -> UnifiedAdapterConfig {
        let mut config = UnifiedAdapterConfig::default();
        config.core.adapter_id = adapter_id;
        config.core.max_connections = max_connections;
        config.core.connection_timeout = Duration::from_secs(timeout_secs);
        config
    }

    /// Migrate from legacy `DiscoveryConfig`
    pub fn migrate_legacy_discovery_config(
        endpoints: Vec<String>,
        timeout_ms: u64,
        cache_enabled: bool,
    ) -> DiscoveryConfig {
        DiscoveryConfig {
            endpoints,
            timeout: Duration::from_millis(timeout_ms),
            cache_enabled,
            ..Default::default()
        }
    }

    /// Migrate from legacy `OptimizationConfig`
    pub fn migrate_legacy_optimization_config(
        enabled: bool,
        level: u8,
        simd_enabled: bool,
    ) -> OptimizationConfig {
        OptimizationConfig {
            enabled,
            level,
            simd_enabled,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_adapter_config_default() {
        let config = UnifiedAdapterConfig::default();
        assert_eq!(config.core.adapter_id, "default-adapter");
        assert_eq!(config.core.adapter_type, AdapterType::Universal);
        assert!(config.security.enabled);
        assert!(config.monitoring.enabled);
    }

    #[test]
    fn test_adapter_config_validation() {
        let config = UnifiedAdapterConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config.clone();
        invalid_config.core.adapter_id = String::new();
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_development_config() {
        let config = UnifiedAdapterConfig::development();
        assert!(!config.security.auth_required);
        assert!(!config.security.encryption_in_transit);
        assert!(!config.monitoring.enabled);
        assert_eq!(config.optimization.level, 1);
    }

    #[test]
    fn test_production_config() {
        let config = UnifiedAdapterConfig::production();
        assert_eq!(config.security.auth_level, AuthLevel::MultiFactor);
        assert!(config.security.encryption_at_rest);
        assert_eq!(config.optimization.level, 5);
        assert!(config.service_mesh.enabled);
    }

    #[test]
    fn test_config_serialization() {
        let config = UnifiedAdapterConfig::default();
        let toml_str = config.to_toml().unwrap();
        assert!(toml_str.contains("[core]"));
        assert!(toml_str.contains("[discovery]"));
        assert!(toml_str.contains("[security]"));
    }

    #[test]
    fn test_migration_utilities() {
        let migrated = migration::migrate_legacy_adapter_config("test-adapter".to_string(), 50, 15);
        assert_eq!(migrated.core.adapter_id, "test-adapter");
        assert_eq!(migrated.core.max_connections, 50);
        assert_eq!(migrated.core.connection_timeout, Duration::from_secs(15));
    }
}
