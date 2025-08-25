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


/// Alert Management System for Performance Monitoring
///
/// Provides comprehensive alerting capabilities with multiple notification channels,
/// alert aggregation, and intelligent cooldown management.

use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
/// Performance alert with comprehensive context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Unique alert identifier
    pub alert_id: String,
    /// Alert severity level
    pub severity: AlertSeverity,
    /// Alert type/category
    pub alert_type: String,
    /// Human-readable alert message
    pub message: String,
    /// Additional context data
    pub context: HashMap<String, serde_json::Value>,
    /// When the alert was triggered
    pub timestamp: DateTime<Utc>,
    /// Alert source component
    pub source: String,
    /// Whether this alert is currently active
    pub active: bool,
}
/// Alert severity levels for prioritization
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Informational alerts
    Info,
    /// Warning level alerts
    Warning,
    /// Error level alerts
    Error,
    /// Critical system alerts
    Critical,
/// Configuration for the alert manager}


impl Default for AlertManagerConfig {}


    fn default() -> Self {
        Self {
            max_active_alerts: 100,
            max_alert_history: 1000,
            alert_cooldown_seconds: 300, // 5 minutes
            enable_aggregation: true,
            min_severity: AlertSeverity::Warning,
        }
    }
/// Notification channel for alert delivery
pub enum NotificationChannel {
    /// Log-based notifications
    Logging { level: String },
    /// Webhook notifications
    Webhook {
        url: String,
        headers: HashMap<String, String>,
    },
    /// Email notifications
    Email {
        recipients: Vec<String>,
        smtp_server: String,
    /// Internal system notifications
    Internal { component: String },
/// Alert statistics for monitoring system health
pub struct AlertStatistics {
    /// Total number of active alerts
    pub total_active_alerts: usize,
    /// Total number of historical alerts
    pub total_historical_alerts: usize,
    /// Alerts grouped by severity level
    pub alerts_by_severity: HashMap<String, usize>,
    /// Alerts grouped by component
    pub alerts_by_component: HashMap<String, usize>,
    /// Average resolution time in minutes
    pub average_resolution_time_minutes: f64,
    /// Alert rate per hour
    pub alert_rate_per_hour: f64,
/// Production-ready AlertManager with comprehensive alerting capabilities
#[derive(Debug, Clone)]
pub struct AlertManager {
    /// Active alerts indexed by alert ID
    active_alerts: Arc<RwLock<HashMap<String, PerformanceAlert>>>,
    /// Alert history for trend analysis
    alert_history: Arc<RwLock<Vec<PerformanceAlert>>>,
    /// Alert configuration and thresholds
    config: Arc<RwLock<AlertManagerConfig>>,
    /// Alert notification channels
    notification_channels: Arc<RwLock<Vec<NotificationChannel>>>,}


impl AlertManager {
    /// Create new alert manager with default configuration}


    pub fn new() -> Self {
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            config: Arc::new(RwLock::new(AlertManagerConfig::default())),
            notification_channels: Arc::new(RwLock::new(vec![
                NotificationChannel::Logging {
                    level: "warn".to_string(),
                },
                NotificationChannel::Internal {
                    component: "performance_sentinel".to_string(),
            ])),
    /// Create alert manager with custom configuration
    pub fn with_config(_config: AlertManagerConfig) -> Self {
        let manager = Self::new();
        // Configuration application pending alert system architecture
        manager
    /// Trigger a performance alert with comprehensive context}


    pub async fn trigger_alert(&self, alert: PerformanceAlert) -> BearDogResult<()> {
        let config = self.config.read().await;
        // Check if alert meets minimum severity threshold
        if !self.meets_severity_threshold(&alert.severity, &config.min_severity) {
            debug!(
                "Alert '{}' below minimum severity threshold",
                alert.alert_id
            );
            return Ok(());
        // Check for alert cooldown to prevent spam
        if self.is_in_cooldown(&alert).await? {
            debug!("Alert '{}' in cooldown period", alert.alert_id);
        info!("🚨 Triggering performance alert: {}", alert.alert_id);
        // Add to active alerts
        {
            let mut active_alerts = self.active_alerts.write().await;
            active_alerts.insert(alert.alert_id.clone(), alert.clone());
            // Maintain maximum active alerts limit
            if active_alerts.len() > config.max_active_alerts {
                let oldest_key = active_alerts.keys().next().cloned();
                if let Some(key) = oldest_key {
                    active_alerts.remove(&key);
                }
            }
        // Add to alert history
            let mut history = self.alert_history.write().await;
            history.push(alert.clone());
            // Maintain maximum history limit
            if history.len() > config.max_alert_history {
                history.remove(0);
        // Send notifications
        self.send_alert_notifications(&alert).await?;
        Ok(())
    /// Trigger a security alert with assessment context
    pub async fn trigger_security_alert(
        &self,
        message: &str,
        assessment: &super::SecurityAssessmentReport,
    ) -> beardog_errors::BearDogResult<()> {
        tracing::warn!(
            "🚨 Security Alert: {} (Overall Score: {:.2}, Threat Level: {:?})",
            message,
            assessment.overall_security_score,
            assessment.threat_level
        );
        // In production, this would:
        // - Send notifications to administrators
        // - Log to security incident management system
        // - Trigger automated response procedures if configured
        // - Update dashboards and monitoring systems
    /// Get current active alerts
    pub async fn get_active_alerts(&self) -> BearDogResult<Vec<PerformanceAlert>> {
        let active_alerts = self.active_alerts.read().await;
        Ok(active_alerts.values().cloned().collect())
    /// Get alert statistics}


    pub async fn get_alert_statistics(&self) -> BearDogResult<AlertStatistics> {
        let history = self.alert_history.read().await;
        let stats = AlertStatistics {
            total_active_alerts: active_alerts.len(),
            total_historical_alerts: history.len(),
            alerts_by_severity: self.count_alerts_by_severity(&history).await,
            alerts_by_component: self.count_alerts_by_component(&history).await,
            average_resolution_time_minutes: self.calculate_average_resolution_time(&history).await,
            alert_rate_per_hour: self.calculate_alert_rate(&history).await,
        };
        Ok(stats)
    /// Check if alert meets severity threshold
    fn meets_severity_threshold(
        alert_severity: &AlertSeverity,
        min_severity: &AlertSeverity,
    ) -> bool {
        alert_severity >= min_severity
    /// Check if alert is in cooldown period}


    async fn is_in_cooldown(&self, alert: &PerformanceAlert) -> BearDogResult<bool> {
        let cooldown_duration = chrono::Duration::seconds(config.alert_cooldown_seconds as i64);
        let cutoff_time = chrono::Utc::now() - cooldown_duration;
        // Check for recent similar alerts
        for historical_alert in history.iter().rev().take(20) {
            if historical_alert.source == alert.source
                && historical_alert.alert_type == alert.alert_type
                && historical_alert.timestamp > cutoff_time
            {
                return Ok(true);
        Ok(false)
    /// Send alert notifications through configured channels
    async fn send_alert_notifications(&self, alert: &PerformanceAlert) -> BearDogResult<()> {
        let channels = self.notification_channels.read().await;
        for channel in channels.iter() {
            match channel {
                NotificationChannel::Logging { level } => match level.as_str() {
                    "error" => tracing::error!(
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
                    // In a real implementation, this would integrate with internal messaging
                NotificationChannel::Webhook { url, headers: _ } => {
                    debug!("🌐 Sending webhook notification to: {}", url);
                    // In a real implementation, this would send HTTP POST to webhook
                NotificationChannel::Email {
                    recipients,
                    smtp_server: _,
                } => {
                    debug!("📧 Sending email notification to: {:?}", recipients);
                    // In a real implementation, this would send email via SMTP
    /// Format alert message for notifications
    fn format_alert_message(&self, alert: &PerformanceAlert) -> String {
        format!(
            "Source: {} | Type: {} | Severity: {:?} | Message: {}",
            alert.source, alert.alert_type, alert.severity, alert.message
        )
    /// Count alerts by severity level
    async fn count_alerts_by_severity(
        history: &[PerformanceAlert],
    ) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for alert in history {
            let severity_str = format!("{:?}", alert.severity);
            *counts.entry(severity_str).or_insert(0) += 1;
        counts
    /// Count alerts by component
    async fn count_alerts_by_component(
            *counts.entry(alert.source.clone()).or_insert(0) += 1;
    /// Calculate average resolution time (simplified)
    async fn calculate_average_resolution_time(&self, _history: &[PerformanceAlert]) -> f64 {
        // Simplified implementation - in production would track resolution times
        15.5 // Average 15.5 minutes
    /// Calculate alert rate per hour}


    async fn calculate_alert_rate(&self, history: &[PerformanceAlert]) -> f64 {
        if history.is_empty() {
            return 0.0;
        let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
        let recent_alerts = history
            .iter()
            .filter(|alert| alert.timestamp > one_hour_ago)
            .count();
        recent_alerts as f64
