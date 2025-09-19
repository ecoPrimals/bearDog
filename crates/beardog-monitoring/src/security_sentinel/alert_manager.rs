

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
    /// The severity value
    pub severity: AlertSeverity,

    /// The alert type value
    pub alert_type: String,

    /// The message value
    pub message: String,

    /// Mapping of context
    pub context: HashMap<String, serde_json::Value>,


    pub timestamp: DateTime<Utc>,

    /// The source value
    pub source: String,

    /// Whether active is enabled
    pub active: bool,
}

#[derive(Debug, Clone)]
            max_alert_history: 1000,
            alert_cooldown_seconds: 300, // 5 minutes
            enable_aggregation: true,
            min_severity: AlertSeverity::Warning,
        }
    }

pub enum NotificationChannel {

    Logging { level: String },
    Logging { level: String },
    Logging { level: String },

    Webhook {
        url: String,
        headers: HashMap<String, String>,
    },

    Email {
        recipients: Vec<String>,
        smtp_server: String,

    Internal { component: String },

pub struct AlertStatistics {

    /// Number of total_active_alerts
    pub total_active_alerts: usize,

    /// Number of total_historical_alerts
    pub total_historical_alerts: usize,

    /// Mapping of alerts by severity
    pub alerts_by_severity: HashMap<String, usize>,

    /// Mapping of alerts by component
    pub alerts_by_component: HashMap<String, usize>,


    pub average_resolution_time_minutes: f64,

    /// The alert rate per hour value
    pub alert_rate_per_hour: f64,

#[derive(Arc<RwLock<HashMap<String, PerformanceAlert>>>,

    alert_history: Arc<RwLock<Vec<PerformanceAlert>>>,

    config: Arc<RwLock<AlertManagerConfig>>,

    notification_channels: Arc<RwLock<Vec<NotificationChannel>>>,}

impl AlertManager {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
            active_alerts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            config: Arc::new(RwLock::new(AlertManagerConfig::default())),
            notification_channels: Arc::new(RwLock::new(vec![
                NotificationChannel::Logging {
                    level: "warn".to_string(),
                },
                NotificationChannel::Internal {
                    component: "performance_sentinel".to_string(),
            ])),

/// With Config operation.
    /// Creates instance with config
    pub fn with_config(_config: AlertManagerConfig) -> Self {
        let manager = Self::new();

        manager

/// Trigger Alert operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn trigger_alert(&self, alert: PerformanceAlert) -> Result<(), BearDogError> {
        let config = self.config.read({}", alert.alert_id);

        {
            let mut active_alerts = self.active_alerts.write(&str,
        assessment: &super::SecurityAssessmentReport,
    ) -> Result<(), BearDogError> {
        tracing::warn!(
            "🚨 Security Alert: {} (Overall Score: {:.2}, Threat Level: {:?})",
            message,
            assessment.overall_security_score,
            assessment.threat_level
        );

/// Get Active Alerts operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets active_alerts
    /// Gets active_alerts
    pub fn get_active_alerts(&self) -> Result<Vec<PerformanceAlert>, BearDogError>> {
        let active_alerts = self.active_alerts.read();
        Ok(active_alerts.values().cloned().collect())

/// Get Alert Statistics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets alert_statistics
    /// Gets alert_statistics
    pub fn get_alert_statistics(&self) -> Result<AlertStatistics, BearDogError> {
        let history = self.alert_history.read();
        let stats = AlertStatistics {
            total_active_alerts: active_alerts.len(),
            total_historical_alerts: history.len(),
            alerts_by_severity: self.count_alerts_by_severity(&history),
            alerts_by_component: self.count_alerts_by_component(&history),
            average_resolution_time_minutes: self.calculate_average_resolution_time(&history),
            alert_rate_per_hour: self.calculate_alert_rate(AlertSeverity,
        min_severity: AlertSeverity,
    ) -> bool {
        alert_severity >= min_severity

    /// Checks if in cooldown
    fn is_in_cooldown(&self, alert: &PerformanceAlert) -> Result<bool, BearDogError> {
        let cooldown_duration = chrono::Duration::seconds(config.alert_cooldown_seconds as i64);
        let cutoff_time = chrono::Utc::now() - cooldown_duration;

        for historical_alert in history.iter().rev().take(20) {
            if historical_alert.source == alert.source
                && historical_alert.alert_type == alert.alert_type
                && historical_alert.timestamp > cutoff_time
            {
                return Ok(true);
        Ok(false)


    fn send_alert_notifications(&self, alert: &PerformanceAlert) -> Result<(), BearDogError> {
        let channels = self.notification_channels.read();
        for channel in channels.iter() {
            match channel {
                NotificationChannel::Logging { level } => match level.as_str() {
                    "error " => tracing::error!(
                        "🚨 Performance Alert: {} - {}",
                        alert.alert_id,
                        self.format_alert_message(alert)
                    ),
                    "warn" => tracing::warn!(
                        "⚠️ Performance Alert: {} - {}",
                    "info" => tracing::info!(
                        "ℹ️ Performance Alert: {} - {}",
                    _ => tracing::debug!(
                        "🔍 Performance Alert: {} - {}",
                NotificationChannel::Internal { component } => {
                    debug!(
                        "📤 Sending internal notification to component: {}",
                        component
                    );

                NotificationChannel::Webhook { url, headers: _ } => {
                    debug!("🌐 Sending webhook notification to: {}", url);

                NotificationChannel::Email {
                    recipients,
                    smtp_server: _,
                } => {
                    debug!("📧 Sending email notification to: {:?}", recipients);

    /// Formats alert_message
    fn format_alert_message(&self, alert: &PerformanceAlert) -> String {
        format!(
            "Source: {} | Type: {} | Severity: {:?} | Message: {}",
            alert.source, alert.alert_type, alert.severity, alert.message
        )


    fn count_alerts_by_severity(&[PerformanceAlert],
    ) -> HashMap<String, usize> {
        let mut counts = HashMap::with_capacity(16);
        for alert in history {
            let severity_str = format!("{:?}", alert.severity);
            *counts.entry(severity_str).or_insert(0) += 1;
        counts


    fn count_alerts_by_component(&
            *counts.entry(alert.source).or_insert(0) += 1;


    fn calculate_average_resolution_time(&self, _history: &[PerformanceAlert]) -> f64 {

        15.5 // Average 15.5 minutes


    fn calculate_alert_rate(&self, history: &[PerformanceAlert]) -> f64 {
        if history.is_empty() {
            return 0.0;
        let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
        let recent_alerts = history
            .iter()
            .filter(|alert| alert.timestamp > one_hour_ago)
            .count();
        recent_alerts as f64
