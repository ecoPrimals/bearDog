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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_config_default() {
        let config = CanonicalWorkflowConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.max_concurrent_workflows, 0);
        assert!(!config.retry_enabled);
        assert_eq!(config.max_retries, 0);
    }

    #[test]
    fn test_workflow_config_enabled() {
        let config = CanonicalWorkflowConfig {
            enabled: true,
            max_concurrent_workflows: 100,
            default_timeout: Duration::from_secs(300),
            retry_enabled: true,
            max_retries: 3,
        };
        assert!(config.enabled);
        assert_eq!(config.max_concurrent_workflows, 100);
        assert!(config.retry_enabled);
    }

    #[test]
    fn test_workflow_config_no_retry() {
        let config = CanonicalWorkflowConfig {
            enabled: true,
            retry_enabled: false,
            ..Default::default()
        };
        assert!(!config.retry_enabled);
        assert_eq!(config.max_retries, 0);
    }

    #[test]
    fn test_workflow_config_high_concurrency() {
        let config = CanonicalWorkflowConfig {
            max_concurrent_workflows: 1000,
            ..Default::default()
        };
        assert_eq!(config.max_concurrent_workflows, 1000);
    }

    #[test]
    fn test_workflow_timeout_short() {
        let config = CanonicalWorkflowConfig {
            default_timeout: Duration::from_secs(30),
            ..Default::default()
        };
        assert_eq!(config.default_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_workflow_timeout_long() {
        let config = CanonicalWorkflowConfig {
            default_timeout: Duration::from_secs(3600),
            ..Default::default()
        };
        assert_eq!(config.default_timeout, Duration::from_secs(3600));
    }

    #[test]
    fn test_workflow_max_retries() {
        let config = CanonicalWorkflowConfig {
            retry_enabled: true,
            max_retries: 5,
            ..Default::default()
        };
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_type_alias() {
        let _config: WorkflowConfig = CanonicalWorkflowConfig::default();
    }
}
