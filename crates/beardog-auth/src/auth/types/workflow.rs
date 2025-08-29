use super::authorization::ResourcePermission;
use super::genetics::NodeCapability;
use super::spawning::SpawnPurpose;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    pub id: String,

    pub workflow_type: BearDogWorkflowType,

    pub requester_node_id: String,

    pub target_nodes: Vec<String>,

    pub required_permissions: Vec<ResourcePermission>,

    pub automated_checks: Vec<AutomatedCheck>,

    pub escalation_conditions: Vec<EscalationCondition>,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    DataBackup {
        source_node: String,

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
    ResourceAvailability,

    SecurityClearance,

    ComplianceValidation,

    TrustVerification,

    CapabilityMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    HighRiskOperation,

    ComplianceViolation,

    UnknownNode,

    ResourceExhaustion,

    SecurityThreat,
}

pub enum WorkflowStatus {
    Pending,

    InProgress,

    Completed,

    Failed(String),

    RequiresApproval,
}
