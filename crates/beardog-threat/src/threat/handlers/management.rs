#![allow(clippy::needless_doctest_main)]

use super::core::ThreatDetectionEngine;
use crate::threat::types::engine::rules::ThreatRuleType;
use crate::threat::types::*;
use beardog_errors::BearDogError;
use tracing::info;
impl ThreatDetectionEngine {
    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        info!("Adding detection rule: {} - {}", rule.id, rule.name);
        self.detection_rules.push(rule);
    }

    pub fn remove_detection_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        let removed = self.detection_rules.len() < initial_len;
        if removed {
            info!("Removed detection rule: {}", rule_id);
        }
        removed
    }

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

    pub fn get_statistics(&self) -> ThreatDetectionStats {
        self.stats.clone()
    }

    pub fn reset_statistics(&mut self) {
        self.stats = ThreatDetectionStats::default();
        info!("Threat detection statistics reset");
    }

    pub fn get_detection_rules(&self) -> &Vec<DetectionRule> {
        &self.detection_rules
    }

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

    pub async fn load_default_rules(&mut self) -> Result<(), BearDogError> {
        let mut brute_force_rule = DetectionRule::new(
            "Brute Force Attack Detection",
            "Detects multiple failed login attempts from the same source",
            ThreatRuleType::Behavioral,
            ThreatSeverity::High,
            "failed_attempts > 5",
        );
        brute_force_rule.id = "brute_force_detection".to_string();
        brute_force_rule.threat_type = ThreatType::BruteForce;
        brute_force_rule.condition = RuleCondition::FieldGreaterThan {
            field: "failed_attempts".to_string(),
            value: "5".to_string(),
        };

        let mut anomaly_rule = DetectionRule::new(
            "Network Anomaly Detection",
            "Detects unusual network traffic patterns",
            ThreatRuleType::Statistical,
            ThreatSeverity::Medium,
            "network_anomaly_score > 0.8",
        );
        anomaly_rule.id = "network_anomaly_detection".to_string();
        anomaly_rule.threat_type = ThreatType::Anomaly;
        anomaly_rule.condition = RuleCondition::FieldGreaterThan {
            field: "anomaly_score".to_string(),
            value: "0.8".to_string(),
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

    pub fn get_system_health(&self) -> SystemHealth {
        SystemHealth {
            active_rules: self.detection_rules.iter().filter(|r| r.enabled).count(),
            total_rules: self.detection_rules.len(),
            active_threats: self.active_threats.len(),
            blocked_sources: self.blocked_sources.len(),
            quarantined_systems: self.quarantined_systems.len(),
            threat_feeds: self.threat_feeds.len(),
            ml_models: self.ml_models.len(),
            events_processed: self.stats.total_threats as usize,
            system_uptime: "N/A".to_string(), // Would be calculated from start time
            memory_usage: "N/A".to_string(),  // Would be calculated from system metrics
            cpu_usage: "N/A".to_string(),     // Would be calculated from system metrics
        }
    }

    pub async fn perform_maintenance(&mut self) -> Result<(), BearDogError> {
        info!("Starting system maintenance");

        {
            let mut history = self.event_history.write().await;
            if history.len() > 5000 {
                history.drain(0..1000);
                info!("Cleaned up old event history");
            }
        }

        {
            let mut incidents = self.active_incidents.write().await;
            let initial_count = incidents.len();
            incidents.retain(|_, incident| incident.status != IncidentStatus::Resolved);
            let removed = initial_count - incidents.len();
            if removed > 0 {
                info!("Cleaned up {} resolved incidents", removed);
            }
        }

        for _rule in &mut self.detection_rules {}

        info!("System maintenance completed");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SystemHealth {
    pub active_rules: usize,

    pub total_rules: usize,

    pub active_threats: usize,

    pub blocked_sources: usize,

    pub quarantined_systems: usize,

    pub threat_feeds: usize,

    pub ml_models: usize,

    pub events_processed: usize,

    pub system_uptime: String,

    pub memory_usage: String,

    pub cpu_usage: String,
}
