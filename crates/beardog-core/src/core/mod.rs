// SPDX-License-Identifier: AGPL-3.0-only

//! Core Module - System Components and Lifecycle Management
//!
//! The core module provides foundational system components for BearDog including:
//!
//! - **Components**: Component registration, health checking, and coordination
//! - **Lifecycle**: Service state transitions and shutdown handling
//! - **Monitoring**: System metrics, alerts, and health reporting
//! - **Security**: Core security provider and cryptographic operations
//! - **State**: System-wide state management
//!
//! # Architecture
//!
//! The core module follows a modular design where each subsystem can operate
//! independently while integrating through well-defined interfaces. The
//! `BearDogCore` struct serves as the main entry point for system initialization.
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_core::core::BearDogCore;
//!
//! let core = BearDogCore::new();
//! core.initialize()?;
//! ```

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
/// Universal adapter wiring for capability bridging inside the core runtime.
pub mod adapter;
/// HSM and public key persistence hooks (phase-2 key management).
pub mod key_management;
/// Metrics, alerts, and health signals for core components.
pub mod monitoring;
/// Core cryptographic and policy enforcement surface.
pub mod security;
/// Shared core state machine and coordination primitives.
pub mod state;
/// [`BearDogCore`] entrypoint: bootstrap, lifecycle, and subsystem orchestration.
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
