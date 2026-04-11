// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(clippy::needless_doctest_main)]

//! Management extensions: rule CRUD, health probes, maintenance, and aggregate stats.

use super::core::ThreatDetectionEngine;
use crate::threat::types::engine::threat_engine::ThreatDetectionStats;
use crate::threat::types::{
    DetectionRule, ResponseStatus, RuleCondition, ThreatRuleType, ThreatSeverity,
};
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused SystemTime import
use tracing::info;

const MAX_THREAT_HISTORY_ENTRIES: usize = 5000;
const THREAT_HISTORY_DRAIN_COUNT: usize = 1000;

/// Rolled-up health for the threat management subsystem and its components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system status
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Individual component health states
    /// Mapping of components
    pub components: HashMap<String, HealthStatus>,
    /// Health check timestamp
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Health assessment details
    /// The details value
    pub details: String,
}
impl ThreatDetectionEngine {
    /// Add Detection Rule operation.
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        info!("Adding detection rule: {} - {}", rule.id, rule.name);
        self.detection_rules.push(rule);
    }

    /// Remove Detection Rule operation.
    /// Removes `detection_rule`
    pub fn remove_detection_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        let removed = self.detection_rules.len() < initial_len;
        if removed {
            info!("Removed detection rule: {}", rule_id);
        }
        removed
    }

    /// Enable Detection Rule operation.
    pub fn enable_detection_rule(&mut self, rule_id: &str, enabled: bool) -> bool {
        for rule in &mut self.detection_rules {
            if rule.id == rule_id {
                rule.enabled = enabled;
                let status = if enabled { "enabled" } else { "disabled" };
                info!("Detection rule {} {}", rule_id, status);
                return true;
            }
        }
        false
    }

    /// Get Statistics operation.
    /// Gets statistics
    #[must_use]
    pub const fn get_statistics(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    /// Reset Statistics operation.
    pub fn reset_statistics(&mut self) {
        self.stats = ThreatDetectionStats::default();
    }

    /// Update Detection Rule operation.
    /// Updates `detection_rule`
    pub fn update_detection_rule(&mut self, rule_id: &str, updated_rule: DetectionRule) -> bool {
        for rule in &mut self.detection_rules {
            if rule.id == rule_id {
                *rule = updated_rule;
                info!("Updated detection rule: {}", rule_id);
                return true;
            }
        }
        false
    }

    /// Load Default Rules operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Loads `default_rules`
    pub fn load_default_rules(&mut self) -> Result<(), BearDogError> {
        let brute_force_rule = DetectionRule {
            name: "Brute Force Attack Detection".to_string(),
            description: "Detects multiple failed login attempts from the same source".to_string(),
            severity: ThreatSeverity::High,
            id: "brute_force_detection".to_string(),
            rule_type: ThreatRuleType::Signature,
            condition: RuleCondition::FieldGreaterThan {
                field: "failed_attempts".to_string(),
                threshold: 5.0,
            },
            ..Default::default()
        };

        let anomaly_rule = DetectionRule {
            name: "Network Anomaly Detection".to_string(),
            description: "Detects unusual network traffic patterns".to_string(),
            severity: ThreatSeverity::Medium,
            id: "network_anomaly_detection".to_string(),
            rule_type: ThreatRuleType::Anomaly,
            condition: RuleCondition::FieldGreaterThan {
                field: "anomaly_score".to_string(),
                threshold: 0.8,
            },
            ..Default::default()
        };

        let default_rules = vec![brute_force_rule, anomaly_rule];

        for rule in default_rules {
            self.detection_rules.push(rule);
        }

        info!(
            "Loaded {} default detection rules",
            self.detection_rules.len()
        );
        Ok(())
    }

    /// Get System Health operation.
    /// Gets `system_health`
    #[must_use]
    pub fn get_system_health(&self) -> SystemHealth {
        let mut component_status = HashMap::new();

        // Add component health based on system state
        let rules_health = if self.detection_rules.iter().any(|r| r.enabled) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
        component_status.insert("detection_rules".to_string(), rules_health);

        let threats_health = if self.active_threats.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
        component_status.insert("threat_status".to_string(), threats_health);

        SystemHealth {
            overall_status: if self.active_threats.is_empty()
                && self.detection_rules.iter().any(|r| r.enabled)
            {
                HealthStatus::Healthy
            } else {
                HealthStatus::Degraded
            },
            components: component_status,
            last_check: chrono::Utc::now(),
            details: "Threat management system health check".to_string(),
        }
    }

    /// Trims historical events and completed incidents to bound memory usage.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn perform_maintenance(&mut self) -> Result<(), BearDogError> {
        info!("Starting system maintenance");

        {
            let mut history = self.event_history.write().await;
            if history.len() > MAX_THREAT_HISTORY_ENTRIES {
                history.drain(0..THREAT_HISTORY_DRAIN_COUNT);
                info!("Cleaned up old event history");
            }
        }

        {
            let mut incidents = self.active_incidents.write().await;
            let initial_count = incidents.len();
            incidents.retain(|_, incident| incident.status != ResponseStatus::Completed);
            let removed = initial_count - incidents.len();
            if removed > 0 {
                info!("Cleaned up {} resolved incidents", removed);
            }
        }

        Ok(())
    }

    /// Get system status by reading host metrics from `/proc` (Linux).
    #[must_use]
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus::from_proc()
    }
}

/// Host metrics read from `/proc` on Linux, with safe fallbacks on other platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System uptime from `/proc/uptime`.
    pub system_uptime: String,
    /// Memory usage summary from `/proc/meminfo`.
    pub memory_usage: String,
    /// CPU usage summary from `/proc/stat` (idle percentage).
    pub cpu_usage: String,
}

impl SystemStatus {
    /// Read host metrics from `/proc` (Linux) or return "unavailable" on other platforms.
    #[must_use]
    fn from_proc() -> Self {
        Self {
            system_uptime: Self::read_uptime(),
            memory_usage: Self::read_memory(),
            cpu_usage: Self::read_cpu(),
        }
    }

    fn read_uptime() -> String {
        std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next().map(String::from))
            .map_or_else(|| "unavailable".to_string(), |secs| format!("{secs}s"))
    }

    fn read_memory() -> String {
        std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|content| {
                let mut total_kb = 0u64;
                let mut avail_kb = 0u64;
                for line in content.lines() {
                    if let Some(rest) = line.strip_prefix("MemTotal:") {
                        total_kb = rest
                            .split_whitespace()
                            .next()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0);
                    } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
                        avail_kb = rest
                            .split_whitespace()
                            .next()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0);
                    }
                }
                if total_kb > 0 {
                    let used_mb = (total_kb.saturating_sub(avail_kb)) / 1024;
                    let total_mb = total_kb / 1024;
                    Some(format!("{used_mb}/{total_mb} MB"))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unavailable".to_string())
    }

    fn read_cpu() -> String {
        std::fs::read_to_string("/proc/stat")
            .ok()
            .and_then(|content| {
                let cpu_line = content.lines().next()?;
                let vals: Vec<u64> = cpu_line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|v| v.parse().ok())
                    .collect();
                if vals.len() >= 4 {
                    let total: u64 = vals.iter().sum();
                    let idle = vals[3];
                    if total > 0 {
                        let usage = 100u64.saturating_sub(idle * 100 / total);
                        return Some(format!("{usage}%"));
                    }
                }
                None
            })
            .unwrap_or_else(|| "unavailable".to_string())
    }
}

/// Point-in-time counts surfaced by management dashboards.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatStats {
    /// Number of `total_rules`
    pub total_rules: usize,
    /// Number of `active_threats`
    pub active_threats: usize,
    /// Number of `blocked_sources`
    pub blocked_sources: usize,
    /// Number of `quarantined_systems`
    pub quarantined_systems: usize,
    /// Number of `threat_feeds`
    pub threat_feeds: usize,
    /// Number of `ml_models`
    pub ml_models: usize,
    /// Number of `events_processed`
    pub events_processed: usize,
}
