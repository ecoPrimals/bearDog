//! Unit tests for the workflows module
//!
//! Contains all test functions for workflow functionality.

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test workflow request creation
    #[tokio::test]
    async fn test_workflow_request_creation() {
        let request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test_user".to_string(),
            target: WorkflowTarget::System,
            parameters: std::collections::HashMap::new(),
            reason: "Testing workflow creation".to_string(),
            priority: WorkflowPriority::Normal,
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(request.workflow_type, WorkflowType::KeyRotation);
        assert_eq!(request.initiator, "test_user");
        assert_eq!(request.priority, WorkflowPriority::Normal);
    }

    /// Test workflow engine creation
    #[tokio::test]
    async fn test_workflow_engine_creation() {
        let engine = WorkflowEngine::new().await;
        // Engine should be created successfully
        // This is mainly testing compilation and basic initialization
    }

    /// Test workflow store operations
    #[tokio::test]
    async fn test_workflow_store_operations() {
        let store = InMemoryWorkflowStore::new();

        let workflow = Workflow {
            id: "test-workflow-1".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test_user".to_string(),
            target: WorkflowTarget::System,
            parameters: std::collections::HashMap::new(),
            approval_requirements: ApprovalRequirements {
                required_approvals: 1,
                required_roles: vec!["admin".to_string()],
                approval_hierarchy: vec![],
                min_approval_time: chrono::Duration::minutes(5),
                max_approval_time: chrono::Duration::hours(24),
                delegation_allowed: true,
                self_approval_allowed: false,
            },
            status: WorkflowStatus::PendingApprovals,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
            approvals: vec![],
            audit_trail: vec![],
            metadata: std::collections::HashMap::new(),
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
            approver: "test_approver".to_string(),
            approver_role: "admin".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Approved for testing".to_string()),
            timestamp: chrono::Utc::now(),
            signature: None,
            metadata: std::collections::HashMap::new(),
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
        let config = NotificationConfig::default();
        let engine = NotificationEngine::new(config);

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

        assert!(requirements.required_approvals > 0);
        assert!(!requirements.required_roles.is_empty());
    }

    /// Test workflow processors
    #[tokio::test]
    async fn test_workflow_processors() {
        let processor = KeyRotationProcessor;
        let workflow = create_test_workflow();

        let result = processor.process_workflow(&workflow).await.unwrap();
        assert!(result.success);
        assert_eq!(result.workflow_id, workflow.id);
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
        Workflow {
            id: "test-workflow".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test_user".to_string(),
            target: WorkflowTarget::System,
            parameters: std::collections::HashMap::new(),
            approval_requirements: ApprovalRequirements {
                required_approvals: 1,
                required_roles: vec!["admin".to_string()],
                approval_hierarchy: vec![],
                min_approval_time: chrono::Duration::minutes(5),
                max_approval_time: chrono::Duration::hours(24),
                delegation_allowed: true,
                self_approval_allowed: false,
            },
            status: WorkflowStatus::PendingApprovals,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
            approvals: vec![],
            audit_trail: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
}
