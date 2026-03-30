// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical Workflow Configuration
//!
//! Configuration for the BearDog workflow orchestration engine.
//! Controls workflow execution, concurrency, timeouts, and retry behavior.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Workflow engine configuration for `BearDog`
///
/// Controls the behavior of the workflow orchestration engine, including
/// execution limits, timeout policies, and retry strategies.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::workflow::CanonicalWorkflowConfig;
/// use std::time::Duration;
///
/// let config = CanonicalWorkflowConfig {
///     enabled: true,
///     max_concurrent_workflows: 100,
///     default_timeout: Duration::from_secs(300),
///     retry_enabled: true,
///     max_retries: 3,
/// };
/// ```
///
/// # Production Configuration
///
/// For production deployments, consider:
/// - Set `max_concurrent_workflows` based on available resources
/// - Configure `default_timeout` based on expected workflow duration
/// - Enable `retry_enabled` for resilience against transient failures
/// - Set `max_retries` to prevent infinite retry loops
///
/// # See Also
///
/// - `beardog_workflows` - Workflow execution engine
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalWorkflowConfig {
    /// Whether the workflow engine is enabled
    ///
    /// When disabled, no workflows will be processed. Useful for
    /// maintenance or gradual rollout of workflow features.
    pub enabled: bool,

    /// Maximum number of concurrent workflows
    ///
    /// Limits the number of workflows that can execute simultaneously.
    /// Helps prevent resource exhaustion. Set to 0 for unlimited (not recommended).
    ///
    /// # Recommendations
    /// - Development: 10-50
    /// - Production: 100-1000 based on available resources
    pub max_concurrent_workflows: u32,

    /// Default timeout duration for workflow execution
    ///
    /// Workflows exceeding this duration will be terminated.
    /// Individual workflows can override this with their own timeout.
    ///
    /// # Recommendations
    /// - Short workflows: 30-60 seconds
    /// - Standard workflows: 5-10 minutes
    /// - Long-running workflows: 30-60 minutes
    pub default_timeout: Duration,

    /// Whether automatic workflow retry is enabled
    ///
    /// When enabled, failed workflows will be automatically retried
    /// according to the `max_retries` configuration.
    ///
    /// # Use Cases
    /// - Enable for production resilience
    /// - Disable for debugging to see immediate failures
    pub retry_enabled: bool,

    /// Maximum number of retry attempts for failed workflows
    ///
    /// After this many failures, the workflow will be marked as permanently failed.
    /// Only applies when `retry_enabled` is true.
    ///
    /// # Recommendations
    /// - Transient failures: 3-5 retries
    /// - Critical workflows: 5-10 retries
    /// - Non-critical workflows: 1-3 retries
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
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_workflow_config_no_retry() {
        let config = CanonicalWorkflowConfig {
            enabled: true,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
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
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            ..Default::default()
        };
        assert_eq!(config.max_concurrent_workflows, 1000);
    }

    #[test]
    fn test_workflow_timeout_short() {
        let config = CanonicalWorkflowConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            default_timeout: Duration::from_secs(30),
            ..Default::default()
        };
        assert_eq!(config.default_timeout, Duration::from_secs(30));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_workflow_timeout_long() {
        let config = CanonicalWorkflowConfig {
            default_timeout: Duration::from_secs(3600),
            ..Default::default()
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.default_timeout, Duration::from_secs(3600));
    }

    #[test]
    fn test_workflow_max_retries() {
        let config = CanonicalWorkflowConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            retry_enabled: true,
            max_retries: 5,
            ..Default::default()
        };
        assert_eq!(config.max_retries, 5);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_type_alias() {
        let _config: WorkflowConfig = CanonicalWorkflowConfig::default();
    }
}
