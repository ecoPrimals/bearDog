// SPDX-License-Identifier: AGPL-3.0-only

// Workflow configuration types for BearDog
// Provides workflow definition, execution, and management types

use crate::canonical::traits::RetryStrategy;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    /// Name of the item
    pub name: String,
    /// Workflow description
    /// The description value
    pub description: String,
    /// Workflow version
    /// The version value
    pub version: String,
    /// Workflow steps
    /// Collection of steps
    pub steps: Vec<WorkflowStep>,
    /// Workflow metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Workflow tags
    /// Collection of tags
    pub tags: Vec<String>,
}

impl Default for WorkflowDefinition {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Workflow".to_string(),
            description: "Default workflow definition".to_string(),
            version: "1.0.0".to_string(),
            steps: Vec::new(),
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }
}

/// Workflow step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step ID
    pub id: String,
    /// Step name
    /// Name of the item
    pub name: String,
    /// Step type
    /// The step type value
    pub step_type: WorkflowStepType,
    /// Step configuration
    pub config: HashMap<String, String>,
    /// Dependencies on other steps
    /// Collection of dependencies
    pub dependencies: Vec<String>,
    /// Step timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
}

/// Workflow step types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of workflow step
pub enum WorkflowStepType {
    /// Action step
    Action,
    /// Decision step
    Decision,
    /// Parallel execution
    Parallel,
    /// Sequential execution
    Sequential,
    /// Custom step type
    Custom { name: String },
    Custom { name: String },
    Custom { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    /// Number of max_attempts
    pub max_attempts: u32,
    /// Delay between retries in seconds
    /// Number of delay_seconds
    pub delay_seconds: u64,
    /// Backoff multiplier
    /// The backoff multiplier value
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            delay_seconds: 1,
            backoff_multiplier: 2.0,
        }
    }
}

// Implement RetryStrategy trait for workflow retry configuration
impl RetryStrategy for RetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Exponential backoff based on delay_seconds and multiplier
        let delay_secs = self.delay_seconds as f64 
            * self.backoff_multiplier.powi(attempt as i32);
        Duration::from_secs(delay_secs as u64)
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Workflow: retry on most errors (can be refined based on error type)
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        attempts >= self.max_attempts
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

/// Workflow status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently running
    Running,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed
    Failed,
    /// Workflow was cancelled
    Cancelled,
    /// Workflow is paused
    Paused,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Workflow execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionState {
    /// Total number of steps
    /// Number of total_steps
    pub total_steps: usize,
    /// Number of completed steps
    /// Number of completed_steps
    pub completed_steps: usize,
    /// Failed step indices
    /// Collection of failed steps
    pub failed_steps: Vec<usize>,
    /// Current workflow status
    /// Current status of the component
    pub status: WorkflowStatus,
    /// Error message if failed
    /// Optional error message
    pub error_message: Option<String>,
}

impl Default for WorkflowExecutionState {
    fn default() -> Self {
        Self {
            total_steps: 0,
            completed_steps: 0,
            failed_steps: Vec::new(),
            status: WorkflowStatus::Pending,
            error_message: None,
        }
    }
}

/// Workflow execution instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Execution ID
    pub id: String,
    /// Workflow definition
    /// The workflow value
    pub workflow: WorkflowDefinition,
    /// Execution state
    /// The state value
    pub state: WorkflowExecutionState,
    /// Execution start time
    /// Optional started at
    pub started_at: Option<DateTime<Utc>>,
    /// Execution end time
    /// Optional completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Execution context
    /// Mapping of context
    pub context: HashMap<String, String>,
    /// Execution logs
    /// Collection of logs
    pub logs: Vec<String>,
}

impl Default for WorkflowExecution {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            workflow: WorkflowDefinition::default(),
            state: WorkflowExecutionState::default(),
            started_at: None,
            completed_at: None,
            context: HashMap::new(),
            logs: Vec::new(),
        }
    }
}

/// Workflow engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEngineConfig {
    /// Maximum concurrent executions
    /// Number of max_concurrent_executions
    pub max_concurrent_executions: u32,
    /// Default step timeout in seconds
    pub default_step_timeout_seconds: u64,
    /// Execution history retention in days
    /// Number of history_retention_days
    pub history_retention_days: u32,
    /// Enable execution logging
    /// Whether logging is enabled
    pub logging_enabled: bool,
    /// Metrics collection enabled
    /// Whether metrics is enabled
    pub metrics_enabled: bool,
}

impl Default for WorkflowEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_executions: 100,
            default_step_timeout_seconds: 300,
            history_retention_days: 30,
            logging_enabled: true,
            metrics_enabled: true,
        }
    }
}

impl WorkflowDefinition {
    /// Create a new workflow definition
    /// Creates a new instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Add a step to the workflow
    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    /// Get a step by ID
    /// Gets step
    /// Gets step
    pub fn get_step(&self, step_id: &str) -> Option<&WorkflowStep> {
        self.steps.iter().find(|step| step.id == step_id)
    }

    /// Validate the workflow definition
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Workflow name cannot be empty".to_string());
        }

        if self.steps.is_empty() {
            return Err("Workflow must have at least one step".to_string());
        }

        // Check for duplicate step IDs
        let mut step_ids = std::collections::HashSet::new();
        for step in &self.steps {
            if !step_ids.insert(&step.id) {
                return Err(format!("Duplicate step ID: {}", step.id));
            }
        }

        // Validate step dependencies
        for step in &self.steps {
            for dep_id in &step.dependencies {
                if !step_ids.contains(dep_id) {
                    return Err(format!(
                        "Step "{}" depends on non-existent step "{}"",
                        step.id, dep_id
                    ));
                }
            }
        }

        Ok(())
    }

    /// Checks if circular dependencies
    /// Checks if circular dependencies
    pub fn has_circular_dependencies(&self) -> bool {
        // Simple cycle detection using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for step in &self.steps {
            if !visited.contains(&step.id)
                && self.has_cycle_util(&step.id, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }
        false
    }


    fn has_cycle_util(
        &self,
        step_id: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        visited.insert(step_id.to_string());
        rec_stack.insert(step_id.to_string());

        if let Some(step) = self.get_step(step_id) {
            for dep_id in &step.dependencies {
                if !visited.contains(dep_id) {
                    if self.has_cycle_util(dep_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep_id) {
                    return true;
                }
            }
        }

        rec_stack.remove(step_id);
        false
    }
}

impl WorkflowExecution {
    /// Create a new workflow execution
    /// Creates a new instance
    pub fn new(workflow: WorkflowDefinition) -> Self {
        let total_steps = workflow.steps.len();
        Self {
            workflow,
            state: WorkflowExecutionState {
                total_steps,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Start the workflow execution
    /// Starts service
    /// Starts service
    pub fn start(&mut self) {
        self.started_at = Some(Utc::now());
        self.state.status = WorkflowStatus::Running;
        self.add_log("Workflow execution started".to_string());
    }

    /// Complete the workflow execution
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.state.status = WorkflowStatus::Completed;
        self.add_log("Workflow execution completed".to_string());
    }

    /// Fail the workflow execution
    pub fn fail(&mut self, error_message: String) {
        self.completed_at = Some(Utc::now());
        self.state.status = WorkflowStatus::Failed;
        self.state.error_message = Some(error_message.clone());
        self.add_log(format!("Workflow execution failed: {error_message}"));
    }

    /// Add a log entry
    pub fn add_log(&mut self, message: String) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        self.logs.push(format!("[{timestamp}] {message}"));
    }

    /// Get execution progress as percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.state.total_steps == 0 {
            return 0.0;
        }
        (self.state.completed_steps as f64 / self.state.total_steps as f64) * 100.0
    }

    /// Check if execution is complete
    /// Checks if complete
    /// Checks if complete
    pub fn is_complete(&self) -> bool {
        matches!(
            self.state.status,
            WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled
        )
    }
}
