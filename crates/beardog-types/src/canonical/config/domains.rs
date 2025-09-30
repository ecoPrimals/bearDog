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

pub mod ai_config;
pub mod monitoring_config;
pub mod discovery_config;
pub mod workflow_config;
pub mod adapter;
pub mod compliance;
pub mod security;

// Re-export all domain configurations for easy access
pub use ai_config::*;
pub use monitoring_config::*;
pub use discovery_config::{ConsolidatedDiscoveryConfig, ServiceRegistryConfig, NetworkDiscoveryConfig, QuantumDiscoveryConfig, DiscoveryCacheConfig, DiscoverySecurityConfig};
pub use workflow_config::{ConsolidatedWorkflowConfig, WorkflowEngineConfig, WorkflowEscalationConfig, EscalationRule, NotificationConfig, RateLimitConfig, SchedulingConfig, QueueConfig, TimeoutConfig, PersistenceConfig, ConnectionConfig, RetentionConfig, ArchiveConfig};
pub use compliance::*;

// Re-export RetryConfig from discovery (to avoid ambiguity)
pub use discovery_config::RetryConfig as DiscoveryRetryConfig;
pub use workflow_config::RetryConfig as WorkflowRetryConfig;

// All domain modules have been extracted and are complete 