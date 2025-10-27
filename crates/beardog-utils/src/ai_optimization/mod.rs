//! # AI Optimization Module
//!
//! **Ultra-pedantic AI-driven performance optimization for the `BearDog` ecosystem.**
//!
//! This module provides sophisticated artificial intelligence capabilities for:
//! - **Performance Prediction**: ML-based resource usage forecasting
//! - **Adaptive Optimization**: Real-time system tuning based on AI insights  
//! - **Neural Network Integration**: Deep learning for pattern recognition
//! - **Historical Analysis**: Learning from past optimization decisions
//!
//! ## Architecture
//!
//! The AI optimization system consists of:
//! - [`engine`]: Core AI optimization engine with neural networks
//! - [`history`]: Historical optimization data and learning patterns
//! - [`neural_network`]: Simple neural network implementation
//! - [`predictor`]: Resource usage prediction algorithms
//! - [`types`]: Type definitions for AI optimization components
//!
//! ## Examples
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use beardog_utils::ai_optimization::AIOptimizationEngine;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create AI optimization engine
//! let engine = AIOptimizationEngine::new(Duration::from_secs(60))?;
//!
//! // Start AI-driven optimization
//! engine.start_optimization().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance
//!
//! This module is optimized for:
//! - **Zero-allocation patterns** in hot paths
//! - **Const evaluation** for compile-time optimizations
//! - **Inline assembly** for critical performance sections
//! - **SIMD vectorization** where applicable
//!
//! ## Security
//!
//! All AI operations maintain:
//! - **Memory safety** with zero unsafe code
//! - **Input validation** for all external data
//! - **Resource bounds** to prevent `DoS` attacks
//! - **Privacy preservation** in ML model training

pub mod engine;
pub mod history;
pub mod neural_network;
pub mod predictor;
pub mod types;

#[cfg(test)]
mod tests_comprehensive;

pub use engine::AIOptimizationEngine;
pub use history::OptimizationHistory;
pub use neural_network::SimpleNeuralNetwork;
pub use predictor::ResourcePredictor;
pub use types::*;
