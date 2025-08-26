

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use crate::config::UnifiedProcessorConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEngineConfig {

    pub max_concurrent: usize,

    pub timeout_secs: u64,

    pub audit_enabled: bool,

    pub metrics_enabled: bool,

    pub thread_pool_size: Option<usize>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPolicyConfig {

    pub max_concurrent_per_user: u32,

    pub max_concurrent_global: u32,

    pub default_timeout: Duration,

    pub approval_timeout: Duration,

    pub auto_cleanup_enabled: bool,

    pub retention_period: Duration,

    pub max_history_entries: usize,

    pub notifications_enabled: bool,

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
            approval_requirements: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct WorkflowProcessorConfig {

    pub security: UnifiedProcessorConfig,

    pub key_management: UnifiedProcessorConfig,

    pub system: UnifiedProcessorConfig,

    pub policy: UnifiedProcessorConfig,

    pub registry: UnifiedProcessorConfig,

    pub user_management: UserManagementProcessorConfig,
}

#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct SecurityProcessorConfig {

    pub audit_enabled: bool,

    pub timeout_secs: u64,

    pub max_operations: usize,

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

#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct KeyManagementConfig {

    pub default_key_type: String,

    pub rotation_interval_secs: u64,

    pub auto_backup: bool,

    pub max_keys_per_workflow: usize,

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

#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct SystemProcessorConfig {

    pub monitoring_enabled: bool,

    pub health_check_interval_secs: u64,

    pub max_operations: usize,

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

#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct PolicyProcessorConfig {

    pub validation_enabled: bool,

    pub cache_ttl_secs: u64,

    pub max_rules: usize,

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

#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct RegistryProcessorConfig {

    pub sync_interval_secs: u64,

    pub max_operations: usize,

    pub caching_enabled: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserManagementProcessorConfig {

    pub validation_enabled: bool,

    pub session_timeout_secs: u64,

    pub max_operations: usize,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRetryConfig {

    pub max_attempts: u32,

    pub initial_delay: Duration,

    pub max_delay: Duration,

    pub backoff_multiplier: f64,

    pub exponential_backoff: bool,

    pub retry_on_timeout: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNotificationConfig {

    pub email_enabled: bool,

    pub webhook_enabled: bool,

    pub in_app_enabled: bool,

    pub retry_config: WorkflowRetryConfig,

    pub email_settings: EmailNotificationSettings,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotificationSettings {

    pub smtp_host: String,

    pub smtp_port: u16,

    pub tls_enabled: bool,

    pub sender_email: String,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotificationSettings {

    pub timeout_secs: u64,

    pub signatures_enabled: bool,

    pub webhook_secret: Option<String>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {

    pub tier: ApprovalTier,

    pub required_count: u32,

    pub required_roles: Vec<String>,

    pub parallel_approvals: bool,

    pub timeout: Duration,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalTier {

    Basic,

    Enhanced,

    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoApprovalRule {

    pub name: String,

    pub condition: String,

    pub enabled: bool,

    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostWorkflowConfig {

    pub enabled: bool,

    pub compile_time_validation: bool,

    pub const_generics_enabled: bool,

    pub simd_enabled: bool,

    pub lock_free_enabled: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostMemoryPoolConfig {

    pub initial_size: usize,

    pub max_size: usize,

    pub enabled: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct WorkflowConfig {

    pub engine: WorkflowEngineConfig,

    pub policy: WorkflowPolicyConfig,

    pub processors: WorkflowProcessorConfig,

    pub notifications: WorkflowNotificationConfig,

    pub zero_cost: ZeroCostWorkflowConfig,

    pub storage: WorkflowStorageConfig,

    pub monitoring: WorkflowMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStorageConfig {

    pub backend_type: WorkflowStorageBackend,

    pub connection_string: Option<String>,

    pub encryption_enabled: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStorageBackend {

    Memory,

    PostgreSQL,

    Redis,

    FileSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRetentionPolicy {

    pub completed_retention: Duration,

    pub failed_retention: Duration,

    pub cancelled_retention: Duration,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMonitoringConfig {

    pub metrics_enabled: bool,

    pub health_monitoring_enabled: bool,

    pub metrics_interval: Duration,

    pub health_check_interval: Duration,

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

        assert_eq!(config.engine.max_concurrent, 100);
        assert_eq!(config.engine.timeout_secs, 300);
        assert!(config.engine.audit_enabled);
        assert!(config.engine.metrics_enabled);
        assert!(config.engine.persistence_enabled);

        assert_eq!(config.policy.max_concurrent_per_user, 10);
        assert_eq!(config.policy.max_concurrent_global, 1000);
        assert!(config.policy.auto_cleanup_enabled);
        assert!(config.policy.notifications_enabled);

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
