//! Integration and system configuration types
//!
//! Contains all workflow, adapter, and external system integration configuration structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::network::{NodeCommunicationConfig, RetryConfig};

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Default approval timeout
    pub default_approval_timeout: Duration,
    /// Maximum concurrent workflows
    pub max_concurrent_workflows: usize,
    /// Workflow storage configuration
    pub storage: WorkflowStorageConfig,
    /// Notification configuration
    pub notifications: NotificationConfig,
    /// Policy configuration
    pub policies: PolicyConfig,
    /// User role mappings (user_id -> role)
    pub role_mappings: Option<HashMap<String, String>>,
    /// List of admin users
    pub admin_users: Option<Vec<String>>,
    /// List of approver users
    pub approver_users: Option<Vec<String>>,
}

/// Workflow storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStorageConfig {
    /// Storage type
    pub storage_type: String,
    /// Storage configuration
    pub config: HashMap<String, String>,
}

/// Notification configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationConfig {
    /// Enable notifications
    pub enabled: bool,
    /// Email notification settings
    pub email: EmailConfig,
    /// Webhook notification settings
    pub webhook: WebhookConfig,
}

/// Email notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// Enable email notifications
    pub enabled: bool,
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Username for SMTP auth
    pub username: String,
    /// Password for SMTP auth
    pub password: String,
    /// From address
    pub from_address: String,
}

/// Webhook notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Enable webhook notifications
    pub enabled: bool,
    /// Webhook URL
    pub url: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Timeout for webhook requests
    pub timeout: Duration,
}

/// Policy configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Default policy type
    pub default_policy: String,
    /// Policy evaluation timeout
    pub evaluation_timeout: Duration,
    /// Custom policies
    pub custom_policies: HashMap<String, String>,
    /// Eligible users by workflow type
    pub eligible_users_by_workflow: Option<HashMap<String, Vec<String>>>,
    /// Eligible users by role
    pub eligible_users_by_role: Option<HashMap<String, Vec<String>>>,
}

/// Adapter configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdapterConfigs {
    /// Adapter configuration for external system integrations
    pub external_systems: AdapterConfig,
}

/// Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdapterConfig {
    /// Rust ecosystem integrations (always free/enabled)
    pub rust_ecosystem: RustEcosystemConfig,
    /// Licensed external system integrations
    pub external_systems: ExternalSystemsConfig,
}

/// Rust ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RustEcosystemConfig {
    /// NestGate secure file transfer (if part of your Rust ecosystem)
    pub nestgate: Option<RustProjectConfig>,
    /// SongBird communication platform (if part of your Rust ecosystem)
    pub songbird: Option<RustProjectConfig>,
    /// Other Rust projects in your ecosystem
    pub additional_projects: HashMap<String, RustProjectConfig>,
}

/// Rust project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustProjectConfig {
    /// Always enabled for Rust ecosystem
    pub enabled: bool,
    /// Project endpoint URL
    pub endpoint: String,
    /// Connection timeout
    pub timeout_ms: u32,
    /// TLS configuration
    pub tls: Option<NodeCommunicationConfig>,
    /// Optional authentication (for inter-service auth)
    pub auth: Option<NodeCommunicationConfig>,
}

/// External systems configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalSystemsConfig {
    /// Enterprise HSM integrations
    pub hsm_systems: HashMap<String, ExternalSystemConfig>,
    /// SIEM system integrations
    pub siem_systems: HashMap<String, ExternalSystemConfig>,
    /// Enterprise database integrations
    pub database_systems: HashMap<String, ExternalSystemConfig>,
    /// Cloud service integrations
    pub cloud_services: HashMap<String, ExternalSystemConfig>,
    /// Authentication system integrations
    pub auth_systems: HashMap<String, ExternalSystemConfig>,
    /// Backup system integrations
    pub backup_systems: HashMap<String, ExternalSystemConfig>,
    /// Messaging system integrations
    pub messaging_systems: HashMap<String, ExternalSystemConfig>,
}

/// External system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSystemConfig {
    /// Requires valid license to enable
    pub enabled: bool,
    /// System endpoint URL
    pub endpoint: String,
    /// Connection timeout
    pub timeout_ms: u32,
    /// License file path
    pub license_file: Option<String>,
    /// System-specific configuration
    pub system_config: HashMap<String, String>,
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
}

// Default implementations
impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            default_approval_timeout: Duration::from_secs(24 * 3600), // 24 hours
            max_concurrent_workflows: 100,
            storage: WorkflowStorageConfig::default(),
            notifications: NotificationConfig::default(),
            policies: PolicyConfig::default(),
            role_mappings: None,
            admin_users: Some(vec!["admin".to_string(), "root".to_string()]),
            approver_users: Some(vec!["approver".to_string(), "manager".to_string()]),
        }
    }
}

impl Default for WorkflowStorageConfig {
    fn default() -> Self {
        Self {
            storage_type: "in_memory".to_string(),
            config: {
                let mut config = HashMap::new();
                config.insert("max_entries".to_string(), "10000".to_string());
                config
            },
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_server: super::constants::endpoints::DEFAULT_SMTP_SERVER.to_string(),
            smtp_port: 587,
            username: "".to_string(),
            password: "".to_string(),
            from_address: "noreply@beardog.local".to_string(),
        }
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "".to_string(),
            auth_token: None,
            timeout: Duration::from_secs(10),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        let mut eligible_by_workflow = HashMap::new();
        eligible_by_workflow.insert(
            "key_rotation".to_string(),
            vec![
                "admin".to_string(),
                "security_admin".to_string(),
                "key_manager".to_string(),
            ],
        );
        eligible_by_workflow.insert(
            "key_deletion".to_string(),
            vec![
                "admin".to_string(),
                "security_admin".to_string(),
                "key_manager".to_string(),
            ],
        );
        eligible_by_workflow.insert(
            "policy_change".to_string(),
            vec![
                "admin".to_string(),
                "policy_admin".to_string(),
                "system_admin".to_string(),
            ],
        );
        eligible_by_workflow.insert(
            "configuration_change".to_string(),
            vec![
                "admin".to_string(),
                "policy_admin".to_string(),
                "system_admin".to_string(),
            ],
        );
        eligible_by_workflow.insert(
            "user_provisioning".to_string(),
            vec![
                "admin".to_string(),
                "hr_admin".to_string(),
                "user_admin".to_string(),
            ],
        );
        eligible_by_workflow.insert(
            "emergency_access".to_string(),
            vec![
                "admin".to_string(),
                "emergency_contact".to_string(),
                "security_admin".to_string(),
            ],
        );

        let mut eligible_by_role = HashMap::new();
        eligible_by_role.insert(
            "admin".to_string(),
            vec![
                "admin".to_string(),
                "root".to_string(),
                "system.admin".to_string(),
            ],
        );
        eligible_by_role.insert(
            "security_admin".to_string(),
            vec![
                "security.admin".to_string(),
                "sec.admin".to_string(),
                "security_officer".to_string(),
            ],
        );
        eligible_by_role.insert(
            "key_manager".to_string(),
            vec![
                "key.manager".to_string(),
                "crypto.admin".to_string(),
                "security.key_manager".to_string(),
            ],
        );

        Self {
            default_policy: "standard".to_string(),
            evaluation_timeout: Duration::from_secs(30),
            custom_policies: HashMap::new(),
            eligible_users_by_workflow: Some(eligible_by_workflow),
            eligible_users_by_role: Some(eligible_by_role),
        }
    }
}
