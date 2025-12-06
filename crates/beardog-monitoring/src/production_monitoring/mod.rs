

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod types;
pub mod metrics_collector;
pub mod alert_manager;
pub mod health_checker;
pub mod performance_analyzer;
pub mod security_monitor;
pub mod biome_tracker;
pub mod monitor;

#[allow(unused_imports, clippy::float_cmp, clippy::absurd_extreme_comparisons, unused_comparisons, clippy::nonminimal_bool)]
#[cfg(test)]
mod tests;

pub use types::*;
pub use metrics_collector::MetricsCollector;
pub use alert_manager::AlertManager;
pub use health_checker::HealthChecker;
pub use performance_analyzer::PerformanceAnalyzer;
pub use security_monitor::SecurityMonitor;
pub use biome_tracker::BiomeTracker;
pub use monitor::ProductionMonitor; 
