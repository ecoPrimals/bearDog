// Threat Intelligence Feeds - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready threat intelligence feed processing.

use crate::threat::types::{
    AssetCriticality, DetectionMethod, IndicatorType, ProtectionLevel, SourceClassification,
    ThreatAction, ThreatDetectionEngine, ThreatEvent, ThreatIndicator, ThreatIntelligenceFeed,
    ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons
)]
#[cfg(test)]
use std::time::SystemTime;

/// Threat intelligence feed manager
impl ThreatDetectionEngine {
    /// Process threat intelligence feeds
    pub fn check_threat_feeds(
        &self,
        event_data: &HashMap<String, String>,
    ) -> Result<Vec<ThreatEvent>, BearDogError> {
        let mut feed_threats = Vec::new();

        // Process each configured threat feed
        for feed in self.threat_feeds.values() {
            if let Some(threat_event) = self.check_feed_indicators(feed, event_data)? {
                feed_threats.push(threat_event);
            }
        }

        Ok(feed_threats)
    }

    fn check_feed_indicators(
        &self,
        feed: &ThreatIntelligenceFeed,
        event_data: &HashMap<String, String>,
    ) -> Result<Option<ThreatEvent>, BearDogError> {
        // Check each indicator in the feed
        for indicator in &feed.indicators {
            if self.matches_event_data(event_data, indicator) {
                // Create threat event from matched indicator
                let threat_event =
                    self.create_threat_event_from_intelligence(feed, indicator, event_data)?;
                return Ok(Some(threat_event));
            }
        }

        Ok(None)
    }

    /// Check if indicator matches event data
    fn matches_event_data(
        &self,
        event_data: &HashMap<String, String>,
        indicator: &ThreatIndicator,
    ) -> bool {
        match indicator.indicator_type {
            IndicatorType::IpAddress => {
                event_data.get("source_ip") == Some(&indicator.value)
                    || event_data.get("destination_ip") == Some(&indicator.value)
            }
            IndicatorType::DomainName => {
                event_data.get("domain") == Some(&indicator.value)
                    || event_data
                        .get("hostname")
                        .is_some_and(|hostname| hostname.contains(&indicator.value))
            }
            IndicatorType::Url => event_data
                .get("url")
                .is_some_and(|url| url.contains(&indicator.value)),
            IndicatorType::FileHash => {
                event_data.get("file_hash") == Some(&indicator.value)
                    || event_data.get("sha256") == Some(&indicator.value)
                    || event_data.get("md5") == Some(&indicator.value)
            }
            IndicatorType::EmailAddress => event_data.get("email") == Some(&indicator.value),
            IndicatorType::UserAgent => event_data.get("user_agent") == Some(&indicator.value),
            IndicatorType::RegistryKey => event_data.get("registry_key") == Some(&indicator.value),
            IndicatorType::ProcessName => event_data.get("process_name") == Some(&indicator.value),
        }
    }

    /// Create threat event from intelligence match
    /// Creates `threat_event_from_intelligence`
    fn create_threat_event_from_intelligence(
        &self,
        feed: &ThreatIntelligenceFeed,
        indicator: &ThreatIndicator,
        event_data: &HashMap<String, String>,
    ) -> Result<ThreatEvent, BearDogError> {
        let threat_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        // Create threat source
        let threat_source = ThreatSource {
            source_type: "threat_intelligence".to_string(),
            identifier: Uuid::new_v4().to_string(),
            id: Uuid::new_v4().to_string(),
            ip_address: event_data.get("source_ip").cloned(),
            hostname: event_data.get("hostname").cloned(),
            user_agent: event_data.get("user_agent").cloned(),
            location: None,
            geolocation: None,
            threat_actor: None,
            classification: SourceClassification::KnownMalicious,
            reputation: Some(0.1),
            reputation_score: 0.1, // Low reputation for known malicious
            confidence_score: 0.9, // High confidence from threat intel
            first_seen: Some(timestamp.into()),
            last_seen: Some(timestamp.into()),
            metadata: HashMap::with_capacity(16),
        };

        // Create threat target
        let threat_target = ThreatTarget {
            target_type: "network_endpoint".to_string(),
            identifier: Uuid::new_v4().to_string(),
            id: Uuid::new_v4().to_string(),
            resource_id: event_data
                .get("resource_id")
                .cloned()
                .unwrap_or_else(|| format!("resource-{}", Uuid::new_v4())),
            node_id: event_data.get("node_id").cloned(),
            user_account: event_data.get("user_account").cloned(),
            asset_criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: event_data.get("service ").cloned(),
            port: event_data.get("port").and_then(|s| s.parse().ok()),
            protocol: event_data.get("protocol").cloned(),
            metadata: HashMap::with_capacity(16),
            resource_type: event_data
                .get("resource_type")
                .cloned()
                .unwrap_or_else(|| "network-endpoint".to_string()),
            criticality: ThreatSeverity::Medium,
            ip_address: event_data.get("destination_ip").cloned(),
            hostname: event_data.get("target_hostname").cloned(),
        };

        // Create the threat event
        let threat_event = ThreatEvent {
            id: threat_id,
            threat_type: ThreatType::Malicious,
            severity: ThreatSeverity::High, // Intelligence matches are typically high severity
            status: ThreatStatus::Active,
            source: threat_source,
            target: threat_target,
            detected_at: timestamp.into(),
            timestamp: timestamp.into(),
            confidence: indicator.confidence,
            score: 85, // High threat score for intelligence matches
            description: format!(
                "Threat intelligence match: {:?} indicator {} from feed {}",
                indicator.indicator_type, indicator.value, feed.name
            ),
            detection_method: DetectionMethod::ThreatIntelligence,
            evidence: vec![format!(
                "Matched {} indicator: {}",
                indicator.indicator_type.to_string().to_lowercase(),
                indicator.value
            )],
            recommended_actions: vec![
                ThreatAction::BlockSource,
                ThreatAction::AlertSecurityTeam,
                ThreatAction::InvestigateActivity,
            ],
            assigned_analyst: None,
            related_events: vec![],
            raw_data: None,
            mitigated: false,
            mitigation_actions: Vec::new(),
            metadata: HashMap::new(),
            mitigation_steps: vec![],
        };

        Ok(threat_event)
    }

    /// Update threat intelligence feed (delegated to engine)
    /// Updates `threat_feed_handler`
    /// Updates `threat_feed_handler`
    pub fn update_threat_feed_handler(
        &mut self,
        feed: ThreatIntelligenceFeed,
    ) -> Result<(), BearDogError> {
        use std::collections::hash_map::Entry;

        match self.threat_feeds.entry(feed.id.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(feed);
                Ok(())
            }
            Entry::Vacant(_) => Err(BearDogError::not_found(format!(
                "Threat feed with ID {} not found",
                feed.id
            ))),
        }
    }

    /// Get threat feed statistics
    /// Gets `feed_statistics`
    /// Gets `feed_statistics`
    #[must_use]
    pub fn get_feed_statistics(&self) -> ThreatFeedStats {
        let total_feeds = self.threat_feeds.len();
        let total_indicators: usize = self
            .threat_feeds
            .values()
            .map(|feed| feed.indicators.len())
            .sum();

        let active_feeds = self
            .threat_feeds
            .values()
            .filter(|feed| feed.enabled)
            .count();

        ThreatFeedStats {
            total_feeds,
            active_feeds,
            total_indicators,
            last_updated: Utc::now(),
        }
    }
}

/// Threat feed statistics
#[derive(Debug, Clone)]
pub struct ThreatFeedStats {
    /// Number of `total_feeds`
    /// Number of `total_feeds`
    pub total_feeds: usize,
    /// Number of `active_feeds`
    /// Number of `active_feeds`
    pub active_feeds: usize,
    /// Number of `total_indicators`
    /// Number of `total_indicators`
    pub total_indicators: usize,
    /// The last updated value
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons
)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat::ThreatDetectionConfig;

    #[test]
    fn test_indicator_type_display() {
        assert_eq!(IndicatorType::IpAddress.to_string(), "IP Address");
        assert_eq!(IndicatorType::DomainName.to_string(), "Domain Name");
        assert_eq!(IndicatorType::FileHash.to_string(), "File Hash");
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[tokio::test]
    async fn test_threat_feed_processing() -> Result<(), BearDogError> {
        let config = ThreatDetectionConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut engine = ThreatDetectionEngine::new(config)?;

        // Create a test threat feed
        let feed = ThreatIntelligenceFeed {
            id: "test-feed-1".to_string(),
            name: "Test Feed".to_string(),
            source: "test_source".to_string(),
            enabled: true,
            reliability: 0.9,
            last_updated: SystemTime::now(),
            indicators: vec![ThreatIndicator {
                indicator_type: IndicatorType::IpAddress,
                value: "192.168.1.100".to_string(),
                confidence: 0.9,
                source: "test_source".to_string(),
                created_at: SystemTime::now(),
            }],
        };

        engine.add_threat_feed(feed);

        // Test event data that should match
        let mut event_data = HashMap::new();
        event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
        event_data.insert("event_type".to_string(), "network_connection".to_string());

        let matches = engine.check_threat_feeds(&event_data).unwrap_or_default();
        assert!(!matches.is_empty(), "Should detect threat indicators");

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_indicator_matching() -> Result<(), BearDogError> {
        let config = ThreatDetectionConfig::default();
        // Test manual indicator creation
        let engine = ThreatDetectionEngine::new(config)?;

        let indicator = ThreatIndicator {
            indicator_type: IndicatorType::IpAddress,
            value: "10.0.0.1".to_string(),
            confidence: 0.8,
            source: "manual_test".to_string(),
            created_at: SystemTime::now(),
        };

        let mut event_data = HashMap::new();
        event_data.insert("source_ip".to_string(), "10.0.0.1".to_string());

        assert!(engine.matches_event_data(&event_data, &indicator));

        // Test non-matching data
        event_data.insert("source_ip".to_string(), "10.0.0.2".to_string());
        assert!(!engine.matches_event_data(&event_data, &indicator));

        Ok(())
    }
}
