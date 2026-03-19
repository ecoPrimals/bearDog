// SPDX-License-Identifier: AGPL-3.0-only

//! # Incident Response and Security Events
//!
//! This module provides types for incident response management, security events,
//! and automated response workflows using canonical systems.

use super::core::{ThreatAction, ThreatSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Security event for comprehensive logging and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event identifier
    pub event_id: String,
    /// Event type
    pub event_type: SecurityEventType,
    /// Event severity
    pub severity: ThreatSeverity,
    /// Event source system
    pub source_system: String,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event description
    pub description: String,
    /// Related threat event (if any)
    pub related_threat_id: Option<String>,
    /// User involved (if applicable)
    pub user: Option<String>,
    /// Source IP address
    pub source_ip: Option<String>,
    /// Destination IP address
    pub destination_ip: Option<String>,
    /// Additional event data
    pub event_data: HashMap<String, String>,
    /// Event tags for categorization
    pub tags: Vec<String>,
}

impl SecurityEvent {
    /// Create a new security event
    #[must_use]
    pub fn new(
        event_type: SecurityEventType,
        severity: ThreatSeverity,
        source_system: String,
        description: String,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            severity,
            source_system,
            timestamp: SystemTime::now(),
            description,
            related_threat_id: None,
            user: None,
            source_ip: None,
            destination_ip: None,
            event_data: HashMap::new(),
            tags: Vec::new(),
        }
    }

    #[must_use]
    /// Add event data key-value pair
    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.event_data.insert(key, value);
        self
    }
 #[must_use]

    /// Add a tag to the event
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

/// Types of security events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEventType {
    /// Authentication event
    Authentication,
    /// Authorization event
    Authorization,
    /// Data access event
    DataAccess,
    /// System access event
    SystemAccess,
    /// Configuration change
    ConfigurationChange,
    /// Policy violation
    PolicyViolation,
    /// Malware detection
    MalwareDetection,
    /// Network intrusion
    NetworkIntrusion,
    /// Data exfiltration
    DataExfiltration,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Audit event
    Audit,
    /// System event
    System,
}

impl std::fmt::Display for SecurityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authentication => write!(f, "Authentication"),
            Self::Authorization => write!(f, "Authorization"),
            Self::DataAccess => write!(f, "DataAccess"),
            Self::SystemAccess => write!(f, "SystemAccess"),
            Self::ConfigurationChange => write!(f, "ConfigurationChange"),
            Self::PolicyViolation => write!(f, "PolicyViolation"),
            Self::MalwareDetection => write!(f, "MalwareDetection"),
            Self::NetworkIntrusion => write!(f, "NetworkIntrusion"),
            Self::DataExfiltration => write!(f, "DataExfiltration"),
            Self::PrivilegeEscalation => write!(f, "PrivilegeEscalation"),
            Self::Audit => write!(f, "Audit"),
            Self::System => write!(f, "System"),
        }
    }
}

impl SecurityEventType {
    /// Convert to string representation (compatibility)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Authentication => "Authentication",
            Self::Authorization => "Authorization",
            Self::DataAccess => "DataAccess",
            Self::SystemAccess => "SystemAccess",
            Self::ConfigurationChange => "ConfigurationChange",
            Self::PolicyViolation => "PolicyViolation",
            Self::MalwareDetection => "MalwareDetection",
            Self::NetworkIntrusion => "NetworkIntrusion",
            Self::DataExfiltration => "DataExfiltration",
            Self::PrivilegeEscalation => "PrivilegeEscalation",
            Self::Audit => "Audit",
            Self::System => "System",
        }
    }
}

/// Incident response plan and execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    /// Response identifier
    pub response_id: String,
    /// Related threat event identifier
    pub threat_event_id: String,
    /// Response plan identifier
    pub response_plan_id: String,
    /// Current response status
    pub status: ResponseStatus,
    /// Response priority level
    pub priority: ResponsePriority,
    /// Assigned response team
    pub assigned_team: String,
    /// Lead responder
    pub lead_responder: String,
    /// Response actions to execute
    pub actions: Vec<ResponseAction>,
    /// Response timeline
    pub timeline: Vec<ResponseTimelineEntry>,
    /// Response start time
    pub started_at: SystemTime,
    /// Response completion time
    pub completed_at: Option<SystemTime>,
    /// Response notes and updates
    pub notes: Vec<ResponseNote>,
    /// Response metrics
    pub metrics: ResponseMetrics,
}

/// Status of incident response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    /// Response initiated
    Initiated,
    /// Response in progress
    InProgress,
    /// Response escalated
    Escalated,
    /// Response on hold
    OnHold,
    /// Response completed successfully
    Completed,
    /// Response failed
    Failed,
    /// Response cancelled
    Cancelled,
}

/// Priority levels for incident response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResponsePriority {
    /// Low priority response
    Low,
    /// Medium priority response
    Medium,
    /// High priority response
    High,
    /// Critical priority response
    Critical,
    /// Emergency response
    Emergency,
}

/// Individual response action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    /// Action identifier
    pub action_id: String,
    /// Action type
    pub action_type: ThreatAction,
    /// Action description
    pub description: String,
    /// Action status
    pub status: ActionStatus,
    /// Assigned executor
    pub assigned_to: String,
    /// Action priority
    pub priority: u8,
    /// Estimated duration in minutes
    pub estimated_duration_minutes: u32,
    /// Actual duration in minutes
    pub actual_duration_minutes: Option<u32>,
    /// Action start time
    pub started_at: Option<SystemTime>,
    /// Action completion time
    pub completed_at: Option<SystemTime>,
    /// Action result
    pub result: Option<ActionResult>,
    /// Action notes
    pub notes: String,
    /// Required approvals
    pub required_approvals: Vec<String>,
    /// Received approvals
    pub received_approvals: Vec<Approval>,
}

/// Status of response actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionStatus {
    /// Action pending execution
    Pending,
    /// Action awaiting approval
    AwaitingApproval,
    /// Action approved and ready
    Approved,
    /// Action in progress
    InProgress,
    /// Action completed successfully
    Completed,
    /// Action failed
    Failed,
    /// Action cancelled
    Cancelled,
    /// Action skipped
    Skipped,
}

/// Result of a response action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Whether the action was successful
    pub success: bool,
    /// Result message
    pub message: String,
    /// Result data
    pub data: HashMap<String, String>,
    /// Impact assessment
    pub impact: ActionImpact,
}

/// Impact assessment of response actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionImpact {
    /// Systems affected by the action
    pub affected_systems: Vec<String>,
    /// Users affected by the action
    pub affected_users: Vec<String>,
    /// Services affected by the action
    pub affected_services: Vec<String>,
    /// Business impact level
    pub business_impact: BusinessImpactLevel,
    /// Expected downtime in minutes
    pub expected_downtime_minutes: u32,
    /// Actual downtime in minutes
    pub actual_downtime_minutes: Option<u32>,
}

/// Business impact levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BusinessImpactLevel {
    /// No business impact
    None,
    /// Low business impact
    Low,
    /// Medium business impact
    Medium,
    /// High business impact
    High,
    /// Critical business impact
    Critical,
}

/// Approval for response actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// Approver identifier
    pub approver: String,
    /// Approval timestamp
    pub approved_at: SystemTime,
    /// Approval decision
    pub decision: ApprovalDecision,
    /// Approval comments
    pub comments: String,
}

/// Approval decisions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalDecision {
    /// Action approved
    Approved,
    /// Action rejected
    Rejected,
    /// Action approved with conditions
    ApprovedWithConditions,
}

/// Timeline entry for response tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimelineEntry {
    /// Timeline entry identifier
    pub entry_id: String,
    /// Entry timestamp
    pub timestamp: SystemTime,
    /// Entry type
    pub entry_type: TimelineEntryType,
    /// Entry description
    pub description: String,
    /// Person responsible for the entry
    pub person: String,
    /// Additional entry data
    pub data: HashMap<String, String>,
}

/// Types of timeline entries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimelineEntryType {
    /// Response initiated
    ResponseInitiated,
    /// Action started
    ActionStarted,
    /// Action completed
    ActionCompleted,
    /// Escalation occurred
    Escalation,
    /// Status update
    StatusUpdate,
    /// Note added
    NoteAdded,
    /// Approval requested
    ApprovalRequested,
    /// Approval received
    ApprovalReceived,
    /// Response completed
    ResponseCompleted,
}

/// Response notes and updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseNote {
    /// Note identifier
    pub note_id: String,
    /// Note timestamp
    pub timestamp: SystemTime,
    /// Note author
    pub author: String,
    /// Note content
    pub content: String,
    /// Note visibility
    pub visibility: NoteVisibility,
    /// Note tags
    pub tags: Vec<String>,
}

/// Visibility levels for response notes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoteVisibility {
    /// Visible to response team only
    Team,
    /// Visible to organization
    Organization,
    /// Visible to management
    Management,
    /// Public visibility
    Public,
}

/// Response performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetrics {
    /// Time to first response in minutes
    pub time_to_first_response_minutes: u32,
    /// Time to containment in minutes
    pub time_to_containment_minutes: Option<u32>,
    /// Time to resolution in minutes
    pub time_to_resolution_minutes: Option<u32>,
    /// Number of actions executed
    pub actions_executed: u32,
    /// Number of failed actions
    pub actions_failed: u32,
    /// Number of escalations
    pub escalations_count: u32,
    /// Total response cost
    pub total_cost: Option<f64>,
    /// Response effectiveness score (0.0 - 1.0)
    pub effectiveness_score: Option<f64>,
}

/// Automated response rule for threat handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedResponseRule {
    /// Rule identifier
    pub rule_id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule conditions
    pub conditions: Vec<RuleCondition>,
    /// Actions to execute when conditions are met
    pub actions: Vec<ThreatAction>,
    /// Rule priority
    pub priority: u8,
    /// Whether rule is enabled
    pub is_enabled: bool,
    /// Rule author
    pub author: String,
    /// Rule creation timestamp
    pub created_at: SystemTime,
    /// Rule last modified timestamp
    pub last_modified: SystemTime,
    /// Rule execution count
    pub execution_count: u32,
    /// Rule success rate
    pub success_rate: f64,
}

/// Conditions for automated response rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Threat severity condition
    ThreatSeverity {
        /// Minimum severity level
        min_severity: ThreatSeverity,
    },
    /// Threat confidence condition
    ThreatConfidence {
        /// Minimum confidence score
        min_confidence: f64,
    },
    /// Threat type condition
    ThreatType {
        /// Allowed threat types
        allowed_types: Vec<String>,
    },
    /// Source reputation condition
    SourceReputation {
        /// Maximum reputation score
        max_reputation: u8,
    },
    /// Time-based condition
    TimeWindow {
        /// Start hour (0-23)
        start_hour: u8,
        /// End hour (0-23)
        end_hour: u8,
    },
    /// Asset criticality condition
    AssetCriticality {
        /// Minimum asset criticality
        min_criticality: String,
    },
    /// Field greater than condition (compatibility)
    FieldGreaterThan {
        /// Field name
        field: String,
        /// Threshold value
        value: f64,
    },
} 