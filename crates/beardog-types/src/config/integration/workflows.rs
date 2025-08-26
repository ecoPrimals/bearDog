

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationWorkflowsConfig {
    pub enabled: bool,
    pub workflow_timeout: Duration,
    pub max_concurrent_workflows: u32,
    pub approval: WorkflowApprovalConfig,
}

impl Default for IntegrationWorkflowsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workflow_timeout: Duration::from_secs(3600),
            max_concurrent_workflows: 10,
            approval: WorkflowApprovalConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowApprovalConfig {
    pub required: bool,
    pub approval_timeout: Duration,
    pub auto_approve_threshold: u32,
}

impl Default for WorkflowApprovalConfig {
    fn default() -> Self {
        Self {
            required: false,
            approval_timeout: Duration::from_secs(86400), // 24 hours
            auto_approve_threshold: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionConfig {
    pub max_execution_time: Duration,
    pub retry_attempts: u32,
    pub parallel_execution: bool,
}

impl Default for WorkflowExecutionConfig {
    fn default() -> Self {
        Self {
            max_execution_time: Duration::from_secs(1800), // 30 minutes
            retry_attempts: 3,
            parallel_execution: true,
        }
    }
}
