//! Integration and system configuration types
//!
//! Contains all workflow, adapter, and external system integration configuration structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::network::{InterServiceAuth, RetryConfig, TlsConfig};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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
}

/// Adapter configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterConfigs {
    /// Adapter configuration for external system integrations
    pub external_systems: AdapterConfig,
}

/// Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterConfig {
    /// Rust ecosystem integrations (always free/enabled)
    pub rust_ecosystem: RustEcosystemConfig,
    /// Licensed external system integrations
    pub external_systems: ExternalSystemsConfig,
}

/// Rust ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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
    pub tls: Option<TlsConfig>,
    /// Optional authentication (for inter-service auth)
    pub auth: Option<InterServiceAuth>,
}

/// External systems configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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
        Self {
            default_policy: "standard".to_string(),
            evaluation_timeout: Duration::from_secs(30),
            custom_policies: HashMap::new(),
        }
    }
}




