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


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::core::types::CanonicalWorkflow;

/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowExecutionStatus {
    /// Not started
    NotStarted,
    /// Pending execution
    Pending,
    /// Running
    Running,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled
    Cancelled,
    /// Timeout
    Timeout,
}

/// Workflow processing metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
}

/// Workflow processing result - unified canonical definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingResult {
    /// Processing success status
    pub success: bool,
    /// Processing message
    pub message: String,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
    /// Workflow ID
    pub workflow_id: String,
    /// Processor name
    pub processor_name: String,
    /// Processing status
    pub status: WorkflowExecutionStatus,
    /// Execution duration (alternative field for compatibility)
    pub execution_duration_ms: Option<u64>,
    /// Steps completed
    pub steps_completed: Option<u32>,
    /// Total steps
    pub steps_total: Option<u32>,
    /// Output data from the workflow
    pub output_data: Option<serde_json::Value>,
    /// List of actions taken
    pub actions_taken: Vec<String>,
    /// Processing metrics
    pub metrics: WorkflowMetrics,
    /// Warnings generated during processing
    pub warnings: Vec<String>,
}

impl WorkflowProcessingResult {
    /// Create a successful processing result with minimal required fields
    pub fn success(
        workflow_id: String,
        processor_name: String,
        message: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            success: true,
            message,
            duration_ms,
            workflow_id,
            processor_name,
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

    /// Create a failed processing result
    pub fn failure(
        workflow_id: String,
        processor_name: String,
        error_message: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            success: false,
            message: error_message,
            duration_ms,
            workflow_id,
            processor_name,
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

    /// Zero-copy builder pattern for performance-critical paths
    pub fn builder() -> WorkflowProcessingResultBuilder {
        WorkflowProcessingResultBuilder::default()
    }
}

/// Zero-copy builder for WorkflowProcessingResult
#[derive(Default)]
pub struct WorkflowProcessingResultBuilder {
    success: bool,
    message: Option<String>,
    duration_ms: u64,
    workflow_id: Option<String>,
    processor_name: Option<String>,
    status: Option<WorkflowExecutionStatus>,
    #[allow(dead_code)]
    output_data: Option<serde_json::Value>,
    actions_taken: Vec<String>,
    warnings: Vec<String>,
}

impl WorkflowProcessingResultBuilder {
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    pub fn processor_name(mut self, processor_name: impl Into<String>) -> Self {
        self.processor_name = Some(processor_name.into());
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

/// Workflow execution details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Execution ID
    pub execution_id: String,
    /// Workflow being executed
    pub workflow: CanonicalWorkflow,
    /// Processor handling the execution
    pub processor: String,
    /// Current execution state
    pub state: WorkflowExecutionStatus,
    /// Execution result
    pub result: Option<WorkflowProcessingResult>,
    /// Execution start time
    pub started_at: DateTime<Utc>,
    /// Execution completion time
    pub completed_at: Option<DateTime<Utc>>,
}

// This eliminates the duplicate trait definition and ensures single source of truth

// RE-EXPORT CANONICAL TRAIT - Provides backward compatibility during migration
// ✅ CANONICAL WORKFLOW PROCESSOR - Re-exported from the canonical location
//
// This re-export provides backward compatibility while encouraging migration
// to the canonical import path: `beardog_traits::canonical::WorkflowProcessor`