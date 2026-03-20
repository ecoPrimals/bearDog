// SPDX-License-Identifier: AGPL-3.0-only

// **ADVANCED METRICS SYSTEM** - Modularized Architecture
//
// This module provides a comprehensive advanced metrics system that enables
// detailed performance, security, and ecosystem monitoring through a unified,
// extensible, and type-safe interface.
//
// ## Module Organization
//
// - `types` - Core metric types, enums, and data structures
// - `config` - Configuration structures and defaults
// - `core` - Core metrics system implementation
// - `performance` - Performance analysis and monitoring
// - `security` - Security metrics and threat detection
// - `ecosystem` - Ecosystem health monitoring
// - `analysis` - Anomaly detection and trend analysis
// - `storage` - Metrics storage and retrieval

// Public API modules
/// Statistical anomaly detection and trend analysis over metric streams.
pub mod analysis;
/// Configuration for collection intervals, retention, and feature toggles.
pub mod config;
/// Main [`AdvancedMetricsSystem`] implementation (record, summarize, subscribe).
pub mod core;
/// Cross-service ecosystem health signals and interaction metrics.
pub mod ecosystem;
/// Performance analyzer hook for interpreting [`crate::advanced_metrics::types::PerformanceMetric`] data.
pub mod performance;
/// Security-oriented counters and event aggregation.
pub mod security;
/// In-memory storage for performance, security, ecosystem, and custom metrics.
pub mod storage;
/// Shared types for advanced metrics (events, enums, summaries).
pub mod types;

// Re-export main types for backwards compatibility
pub use analysis::{AnomalyDetector, TrendAnalyzer};
pub use config::*;
pub use core::AdvancedMetricsSystem;
pub use ecosystem::EcosystemHealthMonitor;
pub use performance::PerformanceAnalyzer;
pub use security::SecurityMetrics;
pub use storage::MetricsStore;
pub use types::*;
