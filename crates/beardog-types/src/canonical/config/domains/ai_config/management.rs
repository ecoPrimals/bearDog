// SPDX-License-Identifier: AGPL-3.0-or-later

//! AI Management and Security Configuration
//!
//! This module contains configuration types for AI model management,
//! performance optimization, and security settings.

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};

/// Decision engine configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionEngineConfig {
    /// Enable decision engine
    pub enabled: bool,
    /// Decision strategy
    pub strategy: DecisionStrategy,
    /// Confidence threshold
    pub confidence_threshold: f64,
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: DecisionStrategy::ConsensusVoting,
            confidence_threshold: std::env::var(env_keys::ENV_AI_CONFIDENCE_THRESHOLD)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.8),
        }
    }
}

/// Decision strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionStrategy {
    /// Consensus voting
    ConsensusVoting,
    /// Weighted voting
    WeightedVoting,
    /// Hierarchical decision making
    Hierarchical,
}

/// Model management configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelManagementConfig {
    /// Enable model management
    pub enabled: bool,
    /// Model registry endpoint
    pub registry_endpoint: String,
    /// Version control enabled
    pub version_control: bool,
    /// Model versioning strategy
    pub versioning_strategy: VersioningStrategy,
}

impl Default for ModelManagementConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            registry_endpoint: String::new(),
            version_control: true,
            versioning_strategy: VersioningStrategy::Semantic,
        }
    }
}

/// Versioning strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersioningStrategy {
    /// Semantic versioning
    Semantic,
    /// Timestamp-based versioning
    Timestamp,
    /// Hash-based versioning
    Hash,
}

/// AI performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiPerformanceConfig {
    /// Enable performance optimization
    pub enabled: bool,
    /// Use GPU acceleration
    pub use_gpu: bool,
    /// Number of worker threads
    pub num_workers: usize,
    /// Enable mixed precision training
    pub mixed_precision: bool,
}

impl Default for AiPerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            use_gpu: false,
            num_workers: std::env::var(env_keys::ENV_AI_NUM_WORKERS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            mixed_precision: false,
        }
    }
}

/// AI security configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiSecurityConfig {
    /// Enable security features
    pub enabled: bool,
    /// Model encryption enabled
    pub model_encryption: bool,
    /// Differential privacy enabled
    pub differential_privacy: bool,
    /// Privacy budget
    pub privacy_budget: Option<f64>,
}

impl Default for AiSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            model_encryption: false,
            differential_privacy: false,
            privacy_budget: None,
        }
    }
}
