// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Alerting Configuration
//
// This module consolidates all alerting configuration patterns from across the codebase.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::MonitoringConfigValidation;

/// **UNIFIED ALERTING CONFIGURATION** - Consolidates all alerting configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAlertingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Evaluation Interval
    /// The evaluation interval value
    pub evaluation_interval: Duration,
    /// Notification Timeout
    pub notification_timeout: Duration,
    /// Max Alerts Per Minute
    /// Number of `max_alerts_per_minute`
    pub max_alerts_per_minute: u32,

    /// **ALERT RULES**
    /// Collection of rules
    pub rules: Vec<AlertRule>,
    /// Rule Groups
    /// Collection of rule groups
    pub rule_groups: Vec<AlertRuleGroup>,

    /// **NOTIFICATION SETTINGS**
    /// The notifications value
    pub notifications: AlertNotificationConfig,
    /// Escalation
    /// The escalation value
    pub escalation: AlertEscalationConfig,
    /// Suppression
    /// The suppression value
    pub suppression: AlertSuppressionConfig,
}

impl Default for UnifiedAlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            evaluation_interval: Duration::from_secs(
                std::env::var(env_keys::ENV_ALERT_EVALUATION_INTERVAL_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            notification_timeout: Duration::from_secs(
                std::env::var(env_keys::ENV_ALERT_NOTIFICATION_TIMEOUT_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_alerts_per_minute: std::env::var(env_keys::ENV_MAX_ALERTS_PER_MINUTE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            rules: Vec::new(),
            rule_groups: Vec::new(),
            notifications: AlertNotificationConfig::default(),
            escalation: AlertEscalationConfig::default(),
            suppression: AlertSuppressionConfig::default(),
        }
    }
}

/// Alert rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Name
    /// Name of the item
    pub name: String,
    /// Condition
    /// The condition value
    pub condition: AlertCondition,
    /// Severity
    /// The severity value
    pub severity: AlertSeverity,
    /// Duration
    /// The duration value
    pub duration: Duration,
    /// Labels
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Annotations
    /// Mapping of annotations
    pub annotations: HashMap<String, String>,
}

/// Alert conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    /// Metric threshold alert condition
    MetricThreshold {
        /// Name of the metric to monitor
        metric: String,
        /// Comparison applied to the sampled metric vs the `threshold` field.
        operator: ComparisonOperator,
        /// Threshold value to compare against
        threshold: f64,
    },
    /// Health check failure threshold exceeded
    HealthCheckFailure {
        /// Service name that failed health checks
        service: String,
        /// Consecutive failures required before firing.
        failure_count: u32,
    },
    /// Error rate threshold configuration
    ErrorRate {
        /// Service or dependency whose error budget is tracked.
        service: String,
        /// Error rate threshold (0.0 to 1.0) that triggers alert
        rate_threshold: f64,
        /// Time window over which to calculate error rate
        time_window: Duration,
    },
    /// Custom alert condition with user-defined parameters
    Custom {
        /// Expression language snippet evaluated by the alerting engine.
        expression: String,
        /// Bound parameters for the expression.
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// `GreaterThan` variant
    GreaterThan,
    /// `LessThan` variant
    LessThan,
    /// Equal variant
    Equal,
    /// `NotEqual` variant
    NotEqual,
    /// `GreaterThanOrEqual` variant
    GreaterThanOrEqual,
    /// `LessThanOrEqual` variant
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Critical variant
    Critical,
    /// High variant
    High,
    /// Medium variant
    Medium,
    /// Low variant
    Low,
    /// Info variant
    Info,
}

/// Alert rule group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRuleGroup {
    /// Name
    /// Name of the item
    pub name: String,
    /// Interval
    /// The interval value
    pub interval: Duration,
    /// Rules
    /// Collection of rules
    pub rules: Vec<String>, // Rule names
}

/// Alert notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotificationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Channels
    /// Collection of channels
    pub channels: Vec<NotificationChannel>,
    /// Templates
    /// Mapping of templates
    pub templates: HashMap<String, NotificationTemplate>,
    /// Routing
    /// The routing value
    pub routing: NotificationRouting,
}

impl Default for AlertNotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: Vec::new(),
            templates: HashMap::new(),
            routing: NotificationRouting::default(),
        }
    }
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Name
    /// Name of the item
    pub name: String,
    /// Channel Type
    /// The channel type value
    pub channel_type: NotificationChannelType,
    /// Config
    pub config: HashMap<String, serde_json::Value>,
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Notification channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of notification channel
pub enum NotificationChannelType {
    /// Email variant
    Email,
    /// Slack variant
    Slack,
    /// Discord variant
    Discord,
    /// `PagerDuty` variant
    PagerDuty,
    /// Webhook variant
    Webhook,
    /// SMS variant
    SMS,
    /// Custom alert severity level
    Custom(String),
}

/// Notification template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    /// Name
    /// Name of the item
    pub name: String,
    /// Subject
    /// The subject value
    pub subject: String,
    /// Body
    /// The body value
    pub body: String,
    /// Format
    pub format: TemplateFormat,
}

/// Formatting mode for outbound alert bodies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateFormat {
    /// `PlainText` variant
    PlainText,
    /// HTML variant
    HTML,
    /// Markdown variant
    Markdown,
    /// JSON variant
    JSON,
}

/// Notification routing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationRouting {
    /// Default Channel
    /// Optional default channel
    pub default_channel: Option<String>,
    /// Routes
    /// Collection of routes
    pub routes: Vec<RoutingRule>,
}

/// Routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Name
    /// Name of the item
    pub name: String,
    /// Matchers
    /// Collection of matchers
    pub matchers: Vec<RoutingMatcher>,
    /// Channels
    /// Collection of channels
    pub channels: Vec<String>,
    /// Continue Routing
    /// Whether `continue_routing` is enabled
    pub continue_routing: bool,
}

/// Routing matcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingMatcher {
    /// Label
    /// The label value
    pub label: String,
    /// Operator
    /// The operator value
    pub operator: MatchOperator,
    /// Value
    /// The value value
    pub value: String,
}

/// Match operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchOperator {
    /// Equals variant
    Equals,
    /// `NotEquals` variant
    NotEquals,
    /// Matches variant
    Matches,
    /// `NotMatches` variant
    NotMatches,
}

/// Alert escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertEscalationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Escalation Rules
    /// Collection of escalation rules
    pub escalation_rules: Vec<EscalationRule>,
}

/// Escalation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Name
    /// Name of the item
    pub name: String,
    /// Trigger After
    /// The trigger after value
    pub trigger_after: Duration,
    /// Channels
    /// Collection of channels
    pub channels: Vec<String>,
    /// Severity Filter
    /// Optional severity filter
    pub severity_filter: Option<AlertSeverity>,
}

/// Alert suppression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSuppressionConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Suppression Rules
    /// Collection of suppression rules
    pub suppression_rules: Vec<SuppressionRule>,
    /// Maintenance Windows
    /// Collection of maintenance windows
    pub maintenance_windows: Vec<MaintenanceWindow>,
}

impl Default for AlertSuppressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            suppression_rules: Vec::new(),
            maintenance_windows: Vec::new(),
        }
    }
}

/// Suppression rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionRule {
    /// Name
    /// Name of the item
    pub name: String,
    /// Matchers
    /// Collection of matchers
    pub matchers: Vec<RoutingMatcher>,
    /// Duration
    /// The duration value
    pub duration: Duration,
}

/// Maintenance window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Name
    /// Name of the item
    pub name: String,
    /// Start Time
    pub start_time: String, // Cron expression
    /// Duration
    /// The duration value
    pub duration: Duration,
    /// Services
    /// Collection of services
    pub services: Vec<String>,
}

impl MonitoringConfigValidation for UnifiedAlertingConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.evaluation_interval.is_zero() {
            return Err(BearDogError::business(
                "Evaluation interval must be greater than zero".to_string(),
            ));
        }

        if self.notification_timeout.is_zero() {
            return Err(BearDogError::business(
                "Notification timeout must be greater than zero".to_string(),
            ));
        }

        // Validate alert rules
        for rule in &self.rules {
            if rule.name.is_empty() {
                return Err(BearDogError::business(
                    "Alert rule name cannot be empty".to_string(),
                ));
            }
            if rule.duration.is_zero() {
                return Err(BearDogError::business(
                    "Alert rule duration must be greater than zero".to_string(),
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
