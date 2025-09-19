// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::authorization::ResourcePermission;
use super::genetics::NodeCapability;
use super::spawning::SpawnPurpose;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    pub workflow_id: String,
    /// The workflow type value
    pub workflow_type: BearDogWorkflowType,
    pub requester_node_id: String,
    /// Collection of target nodes
    pub target_nodes: Vec<String>,
    /// Collection of required permissions
    pub required_permissions: Vec<ResourcePermission>,
    /// Collection of automated checks
    pub automated_checks: Vec<AutomatedCheck>,
    /// Collection of escalation conditions
    pub escalation_conditions: Vec<EscalationCondition>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The expires at value
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of bear dog workflow
pub enum BearDogWorkflowType {
    /// Represents data replication variant
    DataReplication {
        backup_nodes: Vec<String>,
        encryption_required: bool,
    },
    ComplianceAudit {
        audit_scope: Vec<String>,
        standards: Vec<String>,
        automated_remediation: bool,
    },
    SecurityIncidentResponse {
        threat_level: u8,
        affected_resources: Vec<String>,
        response_team: Vec<String>,
    },
    GeneticSpawning {
        parent_genetics: Vec<String>,
        spawn_purpose: SpawnPurpose,
        target_capabilities: Vec<NodeCapability>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatedCheck {
    /// Represents resource availability variant
    ResourceAvailability,
    /// Represents security clearance variant
    SecurityClearance,
    /// Represents compliance validation variant
    ComplianceValidation,
    /// Represents trust verification variant
    TrustVerification,
    /// Represents capability match variant
    CapabilityMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    /// Represents high risk operation variant
    HighRiskOperation,
    /// Represents compliance violation variant
    ComplianceViolation,
    /// Represents unknown node variant
    UnknownNode,
    /// Represents resource exhaustion variant
    ResourceExhaustion,
    /// Represents security threat variant
    SecurityThreat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Operation in progress
    Pending,
    /// Operation in progress
    InProgress,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed(String),
    /// Represents requires approval variant
    RequiresApproval,
}
