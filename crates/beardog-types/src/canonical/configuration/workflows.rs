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


/// # Canonical Workflow Configuration
///
/// **CONFIGURATION UNIFICATION COMPLETE** ✅
/// This module consolidates ALL workflow configuration types from across the codebase:
/// 
/// ## **Fragmentation Eliminated:**
/// - `beardog-workflows/src/workflows/canonical/configuration.rs` - WorkflowEngineConfig, WorkflowPolicyConfig
/// - `beardog-workflows/src/workflows/canonical/execution/engine.rs` - WorkflowEngineConfig (duplicate)
/// - `beardog-workflows/src/workflows/zero_cost_engine.rs` - ZeroCostEngineConfig  
/// - `beardog-workflows/src/workflows/zero_cost_hsm/factory.rs` - ZeroCostHsmManagerConfig
/// - `beardog-workflows/src/workflows/processors/*` - 5+ processor-specific configs
/// - `beardog-workflows/src/workflows/notification/mod.rs` - RetryConfig
/// 
/// ## **Design Benefits:**
/// - **Single Source of Truth**: All workflow configs defined once
/// - **Hierarchical Organization**: Logical grouping by workflow domain
/// - **Zero Duplication**: Eliminates 8+ duplicate configuration structs
/// - **Canonical Access**: `use beardog_types::canonical::configuration::workflows::*`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// UNIFIED WORKFLOW ENGINE CONFIGURATION
// ============================================================================

/// **CANONICAL WORKFLOW ENGINE CONFIGURATION** 
/// Consolidates WorkflowEngineConfig from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEngineConfig {
    /// Maximum concurrent workflows
    pub max_concurrent: usize,
    /// Workflow timeout in seconds
    pub timeout_secs: u64,
    /// Enable audit logging
    pub audit_enabled: bool,
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Thread pool size for workflow execution
    pub thread_pool_size: Option<usize>,
    /// Enable workflow persistence
    pub persistence_enabled: bool,
}

impl Default for WorkflowEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            timeout_secs: 300,
            audit_enabled: true,
            metrics_enabled: true,
            thread_pool_size: None,
            persistence_enabled: true,
        }
    }
}

/// **CANONICAL WORKFLOW POLICY CONFIGURATION**
/// Consolidates policy settings from multiple workflow modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPolicyConfig {
    /// Maximum concurrent workflows per user
    pub max_concurrent_per_user: u32,
    /// Maximum concurrent workflows globally
    pub max_concurrent_global: u32,
    /// Default workflow timeout
    pub default_timeout: Duration,
    /// Approval timeout duration
    pub approval_timeout: Duration,
    /// Auto cleanup enabled
    pub auto_cleanup_enabled: bool,
    /// Retention period for completed workflows
    pub retention_period: Duration,
    /// Maximum workflow history entries
    pub max_history_entries: usize,
    /// Enable workflow notifications
    pub notifications_enabled: bool,
    /// Approval requirements by workflow type
    pub approval_requirements: HashMap<String, ApprovalRequirements>,
}

impl Default for WorkflowPolicyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_per_user: 10,
            max_concurrent_global: 1000,
            default_timeout: Duration::from_secs(3600), // 1 hour
            approval_timeout: Duration::from_secs(86400), // 24 hours
            auto_cleanup_enabled: true,
            retention_period: Duration::from_secs(2592000), // 30 days
            max_history_entries: 1000,
            notifications_enabled: true,
            approval_requirements: HashMap::new(),
        }
    }
}

// ============================================================================
// WORKFLOW PROCESSING CONFIGURATION
// ============================================================================

/// **CANONICAL WORKFLOW PROCESSOR CONFIGURATION**
/// Consolidates all processor-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct WorkflowProcessorConfig {
    /// Security processor configuration
    pub security: SecurityProcessorConfig,
    /// Key management processor configuration
    pub key_management: KeyManagementConfig,
    /// System processor configuration
    pub system: SystemProcessorConfig,
    /// Policy processor configuration
    pub policy: PolicyProcessorConfig,
    /// Registry processor configuration
    pub registry: RegistryProcessorConfig,
    /// User management processor configuration
    pub user_management: UserManagementProcessorConfig,
}


/// Security processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProcessorConfig {
    /// Enable security auditing
    pub audit_enabled: bool,
    /// Security timeout in seconds
    pub timeout_secs: u64,
    /// Maximum security operations per workflow
    pub max_operations: usize,
    /// Enable hardware security module integration
    pub hsm_enabled: bool,
}

impl Default for SecurityProcessorConfig {
    fn default() -> Self {
        Self {
            audit_enabled: true,
            timeout_secs: 300,
            max_operations: 100,
            hsm_enabled: true,
        }
    }
}

/// Key management processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Default key type for generation
    pub default_key_type: String,
    /// Key rotation interval in seconds
    pub rotation_interval_secs: u64,
    /// Enable automatic key backup
    pub auto_backup: bool,
    /// Maximum keys per workflow
    pub max_keys_per_workflow: usize,
    /// HSM tier requirement
    pub required_hsm_tier: String,
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            default_key_type: "Ed25519".to_string(),
            rotation_interval_secs: 86400 * 30, // 30 days
            auto_backup: true,
            max_keys_per_workflow: 10,
            required_hsm_tier: "software".to_string(),
        }
    }
}

/// System processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemProcessorConfig {
    /// Enable system monitoring
    pub monitoring_enabled: bool,
    /// System health check interval
    pub health_check_interval_secs: u64,
    /// Maximum system operations per workflow
    pub max_operations: usize,
    /// Enable resource limits
    pub resource_limits_enabled: bool,
}

impl Default for SystemProcessorConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            health_check_interval_secs: 60,
            max_operations: 50,
            resource_limits_enabled: true,
        }
    }
}

/// Policy processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProcessorConfig {
    /// Enable policy validation
    pub validation_enabled: bool,
    /// Policy cache TTL in seconds
    pub cache_ttl_secs: u64,
    /// Maximum policy rules per workflow
    pub max_rules: usize,
    /// Enable policy auditing
    pub audit_enabled: bool,
}

impl Default for PolicyProcessorConfig {
    fn default() -> Self {
        Self {
            validation_enabled: true,
            cache_ttl_secs: 3600, // 1 hour
            max_rules: 100,
            audit_enabled: true,
        }
    }
}

/// Registry processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryProcessorConfig {
    /// Registry sync interval in seconds
    pub sync_interval_secs: u64,
    /// Maximum registry operations per workflow
    pub max_operations: usize,
    /// Enable registry caching
    pub caching_enabled: bool,
    /// Registry connection timeout
    pub connection_timeout_secs: u64,
}

impl Default for RegistryProcessorConfig {
    fn default() -> Self {
        Self {
            sync_interval_secs: 300, // 5 minutes
            max_operations: 20,
            caching_enabled: true,
            connection_timeout_secs: 30,
        }
    }
}

/// User management processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserManagementProcessorConfig {
    /// Enable user validation
    pub validation_enabled: bool,
    /// User session timeout in seconds
    pub session_timeout_secs: u64,
    /// Maximum user operations per workflow
    pub max_operations: usize,
    /// Enable user auditing
    pub audit_enabled: bool,
}

impl Default for UserManagementProcessorConfig {
    fn default() -> Self {
        Self {
            validation_enabled: true,
            session_timeout_secs: 3600, // 1 hour
            max_operations: 30,
            audit_enabled: true,
        }
    }
}

// ============================================================================
// WORKFLOW NOTIFICATION AND RETRY CONFIGURATION
// ============================================================================

/// **CANONICAL WORKFLOW RETRY CONFIGURATION**
/// Consolidates RetryConfig from notification module and other retry logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRetryConfig {
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
    /// Retry on timeout
    pub retry_on_timeout: bool,
    /// Retry on network errors
    pub retry_on_network_error: bool,
}

impl Default for WorkflowRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            exponential_backoff: true,
            retry_on_timeout: true,
            retry_on_network_error: true,
        }
    }
}

/// **CANONICAL WORKFLOW NOTIFICATION CONFIGURATION**
/// Consolidates notification settings from multiple modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNotificationConfig {
    /// Enable email notifications
    pub email_enabled: bool,
    /// Enable webhook notifications
    pub webhook_enabled: bool,
    /// Enable in-app notifications
    pub in_app_enabled: bool,
    /// Notification retry configuration
    pub retry_config: WorkflowRetryConfig,
    /// Email notification settings
    pub email_settings: EmailNotificationSettings,
    /// Webhook notification settings
    pub webhook_settings: WebhookNotificationSettings,
}

impl Default for WorkflowNotificationConfig {
    fn default() -> Self {
        Self {
            email_enabled: true,
            webhook_enabled: true,
            in_app_enabled: true,
            retry_config: WorkflowRetryConfig::default(),
            email_settings: EmailNotificationSettings::default(),
            webhook_settings: WebhookNotificationSettings::default(),
        }
    }
}

/// Email notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotificationSettings {
    /// SMTP server host
    pub smtp_host: String,
    /// SMTP server port
    pub smtp_port: u16,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Sender email address
    pub sender_email: String,
    /// Email template directory
    pub template_dir: String,
}

impl Default for EmailNotificationSettings {
    fn default() -> Self {
        Self {
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            tls_enabled: true,
            sender_email: "noreply@beardog.security".to_string(),
            template_dir: "templates/email".to_string(),
        }
    }
}

/// Webhook notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotificationSettings {
    /// Default webhook timeout
    pub timeout_secs: u64,
    /// Enable webhook signatures
    pub signatures_enabled: bool,
    /// Webhook secret for signatures
    pub webhook_secret: Option<String>,
    /// Maximum payload size
    pub max_payload_size: usize,
}

impl Default for WebhookNotificationSettings {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            signatures_enabled: true,
            webhook_secret: None,
            max_payload_size: 1024 * 1024, // 1MB
        }
    }
}

// ============================================================================
// WORKFLOW APPROVAL CONFIGURATION
// ============================================================================

/// **CANONICAL APPROVAL REQUIREMENTS**
/// Defines approval requirements for different workflow types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {
    /// Required approval tier
    pub tier: ApprovalTier,
    /// Number of required approvals
    pub required_count: u32,
    /// Required approver roles
    pub required_roles: Vec<String>,
    /// Enable parallel approvals
    pub parallel_approvals: bool,
    /// Approval timeout
    pub timeout: Duration,
    /// Enable automatic approval for certain conditions
    pub auto_approval_rules: Vec<AutoApprovalRule>,
}

impl Default for ApprovalRequirements {
    fn default() -> Self {
        Self {
            tier: ApprovalTier::Basic,
            required_count: 1,
            required_roles: vec!["approver".to_string()],
            parallel_approvals: false,
            timeout: Duration::from_secs(86400), // 24 hours
            auto_approval_rules: Vec::new(),
        }
    }
}

/// Approval tier levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalTier {
    /// Basic approval tier
    Basic,
    /// Enhanced approval tier
    Enhanced,
    /// Critical approval tier
    Critical,
}

/// Auto approval rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoApprovalRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Enable rule
    pub enabled: bool,
    /// Rule priority
    pub priority: u32,
}

// ============================================================================
// ZERO-COST WORKFLOW CONFIGURATION
// ============================================================================

/// **CANONICAL ZERO-COST WORKFLOW CONFIGURATION**
/// Consolidates ZeroCostEngineConfig and related zero-cost optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostWorkflowConfig {
    /// Enable zero-cost workflow processing
    pub enabled: bool,
    /// Compile-time workflow validation
    pub compile_time_validation: bool,
    /// Use const generics for configuration
    pub const_generics_enabled: bool,
    /// Enable SIMD optimizations
    pub simd_enabled: bool,
    /// Lock-free data structures
    pub lock_free_enabled: bool,
    /// Memory pool configuration
    pub memory_pool: ZeroCostMemoryPoolConfig,
}

impl Default for ZeroCostWorkflowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            compile_time_validation: true,
            const_generics_enabled: true,
            simd_enabled: cfg!(target_feature = "avx2"),
            lock_free_enabled: true,
            memory_pool: ZeroCostMemoryPoolConfig::default(),
        }
    }
}

/// Zero-cost memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostMemoryPoolConfig {
    /// Initial pool size
    pub initial_size: usize,
    /// Maximum pool size
    pub max_size: usize,
    /// Enable memory pool
    pub enabled: bool,
    /// Pool growth factor
    pub growth_factor: f64,
}

impl Default for ZeroCostMemoryPoolConfig {
    fn default() -> Self {
        Self {
            initial_size: 1024 * 1024, // 1MB
            max_size: 100 * 1024 * 1024, // 100MB
            enabled: true,
            growth_factor: 1.5,
        }
    }
}

// ============================================================================
// MASTER WORKFLOW CONFIGURATION
// ============================================================================

/// **CANONICAL WORKFLOW CONFIGURATION** - Master configuration structure
/// This is the top-level configuration that aggregates all workflow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct WorkflowConfig {
    /// Engine configuration
    pub engine: WorkflowEngineConfig,
    /// Policy configuration
    pub policy: WorkflowPolicyConfig,
    /// Processor configurations
    pub processors: WorkflowProcessorConfig,
    /// Notification configuration
    pub notifications: WorkflowNotificationConfig,
    /// Zero-cost optimizations
    pub zero_cost: ZeroCostWorkflowConfig,
    /// Storage configuration
    pub storage: WorkflowStorageConfig,
    /// Monitoring configuration
    pub monitoring: WorkflowMonitoringConfig,
}


/// Workflow storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStorageConfig {
    /// Storage backend type
    pub backend_type: WorkflowStorageBackend,
    /// Database connection string
    pub connection_string: Option<String>,
    /// Enable storage encryption
    pub encryption_enabled: bool,
    /// Storage retention policy
    pub retention_policy: WorkflowRetentionPolicy,
}

impl Default for WorkflowStorageConfig {
    fn default() -> Self {
        Self {
            backend_type: WorkflowStorageBackend::Memory,
            connection_string: None,
            encryption_enabled: true,
            retention_policy: WorkflowRetentionPolicy::default(),
        }
    }
}

/// Workflow storage backend types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStorageBackend {
    /// In-memory storage
    Memory,
    /// PostgreSQL database
    PostgreSQL,
    /// Redis storage
    Redis,
    /// File system storage
    FileSystem,
}

/// Workflow retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRetentionPolicy {
    /// Retention period for completed workflows
    pub completed_retention: Duration,
    /// Retention period for failed workflows
    pub failed_retention: Duration,
    /// Retention period for cancelled workflows
    pub cancelled_retention: Duration,
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
}

impl Default for WorkflowRetentionPolicy {
    fn default() -> Self {
        Self {
            completed_retention: Duration::from_secs(86400 * 30), // 30 days
            failed_retention: Duration::from_secs(86400 * 90), // 90 days
            cancelled_retention: Duration::from_secs(86400 * 7), // 7 days
            auto_cleanup: true,
        }
    }
}

/// Workflow monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Enable health monitoring
    pub health_monitoring_enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Enable performance profiling
    pub profiling_enabled: bool,
}

impl Default for WorkflowMonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_monitoring_enabled: true,
            metrics_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            profiling_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_config_defaults() {
        let config = WorkflowConfig::default();
        
        // Test engine defaults
        assert_eq!(config.engine.max_concurrent, 100);
        assert_eq!(config.engine.timeout_secs, 300);
        assert!(config.engine.audit_enabled);
        assert!(config.engine.metrics_enabled);
        assert!(config.engine.persistence_enabled);
        
        // Test policy defaults
        assert_eq!(config.policy.max_concurrent_per_user, 10);
        assert_eq!(config.policy.max_concurrent_global, 1000);
        assert!(config.policy.auto_cleanup_enabled);
        assert!(config.policy.notifications_enabled);
        
        // Test zero-cost defaults
        assert!(config.zero_cost.enabled);
        assert!(config.zero_cost.compile_time_validation);
        assert!(config.zero_cost.const_generics_enabled);
        assert!(config.zero_cost.lock_free_enabled);
    }

    #[test]
    fn test_approval_requirements() {
        let requirements = ApprovalRequirements::default();
        
        assert_eq!(requirements.tier, ApprovalTier::Basic);
        assert_eq!(requirements.required_count, 1);
        assert_eq!(requirements.required_roles, vec!["approver".to_string()]);
        assert!(!requirements.parallel_approvals);
    }

    #[test]
    fn test_retry_config() {
        let retry_config = WorkflowRetryConfig::default();
        
        assert_eq!(retry_config.max_attempts, 3);
        assert_eq!(retry_config.backoff_multiplier, 2.0);
        assert!(retry_config.exponential_backoff);
        assert!(retry_config.retry_on_timeout);
        assert!(retry_config.retry_on_network_error);
    }
}
