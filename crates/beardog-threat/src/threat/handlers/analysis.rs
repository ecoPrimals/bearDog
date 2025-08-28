use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

use crate::threat::types::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    pub ml_enhancement: bool,

    pub max_concurrent_analyses: usize,

    pub detection_sensitivity: f64,

    pub enable_threat_feeds: bool,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            ml_enhancement: true,
            max_concurrent_analyses: 10,
            detection_sensitivity: 0.7,
            enable_threat_feeds: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    pub events_analyzed: u64,

    pub threats_detected: u64,

    pub rules_triggered: u64,

    pub false_positives: u64,

    pub accuracy_percentage: f64,

    pub last_analysis: DateTime<Utc>,
}

impl Default for ThreatDetectionStats {
    fn default() -> Self {
        Self {
            events_analyzed: 0,
            threats_detected: 0,
            rules_triggered: 0,
            false_positives: 0,
            accuracy_percentage: 0.0,
            last_analysis: Utc::now(),
        }
    }
}

#[derive(Debug)]
pub struct ThreatDetectionEngine {
    pub config: ThreatDetectionConfig,

    pub detection_rules: Vec<DetectionRule>,

    pub stats: ThreatDetectionStats,

    pub threat_signatures: HashMap<String, String>,
}

impl ThreatDetectionEngine {
    pub fn new(config: ThreatDetectionConfig) -> Self {
        Self {
            config,
            detection_rules: Self::load_default_rules(),
            stats: ThreatDetectionStats::default(),
            threat_signatures: HashMap::with_capacity(16),
        }
    }

    pub async fn analyze_event(
        &mut self,
        event_data: &HashMap<&str, &str>,
    ) -> Result<Vec<ThreatEvent>, BearDogError> {
        self.stats.events_analyzed += 1;
        self.stats.last_analysis = Utc::now();

        let mut detected_threats = Vec::new();

        for rule in &self.detection_rules {
            if !rule.enabled {
                continue;
            }

            // Evaluate the rule condition against the event data
            if !rule.condition.evaluate(event_data) {
                continue;
            }

            let threat_event = self.create_threat_event(rule, event_data)?;
            detected_threats.push(threat_event);

            self.stats.rules_triggered += 1;
            info!("Rule '{}' triggered for event", rule.name);
        }

        if self.config.ml_enhancement {
            let ml_threats = self.ml_based_detection(event_data).await?;
            detected_threats.extend(ml_threats);
        }

        self.stats.threats_detected += detected_threats.len() as u64;

        Ok(detected_threats)
    }

    #[allow(dead_code)]
    fn evaluate_condition(&self, field_value: &str, operator: &str, expected_value: &str) -> bool {
        match operator {
            "equals" => field_value == expected_value,
            "contains" => field_value.contains(expected_value),
            "starts_with" => field_value.starts_with(expected_value),
            "ends_with" => field_value.ends_with(expected_value),
            "regex" => {
                field_value.contains(expected_value) // Simplified
            }
            "greater_than" => {
                if let (Ok(field_num), Ok(expected_num)) =
                    (field_value.parse::<f64>(), expected_value.parse::<f64>())
                {
                    field_num > expected_num
                } else {
                    false
                }
            }
            "less_than" => {
                if let (Ok(field_num), Ok(expected_num)) =
                    (field_value.parse::<f64>(), expected_value.parse::<f64>())
                {
                    field_num < expected_num
                } else {
                    false
                }
            }
            _ => {
                warn!("Unknown operator: {}", operator);
                false
            }
        }
    }

    async fn ml_based_detection(
        &self,
        event_data: &HashMap<&str, &str>,
    ) -> Result<Vec<ThreatEvent>, BearDogError> {
        let _event_data = event_data; // Avoid unused parameter warning
        Ok(Vec::new())
    }

    fn create_threat_event(
        &self,
        rule: &DetectionRule,
        event_data: &HashMap<&str, &str>,
    ) -> Result<ThreatEvent, BearDogError> {
        let threat_event = ThreatEvent {
            id: uuid::Uuid::new_v4().to_string(),
            threat_type: rule.threat_type.clone(),
            severity: rule.severity.clone(),
            score: (self.calculate_confidence(&rule.severity) * 100.0) as u8,
            timestamp: Utc::now(),
            source: ThreatSource::default(), // Will need proper source construction
            target: ThreatTarget::default(), // Will need proper target construction
            description: format!("Rule '{}' detected: {}", rule.name, rule.description),
            detection_method: DetectionMethod::RuleBased,
            evidence: Vec::new(),
            recommended_actions: Vec::new(),
            status: ThreatStatus::Active,
            assigned_analyst: None,
            related_events: Vec::new(),
            mitigation_steps: Vec::new(),
            confidence: self.calculate_confidence(&rule.severity),
            raw_data: Some(format!("{event_data:?}")),
            mitigated: false,
            mitigation_actions: Vec::new(),
        };

        Ok(threat_event)
    }

    fn calculate_confidence(&self, severity: &ThreatSeverity) -> f64 {
        match severity {
            ThreatSeverity::Critical => 0.95,
            ThreatSeverity::High => 0.85,
            ThreatSeverity::Medium => 0.6,
            ThreatSeverity::Low => 0.4,
            ThreatSeverity::Info => 0.2,
        }
    }

    fn load_default_rules() -> Vec<DetectionRule> {
        vec![
            DetectionRule::simple(
                "failed_login_attempts",
                "Failed Login Attempts",
                "Multiple failed login attempts from same source",
                ThreatType::AuthenticationFailure,
                ThreatSeverity::Medium,
                RuleCondition::FieldEquals {
                    field: "event_type".to_string(),
                    value: "login_failure".to_string(),
                },
                vec!["log".to_string(), "alert".to_string()],
            ),
            DetectionRule::simple(
                "suspicious_network_activity",
                "Suspicious Network Activity",
                "Unusual network traffic patterns detected",
                ThreatType::NetworkIntrusion,
                ThreatSeverity::High,
                RuleCondition::FieldEquals {
                    field: "event_type".to_string(),
                    value: "network_anomaly".to_string(),
                },
                vec!["log".to_string(), "alert".to_string(), "block".to_string()],
            ),
        ]
    }

    pub fn add_rule(&mut self, rule: DetectionRule) -> Result<(), BearDogError> {
        if self.detection_rules.iter().any(|r| r.id == rule.id) {
            return Err(BearDogError::configuration(
                format_args!("Rule with ID '{}' already exists", rule.id).to_string(),
            ));
        }

        self.detection_rules.push(rule);
        info!("Added new threat detection rule");
        Ok(())
    }

    pub fn remove_rule(&mut self, rule_id: &str) -> Result<(), BearDogError> {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);

        if self.detection_rules.len() == initial_len {
            return Err(BearDogError::configuration(
                format_args!("Rule with ID '{rule_id}' not found").to_string(),
            ));
        }

        info!("Removed threat detection rule: {}", rule_id);
        Ok(())
    }

    pub fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    pub async fn update_threat_signatures(
        &mut self,
        signatures: HashMap<&str, &str>,
    ) -> Result<(), BearDogError> {
        let string_signatures: HashMap<String, String> = signatures
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.threat_signatures.extend(string_signatures);
        info!(
            "Updated threat signatures database with {} entries",
            self.threat_signatures.len()
        );
        Ok(())
    }
}

impl Default for ThreatDetectionEngine {
    fn default() -> Self {
        Self::new(ThreatDetectionConfig::default())
    }
}
