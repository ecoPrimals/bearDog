

use beardog_errors::error_types::ErrorSeverity; // Use canonical ErrorSeverity
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;
use beardog_types::PerformanceMetrics; // Use unified PerformanceMetrics

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {

    pub success: bool,

    pub data: T,

    pub error: Option<AIFirstError>,

    pub request_id: Uuid,

    pub processing_time_ms: u64,

    pub ai_metadata: AIResponseMetadata,

    pub human_context: Option<HumanInteractionContext>,

    pub confidence_score: f64,

    pub suggested_actions: Vec<SuggestedAction>,
}

pub struct AIFirstError {

    pub code: String,

    pub message: String,

    pub details: HashMap<String, serde_json::Value>,

    pub remediation: Vec<RemediationAction>,

    pub severity: ErrorSeverity,

    pub retryable: bool,

pub struct RemediationAction {

    pub action: String,

    pub automated: bool,

    pub parameters: HashMap<String, serde_json::Value>,

pub struct AIResponseMetadata {

    pub performance: PerformanceMetrics,

    pub resource_usage: ResourceUsageInfo,

    pub quality_metrics: QualityMetrics,

    pub cache_info: CacheInfo,

    pub rate_limit_status: RateLimitStatus,

    pub dependencies: Vec<String>,

pub struct ResourceUsageInfo {

    pub cpu_usage_percent: f64,

    pub memory_usage_bytes: u64,

    pub network_bytes: u64,

    pub disk_io_operations: u32,

pub struct QualityMetrics {

    pub accuracy: f64,

    pub completeness: f64,

    pub reliability: f64,

    pub security: f64,}

impl QualityMetrics {

    pub fn high_security() -> Self {
        Self {
            accuracy: 0.98,
            completeness: 0.95,
            reliability: 0.99,
            security: 1.0,
        }
    }

    pub fn standard() -> Self {
            accuracy: 0.90,
            completeness: 0.85,
            reliability: 0.90,
            security: 0.85,

pub struct CacheInfo {

    pub cache_hit: bool,

    pub cache_key: Option<String>,

    pub ttl_seconds: Option<u64>,

    pub freshness: f64,}

impl CacheInfo {

    pub fn no_cache() -> Self {
            cache_hit: false,
            cache_key: None,
            ttl_seconds: None,
            freshness: 1.0, // Fresh data

    pub fn cache_hit(key: &str, ttl: u64, freshness: f64) -> Self {
            cache_hit: true,
            cache_key: Some(key),
            ttl_seconds: Some(ttl),
            freshness,

pub struct RateLimitStatus {

    pub requests_remaining: u32,

    pub reset_time: DateTime<Utc>,

    pub limit: u32,

    pub limited: bool,

pub struct HumanInteractionContext {

    pub preferences: UserPreferences,

    pub task_context: TaskContext,

    pub interaction_history: Vec<InteractionEvent>,

    pub oversight_level: OversightLevel,

pub struct UserPreferences {

    pub interaction_style: InteractionStyle,

    pub auto_approval_thresholds: HashMap<String, f64>,

    pub notifications: NotificationPreferences,

    pub security_preferences: SecurityPreferences,

pub enum InteractionStyle {

    Minimal,

    Balanced,

    Interactive,

    Custom(HashMap<String, serde_json::Value>),

pub struct NotificationPreferences {

    pub email_enabled: bool,

    pub push_enabled: bool,

    pub urgency_threshold: UrgencyLevel,

pub struct SecurityPreferences {

    pub require_2fa: bool,

    pub biometric_enabled: bool,

    pub security_notifications: SecurityNotificationLevel,

pub enum SecurityNotificationLevel {

    All,

    Important,

    Critical,

    None,

pub struct TaskContext {

    pub task_id: Option<String>,

    pub task_type: String,

    pub priority: TaskPriority,

    pub deadline: Option<DateTime<Utc>>,

    pub related_tasks: Vec<String>,

pub enum TaskPriority {

    Low,

    Normal,

    High,

pub struct InteractionEvent {

    pub timestamp: DateTime<Utc>,

    pub event_type: InteractionEventType,

    pub description: String,

    pub metadata: HashMap<String, serde_json::Value>,

pub enum InteractionEventType {

    UserInput,

    SystemResponse,

    ApprovalRequested,

    ApprovalGranted,

    ApprovalDenied,

    AutomatedAction,

pub enum OversightLevel {

    Optional,

    Recommended,

    Required,

    Continuous,

pub struct SuggestedAction {

    pub confidence: f64,

    pub human_approval_recommended: bool,

    pub expected_outcome: String,

    pub risk_assessment: RiskAssessment,

pub struct RiskAssessment {

    pub risk_level: RiskLevel,

    pub risk_factors: Vec<String>,

    pub mitigation_strategies: Vec<String>,

    pub impact_assessment: ImpactAssessment,

pub use beardog_types::canonical::RiskLevel;

pub struct ImpactAssessment {

    pub security_impact: ImpactLevel,

    pub performance_impact: ImpactLevel,

    pub user_experience_impact: ImpactLevel,

    pub system_stability_impact: ImpactLevel,

pub enum ImpactLevel {

    Medium,

pub enum UrgencyLevel {

pub struct AIFirstResponseBuilder<T> {
    data: T,
    request_id: Uuid,
    start_time: std::time::Instant,
    error: Option<AIFirstError>,
    confidence_score: f64,
    human_context: Option<HumanInteractionContext>,
    suggested_actions: Vec<SuggestedAction>,
}

impl<T> AIFirstResponseBuilder<T> {

    pub fn new(data: T, request_id: Uuid) -> Self {
            data,
            request_id,
            start_time: std::time::Instant::now(),
            error: None,
            confidence_score: 0.95, // Default high confidence
            human_context: None,
            suggested_actions: Vec::new(),

    pub fn with_error(mut self, error: AIFirstError) -> Self {
        self.error = Some(error);
        self

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence;

    pub fn with_human_context(mut self, context: HumanInteractionContext) -> Self {
        self.human_context = Some(context);

    pub fn with_suggested_action(mut self, action: SuggestedAction) -> Self {
        self.suggested_actions.push(action);

    pub fn build(self) -> AIFirstResponse<T> {
        let processing_time_ms = self.start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;
        let success = self.error.is_none();
        debug!(
            "Building AI-First response - success: {}, confidence: {:.2}, processing_time: {}ms",
            success, self.confidence_score, processing_time_ms
        );
        AIFirstResponse {
            success,
            data: self.data,
            error: self.error,
            request_id: self.request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata {
                performance: PerformanceMetrics {
                    db_query_time_ms: None,
                    cache_lookup_time_ms: None,
                    external_api_time_ms: None,
                    crypto_operation_time_ms: None,
                    total_processing_time_ms: processing_time_ms as f64,
                },
                resource_usage: ResourceUsageInfo {
                    cpu_usage_percent: 10.0,         // Default estimate
                    memory_usage_bytes: 1024 * 1024, // 1MB default
                    network_bytes: 0,
                    disk_io_operations: 0,
                quality_metrics: if success {
                    QualityMetrics::high_security()
                } else {
                    QualityMetrics::standard()
                cache_info: CacheInfo::no_cache(),
                rate_limit_status: RateLimitStatus {
                    requests_remaining: 1000,
                    reset_time: Utc::now() + chrono::Duration::hours(1),
                    limit: 1000,
                    limited: false,
                dependencies: vec!["beardog-core".to_string()],
            },
            human_context: self.human_context,
            confidence_score: self.confidence_score,
            suggested_actions: self.suggested_actions,
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ai_first_response_builder() {
        let request_id = Uuid::new_v4();
        let response = AIFirstResponseBuilder::new("test_data", request_id)
            .with_confidence(0.85)
            .build();
        assert!(response.success);
        assert_eq!(response.data, "test_data");
        assert_eq!(response.request_id, request_id);
        assert_eq!(response.confidence_score, 0.85);

        assert!(response.processing_time_ms < 10000); // Reasonable upper bound}

    fn test_quality_metrics() {
        let high_security = QualityMetrics::high_security();
        assert!(high_security.security >= 0.9);
        assert!(high_security.reliability >= 0.9);
        let standard = QualityMetrics::standard();
        assert!(standard.accuracy >= 0.8);
    fn test_cache_info() {
        let no_cache = CacheInfo::no_cache();
        assert!(!no_cache.cache_hit);
        assert_eq!(no_cache.freshness, 1.0);
        let cache_hit = CacheInfo::cache_hit("test_key".to_string(), 300, 0.8);
        assert!(cache_hit.cache_hit);
        assert_eq!(cache_hit.cache_key, Some("test_key".to_string()));
