// **UNIFIED UNIVERSAL ADAPTER SYSTEM** - Modularized Architecture
//
// This module provides a comprehensive universal adapter architecture that enables
// seamless integration with diverse ecosystem services, protocols, and platforms
// through a unified, extensible, and type-safe interface.
//
// ## Module Organization
//
// - `types` - Core types, enums, and data structures
// - `config` - Configuration structures and defaults
// - `core` - Core adapter implementation
// - `production` - Production-ready adapter with enhanced features
// - `connection` - Connection management and pooling
// - `events` - Event handling and subscription system
// - `metrics` - Metrics collection and reporting

// Public API modules
/// Configuration management
/// Configuration management
pub mod config;
pub mod connection;
/// Core functionality
/// Core functionality
pub mod core;
pub mod events;
pub mod metrics;
pub mod production;
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
