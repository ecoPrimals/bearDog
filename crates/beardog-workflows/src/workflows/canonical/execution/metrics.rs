

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::engine::WorkflowExecutionStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {

    pub processing_time_ms: u64,

    pub memory_usage_bytes: u64,

    pub cpu_usage_percent: f64,

    pub steps_executed: u32,

    pub error_count: u32,

    pub custom_metrics: HashMap<String, f64>,
}

impl Default for WorkflowMetrics {
    fn default() -> Self {
        Self {
            processing_time_ms: 0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            steps_executed: 0,
            error_count: 0,
            custom_metrics: HashMap::with_capacity(16),
        }
    }
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

    pub actions_taken: Vec<String>,

    pub processing_time_ms: u64,

    pub warnings: Vec<String>,

    pub metrics: Option<WorkflowMetrics>,

    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowProcessingResult {
    fn default() -> Self {
        Self {
            success: false,
            message: String::with_capacity(64),
            duration_ms: 0,
            workflow_id: String::with_capacity(64),
            processor_name: String::with_capacity(64),
            status: WorkflowExecutionStatus::NotStarted,
            execution_duration_ms: None,
            actions_taken: Vec::new(),
            processing_time_ms: 0,
            warnings: Vec::new(),
            metrics: None,
            metadata: HashMap::with_capacity(16),
        }
    }
}

pub struct WorkflowProcessingResultBuilder {
    success: bool,
    message: Option<String>,
    duration_ms: u64,
    workflow_id: Option<String>,
    processor_name: Option<String>,
    status: Option<WorkflowExecutionStatus>,
    actions_taken: Vec<String>,
    warnings: Vec<String>,
    metrics: Option<WorkflowMetrics>,
    metadata: HashMap<String, serde_json::Value>,
}

impl WorkflowProcessingResultBuilder {

    pub fn new() -> Self {
        Self {
            success: false,
            message: None,
            duration_ms: 0,
            workflow_id: None,
            processor_name: None,
            status: None,
            actions_taken: Vec::new(),
            warnings: Vec::new(),
            metrics: None,
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn message<'a>(mut self, message: impl Into<&'a str>) -> Self {
        self.message = Some(message.into().to_string());
        self
    }

    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set the workflow ID for this metrics entry
    pub fn workflow_id<'a>(mut self, workflow_id: impl Into<&'a str>) -> Self {
        self.workflow_id = Some(workflow_id.into().to_string());
        self
    }

    /// Set the processor name for this metrics entry
    pub fn processor_name<'a>(mut self, processor_name: impl Into<&'a str>) -> Self {
        self.processor_name = Some(processor_name.into().to_string());
        self
    }

    pub fn status(mut self, status: WorkflowExecutionStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Add an action to the execution metrics
    pub fn add_action<'a>(mut self, action: impl Into<&'a str>) -> Self {
        self.actions_taken.push(action.into().to_string());
        self
    }

    /// Add a warning to the execution metrics
    pub fn add_warning<'a>(mut self, warning: impl Into<&'a str>) -> Self {
        self.warnings.push(warning.into().to_string());
        self
    }

    pub fn metrics(mut self, metrics: WorkflowMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Add metadata to the execution metrics
    pub fn add_metadata<'a>(mut self, key: impl Into<&'a str>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into().to_string(), value);
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
            actions_taken: self.actions_taken,
            processing_time_ms: self.duration_ms,
            warnings: self.warnings,
            metrics: self.metrics,
            metadata: self.metadata,
        }
    }
}

impl Default for WorkflowProcessingResultBuilder {
    fn default() -> Self {
        Self::new()
    }
} 