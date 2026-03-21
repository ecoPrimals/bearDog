// SPDX-License-Identifier: AGPL-3.0-only

//! Operational metrics and alerting configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Metric types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of metric
pub enum MetricType {
    /// Request latency
    RequestLatency,
    /// Request throughput
    RequestThroughput,
    /// Error rate
    ErrorRate,
    /// Model accuracy
    ModelAccuracy,
    /// Resource utilization
    ResourceUtilization,
    /// Prediction confidence
    PredictionConfidence,
}

/// Alerting configuration for model monitoring notifications
///
/// Configures rules and channels for alerting operators when models
/// exhibit anomalous behavior or performance degradation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Collection of alert rules defining conditions that trigger notifications
    pub rules: Vec<AlertRule>,
    /// Collection of notification channels for delivering alerts (email, Slack, SMS, webhook)
    pub channels: Vec<NotificationChannel>,
}

/// Alert rule definition for monitoring thresholds
///
/// Defines a monitoring rule that triggers an alert when a metric
/// crosses a threshold within a specified time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Human-readable name identifying this alert rule
    pub name: String,
    /// Type of metric to monitor (latency, throughput, error rate, accuracy, etc.)
    pub metric: MetricType,
    /// Numeric threshold value that triggers the alert when crossed
    pub threshold: f64,
    /// Comparison operator for threshold evaluation (>, <, ==, >=, <=)
    pub operator: ComparisonOperator,
    /// Time window over which to evaluate the metric before triggering
    pub window: Duration,
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    EqualTo,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
}

/// Notification channels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationChannel {
    /// Email notification
    Email,
    /// Slack notification
    Slack,
    /// SMS notification
    Sms,
    /// Webhook notification
    Webhook,
}
