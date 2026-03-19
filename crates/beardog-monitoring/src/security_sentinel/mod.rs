// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySentinelConfig {
    /// Whether security monitoring is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Maximum number of events to track
    /// Number of `max_events`
    pub max_events: usize,
    /// Event retention period in hours
    /// Number of `retention_hours`
    pub retention_hours: u64,
    /// Whether to log security events
    /// Whether `log_events` is enabled
    pub log_events: bool,
    /// Number of `alert_threshold`
    pub alert_threshold: u32,
    /// Whether to enable real-time monitoring
    pub real_time_monitoring: bool,
    /// Monitoring interval in seconds
    /// Number of `monitoring_interval_seconds`
    pub monitoring_interval_seconds: u64,
}

impl Default for SecuritySentinelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_events: 10000,
            retention_hours: 24,
            log_events: true,
            alert_threshold: 10,
            real_time_monitoring: true,
            monitoring_interval_seconds: 60,
        }
    }
}

/// `SecuritySentinelStats` tracks security monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySentinelStats {
    /// Total number of security events processed
    /// Number of `total_events`
    pub total_events: u64,
    /// Number of authentication failures
    /// Number of `auth_failures`
    pub auth_failures: u64,
    /// Number of access violations
    /// Number of `access_violations`
    pub access_violations: u64,
    /// Number of compliance violations
    /// Number of `compliance_violations`
    pub compliance_violations: u64,
    /// Number of suspicious activities detected
    /// Number of `suspicious_activities`
    pub suspicious_activities: u64,
    /// Number of blocked requests
    /// Number of `blocked_requests`
    pub blocked_requests: u64,
    /// Last event timestamp
    pub last_event_time: Option<DateTime<Utc>>,
    /// Monitoring start time
    pub monitoring_start_time: DateTime<Utc>,
}

impl Default for SecuritySentinelStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            auth_failures: 0,
            access_violations: 0,
            compliance_violations: 0,
            suspicious_activities: 0,
            blocked_requests: 0,
            last_event_time: None,
            monitoring_start_time: Utc::now(),
        }
    }
}

/// `SecurityStatusReport` provides current security status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatusReport {
    /// Current monitoring status
    /// Current status of the component
    pub status: String,
    /// Security statistics
    /// The stats value
    pub stats: SecuritySentinelStats,
    /// Overall health status
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Report generation timestamp
    pub timestamp: DateTime<Utc>,
}

/// `SecuritySentinel` provides comprehensive security monitoring
#[derive(Debug)]
pub struct SecuritySentinel {
    config: SecuritySentinelConfig,
    stats: Arc<RwLock<SecuritySentinelStats>>,
    monitoring_active: Arc<AtomicBool>,
    event_counter: Arc<AtomicUsize>,
}

impl SecuritySentinel {
    /// Creates a new `SecuritySentinel` instance
    #[must_use]
    /// Creates a new instance
    pub fn new(config: SecuritySentinelConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(SecuritySentinelStats::default())),
            monitoring_active: Arc::new(AtomicBool::new(false)),
            event_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Starts security monitoring
    ///
    /// # Errors
    /// Returns an error if monitoring cannot be started
    /// Starts monitoring
    /// Starts monitoring
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        self.monitoring_active.store(true, Ordering::Relaxed);
        tracing::info!("Security monitoring started");
        Ok(())
    }

    /// Stops security monitoring
    ///
    /// # Errors
    /// Returns an error if monitoring cannot be stopped
    /// Stops monitoring
    /// Stops monitoring
    pub fn stop_monitoring(&self) -> Result<(), BearDogError> {
        self.monitoring_active.store(false, Ordering::Relaxed);
        tracing::info!("Security monitoring stopped");
        Ok(())
    }

    /// Processes a security event
    ///
    /// # Errors
    /// Returns an error if the event cannot be processed
    /// Processes `security_event`
    /// Processes `security_event`
    pub async fn process_security_event(
        &self,
        event_type: &str,
        _event_data: HashMap<&str, &str>,
    ) -> Result<(), BearDogError> {
        let _event_count = self.event_counter.fetch_add(1, Ordering::Relaxed);

        {
            let mut stats = self.stats.write().await;
            stats.total_events += 1;
            stats.last_event_time = Some(Utc::now());

            match event_type {
                "auth_failure" => {
                    stats.auth_failures += 1;
                }
                "access_violation" => {
                    stats.access_violations += 1;
                }
                "compliance_violation" => {
                    stats.compliance_violations += 1;
                }
                _ => {
                    stats.suspicious_activities += 1;
                }
            }
        } // stats lock is dropped here

        Ok(())
    }

    /// Gets the current security statistics
    ///
    /// # Errors
    /// Returns an error if statistics cannot be retrieved
    pub async fn get_statistics(&self) -> SecuritySentinelStats {
        self.stats.read().await.clone()
    }

    /// Gets a comprehensive security status report
    ///
    /// # Errors
    /// Returns an error if the status report cannot be generated
    /// Gets `status_report`
    /// Gets `status_report`
    pub async fn get_status_report(&self) -> Result<SecurityStatusReport, BearDogError> {
        let stats = self.stats.read().await.clone();
        let status = if self.monitoring_active.load(Ordering::Relaxed) {
            "ACTIVE"
        } else {
            "INACTIVE"
        };

        let health_status = if stats.compliance_violations > 0
            || stats.suspicious_activities > self.config.alert_threshold.into()
        {
            HealthStatus::Unhealthy
        } else if stats.auth_failures > 5 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        Ok(SecurityStatusReport {
            status: status.to_string(),
            stats,
            health_status,
            timestamp: Utc::now(),
        })
    }

    /// Checks if monitoring is currently active
    #[must_use]
    /// Checks if monitoring active
    /// Checks if monitoring active
    pub fn is_monitoring_active(&self) -> bool {
        self.monitoring_active.load(Ordering::Relaxed)
    }

    /// Gets the current event count
    #[must_use]
    /// Gets `event_count`
    /// Gets `event_count`
    pub fn get_event_count(&self) -> usize {
        self.event_counter.load(Ordering::Relaxed)
    }
}

impl Default for SecuritySentinel {
    fn default() -> Self {
        Self::new(SecuritySentinelConfig::default())
    }
}

pub mod performance_metrics;
pub mod system_metrics;

#[cfg(test)]
mod comprehensive_tests;
