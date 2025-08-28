use super::core::ThreatDetectionEngine;
use crate::threat::types::*;
use beardog_errors::BearDogError;

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

impl ThreatDetectionEngine {
    pub async fn check_threat_feeds(
        &self,
        event_data: &HashMap<&str, &str>,
    ) -> Result<Vec<ThreatEvent>, BearDogError> {
        let mut feed_threats = Vec::new();
        for feed in self.threat_feeds.values() {
            for indicator in &feed.indicators {
                if self.matches_indicator(event_data, indicator) {
                    let threat = self.create_feed_threat_event(feed, indicator, event_data)?;
                    feed_threats.push(threat);
                }
            }
        }
        Ok(feed_threats)
    }

    fn matches_indicator(
        &self,
        event_data: &HashMap<&str, &str>,
        indicator: &ThreatIndicator,
    ) -> bool {
        match indicator.indicator_type {
            IndicatorType::IpAddress => {
                (event_data.get("source_ip") == Some(&indicator.value.as_str()))
                    || (event_data.get("dest_ip") == Some(&indicator.value.as_str()))
            }
            IndicatorType::DomainName => {
                (event_data.get("domain") == Some(&indicator.value.as_str()))
                    || event_data
                        .get("hostname")
                        .is_some_and(|hostname| hostname.contains(indicator.value.as_str()))
            }
            IndicatorType::Url => event_data
                .get("url")
                .is_some_and(|url| url.contains(indicator.value.as_str())),
            IndicatorType::FileHash => {
                (event_data.get("file_hash") == Some(&indicator.value.as_str()))
                    || (event_data.get("email") == Some(&indicator.value.as_str()))
            }
            IndicatorType::EmailAddress => {
                event_data.get("email") == Some(&indicator.value.as_str())
            }
            IndicatorType::UserAgent => event_data
                .get("user_agent")
                .is_some_and(|ua| ua.contains(indicator.value.as_str())),
            _ => false,
        }
    }

    fn create_feed_threat_event(
        &self,
        feed: &ThreatIntelligenceFeed,
        indicator: &ThreatIndicator,
        event_data: &HashMap<&str, &str>,
    ) -> Result<ThreatEvent, BearDogError> {
        let threat_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        let threat_source = ThreatSource {
            id: Uuid::new_v4().to_string(),
            source_type: "threat_intelligence".to_string(),
            ip_address: event_data.get("source_ip").map(|s| s.to_string()),
            hostname: event_data.get("hostname").map(|s| s.to_string()),
            user_agent: event_data.get("user_agent").map(|s| s.to_string()),
            geolocation: None, // Will be processed separately if needed
            classification: SourceClassification::KnownMalicious,
            reputation_score: 0.1, // Low reputation for known malicious
            threat_actor: None,    // Convert from indicator if available
            confidence_score: 0.8, // High confidence from threat intel
            first_seen: Some(timestamp),
            last_seen: Some(timestamp),
            threat_score: 80.0, // High threat score
            metadata: HashMap::new(),
        };

        let threat_target = ThreatTarget {
            id: Uuid::new_v4().to_string(),
            target_type: "network_endpoint".to_string(),
            resource_id: event_data
                .get("resource_id")
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("resource-{}", Uuid::new_v4())),
            resource_type: event_data
                .get("resource_type")
                .map(|s| s.to_string())
                .unwrap_or_else(|| "network-endpoint".to_string()),
            node_id: event_data.get("node_id").map(|s| s.to_string()),
            user_account: event_data.get("user_account").map(|s| s.to_string()),
            asset_criticality: AssetCriticality::Medium, // Default criticality
            protection_level: ProtectionLevel::Standard, // Default protection level
            criticality: AssetCriticality::Medium,       // Duplicate field for compatibility
            ip_address: event_data.get("target_ip").map(|s| s.to_string()),
            hostname: event_data.get("target_hostname").map(|s| s.to_string()),
            service: event_data.get("service").map(|s| s.to_string()),
            port: event_data.get("port").and_then(|s| s.parse().ok()),
            protocol: event_data.get("protocol").map(|s| s.to_string()),
            metadata: HashMap::new(),
        };

        let threat_event = ThreatEvent {
            id: threat_id,
            threat_type: ThreatType::Malicious,
            severity: ThreatSeverity::High, // Intelligence matches are typically high severity
            status: ThreatStatus::Active,
            description: format!(
                "Threat intelligence match: {:?} indicator '{}' from feed '{}'",
                indicator.indicator_type, indicator.value, feed.name
            ),
            source: threat_source,
            target: threat_target,
            timestamp,
            score: 80, // High threat score for intelligence matches
            detection_method: DetectionMethod::ThreatIntelligence,
            evidence: vec![],
            recommended_actions: vec![ThreatAction::BlockSource, ThreatAction::AlertSecurityTeam],
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
            confidence: 0.8, // High confidence from threat intel
            raw_data: None,
            mitigated: false,
            mitigation_actions: vec![],
        };
        Ok(threat_event)
    }

    pub fn add_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }
}
