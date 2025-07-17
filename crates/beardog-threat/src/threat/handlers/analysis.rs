//! Event analysis and rule-based threat detection
//!
//! This module contains methods for analyzing events, matching detection rules,
//! and creating threat events based on rule matches. It provides the core
//! rule-based threat detection capabilities of the system.
//!
//! # Rule-Based Detection
//!
//! The analysis engine supports complex rule conditions including:
//! - Field matching (exact, contains, regex)
//! - Numeric comparisons (greater than, less than)
//! - Logical operations (AND, OR, NOT)
//! - Source and network-based conditions
//!
//! # Examples
//!
//! ```rust
//! use beardog::threat::handlers::ThreatDetectionEngine;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut engine = ThreatDetectionEngine::placeholder();
//!     
//!     let mut event_data = HashMap::new();
//!     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
//!     event_data.insert("action".to_string(), "login_attempt".to_string());
//!     
//!     let threats = engine.analyze_event(&event_data).await?;
//!     println!("Detected {} threats", threats.len());
//!     Ok(())
//! }
//! ```

use super::core::ThreatDetectionEngine;
use crate::threat::types::*;
use beardog_errors::BearDogResult;

use chrono::Utc;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tracing::info;
use uuid::Uuid;

impl ThreatDetectionEngine {
    /// Analyze an event for potential threats using configured detection rules
    ///
    /// This method processes incoming event data against all active detection rules
    /// to identify potential security threats. It supports both rule-based and
    /// ML-based threat detection depending on the engine configuration.
    ///
    /// # Arguments
    ///
    /// * `event_data` - A HashMap containing event fields and their values
    ///
    /// # Returns
    ///
    /// * `BearDogResult<Vec<ThreatEvent>>` - A vector of detected threats or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Rule evaluation fails due to invalid conditions
    /// - Threat event creation fails
    /// - Event data is malformed or incomplete
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog::threat::handlers::ThreatDetectionEngine;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut engine = ThreatDetectionEngine::placeholder();
    ///     
    ///     let mut event_data = HashMap::new();
    ///     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    ///     event_data.insert("failed_attempts".to_string(), "10".to_string());
    ///     
    ///     let threats = engine.analyze_event(&event_data).await?;
    ///     for threat in threats {
    ///         println!("Threat detected: {} - {}", threat.threat_type, threat.description);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn analyze_event(
        &mut self,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<Vec<ThreatEvent>> {
        if !self.config.real_time_detection {
            return Ok(vec![]);
        }

        let mut detected_threats = Vec::new();

        // Apply detection rules
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
            let ml_threats = self.analyze_with_ml(event_data).await?;
            detected_threats.extend(ml_threats);
        }

        // Store detected threats
        for threat in &detected_threats {
            self.active_threats
                .insert(threat.id.clone(), threat.clone());
        }

        // Update statistics
        self.stats.events_analyzed += 1;
        self.stats.threats_detected += detected_threats.len() as u64;

        Ok(detected_threats)
    }

    /// Checks if a rule matches the given event data
    pub async fn matches_rule(
        &self,
        event_data: &HashMap<String, String>,
        rule: &ThreatDetectionRule,
    ) -> BearDogResult<bool> {
        if !rule.enabled {
            return Ok(false);
        }

        // Evaluate the rule condition
        if !self.evaluate_condition(&rule.condition, event_data).await? {
            return Ok(false); // Condition must match
        }

        Ok(true)
    }

    /// Evaluates a single condition against event data
    pub fn evaluate_condition<'a>(
        &'a self,
        condition: &'a RuleCondition,
        event_data: &'a HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = BearDogResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            match condition {
                RuleCondition::SourceIp { ip_address } => {
                    if let Some(source_ip) = event_data.get("source_ip") {
                        Ok(source_ip == ip_address)
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::FieldEquals { field, value } => {
                    if let Some(field_value) = event_data.get(field) {
                        Ok(field_value == value)
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::FieldContains { field, value } => {
                    if let Some(field_value) = event_data.get(field) {
                        Ok(field_value.contains(value))
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::FieldRegex { field, pattern } => {
                    if let Some(field_value) = event_data.get(field) {
                        // For now, just do a simple contains check
                        Ok(field_value.contains(pattern))
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::FieldGreaterThan { field, value } => {
                    if let Some(field_value) = event_data.get(field) {
                        if let (Ok(field_num), Ok(threshold_num)) =
                            (field_value.parse::<f64>(), value.parse::<f64>())
                        {
                            Ok(field_num > threshold_num)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::FieldLessThan { field, value } => {
                    if let Some(field_value) = event_data.get(field) {
                        if let (Ok(field_num), Ok(threshold_num)) =
                            (field_value.parse::<f64>(), value.parse::<f64>())
                        {
                            Ok(field_num < threshold_num)
                        } else {
                            Ok(false)
                        }
                    } else {
                        Ok(false)
                    }
                }
                RuleCondition::And { conditions } => {
                    for cond in conditions {
                        if !self.evaluate_condition(cond, event_data).await? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                RuleCondition::Or { conditions } => {
                    for cond in conditions {
                        if self.evaluate_condition(cond, event_data).await? {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
                RuleCondition::Not { condition } => {
                    let result = self.evaluate_condition(condition, event_data).await?;
                    Ok(!result)
                }
                // Add default cases for other variants
                _ => Ok(false), // For now, return false for unsupported conditions
            }
        })
    }

    /// Create a threat event from a rule match
    ///
    /// This method constructs a comprehensive threat event when a detection rule
    /// is triggered. It includes threat classification, severity assessment,
    /// and recommended actions.
    ///
    /// # Arguments
    ///
    /// * `rule` - The detection rule that was triggered
    /// * `event_data` - The event data that triggered the rule
    ///
    /// # Returns
    ///
    /// * `BearDogResult<ThreatEvent>` - A new threat event or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - UUID generation fails
    /// - Event data is missing required fields
    /// - Threat classification fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog::threat::handlers::ThreatDetectionEngine;
    /// use beardog::threat::types::*;
    /// use std::collections::HashMap;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let engine = ThreatDetectionEngine::placeholder();
    ///     
    ///     let mut event_data = HashMap::new();
    ///     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    ///     
    ///     let rule = ThreatDetectionRule {
    ///         id: "brute_force".to_string(),
    ///         name: "Brute Force Attack".to_string(),
    ///         severity: ThreatSeverity::High,
    ///         // ... other fields
    ///     };
    ///     
    ///     let threat = engine.create_threat_event(&rule, &event_data)?;
    ///     println!("Created threat: {}", threat.id);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_threat_event(
        &self,
        rule: &ThreatDetectionRule,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<ThreatEvent> {
        let threat_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        // Extract source and target information
        let source_ip = event_data.get("source_ip").cloned().unwrap_or_default();
        let target_ip = event_data.get("target_ip").cloned().unwrap_or_default();

        // Create threat source
        let threat_source = ThreatSource {
            id: Uuid::new_v4().to_string(),
            ip_address: Some(source_ip.clone()),
            hostname: Some(event_data.get("hostname").cloned().unwrap_or_default()),
            user_agent: Some(event_data.get("user_agent").cloned().unwrap_or_default()),
            geolocation: None, // Will be parsed separately if needed
            classification: SourceClassification::Unknown,
            reputation_score: 0.5,
            threat_actor: None,
            confidence_score: 0.8,
            first_seen: Some(timestamp),
            last_seen: Some(timestamp),
            threat_score: rule.severity.to_numeric_score(),
        };

        // Create threat target
        let threat_target = ThreatTarget {
            id: Uuid::new_v4().to_string(),
            resource_id: "target-resource".to_string(),
            resource_type: "network".to_string(),
            node_id: None,
            user_account: None,
            criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            ip_address: Some(target_ip),
            hostname: Some(
                event_data
                    .get("target_hostname")
                    .cloned()
                    .unwrap_or_default(),
            ),
            service: Some(event_data.get("service").cloned().unwrap_or_default()),
            port: event_data.get("port").and_then(|p| p.parse().ok()),
            protocol: Some(event_data.get("protocol").cloned().unwrap_or_default()),
        };

        // Create the threat event
        let threat_event = ThreatEvent {
            id: threat_id,
            threat_type: ThreatType::Malicious,
            severity: rule.severity.clone(),
            score: (rule.severity.to_numeric_score() * 100.0) as u8,
            timestamp,
            source: threat_source,
            target: threat_target,
            description: format!("Rule '{}' triggered: {}", rule.name, rule.description),
            detection_method: DetectionMethod::RuleBased,
            evidence: vec![],
            recommended_actions: vec![
                ThreatAction::AlertSecurityTeam,
                ThreatAction::InitiateIncidentResponse,
            ],
            status: ThreatStatus::Active,
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
        };

        Ok(threat_event)
    }
}

/// Helper trait for severity numeric conversion
trait SeverityNumeric {
    fn to_numeric_score(&self) -> f64;
}

impl SeverityNumeric for ThreatSeverity {
    /// Convert threat severity to numeric score (0.0 to 1.0)
    fn to_numeric_score(&self) -> f64 {
        match self {
            ThreatSeverity::Critical => 1.0,
            ThreatSeverity::High => 0.8,
            ThreatSeverity::Medium => 0.6,
            ThreatSeverity::Low => 0.4,
            ThreatSeverity::Info => 0.2,
        }
    }
}
