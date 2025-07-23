//! Unit tests for the workflows module
//!
//! Contains all test functions for workflow functionality.

use super::*;

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// Test workflow request creation
    #[tokio::test]
    async fn test_workflow_request_creation() {
        let request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            target: WorkflowTarget::SystemResource("test_system".to_string()),
            requested_by: "test_user".to_string(),
            initiator: "test_user".to_string(),
            description: "Testing workflow creation".to_string(),
            priority: WorkflowPriority::Normal,
            properties: std::collections::HashMap::new(),
        };

        assert_eq!(request.workflow_type, WorkflowType::KeyRotation);
        assert_eq!(request.initiator, "test_user");
        assert_eq!(request.priority, WorkflowPriority::Normal);
    }

    /// Test workflow engine creation
    #[tokio::test]
    async fn test_workflow_engine_creation() {
        let _engine = WorkflowEngine::new().await;
        // Engine should be created successfully
        // This is mainly testing compilation and basic initialization
    }

    /// Test workflow store operations
    #[tokio::test]
    async fn test_workflow_store_operations() {
        let store = InMemoryWorkflowStore::new();

        let now = chrono::Utc::now();
        let workflow = Workflow {
            id: "test-workflow-1".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            status: WorkflowStatus::PendingApprovals,
            target: WorkflowTarget::SystemResource("test_system".to_string()),
            approval_requirements: ApprovalRequirements {
                tiers: vec![],
                minimum_approvals: 1,
                require_all_tiers: false,
                approval_timeout: Some(chrono::Duration::hours(24)),
                allow_delegation: true,
            },
            priority: WorkflowPriority::Normal,
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            requested_by: "test_user".to_string(),
            initiator: "test_user".to_string(),
            description: "Test workflow creation".to_string(),
            metadata: std::collections::HashMap::new(),
            timeout_duration: Some(chrono::Duration::hours(24)),
            properties: std::collections::HashMap::new(),
            parameters: std::collections::HashMap::new(),
            approvals: vec![],
            audit_trail: vec![],
        };

        // Store workflow
        store.store_workflow(&workflow).await.unwrap();

        // Retrieve workflow
        let retrieved = store.get_workflow("test-workflow-1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "test-workflow-1");

        // List workflows
        let workflows = store.list_workflows(None).await.unwrap();
        assert_eq!(workflows.len(), 1);
    }

    /// Test approval store operations
    #[tokio::test]
    async fn test_approval_store_operations() {
        let store = InMemoryApprovalStore::new();

        let approval = ApprovalRecord {
            id: "test-approval-1".to_string(),
            workflow_id: "test-workflow-1".to_string(),
            approver_id: "test_approver".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Approved for testing".to_string()),
            decided_at: chrono::Utc::now(),
            signature: None,
            approver_ip: None,
        };

        // Store approval
        store.store_approval(&approval).await.unwrap();

        // Retrieve approvals
        let approvals = store.get_approvals("test-workflow-1").await.unwrap();
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].id, "test-approval-1");
    }

    /// Test notification engine
    #[tokio::test]
    async fn test_notification_engine() {
        let _config = NotificationConfig::default();
        let engine = crate::workflows::notification::NotificationEngine::new(
            beardog_config::integration::NotificationConfig::default(),
        );

        let workflow = create_test_workflow();

        // Test workflow initiated notification
        let result = engine.notify_workflow_initiated(&workflow).await;
        assert!(result.is_ok());

        // Test workflow completed notification
        let result = engine.notify_workflow_completed(&workflow).await;
        assert!(result.is_ok());
    }

    /// Test policy engine
    #[tokio::test]
    async fn test_policy_engine() {
        let config = PolicyConfig::default();
        let engine = WorkflowPolicyEngine::new(config);

        let requirements = engine
            .determine_approval_requirements(&WorkflowType::KeyRotation, &WorkflowPriority::Normal)
            .await
            .unwrap();

        assert!(requirements.minimum_approvals > 0);
    }

    /// Test workflow processors
    #[tokio::test]
    async fn test_workflow_processors() {
        let processor = KeyRotationProcessor;
        let workflow = create_test_workflow();

        let result = processor.process_workflow(&workflow).await.unwrap();
        assert!(result.success);
        assert!(result.success);
    }

    /// Test workflow processor registry
    #[tokio::test]
    async fn test_processor_registry() {
        let registry = WorkflowProcessorRegistry::new();

        let processor = registry.get_processor(&WorkflowType::KeyRotation);
        assert!(processor.is_some());

        let processors = registry.list_processors();
        assert!(!processors.is_empty());
    }

    /// Helper function to create a test workflow
    fn create_test_workflow() -> Workflow {
        let mut parameters = std::collections::HashMap::new();
        parameters.insert(
            "key_id".to_string(),
            serde_json::Value::String("test-key-123".to_string()),
        );
        parameters.insert(
            "reason".to_string(),
            serde_json::Value::String("scheduled_rotation".to_string()),
        );

        let test_now = chrono::Utc::now();
        Workflow {
            id: "test-workflow".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            status: WorkflowStatus::Approved,
            target: WorkflowTarget::SystemResource("test_system".to_string()),
            approval_requirements: ApprovalRequirements {
                tiers: vec![],
                minimum_approvals: 1,
                require_all_tiers: false,
                approval_timeout: Some(chrono::Duration::hours(24)),
                allow_delegation: true,
            },
            priority: WorkflowPriority::Normal,
            created_at: test_now,
            expires_at: test_now + chrono::Duration::hours(24),
            requested_by: "test_user".to_string(),
            initiator: "test_user".to_string(),
            description: "Test workflow for processor testing".to_string(),
            metadata: std::collections::HashMap::new(),
            timeout_duration: Some(chrono::Duration::hours(24)),
            properties: parameters.clone(),
            parameters,
            approvals: vec![],
            audit_trail: vec![],
        }
    }
}
