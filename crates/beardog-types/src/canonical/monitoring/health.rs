// SPDX-License-Identifier: AGPL-3.0-only

// Unified Health Monitoring Configuration
//
// This module consolidates all health monitoring configuration patterns from across
// the codebase into a single, canonical system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::MonitoringConfigValidation;

/// **UNIFIED HEALTH CONFIGURATION** - Consolidates all health monitoring configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHealthConfig {
    /// **CORE SETTINGS**
    /// Whether feature is enabled
    pub enabled: bool,
    /// Global Timeout
    pub global_timeout: Duration,
    /// Check Interval
    /// The check interval value
    pub check_interval: Duration,
    /// Failure Threshold
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Success Threshold
    /// Number of `success_threshold`
    pub success_threshold: u32,

    /// **HEALTH CHECK TYPES**
    /// The http checks value
    pub http_checks: HttpHealthCheckConfig,
    /// Tcp Checks
    /// The tcp checks value
    pub tcp_checks: TcpHealthCheckConfig,
    /// Database Checks
    /// The database checks value
    pub database_checks: DatabaseHealthCheckConfig,
    /// Service Checks
    /// The service checks value
    pub service_checks: ServiceHealthCheckConfig,
    /// Custom Checks
    /// Mapping of custom checks
    pub custom_checks: HashMap<String, CustomHealthCheckConfig>,

    /// **MONITORING SETTINGS**
    /// The monitoring value
    pub monitoring: HealthMonitoringConfig,
    /// Alerting
    /// The alerting value
    pub alerting: HealthAlertingConfig,
    /// Recovery
    /// The recovery value
    pub recovery: HealthRecoveryConfig,
}

impl Default for UnifiedHealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            global_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_GLOBAL_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            failure_threshold: std::env::var("BEARDOG_HEALTH_FAILURE_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            success_threshold: std::env::var("BEARDOG_HEALTH_SUCCESS_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
            http_checks: HttpHealthCheckConfig::default(),
            tcp_checks: TcpHealthCheckConfig::default(),
            database_checks: DatabaseHealthCheckConfig::default(),
            service_checks: ServiceHealthCheckConfig::default(),
            custom_checks: HashMap::new(),
            monitoring: HealthMonitoringConfig::default(),
            alerting: HealthAlertingConfig::default(),
            recovery: HealthRecoveryConfig::default(),
        }
    }
}

/// HTTP health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoints
    /// Collection of endpoints
    pub endpoints: Vec<HttpEndpoint>,
    /// Default Timeout
    pub default_timeout: Duration,
    /// Follow Redirects
    /// Whether `follow_redirects` is enabled
    pub follow_redirects: bool,
    /// Verify Ssl
    /// Whether `verify_ssl` is enabled
    pub verify_ssl: bool,
    /// User Agent
    /// The user agent value
    pub user_agent: String,
}

impl Default for HttpHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoints: Vec::new(),
            default_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HTTP_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            ),
            follow_redirects: true,
            verify_ssl: true,
            user_agent: "BearDog-HealthChecker/4.0.0".to_string(),
        }
    }
}

/// HTTP endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpEndpoint {
    /// Name
    /// Name of the item
    pub name: String,
    /// Url
    /// The url value
    pub url: String,
    /// Method
    /// The method value
    pub method: HttpMethod,
    /// Headers
    /// Mapping of headers
    pub headers: HashMap<String, String>,
    /// Expected Status
    /// Current status of the expected
    pub expected_status: Vec<u16>,
    /// Expected Body
    /// Optional expected body
    pub expected_body: Option<String>,
    /// Timeout
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    /// GET variant
    GET,
    /// POST variant
    POST,
    /// PUT variant
    PUT,
    /// HEAD variant
    HEAD,
    /// OPTIONS variant
    OPTIONS,
}

/// TCP health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpHealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoints
    /// Collection of endpoints
    pub endpoints: Vec<TcpEndpoint>,
    /// Default Timeout
    pub default_timeout: Duration,
    /// Connection Reuse
    /// Whether `connection_reuse` is enabled
    pub connection_reuse: bool,
}

impl Default for TcpHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoints: Vec::new(),
            default_timeout: Duration::from_secs(
                std::env::var("BEARDOG_TCP_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
            ),
            connection_reuse: false,
        }
    }
}

/// TCP endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpEndpoint {
    /// Name
    /// Name of the item
    pub name: String,
    /// Host
    /// The host value
    pub host: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// Timeout
    pub timeout: Option<Duration>,
}

/// Database health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Connections
    /// Collection of connections
    pub connections: Vec<DatabaseConnection>,
    /// Default Timeout
    pub default_timeout: Duration,
    /// Test Query
    /// The test query value
    pub test_query: String,
}

impl Default for DatabaseHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            connections: Vec::new(),
            default_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DB_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(15),
            ),
            test_query: "SELECT 1".to_string(),
        }
    }
}

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    /// Name
    /// Name of the item
    pub name: String,
    /// Connection String
    /// The connection string value
    pub connection_string: String,
    /// Database Type
    /// The database type value
    pub database_type: DatabaseType,
    /// Timeout
    pub timeout: Option<Duration>,
    /// Custom Query
    /// Optional custom query
    pub custom_query: Option<String>,
}

/// Database types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of database
pub enum DatabaseType {
    /// `PostgreSQL` variant
    PostgreSQL,
    /// `MySQL` variant
    MySQL,
    /// `SQLite` variant
    SQLite,
    /// `MongoDB` variant
    MongoDB,
    /// Redis variant
    Redis,
    /// Custom health check type
    Custom(String),
}

/// Service health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Services
    /// Collection of services
    pub services: Vec<ServiceEndpoint>,
    /// Discovery Enabled
    /// Whether discovery is enabled
    pub discovery_enabled: bool,
    /// Auto Discovery Interval
    /// The auto discovery interval value
    pub auto_discovery_interval: Duration,
}

impl Default for ServiceHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            services: Vec::new(),
            discovery_enabled: true,
            auto_discovery_interval: Duration::from_secs(
                std::env::var("BEARDOG_SERVICE_DISCOVERY_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
        }
    }
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Name
    /// Name of the item
    pub name: String,
    /// Service Type
    /// The service type value
    pub service_type: ServiceType,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Health Path
    /// The health path value
    pub health_path: String,
    /// Timeout
    pub timeout: Option<Duration>,
    /// Metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Service types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of service
pub enum ServiceType {
    /// HTTP variant
    HTTP,
    /// `GRpc` variant
    GRpc,
    /// WebSocket variant
    WebSocket,
    /// `MessageQueue` variant
    MessageQueue,
    /// Database variant
    Database,
    /// Cache variant
    Cache,
    /// Custom health check category
    Custom(String),
}

/// Custom health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomHealthCheckConfig {
    /// Name
    /// Name of the item
    pub name: String,
    /// Check Type
    /// The check type value
    pub check_type: String,
    /// Config
    pub config: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout: Duration,
    /// Interval
    /// The interval value
    pub interval: Duration,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Track Response Times
    pub track_response_times: bool,
    /// Track Failure Patterns
    /// Whether `track_failure_patterns` is enabled
    pub track_failure_patterns: bool,
    /// Store History
    /// Whether `store_history` is enabled
    pub store_history: bool,
    /// History Retention
    /// The history retention value
    pub history_retention: Duration,
    /// Metrics Export
    /// Whether `metrics_export` is enabled
    pub metrics_export: bool,
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            track_response_times: true,
            track_failure_patterns: true,
            store_history: true,
            history_retention: Duration::from_secs(86400 * 7), // 7 days
            metrics_export: true,
        }
    }
}

/// Health alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAlertingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Alert On Failure
    /// Whether `alert_on_failure` is enabled
    pub alert_on_failure: bool,
    /// Alert On Recovery
    /// Whether `alert_on_recovery` is enabled
    pub alert_on_recovery: bool,
    /// Alert On Degraded
    /// Whether `alert_on_degraded` is enabled
    pub alert_on_degraded: bool,
    /// Notification Channels
    /// Collection of notification channels
    pub notification_channels: Vec<String>,
    /// Escalation Rules
    /// Collection of escalation rules
    pub escalation_rules: Vec<EscalationRule>,
}

impl Default for HealthAlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alert_on_failure: true,
            alert_on_recovery: true,
            alert_on_degraded: true,
            notification_channels: Vec::new(),
            escalation_rules: Vec::new(),
        }
    }
}

/// Escalation rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Name
    /// Name of the item
    pub name: String,
    /// Condition
    /// The condition value
    pub condition: EscalationCondition,
    /// Delay
    /// The delay value
    pub delay: Duration,
    /// Channels
    /// Collection of channels
    pub channels: Vec<String>,
}

/// Escalation conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    /// Escalate after a specific number of failures
    FailureCount(u32),
    FailureDuration(Duration),
    /// Escalate when failure percentage exceeds threshold
    FailurePercentage(f64),
    /// Custom escalation condition with parameters
    Custom {
        /// Condition expression or identifier
        condition: String,
        /// Condition-specific parameters
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Health recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRecoveryConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Auto Recovery Attempts
    /// Number of `auto_recovery_attempts`
    pub auto_recovery_attempts: u32,
    /// Recovery Delay
    /// The recovery delay value
    pub recovery_delay: Duration,
    /// Recovery Backoff
    /// The recovery backoff value
    pub recovery_backoff: f64,
    /// Max Recovery Delay
    /// The max recovery delay value
    pub max_recovery_delay: Duration,
    /// Recovery Actions
    /// Collection of recovery actions
    pub recovery_actions: Vec<RecoveryAction>,
}

impl Default for HealthRecoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_recovery_attempts: std::env::var("BEARDOG_AUTO_RECOVERY_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            recovery_delay: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_RECOVERY_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            recovery_backoff: std::env::var("BEARDOG_HEALTH_RECOVERY_BACKOFF")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2.0),
            max_recovery_delay: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_MAX_RECOVERY_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
            recovery_actions: Vec::new(),
        }
    }
}

/// Recovery action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAction {
    /// Name
    /// Name of the item
    pub name: String,
    /// Action Type
    /// The action type value
    pub action_type: RecoveryActionType,
    /// Config
    pub config: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout: Duration,
}

/// Recovery action types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of recovery action
pub enum RecoveryActionType {
    /// `RestartService` variant
    RestartService,
    /// `ClearCache` variant
    ClearCache,
    /// `ReconnectDatabase` variant
    ReconnectDatabase,
    /// `ScaleService` variant
    ScaleService,
    /// `RunScript` variant
    RunScript,
    /// Custom action with user-defined command
    Custom(String),
}

impl MonitoringConfigValidation for UnifiedHealthConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.check_interval.is_zero() {
            return Err(BearDogError::business(
                "Health check interval must be greater than zero".to_string(),
            ));
        }

        if self.global_timeout.is_zero() {
            return Err(BearDogError::business(
                "Global timeout must be greater than zero".to_string(),
            ));
        }

        if self.failure_threshold == 0 {
            return Err(BearDogError::business(
                "Failure threshold must be greater than zero".to_string(),
            ));
        }

        if self.success_threshold == 0 {
            return Err(BearDogError::business(
                "Success threshold must be greater than zero".to_string(),
            ));
        }

        // Validate HTTP endpoints
        for endpoint in &self.http_checks.endpoints {
            if endpoint.name.is_empty() {
                return Err(BearDogError::business(
                    "HTTP endpoint name cannot be empty".to_string(),
                ));
            }
            if endpoint.url.is_empty() {
                return Err(BearDogError::business(
                    "HTTP endpoint URL cannot be empty".to_string(),
                ));
            }
            if endpoint.expected_status.is_empty() {
                return Err(BearDogError::business(
                    "HTTP endpoint must have at least one expected status code".to_string(),
                ));
            }
        }

        // Validate TCP endpoints
        for endpoint in &self.tcp_checks.endpoints {
            if endpoint.name.is_empty() {
                return Err(BearDogError::business(
                    "TCP endpoint name cannot be empty".to_string(),
                ));
            }
            if endpoint.host.is_empty() {
                return Err(BearDogError::business(
                    "TCP endpoint host cannot be empty".to_string(),
                ));
            }
            if endpoint.port == 0 {
                return Err(BearDogError::business(
                    "TCP endpoint port must be greater than zero".to_string(),
                ));
            }
        }

        // Validate database connections
        for connection in &self.database_checks.connections {
            if connection.name.is_empty() {
                return Err(BearDogError::business(
                    "Database connection name cannot be empty".to_string(),
                ));
            }
            if connection.connection_string.is_empty() {
                return Err(BearDogError::business(
                    "Database connection string cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
