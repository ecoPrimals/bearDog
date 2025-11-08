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

        match workflow_type {
            BearDogWorkflowType::DataReplication {
                backup_nodes,
                encryption_required,
            } => {
                assert_eq!(backup_nodes.len(), 2);
                assert!(encryption_required);
            }
            _ => panic!("Expected DataReplication variant"),
        }
    }

    #[test]
    fn test_workflow_type_compliance_audit() {
        let workflow_type = BearDogWorkflowType::ComplianceAudit {
            audit_scope: vec!["financial".to_string()],
            standards: vec!["SOC2".to_string(), "GDPR".to_string()],
            automated_remediation: true,
        };

        match workflow_type {
            BearDogWorkflowType::ComplianceAudit { standards, .. } => {
                assert_eq!(standards.len(), 2);
            }
            _ => panic!("Expected ComplianceAudit variant"),
        }
    }

    #[test]
    fn test_workflow_type_security_incident() {
        let workflow_type = BearDogWorkflowType::SecurityIncidentResponse {
            threat_level: 8,
            affected_resources: vec!["db-1".to_string()],
            response_team: vec!["security-team".to_string()],
        };

        match workflow_type {
            BearDogWorkflowType::SecurityIncidentResponse { threat_level, .. } => {
                assert_eq!(threat_level, 8);
            }
            _ => panic!("Expected SecurityIncidentResponse variant"),
        }
    }

    #[test]
    fn test_workflow_type_genetic_spawning() {
        let workflow_type = BearDogWorkflowType::GeneticSpawning {
            parent_genetics: vec!["gen-1".to_string()],
            spawn_purpose: SpawnPurpose::TaskExecution,
            target_capabilities: vec![NodeCapability::BasicOperations],
        };

        match workflow_type {
            BearDogWorkflowType::GeneticSpawning {
                parent_genetics, ..
            } => {
                assert!(!parent_genetics.is_empty());
            }
            _ => panic!("Expected GeneticSpawning variant"),
        }
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
        match status {
            WorkflowStatus::Failed(msg) => {
                assert_eq!(msg, "Network timeout");
            }
            _ => panic!("Expected Failed variant"),
        }
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
