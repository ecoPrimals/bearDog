// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod engine;
pub mod history;
pub mod neural_network;
pub mod predictor;
pub mod types;

pub use engine::AIOptimizationEngine;
pub use history::OptimizationHistory;
pub use neural_network::SimpleNeuralNetwork;
pub use predictor::ResourcePredictor;
pub use types::*;
