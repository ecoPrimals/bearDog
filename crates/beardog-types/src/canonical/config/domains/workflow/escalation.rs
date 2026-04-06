// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workflow Escalation Configuration
//!
//! Configuration for workflow escalation policies, rules, and notifications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Workflow escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowEscalationConfig {
    /// Enable escalation
    pub enabled: bool,

    /// Escalation rules
    pub rules: Vec<EscalationRule>,

    /// Default escalation timeout
    pub default_timeout: Duration,

    /// Notification configuration
    pub notifications: NotificationConfig,
}

/// Escalation rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EscalationRule {
    /// Rule name (`Arc<str>` for fast cloning across workflow evaluations)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub name: Arc<str>,

    /// Condition expression (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub condition: Arc<str>,

    /// Escalation target identifier (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub target: Arc<str>,

    /// Timeout
    pub timeout: Duration,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationConfig {
    /// Notification channels
    pub channels: Vec<String>,

    /// Templates
    pub templates: HashMap<String, String>,

    /// Rate limiting
    pub rate_limit: super::super::network::RateLimitConfig,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl WorkflowEscalationConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            enabled: get_bool(source, "BEARDOG_WORKFLOW_POLICY_ENABLED", false),
            rules: vec![],
            default_timeout: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_WORKFLOW_DEFAULT_TIMEOUT_SECS",
                300,
            )),
            notifications: NotificationConfig::default(),
        }
    }
}

impl Default for WorkflowEscalationConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            channels: vec!["email".to_string()],
            templates: HashMap::new(),
            rate_limit: super::super::network::RateLimitConfig::default(),
        }
    }
}
