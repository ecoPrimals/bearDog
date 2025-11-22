//! # Configuration Domains Module
//!
//! This module organizes configuration types by functional domain, extracted from
//! the large consolidated_domains.rs file for better maintainability and compliance
//! with the 2000-line file size limit.
//!
//! ## Domain Organization
//!
//! - **AI Configuration**: AI/ML, hybrid intelligence, training, inference
//! - **Monitoring Configuration**: Metrics, health checks, alerting, observability
//! - **Discovery Configuration**: Service discovery, registry, capability detection
//! - **Workflow Configuration**: Workflow engine, scheduling, orchestration
//! - **Security Configuration**: Security policies, compliance, audit trails
//! - **Testing Configuration**: Test configs, benchmarks, API tests, production validation
//! - **Network Configuration**: Connection pools, timeouts, load balancing, endpoints

pub mod adapter;
pub mod ai_config;
pub mod bootstrap;
pub mod compliance;
pub mod database;
pub mod discovery; // ✅ Canonical DiscoveryConfig (Nov 10, 2025)
pub mod discovery_config; // ⚠️ DEPRECATED - Use discovery_unified (Nov 8, 2025)
pub mod discovery_unified;
pub mod monitoring_config;
pub mod network;
pub mod retry; // ✅ Canonical RetryConfig (Nov 8, 2025)
pub mod security;
pub mod system;
pub mod testing;
pub mod threat;
pub mod timeout; // ⚠️  DEPRECATED - Use timeout_unified (Nov 8, 2025)
pub mod timeout_unified; // ✅ Unified TimeoutConfig (Nov 8, 2025)
pub mod workflow_config;

// Re-export all domain configurations for easy access
pub use ai_config::*;
pub use bootstrap::*;
pub use compliance::*;
pub use database::*;
// Re-export deprecated discovery types for backward compatibility
#[allow(deprecated)]
pub use discovery_config::{
    ConsolidatedDiscoveryConfig, DiscoveryCacheConfig, DiscoverySecurityConfig,
    NetworkDiscoveryConfig, QuantumDiscoveryConfig, ServiceRegistryConfig,
};
pub use discovery_unified::{
    CircuitBreakerConfig as UnifiedCircuitBreakerConfig,
    DiscoveryCacheConfig as UnifiedDiscoveryCacheConfig, DiscoveryProtocol,
    DiscoverySecurityConfig as UnifiedDiscoverySecurityConfig, LoadBalancingAlgorithm,
    LoadBalancingConfig as UnifiedLoadBalancingConfig,
    NetworkDiscoveryConfig as UnifiedNetworkDiscoveryConfig,
    QuantumDiscoveryConfig as UnifiedQuantumDiscoveryConfig,
    ServiceRegistryConfig as UnifiedServiceRegistryConfig, UnifiedDiscoveryConfig,
};
pub use monitoring_config::*;
pub use system::{
    LogFormat, LogLevel, LogRotationConfig, LogRotationFrequency, LogTarget, LogTargetType,
    LoggingConfig,
};
pub use testing::{
    CanonicalApiTestConfig, CanonicalBenchmarkConfig, CanonicalProductionTestConfig,
    CanonicalTestConfig, TestCredentials,
};
pub use threat::{
    CanonicalThreatDetectionConfig, SensitivityLevel, ThreatConfig, ThreatDetectionConfig,
    ThreatResponseConfig, UnifiedThreatConfig,
};
pub use workflow_config::{
    ArchiveConfig, ConnectionConfig, ConsolidatedWorkflowConfig, EscalationRule,
    NotificationConfig, PersistenceConfig, QueueConfig,
    /* RateLimitConfig - use network::RateLimitConfig */ RetentionConfig, SchedulingConfig,
    TimeoutConfig, WorkflowEngineConfig, WorkflowEscalationConfig,
};

// Re-export canonical configs (Nov 8, 2025 unification)
pub use retry::CanonicalRetryConfig;
pub use timeout::CanonicalTimeoutConfig;

// Legacy re-exports (DEPRECATED - use CanonicalRetryConfig instead)
// These will be removed once all code migrates to canonical versions
pub use discovery_config::RetryConfig as DiscoveryRetryConfig;
pub use workflow_config::RetryConfig as WorkflowRetryConfig;

// All domain modules have been extracted and are complete

// Tests moved to canonical/config/tests/
