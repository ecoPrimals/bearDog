// Canonical Workflow Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical workflow engine configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalWorkflowConfig {
    /// Whether workflow engine is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Maximum number of concurrent workflows
    /// Number of `max_concurrent_workflows`
    pub max_concurrent_workflows: u32,
    /// Default timeout duration for workflow execution
    pub default_timeout: Duration,
    /// Whether workflow retry is enabled
    /// Whether retry is enabled
    pub retry_enabled: bool,
    /// Maximum number of workflow retries
    /// Number of `max_retries`
    pub max_retries: u32,
}

/// Type alias for canonical workflow configuration
pub type WorkflowConfig = CanonicalWorkflowConfig;
