// SPDX-License-Identifier: AGPL-3.0-or-later

// Configuration types for hybrid intelligence system

use super::{IntelligenceCapability, MachineLearningConfig};
use serde::{Deserialize, Serialize};

/// Strategy for reaching consensus in multi-agent decision making
///
/// Defines how multiple AI agents or human experts combine their inputs
/// to reach a unified decision.
#[derive(Debug, Clone, Copy)]
pub enum ConsensusStrategy {
    /// Majority rule consensus requiring >50% agreement
    Majority,
    /// Weighted consensus based on expertise and trust scores
    Weighted,
    /// Unanimous consensus requiring 100% agreement
    Unanimous,
}

/// Operating mode for the hybrid intelligence system
///
/// Controls the balance between human decision-making and AI automation,
/// ranging from pure human control to AI autonomy with oversight.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntelligenceMode {
    /// Pure human decision making without AI assistance
    Human,
    /// AI-assisted human decision making with recommendations
    HybridAssisted,
    /// Fully autonomous AI decision making with human oversight
    AutonomousAI,
}

/// Machine learning algorithm type
///
/// Specifies the type of learning algorithm used for model training
/// and optimization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningAlgorithm {
    /// Reward-based learning through trial and error
    ReinforcementLearning,
    /// Supervised learning with human-labeled training datasets
    SupervisedLearning,
    /// Unsupervised pattern discovery and clustering algorithms
    UnsupervisedLearning,
}

/// Machine learning training configuration
///
/// Configures hyperparameters for training machine learning models
/// in the hybrid intelligence system.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MLConfig {
    /// Learning rate for gradient descent optimization
    pub learning_rate: f64,
    /// Number of samples per training batch
    pub batch_size: usize,
    /// Maximum number of training epochs
    pub max_epochs: u32,
    /// Enable early stopping to prevent overfitting
    pub early_stopping: bool,
}

/// Neural network architecture configuration
///
/// Defines the structure and hyperparameters for neural network models
/// used in the hybrid intelligence system.
#[derive(Debug, Clone)]
pub struct NeuralConfig {
    /// Number of neurons in each hidden layer
    pub hidden_layers: Vec<usize>,
    /// Activation function type (relu, sigmoid, tanh)
    pub activation: String,
    /// Dropout rate for regularization (0.0 to 1.0)
    pub dropout_rate: f64,
    /// Enable batch normalization between layers
    pub batch_norm: bool,
}

/// Decision engine configuration
///
/// Controls how the hybrid intelligence system makes decisions,
/// including confidence thresholds and human feedback integration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecisionConfig {
    /// Minimum confidence threshold for automated decisions (0.0 to 1.0)
    pub confidence_threshold: f64,
    /// Enable human feedback integration for decision validation
    pub enable_human_feedback: bool,
    /// Maximum decision processing time in milliseconds
    pub max_processing_time_ms: u64,
}

/// Learning system configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Enable online learning
    /// Whether `online_learning` is enabled
    pub online_learning: bool,
    /// Experience replay buffer size
    /// Number of `replay_buffer_size`
    pub replay_buffer_size: usize,
    /// Learning update frequency
    /// Number of `update_frequency`
    pub update_frequency: u32,
    /// Exploration vs exploitation balance
    /// The exploration rate value
    pub exploration_rate: f64,
}

/// Prediction engine configuration
#[derive(Debug, Clone)]
pub struct PredictionConfig {
    /// Prediction horizon in time steps
    /// Number of horizon
    pub horizon: u32,
    /// Uncertainty quantification method
    /// The uncertainty method value
    pub uncertainty_method: String,
    /// Monte Carlo sample count
    /// Number of `mc_samples`
    pub mc_samples: u32,
    /// Prediction confidence threshold
    pub confidence_threshold: f64,
}

/// Optimization configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Optimization algorithm to use
    /// The algorithm value
    pub algorithm: OptimizationAlgorithm,
    /// Maximum optimization iterations
    /// Number of `max_iterations`
    pub max_iterations: u32,
    /// Convergence tolerance
    /// The tolerance value
    pub tolerance: f64,
    /// Enable parallel optimization
    /// Whether parallel is enabled
    pub parallel: bool,
}

/// Optimization algorithm selection
///
/// Specifies which optimization algorithm to use for training
/// and parameter updates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Stochastic gradient descent
    SGD,
    /// Adam optimizer with adaptive learning rates
    Adam,
    /// `RMSprop` optimizer with adaptive learning rate per parameter
    RMSprop,
}

/// Complete hybrid intelligence system configuration
///
/// Comprehensive configuration for the AI-human hybrid intelligence system,
/// including learning algorithms, neural networks, decision making, and optimization.
#[derive(Debug, Clone)]
pub struct HybridIntelligenceConfig {
    /// Current intelligence processing mode
    pub mode: IntelligenceMode,
    /// Selected learning algorithm type
    pub learning_algorithm: LearningAlgorithm,
    /// Weight assigned to human feedback (0.0 to 1.0)
    pub human_feedback_weight: f64,
    /// Minimum confidence threshold for AI decisions (0.0 to 1.0)
    pub ai_confidence_threshold: f64,
    /// Unique system identifier for this hybrid intelligence instance
    pub system_id: String,
    /// List of enabled intelligence capabilities
    pub enabled_capabilities: Vec<IntelligenceCapability>,
    /// Machine learning configuration settings
    pub ml_config: MachineLearningConfig,
    /// Neural network configuration
    pub neural_config: super::types::NeuralNetworkConfig,
    /// Decision engine configuration
    pub decision_config: super::types::DecisionEngineConfig,
    /// Learning system configuration
    pub learning_config: super::types::LearningConfig,
    /// Prediction engine configuration
    pub prediction_config: super::types::PredictionConfig,
    /// Optimization configuration
    pub optimization_config: super::types::OptimizationConfig,
}

impl Default for HybridIntelligenceConfig {
    fn default() -> Self {
        Self {
            mode: IntelligenceMode::HybridAssisted,
            learning_algorithm: LearningAlgorithm::ReinforcementLearning,
            human_feedback_weight: 0.3,
            ai_confidence_threshold: 0.8,
            system_id: "default-hybrid-intelligence".to_string(),
            enabled_capabilities: vec![],
            ml_config: MachineLearningConfig::default(),
            neural_config: create_default_neural_config_for_default(),
            decision_config: create_default_decision_config_for_default(),
            learning_config: super::types::LearningConfig::default(),
            prediction_config: super::types::PredictionConfig::default(),
            optimization_config: super::types::OptimizationConfig::default(),
        }
    }
}

// Helper functions for Default implementation
fn create_default_neural_config_for_default() -> super::types::NeuralNetworkConfig {
    // Create a simplified neural network configuration using defaults
    super::types::NeuralNetworkConfig::default()
}

fn create_default_decision_config_for_default() -> super::types::DecisionEngineConfig {
    // Create a simplified decision engine configuration using defaults
    super::types::DecisionEngineConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intelligence_mode_json_roundtrip() {
        let m = IntelligenceMode::AutonomousAI;
        let v = serde_json::to_value(m).expect("serialize IntelligenceMode");
        let back: IntelligenceMode =
            serde_json::from_value(v).expect("deserialize IntelligenceMode");
        assert!(matches!(back, IntelligenceMode::AutonomousAI));
    }

    #[test]
    fn learning_algorithm_json_roundtrip() {
        let a = LearningAlgorithm::SupervisedLearning;
        let s = serde_json::to_string(&a).expect("serialize LearningAlgorithm");
        let back: LearningAlgorithm =
            serde_json::from_str(&s).expect("deserialize LearningAlgorithm");
        assert!(matches!(back, LearningAlgorithm::SupervisedLearning));
    }

    #[test]
    fn ml_config_json_roundtrip() {
        let c = MLConfig {
            learning_rate: 0.01,
            batch_size: 32,
            max_epochs: 10,
            early_stopping: true,
        };
        let v = serde_json::to_value(c).expect("serialize MLConfig");
        let back: MLConfig = serde_json::from_value(v).expect("deserialize MLConfig");
        assert_eq!(back.batch_size, 32);
        assert!(back.early_stopping);
    }

    #[test]
    fn decision_config_json_roundtrip() {
        let c = DecisionConfig {
            confidence_threshold: 0.9,
            enable_human_feedback: false,
            max_processing_time_ms: 500,
        };
        let back: DecisionConfig =
            serde_json::from_str(&serde_json::to_string(&c).expect("serialize DecisionConfig"))
                .expect("deserialize DecisionConfig");
        assert_eq!(back.max_processing_time_ms, 500);
    }

    #[test]
    fn hybrid_intelligence_config_default_smoke() {
        let cfg = HybridIntelligenceConfig::default();
        assert_eq!(cfg.human_feedback_weight, 0.3);
        assert!(matches!(cfg.mode, IntelligenceMode::HybridAssisted));
    }
}
