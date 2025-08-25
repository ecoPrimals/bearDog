// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info, warn};

use crate::threat::types::{ThreatEvent, ThreatRule, ThreatSeverity, ThreatType};
use beardog_errors::{BearDogError, BearDogResult};

/// **CANONICAL THREAT ANALYSIS ENGINE** - Advanced threat detection and analysis
/// This module provides comprehensive threat detection, analysis, and response
/// capabilities with machine learning enhancement and rule-based detection.

/// Threat detection engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable machine learning enhancement
    pub ml_enhancement: bool,
    /// Maximum number of concurrent threat analyses
    pub max_concurrent_analyses: usize,
    /// Threat detection sensitivity (0.0 to 1.0)
    pub detection_sensitivity: f64,
    /// Enable real-time threat feeds
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

/// Threat detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Total events analyzed
    pub events_analyzed: u64,
    /// Total threats detected
    pub threats_detected: u64,
    /// Number of rules triggered
    pub rules_triggered: u64,
    /// False positives detected
    pub false_positives: u64,
    /// Analysis accuracy percentage
    pub accuracy_percentage: f64,
    /// Last analysis timestamp
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

/// Main threat detection engine
#[derive(Debug)]
pub struct ThreatDetectionEngine {
    /// Configuration settings
    pub config: ThreatDetectionConfig,
    /// Detection rules
    pub detection_rules: Vec<ThreatRule>,
    /// Analysis statistics
    pub stats: ThreatDetectionStats,
    /// Threat signatures database
    pub threat_signatures: HashMap<String, String>,
}

impl ThreatDetectionEngine {
    /// Create a new threat detection engine
    pub fn new(config: ThreatDetectionConfig) -> Self {
        Self {
            config,
            detection_rules: Self::load_default_rules(),
            stats: ThreatDetectionStats::default(),
            threat_signatures: HashMap::new(),
        }
    }

    /// Analyze event data for potential threats
    pub async fn analyze_event(
        &mut self,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<Vec<ThreatEvent>> {
        self.stats.events_analyzed += 1;
        self.stats.last_analysis = Utc::now();
        
        let mut detected_threats = Vec::new();

        // Rule-based detection
        for rule in &self.detection_rules {
            if !rule.enabled {
                continue;
            }
            
            if self.matches_rule(event_data, rule).await? {
                let threat_event = self.create_threat_event(rule, event_data)?;
                detected_threats.push(threat_event);
                // Update rule statistics
                self.stats.rules_triggered += 1;
                info!("Rule '{}' triggered for event", rule.name);
            }
        }
        
        // Apply ML-based detection if enabled
        if self.config.ml_enhancement {
            let ml_threats = self.ml_based_detection(event_data).await?;
            detected_threats.extend(ml_threats);
        }

        // Update detection statistics
        self.stats.threats_detected += detected_threats.len() as u64;
        
        Ok(detected_threats)
    }

    /// Check if event data matches a specific rule
    async fn matches_rule(
        &self,
        event_data: &HashMap<String, String>,
        rule: &ThreatRule,
    ) -> BearDogResult<bool> {
        if !rule.enabled {
            return Ok(false);
        }

        // Check rule conditions
        for condition in &rule.conditions {
            let field_value = event_data.get(&condition.field);
            
            match field_value {
                Some(value) => {
                    if !self.evaluate_condition(value, &condition.operator, &condition.value) {
                        return Ok(false);
                    }
                }
                None => {
                    if condition.required {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, field_value: &str, operator: &str, expected_value: &str) -> bool {
        match operator {
            "equals" => field_value == expected_value,
            "contains" => field_value.contains(expected_value),
            "starts_with" => field_value.starts_with(expected_value),
            "ends_with" => field_value.ends_with(expected_value),
            "regex" => {
                // In a real implementation, compile and cache regex patterns
                field_value.contains(expected_value) // Simplified
            }
            "greater_than" => {
                if let (Ok(field_num), Ok(expected_num)) = (field_value.parse::<f64>(), expected_value.parse::<f64>()) {
                    field_num > expected_num
                } else {
                    false
                }
            }
            "less_than" => {
                if let (Ok(field_num), Ok(expected_num)) = (field_value.parse::<f64>(), expected_value.parse::<f64>()) {
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

    /// Machine learning based threat detection
    async fn ml_based_detection(
        &self,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<Vec<ThreatEvent>> {
        // In a real implementation, this would use trained ML models
        // For now, return empty vector as placeholder
        let _event_data = event_data; // Avoid unused parameter warning
        Ok(Vec::new())
    }

    /// Create a threat event from a triggered rule
    fn create_threat_event(
        &self,
        rule: &ThreatRule,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<ThreatEvent> {
        let threat_event = ThreatEvent {
            id: uuid::Uuid::new_v4().to_string(),
            threat_type: rule.threat_type.clone(),
            severity: rule.severity.clone(),
            confidence: self.calculate_confidence(&rule.severity),
            source: event_data.get("source").unwrap_or(&"unknown".to_string()).clone(),
            target: event_data.get("target").unwrap_or(&"unknown".to_string()).clone(),
            description: format!("Rule '{}' detected: {}", rule.name, rule.description),
            timestamp: Utc::now(),
            raw_data: event_data.clone(),
            mitigated: false,
            mitigation_actions: Vec::new(),
        };

        Ok(threat_event)
    }

    /// Calculate confidence score based on threat severity
    fn calculate_confidence(&self, severity: &ThreatSeverity) -> f64 {
        match severity {
            ThreatSeverity::Critical => 0.95,
            ThreatSeverity::High => 0.85,
            ThreatSeverity::Medium => 0.6,
            ThreatSeverity::Low => 0.4,
            ThreatSeverity::Info => 0.2,
        }
    }

    /// Load default threat detection rules
    fn load_default_rules() -> Vec<ThreatRule> {
        vec![
            ThreatRule {
                id: "failed_login_attempts".to_string(),
                name: "Failed Login Attempts".to_string(),
                description: "Multiple failed login attempts from same source".to_string(),
                enabled: true,
                threat_type: ThreatType::AuthenticationFailure,
                severity: ThreatSeverity::Medium,
                conditions: vec![
                    crate::threat::types::RuleCondition {
                        field: "event_type".to_string(),
                        operator: "equals".to_string(),
                        value: "login_failure".to_string(),
                        required: true,
                    }
                ],
                actions: vec!["log".to_string(), "alert".to_string()],
            },
            ThreatRule {
                id: "suspicious_network_activity".to_string(),
                name: "Suspicious Network Activity".to_string(),
                description: "Unusual network traffic patterns detected".to_string(),
                enabled: true,
                threat_type: ThreatType::NetworkIntrusion,
                severity: ThreatSeverity::High,
                conditions: vec![
                    crate::threat::types::RuleCondition {
                        field: "event_type".to_string(),
                        operator: "equals".to_string(),
                        value: "network_anomaly".to_string(),
                        required: true,
                    }
                ],
                actions: vec!["log".to_string(), "alert".to_string(), "block".to_string()],
            },
        ]
    }

    /// Add a new detection rule
    pub fn add_rule(&mut self, rule: ThreatRule) -> BearDogResult<()> {
        // Check for duplicate rule IDs
        if self.detection_rules.iter().any(|r| r.id == rule.id) {
            return Err(BearDogError::configuration(
                format!("Rule with ID '{}' already exists", rule.id)
            ));
        }

        self.detection_rules.push(rule);
        info!("Added new threat detection rule");
        Ok(())
    }

    /// Remove a detection rule
    pub fn remove_rule(&mut self, rule_id: &str) -> BearDogResult<()> {
        let initial_len = self.detection_rules.len();
        self.detection_rules.retain(|rule| rule.id != rule_id);
        
        if self.detection_rules.len() == initial_len {
            return Err(BearDogError::configuration(
                format!("Rule with ID '{}' not found", rule_id)
            ));
        }

        info!("Removed threat detection rule: {}", rule_id);
        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    /// Update threat signatures database
    pub async fn update_threat_signatures(&mut self, signatures: HashMap<String, String>) -> BearDogResult<()> {
        self.threat_signatures.extend(signatures);
        info!("Updated threat signatures database with {} entries", self.threat_signatures.len());
        Ok(())
    }
}

impl Default for ThreatDetectionEngine {
    fn default() -> Self {
        Self::new(ThreatDetectionConfig::default())
    }
}
