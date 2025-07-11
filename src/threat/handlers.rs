//! Implementation logic and handlers for threat detection
//! 
//! Contains the main business logic and implementation details for ThreatDetectionEngine.

use super::types::*;
use super::ml_engine::*;
use crate::{BearDogError, BearDogResult};

use chrono::{Duration, Utc, Timelike};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Enhanced Threat Detection Engine with ML capabilities
pub struct ThreatDetectionEngine {
    config: ThreatDetectionConfig,
    active_threats: HashMap<String, ThreatEvent>,
    blocked_sources: HashSet<String>,
    quarantined_systems: HashSet<String>,
    threat_feeds: HashMap<String, ThreatIntelligenceFeed>,
    detection_rules: Vec<ThreatDetectionRule>,
    stats: ThreatDetectionStats,
    ml_models: HashMap<String, MlModel>,
    ml_engine: Option<Arc<MlThreatEngine>>,
    event_history: Arc<RwLock<Vec<ThreatEvent>>>,
    active_incidents: Arc<RwLock<HashMap<String, IncidentResponse>>>,
}

impl ThreatDetectionEngine {
    /// Create a new threat detection engine
    pub async fn new(config: ThreatDetectionConfig) -> BearDogResult<Self> {
        let mut engine = Self {
            config: config.clone(),
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
        };

        // Initialize ML engine if enabled
        if config.ml_enhancement {
            let ml_config = MlEngineConfig::default();
            let ml_engine = MlThreatEngine::new(ml_config).await?;
            engine.ml_engine = Some(Arc::new(ml_engine));
            info!("🧠 ML-powered threat detection engine initialized");
        }

        // Load default detection rules
        engine.load_default_rules().await?;

        Ok(engine)
    }

    /// Create placeholder threat detection engine for testing
    pub fn placeholder() -> Self {
        // Create a placeholder with default config
        let config = crate::threat::types::ThreatDetectionConfig::default();
        // Since new() is async, we need to create a minimal placeholder
        Self {
            config,
            active_threats: HashMap::new(),
            detection_rules: Vec::new(),
            blocked_sources: HashSet::new(),
            quarantined_systems: HashSet::new(),
            threat_feeds: HashMap::new(),
            stats: crate::threat::types::ThreatDetectionStats::default(),
            ml_models: HashMap::new(),
            ml_engine: None,  // No ML engine for placeholder
            event_history: Arc::new(RwLock::new(Vec::new())),
            active_incidents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Analyze an event for potential threats
    pub async fn analyze_event(&mut self, event_data: &HashMap<String, String>) -> BearDogResult<Vec<ThreatEvent>> {
        if !self.config.real_time_detection {
            return Ok(vec![]);
        }

        let mut detected_threats = Vec::new();
        let ml_predictions: Vec<(String, f64)> = Vec::new();

        // Apply detection rules
        for rule in &self.detection_rules {
            if !rule.enabled {
                continue;
            }

            if self.matches_rule(event_data, rule).await? {
                let threat = self.create_threat_event(rule, event_data)?;
                if threat.score >= self.config.threat_threshold {
                    detected_threats.push(threat);
                }
            }
        }

        // Apply ML models if enabled
        if self.config.ml_enhancement {
            let ml_threats = self.analyze_with_ml(event_data).await?;
            detected_threats.extend(ml_threats);
        }

        // Check threat intelligence feeds
        let feed_threats = self.check_threat_feeds(event_data).await?;
        detected_threats.extend(feed_threats);

        // Store event in history
        {
            let mut history = self.event_history.write().await;
            history.extend(detected_threats.clone());
            
            // Limit history size
            if history.len() > 10000 {
                history.drain(0..1000);
            }
        }

        // Process detected threats
        for mut threat in &mut detected_threats {
            self.enrich_threat_event(&mut threat).await?;
            self.active_threats.insert(threat.id.clone(), threat.clone());
            
            // Auto-response if enabled
            if self.config.automated_response {
                self.execute_automated_response(&threat).await?;
            }
        }

        // Update statistics
        self.stats.total_threats += detected_threats.len() as u64;
        for threat in &detected_threats {
            let severity_key = threat.severity.to_string();
            *self.stats.threats_by_severity.entry(severity_key).or_insert(0) += 1;
            
            let type_key = threat.threat_type.to_string();
            *self.stats.threats_by_type.entry(type_key).or_insert(0) += 1;
        }

        // Trigger incident response if needed
        for threat in &detected_threats {
            if threat.severity >= ThreatSeverity::High {
                self.trigger_incident_response(threat).await?;
            }
        }

        Ok(detected_threats)
    }

    /// Check if event data matches a detection rule
    async fn matches_rule(&self, event_data: &HashMap<String, String>, rule: &ThreatDetectionRule) -> BearDogResult<bool> {
        for condition in &rule.conditions {
            if !self.evaluate_condition(condition, event_data).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Evaluate individual condition
    async fn evaluate_condition(&self, condition: &RuleCondition, event_data: &HashMap<String, String>) -> BearDogResult<bool> {
        match condition {
            RuleCondition::EventType { event_type } => {
                Ok(event_data.get("event_type").map_or(false, |t| t == event_type))
            }
            RuleCondition::SourceIp { ip_address } => {
                Ok(event_data.get("source_ip").map_or(false, |ip| ip == ip_address))
            }
            RuleCondition::UserAgent { user_agent } => {
                Ok(event_data.get("user_agent").map_or(false, |ua| ua == user_agent))
            }
            RuleCondition::DataSize { size_bytes } => {
                if let Some(size_str) = event_data.get("data_size") {
                    let size: u64 = size_str.parse().unwrap_or(0);
                    Ok(size > *size_bytes)
                } else {
                    Ok(false)
                }
            }
            RuleCondition::TimeRange { start_hour, end_hour } => {
                let current_hour = chrono::Utc::now().hour() as u8;
                if start_hour > end_hour {
                    // Crosses midnight
                    Ok(current_hour >= *start_hour || current_hour <= *end_hour)
                } else {
                    Ok(current_hour >= *start_hour && current_hour <= *end_hour)
                }
            }
            RuleCondition::FrequencyThreshold { count, window_minutes } => {
                let source_ip = event_data.get("source_ip").cloned();
                let window_start = chrono::Utc::now() - chrono::Duration::minutes(*window_minutes as i64);
                
                let recent_events = self.event_history.read().await
                    .iter()
                    .filter(|e| e.timestamp > window_start && e.source.ip_address == source_ip)
                    .count();
                
                Ok(recent_events >= *count as usize)
            }
            _ => Ok(false), // Default for complex conditions
        }
    }

    /// Create a threat event from a rule match
    fn create_threat_event(&self, rule: &ThreatDetectionRule, event_data: &HashMap<String, String>) -> BearDogResult<ThreatEvent> {
        let threat_id = Uuid::new_v4().to_string();
        
        let source = ThreatSource {
            ip_address: event_data.get("source_ip").cloned(),
            hostname: event_data.get("hostname").cloned(),
            geolocation: None, // Would be populated by IP geolocation service
            user_agent: event_data.get("user_agent").cloned(),
            reputation_score: 0.5, // Default neutral reputation
            threat_actor: None,
            classification: SourceClassification::Unknown,
        };

        let target = ThreatTarget {
            resource_id: event_data.get("resource_id").unwrap_or(&"unknown".to_string()).clone(),
            resource_type: event_data.get("resource_type").unwrap_or(&"unknown".to_string()).clone(),
            node_id: event_data.get("node_id").cloned(),
            user_account: event_data.get("user_account").cloned(),
            criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
        };

        let evidence = vec![ThreatEvidence {
            evidence_type: EvidenceType::RuleMatch,
            description: format!("Rule '{}' matched", rule.name),
            data: EvidenceData::Json(serde_json::to_value(event_data)?),
            collected_at: Utc::now(),
            chain_of_custody: vec!["ThreatDetectionEngine".to_string()],
            reliability: 0.9,
        }];

        let recommended_actions = match rule.severity {
            ThreatSeverity::Critical => vec![
                ThreatAction::AlertSecurityTeam,
                ThreatAction::QuarantineSystem,
                ThreatAction::InitiateIncidentResponse,
            ],
            ThreatSeverity::High => vec![
                ThreatAction::AlertSecurityTeam,
                ThreatAction::BlockSource,
            ],
            ThreatSeverity::Medium => vec![
                ThreatAction::AlertSecurityTeam,
            ],
            _ => vec![],
        };

        Ok(ThreatEvent {
            id: threat_id,
            threat_type: rule.threat_type.clone(),
            severity: rule.severity.clone(),
            score: rule.severity.to_score(),
            timestamp: Utc::now(),
            source,
            target,
            description: format!("Detection rule '{}' triggered: {}", rule.name, rule.description),
            detection_method: DetectionMethod::RuleBased,
            evidence,
            recommended_actions,
            status: ThreatStatus::New,
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
        })
    }

    /// Analyze event data using machine learning models
    async fn analyze_with_ml(&self, event_data: &HashMap<String, String>) -> BearDogResult<Vec<ThreatEvent>> {
        let mut ml_threats = Vec::new();

        for (model_id, model) in &self.ml_models {
            // Simulate ML analysis
            let prediction_score = self.simulate_ml_prediction(model, event_data)?;
            
            if prediction_score > 0.8 {
                let threat = ThreatEvent {
                    id: Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Anomaly,
                    severity: ThreatSeverity::from_score((prediction_score * 100.0) as u8),
                    score: (prediction_score * 100.0) as u8,
                    timestamp: Utc::now(),
                    source: ThreatSource {
                        ip_address: event_data.get("source_ip").cloned(),
                        hostname: event_data.get("hostname").cloned(),
                        geolocation: None,
                        user_agent: event_data.get("user_agent").cloned(),
                        reputation_score: 0.5,
                        threat_actor: None,
                        classification: SourceClassification::Suspicious,
                    },
                    target: ThreatTarget {
                        resource_id: event_data.get("resource_id").unwrap_or(&"unknown".to_string()).clone(),
                        resource_type: event_data.get("resource_type").unwrap_or(&"unknown".to_string()).clone(),
                        node_id: event_data.get("node_id").cloned(),
                        user_account: event_data.get("user_account").cloned(),
                        criticality: AssetCriticality::Medium,
                        protection_level: ProtectionLevel::Standard,
                    },
                    description: format!("ML model '{}' detected anomaly with confidence {:.2}", model.name, prediction_score),
                    detection_method: DetectionMethod::MachineLearning,
                    evidence: vec![],
                    recommended_actions: vec![ThreatAction::AlertSecurityTeam],
                    status: ThreatStatus::New,
                    assigned_analyst: None,
                    related_events: vec![],
                    mitigation_steps: vec![],
                };
                ml_threats.push(threat);
            }
        }

        Ok(ml_threats)
    }

    /// Simulate ML model prediction
    fn simulate_ml_prediction(&self, _model: &MlModel, _event_data: &HashMap<String, String>) -> BearDogResult<f64> {
        // In a real implementation, this would run the actual ML model
        // For now, return a random score for demonstration
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        _event_data.get("source_ip").unwrap_or(&"unknown".to_string()).hash(&mut hasher);
        _event_data.get("event_type").unwrap_or(&"unknown".to_string()).hash(&mut hasher);
        
        // Convert hash to a score between 0.0 and 1.0
        let hash = hasher.finish();
        let score = (hash % 100) as f64 / 100.0;
        Ok(score)
    }

    /// Check threat intelligence feeds
    async fn check_threat_feeds(&self, event_data: &HashMap<String, String>) -> BearDogResult<Vec<ThreatEvent>> {
        let mut feed_threats = Vec::new();

        for (_, feed) in &self.threat_feeds {
            if feed.status != FeedStatus::Active {
                continue;
            }

            for indicator in &feed.indicators {
                if self.matches_indicator(event_data, indicator).await? {
                    let threat = self.create_threat_from_indicator(indicator, event_data)?;
                    feed_threats.push(threat);
                }
            }
        }

        Ok(feed_threats)
    }

    /// Check if event data matches a threat indicator
    async fn matches_indicator(&self, event_data: &HashMap<String, String>, indicator: &ThreatIndicator) -> BearDogResult<bool> {
        let matches = match indicator.indicator_type {
            IndicatorType::IpAddress => {
                event_data.get("source_ip").map_or(false, |ip| ip.contains(&indicator.value))
            }
            IndicatorType::DomainName => {
                // Check if domain reputation is good when checking domain-based indicators
                if self.check_domain_reputation(&indicator.value).await? {
                    return Ok(false); // Good domain, not a threat
                }
                
                if let Some(domain) = event_data.get("domain") {
                    domain.contains(&indicator.value)
                } else if let Some(url) = event_data.get("url") {
                    url.contains(&indicator.value)
                } else {
                    false
                }
            }
            IndicatorType::Url => {
                event_data.get("url").map_or(false, |url| url.contains(&indicator.value))
            }
            IndicatorType::FileHash => {
                event_data.get("file_hash").map_or(false, |hash| hash == &indicator.value)
            }
            IndicatorType::EmailAddress => {
                event_data.get("email").map_or(false, |email| email.contains(&indicator.value))
            }
            IndicatorType::UserAgent => {
                event_data.get("user_agent").map_or(false, |ua| ua.contains(&indicator.value))
            }
            IndicatorType::ProcessName => {
                event_data.get("process_name").map_or(false, |proc| proc.contains(&indicator.value))
            }
            IndicatorType::NetworkPattern => {
                // Pattern matching for network traffic
                if let Some(pattern) = event_data.get("network_pattern") {
                    pattern.contains(&indicator.value)
                } else {
                    false
                }
            }
            IndicatorType::BehaviorPattern => {
                // Behavioral pattern matching
                if let Some(behavior) = event_data.get("behavior_pattern") {
                    behavior.contains(&indicator.value)
                } else {
                    false
                }
            }
            _ => false,
        };
        
        Ok(matches)
    }

    /// Create threat event from threat indicator
    fn create_threat_from_indicator(&self, indicator: &ThreatIndicator, event_data: &HashMap<String, String>) -> BearDogResult<ThreatEvent> {
        let threat_type = if !indicator.threat_types.is_empty() {
            indicator.threat_types[0].clone()
        } else {
            ThreatType::Unknown
        };

        let severity = ThreatSeverity::from_score((indicator.confidence * 100.0) as u8);

        Ok(ThreatEvent {
            id: Uuid::new_v4().to_string(),
            threat_type,
            severity: severity.clone(),
            score: (indicator.confidence * 100.0) as u8,
            timestamp: Utc::now(),
            source: ThreatSource {
                ip_address: event_data.get("source_ip").cloned(),
                hostname: event_data.get("hostname").cloned(),
                geolocation: None,
                user_agent: event_data.get("user_agent").cloned(),
                reputation_score: 1.0 - indicator.confidence,
                threat_actor: None,
                classification: SourceClassification::Malicious,
            },
            target: ThreatTarget {
                resource_id: event_data.get("resource_id").unwrap_or(&"unknown".to_string()).clone(),
                resource_type: event_data.get("resource_type").unwrap_or(&"unknown".to_string()).clone(),
                node_id: event_data.get("node_id").cloned(),
                user_account: event_data.get("user_account").cloned(),
                criticality: AssetCriticality::Medium,
                protection_level: ProtectionLevel::Standard,
            },
            description: format!("Threat intelligence match: {} ({})", indicator.value, indicator.indicator_type),
            detection_method: DetectionMethod::ThreatIntelligence,
            evidence: vec![],
            recommended_actions: vec![ThreatAction::BlockSource, ThreatAction::AlertSecurityTeam],
            status: ThreatStatus::New,
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
        })
    }

    /// Enrich threat event with additional context
    async fn enrich_threat_event(&self, threat: &mut ThreatEvent) -> BearDogResult<()> {
        // Add geolocation data
        if let Some(ip) = &threat.source.ip_address {
            threat.source.geolocation = self.get_geolocation(ip).await.ok();
        }

        // Add reputation scoring
        if let Some(ip) = &threat.source.ip_address {
            threat.source.reputation_score = self.get_reputation_score(ip).await.unwrap_or(0.5);
        }

        // Determine source classification based on reputation
        threat.source.classification = if threat.source.reputation_score < 0.3 {
            SourceClassification::Malicious
        } else if threat.source.reputation_score < 0.6 {
            SourceClassification::Suspicious
        } else {
            SourceClassification::Neutral
        };

        Ok(())
    }

    /// Get geolocation for IP address
    async fn get_geolocation(&self, _ip: &str) -> BearDogResult<GeoLocation> {
        // In practice, this would call a geolocation service
        Ok(GeoLocation {
            country: "Unknown".to_string(),
            region: None,
            city: None,
            latitude: None,
            longitude: None,
            is_tor_exit: false,
            is_vpn: false,
            is_proxy: false,
        })
    }

    /// Get reputation score for IP address
    async fn get_reputation_score(&self, _ip: &str) -> BearDogResult<f64> {
        // In practice, this would query reputation databases
        Ok(0.5) // Neutral reputation
    }

    /// Execute automated response actions
    async fn execute_automated_response(&mut self, threat: &ThreatEvent) -> BearDogResult<()> {
        for action in &threat.recommended_actions {
            match action {
                ThreatAction::BlockSource => {
                    if let Some(ip) = &threat.source.ip_address {
                        self.blocked_sources.insert(ip.clone());
                        self.stats.blocked_sources += 1;
                    }
                }
                ThreatAction::QuarantineSystem => {
                    if let Some(node_id) = &threat.target.node_id {
                        self.quarantined_systems.insert(node_id.clone());
                        self.stats.quarantined_systems += 1;
                    }
                }
                ThreatAction::AlertSecurityTeam => {
                    // Send alert notification
                    self.send_alert_notification(threat).await?;
                }
                _ => {
                    // Other actions require manual intervention
                }
            }
        }

        Ok(())
    }

    /// Send alert notification
    async fn send_alert_notification(&self, threat: &ThreatEvent) -> BearDogResult<()> {
        for endpoint in &self.config.notification_endpoints {
            // In practice, this would send to actual notification systems
            println!("ALERT: {} - {} (Severity: {})", threat.threat_type, threat.description, threat.severity);
        }
        Ok(())
    }

    /// Add a detection rule
    pub fn add_detection_rule(&mut self, rule: ThreatDetectionRule) {
        self.detection_rules.push(rule);
    }

    /// Remove a detection rule
    pub fn remove_detection_rule(&mut self, rule_id: &str) -> BearDogResult<()> {
        self.detection_rules.retain(|rule| rule.rule_id != rule_id);
        Ok(())
    }

    /// Update threat intelligence feed
    pub async fn update_threat_feed(&mut self, feed: ThreatIntelligenceFeed) -> BearDogResult<()> {
        self.threat_feeds.insert(feed.id.clone(), feed);
        Ok(())
    }

    /// Get threat statistics
    pub fn get_statistics(&self) -> &ThreatDetectionStats {
        &self.stats
    }

    /// Get active threats
    pub fn get_active_threats(&self) -> Vec<&ThreatEvent> {
        self.active_threats.values()
            .filter(|threat| matches!(threat.status, ThreatStatus::New | ThreatStatus::Investigating | ThreatStatus::Confirmed))
            .collect()
    }

    /// Resolve a threat
    pub async fn resolve_threat(&mut self, threat_id: &str, mitigation_steps: Vec<MitigationStep>) -> BearDogResult<()> {
        if let Some(threat) = self.active_threats.get_mut(threat_id) {
            threat.status = ThreatStatus::Resolved;
            threat.mitigation_steps = mitigation_steps;
            Ok(())
        } else {
            Err(BearDogError::NotFound {
                resource_type: "Threat".to_string(),
                id: threat_id.to_string(),
            })
        }
    }

    /// Mark threat as false positive
    pub async fn mark_false_positive(&mut self, threat_id: &str) -> BearDogResult<()> {
        if let Some(threat) = self.active_threats.get_mut(threat_id) {
            threat.status = ThreatStatus::FalsePositive;
            
            // Update rule statistics
            for rule in &mut self.detection_rules {
                if rule.threat_type == threat.threat_type {
                    rule.false_positive_rate += 1.0;
                    break;
                }
            }
            
            // Update global false positive rate
            let total_resolved = self.active_threats.values()
                .filter(|t| matches!(t.status, ThreatStatus::Resolved | ThreatStatus::FalsePositive))
                .count() as f64;
            let false_positives = self.active_threats.values()
                .filter(|t| t.status == ThreatStatus::FalsePositive)
                .count() as f64;
            
            if total_resolved > 0.0 {
                self.stats.false_positive_rate = false_positives / total_resolved;
            }
            
            Ok(())
        } else {
            Err(BearDogError::NotFound {
                resource_type: "Threat".to_string(),
                id: threat_id.to_string(),
            })
        }
    }

    /// Cleanup old threats
    pub async fn cleanup_old_threats(&mut self, retention_days: u32) -> BearDogResult<()> {
        let cutoff_date = Utc::now() - Duration::days(retention_days as i64);
        
        self.active_threats.retain(|_, threat| {
            threat.timestamp > cutoff_date && 
            !matches!(threat.status, ThreatStatus::Resolved | ThreatStatus::FalsePositive)
        });
        
        Ok(())
    }

    /// Add ML model
    pub fn add_ml_model(&mut self, model: MlModel) {
        self.ml_models.insert(model.id.clone(), model);
    }

    /// Get threat by ID
    pub fn get_threat(&self, threat_id: &str) -> Option<&ThreatEvent> {
        self.active_threats.get(threat_id)
    }

    /// Update threat status
    pub async fn update_threat_status(&mut self, threat_id: &str, status: ThreatStatus, analyst: Option<String>) -> BearDogResult<()> {
        if let Some(threat) = self.active_threats.get_mut(threat_id) {
            threat.status = status;
            threat.assigned_analyst = analyst;
            Ok(())
        } else {
            Err(BearDogError::NotFound {
                resource_type: "Threat".to_string(),
                id: threat_id.to_string(),
            })
        }
    }

    /// Trigger incident response
    async fn trigger_incident_response(&self, threat: &ThreatEvent) -> BearDogResult<()> {
        let incident_id = format!("incident_{}", uuid::Uuid::new_v4());
        
        let incident = IncidentResponse {
            incident_id: incident_id.clone(),
            threat_id: threat.id.clone(),
            severity: threat.severity.clone(),
            status: IncidentStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            assigned_to: None,
            description: threat.description.clone(),
            containment_actions: Vec::new(),
            remediation_actions: Vec::new(),
            lessons_learned: Vec::new(),
        };

        // Store incident
        {
            let mut incidents = self.active_incidents.write().await;
            incidents.insert(incident_id.clone(), incident);
        }

        // Execute automated response actions
        for action in &threat.recommended_actions {
            self.execute_response_action(action, &incident_id).await?;
        }

        info!("🚨 Incident {} triggered for threat {}", incident_id, threat.id);
        Ok(())
    }

    /// Execute response action
    async fn execute_response_action(&self, action: &ThreatAction, incident_id: &str) -> BearDogResult<()> {
        match action {
            ThreatAction::BlockSource => {
                info!("🚫 Blocking source for incident {}", incident_id);
                // TODO: Implement actual source blocking
            }
            ThreatAction::QuarantineSystem => {
                info!("🔒 Quarantining system for incident {}", incident_id);
                // TODO: Implement system quarantine
            }
            ThreatAction::AlertSecurityTeam => {
                warn!("📢 Admin notification [{}]: Security team alerted", incident_id);
                // TODO: Implement admin notification
            }
            _ => {
                // Other actions require manual intervention
            }
        }
        Ok(())
    }

    /// Load default detection rules
    async fn load_default_rules(&mut self) -> BearDogResult<()> {
        // Suspicious login attempts
        self.detection_rules.push(ThreatDetectionRule {
            rule_id: "suspicious_login_001".to_string(),
            name: "Suspicious Login Hours".to_string(),
            description: "Login attempts during unusual hours".to_string(),
            threat_type: ThreatType::SuspiciousLogin,
            severity: ThreatSeverity::Medium,
            conditions: vec![
                RuleCondition::EventType { event_type: "login".to_string() },
                RuleCondition::TimeRange { start_hour: 22, end_hour: 6 },
            ],
            false_positive_rate: 0.15,
            mitre_techniques: vec!["T1078".to_string()],
            response_actions: vec![
                ResponseAction::LogAlert("Suspicious login during off-hours".to_string()),
                ResponseAction::NotifyAdmin("User login outside business hours".to_string()),
            ],
            enabled: true,
        });

        // Large data transfers
        self.detection_rules.push(ThreatDetectionRule {
            rule_id: "data_exfil_001".to_string(),
            name: "Large Data Transfer".to_string(),
            description: "Unusually large data transfer detected".to_string(),
            threat_type: ThreatType::DataExfiltration,
            severity: ThreatSeverity::High,
            conditions: vec![
                RuleCondition::DataSize { size_bytes: 1024 * 1024 * 500 }, // 500MB
            ],
            false_positive_rate: 0.05,
            mitre_techniques: vec!["T1041".to_string(), "T1020".to_string()],
            response_actions: vec![
                ResponseAction::LogAlert("Large data transfer detected".to_string()),
                ResponseAction::NotifyAdmin("Potential data exfiltration detected".to_string()),
            ],
            enabled: true,
        });

        // Brute force attacks
        self.detection_rules.push(ThreatDetectionRule {
            rule_id: "brute_force_001".to_string(),
            name: "Brute Force Attack".to_string(),
            description: "Multiple failed login attempts from same IP".to_string(),
            threat_type: ThreatType::BruteForceAttack,
            severity: ThreatSeverity::High,
            conditions: vec![
                RuleCondition::EventType { event_type: "login_failed".to_string() },
                RuleCondition::FrequencyThreshold { count: 10, window_minutes: 5 },
            ],
            false_positive_rate: 0.03,
            mitre_techniques: vec!["T1110".to_string()],
            response_actions: vec![
                ResponseAction::BlockIp("Source IP".to_string()),
                ResponseAction::LogAlert("Brute force attack detected".to_string()),
            ],
            enabled: true,
        });

        info!("📋 Loaded {} default detection rules", self.detection_rules.len());
        Ok(())
    }

    /// Check domain reputation
    async fn check_domain_reputation(&self, domain: &str) -> BearDogResult<bool> {
        // Simplified reputation check - in reality would query threat intelligence feeds
        let suspicious_domains = vec!["malicious.com", "phishing.net", "spam.org"];
        Ok(!suspicious_domains.contains(&domain))
    }
}

/// Threat analysis result
#[derive(Debug, Clone)]
pub struct ThreatAnalysisResult {
    pub event_id: String,
    pub threats_detected: usize,
    pub detected_threats: Vec<ThreatEvent>,
    pub ml_predictions: Vec<MlPrediction>,
    pub analysis_duration_ms: u64,
    pub recommendations: Vec<String>,
}

/// Threat statistics
#[derive(Debug, Clone)]
pub struct ThreatStatistics {
    pub total_threats: usize,
    pub high_severity_threats: usize,
    pub medium_severity_threats: usize,
    pub low_severity_threats: usize,
    pub active_incidents: usize,
    pub detection_rules_count: usize,
    pub threat_feeds_count: usize,
    pub ml_models_count: usize,
}

/// Incident response structure
#[derive(Debug, Clone)]
pub struct IncidentResponse {
    pub incident_id: String,
    pub threat_id: String,
    pub severity: ThreatSeverity,
    pub status: IncidentStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub assigned_to: Option<String>,
    pub description: String,
    pub containment_actions: Vec<String>,
    pub remediation_actions: Vec<String>,
    pub lessons_learned: Vec<String>,
}

/// Incident status
#[derive(Debug, Clone, PartialEq)]
pub enum IncidentStatus {
    Open,
    InProgress,
    Contained,
    Resolved,
    Closed,
}
