// Workflow Tests
//
// Testing workflow orchestration and process management

use super::*;

#[test]
fn test_workflow_config_default() {
    let config = WorkflowConfig::default();
    
    assert!(config.max_concurrent_workflows > 0);
    assert!(config.default_timeout_seconds > 0);
    assert!(config.retry_attempts > 0);
}

#[test]
fn test_workflow_config_custom() {
    let config = WorkflowConfig {
        max_concurrent_workflows: 20,
        default_timeout_seconds: 600,
        retry_attempts: 5,
        enable_audit_logging: true,
        workflow_storage_path: "/custom/path".to_string(),
    };
    
    assert_eq!(config.max_concurrent_workflows, 20);
    assert_eq!(config.default_timeout_seconds, 600);
    assert_eq!(config.retry_attempts, 5);
    assert!(config.enable_audit_logging);
}

#[test]
fn test_workflow_status_transitions() {
    use types::WorkflowStatus;
    
    let pending = WorkflowStatus::Pending;
    let running = WorkflowStatus::Running;
    let completed = WorkflowStatus::Completed;
    let failed = WorkflowStatus::Failed;
    
    assert_ne!(pending, running);
    assert_ne!(running, completed);
    assert_ne!(completed, failed);
}

#[test]
fn test_workflow_step_creation() {
    use types::WorkflowStep;
    
    let step = WorkflowStep {
        name: "test_step".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        action: "process".to_string(),
        parameters: std::collections::HashMap::new(),
        timeout_seconds: 60,
        retry_on_failure: true,
    };
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    assert_eq!(step.name, "test_step");
    assert_eq!(step.action, "process");
    assert!(step.retry_on_failure);
}

#[test]
fn test_workflow_step_with_parameters() {
    use types::WorkflowStep;
    
    let mut parameters = std::collections::HashMap::new();
    parameters.insert("key1".to_string(), "value1".to_string());
    parameters.insert("key2".to_string(), "value2".to_string());
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    let step = WorkflowStep {
        name: "parameterized_step".to_string(),
        action: "transform".to_string(),
        parameters,
        timeout_seconds: 120,
        retry_on_failure: false,
    };
    
    assert_eq!(step.parameters.len(), 2);
    assert_eq!(step.parameters.get("key1"), Some(&"value1".to_string()));
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: workflows
 // TEST_PRIORITY: normal

#[test]
fn test_workflow_definition_creation() {
    use types::{WorkflowDefinition, WorkflowStep};
    
    let steps = vec![
        WorkflowStep {
            name: "step1".to_string(),
            action: "init".to_string(),
            parameters: std::collections::HashMap::new(),
            timeout_seconds: 30,
            retry_on_failure: true,
        },
        WorkflowStep {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: workflows
            // TEST_PRIORITY: normal
            name: "step2".to_string(),
            action: "process".to_string(),
            parameters: std::collections::HashMap::new(),
            timeout_seconds: 60,
            retry_on_failure: true,
        },
    ];
    
    let definition = WorkflowDefinition {
        id: "workflow_001".to_string(),
        name: "Test Workflow".to_string(),
        description: "A test workflow".to_string(),
        steps,
        version: "1.0.0".to_string(),
    };
    
    assert_eq!(definition.steps.len(), 2);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    assert_eq!(definition.version, "1.0.0");
}

#[test]
fn test_workflow_execution_creation() {
    use types::{WorkflowExecution, WorkflowStatus};
    
    let execution = WorkflowExecution {
        id: "exec_001".to_string(),
        workflow_id: "workflow_001".to_string(),
        status: WorkflowStatus::Pending,
        current_step: 0,
        started_at: None,
        completed_at: None,
        error: None,
    };
    
    assert_eq!(execution.status, WorkflowStatus::Pending);
    assert_eq!(execution.current_step, 0);
    assert!(execution.started_at.is_none());
}

#[test]
fn test_workflow_execution_progress() {
    use types::{WorkflowExecution, WorkflowStatus};
    
    let mut execution = WorkflowExecution {
        id: "exec_002".to_string(),
        workflow_id: "workflow_002".to_string(),
        status: WorkflowStatus::Running,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        current_step: 2,
        started_at: Some(chrono::Utc::now()),
        completed_at: None,
        error: None,
    };
    
    assert_eq!(execution.current_step, 2);
    assert!(execution.started_at.is_some());
    assert!(execution.completed_at.is_none());
    
    // Simulate completion
    execution.status = WorkflowStatus::Completed;
    execution.completed_at = Some(chrono::Utc::now());
    
    assert_eq!(execution.status, WorkflowStatus::Completed);
    assert!(execution.completed_at.is_some());
// TEST_CATEGORY: unit
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal
}

#[test]
fn test_workflow_execution_error_handling() {
    use types::{WorkflowExecution, WorkflowStatus};
    
    let execution = WorkflowExecution {
        id: "exec_003".to_string(),
        workflow_id: "workflow_003".to_string(),
        status: WorkflowStatus::Failed,
        current_step: 1,
        started_at: Some(chrono::Utc::now()),
        completed_at: Some(chrono::Utc::now()),
        error: Some("Step execution failed".to_string()),
    };
    
    assert_eq!(execution.status, WorkflowStatus::Failed);
    assert!(execution.error.is_some());
    assert_eq!(execution.error.unwrap(), "Step execution failed");
}

#[test]
fn test_workflow_config_validation() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: important
    let config = WorkflowConfig {
        max_concurrent_workflows: 10,
        default_timeout_seconds: 300,
        retry_attempts: 3,
        enable_audit_logging: true,
        workflow_storage_path: "/valid/path".to_string(),
    };
    
    assert!(config.max_concurrent_workflows >= 1);
    assert!(config.default_timeout_seconds >= 1);
    assert!(config.retry_attempts >= 0);
    assert!(!config.workflow_storage_path.is_empty());
}

#[test]
fn test_workflow_step_timeout_validation() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    use types::WorkflowStep;
    
    let step = WorkflowStep {
        name: "timeout_test".to_string(),
        action: "long_running".to_string(),
        parameters: std::collections::HashMap::new(),
        timeout_seconds: 3600, // 1 hour
        retry_on_failure: true,
    };
    
    assert!(step.timeout_seconds > 0);
    assert!(step.timeout_seconds <= 7200); // Max 2 hours
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: workflows
 // TEST_PRIORITY: normal

#[test]
fn test_workflow_empty_steps() {
    use types::WorkflowDefinition;
    
    let definition = WorkflowDefinition {
        id: "empty_workflow".to_string(),
        name: "Empty Workflow".to_string(),
        description: "A workflow with no steps".to_string(),
        steps: vec![],
        version: "1.0.0".to_string(),
    };
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    assert!(definition.steps.is_empty());
}

#[test]
fn test_workflow_multiple_executions() {
    use types::{WorkflowExecution, WorkflowStatus};
    
    let executions = vec![
        WorkflowExecution {
            id: "exec_001".to_string(),
            workflow_id: "workflow_001".to_string(),
            status: WorkflowStatus::Completed,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: workflows
            // TEST_PRIORITY: normal
            current_step: 3,
            started_at: Some(chrono::Utc::now()),
            completed_at: Some(chrono::Utc::now()),
            error: None,
        },
        WorkflowExecution {
            id: "exec_002".to_string(),
            workflow_id: "workflow_001".to_string(),
            status: WorkflowStatus::Running,
            current_step: 1,
            started_at: Some(chrono::Utc::now()),
            completed_at: None,
            error: None,
        },
    ];
    
    assert_eq!(executions.len(), 2);
    assert_eq!(executions[0].status, WorkflowStatus::Completed);
    assert_eq!(executions[1].status, WorkflowStatus::Running);
}

#[test]
fn test_workflow_status_serialization() {
    use types::WorkflowStatus;
    
    let statuses = vec![
        WorkflowStatus::Pending,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        WorkflowStatus::Running,
        WorkflowStatus::Completed,
        WorkflowStatus::Failed,
    ];
    
    for status in statuses {
        let serialized = serde_json::to_string(&status).expect("Should serialize");
        assert!(!serialized.is_empty());
        
        let deserialized: WorkflowStatus = serde_json::from_str(&serialized)
            .expect("Should deserialize");
        assert_eq!(status, deserialized);
    }
}

#[test]
fn test_workflow_step_serialization() {
    use types::WorkflowStep;
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: workflows
     // TEST_PRIORITY: normal
    
    let step = WorkflowStep {
        name: "serialize_test".to_string(),
        action: "test_action".to_string(),
        parameters: std::collections::HashMap::new(),
        timeout_seconds: 90,
        retry_on_failure: true,
    };
    
    let serialized = serde_json::to_string(&step).expect("Should serialize");
    let deserialized: WorkflowStep = serde_json::from_str(&serialized)
        .expect("Should deserialize");
    
    assert_eq!(step.name, deserialized.name);
    assert_eq!(step.action, deserialized.action);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal
#[test]
fn test_workflow_config_clone() {
    let config1 = WorkflowConfig {
        max_concurrent_workflows: 15,
        default_timeout_seconds: 450,
        retry_attempts: 4,
        enable_audit_logging: false,
        workflow_storage_path: "/clone/test".to_string(),
    };
    
    let config2 = config1.clone();
    
    assert_eq!(config1.max_concurrent_workflows, config2.max_concurrent_workflows);
    assert_eq!(config1.default_timeout_seconds, config2.default_timeout_seconds);
    assert_eq!(config1.retry_attempts, config2.retry_attempts);
}
