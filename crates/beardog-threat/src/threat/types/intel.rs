// SPDX-License-Identifier: AGPL-3.0-or-later

//! Threat intelligence indicators and feed metadata.

use super::taxonomy::IndicatorType;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Threat intelligence indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    /// The indicator type value
    pub indicator_type: IndicatorType,
    /// Indicator value
    /// The value value
    pub value: String,
    /// Provider confidence that this indicator is malicious or relevant (0.0–1.0).
    pub confidence: f64,
    /// Source of indicator
    /// The source value
    pub source: String,
    /// Timestamp when indicator was created
    /// The created at value
    pub created_at: SystemTime,
}

/// Threat intelligence feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    /// Feed identifier
    pub id: String,
    /// Feed name
    /// Name of the item
    pub name: String,
    /// Feed source
    /// The source value
    pub source: String,
    /// Last update timestamp
    /// The last updated value
    pub last_updated: SystemTime,
    /// Whether feed is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Feed reliability score
    /// The reliability value
    pub reliability: f64,
    /// Indicators in this feed
    /// Collection of indicators
    pub indicators: Vec<ThreatIndicator>,
}
