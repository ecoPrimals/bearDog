// SPDX-License-Identifier: AGPL-3.0-or-later

//! Specialized Domain Configuration Modules
//!
//! This module contains configuration structures for specialized domains:
//! - Workflow engine
//! - Compliance and regulatory
//! - Performance optimization
//! - Production environment
//! - Deployment and orchestration
//! - Testing framework
//! - Development environment
//! - External adapters
//! - Secure tunnels
//! - Federation
//! - Ecosystem and plugins

use serde::{Deserialize, Serialize};

/// Unified workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedWorkflowConfig {
    /// Maximum number of concurrent workflows
    pub max_concurrent_workflows: usize,
    /// Default workflow timeout
    pub default_timeout_secs: u64,
    /// Enable workflow persistence
    pub enable_persistence: bool,
    /// Retry failed workflows
    pub enable_retry: bool,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
}

impl Default for UnifiedWorkflowConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workflows: 100,
            default_timeout_secs: 300,
            enable_persistence: true,
            enable_retry: true,
            max_retry_attempts: 3,
        }
    }
}

/// Unified compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedComplianceConfig {
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention days
    pub audit_retention_days: u32,
    /// Enable GDPR compliance features
    pub enable_gdpr: bool,
    /// Enable HIPAA compliance features
    pub enable_hipaa: bool,
    /// Data encryption at rest
    pub encrypt_data_at_rest: bool,
}

impl Default for UnifiedComplianceConfig {
    fn default() -> Self {
        Self {
            enable_audit_logging: true,
            audit_retention_days: 365,
            enable_gdpr: true,
            enable_hipaa: false,
            encrypt_data_at_rest: true,
        }
    }
}

/// Unified performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Performance metrics collection interval (seconds)
    pub metrics_interval_secs: u64,
    /// Enable caching
    pub enable_caching: bool,
    /// Cache size limit (MB)
    pub cache_size_mb: usize,
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Connection pool size
    pub connection_pool_size: usize,
}

impl Default for UnifiedPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            metrics_interval_secs: 60,
            enable_caching: true,
            cache_size_mb: 512,
            enable_connection_pooling: true,
            connection_pool_size: 50,
        }
    }
}

/// Unified production configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProductionConfig {
    /// Production environment name
    pub environment: String,
    /// Enable high availability mode
    pub enable_ha: bool,
    /// Health check interval (seconds)
    pub health_check_interval_secs: u64,
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Request timeout (seconds)
    pub request_timeout_secs: u64,
}

impl Default for UnifiedProductionConfig {
    fn default() -> Self {
        Self {
            environment: "production".to_string(),
            enable_ha: true,
            health_check_interval_secs: 30,
            enable_failover: true,
            request_timeout_secs: 60,
        }
    }
}

/// Unified deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDeploymentConfig {
    /// Deployment strategy
    pub strategy: String,
    /// Enable blue-green deployment
    pub enable_blue_green: bool,
    /// Enable canary releases
    pub enable_canary: bool,
    /// Rollback on failure
    pub auto_rollback: bool,
    /// Health check before promotion
    pub health_check_before_promote: bool,
}

impl Default for UnifiedDeploymentConfig {
    fn default() -> Self {
        Self {
            strategy: "rolling".to_string(),
            enable_blue_green: true,
            enable_canary: true,
            auto_rollback: true,
            health_check_before_promote: true,
        }
    }
}

/// Unified testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTestingConfig {
    /// Enable integration tests
    pub enable_integration_tests: bool,
    /// Enable E2E tests
    pub enable_e2e_tests: bool,
    /// Test timeout (seconds)
    pub test_timeout_secs: u64,
    /// Parallel test execution
    pub parallel_execution: bool,
    /// Maximum parallel tests
    pub max_parallel_tests: usize,
}

impl Default for UnifiedTestingConfig {
    fn default() -> Self {
        Self {
            enable_integration_tests: true,
            enable_e2e_tests: true,
            test_timeout_secs: 300,
            parallel_execution: true,
            max_parallel_tests: 4,
        }
    }
}

/// Unified development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDevelopmentConfig {
    /// Enable debug logging
    pub enable_debug_logging: bool,
    /// Enable hot reload
    pub enable_hot_reload: bool,
    /// Enable development tools
    pub enable_dev_tools: bool,
    /// Mock external services
    pub mock_external_services: bool,
}

impl Default for UnifiedDevelopmentConfig {
    fn default() -> Self {
        Self {
            enable_debug_logging: true,
            enable_hot_reload: true,
            enable_dev_tools: true,
            mock_external_services: false,
        }
    }
}

/// Unified adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAdapterConfig {
    /// Enable universal adapters
    pub enable_universal_adapters: bool,
    /// Adapter timeout (seconds)
    pub adapter_timeout_secs: u64,
    /// Enable adapter caching
    pub enable_caching: bool,
    /// Maximum concurrent adapter requests
    pub max_concurrent_requests: usize,
}

impl Default for UnifiedAdapterConfig {
    fn default() -> Self {
        Self {
            enable_universal_adapters: true,
            adapter_timeout_secs: 30,
            enable_caching: true,
            max_concurrent_requests: 50,
        }
    }
}

/// Unified tunnel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTunnelConfig {
    /// Enable secure tunnels
    pub enable_secure_tunnels: bool,
    /// Tunnel encryption algorithm
    pub encryption_algorithm: String,
    /// Enable tunnel compression
    pub enable_compression: bool,
    /// Tunnel keepalive interval (seconds)
    pub keepalive_interval_secs: u64,
}

impl Default for UnifiedTunnelConfig {
    fn default() -> Self {
        Self {
            enable_secure_tunnels: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            enable_compression: true,
            keepalive_interval_secs: 60,
        }
    }
}

/// Unified federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedFederationConfig {
    /// Enable federation
    pub enable_federation: bool,
    /// Federation protocol
    pub protocol: String,
    /// Trust verification required
    pub require_trust_verification: bool,
    /// Sync interval (seconds)
    pub sync_interval_secs: u64,
}

impl Default for UnifiedFederationConfig {
    fn default() -> Self {
        Self {
            enable_federation: true,
            protocol: "sovereign".to_string(),
            require_trust_verification: true,
            sync_interval_secs: 300,
        }
    }
}

/// Unified ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedEcosystemConfig {
    /// Enable ecosystem integration
    pub enable_integration: bool,
    /// Service discovery protocol
    pub discovery_protocol: String,
    /// Enable capability-based routing
    pub enable_capability_routing: bool,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval_secs: u64,
}

impl Default for UnifiedEcosystemConfig {
    fn default() -> Self {
        Self {
            enable_integration: true,
            discovery_protocol: "mdns".to_string(),
            enable_capability_routing: true,
            heartbeat_interval_secs: 30,
        }
    }
}
