// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core System State Tracking
//!
//! Maintains the current state of all `BearDog` system components.

use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::collections::HashMap;

/// Core system state tracking
///
/// Maintains the current state of all `BearDog` system components including
/// health status, component registry, and system uptime information.
#[derive(Debug, Clone)]
pub struct CoreState {
    /// Status of individual system components
    /// Mapping of components
    pub components: HashMap<String, ComponentStatus>,
    /// Overall system health status
    /// The overall health value
    pub overall_health: HealthStatus,
    /// System start time for uptime calculation
    pub start_time: std::time::Instant,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            components: HashMap::with_capacity(10),
            overall_health: HealthStatus::Healthy,
            start_time: std::time::Instant::now(),
        }
    }
}
