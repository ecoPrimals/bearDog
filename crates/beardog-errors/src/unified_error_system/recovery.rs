//! # Error Recovery Module
//!
//! This module contains error recovery and remediation functionality.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::RemediationAction;

/// **ERROR RECOVERY** - Error recovery information and mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecovery {
    /// Whether the error is recoverable
    pub is_recoverable: bool,

    /// Recovery strategy to use
    pub strategy: RecoveryStrategy,

    /// Recovery suggestions
    pub suggestions: Vec<String>,

    /// Maximum retry attempts
    pub max_retries: u32,

    /// Retry delay
    pub retry_delay: Duration,

    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,

    /// Recovery timeout
    pub recovery_timeout: Duration,

    /// Prerequisites for recovery
    pub prerequisites: Vec<String>,
}

/// **ERROR REMEDIATION** - Error remediation suggestions and actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRemediation {
    /// Available remediation actions
    pub actions: Vec<RemediationAction>,

    /// Recommended action priority order
    pub priority_order: Vec<String>,

    /// Automatic remediation enabled
    pub auto_remediation: bool,

    /// Remediation timeout
    pub timeout: Duration,

    /// Success criteria for remediation
    pub success_criteria: Vec<String>,
}

/// **RECOVERY STRATEGY** - Different strategies for error recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// No recovery possible
    None,
    /// Simple retry with fixed delay
    SimpleRetry,
    /// Exponential backoff retry
    ExponentialBackoff,
    /// Circuit breaker pattern
    CircuitBreaker,
    /// Fallback to alternative service
    Fallback,
    /// Manual intervention required
    Manual,
    /// Custom recovery strategy
    Custom(String),
}

impl ErrorRecovery {
    /// Create a new error recovery with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_recoverable: false,
            strategy: RecoveryStrategy::None,
            suggestions: Vec::new(),
            max_retries: 0,
            retry_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            recovery_timeout: Duration::from_secs(30),
            prerequisites: Vec::new(),
        }
    }

    #[must_use]
    /// Create a recoverable error recovery configuration
    pub fn recoverable() -> Self {
        Self {
            is_recoverable: true,
            strategy: RecoveryStrategy::SimpleRetry,
            suggestions: vec!["Retry the operation".to_string()],
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            recovery_timeout: Duration::from_secs(30),
            prerequisites: Vec::new(),
        }
    }
 #[must_use]

    /// Create recovery with exponential backoff
    pub fn with_exponential_backoff(max_retries: u32, initial_delay: Duration) -> Self {
        Self {
            is_recoverable: true,
            strategy: RecoveryStrategy::ExponentialBackoff,
            suggestions: vec!["Retry with exponential backoff".to_string()],
            max_retries,
            retry_delay: initial_delay,
            backoff_multiplier: 2.0,
            recovery_timeout: Duration::from_secs(300),
            prerequisites: Vec::new(),
        }
    }

    /// Add a recovery suggestion
    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    /// Add a prerequisite for recovery
    pub fn add_prerequisite(&mut self, prerequisite: String) {
        self.prerequisites.push(prerequisite);
    }

    /// Check if recovery should be attempted
    #[must_use]
    pub fn should_retry(&self, attempt: u32) -> bool {
        self.is_recoverable && attempt < self.max_retries
    }

    /// Calculate retry delay for the given attempt
    pub fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        match self.strategy {
            RecoveryStrategy::SimpleRetry => self.retry_delay,
            RecoveryStrategy::ExponentialBackoff => {
                let delay_secs = self.retry_delay.as_secs() as f64
                    * self.backoff_multiplier.powi(attempt as i32);
                Duration::from_secs(delay_secs as u64)
            }
            _ => self.retry_delay,
        }
    }
}

impl ErrorRemediation {
    /// Create a new error remediation with default settings
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            priority_order: Vec::new(),
            auto_remediation: false,
            timeout: Duration::from_secs(60),
            success_criteria: Vec::new(),
        }
    }

    /// Create remediation with actions
    pub fn with_actions(actions: Vec<RemediationAction>) -> Self {
        let priority_order = actions.iter().map(|a| a.id.clone()).collect();
        Self {
            actions,
            priority_order,
            auto_remediation: false,
            timeout: Duration::from_secs(60),
            success_criteria: Vec::new(),
        }
    }

    #[must_use]
    /// Add a remediation action
    pub fn add_action(&mut self, action: RemediationAction) {
        self.priority_order.push(action.id.clone());
        self.actions.push(action);
    }

    /// Get actions in priority order
    pub fn get_prioritized_actions(&self) -> Vec<&RemediationAction> {
        let mut prioritized = Vec::new();
        for id in &self.priority_order {
            if let Some(action) = self.actions.iter().find(|a| &a.id == id) {
                prioritized.push(action);
            }
        }
        prioritized
    }

    /// Get automatic remediation actions
    pub fn get_automatic_actions(&self) -> Vec<&RemediationAction> {
        self.actions
            .iter()
            .filter(|a| matches!(a.action_type, super::RemediationActionType::Automatic))
            .collect()
    }

    /// Add success criterion
    pub fn add_success_criterion(&mut self, criterion: String) {
        self.success_criteria.push(criterion);
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorRemediation {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RecoveryStrategy {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for RecoveryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::SimpleRetry => write!(f, "SIMPLE_RETRY"),
            Self::ExponentialBackoff => write!(f, "EXPONENTIAL_BACKOFF"),
            Self::CircuitBreaker => write!(f, "CIRCUIT_BREAKER"),
            Self::Fallback => write!(f, "FALLBACK"),
            Self::Manual => write!(f, "MANUAL"),
            Self::Custom(strategy) => write!(f, "CUSTOM({})", strategy),
        }
    }
}
