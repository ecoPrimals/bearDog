// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hybrid intelligence core: runtime types, system orchestration, and the rich system builder.
//!
//! This module is split by responsibility:
//! - `types` — events, metrics, prediction/decision results, commands, and status snapshots
//! - `system` — `HybridIntelligenceSystem` lifecycle, prediction, decisions, and monitoring
//! - `builder` — `HybridIntelligenceBuilder` for assembling a system with defaults

mod builder;
mod system;
mod types;

pub use builder::HybridIntelligenceBuilder;
pub use system::HybridIntelligenceSystem;
pub use types::{
    DecisionContext, DecisionResult, IntelligenceEvent, IntelligenceEventType, IntelligenceMetrics,
    PredictionResult, SystemCommand, SystemStatus,
};

#[cfg(test)]
#[path = "../core_tests.rs"]
mod core_tests;
