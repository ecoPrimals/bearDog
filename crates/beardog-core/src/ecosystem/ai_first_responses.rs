// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused import: tracing::debug
use uuid::Uuid;

/// AI response metadata
///
/// Metadata about an AI-generated response including model version,
/// confidence scores, and processing time information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {
    /// Version of the AI model that generated this response
    pub model_version: String,
    /// Confidence score of the AI response (0.0 to 1.0)
    pub confidence_score: f64,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// AI-first error information
///
/// Structured error information from AI processing operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstError {
    /// Type classification of the error
    pub error_type: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details and context
    pub details: HashMap<String, String>,
}

/// Human interaction context
///
/// Context information about the human user interacting with the AI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanInteractionContext {
    /// Unique identifier for the user
    pub user_id: String,
    /// Session identifier for this interaction
    pub session_id: String,
    /// User preferences and configuration settings
    pub preferences: HashMap<String, String>,
}

/// Suggested action for the user
///
/// An action suggested by the AI system for the user to consider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    /// Type classification of the suggested action
    pub action_type: String,
    /// Human-readable description of the action
    pub description: String,
    /// Priority level of the action (0-255, higher is more urgent)
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

/// AI-first response wrapper
///
/// Wrapper for AI-generated responses with metadata, error handling,
/// and suggested actions. Generic over the response data type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstResponse<T> {
    /// Whether the operation succeeded
    pub success: bool,
    /// Response data payload
    pub data: T,
    /// Error information if operation failed
    pub error: Option<AIFirstError>,
    /// Unique identifier for this request
    pub request_id: Uuid,
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
    /// AI model metadata for this response
    pub ai_metadata: AIResponseMetadata,
    /// Optional context about the human user
    pub human_context: Option<HumanInteractionContext>,
    /// Overall confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Actions suggested by the AI for the user
    pub suggested_actions: Vec<SuggestedAction>,
}

/// Remediation action
///
/// Action to remediate an issue or error condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Description of the remediation action
    pub action: String,
    /// Whether this action can be automated
    pub automated: bool,
    /// Parameters for executing the action
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Resource usage information
///
/// Tracks resource consumption for an AI operation including
/// CPU, memory, network, and disk I/O usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageInfo {
    /// CPU time consumed in milliseconds
    pub cpu_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Number of network requests made
    pub network_requests: u32,
    /// Number of disk I/O operations performed
    pub disk_io_operations: u32,
}

/// Quality metrics
///
/// Metrics measuring various quality aspects of an AI response
/// including accuracy, completeness, reliability, and security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Accuracy score (0.0 to 1.0)
    pub accuracy: f64,
    /// Completeness score (0.0 to 1.0)
    pub completeness: f64,
    /// Reliability score (0.0 to 1.0)
    pub reliability: f64,
    /// Security score (0.0 to 1.0)
    pub security: f64,
}

impl QualityMetrics {
    /// High Security operation.
    #[must_use]
    pub const fn high_security() -> Self {
        Self {
            accuracy: 0.98,
            completeness: 0.95,
            reliability: 0.99,
            security: 1.0,
        }
    }

    /// Standard operation.
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            accuracy: 0.90,
            completeness: 0.85,
            reliability: 0.90,
            security: 0.85,
        }
    }
}

/// Cache metadata for AI response caching
///
/// Tracks whether a response came from cache, its freshness,
/// and time-to-live information for cache management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    /// Whether this response was served from cache
    pub cache_hit: bool,
    /// Cache key used to store/retrieve this response
    pub cache_key: Option<String>,
    /// Time-to-live for cached response in seconds
    pub ttl_seconds: Option<u64>,
    /// Freshness score (0.0-1.0, where 1.0 is completely fresh)
    pub freshness: f64,
}

impl CacheInfo {
    /// No Cache operation.
    #[must_use]
    pub const fn no_cache() -> Self {
        Self {
            cache_hit: false,
            cache_key: None,
            ttl_seconds: None,
            freshness: 1.0, // Fresh data
        }
    }

    /// Cache Hit operation.
    #[must_use]
    pub fn cache_hit(key: &str, ttl: u64, freshness: f64) -> Self {
        Self {
            cache_hit: true,
            cache_key: Some(key.to_string()),
            ttl_seconds: Some(ttl),
            freshness,
        }
    }
}

/// Rate limiting information for AI requests
///
/// Tracks rate limit status to prevent abuse and ensure fair
/// resource allocation across users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Number of requests remaining in current window
    pub remaining_requests: u32,
    /// Timestamp when the rate limit window resets
    pub reset_time: DateTime<Utc>,
    /// Total request limit per window
    pub limit: u32,
    /// Whether the requester is currently rate-limited
    pub limited: bool,
}

/// User preferences for AI interaction and behavior
///
/// Configures how the AI system interacts with this user,
/// including interaction style, notifications, and security settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred interaction style with the AI
    pub interaction_style: InteractionStyle,
    /// Thresholds for automatic approval of actions by category
    pub auto_approval_thresholds: HashMap<String, f64>,
    /// Notification delivery preferences
    pub notifications: NotificationPreferences,
    /// Security and authentication preferences
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

/// Notification delivery preferences
///
/// Controls which notification channels are enabled and the minimum
/// urgency level required to trigger notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Whether email notifications are enabled
    pub email_enabled: bool,
    /// Whether push notifications are enabled
    pub push_enabled: bool,
    /// Minimum urgency level for notifications
    pub urgency_threshold: UrgencyLevel,
}

/// Security and authentication preferences
///
/// Configures security features including two-factor authentication,
/// biometric verification, and security notification levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPreferences {
    /// Whether two-factor authentication is required
    pub require_2fa: bool,
    /// Whether biometric authentication is enabled
    pub biometric_enabled: bool,
    /// Level of security notifications to receive
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

/// Context and metadata for a task or operation
///
/// Provides task identification, priority, deadlines, and relationships
/// to other tasks for workflow management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    /// Unique identifier for this task
    pub task_id: Option<String>,
    /// Type or category of task
    pub task_type: String,
    /// Priority level for task execution
    pub priority: TaskPriority,
    /// Optional deadline for task completion
    pub deadline: Option<DateTime<Utc>>,
    /// Related or dependent tasks
    pub related_tasks: Vec<String>,
}

///
/// Defines the urgency and importance of tasks within the system,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low priority task that can be deferred
    Low,
    /// Normal priority task with standard processing
    Normal,
    /// High priority task requiring immediate attention
    High,
}

/// Record of an interaction event between user and AI
///
/// Captures significant events in the AI-human interaction flow,
/// including user inputs, system responses, and approval workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// Type of interaction event
    pub event_type: InteractionEventType,
    /// Human-readable description of the event
    pub description: String,
    /// Additional contextual metadata about the event
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
    /// System requested human approval for an action
    ApprovalRequested,
    /// Human granted approval for the requested action
    ApprovalGranted,
    /// Human denied approval for the requested action
    ApprovalDenied,
    /// AI performed an automated action without human intervention
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
    /// Human oversight is required for this operation
    Required,
    /// Continuous human monitoring and supervision required
    Continuous,
}

/// Risk assessment for operations and decisions
///
/// Evaluates potential risks, their factors, mitigation strategies,
/// and comprehensive impact analysis across multiple dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level classification
    pub risk_level: RiskLevel,
    /// Identified risk factors contributing to the assessment
    pub risk_factors: Vec<String>,
    /// Recommended mitigation strategies to reduce risk
    pub mitigation_strategies: Vec<String>,
    /// Detailed impact analysis across system dimensions
    pub impact_assessment: ImpactAssessment,
}

// RiskLevel is not defined in canonical module - using SecurityLevel instead
pub use beardog_types::canonical::capabilities::SecurityLevel as RiskLevel;

/// Multi-dimensional impact assessment
///
/// Evaluates the potential impact of operations or changes across
/// security, performance, user experience, and system stability dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Impact on system security posture
    pub security_impact: ImpactLevel,
    /// Impact on system and operation performance
    pub performance_impact: ImpactLevel,
    /// Impact on user experience and satisfaction
    pub user_experience_impact: ImpactLevel,
    /// Impact on overall system stability and reliability
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

/// Builder for constructing AI-first responses with validation
///
/// Provides a fluent interface for building `AIFirstResponse` instances
/// with proper validation, error handling, and context management.
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Millis duration clamped to u64::MAX before cast"
    )]
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

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_first_response_builder() {
        let request_id = Uuid::new_v4();
        let response = AIFirstResponseBuilder::new("test_data", request_id)
            .with_confidence(0.85)
            .build();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(response.success);
        assert_eq!(response.data, "test_data");
        assert_eq!(response.request_id, request_id);
        assert!((response.confidence_score - 0.85).abs() < f64::EPSILON);
        assert!(response.processing_time_ms < 10000); // Reasonable upper bound
    }

    #[test]
    fn test_quality_metrics() {
        let high_security = QualityMetrics::high_security();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(high_security.security >= 0.9);
        assert!(high_security.reliability >= 0.9);
        let standard = QualityMetrics::standard();
        assert!(standard.accuracy >= 0.8);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
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
