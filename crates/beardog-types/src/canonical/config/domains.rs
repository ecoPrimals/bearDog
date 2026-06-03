// SPDX-License-Identifier: AGPL-3.0-or-later

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
pub mod discovery_modules; // Domain-driven refactored components (Dec 18, 2025)
pub mod discovery_unified;
pub mod monitoring_config;
pub mod network;
pub mod retry;
pub mod security;
pub mod system;
pub mod testing;
pub mod threat;
/// Canonical timeout configuration for network and domain operations.
pub mod timeout;
pub mod workflow;

// Re-export all domain configurations for easy access
pub use ai_config::*;
pub use bootstrap::*;
pub use compliance::*;
pub use database::*;
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
// Workflow rate limits: use `network::RateLimitConfig`, not this re-export list.
pub use workflow::{
    ArchiveConfig, ConnectionConfig, ConsolidatedWorkflowConfig, EscalationRule,
    NotificationConfig, PersistenceConfig, QueueConfig, RetentionConfig,
    RetryConfig as WorkflowRetryConfig, SchedulingConfig, TimeoutConfig, WorkflowEngineConfig,
    WorkflowEscalationConfig,
};

// Re-export canonical configs
pub use retry::CanonicalRetryConfig;
pub use timeout::CanonicalTimeoutConfig;

// All domain modules have been extracted and are complete

// Tests moved to canonical/config/tests/
