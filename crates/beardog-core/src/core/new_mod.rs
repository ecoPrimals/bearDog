// SPDX-License-Identifier: AGPL-3.0-only

/// Core component implementations and lifecycle management
///
/// Provides component registration, health checking, and lifecycle management
/// for `BearDog` system components.
pub mod components;
/// Genetic algorithm optimization components
pub mod genetic_optimizer;
/// Service lifecycle management and state transitions
pub mod lifecycle;

// Core submodules
/// Legacy layout: adapter bridge for the experimental core split.
pub mod adapter;
/// Legacy layout: nested [`BearDogCore`] module mirror.
pub mod core;
/// Legacy layout: monitoring hooks for the experimental core split.
pub mod monitoring;
/// Legacy layout: security provider for the experimental core split.
pub mod security;
/// Legacy layout: shared state for the experimental core split.
pub mod state;

// Re-export key types for convenience
pub use adapter::UniversalAdapter;
pub use core::BearDogCore;
pub use genetic_optimizer::{
    GeneticOptimizer, GeneticOptimizerConfig, OptimizationState, PerformanceMetric,
};
pub use monitoring::{
    AlertHandler, AlertSeverity, AlertType, ComponentHealth, SystemAlert, SystemMetrics,
    SystemMonitor, SystemMonitorConfig,
};
pub use security::CoreSecurityProvider;
pub use state::CoreState;

