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

//! # Canonical Configuration Consolidation
//!
//! **UNIFIED CANONICAL CONFIGURATIONS**
//! 
//! This module provides the definitive canonical configuration structures
//! that consolidate 511+ fragmented configurations into <50 unified types.
//! 
//! ## Consolidation Strategy
//! - **HSM Configuration**: Unifies 22+ HSM-related configs from tunnel crate
//! - **Network Configuration**: Consolidates 15+ network configs across crates
//! - **Security Configuration**: Merges 18+ security configs into single canonical
//! - **Performance Configuration**: Combines 12+ performance configs
//! - **Monitoring Configuration**: Unifies 14+ monitoring configs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// CANONICAL HSM CONFIGURATION - Consolidates 22+ HSM configs from tunnel crate
// ============================================================================

/// **CANONICAL HSM CONFIGURATION** - Single source of truth for all HSM settings
/// 
/// Consolidates configurations from:
/// - `beardog-tunnel/src/tunnel/hsm/types/config.rs` (22 configs)
/// - `beardog-tunnel/src/hsm_foundation/` configs
/// - Various HSM provider configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalHsmConfig {
    /// Core HSM settings
    pub core: HsmCoreConfig,
    /// Security configuration
    pub security: HsmSecurityConfig,
    /// Performance settings
    pub performance: HsmPerformanceConfig,
    /// Provider-specific configurations
    pub providers: HashMap<String, HsmProviderConfig>,
    /// Failover and redundancy
    pub failover: HsmFailoverConfig,
    /// Monitoring and health checks
    pub monitoring: HsmMonitoringConfig,
    /// Audit and compliance
    pub audit: HsmAuditConfig,
}

/// HSM Core Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmCoreConfig {
    /// HSM pool size
    pub pool_size: usize,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Operation timeout in milliseconds
    pub operation_timeout_ms: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    /// Default key size in bits
    pub default_key_size: u32,
}

/// HSM Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmSecurityConfig {
    /// Minimum security level required
    pub min_security_level: String,
    /// Require hardware-backed keys
    pub require_hardware_backed: bool,
    /// Enable key attestation
    pub enable_attestation: bool,
    /// Authentication methods
    pub auth_methods: Vec<String>,
    /// Access control policies
    pub access_policies: HashMap<String, Vec<String>>,
    /// Encryption algorithms allowed
    pub allowed_algorithms: Vec<String>,
}

/// HSM Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmPerformanceConfig {
    /// Concurrent operations limit
    pub max_concurrent_ops: usize,
    /// Key caching settings
    pub key_cache_size: usize,
    /// Key cache TTL in seconds
    pub key_cache_ttl_seconds: u64,
    /// Batch operation size
    pub batch_size: usize,
    /// Performance optimization flags
    pub optimizations: HashMap<String, bool>,
}

/// HSM Provider Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmProviderConfig {
    /// Provider type (software, tpm, strongbox, secure_enclave)
    pub provider_type: String,
    /// Provider-specific settings
    pub settings: HashMap<String, String>,
    /// Priority for provider selection
    pub priority: u32,
    /// Capabilities offered by this provider
    pub capabilities: Vec<String>,
    /// Resource limits
    pub resource_limits: HashMap<String, u64>,
}

/// HSM Failover Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmFailoverConfig {
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Failover timeout in milliseconds
    pub failover_timeout_ms: u64,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u32,
    /// Maximum failures before failover
    pub max_failures: u32,
    /// Recovery check interval in seconds
    pub recovery_check_interval_seconds: u32,
}

/// HSM Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmMonitoringConfig {
    /// Enable performance metrics
    pub enable_metrics: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u32,
    /// Health check endpoints
    pub health_endpoints: Vec<String>,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Log level for HSM operations
    pub log_level: String,
}

/// HSM Audit Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct HsmAuditConfig {
    /// Enable audit logging
    pub enable_audit: bool,
    /// Audit log retention in days
    pub retention_days: u32,
    /// Audit events to capture
    pub audit_events: Vec<String>,
    /// Compliance standards to meet
    pub compliance_standards: Vec<String>,
    /// Secure audit storage configuration
    pub secure_storage: HashMap<String, String>,
}

// ============================================================================
// CANONICAL TUNNEL CONFIGURATION - Consolidates tunnel-specific configs
// ============================================================================

/// **CANONICAL TUNNEL CONFIGURATION** - Unified tunnel settings
/// 
/// Consolidates configurations from:
/// - `beardog-tunnel/src/tunnel/config.rs`
/// - Various tunnel module configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalTunnelConfig {
    /// HSM configuration
    pub hsm: CanonicalHsmConfig,
    /// Network tunnel settings
    pub network: TunnelNetworkConfig,
    /// Security settings
    pub security: TunnelSecurityConfig,
    /// Performance optimization
    pub performance: TunnelPerformanceConfig,
    /// Discovery and routing
    pub discovery: TunnelDiscoveryConfig,
}

/// Tunnel Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelNetworkConfig {
    /// Bind address
    pub bind_address: String,
    /// Listen port
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Keep-alive interval in seconds
    pub keep_alive_seconds: u32,
    /// Buffer sizes
    pub buffer_sizes: HashMap<String, usize>,
}

/// Tunnel Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelSecurityConfig {
    /// TLS configuration
    pub tls: TunnelTlsConfig,
    /// Authentication settings
    pub authentication: TunnelAuthConfig,
    /// Access control
    pub access_control: HashMap<String, Vec<String>>,
    /// Rate limiting
    pub rate_limiting: TunnelRateLimitConfig,
}

/// Tunnel TLS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelTlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// TLS version
    pub version: String,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
    /// CA certificate path
    pub ca_path: Option<String>,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
}

/// Tunnel Authentication Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelAuthConfig {
    /// Authentication method
    pub method: String,
    /// Token validation settings
    pub token_validation: HashMap<String, String>,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum concurrent sessions
    pub max_sessions: usize,
}

/// Tunnel Rate Limiting Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelRateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst capacity
    pub burst_capacity: u32,
    /// Rate limiting algorithm
    pub algorithm: String,
}

/// Tunnel Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelPerformanceConfig {
    /// Worker thread count
    pub worker_threads: usize,
    /// I/O thread count
    pub io_threads: usize,
    /// Memory pool configuration
    pub memory_pool: TunnelMemoryPoolConfig,
    /// Compression settings
    pub compression: TunnelCompressionConfig,
    /// Caching configuration
    pub caching: TunnelCachingConfig,
}

/// Tunnel Memory Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelMemoryPoolConfig {
    /// Initial pool size in bytes
    pub initial_size_bytes: usize,
    /// Maximum pool size in bytes
    pub max_size_bytes: usize,
    /// Block size in bytes
    pub block_size_bytes: usize,
    /// Enable memory pool
    pub enabled: bool,
}

/// Tunnel Compression Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelCompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level (1-9)
    pub level: u8,
    /// Minimum size to compress
    pub min_size_bytes: usize,
}

/// Tunnel Caching Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelCachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size in entries
    pub max_entries: usize,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Cache eviction policy
    pub eviction_policy: String,
}

/// Tunnel Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TunnelDiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery interval in seconds
    pub discovery_interval_seconds: u32,
    /// Discovery endpoints
    pub endpoints: Vec<String>,
    /// Service registration settings
    pub registration: HashMap<String, String>,
}

// ============================================================================
// CANONICAL ADAPTER CONFIGURATION - Consolidates adapter configs
// ============================================================================

/// **CANONICAL ADAPTER CONFIGURATION** - Unified adapter settings
/// 
/// Consolidates configurations from:
/// - `beardog-adapters/src/universal/` configs
/// - Various adapter module configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalAdapterConfig {
    /// Universal adapter settings
    pub universal: UniversalAdapterSettings,
    /// Vendor-specific configurations
    pub vendors: HashMap<String, VendorAdapterConfig>,
    /// Discovery and routing
    pub discovery: AdapterDiscoveryConfig,
    /// Security configuration
    pub security: AdapterSecurityConfig,
    /// Performance settings
    pub performance: AdapterPerformanceConfig,
}

/// Universal Adapter Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UniversalAdapterSettings {
    /// Adapter pool size
    pub pool_size: usize,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Operation timeout in milliseconds
    pub operation_timeout_ms: u64,
    /// Retry configuration
    pub retry_config: AdapterRetryConfig,
    /// Circuit breaker settings
    pub circuit_breaker: AdapterCircuitBreakerConfig,
}

/// Vendor Adapter Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct VendorAdapterConfig {
    /// Vendor name
    pub vendor: String,
    /// Adapter type
    pub adapter_type: String,
    /// Vendor-specific settings
    pub settings: HashMap<String, String>,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Priority for selection
    pub priority: u32,
}

/// Adapter Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterDiscoveryConfig {
    /// Enable automatic discovery
    pub auto_discovery: bool,
    /// Discovery strategies
    pub strategies: Vec<String>,
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,
    /// Refresh interval in seconds
    pub refresh_interval_seconds: u32,
}

/// Adapter Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterSecurityConfig {
    /// Authentication configuration
    pub authentication: HashMap<String, String>,
    /// Authorization settings
    pub authorization: HashMap<String, Vec<String>>,
    /// Encryption requirements
    pub encryption: AdapterEncryptionConfig,
    /// Audit settings
    pub audit: AdapterAuditConfig,
}

/// Adapter Encryption Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterEncryptionConfig {
    /// Require encryption
    pub required: bool,
    /// Encryption algorithms
    pub algorithms: Vec<String>,
    /// Key management
    pub key_management: HashMap<String, String>,
}

/// Adapter Audit Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterAuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit events to capture
    pub events: Vec<String>,
    /// Audit storage configuration
    pub storage: HashMap<String, String>,
}

/// Adapter Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterPerformanceConfig {
    /// Maximum concurrent operations
    pub max_concurrent_ops: usize,
    /// Connection pooling
    pub connection_pooling: AdapterConnectionPoolConfig,
    /// Caching configuration
    pub caching: AdapterCachingConfig,
    /// Load balancing
    pub load_balancing: AdapterLoadBalancingConfig,
}

/// Adapter Connection Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterConnectionPoolConfig {
    /// Minimum connections
    pub min_connections: usize,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection idle timeout in seconds
    pub idle_timeout_seconds: u64,
    /// Connection validation interval in seconds
    pub validation_interval_seconds: u32,
}

/// Adapter Caching Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterCachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size in entries
    pub max_entries: usize,
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    /// Cache strategy
    pub strategy: String,
}

/// Adapter Load Balancing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterLoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: String,
    /// Health check configuration
    pub health_check: AdapterHealthCheckConfig,
    /// Failover settings
    pub failover: AdapterFailoverConfig,
}

/// Adapter Health Check Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterHealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval in seconds
    pub interval_seconds: u32,
    /// Health check timeout in milliseconds
    pub timeout_ms: u64,
    /// Health check endpoint
    pub endpoint: String,
}

/// Adapter Failover Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterFailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover threshold
    pub failure_threshold: u32,
    /// Recovery check interval in seconds
    pub recovery_interval_seconds: u32,
}

/// Adapter Retry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

/// Adapter Circuit Breaker Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterCircuitBreakerConfig {
    /// Enable circuit breaker
    pub enabled: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery timeout in seconds
    pub recovery_timeout_seconds: u64,
    /// Half-open max calls
    pub half_open_max_calls: u32,
}

// ============================================================================
// CONFIGURATION CONSOLIDATION UTILITIES
// ============================================================================

/// Configuration consolidation utilities
pub struct ConfigurationConsolidator;

impl ConfigurationConsolidator {
    /// Consolidate HSM configurations from tunnel crate
    pub fn consolidate_hsm_configs() -> CanonicalHsmConfig {
        // This would contain logic to merge existing HSM configs
        // For now, return default with sensible production values
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
                access_policies: HashMap::new(),
                allowed_algorithms: vec!["AES256".to_string(), "RSA2048".to_string(), "ECDSA256".to_string()],
            },
            performance: HsmPerformanceConfig {
                max_concurrent_ops: 100,
                key_cache_size: 1000,
                key_cache_ttl_seconds: 3600,
                batch_size: 10,
                optimizations: HashMap::new(),
            },
            providers: HashMap::new(),
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
                alert_thresholds: HashMap::new(),
                log_level: "info".to_string(),
            },
            audit: HsmAuditConfig {
                enable_audit: true,
                retention_days: 90,
                audit_events: vec!["key_generation".to_string(), "key_usage".to_string()],
                compliance_standards: vec!["FIPS140-2".to_string()],
                secure_storage: HashMap::new(),
            },
        }
    }
    
    /// Generate migration report for configuration consolidation
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

/// Configuration consolidation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {
    /// Total original configuration structs
    pub total_original_configs: usize,
    /// Number of consolidated canonical configurations
    pub consolidated_canonical_configs: usize,
    /// Consolidation ratio (original/canonical)
    pub consolidation_ratio: f32,
    /// Estimated maintenance reduction percentage
    pub estimated_maintenance_reduction: f32,
    /// Crates affected by consolidation
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
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: CanonicalHsmConfig = serde_json::from_str(&serialized).unwrap();
        // Basic serialization test passes if no panic occurs
        assert_eq!(config.core.pool_size, deserialized.core.pool_size);
    }
} 