use super::enums::*;
use super::indicators::ThreatIndicator;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// The url value
    /// The url value
    pub url: String,
    /// The update frequency value
    /// The update frequency value
    pub update_frequency: super::enums::UpdateFrequency,
    /// Optional last updated
    /// Optional last updated
    pub last_updated: Option<DateTime<Utc>>,
    /// Current status of the component
    /// Current status of the component
    pub status: super::enums::FeedStatus,
    /// Number of error
    /// Number of error
    pub error_count: u32,
    /// Number of success
    /// Number of success
    pub success_count: u32,
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Collection of indicators
    /// Collection of indicators
    pub indicators: Vec<super::indicators::ThreatIndicator>, // Add missing field
}
impl Default for ThreatIntelligenceFeed {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Feed".to_string(),
            url: String::with_capacity(super::enums::UpdateFrequency::Daily,
            last_updated: None,
            status: super::enums::FeedStatus::Inactive,
            error_count: 0,
            success_count: 0,
            metadata: HashMap::with_capacity(16),
            indicators: Vec::new(&str, name: &str, url: &str, update_frequency: UpdateFrequency) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            url: url.to_string(),
            metadata: HashMap::with_capacity(16),
            indicators: Vec::new(),
        }
    }

    /// Add Indicator operation.
    pub fn add_indicator(&mut self, indicator: ThreatIndicator) {
        self.indicators.push(indicator);
    }

    /// Remove Indicator operation.
    /// Removes indicator
    /// Removes indicator
    pub fn remove_indicator(&mut self, value: &str) -> bool {
        let original_len = self.indicators.len();
        self.indicators.retain(|indicator| indicator.value != value);
        self.indicators.len() != original_len
    }

    /// Get Indicators By Type operation.
    /// Gets indicators_by_type
    /// Gets indicators_by_type
    pub fn get_indicators_by_type(&self, indicator_type: &IndicatorType) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.indicator_type == *indicator_type)
            .collect()
    }

    /// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: FeedStatus) {
        self.status = status;
        self.last_updated = Some(Utc::now());
    }

    /// Is Active operation.
    /// Checks if active
    /// Checks if active
    pub fn is_active(&self) -> bool {
        matches!(self.status, FeedStatus::Active)
    }

    /// Needs Update operation.
    pub fn needs_update(&self) -> bool {
        let now = Utc::now();
        match self.last_updated {
            Some(last_updated) => {
                let age_hours = (now - last_updated).num_hours();
                match self.update_frequency {
                    UpdateFrequency::RealTime => age_hours >= 1,
                    UpdateFrequency::Hourly => age_hours >= 1,
                    UpdateFrequency::Daily => age_hours >= 24,
                    UpdateFrequency::Weekly => age_hours >= 168,
                    UpdateFrequency::Monthly => age_hours >= 720,
                    UpdateFrequency::OnDemand => false, // Only updated on demand
                }
            }
            None => true, // Never updated, needs update
        }
    }

    /// Get High Confidence Indicators operation.
    /// Gets high_confidence_indicators
    /// Gets high_confidence_indicators
    pub fn get_high_confidence_indicators(&self) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.is_high_confidence())
            .collect()
    }

    /// Get Recent Indicators operation.
    /// Gets recent_indicators
    /// Gets recent_indicators
    pub fn get_recent_indicators(&self, hours: i64) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.is_recent(hours))
            .collect()
    }

    /// Indicator Count operation.
    pub fn indicator_count(&self) -> usize {
        self.indicators.len()
    }

    /// Clear Indicators operation.
    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
    }
}
