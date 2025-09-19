// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::threat::ml_engine::SmartThreatMLEngine;
use crate::threat::types::engine::threat_engine::ThreatDetectionStats;
use crate::threat::types::{
    DetectionRule, IncidentResponse, MlModel, ThreatDetectionConfig, ThreatEvent,
    ThreatIntelligenceFeed,
};
use beardog_errors::BearDogError;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct ThreatDetectionEngine {
    pub config: ThreatDetectionConfig,

    /// Mapping of active threats
    /// Mapping of active threats
    pub active_threats: HashMap<String, ThreatEvent>,

    /// The blocked sources value
    /// The blocked sources value
    pub blocked_sources: HashSet<String>,

    /// The quarantined systems value
    /// The quarantined systems value
    pub quarantined_systems: HashSet<String>,

    /// Mapping of threat feeds
    /// Mapping of threat feeds
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,

    /// Collection of detection rules
    /// Collection of detection rules
    pub detection_rules: Vec<DetectionRule>,

    /// The stats value
    /// The stats value
    pub stats: ThreatDetectionStats,

    /// Mapping of ml models
    /// Mapping of ml models
    pub ml_models: HashMap<String, MlModel>,

    /// Optional ml engine
    /// Optional ml engine
    pub ml_engine: Option<Arc<SmartThreatMLEngine>>,

    /// The event history value
    /// The event history value
    pub event_history: Arc<RwLock<Vec<ThreatEvent>>>,

    pub active_incidents: Arc<RwLock<HashMap<String, IncidentResponse>>>,
}
impl ThreatDetectionEngine {
    /// New operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: ThreatDetectionConfig) -> Result<Self, BearDogError> {
        let ml_enhancement = config.ml_enhancement;
        let mut engine = Self {
            config,
            active_threats: HashMap::with_capacity(16),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::with_capacity(16),
            detection_rules: Vec::new(),
            stats: ThreatDetectionStats::default(),
            ml_models: HashMap::with_capacity(16),
            ml_engine: if ml_enhancement {
                Some(Arc::new(SmartThreatMLEngine::new()))
            } else {
                None
            },
            event_history: Arc::new(RwLock::new(Vec::new())),
            active_incidents: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        };

        if ml_enhancement {
            let ml_engine = SmartThreatMLEngine::new();
            engine.ml_engine = Some(Arc::new(ml_engine));
            info!("ML threat engine initialized successfully");
        }
        Ok(engine)
    }

    /// Removes rule
    /// Removes rule
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        self.remove_detection_rule(rule_id)
    }
}
