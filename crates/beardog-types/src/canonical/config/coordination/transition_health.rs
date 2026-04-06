// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(missing_docs)]

//! Coordination model transitions, health monitoring, and related actions.

use super::supporting::{RollbackStrategy, ValidationCheck};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationTransitionConfig {
    /// Enable automatic transitions between coordination models
    /// Whether `auto_transition` is enabled
    pub auto_transition_enabled: bool,
    /// Conditions that trigger transitions
    /// Collection of transition triggers
    pub transition_triggers: Vec<TransitionTrigger>,
    /// Transition strategies
    /// Mapping of transition strategies
    pub transition_strategies: HashMap<String, TransitionStrategy>,
    /// Cooldown period between transitions
    /// The transition cooldown value
    pub transition_cooldown: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionTrigger {
    /// Trigger identifier
    pub trigger_id: Arc<str>,
    /// Condition that activates the trigger
    /// The condition value
    pub condition: TriggerCondition,
    /// Target coordination model
    /// The target model value
    pub target_model: Arc<str>,
    /// Priority of this trigger
    /// Number of priority
    pub priority: u32,
}

/// Condition that can trigger a transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    PerformanceThreshold {
        metric: Arc<str>,
        threshold: f64,
    },
    /// System load exceeds threshold
    LoadThreshold {
        threshold: f64,
    },
    /// Specific time-based condition
    TimeCondition {
        condition: Arc<str>,
    },
    /// External event occurs
    ExternalEvent {
        event_type: Arc<str>,
    },
    /// Participant availability changes
    AvailabilityChange {
        min_participants: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStrategy {
    /// Strategy identifier
    pub strategy_id: Arc<str>,
    /// Steps to execute during transition
    /// Collection of transition steps
    pub transition_steps: Vec<TransitionStep>,
    /// Rollback strategy if transition fails
    /// Optional rollback strategy
    pub rollback_strategy: Option<RollbackStrategy>,
    /// Validation checks during transition
    pub validation_checks: Vec<ValidationCheck>,
}

/// Step in coordination transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStep {
    /// Step identifier
    pub step_id: Arc<str>,
    /// The action value
    pub action: TransitionAction,
    pub timeout: Duration,
    /// Collection of success conditions
    pub success_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionAction {
    /// Notify participants of transition
    NotifyParticipants {
        message: Arc<str>,
    },
    /// Transfer authority/responsibility
    TransferAuthority {
        from: Arc<str>,
        to: Arc<str>,
    },
    /// Update configuration
    UpdateConfiguration {
        config_changes: HashMap<Arc<str>, Arc<str>>,
    },
    /// Validate system state
    ValidateState {
        validation_type: Arc<str>,
    },
    WaitForCondition {
        condition: Arc<str>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationHealthConfig {
    /// Enable health monitoring
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
    /// Health check frequency
    /// The check frequency value
    pub check_frequency: Duration,
    /// Health metrics to track
    /// Collection of health metrics
    pub health_metrics: Vec<HealthMetric>,
    /// Mapping of alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetric {
    /// Metric identifier
    pub metric_id: Arc<str>,
    /// Description of the metric
    /// The description value
    pub description: Arc<str>,
    /// How to calculate the metric
    /// The calculation method value
    pub calculation_method: Arc<str>,
    /// The target value value
    pub target_value: f64,
    /// Current measured value
    /// The current value value
    pub current_value: f64,
    /// Trend over time
    /// The trend value
    pub trend: HealthTrend,
}

/// Trend in health metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthTrend {
    /// Currently improving
    Improving,
    /// Represents stable variant
    Stable,
    /// Currently declining
    Declining,
    /// Represents volatile variant
    Volatile,
}
