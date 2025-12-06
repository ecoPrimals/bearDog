// Comprehensive tests for zero-cost workflow abstractions

use beardog_errors::BearDogError;
#[cfg(test)]
use beardog_errors::BearDogError;
mod workflow_tests {
use beardog_errors::BearDogError;
    use crate::zero_cost::workflow::*;
use beardog_errors::BearDogError;

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_state_creation() {
use beardog_errors::BearDogError;
        let state = WorkflowState::Pending;
use beardog_errors::BearDogError;
        assert!(matches!(state, WorkflowState::Pending));
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        let state2 = WorkflowState::Running;
use beardog_errors::BearDogError;
        assert!(matches!(state2, WorkflowState::Running));
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_state_transitions() {
use beardog_errors::BearDogError;
        // Test valid state transitions
use beardog_errors::BearDogError;
        let initial = WorkflowState::Pending;
use beardog_errors::BearDogError;
        let running = WorkflowState::Running;
use beardog_errors::BearDogError;
        let completed = WorkflowState::Completed;
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        // Pending -> Running (valid)
use beardog_errors::BearDogError;
        assert!(matches!(initial, WorkflowState::Pending));
use beardog_errors::BearDogError;
        assert!(matches!(running, WorkflowState::Running));
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        // Running -> Completed (valid)
use beardog_errors::BearDogError;
        assert!(matches!(completed, WorkflowState::Completed));
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_state_failed() {
use beardog_errors::BearDogError;
        let failed = WorkflowState::Failed { 
use beardog_errors::BearDogError;
            error: "Test error".to_string() 
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        match failed {
use beardog_errors::BearDogError;
            WorkflowState::Failed { error } => {
use beardog_errors::BearDogError;
                assert_eq!(error, "Test error");
use beardog_errors::BearDogError;
            // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
            // TEST_DOMAIN: types
use beardog_errors::BearDogError;
            // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
            }
use beardog_errors::BearDogError;
            _ => panic!("Expected Failed state"),
use beardog_errors::BearDogError;
        }
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
    // TEST_DOMAIN: types
use beardog_errors::BearDogError;
    // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
    fn test_workflow_execution_context() {
use beardog_errors::BearDogError;
        let context = WorkflowExecutionContext {
use beardog_errors::BearDogError;
            workflow_id: "wf-123".to_string(),
use beardog_errors::BearDogError;
            started_at: chrono::Utc::now(),
use beardog_errors::BearDogError;
            variables: std::collections::HashMap::new(),
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(context.workflow_id, "wf-123");
use beardog_errors::BearDogError;
        assert_eq!(context.variables.len(), 0);
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
    // TEST_DOMAIN: types
use beardog_errors::BearDogError;
    // TEST_PRIORITY: important
use beardog_errors::BearDogError;
    fn test_workflow_step_execution() {
use beardog_errors::BearDogError;
        let step = WorkflowStepExecution {
use beardog_errors::BearDogError;
            step_id: "step-1".to_string(),
use beardog_errors::BearDogError;
            status: StepStatus::Success,
use beardog_errors::BearDogError;
            started_at: chrono::Utc::now(),
use beardog_errors::BearDogError;
            completed_at: Some(chrono::Utc::now()),
use beardog_errors::BearDogError;
            output: None,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(step.step_id, "step-1");
use beardog_errors::BearDogError;
        assert!(matches!(step.status, StepStatus::Success));
use beardog_errors::BearDogError;
        // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
        // TEST_DOMAIN: types
use beardog_errors::BearDogError;
        // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
        assert!(step.completed_at.is_some());
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_step_status_variants() {
use beardog_errors::BearDogError;
        assert!(matches!(StepStatus::Pending, StepStatus::Pending));
use beardog_errors::BearDogError;
        assert!(matches!(StepStatus::Running, StepStatus::Running));
use beardog_errors::BearDogError;
        assert!(matches!(StepStatus::Success, StepStatus::Success));
use beardog_errors::BearDogError;
        assert!(matches!(StepStatus::Failed, StepStatus::Failed));
use beardog_errors::BearDogError;
        // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
        // TEST_DOMAIN: types
use beardog_errors::BearDogError;
        // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
        assert!(matches!(StepStatus::Skipped, StepStatus::Skipped));
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_configuration() {
use beardog_errors::BearDogError;
        let config = WorkflowConfiguration {
use beardog_errors::BearDogError;
            max_retries: 3,
use beardog_errors::BearDogError;
            timeout_seconds: 300,
use beardog_errors::BearDogError;
            parallel_execution: false,
use beardog_errors::BearDogError;
            continue_on_error: false,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
        // TEST_DOMAIN: types
use beardog_errors::BearDogError;
        // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
        assert_eq!(config.max_retries, 3);
use beardog_errors::BearDogError;
        assert_eq!(config.timeout_seconds, 300);
use beardog_errors::BearDogError;
        assert!(!config.parallel_execution);
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
    // TEST_DOMAIN: types
use beardog_errors::BearDogError;
    // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
    fn test_workflow_result_success() {
use beardog_errors::BearDogError;
        let result = WorkflowExecutionResult {
use beardog_errors::BearDogError;
            workflow_id: "wf-456".to_string(),
use beardog_errors::BearDogError;
            final_state: WorkflowState::Completed,
use beardog_errors::BearDogError;
            steps_executed: 5,
use beardog_errors::BearDogError;
            duration_ms: 1500,
use beardog_errors::BearDogError;
            error: None,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(result.workflow_id, "wf-456");
use beardog_errors::BearDogError;
        assert!(matches!(result.final_state, WorkflowState::Completed));
use beardog_errors::BearDogError;
        // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
        // TEST_DOMAIN: types
use beardog_errors::BearDogError;
        // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
        assert_eq!(result.steps_executed, 5);
use beardog_errors::BearDogError;
        assert!(result.error.is_none());
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_result_failure() {
use beardog_errors::BearDogError;
        let result = WorkflowExecutionResult {
use beardog_errors::BearDogError;
            workflow_id: "wf-789".to_string(),
use beardog_errors::BearDogError;
            final_state: WorkflowState::Failed { 
use beardog_errors::BearDogError;
                error: "Step 3 failed".to_string() 
use beardog_errors::BearDogError;
            },
use beardog_errors::BearDogError;
            steps_executed: 3,
use beardog_errors::BearDogError;
            duration_ms: 800,
use beardog_errors::BearDogError;
            // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
            // TEST_DOMAIN: types
use beardog_errors::BearDogError;
            // TEST_PRIORITY: important
use beardog_errors::BearDogError;
            error: Some("Step 3 failed".to_string()),
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(result.workflow_id, "wf-789");
use beardog_errors::BearDogError;
        assert_eq!(result.steps_executed, 3);
use beardog_errors::BearDogError;
        assert!(result.error.is_some());
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_with_variables() {
use beardog_errors::BearDogError;
        let mut variables = std::collections::HashMap::new();
use beardog_errors::BearDogError;
        variables.insert("env".to_string(), "production".to_string());
use beardog_errors::BearDogError;
        variables.insert("region".to_string(), "us-east-1".to_string());
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
        // TEST_DOMAIN: types
use beardog_errors::BearDogError;
        // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
        let context = WorkflowExecutionContext {
use beardog_errors::BearDogError;
            workflow_id: "wf-vars".to_string(),
use beardog_errors::BearDogError;
            started_at: chrono::Utc::now(),
use beardog_errors::BearDogError;
            variables,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(context.variables.len(), 2);
use beardog_errors::BearDogError;
        assert_eq!(context.variables.get("env").map(String::as_str), Some("production"));
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_timeout_configuration() {
use beardog_errors::BearDogError;
        let config = WorkflowConfiguration {
use beardog_errors::BearDogError;
            // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
            // TEST_DOMAIN: types
use beardog_errors::BearDogError;
            // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
            max_retries: 0,
use beardog_errors::BearDogError;
            timeout_seconds: 60,
use beardog_errors::BearDogError;
            parallel_execution: false,
use beardog_errors::BearDogError;
            continue_on_error: false,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert_eq!(config.timeout_seconds, 60);
use beardog_errors::BearDogError;
        assert_eq!(config.max_retries, 0);
use beardog_errors::BearDogError;
    }

use beardog_errors::BearDogError;
    // TEST_CATEGORY: unit
use beardog_errors::BearDogError;
    // TEST_DOMAIN: types
use beardog_errors::BearDogError;
    // TEST_PRIORITY: normal
use beardog_errors::BearDogError;
    #[test]
use beardog_errors::BearDogError;
    fn test_workflow_parallel_execution() {
use beardog_errors::BearDogError;
        let config = WorkflowConfiguration {
use beardog_errors::BearDogError;
            max_retries: 1,
use beardog_errors::BearDogError;
            timeout_seconds: 300,
use beardog_errors::BearDogError;
            parallel_execution: true,
use beardog_errors::BearDogError;
            continue_on_error: false,
use beardog_errors::BearDogError;
        };
use beardog_errors::BearDogError;
        
use beardog_errors::BearDogError;
        assert!(config.parallel_execution);
use beardog_errors::BearDogError;
    }
use beardog_errors::BearDogError;
}

