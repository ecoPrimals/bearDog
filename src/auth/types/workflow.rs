//! Workflow system types for cross-node operations
//!
//! This module contains all types related to workflow management,
//! including workflow requests, definitions, automated checks, and escalation conditions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::authorization::ResourcePermission;
use super::genetics::NodeCapability;
use super::spawning::{SpawnPurpose, TaskType};

/// Cross-node workflow request
///
/// Represents a request to execute a workflow across multiple nodes,
/// including automation checks and escalation conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    /// Unique request identifier
    pub id: String,
    /// Type of workflow requested
    pub workflow_type: BearDogWorkflowType,
    /// Node ID making the request
    pub requester_node_id: String,
    /// Target nodes for the workflow
    pub target_nodes: Vec<String>,
    /// Required permissions for the workflow
    pub required_permissions: Vec<ResourcePermission>,
    /// Automated checks to perform
    pub automated_checks: Vec<AutomatedCheck>,
    /// Conditions that trigger escalation
    pub escalation_conditions: Vec<EscalationCondition>,
    /// When the request was created
    pub created_at: DateTime<Utc>,
    /// When the request expires
    pub expires_at: DateTime<Utc>,
}

/// Workflow types for cross-node operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    /// Data backup workflow
    DataBackup {
        /// Source node for the backup
        source_node: String,
        /// Destination nodes for backup storage
        backup_nodes: Vec<String>,
        /// Whether encryption is required
        encryption_required: bool,
    },
    /// Compliance audit workflow
    ComplianceAudit {
        /// Scope of the audit
        audit_scope: Vec<String>,
        /// Standards to audit against
        standards: Vec<String>,
        /// Whether automated remediation is enabled
        automated_remediation: bool,
    },
    /// Security incident response workflow
    SecurityIncidentResponse {
        /// Threat level (1-10)
        threat_level: u8,
        /// Affected resources
        affected_resources: Vec<String>,
        /// Response team members
        response_team: Vec<String>,
    },
    /// Genetic spawning workflow
    GeneticSpawning {
        /// Parent genetics to use
        parent_genetics: Vec<String>,
        /// Purpose for spawning
        spawn_purpose: SpawnPurpose,
        /// Target capabilities for spawned instance
        target_capabilities: Vec<NodeCapability>,
    },
}

/// Automated checks for workflow approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatedCheck {
    /// Check resource availability
    ResourceAvailability,
    /// Check security clearance
    SecurityClearance,
    /// Validate compliance requirements
    ComplianceValidation,
    /// Verify trust relationships
    TrustVerification,
    /// Match capabilities
    CapabilityMatch,
}

/// Conditions that trigger escalation to human approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    /// High risk operation detected
    HighRiskOperation,
    /// Compliance violation detected
    ComplianceViolation,
    /// Unknown node detected
    UnknownNode,
    /// Resource exhaustion detected
    ResourceExhaustion,
    /// Security threat detected
    SecurityThreat,
}

/// Workflow status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is pending
    Pending,
    /// Workflow is in progress
    InProgress,
    /// Workflow has completed successfully
    Completed,
    /// Workflow has failed with an error
    Failed(String),
    /// Workflow requires human approval
    RequiresApproval,
} 