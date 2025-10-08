// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused import: tracing::debug
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {
    /// The model version value
    pub model_version: String,
    /// Confidence score of the AI response (0.0 to 1.0)
    pub confidence_score: f64,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstError {
    /// Type classification of the error
    /// The error type value
    pub error_type: String,
    /// Human-readable error message
    /// The message value
    pub message: String,
    /// Additional error details and context
    /// Mapping of details
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanInteractionContext {
    pub user_id: String,
    pub session_id: String,
    /// User preferences and configuration settings
    /// Mapping of preferences
    pub preferences: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    /// Type classification of the suggested action
    /// The action type value
    pub action_type: String,
    /// Human-readable description of the action
    /// The description value
    pub description: String,
    /// Priority level of the action (0-255, higher is more urgent)
    /// Number of priority
    pub priority: u8,
}

impl std::fmt::Display for SuggestedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} (priority: {})",
            self.action_type, self.description, self.priority
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {
    /// Whether success is enabled
    pub success: bool,
    /// The data value
    pub data: T,
    /// Optional error
    pub error: Option<AIFirstError>,
    pub request_id: Uuid,
    pub processing_time_ms: u64,
    /// The ai metadata value
    pub ai_metadata: AIResponseMetadata,
    /// Optional human context
    pub human_context: Option<HumanInteractionContext>,
    pub confidence_score: f64,
    /// Collection of suggested actions
    pub suggested_actions: Vec<SuggestedAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// The action value
    pub action: String,
    /// Whether automated is enabled
    pub automated: bool,
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageInfo {
    pub cpu_time_ms: u64,
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// Number of `network_requests`
    pub network_requests: u32,
    /// Number of `disk_io_operations`
    pub disk_io_operations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// The accuracy value
    pub accuracy: f64,
    /// The completeness value
    pub completeness: f64,
    /// The reliability value
    pub reliability: f64,
    /// The security value
    pub security: f64,
}

impl QualityMetrics {
    /// High Security operation.
    pub const fn high_security() -> Self {
        Self {
            accuracy: 0.98,
            completeness: 0.95,
            reliability: 0.99,
            security: 1.0,
        }
    }

    /// Standard operation.
    pub const fn standard() -> Self {
        Self {
            accuracy: 0.90,
            completeness: 0.85,
            reliability: 0.90,
            security: 0.85,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    /// Whether `cache_hit` is enabled
    pub cache_hit: bool,
    /// Optional cache key
    pub cache_key: Option<String>,
    /// Optional ttl seconds
    pub ttl_seconds: Option<u64>,
    /// The freshness value
    pub freshness: f64,
}

impl CacheInfo {
    /// No Cache operation.
    pub const fn no_cache() -> Self {
        Self {
            cache_hit: false,
            cache_key: None,
            ttl_seconds: None,
            freshness: 1.0, // Fresh data
        }
    }

    /// Cache Hit operation.
    pub fn cache_hit(key: &str, ttl: u64, freshness: f64) -> Self {
        Self {
            cache_hit: true,
            cache_key: Some(key.to_string()),
            ttl_seconds: Some(ttl),
            freshness,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Number of `remaining_requests`
    pub remaining_requests: u32,
    pub reset_time: DateTime<Utc>,
    /// Number of limit
    pub limit: u32,
    /// Whether limited is enabled
    pub limited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// The interaction style value
    pub interaction_style: InteractionStyle,
    /// Mapping of auto approval thresholds
    pub auto_approval_thresholds: HashMap<String, f64>,
    /// The notifications value
    pub notifications: NotificationPreferences,
    /// The security preferences value
    pub security_preferences: SecurityPreferences,
}

///
/// Defines how the AI system should interact with users,
/// from minimal automated responses to highly interactive experiences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionStyle {
    /// Minimal interaction with mostly automated responses
    Minimal,
    /// Balanced approach with selective user interaction
    Balanced,
    /// High level of user interaction and confirmation
    Interactive,
    /// Custom interaction style with configurable parameters
    Custom(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Whether email is enabled
    pub email_enabled: bool,
    /// Whether push is enabled
    pub push_enabled: bool,
    /// The urgency threshold value
    pub urgency_threshold: UrgencyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPreferences {
    /// Whether `require_2fa` is enabled
    pub require_2fa: bool,
    /// Whether biometric is enabled
    pub biometric_enabled: bool,
    /// The security notifications value
    pub security_notifications: SecurityNotificationLevel,
}

/// Level of security notifications to receive
///
/// Controls which security events trigger notifications,
/// allowing users to filter based on importance level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityNotificationLevel {
    /// Receive all security notifications
    All,
    /// Receive only important security notifications
    Important,
    /// Receive only critical security notifications
    Critical,
    /// Disable all security notifications
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    pub task_id: Option<String>,
    /// The task type value
    pub task_type: String,
    /// The priority value
    pub priority: TaskPriority,
    /// Optional deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Collection of related tasks
    pub related_tasks: Vec<String>,
}

///
/// Defines the urgency and importance of tasks within the system,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low priority task that can be deferred
    Low,
    Normal,
    /// High priority task requiring immediate attention
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub timestamp: DateTime<Utc>,
    /// The event type value
    pub event_type: InteractionEventType,
    /// The description value
    pub description: String,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of interaction event in the AI-first system
///
/// Categorizes different types of events that occur during
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of interaction event
pub enum InteractionEventType {
    /// User provided input to the system
    UserInput,
    /// System generated a response
    SystemResponse,
    ApprovalRequested,
    ApprovalGranted,
    ApprovalDenied,
    AutomatedAction,
}

///
/// Defines the degree of human oversight and supervision required
/// and criticality assessments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OversightLevel {
    /// Human oversight is optional - AI can operate fully autonomously
    Optional,
    /// Human oversight is recommended but not required
    Recommended,
    Required,
    /// Continuous human monitoring and supervision required
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// The risk level value
    pub risk_level: RiskLevel,
    /// Collection of risk factors
    pub risk_factors: Vec<String>,
    /// Collection of mitigation strategies
    pub mitigation_strategies: Vec<String>,
    /// The impact assessment value
    pub impact_assessment: ImpactAssessment,
}

// RiskLevel is not defined in canonical module - using SecurityLevel instead
pub use beardog_types::canonical::capabilities::SecurityLevel as RiskLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// The security impact value
    pub security_impact: ImpactLevel,
    pub performance_impact: ImpactLevel,
    /// The user experience impact value
    pub user_experience_impact: ImpactLevel,
    /// The system stability impact value
    pub system_stability_impact: ImpactLevel,
}

///
/// Categorizes the potential impact of operations, changes, or issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// Low impact with minimal effects on system operation
    Low,
    /// Medium impact with noticeable but manageable effects
    Medium,
    /// High impact with significant effects requiring immediate attention
    High,
}

///
/// various system tasks, issues, and operational requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    /// Low urgency - can be handled during normal business hours
    Low,
    /// Medium urgency - should be addressed within reasonable timeframes
    Medium,
    /// High urgency - requires immediate attention and resolution
    High,
}

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
    /// New operation.
    /// Creates a new instance
    pub fn new(data: T, request_id: Uuid) -> Self {
        Self {
            data,
            request_id,
            start_time: std::time::Instant::now(),
            error: None,
            confidence_score: 0.95, // Default high confidence
            human_context: None,
            suggested_actions: Vec::new(),
        }
    }

    /// With Error operation.
    /// Creates instance with error
    #[must_use]
    pub fn with_error(mut self, error: AIFirstError) -> Self {
        self.error = Some(error);
        self
    }

    /// With Confidence operation.
    /// Creates instance with confidence
    #[must_use]
    pub const fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence;
        self
    }

    /// With Human Context operation.
    /// Creates instance with human context
    #[must_use]
    pub fn with_human_context(mut self, context: HumanInteractionContext) -> Self {
        self.human_context = Some(context);
        self
    }

    /// With Suggested Action operation.
    /// Creates instance with suggested action
    #[must_use]
    pub fn with_suggested_action(mut self, action: SuggestedAction) -> Self {
        self.suggested_actions.push(action);
        self
    }

    /// Build operation.
    /// Builds component
    /// Builds component
    #[allow(clippy::cast_possible_truncation)] // Explicitly bounded to u64::MAX
    pub fn build(self) -> AIFirstResponse<T> {
        let processing_time_ms = self
            .start_time
            .elapsed()
            .as_millis()
            .min(u128::from(u64::MAX)) as u64;
        let success = self.error.is_none();
        tracing::debug!(
            "AI response built: success: {}, confidence: {:.2}, processing_time: {}ms",
            success,
            self.confidence_score,
            processing_time_ms
        );
        AIFirstResponse {
            success,
            data: self.data,
            error: self.error,
            request_id: self.request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata {
                model_version: "beardog-ai-v1.0".to_string(),
                confidence_score: if success { 0.95 } else { 0.1 },
                processing_time_ms,
            },
            human_context: self.human_context,
            confidence_score: self.confidence_score,
            suggested_actions: self.suggested_actions,
        }
    }
}

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
        assert!((response.confidence_score - 0.85).abs() < f64::EPSILON);
        assert!(response.processing_time_ms < 10000); // Reasonable upper bound
    }

    #[test]
    fn test_quality_metrics() {
        let high_security = QualityMetrics::high_security();
        assert!(high_security.security >= 0.9);
        assert!(high_security.reliability >= 0.9);
        let standard = QualityMetrics::standard();
        assert!(standard.accuracy >= 0.8);
    }

    #[test]
    fn test_cache_info() {
        let no_cache = CacheInfo::no_cache();
        assert!(!no_cache.cache_hit);
        assert!((no_cache.freshness - 1.0).abs() < f64::EPSILON);
        let cache_hit = CacheInfo::cache_hit("test_key", 300, 0.8);
        assert!(cache_hit.cache_hit);
        assert_eq!(cache_hit.cache_key, Some("test_key".to_string()));
    }
}
