/// Core component implementations and lifecycle management
///
/// Provides component registration, health checking, and lifecycle management
/// for `BearDog` system components.
pub mod components;

/// Genetic algorithm optimization components
pub mod genetic_optimizer;
/// Service lifecycle management and state transitions
pub mod lifecycle;
#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
mod tests;

// Core submodules
pub mod adapter;
pub mod key_management; // Phase 2: HSM key persistence, public key management
pub mod monitoring;
pub mod security;
pub mod state;
pub mod system;

// Re-export key types for convenience
pub use adapter::UniversalAdapter;
pub use genetic_optimizer::{
    GeneticOptimizer, GeneticOptimizerConfig, OptimizationState, PerformanceMetric,
};
pub use monitoring::{
    AlertHandler, AlertSeverity, AlertType, ComponentHealth, SystemAlert, SystemMetrics,
    SystemMonitor, SystemMonitorConfig,
};
pub use security::CoreSecurityProvider;
pub use state::CoreState;
pub use system::BearDogCore;
