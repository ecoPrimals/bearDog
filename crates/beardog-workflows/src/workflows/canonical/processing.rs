

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::core::types::CanonicalWorkflow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowExecutionStatus {

    NotStarted,

    Pending,

    Running,

    Completed,

    Failed,

    Cancelled,

    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {

    pub processing_time_ms: u64,

    pub memory_usage_bytes: u64,

    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingResult {

    pub success: bool,

    pub message: String,

    pub duration_ms: u64,

    pub workflow_id: String,

    pub processor_name: String,

    pub status: WorkflowExecutionStatus,

    pub execution_duration_ms: Option<u64>,

    pub steps_completed: Option<u32>,

    pub steps_total: Option<u32>,

    pub output_data: Option<serde_json::Value>,

    pub actions_taken: Vec<String>,

    pub metrics: WorkflowMetrics,

    pub warnings: Vec<String>,
}

impl WorkflowProcessingResult {

    pub fn success(
        workflow_id: &str,
        processor_name: &str,
        message: &str,
        duration_ms: u64,
    ) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            duration_ms,
            workflow_id: workflow_id.to_string(),
            processor_name: processor_name.to_string(),
            status: WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(duration_ms),
            steps_completed: Some(1),
            steps_total: Some(1),
            output_data: None,
            actions_taken: vec![],
            metrics: WorkflowMetrics {
                processing_time_ms: duration_ms,
                memory_usage_bytes: 1024,
                cpu_usage_percent: 0.1,
            },
            warnings: vec![],
        }
    }

    pub fn failure(
        workflow_id: &str,
        processor_name: &str,
        error_message: &str,
        duration_ms: u64,
    ) -> Self {
        Self {
            success: false,
            message: error_message.to_string(),
            duration_ms,
            workflow_id: workflow_id.to_string(),
            processor_name: processor_name.to_string(),
            status: WorkflowExecutionStatus::Failed,
            execution_duration_ms: Some(duration_ms),
            steps_completed: Some(0),
            steps_total: Some(1),
            output_data: None,
            actions_taken: vec![],
            metrics: WorkflowMetrics {
                processing_time_ms: duration_ms,
                memory_usage_bytes: 1024,
                cpu_usage_percent: 0.1,
            },
            warnings: vec![],
        }
    }

    pub fn builder() -> WorkflowProcessingResultBuilder {
        WorkflowProcessingResultBuilder::default()
    }
}

#[derive(Default)]
pub struct WorkflowProcessingResultBuilder {
    success: bool,
    message: Option<String>,
    duration_ms: u64,
    workflow_id: Option<String>,
    processor_name: Option<String>,
    status: Option<WorkflowExecutionStatus>,
        output_data: Option<serde_json::Value>,
    actions_taken: Vec<String>,
    warnings: Vec<String>,
}

impl WorkflowProcessingResultBuilder {
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn message<'a>(mut self, message: impl Into<&'a str>) -> Self {
        self.message = Some(message.into().to_string());
        self
    }

    /// Set the workflow ID for this processing entry
    pub fn workflow_id<'a>(mut self, workflow_id: impl Into<&'a str>) -> Self {
        self.workflow_id = Some(workflow_id.into().to_string());
        self
    }

    /// Set the processor name for this processing entry
    pub fn processor_name<'a>(mut self, processor_name: impl Into<&'a str>) -> Self {
        self.processor_name = Some(processor_name.into().to_string());
        self
    }

    pub fn build(self) -> WorkflowProcessingResult {
        WorkflowProcessingResult {
            success: self.success,
            message: self.message.unwrap_or_default(),
            duration_ms: self.duration_ms,
            workflow_id: self.workflow_id.unwrap_or_default(),
            processor_name: self.processor_name.unwrap_or_default(),
            status: self.status.unwrap_or(WorkflowExecutionStatus::Completed),
            execution_duration_ms: Some(self.duration_ms),
            steps_completed: Some(1),
            steps_total: Some(1),
            output_data: None,
            actions_taken: self.actions_taken,
            metrics: WorkflowMetrics {
                processing_time_ms: self.duration_ms,
                memory_usage_bytes: 1024,
                cpu_usage_percent: 0.1,
            },
            warnings: self.warnings,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {

    pub execution_id: String,

    pub workflow: CanonicalWorkflow,

    pub processor: String,

    pub state: WorkflowExecutionStatus,

    pub result: Option<WorkflowProcessingResult>,

    pub started_at: DateTime<Utc>,

    pub completed_at: Option<DateTime<Utc>>,
}

