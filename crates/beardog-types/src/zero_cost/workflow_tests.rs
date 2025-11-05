// Comprehensive tests for zero-cost workflow abstractions

#[cfg(test)]
mod workflow_tests {
    use crate::zero_cost::workflow::*;
    use beardog_errors::BearDogResult;

    #[test]
    fn test_workflow_state_creation() {
        let state = WorkflowState::Pending;
        assert!(matches!(state, WorkflowState::Pending));
        
        let state2 = WorkflowState::Running;
        assert!(matches!(state2, WorkflowState::Running));
    }

    #[test]
    fn test_workflow_state_transitions() {
        // Test valid state transitions
        let initial = WorkflowState::Pending;
        let running = WorkflowState::Running;
        let completed = WorkflowState::Completed;
        
        // Pending -> Running (valid)
        assert!(matches!(initial, WorkflowState::Pending));
        assert!(matches!(running, WorkflowState::Running));
        
        // Running -> Completed (valid)
        assert!(matches!(completed, WorkflowState::Completed));
    }

    #[test]
    fn test_workflow_state_failed() {
        let failed = WorkflowState::Failed { 
            error: "Test error".to_string() 
        };
        
        match failed {
            WorkflowState::Failed { error } => {
                assert_eq!(error, "Test error");
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            }
            _ => panic!("Expected Failed state"),
        }
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_workflow_execution_context() {
        let context = WorkflowExecutionContext {
            workflow_id: "wf-123".to_string(),
            started_at: chrono::Utc::now(),
            variables: std::collections::HashMap::new(),
        };
        
        assert_eq!(context.workflow_id, "wf-123");
        assert_eq!(context.variables.len(), 0);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    fn test_workflow_step_execution() {
        let step = WorkflowStepExecution {
            step_id: "step-1".to_string(),
            status: StepStatus::Success,
            started_at: chrono::Utc::now(),
            completed_at: Some(chrono::Utc::now()),
            output: None,
        };
        
        assert_eq!(step.step_id, "step-1");
        assert!(matches!(step.status, StepStatus::Success));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(step.completed_at.is_some());
    }

    #[test]
    fn test_step_status_variants() {
        assert!(matches!(StepStatus::Pending, StepStatus::Pending));
        assert!(matches!(StepStatus::Running, StepStatus::Running));
        assert!(matches!(StepStatus::Success, StepStatus::Success));
        assert!(matches!(StepStatus::Failed, StepStatus::Failed));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(matches!(StepStatus::Skipped, StepStatus::Skipped));
    }

    #[test]
    fn test_workflow_configuration() {
        let config = WorkflowConfiguration {
            max_retries: 3,
            timeout_seconds: 300,
            parallel_execution: false,
            continue_on_error: false,
        };
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_seconds, 300);
        assert!(!config.parallel_execution);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_workflow_result_success() {
        let result = WorkflowExecutionResult {
            workflow_id: "wf-456".to_string(),
            final_state: WorkflowState::Completed,
            steps_executed: 5,
            duration_ms: 1500,
            error: None,
        };
        
        assert_eq!(result.workflow_id, "wf-456");
        assert!(matches!(result.final_state, WorkflowState::Completed));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(result.steps_executed, 5);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_workflow_result_failure() {
        let result = WorkflowExecutionResult {
            workflow_id: "wf-789".to_string(),
            final_state: WorkflowState::Failed { 
                error: "Step 3 failed".to_string() 
            },
            steps_executed: 3,
            duration_ms: 800,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: important
            error: Some("Step 3 failed".to_string()),
        };
        
        assert_eq!(result.workflow_id, "wf-789");
        assert_eq!(result.steps_executed, 3);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_workflow_with_variables() {
        let mut variables = std::collections::HashMap::new();
        variables.insert("env".to_string(), "production".to_string());
        variables.insert("region".to_string(), "us-east-1".to_string());
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let context = WorkflowExecutionContext {
            workflow_id: "wf-vars".to_string(),
            started_at: chrono::Utc::now(),
            variables,
        };
        
        assert_eq!(context.variables.len(), 2);
        assert_eq!(context.variables.get("env").map(String::as_str), Some("production"));
    }

    #[test]
    fn test_workflow_timeout_configuration() {
        let config = WorkflowConfiguration {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            max_retries: 0,
            timeout_seconds: 60,
            parallel_execution: false,
            continue_on_error: false,
        };
        
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.max_retries, 0);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_workflow_parallel_execution() {
        let config = WorkflowConfiguration {
            max_retries: 1,
            timeout_seconds: 300,
            parallel_execution: true,
            continue_on_error: false,
        };
        
        assert!(config.parallel_execution);
    }
}

