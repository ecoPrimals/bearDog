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


/// Threat intelligence feeds
///
/// This module contains types and functionality for managing threat intelligence
/// feeds, including feed metadata, status tracking, and indicator management.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::{FeedStatus, FeedType, UpdateFrequency};
use super::indicators::{IndicatorType, ThreatIndicator};
/// Threat intelligence feed structure
/// Represents a threat intelligence feed with metadata,
/// status, and indicators of compromise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    /// Feed identifier
    pub id: String,
    /// Feed name
    pub name: String,
    /// Feed URL
    pub url: String,
    /// Feed type
    pub feed_type: FeedType,
    /// Update frequency
    pub update_frequency: UpdateFrequency,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
    /// Feed status
    pub status: FeedStatus,
    /// Indicators of compromise
    pub indicators: Vec<ThreatIndicator>,
}
impl Default for ThreatIntelligenceFeed {}


    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            url: String::new(),
            feed_type: FeedType::IpReputation,
            update_frequency: UpdateFrequency::Daily,
            last_updated: Utc::now(),
            status: FeedStatus::Active,
            indicators: vec![],
        }
    }
impl ThreatIntelligenceFeed {
    /// Create a new threat intelligence feed
    ///
    /// # Arguments
    /// * `id` - Unique feed identifier
    /// * `name` - Human-readable feed name
    /// * `url` - Feed URL
    /// * `feed_type` - Type of feed
    /// # Returns
    /// A new `ThreatIntelligenceFeed` instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatIntelligenceFeed, FeedType};
    /// let feed = ThreatIntelligenceFeed::new(
    ///     "malware-ips".to_string(),
    ///     "Malware IP Feed".to_string(),
    ///     "https://example.com/malware-ips.json".to_string(),
    ///     FeedType::IpReputation
    /// );
    /// ```
    pub fn new(id: String, name: String, url: String, feed_type: FeedType) -> Self {
            id,
            name,
            url,
            feed_type,
    /// Add indicator to feed
    /// * `indicator` - Threat indicator to add
    /// use beardog::threat::types::{ThreatIntelligenceFeed, ThreatIndicator, FeedType};
    /// let mut feed = ThreatIntelligenceFeed::new(
    ///     "test-feed".to_string(),
    ///     "Test Feed".to_string(),
    ///     "https://example.com".to_string(),
    /// let indicator = ThreatIndicator::default();
    /// feed.add_indicator(indicator);
    /// assert_eq!(feed.indicators.len(), 1);
    pub fn add_indicator(&mut self, indicator: ThreatIndicator) {
        self.indicators.push(indicator);
    /// Remove indicator from feed
    /// * `value` - Indicator value to remove
    /// `true` if indicator was found and removed
    /// let mut indicator = ThreatIndicator::default();
    /// indicator.value = "192.168.1.100".to_string();
    /// assert!(feed.remove_indicator("192.168.1.100"));
    /// assert_eq!(feed.indicators.len(), 0);}


    pub fn remove_indicator(&mut self, value: &str) -> bool {
        let original_len = self.indicators.len();
        self.indicators.retain(|indicator| indicator.value != value);
        self.indicators.len() != original_len
    /// Get indicators by type
    /// * `indicator_type` - Type of indicators to retrieve
    /// Vector of indicators matching the specified type
    /// use beardog::threat::types::{ThreatIntelligenceFeed, IndicatorType, FeedType};
    /// let ip_indicators = feed.get_indicators_by_type(&IndicatorType::IpAddress);
    pub fn get_indicators_by_type(&self, indicator_type: &IndicatorType) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.indicator_type == *indicator_type)
            .collect()
    /// Update feed status
    /// * `status` - New feed status
    /// use beardog::threat::types::{ThreatIntelligenceFeed, FeedStatus, FeedType};
    /// feed.update_status(FeedStatus::Error);
    /// assert_eq!(feed.status, FeedStatus::Error);
    pub fn update_status(&mut self, status: FeedStatus) {
        self.status = status;
        self.last_updated = Utc::now();
    /// Check if feed is operational
    /// `true` if feed is active or updating
    /// assert!(feed.is_operational());
    /// assert!(!feed.is_operational());}


    pub fn is_operational(&self) -> bool {
        matches!(self.status, FeedStatus::Active | FeedStatus::Updating)
    /// Get feed age in hours
    /// Hours since last update
    /// let age = feed.age_hours();
    /// assert!(age >= 0);
    pub fn age_hours(&self) -> i64 {
        let now = Utc::now();
        (now - self.last_updated).num_hours()
    /// Check if feed needs update
    /// `true` if feed should be updated based on frequency
    /// use beardog::threat::types::{ThreatIntelligenceFeed, UpdateFrequency, FeedType};
    /// feed.update_frequency = UpdateFrequency::Hourly;
    /// // Check if update is needed
    pub fn needs_update(&self) -> bool {
        let age_hours = self.age_hours();
        match self.update_frequency {
            UpdateFrequency::RealTime => age_hours > 0,
            UpdateFrequency::Hourly => age_hours >= 1,
            UpdateFrequency::Daily => age_hours >= 24,
            UpdateFrequency::Weekly => age_hours >= 168,
            UpdateFrequency::Monthly => age_hours >= 720,
    /// Get high confidence indicators
    /// Vector of indicators with confidence > 0.7
    /// let high_confidence = feed.get_high_confidence_indicators();}


    pub fn get_high_confidence_indicators(&self) -> Vec<&ThreatIndicator> {
            .filter(|indicator| indicator.is_high_confidence())
    /// Get recent indicators
    /// * `hours` - Number of hours to consider as recent
    /// Vector of indicators seen within the specified hours
    /// let recent = feed.get_recent_indicators(24);
    pub fn get_recent_indicators(&self, hours: i64) -> Vec<&ThreatIndicator> {
            .filter(|indicator| indicator.is_recent(hours))
    /// Get indicator count by type
    /// Count of indicators in the feed
    /// let count = feed.indicator_count();
    /// assert_eq!(count, 0);}


    pub fn indicator_count(&self) -> usize {
        self.indicators.len()
    /// Clear all indicators from feed
    /// feed.add_indicator(ThreatIndicator::default());
    /// assert_eq!(feed.indicator_count(), 1);
    /// feed.clear_indicators();
    /// assert_eq!(feed.indicator_count(), 0);
    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
