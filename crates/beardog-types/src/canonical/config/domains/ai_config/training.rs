// SPDX-License-Identifier: AGPL-3.0-or-later

//! Training Configuration
//!
//! This module contains configuration types for ML model training,
//! including training parameters, batch configuration, and checkpointing.

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};

/// **TRAINING CONFIGURATION** - ML model training settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingConfig {
    /// Enable training mode
    pub enabled: bool,

    /// Training data sources
    pub data_sources: Vec<String>,

    /// Training batch size
    pub batch_size: usize,

    /// Learning rate
    pub learning_rate: f64,

    /// Number of training epochs
    pub epochs: u32,

    /// Validation split ratio
    pub validation_split: f64,

    /// Early stopping patience
    pub early_stopping_patience: u32,

    /// Model checkpoint frequency
    pub checkpoint_frequency: u32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            data_sources: vec![],
            batch_size: std::env::var(env_keys::ENV_AI_TRAINING_BATCH_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            learning_rate: std::env::var(env_keys::ENV_AI_LEARNING_RATE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            epochs: std::env::var(env_keys::ENV_AI_EPOCHS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            validation_split: std::env::var(env_keys::ENV_AI_VALIDATION_SPLIT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.2),
            early_stopping_patience: std::env::var(env_keys::ENV_AI_EARLY_STOPPING_PATIENCE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            checkpoint_frequency: std::env::var(env_keys::ENV_AI_CHECKPOINT_FREQUENCY)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
        }
    }
}

/// Training parameters (detailed configuration)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingParams {
    /// Training epochs
    pub epochs: u32,
    /// Batch size
    pub batch_size: usize,
    /// Validation split
    pub validation_split: f64,
    /// Shuffle data
    pub shuffle: bool,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
    /// Verbose output level
    pub verbose: VerboseLevel,
}

impl Default for TrainingParams {
    fn default() -> Self {
        Self {
            epochs: std::env::var(env_keys::ENV_TRAINING_PARAMS_EPOCHS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            batch_size: std::env::var(env_keys::ENV_TRAINING_PARAMS_BATCH_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            validation_split: std::env::var(env_keys::ENV_TRAINING_PARAMS_VALIDATION_SPLIT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.2),
            shuffle: true,
            random_seed: None,
            verbose: VerboseLevel::Normal,
        }
    }
}

/// Verbose output levels for training
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerboseLevel {
    /// No output
    Silent,
    /// Minimal output
    Minimal,
    /// Normal output
    Normal,
    /// Detailed output
    Detailed,
    /// Debug output
    Debug,
}

/// Early stopping configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EarlyStoppingConfig {
    /// Metric to monitor
    pub monitor: String,
    /// Monitoring mode
    pub mode: MonitoringMode,
    /// Patience (epochs to wait)
    pub patience: u32,
    /// Minimum delta for improvement
    pub min_delta: f64,
    /// Restore best weights
    pub restore_best_weights: bool,
    /// Baseline value
    pub baseline: Option<f64>,
}

impl Default for EarlyStoppingConfig {
    fn default() -> Self {
        Self {
            monitor: "val_loss".to_string(),
            mode: MonitoringMode::Min,
            patience: std::env::var(env_keys::ENV_AI_EARLY_STOPPING_PATIENCE_EPOCHS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            min_delta: std::env::var(env_keys::ENV_AI_EARLY_STOPPING_MIN_DELTA)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0001),
            restore_best_weights: true,
            baseline: None,
        }
    }
}

/// Monitoring mode for early stopping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonitoringMode {
    /// Monitor for minimum value
    Min,
    /// Monitor for maximum value
    Max,
    /// Auto-detect from metric name
    Auto,
}

/// Model checkpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointConfig {
    /// Checkpoint directory
    pub checkpoint_dir: String,
    /// Checkpoint frequency (epochs)
    pub frequency: u32,
    /// Save only best model
    pub save_best_only: bool,
    /// Metric to monitor for best model
    pub monitor: String,
    /// Monitoring mode
    pub mode: MonitoringMode,
    /// Save weights only (not full model)
    pub save_weights_only: bool,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            checkpoint_dir: "./checkpoints".to_string(),
            frequency: std::env::var(env_keys::ENV_AI_CHECKPOINT_FREQUENCY_EPOCHS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            save_best_only: true,
            monitor: "val_loss".to_string(),
            mode: MonitoringMode::Min,
            save_weights_only: false,
        }
    }
}

/// Metric types for training evaluation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metric {
    /// Accuracy
    Accuracy,
    /// Precision
    Precision,
    /// Recall
    Recall,
    /// F1 score
    F1Score,
    /// Area under ROC curve
    AucRoc,
    /// Mean absolute error
    MeanAbsoluteError,
    /// Mean squared error
    MeanSquaredError,
    /// Root mean squared error
    RootMeanSquaredError,
    /// Mean absolute percentage error
    MeanAbsolutePercentageError,
    /// R-squared
    RSquared,
    /// Custom metric
    Custom,
}
