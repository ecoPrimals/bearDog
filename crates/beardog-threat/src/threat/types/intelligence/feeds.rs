

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::enums::*;
use super::indicators::ThreatIndicator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    pub id: String,
    pub name: String,
    pub url: String,
    pub update_frequency: super::enums::UpdateFrequency,
    pub last_updated: Option<DateTime<Utc>>,
    pub status: super::enums::FeedStatus,
    pub error_count: u32,
    pub success_count: u32,
    pub metadata: HashMap<String, String>,
    pub indicators: Vec<super::indicators::ThreatIndicator>, // Add missing field
}
impl Default for ThreatIntelligenceFeed {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Feed".to_string(),
            url: String::new(),
            update_frequency: super::enums::UpdateFrequency::Daily,
            last_updated: None,
            status: super::enums::FeedStatus::Inactive,
            error_count: 0,
            success_count: 0,
            metadata: HashMap::new(),
            indicators: Vec::new(),
        }
    }
}
impl ThreatIntelligenceFeed {

    pub fn new(id: &str, name: &str, url: &str, update_frequency: UpdateFrequency) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            url: url.to_string(),
            update_frequency,
            last_updated: None,
            status: FeedStatus::Inactive,
            error_count: 0,
            success_count: 0,
            metadata: HashMap::new(),
            indicators: Vec::new(),
        }
    }

    pub fn add_indicator(&mut self, indicator: ThreatIndicator) {
        self.indicators.push(indicator);
    }

    pub fn remove_indicator(&mut self, value: &str) -> bool {
        let original_len = self.indicators.len();
        self.indicators.retain(|indicator| indicator.value != value);
        self.indicators.len() != original_len
    }

    pub fn get_indicators_by_type(&self, indicator_type: &IndicatorType) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.indicator_type == *indicator_type)
            .collect()
    }

    pub fn update_status(&mut self, status: FeedStatus) {
        self.status = status;
        self.last_updated = Some(Utc::now());
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, FeedStatus::Active)
    }

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

    pub fn get_high_confidence_indicators(&self) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.is_high_confidence())
            .collect()
    }

    pub fn get_recent_indicators(&self, hours: i64) -> Vec<&ThreatIndicator> {
        self.indicators
            .iter()
            .filter(|indicator| indicator.is_recent(hours))
            .collect()
    }

    pub fn indicator_count(&self) -> usize {
        self.indicators.len()
    }

    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
    }
}
