use crate::threat::ml_engine::SmartThreatMLEngine;
use crate::threat::types::*;
use beardog_errors::BearDogError;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct ThreatDetectionEngine {
    pub config: ThreatDetectionConfig,

    pub active_threats: HashMap<String, ThreatEvent>,

    pub blocked_sources: HashSet<String>,

    pub quarantined_systems: HashSet<String>,

    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,

    pub detection_rules: Vec<DetectionRule>,

    pub stats: ThreatDetectionStats,

    pub ml_models: HashMap<String, MlModel>,

    pub ml_engine: Option<Arc<SmartThreatMLEngine>>,

    pub event_history: Arc<RwLock<Vec<ThreatEvent>>>,

    pub active_incidents: Arc<RwLock<HashMap<String, IncidentResponse>>>,
}
impl ThreatDetectionEngine {
    pub async fn new(config: ThreatDetectionConfig) -> Result<Self, BearDogError> {
        let ml_enhancement = config.ml_enhancement;
        let mut engine = Self {
            config,
            active_threats: HashMap::with_capacity(16),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::with_capacity(16),
            detection_rules: Vec::new(),
            stats: ThreatDetectionStats::default(),
            ml_models: HashMap::new(),
            ml_engine: None,
            event_history: Arc::new(RwLock::new(Vec::new())),
            active_incidents: Arc::new(RwLock::new(HashMap::new())),
        };

        if ml_enhancement {
            let ml_engine = SmartThreatMLEngine::new();
            engine.ml_engine = Some(Arc::new(ml_engine));
            info!("ML threat engine initialized successfully");
        }
        Ok(engine)
    }

    pub fn placeholder() -> Self {
        Self {
            config: ThreatDetectionConfig::default(),
            active_threats: HashMap::new(),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::new(),
            detection_rules: Vec::new(),
            stats: ThreatDetectionStats::default(),
            ml_models: HashMap::new(),
            ml_engine: None,
            event_history: Arc::new(RwLock::new(Vec::new())),
            active_incidents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
