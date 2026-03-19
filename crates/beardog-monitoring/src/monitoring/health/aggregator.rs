// SPDX-License-Identifier: AGPL-3.0-only

//! Health Check Aggregator
//!
//! Aggregates health checks from multiple components and provides overall system health status.

use super::super::types::ComponentHealth;
use super::checkers::HealthCheckerType;
use super::traits::HealthChecker;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;

/// Aggregates health checks from multiple components
///
/// The aggregator runs health checks on all registered components
/// and provides an overall health status for the system.
pub struct HealthCheckAggregator {
    checkers: Vec<HealthCheckerType>,
}

impl Default for HealthCheckAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthCheckAggregator {
    /// Creates a new health check aggregator
    #[must_use]
    pub const fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    /// Adds a health checker to the aggregator
    pub fn add_checker(&mut self, checker: HealthCheckerType) {
        self.checkers.push(checker);
    }

    /// Runs all health checks and returns results
    ///
    /// # Errors
    ///
    /// Returns error if unable to collect health check results
    pub async fn check_all(&self) -> Result<Vec<ComponentHealth>, BearDogError> {
        let mut results = Vec::new();

        for checker in &self.checkers {
            match checker.check_health().await {
                Ok(health) => results.push(health),
                Err(e) => {
                    // Record failed health check instead of propagating error
                    results.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {e}")),
                        last_check: Utc::now(),
                        check_duration_ms: 0,
                        metadata: HashMap::with_capacity(16),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Gets overall system health status
    ///
    /// Returns `Unhealthy` if any component is unhealthy, `Healthy` if all are healthy.
    ///
    /// # Errors
    ///
    /// Returns error if unable to check component health
    pub async fn get_overall_status(&self) -> Result<HealthStatus, BearDogError> {
        let results = self.check_all().await?;

        for result in &results {
            if result.status == HealthStatus::Unhealthy {
                return Ok(HealthStatus::Unhealthy);
            }
        }

        Ok(HealthStatus::Healthy)
    }
}
