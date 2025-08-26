

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommercialClassification {

    Legitimate,

    Suspicious {

        confidence: f64,

        patterns: Vec<String>,
    },

    Commercial {

        evidence: Vec<String>,

    Unknown,
}

pub enum ExtractionRisk {

    Low,

    Medium,

    High,

    Critical,

pub enum KeyEvolutionStatus {

    NewLineage,

    SuspiciousEvolution,

    ContinuedLineage,

    Uninitialized,

pub struct UsagePattern {

    pub user_id: String,

    pub request_count: u64,

    pub unique_endpoints: std::collections::HashSet<String>,

    pub request_frequency: f64,

    pub data_volume: u64,

    pub timestamp_pattern: Vec<DateTime<Utc>>,

    pub unusual_patterns: Vec<String>,

pub struct ExtractionAnalysisRequest {

    pub timestamp: DateTime<Utc>,

    pub metadata: HashMap<String, serde_json::Value>,

    pub usage_patterns: Vec<UsagePattern>,

#[derive(Debug)]
pub struct CommercialExtractionDetector {

    pub config: DetectorConfig,

    pub state: Arc<RwLock<HashMap<String, UsagePattern>>>,

pub struct DetectorConfig {

    pub enabled: bool,

    pub sensitivity: f64,

    pub max_patterns: usize,}

impl CommercialExtractionDetector {

    #[must_use] pub fn new(config: DetectorConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
impl Default for DetectorConfig {}

    fn default() -> Self {
            enabled: true,
            sensitivity: 0.7,
            max_patterns: 1000,
impl Default for CommercialExtractionDetector {
        Self::new(DetectorConfig::default())
