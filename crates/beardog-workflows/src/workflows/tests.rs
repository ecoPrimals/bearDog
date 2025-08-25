// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Unit tests for the workflows module
///
/// Contains all test functions for workflow functionality.

use super::*;
#[cfg(test)]
mod unit_tests {
    use super::*;
    /// Test workflow request creation
    #[tokio::test]
    async fn test_workflow_request_creation() {
        let request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            target: WorkflowTarget::SystemResource("test_system".to_string()));
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
    async fn test_workflow_engine_creation() {
        let _engine = WorkflowEngine::new().await;
        // Engine should be created successfully
        // This is mainly testing compilation and basic initialization
    /// Test workflow store operations}


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
            metadata: std::collections::HashMap::new(),
            timeout_duration: Some(chrono::chrono::Duration::hours(24)));
            parameters: std::collections::HashMap::new(),
            approvals: vec![],
            audit_trail: vec![],
        // Store workflow
        store.store_workflow(&workflow).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        // Retrieve workflow
        let retrieved = store.get_workflow("test-workflow-1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?.id, "test-workflow-1");
        // List workflows
        let workflows = store.list_workflows(None).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert_eq!(workflows.len(), 1);
    /// Test approval store operations
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
        // Store approval
        store.store_approval(&approval).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        // Retrieve approvals
        let approvals = store.get_approvals("test-workflow-1").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].id, "test-approval-1");
    /// Test notification engine}


    async fn test_notification_engine() {
        let _config = NotificationConfig::default();
        let engine = crate::workflows::notification::NotificationEngine::new(
            beardog_types::config::integration::NotificationConfig::default(),
        );
        let workflow = create_test_workflow();
        // Test workflow initiated notification
        let result = engine.notify_workflow_initiated(&workflow).await;
        assert!(result.is_ok());
        // Test workflow completed notification
        let result = engine.notify_workflow_completed(&workflow).await;
    /// Test policy engine
    async fn test_policy_engine() {
        let config = PolicyConfig::default();
        let engine = WorkflowPolicyEngine::new(config);
        let requirements = engine
            .determine_approval_requirements(&WorkflowType::KeyRotation, &WorkflowPriority::Normal)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(requirements.minimum_approvals > 0);
    /// Test workflow processors}


    async fn test_workflow_processors() {
        let processor = KeyRotationProcessor;
        let result = processor.process_workflow(&workflow).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(result.success);
    /// Test workflow processor registry
    async fn test_processor_registry() {
        let registry = WorkflowProcessorRegistry::new();
        let processor = registry.get_processor(&WorkflowType::KeyRotation);
        assert!(processor.is_some());
        let processors = registry.list_processors();
        assert!(!processors.is_empty());
    /// Helper function to create a test workflow}


    fn create_test_workflow() -> Workflow {
        let mut parameters = std::collections::HashMap::new();
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
