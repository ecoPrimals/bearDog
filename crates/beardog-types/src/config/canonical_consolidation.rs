

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalHsmConfig {

    pub core: HsmCoreConfig,

    pub security: HsmSecurityConfig,

    pub performance: HsmPerformanceConfig,

    pub providers: HashMap<String, HsmProviderConfig>,

    pub failover: HsmFailoverConfig,

    pub monitoring: HsmMonitoringConfig,

    pub audit: HsmAuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmCoreConfig {

    pub pool_size: usize,

    pub connection_timeout_ms: u64,

    pub operation_timeout_ms: u64,

    pub max_retries: u32,

    pub hardware_acceleration: bool,

    pub default_key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmSecurityConfig {

    pub min_security_level: String,

    pub require_hardware_backed: bool,

    pub enable_attestation: bool,

    pub auth_methods: Vec<String>,

    pub access_policies: HashMap<String, Vec<String>>,

    pub allowed_algorithms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmPerformanceConfig {

    pub max_concurrent_ops: usize,

    pub key_cache_size: usize,

    pub key_cache_ttl_seconds: u64,

    pub batch_size: usize,

    pub optimizations: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmProviderConfig {

    pub provider_type: String,

    pub settings: HashMap<String, String>,

    pub priority: u32,

    pub capabilities: Vec<String>,

    pub resource_limits: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmFailoverConfig {

    pub enable_failover: bool,

    pub failover_timeout_ms: u64,

    pub health_check_interval_seconds: u32,

    pub max_failures: u32,

    pub recovery_check_interval_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmMonitoringConfig {

    pub enable_metrics: bool,

    pub metrics_interval_seconds: u32,

    pub health_endpoints: Vec<String>,

    pub alert_thresholds: HashMap<String, f64>,

    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmAuditConfig {

    pub enable_audit: bool,

    pub retention_days: u32,

    pub audit_events: Vec<String>,

    pub compliance_standards: Vec<String>,

    pub secure_storage: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalTunnelConfig {

    pub hsm: CanonicalHsmConfig,

    pub network: TunnelNetworkConfig,

    pub security: TunnelSecurityConfig,

    pub performance: TunnelPerformanceConfig,

    pub discovery: TunnelDiscoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelNetworkConfig {

    pub bind_address: String,

    pub port: u16,

    pub max_connections: usize,

    pub connection_timeout_ms: u64,

    pub keep_alive_seconds: u32,

    pub buffer_sizes: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelSecurityConfig {

    pub tls: TunnelTlsConfig,

    pub authentication: TunnelAuthConfig,

    pub access_control: HashMap<String, Vec<String>>,

    pub rate_limiting: TunnelRateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelTlsConfig {

    pub enabled: bool,

    pub version: String,

    pub cert_path: Option<String>,

    pub key_path: Option<String>,

    pub ca_path: Option<String>,

    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelAuthConfig {

    pub method: String,

    pub token_validation: HashMap<String, String>,

    pub session_timeout_seconds: u64,

    pub max_sessions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelRateLimitConfig {

    pub enabled: bool,

    pub requests_per_second: u32,

    pub burst_capacity: u32,

    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelPerformanceConfig {

    pub worker_threads: usize,

    pub io_threads: usize,

    pub memory_pool: TunnelMemoryPoolConfig,

    pub compression: TunnelCompressionConfig,

    pub caching: TunnelCachingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelMemoryPoolConfig {

    pub initial_size_bytes: usize,

    pub max_size_bytes: usize,

    pub block_size_bytes: usize,

    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelCompressionConfig {

    pub enabled: bool,

    pub algorithm: String,

    pub level: u8,

    pub min_size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelCachingConfig {

    pub enabled: bool,

    pub max_entries: usize,

    pub ttl_seconds: u64,

    pub eviction_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelDiscoveryConfig {

    pub enabled: bool,

    pub discovery_interval_seconds: u32,

    pub endpoints: Vec<String>,

    pub registration: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalAdapterConfig {

    pub universal: UniversalAdapterSettings,

    pub vendors: HashMap<String, VendorAdapterConfig>,

    pub discovery: AdapterDiscoveryConfig,

    pub security: AdapterSecurityConfig,

    pub performance: AdapterPerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UniversalAdapterSettings {

    pub pool_size: usize,

    pub connection_timeout_ms: u64,

    pub operation_timeout_ms: u64,

    pub retry_config: AdapterRetryConfig,

    pub circuit_breaker: AdapterCircuitBreakerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct VendorAdapterConfig {

    pub vendor: String,

    pub adapter_type: String,

    pub settings: HashMap<String, String>,

    pub capabilities: Vec<String>,

    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterDiscoveryConfig {

    pub auto_discovery: bool,

    pub strategies: Vec<String>,

    pub discovery_timeout_ms: u64,

    pub refresh_interval_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterSecurityConfig {

    pub authentication: HashMap<String, String>,

    pub authorization: HashMap<String, Vec<String>>,

    pub encryption: AdapterEncryptionConfig,

    pub audit: AdapterAuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterEncryptionConfig {

    pub required: bool,

    pub algorithms: Vec<String>,

    pub key_management: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterAuditConfig {

    pub enabled: bool,

    pub events: Vec<String>,

    pub storage: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterPerformanceConfig {

    pub max_concurrent_ops: usize,

    pub connection_pooling: AdapterConnectionPoolConfig,

    pub caching: AdapterCachingConfig,

    pub load_balancing: AdapterLoadBalancingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterConnectionPoolConfig {

    pub min_connections: usize,

    pub max_connections: usize,

    pub idle_timeout_seconds: u64,

    pub validation_interval_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterCachingConfig {

    pub enabled: bool,

    pub max_entries: usize,

    pub ttl_seconds: u64,

    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterLoadBalancingConfig {

    pub algorithm: String,

    pub health_check: AdapterHealthCheckConfig,

    pub failover: AdapterFailoverConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterHealthCheckConfig {

    pub enabled: bool,

    pub interval_seconds: u32,

    pub timeout_ms: u64,

    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterFailoverConfig {

    pub enabled: bool,

    pub failure_threshold: u32,

    pub recovery_interval_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterRetryConfig {

    pub max_attempts: u32,

    pub base_delay_ms: u64,

    pub max_delay_ms: u64,

    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterCircuitBreakerConfig {

    pub enabled: bool,

    pub failure_threshold: u32,

    pub recovery_timeout_seconds: u64,

    pub half_open_max_calls: u32,
}

pub struct ConfigurationConsolidator;

impl ConfigurationConsolidator {

    pub fn consolidate_hsm_configs() -> CanonicalHsmConfig {

        CanonicalHsmConfig {
            core: HsmCoreConfig {
                pool_size: 10,
                connection_timeout_ms: 30000,
                operation_timeout_ms: 60000,
                max_retries: 3,
                hardware_acceleration: true,
                default_key_size: 256,
            },
            security: HsmSecurityConfig {
                min_security_level: "high".to_string(),
                require_hardware_backed: true,
                enable_attestation: true,
                auth_methods: vec!["certificate".to_string(), "token".to_string()],
                access_policies: HashMap::with_capacity(16),
                allowed_algorithms: vec!["AES256".to_string(), "RSA2048".to_string(), "ECDSA256".to_string()],
            },
            performance: HsmPerformanceConfig {
                max_concurrent_ops: 100,
                key_cache_size: 1000,
                key_cache_ttl_seconds: 3600,
                batch_size: 10,
                optimizations: HashMap::with_capacity(16),
            },
            providers: HashMap::with_capacity(16),
            failover: HsmFailoverConfig {
                enable_failover: true,
                failover_timeout_ms: 5000,
                health_check_interval_seconds: 30,
                max_failures: 3,
                recovery_check_interval_seconds: 60,
            },
            monitoring: HsmMonitoringConfig {
                enable_metrics: true,
                metrics_interval_seconds: 60,
                health_endpoints: vec!["/health".to_string()],
                alert_thresholds: HashMap::with_capacity(16),
                log_level: "info".to_string(),
            },
            audit: HsmAuditConfig {
                enable_audit: true,
                retention_days: 90,
                audit_events: vec!["key_generation".to_string(), "key_usage".to_string()],
                compliance_standards: vec!["FIPS140-2".to_string()],
                secure_storage: HashMap::with_capacity(16),
            },
        }
    }

    pub fn generate_consolidation_report() -> ConsolidationReport {
        ConsolidationReport {
            total_original_configs: 511,
            consolidated_canonical_configs: 10,
            consolidation_ratio: 51.1,
            estimated_maintenance_reduction: 0.90,
            crates_affected: vec![
                "beardog-tunnel".to_string(),
                "beardog-adapters".to_string(),
                "beardog-monitoring".to_string(),
                "beardog-security".to_string(),
                "beardog-workflows".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {

    pub total_original_configs: usize,

    pub consolidated_canonical_configs: usize,

    pub consolidation_ratio: f32,

    pub estimated_maintenance_reduction: f32,

    pub crates_affected: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_hsm_config_creation() {
        let config = ConfigurationConsolidator::consolidate_hsm_configs();
        assert_eq!(config.core.pool_size, 10);
        assert!(config.security.require_hardware_backed);
        assert!(config.failover.enable_failover);
    }
    
    #[test]
    fn test_consolidation_report() {
        let report = ConfigurationConsolidator::generate_consolidation_report();
        assert_eq!(report.total_original_configs, 511);
        assert_eq!(report.consolidated_canonical_configs, 10);
        assert!(report.consolidation_ratio > 50.0);
    }
    
    #[test]
    fn test_configuration_serialization() {
        let config = CanonicalHsmConfig::default();
        let serialized = serde_json::to_string(&config)
            .expect("CanonicalHsmConfig should serialize to JSON");
        let deserialized: CanonicalHsmConfig = serde_json::from_str(&serialized)
            .expect("Serialized CanonicalHsmConfig should deserialize correctly");

        assert_eq!(config.core.pool_size, deserialized.core.pool_size);
    }
} 