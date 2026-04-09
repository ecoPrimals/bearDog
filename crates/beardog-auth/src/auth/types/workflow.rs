// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::authorization::ResourcePermission;
use super::genetics::NodeCapability;
use super::spawning::SpawnPurpose;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Declarative description of work that spans multiple nodes and requires elevated permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    /// Caller-supplied identifier used for idempotency and log correlation.
    pub workflow_id: String,
    /// The workflow type value
    pub workflow_type: BearDogWorkflowType,
    /// Node that submitted the workflow and will receive callbacks/approvals.
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

/// Discriminated union of supported multi-node workflow shapes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    /// Represents data replication variant
    DataReplication {
        /// Peers that should receive replicated shards or backups.
        backup_nodes: Vec<String>,
        /// When true, payloads must be encrypted in flight and at rest on replicas.
        encryption_required: bool,
    },
    /// Scheduled or ad-hoc compliance review across `audit_scope` using named `standards`.
    ComplianceAudit {
        /// Resources or services included in the audit boundary.
        audit_scope: Vec<String>,
        /// Framework identifiers (`SOC2`, `GDPR`, …) that drive checklists.
        standards: Vec<String>,
        /// Whether failing controls may trigger automated remediation playbooks.
        automated_remediation: bool,
    },
    /// Coordinated response playbook for an active or suspected incident.
    SecurityIncidentResponse {
        /// Subjective severity on a 0–10 scale for prioritization and paging.
        threat_level: u8,
        /// Asset identifiers impacted or potentially impacted.
        affected_resources: Vec<String>,
        /// Human or bot roles that must acknowledge the workflow.
        response_team: Vec<String>,
    },
    /// Requests synthesis of a child primal with genetics derived from `parent_genetics`.
    GeneticSpawning {
        /// Parent genome identifiers whose traits should be inherited or merged.
        parent_genetics: Vec<String>,
        /// Business justification influencing approval policy.
        spawn_purpose: SpawnPurpose,
        /// Capabilities the spawned primal must exhibit after creation.
        target_capabilities: Vec<NodeCapability>,
    },
}

/// Pre-flight predicate evaluated before a workflow is allowed to run.
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

/// Signals that human review or elevated approval is required before continuing.
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

/// Lifecycle state reported by a [`WorkflowEngine`](crate::auth::types::node_registry::WorkflowEngine) for a submitted workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Accepted but not yet assigned to executors.
    Pending,
    /// Actively running on one or more nodes.
    InProgress,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed(String),
    /// Represents requires approval variant
    RequiresApproval,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_request_creation() {
        let request = CrossNodeWorkflowRequest {
            workflow_id: "wf-001".to_string(),
            workflow_type: BearDogWorkflowType::DataReplication {
                backup_nodes: vec!["node-1".to_string()],
                encryption_required: true,
            },
            requester_node_id: "requester-node".to_string(),
            target_nodes: vec!["target-1".to_string()],
            required_permissions: vec![],
            automated_checks: vec![AutomatedCheck::ResourceAvailability],
            escalation_conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        };

        assert_eq!(request.workflow_id, "wf-001");
        assert_eq!(request.target_nodes.len(), 1);
    }

    #[test]
    fn test_workflow_type_data_replication() {
        let workflow_type = BearDogWorkflowType::DataReplication {
            backup_nodes: vec!["node-1".to_string(), "node-2".to_string()],
            encryption_required: true,
        };

        let BearDogWorkflowType::DataReplication {
            backup_nodes,
            encryption_required,
        } = workflow_type
        else {
            panic!("Expected DataReplication variant");
        };
        assert_eq!(backup_nodes.len(), 2);
        assert!(encryption_required);
    }

    #[test]
    fn test_workflow_type_compliance_audit() {
        let workflow_type = BearDogWorkflowType::ComplianceAudit {
            audit_scope: vec!["financial".to_string()],
            standards: vec!["SOC2".to_string(), "GDPR".to_string()],
            automated_remediation: true,
        };

        let BearDogWorkflowType::ComplianceAudit { standards, .. } = workflow_type else {
            panic!("Expected ComplianceAudit variant");
        };
        assert_eq!(standards.len(), 2);
    }

    #[test]
    fn test_workflow_type_security_incident() {
        let workflow_type = BearDogWorkflowType::SecurityIncidentResponse {
            threat_level: 8,
            affected_resources: vec!["db-1".to_string()],
            response_team: vec!["security-team".to_string()],
        };

        let BearDogWorkflowType::SecurityIncidentResponse { threat_level, .. } = workflow_type
        else {
            panic!("Expected SecurityIncidentResponse variant");
        };
        assert_eq!(threat_level, 8);
    }

    #[test]
    fn test_workflow_type_genetic_spawning() {
        let workflow_type = BearDogWorkflowType::GeneticSpawning {
            parent_genetics: vec!["gen-1".to_string()],
            spawn_purpose: SpawnPurpose::TaskExecution,
            target_capabilities: vec![NodeCapability::BasicOperations],
        };

        let BearDogWorkflowType::GeneticSpawning {
            parent_genetics, ..
        } = workflow_type
        else {
            panic!("Expected GeneticSpawning variant");
        };
        assert!(!parent_genetics.is_empty());
    }

    #[test]
    fn test_automated_check_variants() {
        let checks = [
            AutomatedCheck::ResourceAvailability,
            AutomatedCheck::SecurityClearance,
            AutomatedCheck::ComplianceValidation,
            AutomatedCheck::TrustVerification,
            AutomatedCheck::CapabilityMatch,
        ];

        assert_eq!(checks.len(), 5);
    }

    #[test]
    fn test_escalation_condition_variants() {
        let conditions = [
            EscalationCondition::HighRiskOperation,
            EscalationCondition::ComplianceViolation,
            EscalationCondition::UnknownNode,
            EscalationCondition::ResourceExhaustion,
            EscalationCondition::SecurityThreat,
        ];

        assert_eq!(conditions.len(), 5);
    }

    #[test]
    fn test_workflow_status_pending() {
        let status = WorkflowStatus::Pending;
        assert!(matches!(status, WorkflowStatus::Pending));
    }

    #[test]
    fn test_workflow_status_in_progress() {
        let status = WorkflowStatus::InProgress;
        assert!(matches!(status, WorkflowStatus::InProgress));
    }

    #[test]
    fn test_workflow_status_completed() {
        let status = WorkflowStatus::Completed;
        assert!(matches!(status, WorkflowStatus::Completed));
    }

    #[test]
    fn test_workflow_status_failed() {
        let status = WorkflowStatus::Failed("Network timeout".to_string());
        let WorkflowStatus::Failed(msg) = status else {
            panic!("Expected Failed variant");
        };
        assert_eq!(msg, "Network timeout");
    }

    #[test]
    fn test_workflow_status_requires_approval() {
        let status = WorkflowStatus::RequiresApproval;
        assert!(matches!(status, WorkflowStatus::RequiresApproval));
    }

    #[test]
    fn test_workflow_request_with_multiple_checks() {
        let request = CrossNodeWorkflowRequest {
            workflow_id: "wf-002".to_string(),
            workflow_type: BearDogWorkflowType::ComplianceAudit {
                audit_scope: vec!["security".to_string()],
                standards: vec!["ISO27001".to_string()],
                automated_remediation: false,
            },
            requester_node_id: "auditor-node".to_string(),
            target_nodes: vec!["target-1".to_string(), "target-2".to_string()],
            required_permissions: vec![],
            automated_checks: vec![
                AutomatedCheck::SecurityClearance,
                AutomatedCheck::ComplianceValidation,
                AutomatedCheck::TrustVerification,
            ],
            escalation_conditions: vec![
                EscalationCondition::ComplianceViolation,
                EscalationCondition::HighRiskOperation,
            ],
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(48),
        };

        assert_eq!(request.automated_checks.len(), 3);
        assert_eq!(request.escalation_conditions.len(), 2);
        assert_eq!(request.target_nodes.len(), 2);
    }

    #[test]
    fn test_workflow_request_serialization() {
        let request = CrossNodeWorkflowRequest {
            workflow_id: "wf-003".to_string(),
            workflow_type: BearDogWorkflowType::DataReplication {
                backup_nodes: vec!["backup-node".to_string()],
                encryption_required: true,
            },
            requester_node_id: "req-node".to_string(),
            target_nodes: vec!["target".to_string()],
            required_permissions: vec![],
            automated_checks: vec![],
            escalation_conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
        };

        // Test that serialization/deserialization works
        let json = serde_json::to_string(&request);
        assert!(json.is_ok(), "Should be able to serialize workflow request");
    }
}
