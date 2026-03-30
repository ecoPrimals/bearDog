// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Health Monitoring
//!
//! This module provides health checking and monitoring for HSMs.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// HSM health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmHealthStatus {
    /// HSM is functioning normally
    Healthy,
    /// HSM is operational but experiencing issues
    Degraded,
    /// HSM is not functioning correctly
    Unhealthy,
    /// HSM health status cannot be determined
    Unknown,
}

/// HSM health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthCheck {
    /// Current health status
    pub status: HsmHealthStatus,
    /// Human-readable status message
    pub message: String,
    /// When the last health check was performed
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Number of errors since last healthy state
    pub error_count: u32,
}

impl Default for HsmHealthCheck {
    fn default() -> Self {
        Self {
            status: HsmHealthStatus::Unknown,
            message: "Not checked yet".to_string(),
            last_check: chrono::Utc::now(),
            error_count: 0,
        }
    }
}

/// HSM health monitor
pub struct HsmHealthMonitor {
    last_check: Option<HsmHealthCheck>,
}

impl HsmHealthMonitor {
    /// Creates a new health monitor
    pub fn new() -> Self {
        info!("🏥 Initializing HSM health monitor");
        Self { last_check: None }
    }

    /// # Errors
    ///
    /// Returns an error if the Tor-related operation fails.
    /// Performs a health check
    pub fn check_health(&mut self, hsm_available: bool) -> Result<HsmHealthCheck, BearDogError> {
        debug!("🔍 Performing HSM health check");

        let status = if hsm_available {
            HsmHealthStatus::Healthy
        } else {
            HsmHealthStatus::Unhealthy
        };

        let check = HsmHealthCheck {
            status: status.clone(),
            message: format!(
                "HSM is {}",
                if hsm_available {
                    "available"
                } else {
                    "unavailable"
                }
            ),
            last_check: chrono::Utc::now(),
            error_count: u32::from(!hsm_available),
        };

        self.last_check = Some(check.clone());

        match status {
            HsmHealthStatus::Healthy => {
                info!("✅ HSM health check: Healthy");
            }
            HsmHealthStatus::Unhealthy => {
                warn!("❌ HSM health check: Unhealthy");
            }
            _ => {}
        }

        Ok(check)
    }

    /// Gets the last health check result
    pub fn get_last_check(&self) -> Option<HsmHealthCheck> {
        self.last_check.clone()
    }
}

impl Default for HsmHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HsmHealthMonitor::new();
        assert!(monitor.get_last_check().is_none());
        Ok(())
    }

    #[test]
    fn test_health_check_healthy() -> Result<(), Box<dyn std::error::Error>> {
        let mut monitor = HsmHealthMonitor::new();
        let check = monitor.check_health(true)?;
        assert_eq!(check.status, HsmHealthStatus::Healthy);
        assert_eq!(check.error_count, 0);
        Ok(())
    }

    #[test]
    fn test_health_check_unhealthy() -> Result<(), Box<dyn std::error::Error>> {
        let mut monitor = HsmHealthMonitor::new();
        let check = monitor.check_health(false)?;
        assert_eq!(check.status, HsmHealthStatus::Unhealthy);
        assert_eq!(check.error_count, 1);
        Ok(())
    }

    #[test]
    fn test_last_check() -> Result<(), Box<dyn std::error::Error>> {
        let mut monitor = HsmHealthMonitor::new();
        monitor.check_health(true)?;

        let last_check = monitor.get_last_check();
        assert!(last_check.is_some());
        assert_eq!(
            last_check.ok_or("last_check not found")?.status,
            HsmHealthStatus::Healthy
        );
        Ok(())
    }
}
