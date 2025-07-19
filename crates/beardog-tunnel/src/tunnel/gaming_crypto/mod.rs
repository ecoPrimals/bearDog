//! Gaming Crypto Optimization - Modular Implementation
//!
//! This module provides high-performance, low-latency cryptographic operations
//! specifically optimized for gaming applications. Broken down into focused
//! sub-modules for better maintainability.

pub mod core;
pub mod ecosystem_integration;
pub mod genetic_optimization;
pub mod performance;

// Re-export main types for backward compatibility
pub use core::{EncryptedPacket, GamingCryptoEngine};
pub use ecosystem_integration::{EcosystemOptimizationRequest, EcosystemOptimizationResult};
pub use genetic_optimization::{EvolutionResult, GeneticParameters};
pub use performance::{NetworkPerformanceMetrics, PerformanceAnalysis};
