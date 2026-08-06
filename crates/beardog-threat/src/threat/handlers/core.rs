// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primary async [`ThreatDetectionEngine`](crate::threat::handlers::core::ThreatDetectionEngine) with feeds, rules, ML hooks, and incident tracking.

use crate::threat::ml_engine::SmartThreatMLEngine;
use crate::threat::types::engine::threat_engine::ThreatDetectionStats;
use crate::threat::types::incidents::ManagedIncident;
use crate::threat::types::{
    DetectionRule, IncidentResponse, MlModel, ThreatDetectionConfig, ThreatEvent,
    ThreatIntelligenceFeed,
};
use beardog_errors::BearDogError;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Production-oriented engine surface extended across `handlers::*` impl blocks.
pub struct ThreatDetectionEngine {
    /// Loaded [`ThreatDetectionConfig`].
    pub config: ThreatDetectionConfig,

    /// Mapping of active threats
    pub active_threats: HashMap<String, ThreatEvent>,

    /// The blocked sources value
    pub blocked_sources: HashSet<String>,

    /// The quarantined systems value
    pub quarantined_systems: HashSet<String>,

    /// Mapping of threat feeds
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,

    /// Collection of detection rules
    pub detection_rules: Vec<DetectionRule>,

    /// The stats value
    pub stats: ThreatDetectionStats,

    /// Mapping of ml models
    pub ml_models: HashMap<String, MlModel>,

    /// Optional ml engine
    pub ml_engine: Option<Arc<SmartThreatMLEngine>>,

    /// The event history value
    pub event_history: Arc<RwLock<Vec<ThreatEvent>>>,

    /// Open incidents keyed by incident id, shared with async maintenance tasks.
    pub active_incidents: Arc<RwLock<HashMap<String, IncidentResponse>>>,

    /// Phase 1 in-memory lifecycle store (sync API); ordered by incident id.
    pub managed_incidents: Arc<parking_lot::RwLock<BTreeMap<String, ManagedIncident>>>,
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
            managed_incidents: Arc::new(parking_lot::RwLock::new(BTreeMap::new())),
        };

        if ml_enhancement {
            let ml_engine = SmartThreatMLEngine::new();
            engine.ml_engine = Some(Arc::new(ml_engine));
            info!("ML threat engine initialized successfully");
        }
        Ok(engine)
    }

    /// Removes rule
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        self.remove_detection_rule(rule_id)
    }
}
