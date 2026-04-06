// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration types for the hybrid intelligence system

mod deployment;
mod metrics_alerting;
mod monitoring;
mod optimizer;
mod preprocessing;
mod registry;
mod serving;
mod training_config;

pub use deployment::*;
pub use metrics_alerting::*;
pub use monitoring::*;
pub use optimizer::*;
pub use preprocessing::*;
pub use registry::*;
pub use serving::*;
pub use training_config::*;

// ═══════════════════════════════════════════════════════════════════════════
// REMOVED: Deprecated type aliases (Nov 8, 2025)
// ═══════════════════════════════════════════════════════════════════════════
//
// The following deprecated type aliases were removed as they had zero usage:
// - LoggingConfig → use beardog_types::canonical::config::domains::system::LoggingConfig
// - LogLevel → use beardog_types::canonical::config::domains::system::LogLevel
// - LogFormat → use beardog_types::canonical::config::domains::system::LogFormat
// - LogDestination → use beardog_types::canonical::config::domains::system::LogTargetType
//
// All types are available at their canonical locations in beardog-types.
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests;
