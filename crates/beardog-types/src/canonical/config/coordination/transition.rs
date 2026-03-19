// SPDX-License-Identifier: AGPL-3.0-only

//! Transition management and health monitoring
//!
//! This module handles coordination model transitions and health monitoring
//! to ensure smooth evolution between coordination patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Configuration for coordination model transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationTransitionConfig {
    /// Triggers that initiate transitions
    pub transition_triggers: Vec<TransitionTrigger>,
    /// Available transition strategies
    pub transition_strategies: Vec<TransitionStrategy>,
    /// Rollback configuration
    pub rollback_config: RollbackConfig,
}

/// Trigger conditions for coordination transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionTrigger {
    /// Trigger identifier
    pub trigger_id: String,
    /// Trigger condition
    pub condition: TriggerCondition,
    /// Target coordination model
    pub target_model: String,
    /// Trigger priority (higher = more urgent)
    pub priority: u32,
}

/// Conditions that can trigger coordination transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Performance threshold exceeded
    PerformanceThreshold { metric: String, threshold: f64, comparison: String },
    /// System load threshold
    SystemLoad { cpu_threshold: f64, memory_threshold: f64 },
    /// Participant availability changed
    ParticipantAvailability { min_participants: u32, current_participants: u32 },
    /// Emergency situation detected
    EmergencyDetected { emergency_type: String, severity: u32 },
    /// Scheduled transition
    ScheduledTransition { transition_time: chrono::DateTime<chrono::Utc> },
    /// Manual trigger
    ManualTrigger { triggered_by: String, reason: String },
}

/// Strategy for executing coordination transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStrategy {
    /// Strategy identifier
    pub strategy_id: String,
    /// Strategy name
    pub strategy_name: String,
    /// Transition steps
    pub transition_steps: Vec<TransitionStep>,
    /// Estimated transition time
    pub estimated_duration: Duration,
    /// Rollback threshold (failure rate that triggers rollback)
    pub rollback_threshold: f64,
}

/// Individual step in transition strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStep {
    /// Step identifier
    pub step_id: String,
    /// Step description
    pub description: String,
    /// Step action
    pub action: TransitionAction,
    /// Prerequisites for this step
    pub prerequisites: Vec<String>,
    /// Success criteria
    pub success_criteria: Vec<String>,
    /// Maximum step duration
    pub max_duration: Duration,
}

/// Actions that can be performed during transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionAction {
    /// Notify participants of transition
    NotifyParticipants { message: String, urgency: String },
    /// Migrate coordination state
    MigrateState { source_model: String, target_model: String },
    /// Update participant roles
    UpdateRoles { role_changes: HashMap<String, String> },
    /// Validate new coordination model
    ValidateModel { validation_checks: Vec<String> },
    /// Activate new coordination model
    ActivateModel { model_id: String, configuration: HashMap<String, String> },
}

/// Rollback configuration for failed transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Enable automatic rollback
    pub auto_rollback_enabled: bool,
    /// Rollback strategies available
    pub rollback_strategies: Vec<RollbackStrategy>,
    /// Maximum rollback attempts
    pub max_rollback_attempts: u32,
}

/// Strategy for rolling back failed transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStrategy {
    /// Strategy identifier
    pub strategy_id: String,
    /// Rollback steps
    pub rollback_steps: Vec<String>,
    /// Estimated rollback time
    pub estimated_duration: Duration,
}

/// Health monitoring configuration for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationHealthConfig {
    /// Health metrics to monitor
    pub health_metrics: Vec<HealthMetric>,
    /// Monitoring frequency
    pub monitoring_frequency: Duration,
    /// Health check timeout
    pub health_check_timeout: Duration,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// Individual health metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetric {
    /// Metric identifier
    pub metric_id: String,
    /// Metric name
    pub metric_name: String,
    /// Metric description
    pub description: String,
    /// Current metric value
    pub current_value: Option<f64>,
    /// Healthy range (min, max)
    pub healthy_range: (f64, f64),
    /// Trend analysis
    pub trend: Option<HealthTrend>,
    /// Last measurement time
    pub last_measured: Option<chrono::DateTime<chrono::Utc>>,
}

/// Health trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthTrend {
    /// Metric is improving
    Improving { rate: f64 },
    /// Metric is stable
    Stable { variance: f64 },
    /// Metric is degrading
    Degrading { rate: f64 },
    /// Trend is unknown or insufficient data
    Unknown,
}

impl Default for CoordinationTransitionConfig {
    fn default() -> Self {
        Self {
            transition_triggers: vec![],
            transition_strategies: vec![
                TransitionStrategy {
                    strategy_id: "default_gradual".to_string(),
                    strategy_name: "Default Gradual Transition".to_string(),
                    transition_steps: vec![
                        TransitionStep {
                            step_id: "notify".to_string(),
                            description: "Notify participants of transition".to_string(),
                            action: TransitionAction::NotifyParticipants {
                                message: "Coordination model transition initiated".to_string(),
                                urgency: "normal".to_string(),
                            },
                            prerequisites: vec![],
                            success_criteria: vec!["notification_sent".to_string()],
                            max_duration: Duration::from_secs(60),
                        }
                    ],
                    estimated_duration: Duration::from_secs(300),
                    rollback_threshold: 0.3,
                }
            ],
            rollback_config: RollbackConfig {
                auto_rollback_enabled: true,
                rollback_strategies: vec![],
                max_rollback_attempts: 3,
            },
        }
    }
}

impl Default for CoordinationHealthConfig {
    fn default() -> Self {
        Self {
            health_metrics: vec![
                HealthMetric {
                    metric_id: "response_time".to_string(),
                    metric_name: "Average Response Time".to_string(),
                    description: "Average time for coordination decisions".to_string(),
                    current_value: None,
                    healthy_range: (0.0, 1000.0), // 0-1000ms
                    trend: None,
                    last_measured: None,
                },
                HealthMetric {
                    metric_id: "participation_rate".to_string(),
                    metric_name: "Participant Engagement Rate".to_string(),
                    description: "Percentage of participants actively engaged".to_string(),
                    current_value: None,
                    healthy_range: (0.6, 1.0), // 60-100%
                    trend: None,
                    last_measured: None,
                }
            ],
            monitoring_frequency: Duration::from_secs(60), // Every minute
            health_check_timeout: Duration::from_secs(10),
            alert_thresholds: HashMap::from([
                ("response_time".to_string(), 2000.0), // 2 seconds
                ("participation_rate".to_string(), 0.4), // 40%
            ]),
        }
    }
} 