// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Online Learning and Adaptation
//!
//! This module handles online learning, adaptation strategies, and performance
//! tracking for the hybrid intelligence system.

use super::super::config::HybridIntelligenceConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use beardog_errors::BearDogError;

// OnlineLearningConfig: use `beardog_types::canonical::config::domains::ai_config`.

/// Learning rate adaptation strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LearningRateAdaptation {
    /// Fixed learning rate
    Fixed,
    /// Adaptive learning rate based on performance
    Adaptive,
    /// Scheduled learning rate decay
    Scheduled,
}

/// Adaptation strategies for the learning system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AdaptationStrategy {
    /// Conservative adaptation
    Conservative,
    /// Aggressive adaptation
    Aggressive,
    /// Balanced adaptation
    Balanced,
    /// Custom adaptation based on context
    Custom,
}

/// Learning performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetrics {
    /// Total learning iterations
    pub total_iterations: u64,
    /// Current learning rate
    pub current_learning_rate: f64,
    /// Average loss over recent iterations
    pub average_loss: f64,
    /// Learning convergence rate
    pub convergence_rate: f64,
    /// Adaptation effectiveness
    pub adaptation_effectiveness: f64,
}

/// Performance tracking system
pub struct PerformanceTracker {
    /// Historical performance data
    history: Vec<PerformancePoint>,
    /// Current metrics
    current_metrics: LearningMetrics,
}

/// Point in performance history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePoint {
    /// Timestamp of measurement
    pub timestamp: std::time::SystemTime,
    /// Performance score
    pub score: f64,
    /// Learning rate at this point
    pub learning_rate: f64,
    /// Context information
    pub context: HashMap<String, f64>,
}

/// **ONLINE LEARNING SYSTEM** - Handles adaptive learning
pub struct OnlineLearningSystem {
    /// Configuration
    config: HybridIntelligenceConfig,
    
    /// Learning configuration
    learning_config: OnlineLearningConfig,
    
    /// Performance tracker
    tracker: PerformanceTracker,
    
    /// Current adaptation strategy
    adaptation_strategy: AdaptationStrategy,
    
    /// Learning metrics
    metrics: LearningMetrics,
}

impl OnlineLearningSystem {
    /// Create a new online learning system
    #[must_use]
    pub fn new(config: HybridIntelligenceConfig) -> Self {
        Self {
            config,
            learning_config: OnlineLearningConfig::default(),
            tracker: PerformanceTracker::new(),
            adaptation_strategy: AdaptationStrategy::Balanced,
            metrics: LearningMetrics::default(),
        }
    }
    
    /// Record a decision for learning
    pub async fn record_decision(&mut self, _decision: &super::decision_making::DecisionResult) -> Result<(), BearDogError> {
        // Update learning metrics based on decision
        self.metrics.total_iterations += 1;
        Ok(())
    }
    
    /// Process feedback for learning adaptation
    pub async fn process_feedback(&mut self, _decision_id: &str, _feedback: super::decision_making::LearningFeedback) -> Result<(), BearDogError> {
        // Process feedback and adapt learning parameters
        Ok(())
    }
    
    /// Get current learning metrics
    pub async fn get_metrics(&self) -> Result<LearningMetrics, BearDogError> {
        Ok(self.metrics.clone())
    }
}

impl PerformanceTracker {
    #[must_use]
    /// Create a new performance tracker
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_metrics: LearningMetrics::default(),
        }
    }
}

impl Default for OnlineLearningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            learning_rate: beardog_errors::process_env::var("BEARDOG_AI_ADAPTIVE_LEARNING_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.001),
            batch_size: beardog_errors::process_env::var("BEARDOG_AI_ADAPTIVE_BATCH_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(32),
            memory_buffer_size: beardog_errors::process_env::var("BEARDOG_AI_MEMORY_BUFFER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            update_frequency: Duration::from_secs(
                beardog_errors::process_env::var("BEARDOG_LEARNING_UPDATE_FREQUENCY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
        }
    }
}

impl Default for LearningMetrics {
    fn default() -> Self {
        Self {
            total_iterations: 0,
            current_learning_rate: 0.001,
            average_loss: 0.0,
            convergence_rate: 0.0,
            adaptation_effectiveness: 0.5,
        }
    }
} 