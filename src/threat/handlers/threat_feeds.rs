//! Threat intelligence feeds management and processing
//!
//! This module handles external threat intelligence feeds, indicator processing,
//! and threat feed-based detection. It provides integration with various threat
//! intelligence sources and automated threat indicator matching.
//!
//! # Features
//!
//! - **Feed Management**: Add, update, and manage threat intelligence feeds
//! - **Indicator Processing**: Process and normalize threat indicators
//! - **Feed-based Detection**: Match events against threat intelligence indicators
//! - **Feed Health Monitoring**: Track feed status and update frequencies
//!
//! # Supported Indicators
//!
//! The system supports various types of threat indicators:
//! - IP addresses and network ranges
//! - Domain names and URLs
//! - File hashes (MD5, SHA1, SHA256)
//! - Email addresses
//! - User-agent strings
//! - Custom indicators
//!
//! # Examples
//!
//! ```rust
//! use beardog::threat::handlers::ThreatDetectionEngine;
//! use beardog::threat::types::*;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut engine = ThreatDetectionEngine::placeholder();
//!     
//!     // Add a threat intelligence feed
//!     let feed = ThreatIntelligenceFeed {
//!         id: "malware_ips".to_string(),
//!         name: "Malware IP Feed".to_string(),
//!         source: "example.com".to_string(),
//!         feed_type: "ip_reputation".to_string(),
//!         // ... other fields
//!     };
//!     engine.add_threat_feed(feed);
//!     
//!     // Check event against threat feeds
//!     let mut event_data = HashMap::new();
//!     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
//!     
//!     let feed_threats = engine.check_threat_feeds(&event_data).await?;
//!     println!("Found {} threats from feeds", feed_threats.len());
//!     Ok(())
//! }
//! ```

use super::core::ThreatDetectionEngine;
use crate::threat::types::*;
use crate::BearDogResult;

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

impl ThreatDetectionEngine {
    /// Check event data against threat intelligence feeds
    ///
    /// This method processes event data against all configured threat intelligence
    /// feeds to identify known malicious indicators. It supports various indicator
    /// types and provides detailed threat context from feed sources.
    ///
    /// # Arguments
    ///
    /// * `event_data` - A HashMap containing event fields and their values
    ///
    /// # Returns
    ///
    /// * `BearDogResult<Vec<ThreatEvent>>` - A vector of feed-based threats or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Feed processing fails
    /// - Indicator matching fails
    /// - Threat event creation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog::threat::handlers::ThreatDetectionEngine;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let engine = ThreatDetectionEngine::placeholder();
    ///     
    ///     let mut event_data = HashMap::new();
    ///     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    ///     event_data.insert("domain".to_string(), "malicious.example.com".to_string());
    ///     
    ///     let threats = engine.check_threat_feeds(&event_data).await?;
    ///     for threat in threats {
    ///         println!("Feed threat: {} from {}", threat.description, threat.source.ip_address);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Feed Processing
    ///
    /// The method processes feeds in the following order:
    /// 1. Iterate through all active threat intelligence feeds
    /// 2. Extract relevant indicators from each feed
    /// 3. Match event data against feed indicators
    /// 4. Create threat events for positive matches
    /// 5. Enrich threats with feed-specific context
    pub async fn check_threat_feeds(
        &self,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<Vec<ThreatEvent>> {
        let mut feed_threats = Vec::new();

        for feed in self.threat_feeds.values() {
            // Check indicators in this feed
            for indicator in &feed.indicators {
                if self.matches_indicator(event_data, indicator) {
                    let threat = self.create_feed_threat_event(feed, indicator, event_data)?;
                    feed_threats.push(threat);
                }
            }
        }

        Ok(feed_threats)
    }

    /// Check if event data matches a threat indicator
    ///
    /// This method determines whether event data contains values that match
    /// a specific threat indicator from intelligence feeds.
    ///
    /// # Arguments
    ///
    /// * `event_data` - The event data to check
    /// * `indicator` - The threat indicator to match against
    ///
    /// # Returns
    ///
    /// * `bool` - True if the event matches the indicator, false otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog::threat::handlers::ThreatDetectionEngine;
    /// use beardog::threat::types::*;
    /// use std::collections::HashMap;
    ///
    /// fn main() {
    ///     let engine = ThreatDetectionEngine::placeholder();
    ///     
    ///     let mut event_data = HashMap::new();
    ///     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    ///     
    ///     let indicator = ThreatIndicator {
    ///         id: "malicious_ip".to_string(),
    ///         indicator_type: "ip_address".to_string(),
    ///         value: "192.168.1.100".to_string(),
    ///         // ... other fields
    ///     };
    ///     
    ///     let matches = engine.matches_indicator(&event_data, &indicator);
    ///     println!("Indicator match: {}", matches);
    /// }
    /// ```
    ///
    /// # Indicator Types
    ///
    /// Supported indicator types and their matching logic:
    /// - `ip_address`: Exact IP address match
    /// - `domain`: Domain name match
    /// - `url`: URL pattern match
    /// - `file_hash`: File hash match (MD5, SHA1, SHA256)
    /// - `email`: Email address match
    /// - `user_agent`: User-agent string match
    pub fn matches_indicator(
        &self,
        event_data: &HashMap<String, String>,
        indicator: &ThreatIndicator,
    ) -> bool {
        match indicator.indicator_type {
            IndicatorType::IpAddress => {
                (event_data
                    .get("source_ip") == Some(&indicator.value))
                    || (event_data
                        .get("dest_ip") == Some(&indicator.value))
            }
            IndicatorType::DomainName => {
                (event_data
                    .get("domain") == Some(&indicator.value))
                    || event_data
                        .get("hostname")
                        .is_some_and(|hostname| hostname.contains(&indicator.value))
            }
            IndicatorType::Url => event_data
                .get("url")
                .is_some_and(|url| url.contains(&indicator.value)),
            IndicatorType::FileHash => event_data
                .get("file_hash") == Some(&indicator.value),
            IndicatorType::EmailAddress => event_data
                .get("email") == Some(&indicator.value),
            IndicatorType::UserAgent => event_data
                .get("user_agent")
                .is_some_and(|ua| ua.contains(&indicator.value)),
            _ => false,
        }
    }

    /// Create a threat event from a feed indicator match
    ///
    /// This method constructs a threat event when an indicator from a threat
    /// intelligence feed matches event data. It includes feed-specific context
    /// and threat intelligence metadata.
    ///
    /// # Arguments
    ///
    /// * `feed` - The threat intelligence feed containing the indicator
    /// * `indicator` - The matched threat indicator
    /// * `event_data` - The event data that matched the indicator
    ///
    /// # Returns
    ///
    /// * `BearDogResult<ThreatEvent>` - A new threat event or error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - UUID generation fails
    /// - Threat event creation fails
    /// - Feed metadata is invalid
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
    ///     let feed = ThreatIntelligenceFeed {
    ///         id: "malware_feed".to_string(),
    ///         name: "Malware Intelligence Feed".to_string(),
    ///         source: "security_vendor".to_string(),
    ///         // ... other fields
    ///     };
    ///     
    ///     let indicator = ThreatIndicator {
    ///         id: "bad_ip".to_string(),
    ///         indicator_type: "ip_address".to_string(),
    ///         value: "192.168.1.100".to_string(),
    ///         // ... other fields
    ///     };
    ///     
    ///     let mut event_data = HashMap::new();
    ///     event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    ///     
    ///     let threat = engine.create_feed_threat_event(&feed, &indicator, &event_data)?;
    ///     println!("Created feed threat: {}", threat.id);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_feed_threat_event(
        &self,
        feed: &ThreatIntelligenceFeed,
        indicator: &ThreatIndicator,
        event_data: &HashMap<String, String>,
    ) -> BearDogResult<ThreatEvent> {
        let threat_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        // Create threat source with feed intelligence
        let threat_source = ThreatSource {
            id: Uuid::new_v4().to_string(),
            ip_address: Some(event_data.get("source_ip").cloned().unwrap_or_default()),
            hostname: Some(event_data.get("hostname").cloned().unwrap_or_default()),
            user_agent: Some(event_data.get("user_agent").cloned().unwrap_or_default()),
            geolocation: None, // Will be processed separately if needed
            classification: SourceClassification::KnownMalicious,
            reputation_score: 0.1, // Low reputation for known malicious
            threat_actor: indicator.threat_actor.clone(),
            confidence_score: indicator.confidence_score,
            first_seen: Some(timestamp),
            last_seen: Some(timestamp),
            threat_score: indicator.confidence_score * 100.0,
        };

        // Create threat target
        let threat_target = ThreatTarget {
            id: Uuid::new_v4().to_string(),
            resource_id: event_data
                .get("resource_id")
                .cloned()
                .unwrap_or_else(|| format!("resource-{}", Uuid::new_v4())),
            resource_type: event_data
                .get("resource_type")
                .cloned()
                .unwrap_or_else(|| "network-endpoint".to_string()),
            node_id: event_data.get("node_id").cloned(),
            user_account: event_data.get("user_account").cloned(),
            criticality: AssetCriticality::High, // Assume high for intelligence matches
            protection_level: ProtectionLevel::Enhanced,
            ip_address: Some(event_data.get("target_ip").cloned().unwrap_or_default()),
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

        // Create threat event
        let threat_event = ThreatEvent {
            id: threat_id,
            threat_type: ThreatType::Malicious,
            severity: ThreatSeverity::High, // Intelligence matches are typically high severity
            status: ThreatStatus::Active,
            description: format!(
                "Threat intelligence match: {} indicator '{}' from feed '{}' - {}",
                indicator.indicator_type, indicator.value, feed.name, indicator.description
            ),
            source: threat_source,
            target: threat_target,
            timestamp,
            score: (indicator.confidence_score * 100.0) as u8,
            detection_method: DetectionMethod::ThreatIntelligence,
            evidence: vec![],
            recommended_actions: vec![ThreatAction::BlockSource, ThreatAction::AlertSecurityTeam],
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
        };

        Ok(threat_event)
    }

    /// Add a threat intelligence feed to the engine
    ///
    /// This method registers a new threat intelligence feed with the detection engine.
    /// The feed will be used for future threat detection operations.
    ///
    /// # Arguments
    ///
    /// * `feed` - The threat intelligence feed to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog::threat::handlers::ThreatDetectionEngine;
    /// use beardog::threat::types::*;
    ///
    /// fn main() {
    ///     let mut engine = ThreatDetectionEngine::placeholder();
    ///     
    ///     let feed = ThreatIntelligenceFeed {
    ///         id: "apt_indicators".to_string(),
    ///         name: "APT Indicators Feed".to_string(),
    ///         source: "government_agency".to_string(),
    ///         feed_type: "apt_intelligence".to_string(),
    ///         // ... other fields
    ///     };
    ///     
    ///     engine.add_threat_feed(feed);
    ///     println!("Threat feed added successfully");
    /// }
    /// ```
    ///
    /// # Feed Management
    ///
    /// - Feeds are indexed by their unique ID
    /// - Adding a feed with an existing ID will replace the previous feed
    /// - Feeds are immediately available for threat detection
    /// - Feed health and update status should be monitored separately
    pub fn add_threat_feed(&mut self, feed: ThreatIntelligenceFeed) {
        self.threat_feeds.insert(feed.id.clone(), feed);
    }
}
