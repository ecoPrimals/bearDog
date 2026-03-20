// SPDX-License-Identifier: AGPL-3.0-only

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
//! - [`engine`](crate::ai_optimization::engine): Core AI optimization engine with neural networks
//! - [`history`](crate::ai_optimization::history): Historical optimization data and learning patterns
//! - [`neural_network`](crate::ai_optimization::neural_network): Simple neural network implementation
//! - [`predictor`](crate::ai_optimization::predictor): Resource usage prediction algorithms
//! - [`types`](crate::ai_optimization::types): Type definitions for AI optimization components
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
//! - **Iterator and chunk patterns** that LLVM can auto-vectorize on hot paths
//! - **SIMD vectorization** where applicable
//!
//! ## Security
//!
//! All AI operations maintain:
//! - **Memory safety** with zero unsafe code
//! - **Input validation** for all external data
//! - **Resource bounds** to prevent `DoS` attacks
//! - **Privacy preservation** in ML model training

/// Core scheduling loop and integration glue for the AI optimizer.
pub mod engine;
/// Past decisions and outcomes used for simple learning adjustments.
pub mod history;
/// Minimal feed-forward net for experimentation (not production ML).
pub mod neural_network;
/// Heuristic forecasting from recent samples.
pub mod predictor;
/// Shared structs and enums for the AI optimization pipeline.
pub mod types;

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests_comprehensive;

pub use engine::AIOptimizationEngine;
pub use history::OptimizationHistory;
pub use neural_network::SimpleNeuralNetwork;
pub use predictor::ResourcePredictor;
pub use types::*;
