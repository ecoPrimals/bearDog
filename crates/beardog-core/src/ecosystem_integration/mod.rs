// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod ecosystem_genetic_spawner;
pub mod integration_engine;
pub mod license_manager;
pub mod performance_optimizer;
pub mod types;
pub mod universal_adapter;
pub mod universal_compute_client;

pub use integration_engine::*;
pub use license_manager::*;
// Avoid ambiguous re-exports by aliasing conflicting types
pub use performance_optimizer::{
    CapabilityConnectionPool, ComputeCache, ConnectionMetrics as PerformanceConnectionMetrics,
    EcosystemPerformanceOptimizer, PoolConfig as PerformancePoolConfig,
};
pub use types::*;
pub use universal_adapter::*;
pub use universal_compute_client::*;
