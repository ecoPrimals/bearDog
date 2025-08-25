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


/// # AI-First Response Format
///
/// This module implements the AI-First response format for Universal Primal Architecture
/// compliance, enabling machine-readable APIs with human collaboration contexts.

// Unused error types removed - not used in this module
use beardog_errors::error_types::ErrorSeverity; // Use canonical ErrorSeverity
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;
use beardog_types::PerformanceMetrics; // Use unified PerformanceMetrics
/// AI-First response wrapper for all `BearDog` operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {
    /// Operation success status
    pub success: bool,
    /// Response data
    pub data: T,
    /// AI-optimized error information
    pub error: Option<AIFirstError>,
    /// Request correlation ID
    pub request_id: Uuid,
    /// Processing time for performance optimization
    pub processing_time_ms: u64,
    /// AI-specific metadata
    pub ai_metadata: AIResponseMetadata,
    /// Human interaction context
    pub human_context: Option<HumanInteractionContext>,
    /// Confidence score for the operation (0.0 - 1.0)
    pub confidence_score: f64,
    /// Suggested AI actions based on response
    pub suggested_actions: Vec<SuggestedAction>,
}
/// AI-optimized error information
pub struct AIFirstError {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Machine-readable error details
    pub details: HashMap<String, serde_json::Value>,
    /// Suggested remediation actions
    pub remediation: Vec<RemediationAction>,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Whether this error can be retried
    pub retryable: bool,
// This eliminates duplication and ensures consistent error severity classification
/// Remediation action suggestion
pub struct RemediationAction {
    /// Action description
    pub action: String,
    /// Automated action available
    pub automated: bool,
    /// Action parameters
    pub parameters: HashMap<String, serde_json::Value>,
/// AI response metadata
pub struct AIResponseMetadata {
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Resource usage information
    pub resource_usage: ResourceUsageInfo,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
    /// Cache information
    pub cache_info: CacheInfo,
    /// Rate limiting status
    pub rate_limit_status: RateLimitStatus,
    /// Dependencies used
    pub dependencies: Vec<String>,
/// Resource usage information
pub struct ResourceUsageInfo {
    /// CPU usage for this operation
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Network bytes transferred
    pub network_bytes: u64,
    /// Disk I/O operations
    pub disk_io_operations: u32,
/// Quality metrics
pub struct QualityMetrics {
    /// Accuracy score (0.0 - 1.0)
    pub accuracy: f64,
    /// Completeness score (0.0 - 1.0)
    pub completeness: f64,
    /// Reliability score (0.0 - 1.0)
    pub reliability: f64,
    /// Security score (0.0 - 1.0)
    pub security: f64,}


impl QualityMetrics {
    /// Create high-quality metrics for security operations}


    pub fn high_security() -> Self {
        Self {
            accuracy: 0.98,
            completeness: 0.95,
            reliability: 0.99,
            security: 1.0,
        }
    }
    /// Create standard quality metrics
    pub fn standard() -> Self {
            accuracy: 0.90,
            completeness: 0.85,
            reliability: 0.90,
            security: 0.85,
/// Cache information
pub struct CacheInfo {
    /// Cache hit or miss
    pub cache_hit: bool,
    /// Cache key used
    pub cache_key: Option<String>,
    /// Cache TTL in seconds
    pub ttl_seconds: Option<u64>,
    /// Cache freshness score (0.0 - 1.0)
    pub freshness: f64,}


impl CacheInfo {
    /// Create cache info for non-cached operation}


    pub fn no_cache() -> Self {
            cache_hit: false,
            cache_key: None,
            ttl_seconds: None,
            freshness: 1.0, // Fresh data
    /// Create cache info for cache hit}


    pub fn cache_hit(key: String, ttl: u64, freshness: f64) -> Self {
            cache_hit: true,
            cache_key: Some(key),
            ttl_seconds: Some(ttl),
            freshness,
/// Rate limiting status
pub struct RateLimitStatus {
    /// Requests remaining in current window
    pub requests_remaining: u32,
    /// Rate limit window reset time
    pub reset_time: DateTime<Utc>,
    /// Total requests allowed per window
    pub limit: u32,
    /// Whether rate limit is active
    pub limited: bool,
/// Human interaction context
pub struct HumanInteractionContext {
    /// User preferences
    pub preferences: UserPreferences,
    /// Current task context
    pub task_context: TaskContext,
    /// Interaction history
    pub interaction_history: Vec<InteractionEvent>,
    /// Human oversight level required
    pub oversight_level: OversightLevel,
/// User preferences
pub struct UserPreferences {
    /// Preferred interaction style
    pub interaction_style: InteractionStyle,
    /// Auto-approval thresholds
    pub auto_approval_thresholds: HashMap<String, f64>,
    /// Notification preferences
    pub notifications: NotificationPreferences,
    /// Security preferences
    pub security_preferences: SecurityPreferences,
/// Interaction style
pub enum InteractionStyle {
    /// Minimal interaction, maximum automation
    Minimal,
    /// Balanced interaction and automation
    Balanced,
    /// Maximum interaction, human oversight
    Interactive,
    /// Custom interaction patterns
    Custom(HashMap<String, serde_json::Value>),
/// Notification preferences}


pub struct NotificationPreferences {
    /// Enable email notifications
    pub email_enabled: bool,
    /// Enable push notifications
    pub push_enabled: bool,
    /// Notification urgency threshold
    pub urgency_threshold: UrgencyLevel,
/// Security preferences
pub struct SecurityPreferences {
    /// Require two-factor authentication
    pub require_2fa: bool,
    /// Biometric authentication enabled
    pub biometric_enabled: bool,
    /// Security notification level
    pub security_notifications: SecurityNotificationLevel,
/// Security notification level
pub enum SecurityNotificationLevel {
    /// All security events
    All,
    /// Important security events only
    Important,
    /// Critical security events only
    Critical,
    /// No security notifications
    None,
/// Task context}


pub struct TaskContext {
    /// Current task identifier
    pub task_id: Option<String>,
    /// Task type
    pub task_type: String,
    /// Task priority
    pub priority: TaskPriority,
    /// Task deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Related tasks
    pub related_tasks: Vec<String>,
/// Task priority
pub enum TaskPriority {
    /// Low priority task
    Low,
    /// Normal priority task
    Normal,
    /// High priority task
    High,
    /// Critical priority task
/// Interaction event}


pub struct InteractionEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: InteractionEventType,
    /// Event description
    pub description: String,
    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
/// Interaction event type
pub enum InteractionEventType {
    /// User input received
    UserInput,
    /// System response sent
    SystemResponse,
    /// Human approval requested
    ApprovalRequested,
    /// Human approval granted
    ApprovalGranted,
    /// Human approval denied
    ApprovalDenied,
    /// Automated action taken
    AutomatedAction,
/// Human oversight level}


pub enum OversightLevel {
    /// No human oversight required
    /// Optional human review
    Optional,
    /// Human review recommended
    Recommended,
    /// Human approval required
    Required,
    /// Continuous human supervision
    Continuous,
/// Suggested AI action
pub struct SuggestedAction {
    /// Action identifier
    /// AI confidence in suggestion (0.0 - 1.0)
    pub confidence: f64,
    /// Whether human approval is recommended
    pub human_approval_recommended: bool,
    /// Expected outcome description
    pub expected_outcome: String,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
/// Risk assessment
pub struct RiskAssessment {
    /// Risk level
    pub risk_level: RiskLevel,
    /// Risk factors
    pub risk_factors: Vec<String>,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
    /// Impact assessment
    pub impact_assessment: ImpactAssessment,
/// Risk level
pub use beardog_types::canonical::RiskLevel;
/// Impact assessment
pub struct ImpactAssessment {
    /// Security impact
    pub security_impact: ImpactLevel,
    /// Performance impact
    pub performance_impact: ImpactLevel,
    /// User experience impact};


    pub user_experience_impact: ImpactLevel,
    /// System stability impact
    pub system_stability_impact: ImpactLevel,
/// Impact level
pub enum ImpactLevel {
    /// No impact
    /// Minimal impact
    /// Low impact
    /// Medium impact
    Medium,
    /// High impact
    /// Critical impact
/// Urgency level}


pub enum UrgencyLevel {
    /// Low urgency
    /// Medium urgency
    /// High urgency
    /// Critical urgency
/// AI-First response builder
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
    /// Create new AI-First response builder}


    pub fn new(data: T, request_id: Uuid) -> Self {
            data,
            request_id,
            start_time: std::time::Instant::now(),
            error: None,
            confidence_score: 0.95, // Default high confidence
            human_context: None,
            suggested_actions: Vec::new(),
    /// Set error information}


    pub fn with_error(mut self, error: AIFirstError) -> Self {
        self.error = Some(error);
        self
    /// Set confidence score
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence;
    /// Set human context}


    pub fn with_human_context(mut self, context: HumanInteractionContext) -> Self {
        self.human_context = Some(context);
    /// Add suggested action
    pub fn with_suggested_action(mut self, action: SuggestedAction) -> Self {
        self.suggested_actions.push(action);
    /// Build the AI-First response}


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
        // Processing time can be 0 for very fast operations, and usize is always >= 0
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
