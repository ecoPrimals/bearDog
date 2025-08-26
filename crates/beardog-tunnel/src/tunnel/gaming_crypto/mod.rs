

pub mod core;
pub mod ecosystem_integration;
pub mod genetic_optimization;
pub mod performance;

pub use core::{EncryptedPacket, GamingCryptoEngine};
pub use ecosystem_integration::{EcosystemOptimizationRequest, EcosystemOptimizationResult};
pub use genetic_optimization::{EvolutionResult, GeneticParameters};
pub use performance::{NetworkPerformanceMetrics, PerformanceAnalysis};
