//! Compliance and Workflow Tests for BearDog
//!
//! Tests for compliance monitoring, workflow orchestration, and multi-party approvals

use beardog::compliance::*;
use beardog::workflows::*;
use beardog::config::integration::{WorkflowConfig, WorkflowStorageConfig, NotificationConfig, PolicyConfig};
use beardog::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_compliance_engine_initialization() -> BearDogResult<()> {
    // Test compliance engine initialization
    let config = ComplianceConfig {
        enabled_standards: vec![
            ComplianceStandard::GDPR,
            ComplianceStandard::SOX,
            ComplianceStandard::HIPAA,
        ],
        monitoring_interval: chrono::Duration::minutes(5),
        audit_retention: chrono::Duration::days(365),
        dashboard_refresh_interval: chrono::Duration::minutes(1),
        reporting: ReportingConfig::default(),
    };
    
    let engine = ComplianceEngine::new(config).await?;
    
    // Verify engine is properly initialized
    let status = engine.get_compliance_status().await?;
    assert!(!status.enabled_standards.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_gdpr_compliance_monitoring() -> BearDogResult<()> {
    // Test GDPR-specific compliance monitoring
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::GDPR],
        monitoring_interval: chrono::Duration::minutes(1),
        audit_retention: chrono::Duration::days(365),
        dashboard_refresh_interval: chrono::Duration::minutes(1),
        reporting: ReportingConfig::default(),
    };
    
    let engine = ComplianceEngine::new(config).await?;
    
    // Test GDPR compliance event
    let gdpr_event = ComplianceEvent {
        event_id: "gdpr-test-001".to_string(),
        event_type: ComplianceEventType::DataProcessing,
        timestamp: chrono::Utc::now(),
        user_id: Some("test-user-eu".to_string()),
        data_categories: vec![
            DataCategory::PersonalIdentifiable,
            DataCategory::Sensitive,
        ],
        processing_purpose: Some("user authentication".to_string()),
        legal_basis: Some(LegalBasis::Consent),
        retention_period: Some(chrono::Duration::days(90)),
        metadata: HashMap::from([
            ("jurisdiction".to_string(), "EU".to_string()),
            ("consent_timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
        ]),
    };
    
    // Process the compliance event
    let result = engine.process_compliance_event(gdpr_event).await?;
    
    assert!(result.compliant, "GDPR event with proper consent should be compliant");
    assert!(result.violations.is_empty(), "Should have no violations");
    
    println!("✅ GDPR compliance monitoring successful");
    
    Ok(())
}

#[tokio::test]
async fn test_sox_compliance_monitoring() -> BearDogResult<()> {
    // Test SOX compliance for financial data
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::SOX],
        monitoring_interval: chrono::Duration::minutes(1),
        audit_retention: chrono::Duration::days(2555), // 7 years for SOX
        dashboard_refresh_interval: chrono::Duration::minutes(1),
        reporting: ReportingConfig::default(),
    };
    
    let engine = ComplianceEngine::new(config).await?;
    
    // Test SOX financial record access
    let sox_event = ComplianceEvent {
        event_id: "sox-financial-001".to_string(),
        event_type: ComplianceEventType::FinancialRecordAccess,
        timestamp: chrono::Utc::now(),
        user_id: Some("financial-analyst-001".to_string()),
        data_categories: vec![DataCategory::Financial],
        processing_purpose: Some("quarterly financial analysis".to_string()),
        legal_basis: Some(LegalBasis::LegitimateInterest),
        retention_period: Some(chrono::Duration::days(2555)), // SOX requirement
        metadata: HashMap::from([
            ("department".to_string(), "finance".to_string()),
            ("supervisor_approval".to_string(), "approved".to_string()),
            ("access_level".to_string(), "read-only".to_string()),
        ]),
    };
    
    let result = engine.process_compliance_event(sox_event).await?;
    
    assert!(result.compliant, "SOX event with proper authorization should be compliant");
    
    println!("✅ SOX compliance monitoring successful");
    
    Ok(())
}

#[tokio::test]
async fn test_workflow_engine_initialization() -> BearDogResult<()> {
    // Test workflow engine initialization
    let config = WorkflowConfig {
        default_approval_timeout: Duration::from_secs(3600),
        max_concurrent_workflows: 100,
        storage: WorkflowStorageConfig {
            storage_type: "memory".to_string(),
            config: HashMap::new(),
        },
        notifications: NotificationConfig::default(),
        policies: PolicyConfig::default(),
    };
    
    let engine = MultiPartyWorkflowEngine::new(config).await?;
    
    // Verify engine is properly initialized
    let status = engine.get_status().await?;
    assert_eq!(status.active_workflows, 0);
    
    Ok(())
}

#[tokio::test]
async fn test_automated_consensus_workflow() -> BearDogResult<()> {
    // Test automated consensus workflow for genetic spawning
    let config = WorkflowConfig {
        default_approval_timeout: Duration::from_secs(300),
        max_concurrent_workflows: 10,
        storage: WorkflowStorageConfig {
            storage_type: "memory".to_string(),
            config: HashMap::new(),
        },
        notifications: NotificationConfig::default(),
        policies: PolicyConfig::default(),
    };
    
    let engine = MultiPartyWorkflowEngine::new(config).await?;
    
    // Create automated consensus workflow request
    let workflow_request = WorkflowRequest {
        request_id: "spawn-consensus-001".to_string(),
        workflow_type: WorkflowType::AutomatedConsensus,
        target: WorkflowTarget::CrossNodeOperation("genetic-spawn-request".to_string()),
        priority: WorkflowPriority::Medium,
        requesting_node: "beardog-alpha".to_string(),
        operation_data: serde_json::json!({
            "spawn_purpose": "emergency_response",
            "parent_genetics": ["beardog-genesis", "toadstool-compute-beta"],
            "resource_requirements": {
                "cpu_cores": 4,
                "memory_gb": 8
            }
        }),
        approval_requirements: ApprovalRequirements {
            min_approvals: 2,
            required_roles: vec!["security_node".to_string(), "compute_node".to_string()],
            consensus_threshold: 0.66,
            timeout: Duration::from_secs(300),
        },
        compliance_requirements: vec![
            ComplianceRequirement::DataSovereignty,
            ComplianceRequirement::GDPR,
        ],
        metadata: HashMap::new(),
    };
    
    // Submit workflow request
    let workflow_id = engine.submit_workflow(workflow_request).await?;
    assert!(!workflow_id.is_empty());
    
    // Check workflow status
    let status = engine.get_workflow_status(&workflow_id).await?;
    assert_eq!(status.status, WorkflowStatus::Pending);
    
    // Simulate automated approvals
    let approval1 = ApprovalSubmission {
        workflow_id: workflow_id.clone(),
        approver_id: "security-node-001".to_string(),
        approver_role: "security_node".to_string(),
        decision: ApprovalDecision::Approved,
        reason: "Security review passed - spawn request meets security requirements".to_string(),
        timestamp: chrono::Utc::now(),
        signature: None,
    };
    
    let approval_result = engine.submit_approval(approval1).await?;
    assert!(approval_result.accepted);
    
    let approval2 = ApprovalSubmission {
        workflow_id: workflow_id.clone(),
        approver_id: "compute-node-001".to_string(),
        approver_role: "compute_node".to_string(),
        decision: ApprovalDecision::Approved,
        reason: "Compute resources available - spawn request approved".to_string(),
        timestamp: chrono::Utc::now(),
        signature: None,
    };
    
    // Submit approvals
    engine.submit_approval(approval2).await?;
    
    // Check final status
    let final_status = engine.get_workflow_status(&workflow_id).await?;
    assert_eq!(final_status.status, WorkflowStatus::Approved);
    
    println!("✅ Automated consensus workflow successful");
    println!("   - Workflow ID: {}", workflow_id);
    println!("   - Final status: {:?}", final_status.status);
    
    Ok(())
}

#[tokio::test]
async fn test_human_approval_workflow() -> BearDogResult<()> {
    // Test human approval workflow for sensitive operations
    let config = WorkflowConfig {
        default_approval_timeout: Duration::from_secs(3600),
        max_concurrent_workflows: 10,
        storage: WorkflowStorageConfig {
            storage_type: "memory".to_string(),
            config: HashMap::new(),
        },
        notifications: NotificationConfig::default(),
        policies: PolicyConfig::default(),
    };
    
    let engine = MultiPartyWorkflowEngine::new(config).await?;
    
    // Create human approval workflow request
    let workflow_request = WorkflowRequest {
        request_id: "sensitive-operation-001".to_string(),
        workflow_type: WorkflowType::HumanApprovalRequired,
        target: WorkflowTarget::SecurityOperation("high-privilege-access".to_string()),
        priority: WorkflowPriority::High,
        requesting_node: "beardog-production".to_string(),
        operation_data: serde_json::json!({
            "operation_type": "privileged_data_access",
            "requested_by": "security-analyst-001",
            "data_classification": "confidential",
            "business_justification": "security incident investigation"
        }),
        approval_requirements: ApprovalRequirements {
            min_approvals: 2,
            required_roles: vec!["security_officer".to_string(), "compliance_officer".to_string()],
            consensus_threshold: 1.0, // Unanimous approval required
            timeout: Duration::from_secs(3600),
        },
        compliance_requirements: vec![
            ComplianceRequirement::SOX,
            ComplianceRequirement::GDPR,
        ],
        metadata: HashMap::new(),
    };
    
    // Submit workflow request
    let workflow_id = engine.submit_workflow(workflow_request).await?;
    
    // Simulate human approvals
    let security_approval = ApprovalSubmission {
        workflow_id: workflow_id.clone(),
        approver_id: "security-officer-alice".to_string(),
        approver_role: "security_officer".to_string(),
        decision: ApprovalDecision::Approved,
        reason: "Approved for incident investigation - legitimate business need".to_string(),
        timestamp: chrono::Utc::now(),
        signature: None,
    };
    
    let compliance_approval = ApprovalSubmission {
        workflow_id: workflow_id.clone(),
        approver_id: "compliance-officer-bob".to_string(),
        approver_role: "compliance_officer".to_string(),
        decision: ApprovalDecision::Approved,
        reason: "Compliance review passed - appropriate safeguards in place".to_string(),
        timestamp: chrono::Utc::now(),
        signature: None,
    };
    
    // Submit approvals
    engine.submit_approval(security_approval).await?;
    engine.submit_approval(compliance_approval).await?;
    
    // Check final status
    let final_status = engine.get_workflow_status(&workflow_id).await?;
    assert_eq!(final_status.status, WorkflowStatus::Approved);
    
    println!("✅ Human approval workflow successful");
    
    Ok(())
}

#[tokio::test]
async fn test_workflow_rejection() -> BearDogResult<()> {
    // Test workflow rejection scenario
    let config = WorkflowConfig {
        default_approval_timeout: Duration::from_secs(300),
        max_concurrent_workflows: 10,
        storage: WorkflowStorageConfig {
            storage_type: "memory".to_string(),
            config: HashMap::new(),
        },
        notifications: NotificationConfig::default(),
        policies: PolicyConfig::default(),
    };
    
    let engine = MultiPartyWorkflowEngine::new(config).await?;
    
    // Create workflow request that will be rejected
    let workflow_request = WorkflowRequest {
        request_id: "suspicious-request-001".to_string(),
        workflow_type: WorkflowType::HumanApprovalRequired,
        target: WorkflowTarget::SecurityOperation("suspicious-access".to_string()),
        priority: WorkflowPriority::Low,
        requesting_node: "unknown-node".to_string(),
        operation_data: serde_json::json!({
            "operation_type": "bulk_data_export",
            "requested_by": "unknown-user",
            "data_classification": "sensitive"
        }),
        approval_requirements: ApprovalRequirements {
            min_approvals: 1,
            required_roles: vec!["security_officer".to_string()],
            consensus_threshold: 1.0,
            timeout: Duration::from_secs(300),
        },
        compliance_requirements: vec![],
        metadata: HashMap::new(),
    };
    
    // Submit workflow request
    let workflow_id = engine.submit_workflow(workflow_request).await?;
    
    // Submit rejection
    let rejection = ApprovalSubmission {
        workflow_id: workflow_id.clone(),
        approver_id: "security-officer-alice".to_string(),
        approver_role: "security_officer".to_string(),
        decision: ApprovalDecision::Rejected,
        reason: "Request rejected - unknown requester and suspicious activity pattern".to_string(),
        timestamp: chrono::Utc::now(),
        signature: None,
    };
    
    let rejection_result = engine.submit_approval(rejection).await?;
    assert!(rejection_result.accepted);
    
    // Check final status
    let final_status = engine.get_workflow_status(&workflow_id).await?;
    assert_eq!(final_status.status, WorkflowStatus::Rejected);
    
    println!("✅ Workflow rejection handling successful");
    
    Ok(())
}

#[tokio::test]
async fn test_compliance_violation_detection() -> BearDogResult<()> {
    // Test compliance violation detection
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::GDPR],
        monitoring_interval: chrono::Duration::minutes(1),
        audit_retention: chrono::Duration::days(365),
        dashboard_refresh_interval: chrono::Duration::minutes(1),
        reporting: ReportingConfig::default(),
    };
    
    let engine = ComplianceEngine::new(config).await?;
    
    // Create GDPR violation event (processing without consent)
    let violation_event = ComplianceEvent {
        event_id: "gdpr-violation-001".to_string(),
        event_type: ComplianceEventType::DataProcessing,
        timestamp: chrono::Utc::now(),
        user_id: Some("eu-user-001".to_string()),
        data_categories: vec![DataCategory::PersonalIdentifiable],
        processing_purpose: Some("marketing".to_string()),
        legal_basis: None, // No legal basis = violation
        retention_period: Some(chrono::Duration::days(365)),
        metadata: HashMap::from([
            ("jurisdiction".to_string(), "EU".to_string()),
            ("consent_status".to_string(), "not_obtained".to_string()),
        ]),
    };
    
    // Process the violation event
    let result = engine.process_compliance_event(violation_event).await?;
    
    assert!(!result.compliant, "Event without legal basis should be non-compliant");
    assert!(!result.violations.is_empty(), "Should detect GDPR violation");
    
    // Check violation details
    let violation = &result.violations[0];
    assert_eq!(violation.standard, ComplianceStandard::GDPR);
    assert!(violation.description.contains("legal basis"));
    
    println!("✅ Compliance violation detection successful");
    println!("   - Detected violation: {}", violation.description);
    
    Ok(())
}

// Supporting types for compliance and workflow testing
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum ComplianceEventType {
    DataProcessing,
    DataAccess,
    DataTransfer,
    DataDeletion,
    FinancialRecordAccess,
    SystemAccess,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum DataCategory {
    PersonalIdentifiable,
    Sensitive,
    Financial,
    Health,
    Biometric,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum LegalBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterest,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum ApprovalDecision {
    Approved,
    Rejected,
    ConditionallyApproved,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum WorkflowStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
    Expired,
} 