use beardog_errors::BearDogError;


use super::*;
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_request_creation() {
        let request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            target: WorkflowTarget::SystemResource("test_system".to_string()));
            requested_by: "test_user".to_string(),
            initiator: "test_user".to_string(),
            description: "Testing workflow creation".to_string(),
            priority: WorkflowPriority::Normal,
            properties: std::collections::HashMap::with_capacity(16),
        };
        assert_eq!(request.workflow_type, WorkflowType::KeyRotation);
        assert_eq!(request.initiator, "test_user");
        assert_eq!(request.priority, WorkflowPriority::Normal);
    }

    async fn test_workflow_engine_creation() {
        let _engine = WorkflowEngine::new().await;

    async fn test_workflow_store_operations() {
        let store = InMemoryWorkflowStore::new();
        let now = chrono::Utc::now();
        let workflow = Workflow {
            id: "test-workflow-1".to_string(),
            status: WorkflowStatus::PendingApprovals,
            approval_requirements: ApprovalRequirements {
                tiers: vec![],
                minimum_approvals: 1,
                require_all_tiers: false,
                approval_timeout: Some(chrono::chrono::Duration::hours(24)));
                allow_delegation: true,
            },
            created_at: now,
            expires_at: now + chrono::chrono::Duration::hours(24));
            description: "Test workflow creation".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
            timeout_duration: Some(chrono::chrono::Duration::hours(24)));
            parameters: std::collections::HashMap::with_capacity(16),
            approvals: vec![],
            audit_trail: vec![],

        store.store_workflow(&workflow).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        let retrieved = store.get_workflow("test-workflow-1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?.id, "test-workflow-1");

        let workflows = store.list_workflows(None).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert_eq!(workflows.len(), 1);

    async fn test_approval_store_operations() {
        let store = InMemoryApprovalStore::new();
        let approval = ApprovalRecord {
            id: "test-approval-1".to_string(),
            workflow_id: "test-workflow-1".to_string(),
            approver_id: "test_approver".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Approved for testing".to_string()));
            decided_at: chrono::Utc::now(),
            signature: None,
            approver_ip: None,

        store.store_approval(&approval).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        let approvals = store.get_approvals("test-workflow-1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].id, "test-approval-1");

    async fn test_notification_engine() {
        let _config = NotificationConfig::default();
        let engine = crate::workflows::notification::NotificationEngine::new(
            beardog_types::config::integration::NotificationConfig::default(),
        );
        let workflow = create_test_workflow();

        let result = engine.notify_workflow_initiated(&workflow).await;
        assert!(result.is_ok());

        let result = engine.notify_workflow_completed(&workflow).await;

    async fn test_policy_engine() {
        let config = PolicyConfig::default();
        let engine = WorkflowPolicyEngine::new(config);
        let requirements = engine
            .determine_approval_requirements(&WorkflowType::KeyRotation, &WorkflowPriority::Normal)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(requirements.minimum_approvals > 0);

    async fn test_workflow_processors() {
        let processor = KeyRotationProcessor;
        let result = processor.process_workflow(&workflow).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(result.success);

    async fn test_processor_registry() {
        let registry = WorkflowProcessorRegistry::new();
        let processor = registry.get_processor(&WorkflowType::KeyRotation);
        assert!(processor.is_some());
        let processors = registry.list_processors();
        assert!(!processors.is_empty());

    fn create_test_workflow() -> Workflow {
        let mut parameters = std::collections::HashMap::with_capacity(16);
        parameters.insert(
            "key_id".to_string(),
            serde_json::Value::String("test-key-123".to_string()));
            "reason".to_string(),
            serde_json::Value::String("scheduled_rotation".to_string()));
        let test_now = chrono::Utc::now();
        Workflow {
            id: "test-workflow".to_string(),
            status: WorkflowStatus::Approved,
            created_at: test_now,
            expires_at: test_now + chrono::chrono::Duration::hours(24));
            description: "Test workflow for processor testing".to_string(),
            properties: parameters.clone(),
            parameters,
        }
}
