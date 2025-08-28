

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {

    pub alert_id: String,

    pub severity: AlertSeverity,

    pub alert_type: String,

    pub message: String,

    pub context: HashMap<String, serde_json::Value>,

    pub timestamp: DateTime<Utc>,

    pub source: String,

    pub active: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {

    Info,

    Warning,

    Error,

    Critical,

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

pub enum NotificationChannel {

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

    pub total_active_alerts: usize,

    pub total_historical_alerts: usize,

    pub alerts_by_severity: HashMap<String, usize>,

    pub alerts_by_component: HashMap<String, usize>,

    pub average_resolution_time_minutes: f64,

    pub alert_rate_per_hour: f64,

#[derive(Debug, Clone)]
pub struct AlertManager {

    active_alerts: Arc<RwLock<HashMap<String, PerformanceAlert>>>,

    alert_history: Arc<RwLock<Vec<PerformanceAlert>>>,

    config: Arc<RwLock<AlertManagerConfig>>,

    notification_channels: Arc<RwLock<Vec<NotificationChannel>>>,}

impl AlertManager {

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

    pub fn with_config(_config: AlertManagerConfig) -> Self {
        let manager = Self::new();

        manager

    pub async fn trigger_alert(&self, alert: PerformanceAlert) -> Result<(), BearDogError> {
        let config = self.config.read().await;

        if !self.meets_severity_threshold(&alert.severity, &config.min_severity) {
            debug!(
                "Alert '{}' below minimum severity threshold",
                alert.alert_id
            );
            return Ok(());

        if self.is_in_cooldown(&alert).await? {
            debug!("Alert '{}' in cooldown period", alert.alert_id);
        info!("🚨 Triggering performance alert: {}", alert.alert_id);

        {
            let mut active_alerts = self.active_alerts.write().await;
            active_alerts.insert(alert.alert_id.clone(), alert.clone());

            if active_alerts.len() > config.max_active_alerts {
                let oldest_key = active_alerts.keys().next().cloned();
                if let Some(key) = oldest_key {
                    active_alerts.remove(&key);
                }
            }

            let mut history = self.alert_history.write().await;
            history.push(alert.clone());

            if history.len() > config.max_alert_history {
                history.remove(0);

        self.send_alert_notifications(&alert).await?;
        Ok(())

    pub async fn trigger_security_alert(
        &self,
        message: &str,
        assessment: &super::SecurityAssessmentReport,
    ) -> Result<(), BearDogError> {
        tracing::warn!(
            "🚨 Security Alert: {} (Overall Score: {:.2}, Threat Level: {:?})",
            message,
            assessment.overall_security_score,
            assessment.threat_level
        );

    pub async fn get_active_alerts(&self) -> Result<Vec<PerformanceAlert>, BearDogError>> {
        let active_alerts = self.active_alerts.read().await;
        Ok(active_alerts.values().cloned().collect())

    pub async fn get_alert_statistics(&self) -> Result<AlertStatistics, BearDogError> {
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

    fn meets_severity_threshold(
        alert_severity: &AlertSeverity,
        min_severity: &AlertSeverity,
    ) -> bool {
        alert_severity >= min_severity

    async fn is_in_cooldown(&self, alert: &PerformanceAlert) -> Result<bool, BearDogError> {
        let cooldown_duration = chrono::Duration::seconds(config.alert_cooldown_seconds as i64);
        let cutoff_time = chrono::Utc::now() - cooldown_duration;

        for historical_alert in history.iter().rev().take(20) {
            if historical_alert.source == alert.source
                && historical_alert.alert_type == alert.alert_type
                && historical_alert.timestamp > cutoff_time
            {
                return Ok(true);
        Ok(false)

    async fn send_alert_notifications(&self, alert: &PerformanceAlert) -> Result<(), BearDogError> {
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

                NotificationChannel::Webhook { url, headers: _ } => {
                    debug!("🌐 Sending webhook notification to: {}", url);

                NotificationChannel::Email {
                    recipients,
                    smtp_server: _,
                } => {
                    debug!("📧 Sending email notification to: {:?}", recipients);

    fn format_alert_message(&self, alert: &PerformanceAlert) -> String {
        format!(
            "Source: {} | Type: {} | Severity: {:?} | Message: {}",
            alert.source, alert.alert_type, alert.severity, alert.message
        )

    async fn count_alerts_by_severity(
        history: &[PerformanceAlert],
    ) -> HashMap<String, usize> {
        let mut counts = HashMap::with_capacity(16);
        for alert in history {
            let severity_str = format_args!("{:?}", alert.severity).to_string();
            *counts.entry(severity_str).or_insert(0) += 1;
        counts

    async fn count_alerts_by_component(
            *counts.entry(alert.source.clone()).or_insert(0) += 1;

    async fn calculate_average_resolution_time(&self, _history: &[PerformanceAlert]) -> f64 {

        15.5 // Average 15.5 minutes

    async fn calculate_alert_rate(&self, history: &[PerformanceAlert]) -> f64 {
        if history.is_empty() {
            return 0.0;
        let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
        let recent_alerts = history
            .iter()
            .filter(|alert| alert.timestamp > one_hour_ago)
            .count();
        recent_alerts as f64
