

use super::models::PrivacyAuditEvent;
use beardog_types::canonical::AuditEventType;
use super::types::{ComplianceStatus, PrivacyImpactLevel};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

pub struct PrivacyAuditSystem {

    audit_trail: Arc<RwLock<Vec<PrivacyAuditEvent>>>,

    compliance_config: ComplianceConfig,
}

#[derive(Debug, Clone)]
pub struct ComplianceConfig {
    pub gdpr_enabled: bool,
    pub ccpa_enabled: bool,
    pub hipaa_enabled: bool,
    pub retention_days: u64,
    pub auto_purge: bool,
    pub notification_threshold: u64,}

impl Default for ComplianceConfig {}

    fn default() -> Self {
        Self {
            gdpr_enabled: true,
            ccpa_enabled: true,
            hipaa_enabled: false,
            retention_days: 2555, // 7 years
            auto_purge: true,
            notification_threshold: 100,
        }
    }
impl Default for PrivacyAuditSystem {
        Self::new()}

impl PrivacyAuditSystem {

    pub fn new() -> Self {
        info!("📋 Initializing privacy audit and compliance system");
            audit_trail: Arc::new(RwLock::new(Vec::new())),
            compliance_config: ComplianceConfig::default(),

    pub fn with_config(config: ComplianceConfig) -> Self {
        info!("📋 Initializing privacy audit system with custom compliance config");
            compliance_config: config,

    pub async fn log_audit_event(
        &self,
        event_type: AuditEventType,
        user_action: Option<&str>,
        data_accessed: Option<&str>,
        privacy_impact: PrivacyImpactLevel,
        metadata: HashMap<&str, &str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let event_id = Uuid::new_v4().to_string();
        let event = PrivacyAuditEvent {
            event_id: event_id.clone(),
            event_type: event_type.clone(),
            timestamp: Utc::now(),
            user_action,
            data_accessed,
            privacy_impact: privacy_impact.clone(),
            compliance_status: self
                .determine_compliance_status(&event_type, &privacy_impact)
                .await,
            metadata,
        };
        debug!("📝 Logging privacy audit event: {:?}", event_type);

        {
            let mut trail = self.audit_trail.write().await;
            trail.push(event);

            if self.compliance_config.auto_purge {
                let cutoff = Utc::now()
                    - chrono::Duration::days(self.compliance_config.retention_days as i64);
                trail.retain(|e| e.timestamp > cutoff);
            }

        self.check_notification_threshold().await?;
        Ok(event_id)

    pub async fn get_audit_trail(
        event_type: Option<AuditEventType>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Vec<PrivacyAuditEvent> {
        let trail = self.audit_trail.read().await;
        let filtered: Vec<PrivacyAuditEvent> = trail
            .iter()
            .filter(|event| {

                if let Some(ref filter_type) = event_type {
                    if event.event_type != *filter_type {
                        return false;
                    }
                }

                if let Some(start) = start_date {
                    if event.timestamp < start {
                if let Some(end) = end_date {
                    if event.timestamp > end {
                true
            })
            .take(limit.unwrap_or(1000))
            .cloned()
            .collect();
        debug!("📊 Retrieved {} audit events", filtered.len());
        filtered

    pub async fn generate_compliance_report(
    ) -> Result<ComplianceReport, Box<dyn std::error::Error + Send + Sync>> {
        info!("📊 Generating privacy compliance report");
        let now = Utc::now();
        let last_30_days = now - chrono::Duration::days(30);

        let mut event_counts = HashMap::with_capacity(16);
        let mut violation_count = 0;
        let mut compliance_issues = Vec::new();
        for event in trail.iter() {
            if event.timestamp >= last_30_days {
                let count = event_counts.entry(event.event_type.clone()).or_insert(0);
                *count += 1;
                if event.event_type == AuditEventType::PrivacyViolation {
                    violation_count += 1;
                if event.compliance_status == ComplianceStatus::NonCompliant {
                    compliance_issues.push(event.clone());
        let report = ComplianceReport {
            report_id: Uuid::new_v4().to_string(),
            generated_at: now,
            period_start: last_30_days,
            period_end: now,
            total_events: event_counts.values().sum(),
            event_breakdown: event_counts,
            privacy_violations: violation_count,
            compliance_issues: compliance_issues.len(),
            gdpr_compliant: self.assess_gdpr_compliance(&trail).await,
            ccpa_compliant: self.assess_ccpa_compliance(&trail).await,
            recommendations: self.generate_recommendations(&compliance_issues).await,
        info!(
            "✅ Compliance report generated with {} total events",
            report.total_events
        );
        Ok(report)

    pub async fn export_audit_trail(
        format: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        info!("📤 Exporting audit trail in {} format", format);
        match format.to_lowercase().as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&*trail)?;
                Ok(json.into_bytes())
            "csv" => {
                let mut csv = String::from(
                    "timestamp,event_type,user_action,privacy_impact,compliance_status\n",
                );
                for event in trail.iter() {
                    csv.push_str(&format!(
                        "{},{:?},{},{:?},{:?}\n",
                        event.timestamp.to_rfc3339(),
                        event.event_type,
                        event.user_action.as_deref().unwrap_or(""),
                        event.privacy_impact,
                        event.compliance_status
                    ));
                Ok(csv.into_bytes())
            _ => Err("Unsupported export format".into()),

    async fn determine_compliance_status(
        event_type: &AuditEventType,
        privacy_impact: &PrivacyImpactLevel,
    ) -> ComplianceStatus {
        match (event_type, privacy_impact) {
            (AuditEventType::PrivacyViolation, _) => ComplianceStatus::NonCompliant,
            (_, PrivacyImpactLevel::Critical) => ComplianceStatus::UnderReview,
            (_, PrivacyImpactLevel::High) => ComplianceStatus::UnderReview,
            _ => ComplianceStatus::Compliant,

    async fn check_notification_threshold(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let recent_violations = trail
            .filter(|e| {
                e.timestamp > Utc::now() - chrono::Duration::hours(24)
                    && e.event_type == AuditEventType::PrivacyViolation
            .count();
        if recent_violations as u64 >= self.compliance_config.notification_threshold {
            warn!(
                "🚨 Privacy violation threshold exceeded: {} violations in 24h",
                recent_violations
            );

        Ok(())

    async fn assess_gdpr_compliance(&self, trail: &[PrivacyAuditEvent]) -> bool {
        debug!("🇪🇺 Assessing GDPR compliance for {} events", trail.len());

        for event in trail {

            if event.event_type == AuditEventType::DataAccess
                && !event.metadata.contains_key("consent_id")
            {
                compliance_issues.push("Data processing without explicit consent");

            let event_age = chrono::Utc::now()
                .signed_duration_since(event.timestamp)
                .num_days();
            if event_age > 365 && event.event_type == AuditEventType::DataAccess {
                compliance_issues.push("Data retained beyond reasonable period");

            if event.event_type == AuditEventType::DataSharing
                && !event.metadata.contains_key("adequacy_decision")
                compliance_issues.push("Cross-border transfer without adequate safeguards");
        let is_compliant = compliance_issues.is_empty();
        if is_compliant {
            debug!("✅ GDPR compliance assessment passed");
        } else {
            debug!("⚠️ GDPR compliance issues found: {:?}", compliance_issues);
        is_compliant

    async fn assess_ccpa_compliance(&self, trail: &[PrivacyAuditEvent]) -> bool {
        debug!("🇺🇸 Assessing CCPA compliance for {} events", trail.len());

                && !event.metadata.contains_key("collection_notice")
                compliance_issues.push("Personal information collection without proper notice");

                && !event.metadata.contains_key("opt_out_available")
                compliance_issues.push("Data sale without opt-out mechanism");

            if event.event_type == AuditEventType::DataPurge {
                let response_time = event
                    .metadata
                    .get("response_time_days")
                    .and_then(|v| v.parse::<i64>().ok())
                    .unwrap_or(0);
                if response_time > 45 {
                    compliance_issues.push("Deletion request not processed within 45 days");
            debug!("✅ CCPA compliance assessment passed");
            debug!("⚠️ CCPA compliance issues found: {:?}", compliance_issues);

    async fn generate_recommendations(&self, issues: &[PrivacyAuditEvent]) -> Vec<String> {
        let mut recommendations = Vec::new();
        if !issues.is_empty() {
            recommendations.push("Review and remediate compliance issues".to_string());
            recommendations.push("Enhance privacy protection mechanisms".to_string());
            recommendations.push("Conduct privacy impact assessment".to_string());
        recommendations

pub struct ComplianceReport {
    pub report_id: String,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_events: u64,
    pub event_breakdown: HashMap<AuditEventType, u64>,
    pub privacy_violations: u64,
    pub compliance_issues: usize,
    pub gdpr_compliant: bool,
    pub ccpa_compliant: bool,
    pub recommendations: Vec<String>,
