// SPDX-License-Identifier: AGPL-3.0-only

//! Core enumeration types for hybrid intelligence system
//!
//! This module contains the fundamental enum types used throughout the
//! hybrid intelligence system for decision making, learning, and optimization.

use serde::{Deserialize, Serialize};

/// Confidence level for AI-generated decisions
///
/// Indicates the system's confidence in a decision and whether human
/// review is required or recommended.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionConfidence {
    /// Low confidence, requires human review
    Low,
    /// Medium confidence, human review recommended
    Medium,
    /// High confidence, can proceed autonomously
    High,
}

/// AI model types supported by the hybrid intelligence system
///
/// Specifies the type of machine learning model to use for predictions
/// and decision making.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AIModelType {
    /// Neural network models for complex pattern recognition
    NeuralNetwork,
    /// Decision tree models for interpretable rule-based decisions
    DecisionTree,
    /// Ensemble models combining multiple approaches
    Ensemble,
}

/// Learning feedback types from human operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningFeedback {
    /// Positive feedback - AI decision was correct
    Positive,
    /// Negative feedback - AI decision was incorrect
    Negative,
    /// Neutral feedback - AI decision was acceptable but not optimal
    Neutral,
}

/// Learning rate adaptation strategy
///
/// Controls how the learning rate changes during training to optimize convergence.
#[derive(Debug, Clone, Copy)]
pub enum LearningRateAdaptation {
    /// Fixed learning rate throughout training
    Fixed,
    /// Adaptive learning rate that adjusts based on performance
    Adaptive,
    /// Scheduled learning rate decay over time
    Scheduled,
}

/// Model update frequency
///
/// Determines how often the model parameters are updated during training.
#[derive(Debug, Clone, Copy)]
pub enum UpdateFrequency {
    /// Update model after each batch
    PerBatch,
    /// Update model after each epoch
    PerEpoch,
    /// Update model after each sample
    PerSample,
}

/// Prediction model type
///
/// Specifies the type of prediction task the model performs.
#[derive(Debug, Clone, Copy)]
pub enum PredictionModel {
    /// Classification for categorical predictions
    Classification,
    /// Regression for continuous value predictions
    Regression,
    /// Time series forecasting for temporal data
    TimeSeries,
}

/// Optimization algorithms for model training
///
/// Specifies which optimization algorithm to use for training neural networks
/// and other machine learning models. Each algorithm has different convergence
/// properties and is suited for different types of problems.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Standard gradient descent optimization
    GradientDescent,
    /// Adam optimizer with adaptive learning rates
    Adam,
    /// Stochastic gradient descent with momentum
    SGD,
}

/// Optimizer type for neural network training
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Optimizer {
    /// Standard gradient descent
    GradientDescent,
    /// Adam optimizer
    Adam,
    /// `RMSprop` optimizer
    RMSprop,
    /// Adagrad optimizer
    Adagrad,
}

/// Inference mode for model predictions
#[derive(Debug, Clone, Copy)]
pub enum InferenceMode {
    /// Online inference with real-time predictions
    Online,
    /// Batch inference for multiple samples
    Batch,
    /// Streaming inference for continuous data
    Streaming,
}

/// Learning algorithm type
#[derive(Debug, Clone, Copy)]
pub enum LearningAlgorithmType {
    /// Supervised learning with labeled data
    Supervised,
    /// Unsupervised learning without labels
    Unsupervised,
    /// Reinforcement learning with reward signals
    Reinforcement,
}

/// Optimization strategy for system performance
#[derive(Debug, Clone, Copy)]
pub enum OptimizationStrategy {
    /// Optimize for latency (fastest response)
    Latency,
    /// Optimize for throughput (maximum capacity)
    Throughput,
    /// Balance between latency and throughput
    Balanced,
}

/// Optimization level for model performance
#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    /// No optimization (debug mode)
    None,
    /// Basic optimizations (development)
    Basic,
    /// Aggressive optimizations (production)
    Aggressive,
}

/// System command types for controlling the hybrid intelligence system
#[derive(Debug, Clone)]
pub enum SystemCommand {
    /// Update system configuration
    UpdateConfig(Box<super::config::HybridIntelligenceConfig>),
    /// Shutdown the system gracefully
    Shutdown,
    /// Pause system operations
    Pause,
    /// Resume paused operations
    Resume,
    /// Reset system state
    Reset,
}

/// Intelligence event types for system monitoring
#[derive(Debug, Clone, Copy)]
pub enum IntelligenceEventType {
    /// Decision was made by the system
    DecisionMade,
    /// Learning update applied
    LearningUpdate,
    /// Model performance changed
    PerformanceUpdate,
    /// System configuration changed
    ConfigurationUpdate,
    /// Error occurred
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_decision_confidence() {
        let v = DecisionConfidence::High;
        let json = serde_json::to_value(v).expect("serialize DecisionConfidence");
        let back: DecisionConfidence =
            serde_json::from_value(json).expect("deserialize DecisionConfidence");
        assert!(matches!(back, DecisionConfidence::High));
    }

    #[test]
    fn serde_roundtrip_ai_model_type() {
        let v = AIModelType::NeuralNetwork;
        let s = serde_json::to_string(&v).expect("serialize AIModelType");
        let back: AIModelType = serde_json::from_str(&s).expect("deserialize AIModelType");
        assert!(matches!(back, AIModelType::NeuralNetwork));
    }

    #[test]
    fn serde_roundtrip_learning_feedback() {
        for fb in [
            LearningFeedback::Positive,
            LearningFeedback::Negative,
            LearningFeedback::Neutral,
        ] {
            let s = serde_json::to_string(&fb).expect("serialize LearningFeedback");
            let back: LearningFeedback =
                serde_json::from_str(&s).expect("deserialize LearningFeedback");
            assert_eq!(format!("{fb:?}"), format!("{back:?}"));
        }
    }

    #[test]
    fn serde_roundtrip_optimization_algorithm_and_optimizer() {
        let oa = OptimizationAlgorithm::Adam;
        let s = serde_json::to_string(&oa).expect("serialize OptimizationAlgorithm");
        let back: OptimizationAlgorithm =
            serde_json::from_str(&s).expect("deserialize OptimizationAlgorithm");
        assert!(matches!(back, OptimizationAlgorithm::Adam));

        let op = Optimizer::RMSprop;
        let s2 = serde_json::to_string(&op).expect("serialize Optimizer");
        let back2: Optimizer = serde_json::from_str(&s2).expect("deserialize Optimizer");
        assert!(matches!(back2, Optimizer::RMSprop));
    }

    #[test]
    fn debug_non_serde_enums_nonempty() {
        assert!(!format!("{:?}", LearningRateAdaptation::Adaptive).is_empty());
        assert!(!format!("{:?}", UpdateFrequency::PerEpoch).is_empty());
        assert!(!format!("{:?}", PredictionModel::TimeSeries).is_empty());
        assert!(!format!("{:?}", InferenceMode::Streaming).is_empty());
        assert!(!format!("{:?}", LearningAlgorithmType::Reinforcement).is_empty());
        assert!(!format!("{:?}", OptimizationStrategy::Throughput).is_empty());
        assert!(!format!("{:?}", OptimizationLevel::Aggressive).is_empty());
        assert!(!format!("{:?}", IntelligenceEventType::Error).is_empty());
    }
}
