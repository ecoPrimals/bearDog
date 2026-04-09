// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health Check Traits
//!
//! Core trait definitions for health checking functionality across the system.

use super::super::types::ComponentHealth;
use beardog_errors::BearDogError;

/// Health checker trait for component health monitoring
///
/// Implement this trait for any component that needs health monitoring.
/// The async design allows for I/O operations during health checks.
#[expect(
    async_fn_in_trait,
    reason = "health checks perform async I/O; lint retained until async traits stabilize"
)]
pub trait HealthChecker: Send + Sync {
    /// Check the health of this component
    ///
    /// # Errors
    ///
    /// Returns error if health check fails or cannot be performed
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError>;

    /// Get the name of this component
    fn component_name(&self) -> &str;
}
