// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Workflow Configuration Domain
//!
//! This module contains all workflow-related configuration types, organized into
//! logical submodules for better maintainability and clarity.
//!
//! ## Module Organization
//!
//! - `engine`: Workflow execution engine, queues, and timeouts
//! - `escalation`: Escalation policies, rules, and notifications
//! - `scheduling`: Workflow scheduling and timing
//! - `persistence`: Data persistence, retention, and archival
//! - `retry`: Retry policies and exponential backoff
//!
//! ## Usage
//!
//! ```rust,ignore
//! use beardog_types::canonical::config::domains::workflow::ConsolidatedWorkflowConfig;
//! use beardog_types::canonical::config::r#trait::BearDogConfig;
//!
//! let config = ConsolidatedWorkflowConfig::default();
//! config.validate()?;
//! ```

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::canonical::config::r#trait::BearDogConfig;

// ============================================================================
// Submodule Declarations
// ============================================================================

pub mod engine;
pub mod escalation;
pub mod persistence;
pub mod retry;
pub mod scheduling;

// ============================================================================
// Re-exports for Easy Access
// ============================================================================

// Engine types
pub use engine::{QueueConfig, TimeoutConfig, WorkflowEngineConfig};

// Escalation types
pub use escalation::{EscalationRule, NotificationConfig, WorkflowEscalationConfig};

// Persistence types
pub use persistence::{ArchiveConfig, ConnectionConfig, PersistenceConfig, RetentionConfig};

// Retry types
pub use retry::RetryConfig;

// Scheduling types
pub use scheduling::SchedulingConfig;

// ============================================================================
// Top-Level Consolidated Config
// ============================================================================

/// **CONSOLIDATED WORKFLOW CONFIGURATION** - Unifies all workflow configs
///
/// This is the main entry point for workflow configuration. It consolidates
/// engine, escalation, scheduling, and persistence settings into a single
/// coherent configuration structure.
///
/// ## Architecture
///
/// The workflow configuration is organized into five main areas:
///
/// 1. **Engine**: Core execution engine (worker pools, queues, timeouts)
/// 2. **Escalation**: Error escalation and notification policies
/// 3. **Scheduling**: Workflow scheduling and timing policies
/// 4. **Persistence**: Data storage, retention, and archival
/// 5. **Retry**: Retry policies with exponential backoff
///
/// ## Example
///
/// ```rust,ignore
/// use beardog_types::canonical::config::domains::workflow::ConsolidatedWorkflowConfig;
///
/// let config = ConsolidatedWorkflowConfig {
///     enabled: true,
///     engine: WorkflowEngineConfig::default(),
///     escalation: WorkflowEscalationConfig::default(),
///     scheduling: SchedulingConfig::default(),
///     persistence: PersistenceConfig::default(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedWorkflowConfig {
    /// Enable workflow processing
    pub enabled: bool,

    /// Workflow engine configuration
    pub engine: WorkflowEngineConfig,

    /// Escalation configuration
    pub escalation: WorkflowEscalationConfig,

    /// Scheduling configuration
    pub scheduling: SchedulingConfig,

    /// Persistence configuration
    pub persistence: PersistenceConfig,
}

// ============================================================================
// Default Implementation
// ============================================================================

impl Default for ConsolidatedWorkflowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            engine: WorkflowEngineConfig::default(),
            escalation: WorkflowEscalationConfig::default(),
            scheduling: SchedulingConfig::default(),
            persistence: PersistenceConfig::default(),
        }
    }
}

// ============================================================================
// BearDogConfig Implementation
// ============================================================================

impl BearDogConfig for ConsolidatedWorkflowConfig {
    fn domain() -> &'static str
    where
        Self: Sized,
    {
        "workflow"
    }

    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            if self.engine.worker_pool_size == 0 {
                return Err(BearDogError::validation(
                    "Worker pool size must be greater than 0",
                ));
            }

            if self.engine.queue.capacity == 0 {
                return Err(BearDogError::validation(
                    "Queue capacity must be greater than 0",
                ));
            }

            if self.scheduling.max_concurrent == 0 {
                return Err(BearDogError::validation(
                    "Max concurrent workflows must be greater than 0",
                ));
            }

            if self.persistence.enabled && self.persistence.connection.pool_size == 0 {
                return Err(BearDogError::validation(
                    "Connection pool size must be greater than 0 when persistence is enabled",
                ));
            }

            if self.escalation.enabled && self.escalation.rules.is_empty() {
                return Err(BearDogError::validation(
                    "Escalation rules cannot be empty when escalation is enabled",
                ));
            }
        }
        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        Ok(Self {
            enabled: other.enabled,
            engine: if other.enabled {
                other.engine.clone()
            } else {
                self.engine.clone()
            },
            escalation: if other.escalation.enabled {
                other.escalation.clone()
            } else {
                self.escalation.clone()
            },
            scheduling: if other.enabled {
                other.scheduling.clone()
            } else {
                self.scheduling.clone()
            },
            persistence: if other.persistence.enabled {
                other.persistence.clone()
            } else {
                self.persistence.clone()
            },
        })
    }

    fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Ok(enabled) = std::env::var(env_keys::ENV_WORKFLOW_ENABLED) {
            config.enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(worker_pool_size) = std::env::var(env_keys::ENV_WORKFLOW_WORKER_POOL_SIZE) {
            config.engine.worker_pool_size = worker_pool_size.parse().unwrap_or(10);
        }

        if let Ok(queue_capacity) = std::env::var(env_keys::ENV_WORKFLOW_QUEUE_CAPACITY) {
            config.engine.queue.capacity = queue_capacity.parse().unwrap_or(1000);
        }

        if let Ok(max_concurrent) = std::env::var(env_keys::ENV_WORKFLOW_MAX_CONCURRENT) {
            config.scheduling.max_concurrent = max_concurrent.parse().unwrap_or(5);
        }

        if let Ok(persistence_enabled) = std::env::var(env_keys::ENV_WORKFLOW_PERSISTENCE_ENABLED) {
            config.persistence.enabled = persistence_enabled.parse().unwrap_or(true);
        }

        if let Ok(backend) = std::env::var(env_keys::ENV_WORKFLOW_PERSISTENCE_BACKEND) {
            config.persistence.backend = Arc::from(backend.as_str());
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!("Failed to serialize workflow config to TOML: {e}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::config::r#trait::BearDogConfig;

    #[test]
    fn consolidated_workflow_default_domain_validate() {
        let c = ConsolidatedWorkflowConfig::default();
        assert!(c.enabled);
        assert_eq!(ConsolidatedWorkflowConfig::domain(), "workflow");
        c.validate()
            .expect("default workflow config should validate");
    }

    #[test]
    fn consolidated_workflow_serde_roundtrip() {
        let c = ConsolidatedWorkflowConfig::default();
        let v = serde_json::to_value(&c).expect("serialize workflow config");
        let back: ConsolidatedWorkflowConfig =
            serde_json::from_value(v).expect("deserialize workflow config");
        assert_eq!(c, back);
    }

    #[test]
    fn validate_errors_when_enabled_and_invalid_engine() {
        let mut c = ConsolidatedWorkflowConfig::default();
        c.engine.worker_pool_size = 0;
        assert!(c.validate().is_err());

        c = ConsolidatedWorkflowConfig::default();
        c.engine.queue.capacity = 0;
        assert!(c.validate().is_err());

        c = ConsolidatedWorkflowConfig::default();
        c.scheduling.max_concurrent = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_persistence_pool_when_enabled() {
        let mut c = ConsolidatedWorkflowConfig::default();
        c.persistence.enabled = true;
        c.persistence.connection.pool_size = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_escalation_rules_when_enabled() {
        let mut c = ConsolidatedWorkflowConfig::default();
        c.escalation.enabled = true;
        c.escalation.rules.clear();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_skips_subchecks_when_disabled() {
        let mut c = ConsolidatedWorkflowConfig::default();
        c.enabled = false;
        c.engine.worker_pool_size = 0;
        c.validate()
            .expect("disabled workflow skips engine validation");
    }

    #[test]
    fn merge_prefers_other_when_enabled() {
        let base = ConsolidatedWorkflowConfig::default();
        let mut other = ConsolidatedWorkflowConfig::default();
        other.enabled = true;
        other.engine.worker_pool_size = 42;
        let m = base.merge(&other).expect("merge");
        assert_eq!(m.engine.worker_pool_size, 42);
    }

    #[test]
    fn merge_keeps_base_engine_when_other_disabled() {
        let base = ConsolidatedWorkflowConfig::default();
        let mut other = ConsolidatedWorkflowConfig::default();
        other.enabled = false;
        other.engine.worker_pool_size = 99;
        let m = base.merge(&other).expect("merge");
        assert_eq!(m.engine.worker_pool_size, base.engine.worker_pool_size);
    }

    #[test]
    fn to_toml_roundtrip() {
        let c = ConsolidatedWorkflowConfig::default();
        let s = c.to_toml().expect("to_toml");
        assert!(!s.is_empty());
        let parsed: ConsolidatedWorkflowConfig = toml::from_str(&s).expect("from_str");
        assert_eq!(parsed.enabled, c.enabled);
    }

    #[test]
    fn from_env_smoke() {
        let _ = ConsolidatedWorkflowConfig::from_env();
    }
}
