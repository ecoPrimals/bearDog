// SPDX-License-Identifier: AGPL-3.0-only

//! # Unified Universal Adapter System
//!
//! Provides a comprehensive universal adapter architecture enabling seamless
//! integration with diverse ecosystem services, protocols, and platforms through
//! a unified, extensible, and type-safe interface.
//!
//! ## Overview
//!
//! The universal adapter solves the "2^n hardcoding problem" by providing O(1)
//! adapters that work with any service by discovering capabilities dynamically:
//! - No hardcoded service names or endpoints
//! - Capability-based service discovery
//! - Protocol-agnostic communication
//! - Zero-knowledge bootstrap support
//!
//! ## Module Organization
//!
//! - `types` - Core types, enums, and data structures
//! - Configuration structures and defaults
// - `core` - Core adapter implementation
// - `production` - Production-ready adapter with enhanced features
// - `connection` - Connection management and pooling
// - `events` - Event handling and subscription system
// - `metrics` - Metrics collection and reporting

// Public API modules
/// Configuration management
pub mod config;
/// Connection pooling and management
pub mod connection;
/// Core functionality
pub mod core;
/// Event handling and subscriptions
pub mod events;
/// Metrics collection and reporting
pub mod metrics;
/// Production-ready adapter implementation
pub mod production;
/// Core type definitions
pub mod types;

// Re-export main types for backwards compatibility
pub use config::*;
pub use connection::ConnectionPool;
pub use core::UniversalAdapter;
pub use events::{AdapterEvent, EventSubscriber};
pub use metrics::{AdapterMetrics, ConnectionMetrics};
pub use production::ProductionUniversalAdapter;
pub use types::*;

// Re-export canonical connection pool config
pub use beardog_types::canonical::config::domains::network::ConnectionPoolConfig;

#[cfg(test)]
mod tests;
