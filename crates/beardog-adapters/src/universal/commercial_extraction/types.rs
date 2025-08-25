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


/// Commercial Extraction Detection Types
///
/// Contains all types and structures for detecting and preventing commercial extraction.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
/// Classification of commercial extraction risk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommercialClassification {
    /// Legitimate personal or organizational use
    Legitimate,
    /// Suspicious patterns detected
    Suspicious {
        /// Confidence score (0.0-1.0)
        confidence: f64,
        /// Detected patterns
        patterns: Vec<String>,
    },
    /// Confirmed commercial extraction
    Commercial {
        /// Evidence of extraction
        evidence: Vec<String>,
    /// Unclear classification
    Unknown,
}
/// Risk level for extraction activities
pub enum ExtractionRisk {
    /// Low risk activity
    Low,
    /// Medium risk activity
    Medium,
    /// High risk activity
    High,
    /// Critical risk requiring immediate action
    Critical,
/// Status of key evolution for a user}


pub enum KeyEvolutionStatus {
    /// New key lineage started
    NewLineage,
    /// Suspicious evolution pattern detected
    SuspiciousEvolution,
    /// Key lineage continuing normally
    ContinuedLineage,
    /// Key evolution system not initialized
    Uninitialized,
/// User usage pattern analysis
pub struct UsagePattern {
    /// User identifier
    pub user_id: String,
    /// Total number of requests
    pub request_count: u64,
    /// Unique endpoints accessed
    pub unique_endpoints: std::collections::HashSet<String>,
    /// Request frequency (requests per hour)
    pub request_frequency: f64,
    /// Total data volume processed
    pub data_volume: u64,
    /// Timestamp pattern for timing analysis
    pub timestamp_pattern: Vec<DateTime<Utc>>,
    /// Detected unusual patterns
    pub unusual_patterns: Vec<String>,
/// Request for extraction analysis
pub struct ExtractionAnalysisRequest {
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Request metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Current usage patterns
    pub usage_patterns: Vec<UsagePattern>,
/// Commercial extraction detector main struct
#[derive(Debug)]
pub struct CommercialExtractionDetector {
    /// Detector configuration
    pub config: DetectorConfig,
    /// Internal state - usage patterns stored by user ID
    pub state: Arc<RwLock<HashMap<String, UsagePattern>>>,
/// Configuration for commercial extraction detection
pub struct DetectorConfig {
    /// Enable detection
    pub enabled: bool,
    /// Sensitivity threshold
    pub sensitivity: f64,
    /// Maximum patterns to track
    pub max_patterns: usize,}


impl CommercialExtractionDetector {
    /// Create new detector with configuration}


    #[must_use] pub fn new(config: DetectorConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
impl Default for DetectorConfig {}


    fn default() -> Self {
            enabled: true,
            sensitivity: 0.7,
            max_patterns: 1000,
impl Default for CommercialExtractionDetector {
        Self::new(DetectorConfig::default())
