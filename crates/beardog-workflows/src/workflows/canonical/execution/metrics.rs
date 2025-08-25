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


/// # Canonical Workflow Metrics
///
/// **UNIFIED METRICS SYSTEM** for the BearDog workflow system
/// This module provides workflow performance metrics and processing results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::engine::WorkflowExecutionStatus;

/// **CANONICAL** Workflow processing metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Number of steps executed
    pub steps_executed: u32,
    /// Number of errors encountered
    pub error_count: u32,
    /// Additional custom metrics
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
            custom_metrics: HashMap::new(),
        }
    }
}

/// **CANONICAL** Workflow processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingResult {
    /// Success status
    pub success: bool,
    /// Result message
    pub message: String,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
    /// Workflow ID that was processed
    pub workflow_id: String,
    /// Name of the processor that handled the workflow
    pub processor_name: String,
    /// Final execution status
    pub status: WorkflowExecutionStatus,
    /// Execution duration in milliseconds
    pub execution_duration_ms: Option<u64>,
    /// Actions taken during processing
    pub actions_taken: Vec<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Warnings generated during processing
    pub warnings: Vec<String>,
    /// Processing metrics
    pub metrics: Option<WorkflowMetrics>,
    /// Additional result metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowProcessingResult {
    fn default() -> Self {
        Self {
            success: false,
            message: String::new(),
            duration_ms: 0,
            workflow_id: String::new(),
            processor_name: String::new(),
            status: WorkflowExecutionStatus::NotStarted,
            execution_duration_ms: None,
            actions_taken: Vec::new(),
            processing_time_ms: 0,
            warnings: Vec::new(),
            metrics: None,
            metadata: HashMap::new(),
        }
    }
}

/// **CANONICAL** Builder for WorkflowProcessingResult
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
    /// Create a new builder
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
            metadata: HashMap::new(),
        }
    }

    /// Set success status
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Set result message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set processing duration
    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set workflow ID
    pub fn workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    /// Set processor name
    pub fn processor_name(mut self, processor_name: impl Into<String>) -> Self {
        self.processor_name = Some(processor_name.into());
        self
    }

    /// Set execution status
    pub fn status(mut self, status: WorkflowExecutionStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Add an action taken
    pub fn add_action(mut self, action: impl Into<String>) -> Self {
        self.actions_taken.push(action.into());
        self
    }

    /// Add a warning
    pub fn add_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Set metrics
    pub fn metrics(mut self, metrics: WorkflowMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Build the result
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