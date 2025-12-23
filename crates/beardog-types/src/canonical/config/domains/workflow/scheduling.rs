//! Workflow Scheduling Configuration
//!
//! Configuration for workflow scheduling policies and execution timing.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulingConfig {
    /// Scheduler type (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub scheduler_type: Arc<str>,

    /// Default schedule (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub default_schedule: Arc<str>,

    /// Time zone (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub timezone: Arc<str>,

    /// Concurrent execution limit
    pub max_concurrent: usize,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl SchedulingConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            scheduler_type: Arc::from(
                source
                    .get_or("BEARDOG_WORKFLOW_SCHEDULER_TYPE", "cron")
                    .as_str(),
            ),
            default_schedule: Arc::from(
                source
                    .get_or("BEARDOG_WORKFLOW_DEFAULT_SCHEDULE", "0 0 * * *")
                    .as_str(),
            ),
            timezone: Arc::from(source.get_or("BEARDOG_WORKFLOW_TIMEZONE", "UTC").as_str()),
            max_concurrent: get_parsed(source, "BEARDOG_WORKFLOW_MAX_CONCURRENT", 5),
        }
    }
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}
