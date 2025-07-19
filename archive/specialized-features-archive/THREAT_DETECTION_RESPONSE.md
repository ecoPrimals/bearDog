# BearDog Threat Detection & Response Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** CRITICAL  

## 🎯 **Overview**

BearDog's Threat Detection & Response system provides real-time security monitoring and automated incident response:
- **Real-time threat detection** using ML and behavioral analysis
- **Automated incident response** with configurable playbooks
- **Advanced persistent threat (APT)** detection
- **Zero-day attack protection**
- **Threat intelligence integration**
- **Security orchestration, automation and response (SOAR)**

## 🛡️ **Threat Detection Architecture**

### **Core Threat Detection Engine**
```rust
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

pub struct ThreatDetectionEngine {
    config: Arc<ThreatDetectionConfig>,
    ml_models: HashMap<ThreatType, Arc<dyn ThreatModel>>,
    behavioral_analyzer: Arc<BehavioralAnalyzer>,
    anomaly_detector: Arc<AnomalyDetector>,
    threat_intelligence: Arc<ThreatIntelligenceProvider>,
    pattern_matcher: Arc<PatternMatcher>,
    
    // Detection state
    active_threats: Arc<RwLock<HashMap<String, ActiveThreat>>>,
    threat_cache: Arc<RwLock<ThreatCache>>,
    detection_metrics: Arc<RwLock<DetectionMetrics>>,
    
    // Response components
    response_engine: Arc<ResponseEngine>,
    incident_manager: Arc<IncidentManager>,
    notification_system: Arc<ThreatNotificationSystem>,
}

impl ThreatDetectionEngine {
    pub async fn new(config: ThreatDetectionConfig) -> Result<Self> {
        let mut ml_models: HashMap<ThreatType, Arc<dyn ThreatModel>> = HashMap::new();
        
        // Initialize ML models for different threat types
        if config.ml_detection.enable_login_anomaly {
            ml_models.insert(
                ThreatType::LoginAnomaly,
                Arc::new(LoginAnomalyModel::load(&config.ml_detection.models.login_anomaly_path).await?)
            );
        }
        
        if config.ml_detection.enable_access_pattern {
            ml_models.insert(
                ThreatType::AccessPatternAnomaly,
                Arc::new(AccessPatternModel::load(&config.ml_detection.models.access_pattern_path).await?)
            );
        }
        
        if config.ml_detection.enable_behavioral {
            ml_models.insert(
                ThreatType::BehavioralAnomaly,
                Arc::new(BehavioralModel::load(&config.ml_detection.models.behavioral_path).await?)
            );
        }
        
        if config.ml_detection.enable_data_exfiltration {
            ml_models.insert(
                ThreatType::DataExfiltration,
                Arc::new(DataExfiltrationModel::load(&config.ml_detection.models.data_exfiltration_path).await?)
            );
        }
        
        let behavioral_analyzer = Arc::new(BehavioralAnalyzer::new(&config.behavioral_analysis).await?);
        let anomaly_detector = Arc::new(AnomalyDetector::new(&config.anomaly_detection).await?);
        let threat_intelligence = Arc::new(ThreatIntelligenceProvider::new(&config.threat_intelligence).await?);
        let pattern_matcher = Arc::new(PatternMatcher::new(&config.pattern_matching).await?);
        
        let response_engine = Arc::new(ResponseEngine::new(&config.response).await?);
        let incident_manager = Arc::new(IncidentManager::new(&config.incident_management).await?);
        let notification_system = Arc::new(ThreatNotificationSystem::new(&config.notifications).await?);
        
        Ok(Self {
            config: Arc::new(config),
            ml_models,
            behavioral_analyzer,
            anomaly_detector,
            threat_intelligence,
            pattern_matcher,
            active_threats: Arc::new(RwLock::new(HashMap::new())),
            threat_cache: Arc::new(RwLock::new(ThreatCache::new(10000))),
            detection_metrics: Arc::new(RwLock::new(DetectionMetrics::new())),
            response_engine,
            incident_manager,
            notification_system,
        })
    }
    
    pub async fn analyze_security_event(&self, event: SecurityEvent) -> Result<ThreatAnalysisResult> {
        let analysis_start = std::time::Instant::now();
        let mut threat_indicators = Vec::new();
        let mut threat_level = ThreatLevel::None;
        
        // Quick cache lookup
        if let Some(cached_result) = self.check_threat_cache(&event).await? {
            return Ok(cached_result);
        }
        
        // ML-based threat detection
        for (threat_type, model) in &self.ml_models {
            let prediction = model.predict(&event).await?;
            
            if prediction.probability > self.config.ml_detection.threshold {
                threat_indicators.push(ThreatIndicator {
                    indicator_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: threat_type.clone(),
                    confidence: prediction.confidence,
                    severity: ThreatSeverity::from_probability(prediction.probability),
                    description: prediction.description,
                    evidence: prediction.evidence,
                    mitre_techniques: prediction.mitre_techniques,
                    recommendations: prediction.recommendations,
                });
                
                threat_level = threat_level.max(prediction.threat_level);
            }
        }
        
        // Behavioral analysis
        if self.config.behavioral_analysis.enabled {
            let behavioral_result = self.behavioral_analyzer.analyze(&event).await?;
            
            if behavioral_result.anomaly_score > self.config.behavioral_analysis.threshold {
                threat_indicators.push(ThreatIndicator {
                    indicator_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::BehavioralAnomaly,
                    confidence: behavioral_result.confidence,
                    severity: ThreatSeverity::from_score(behavioral_result.anomaly_score),
                    description: behavioral_result.description,
                    evidence: behavioral_result.evidence,
                    mitre_techniques: behavioral_result.mitre_techniques,
                    recommendations: behavioral_result.recommendations,
                });
                
                threat_level = threat_level.max(ThreatLevel::Medium);
            }
        }
        
        // Statistical anomaly detection
        if self.config.anomaly_detection.enabled {
            let anomaly_results = self.anomaly_detector.detect_anomalies(&event).await?;
            
            for anomaly in anomaly_results {
                if anomaly.severity >= self.config.anomaly_detection.min_severity {
                    threat_indicators.push(ThreatIndicator {
                        indicator_id: uuid::Uuid::new_v4().to_string(),
                        threat_type: ThreatType::StatisticalAnomaly,
                        confidence: anomaly.confidence,
                        severity: anomaly.severity,
                        description: anomaly.description,
                        evidence: anomaly.evidence,
                        mitre_techniques: anomaly.mitre_techniques,
                        recommendations: anomaly.recommendations,
                    });
                    
                    threat_level = threat_level.max(ThreatLevel::Medium);
                }
            }
        }
        
        // Threat intelligence lookup
        if self.config.threat_intelligence.enabled {
            let intel_results = self.threat_intelligence.lookup_indicators(&event).await?;
            
            for intel_hit in intel_results {
                threat_indicators.push(ThreatIndicator {
                    indicator_id: uuid::Uuid::new_v4().to_string(),
                    threat_type: ThreatType::ThreatIntelligence,
                    confidence: intel_hit.confidence,
                    severity: intel_hit.severity,
                    description: format!("Threat intelligence match: {}", intel_hit.description),
                    evidence: intel_hit.evidence,
                    mitre_techniques: intel_hit.mitre_techniques,
                    recommendations: intel_hit.recommendations,
                });
                
                threat_level = threat_level.max(intel_hit.threat_level);
            }
        }
        
        // Pattern matching
        if self.config.pattern_matching.enabled {
            let pattern_matches = self.pattern_matcher.match_patterns(&event).await?;
            
            for pattern_match in pattern_matches {
                if pattern_match.confidence > self.config.pattern_matching.threshold {
                    threat_indicators.push(ThreatIndicator {
                        indicator_id: uuid::Uuid::new_v4().to_string(),
                        threat_type: ThreatType::PatternMatch,
                        confidence: pattern_match.confidence,
                        severity: pattern_match.severity,
                        description: format!("Pattern match: {}", pattern_match.pattern_name),
                        evidence: pattern_match.evidence,
                        mitre_techniques: pattern_match.mitre_techniques,
                        recommendations: pattern_match.recommendations,
                    });
                    
                    threat_level = threat_level.max(pattern_match.threat_level);
                }
            }
        }
        
        // Create comprehensive threat analysis result
        let result = ThreatAnalysisResult {
            event_id: event.id.clone(),
            analysis_id: uuid::Uuid::new_v4().to_string(),
            threat_level,
            threat_indicators,
            risk_score: self.calculate_risk_score(&threat_indicators),
            recommended_actions: self.generate_recommended_actions(&threat_indicators),
            analysis_time_ms: analysis_start.elapsed().as_millis() as u64,
            analyzed_at: Utc::now(),
            metadata: HashMap::new(),
        };
        
        // Cache the result
        self.cache_threat_analysis(&event, &result).await?;
        
        // Update metrics
        self.update_detection_metrics(&result).await?;
        
        // Trigger response if threat level is high enough
        if threat_level >= self.config.response.auto_response_threshold {
            self.trigger_automated_response(&event, &result).await?;
        }
        
        Ok(result)
    }
    
    async fn trigger_automated_response(&self, event: &SecurityEvent, analysis: &ThreatAnalysisResult) -> Result<()> {
        // Create incident if threshold is met
        if analysis.threat_level >= ThreatLevel::High {
            let incident = self.incident_manager.create_incident(event, analysis).await?;
            
            // Execute automated response playbook
            self.response_engine.execute_response_playbook(&incident).await?;
            
            // Send notifications
            self.notification_system.send_threat_alert(&incident, analysis).await?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ThreatType {
    LoginAnomaly,
    AccessPatternAnomaly,
    BehavioralAnomaly,
    DataExfiltration,
    PrivilegeEscalation,
    LateralMovement,
    Persistence,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    Collection,
    CommandAndControl,
    Exfiltration,
    Impact,
    ThreatIntelligence,
    StatisticalAnomaly,
    PatternMatch,
    ZeroDay,
    Apt,
    InsiderThreat,
    Malware,
    Phishing,
    SocialEngineering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_id: String,
    pub threat_type: ThreatType,
    pub confidence: f64,
    pub severity: ThreatSeverity,
    pub description: String,
    pub evidence: Vec<EvidenceItem>,
    pub mitre_techniques: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl ThreatSeverity {
    pub fn from_probability(probability: f64) -> Self {
        match probability {
            p if p >= 0.9 => ThreatSeverity::Critical,
            p if p >= 0.7 => ThreatSeverity::High,
            p if p >= 0.5 => ThreatSeverity::Medium,
            p if p >= 0.3 => ThreatSeverity::Low,
            _ => ThreatSeverity::Info,
        }
    }
    
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.8 => ThreatSeverity::Critical,
            s if s >= 0.6 => ThreatSeverity::High,
            s if s >= 0.4 => ThreatSeverity::Medium,
            s if s >= 0.2 => ThreatSeverity::Low,
            _ => ThreatSeverity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub evidence_type: EvidenceType,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    LogEntry,
    NetworkFlow,
    FileHash,
    ProcessExecution,
    RegistryModification,
    NetworkConnection,
    UserBehavior,
    SystemCall,
    ApiCall,
    DatabaseQuery,
}
```

## 🤖 **Machine Learning Models**

### **Login Anomaly Detection Model**
```rust
use ort::{Environment, ExecutionProvider, Session, SessionBuilder, Value};

pub struct LoginAnomalyModel {
    session: Session,
    input_names: Vec<String>,
    output_names: Vec<String>,
    feature_scaler: FeatureScaler,
}

impl LoginAnomalyModel {
    pub async fn load(model_path: &str) -> Result<Self> {
        let environment = Environment::builder()
            .with_name("LoginAnomalyModel")
            .build()?;
        
        let session = SessionBuilder::new(&environment)?
            .with_execution_providers([ExecutionProvider::CPU(Default::default())])?
            .with_model_from_file(model_path)?;
        
        let input_names = session.inputs.iter().map(|input| input.name.clone()).collect();
        let output_names = session.outputs.iter().map(|output| output.name.clone()).collect();
        
        // Load feature scaler
        let scaler_path = format!("{}.scaler", model_path);
        let feature_scaler = FeatureScaler::load(&scaler_path)?;
        
        Ok(Self {
            session,
            input_names,
            output_names,
            feature_scaler,
        })
    }
}

#[async_trait]
impl ThreatModel for LoginAnomalyModel {
    async fn predict(&self, event: &SecurityEvent) -> Result<ThreatPrediction> {
        // Extract features from security event
        let features = self.extract_login_features(event)?;
        
        // Scale features
        let scaled_features = self.feature_scaler.transform(&features)?;
        
        // Create input tensor
        let input_tensor = Array2::from_shape_vec((1, scaled_features.len()), scaled_features)?;
        let input_value = Value::from_array(self.session.allocator(), &input_tensor)?;
        
        // Run inference
        let outputs = self.session.run(vec![input_value])?;
        
        // Extract prediction results
        let prediction_scores: Vec<f32> = outputs[0].try_extract()?;
        let anomaly_probability = prediction_scores[0] as f64;
        
        // Interpret results
        let threat_level = if anomaly_probability > 0.8 {
            ThreatLevel::High
        } else if anomaly_probability > 0.6 {
            ThreatLevel::Medium
        } else if anomaly_probability > 0.4 {
            ThreatLevel::Low
        } else {
            ThreatLevel::None
        };
        
        Ok(ThreatPrediction {
            probability: anomaly_probability,
            confidence: self.calculate_confidence(anomaly_probability),
            threat_level,
            description: self.generate_description(anomaly_probability, &features),
            evidence: self.generate_evidence(event, &features),
            mitre_techniques: vec!["T1078".to_string()], // Valid Accounts
            recommendations: self.generate_recommendations(anomaly_probability),
        })
    }
    
    fn model_type(&self) -> ThreatType {
        ThreatType::LoginAnomaly
    }
    
    fn model_version(&self) -> String {
        "1.0.0".to_string()
    }
}

impl LoginAnomalyModel {
    fn extract_login_features(&self, event: &SecurityEvent) -> Result<Vec<f64>> {
        let mut features = Vec::new();
        
        // Time-based features
        let hour_of_day = event.timestamp.hour() as f64;
        let day_of_week = event.timestamp.weekday().num_days_from_monday() as f64;
        let is_weekend = if day_of_week >= 5.0 { 1.0 } else { 0.0 };
        
        features.extend_from_slice(&[hour_of_day, day_of_week, is_weekend]);
        
        // Location-based features
        if let Some(location) = &event.location {
            let is_new_location = self.is_new_location(&event.user_id, location)? as u32 as f64;
            let distance_from_usual = self.calculate_distance_from_usual(&event.user_id, location)?;
            
            features.extend_from_slice(&[is_new_location, distance_from_usual]);
        } else {
            features.extend_from_slice(&[0.0, 0.0]);
        }
        
        // Device-based features
        if let Some(device) = &event.device_info {
            let is_new_device = self.is_new_device(&event.user_id, device)? as u32 as f64;
            let device_risk_score = self.calculate_device_risk_score(device)?;
            
            features.extend_from_slice(&[is_new_device, device_risk_score]);
        } else {
            features.extend_from_slice(&[0.0, 0.0]);
        }
        
        // Behavioral features
        let login_frequency = self.get_login_frequency(&event.user_id)?;
        let time_since_last_login = self.get_time_since_last_login(&event.user_id)?;
        let failed_attempts_recently = self.get_recent_failed_attempts(&event.user_id)? as f64;
        
        features.extend_from_slice(&[login_frequency, time_since_last_login, failed_attempts_recently]);
        
        Ok(features)
    }
    
    fn generate_description(&self, probability: f64, features: &[f64]) -> String {
        if probability > 0.8 {
            "High anomaly score detected for login event. Multiple suspicious indicators present.".to_string()
        } else if probability > 0.6 {
            "Moderate anomaly score detected. Some unusual login characteristics identified.".to_string()
        } else {
            "Low anomaly score. Login appears normal.".to_string()
        }
    }
    
    fn generate_recommendations(&self, probability: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if probability > 0.8 {
            recommendations.extend_from_slice(&[
                "Require additional authentication factors".to_string(),
                "Temporarily suspend account pending investigation".to_string(),
                "Contact user through alternative communication channel".to_string(),
                "Review recent account activity for other anomalies".to_string(),
            ]);
        } else if probability > 0.6 {
            recommendations.extend_from_slice(&[
                "Monitor subsequent user activity closely".to_string(),
                "Consider requiring step-up authentication".to_string(),
                "Log detailed session information".to_string(),
            ]);
        }
        
        recommendations
    }
}
```

### **Behavioral Analysis Engine**
```rust
pub struct BehavioralAnalyzer {
    config: BehavioralAnalysisConfig,
    baseline_store: Arc<dyn BaselineStore>,
    pattern_learner: Arc<PatternLearner>,
    deviation_calculator: Arc<DeviationCalculator>,
}

impl BehavioralAnalyzer {
    pub async fn analyze(&self, event: &SecurityEvent) -> Result<BehavioralAnalysisResult> {
        // Get user baseline
        let baseline = self.baseline_store.get_user_baseline(&event.user_id).await?;
        
        // Extract behavioral features
        let current_behavior = self.extract_behavioral_features(event)?;
        
        // Calculate deviation from baseline
        let deviation = self.deviation_calculator.calculate_deviation(&baseline, &current_behavior)?;
        
        // Calculate anomaly score
        let anomaly_score = self.calculate_anomaly_score(&deviation)?;
        
        // Generate analysis result
        Ok(BehavioralAnalysisResult {
            user_id: event.user_id.clone(),
            event_id: event.id.clone(),
            anomaly_score,
            confidence: self.calculate_confidence(&deviation),
            deviations: deviation.significant_deviations,
            baseline_comparison: deviation.baseline_comparison,
            description: self.generate_behavioral_description(&deviation),
            evidence: self.generate_behavioral_evidence(event, &deviation),
            mitre_techniques: self.map_to_mitre_techniques(&deviation),
            recommendations: self.generate_behavioral_recommendations(&deviation),
        })
    }
    
    fn extract_behavioral_features(&self, event: &SecurityEvent) -> Result<BehavioralFeatures> {
        Ok(BehavioralFeatures {
            access_patterns: self.extract_access_patterns(event)?,
            timing_patterns: self.extract_timing_patterns(event)?,
            resource_usage: self.extract_resource_usage(event)?,
            interaction_patterns: self.extract_interaction_patterns(event)?,
            navigation_patterns: self.extract_navigation_patterns(event)?,
        })
    }
    
    fn calculate_anomaly_score(&self, deviation: &BehavioralDeviation) -> Result<f64> {
        let mut score = 0.0;
        let mut weight_sum = 0.0;
        
        for (feature, deviation_value) in &deviation.feature_deviations {
            let weight = self.config.feature_weights.get(feature).unwrap_or(&1.0);
            score += deviation_value * weight;
            weight_sum += weight;
        }
        
        Ok(if weight_sum > 0.0 { score / weight_sum } else { 0.0 })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralFeatures {
    pub access_patterns: AccessPatterns,
    pub timing_patterns: TimingPatterns,
    pub resource_usage: ResourceUsage,
    pub interaction_patterns: InteractionPatterns,
    pub navigation_patterns: NavigationPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatterns {
    pub resources_accessed: Vec<String>,
    pub access_frequency: HashMap<String, u32>,
    pub access_sequence: Vec<String>,
    pub privilege_level_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingPatterns {
    pub session_duration: Duration,
    pub time_between_actions: Vec<Duration>,
    pub active_hours: Vec<u8>,
    pub peak_activity_periods: Vec<(DateTime<Utc>, DateTime<Utc>)>,
}
```

## 🚨 **Incident Response Engine**

### **Automated Response Playbooks**
```rust
pub struct ResponseEngine {
    config: ResponseConfig,
    playbooks: HashMap<ThreatType, ResponsePlaybook>,
    action_executors: HashMap<ResponseActionType, Box<dyn ResponseActionExecutor>>,
    orchestrator: Arc<ResponseOrchestrator>,
}

impl ResponseEngine {
    pub async fn execute_response_playbook(&self, incident: &SecurityIncident) -> Result<ResponseResult> {
        let playbook = self.select_appropriate_playbook(incident)?;
        
        let execution_context = ResponseExecutionContext {
            incident: incident.clone(),
            playbook: playbook.clone(),
            start_time: Utc::now(),
            environment: self.get_environment_context().await?,
        };
        
        let result = self.orchestrator.execute_playbook(&execution_context).await?;
        
        // Log response execution
        self.log_response_execution(&execution_context, &result).await?;
        
        Ok(result)
    }
    
    fn select_appropriate_playbook(&self, incident: &SecurityIncident) -> Result<&ResponsePlaybook> {
        // Select playbook based on threat type and severity
        let primary_threat_type = incident.primary_threat_type();
        
        self.playbooks.get(&primary_threat_type)
            .or_else(|| self.playbooks.get(&ThreatType::Default))
            .ok_or_else(|| BearDogError::NoPlaybookFound(primary_threat_type))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePlaybook {
    pub name: String,
    pub threat_types: Vec<ThreatType>,
    pub severity_threshold: ThreatSeverity,
    pub actions: Vec<ResponseAction>,
    pub parallel_execution: bool,
    pub timeout: Duration,
    pub rollback_actions: Vec<ResponseAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    pub action_id: String,
    pub action_type: ResponseActionType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub conditions: Vec<ActionCondition>,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseActionType {
    BlockUser,
    BlockIpAddress,
    IsolateDevice,
    DisableAccount,
    ForcePasswordReset,
    RequireReauthentication,
    EscalateToSoc,
    NotifyAdministrator,
    CreateTicket,
    CollectForensics,
    QuarantineFile,
    KillProcess,
    BlockDomain,
    UpdateFirewallRules,
    RevokeToken,
    BackupData,
    Custom(String),
}

// Individual action executor implementations
pub struct BlockUserExecutor {
    user_management_client: Arc<dyn UserManagementClient>,
}

#[async_trait]
impl ResponseActionExecutor for BlockUserExecutor {
    async fn execute(&self, action: &ResponseAction, context: &ResponseExecutionContext) -> Result<ActionResult> {
        let user_id = action.parameters.get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::MissingActionParameter("user_id".to_string()))?;
        
        let block_duration = action.parameters.get("duration_minutes")
            .and_then(|v| v.as_u64())
            .unwrap_or(60); // Default 1 hour
        
        let result = self.user_management_client
            .block_user(user_id, Duration::from_minutes(block_duration))
            .await?;
        
        Ok(ActionResult {
            action_id: action.action_id.clone(),
            success: result.success,
            message: format!("User {} blocked for {} minutes", user_id, block_duration),
            artifacts: vec![ActionArtifact {
                artifact_type: "user_block_record".to_string(),
                data: serde_json::to_value(&result)?,
            }],
            executed_at: Utc::now(),
        })
    }
    
    fn action_type(&self) -> ResponseActionType {
        ResponseActionType::BlockUser
    }
}

pub struct IsolateDeviceExecutor {
    network_management_client: Arc<dyn NetworkManagementClient>,
}

#[async_trait]
impl ResponseActionExecutor for IsolateDeviceExecutor {
    async fn execute(&self, action: &ResponseAction, context: &ResponseExecutionContext) -> Result<ActionResult> {
        let device_id = action.parameters.get("device_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::MissingActionParameter("device_id".to_string()))?;
        
        let isolation_vlan = action.parameters.get("isolation_vlan")
            .and_then(|v| v.as_str())
            .unwrap_or("quarantine");
        
        let result = self.network_management_client
            .isolate_device(device_id, isolation_vlan)
            .await?;
        
        Ok(ActionResult {
            action_id: action.action_id.clone(),
            success: result.success,
            message: format!("Device {} isolated to VLAN {}", device_id, isolation_vlan),
            artifacts: vec![ActionArtifact {
                artifact_type: "device_isolation_record".to_string(),
                data: serde_json::to_value(&result)?,
            }],
            executed_at: Utc::now(),
        })
    }
    
    fn action_type(&self) -> ResponseActionType {
        ResponseActionType::IsolateDevice
    }
}
```

## ⚙️ **Configuration**

### **Threat Detection Configuration**
```toml
[threat_detection]
# General settings
enabled = true
real_time_processing = true
batch_processing_enabled = true
processing_threads = 8
max_events_per_second = 10000

[threat_detection.ml_detection]
# Machine learning detection settings
enabled = true
threshold = 0.6
enable_login_anomaly = true
enable_access_pattern = true
enable_behavioral = true
enable_data_exfiltration = true
model_update_interval_hours = 24

[threat_detection.ml_detection.models]
# Model file paths
login_anomaly_path = "./models/login_anomaly.onnx"
access_pattern_path = "./models/access_pattern.onnx"
behavioral_path = "./models/behavioral.onnx"
data_exfiltration_path = "./models/data_exfiltration.onnx"

[threat_detection.behavioral_analysis]
# Behavioral analysis settings
enabled = true
threshold = 0.7
learning_period_days = 30
baseline_update_interval_hours = 6
min_events_for_baseline = 100

[threat_detection.behavioral_analysis.feature_weights]
# Feature importance weights
access_patterns = 1.0
timing_patterns = 0.8
resource_usage = 0.9
interaction_patterns = 0.7
navigation_patterns = 0.6

[threat_detection.anomaly_detection]
# Statistical anomaly detection
enabled = true
min_severity = "medium"
window_size_minutes = 60
detection_algorithms = ["zscore", "isolation_forest", "one_class_svm"]

[threat_detection.threat_intelligence]
# Threat intelligence integration
enabled = true
providers = ["misp", "otx", "virustotal", "internal"]
refresh_interval_hours = 4
cache_size = 100000

[threat_detection.threat_intelligence.misp]
# MISP integration
url = "https://misp.internal.com"
api_key_env_var = "MISP_API_KEY"
verify_ssl = true
event_filters = ["malware", "apt", "phishing"]

[threat_detection.pattern_matching]
# Pattern matching rules
enabled = true
threshold = 0.8
rule_files = ["./rules/sigma", "./rules/yara", "./rules/custom"]
update_interval_hours = 1

[threat_detection.response]
# Automated response settings
auto_response_enabled = true
auto_response_threshold = "high"
max_concurrent_responses = 10
response_timeout_minutes = 30

[threat_detection.response.playbooks]
# Response playbook mappings
login_anomaly = "isolate_and_verify"
data_exfiltration = "block_and_investigate"
privilege_escalation = "disable_and_alert"
insider_threat = "monitor_and_collect"

[threat_detection.incident_management]
# Incident management settings
auto_create_incidents = true
incident_severity_mapping = { "critical" = "p1", "high" = "p2", "medium" = "p3" }
escalation_enabled = true
escalation_threshold_minutes = 15

[threat_detection.notifications]
# Notification settings
enabled = true
channels = ["email", "slack", "webhook"]
severity_filters = { "email" = "high", "slack" = "medium", "webhook" = "low" }

[threat_detection.notifications.email]
# Email notifications
smtp_server = "smtp.internal.com"
from_address = "beardog-threats@company.com"
to_addresses = ["soc@company.com", "security@company.com"]

[threat_detection.notifications.slack]
# Slack notifications
webhook_url_env_var = "THREAT_SLACK_WEBHOOK"
channel = "#security-alerts"
mention_users = ["@security-team"]

[threat_detection.performance]
# Performance settings
cache_size = 50000
cache_ttl_minutes = 30
metrics_collection_interval_seconds = 30
model_inference_timeout_ms = 1000
```

## 📊 **Threat Intelligence Integration**

### **Multi-Source Intelligence Provider**
```rust
pub struct ThreatIntelligenceProvider {
    config: ThreatIntelligenceConfig,
    providers: HashMap<String, Box<dyn IntelligenceSource>>,
    intelligence_cache: Arc<RwLock<IntelligenceCache>>,
    correlation_engine: Arc<CorrelationEngine>,
}

impl ThreatIntelligenceProvider {
    pub async fn lookup_indicators(&self, event: &SecurityEvent) -> Result<Vec<IntelligenceHit>> {
        let mut all_hits = Vec::new();
        
        // Extract indicators from event
        let indicators = self.extract_indicators(event)?;
        
        // Query all enabled providers
        for (provider_name, provider) in &self.providers {
            match provider.lookup_indicators(&indicators).await {
                Ok(hits) => {
                    all_hits.extend(hits.into_iter().map(|hit| IntelligenceHit {
                        provider: provider_name.clone(),
                        ..hit
                    }));
                }
                Err(e) => {
                    tracing::warn!("Threat intelligence lookup failed for {}: {}", provider_name, e);
                }
            }
        }
        
        // Correlate and deduplicate hits
        let correlated_hits = self.correlation_engine.correlate_hits(&all_hits).await?;
        
        // Cache results
        self.cache_intelligence_results(&indicators, &correlated_hits).await?;
        
        Ok(correlated_hits)
    }
    
    fn extract_indicators(&self, event: &SecurityEvent) -> Result<ThreatIndicators> {
        let mut indicators = ThreatIndicators::new();
        
        // Extract IP addresses
        if let Some(ip) = &event.source_ip {
            indicators.ip_addresses.push(ip.clone());
        }
        
        if let Some(ip) = &event.destination_ip {
            indicators.ip_addresses.push(ip.clone());
        }
        
        // Extract domains
        if let Some(url) = &event.url {
            if let Ok(parsed_url) = url::Url::parse(url) {
                if let Some(host) = parsed_url.host_str() {
                    indicators.domains.push(host.to_string());
                }
            }
        }
        
        // Extract file hashes
        if let Some(file_info) = &event.file_info {
            if let Some(hash) = &file_info.sha256 {
                indicators.file_hashes.push(hash.clone());
            }
            if let Some(hash) = &file_info.md5 {
                indicators.file_hashes.push(hash.clone());
            }
        }
        
        // Extract email addresses
        if let Some(email) = &event.email {
            indicators.email_addresses.push(email.clone());
        }
        
        Ok(indicators)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicators {
    pub ip_addresses: Vec<String>,
    pub domains: Vec<String>,
    pub file_hashes: Vec<String>,
    pub email_addresses: Vec<String>,
    pub urls: Vec<String>,
    pub user_agents: Vec<String>,
}

#[async_trait]
pub trait IntelligenceSource: Send + Sync {
    async fn lookup_indicators(&self, indicators: &ThreatIndicators) -> Result<Vec<IntelligenceHit>>;
    fn source_name(&self) -> &str;
    fn source_reliability(&self) -> f64;
    async fn health_check(&self) -> Result<bool>;
}

// MISP (Malware Information Sharing Platform) integration
pub struct MispIntelligenceSource {
    config: MispConfig,
    client: reqwest::Client,
}

#[async_trait]
impl IntelligenceSource for MispIntelligenceSource {
    async fn lookup_indicators(&self, indicators: &ThreatIndicators) -> Result<Vec<IntelligenceHit>> {
        let mut hits = Vec::new();
        
        // Query MISP for each indicator type
        for ip in &indicators.ip_addresses {
            if let Ok(results) = self.query_misp_attribute("ip-dst", ip).await {
                hits.extend(results);
            }
        }
        
        for domain in &indicators.domains {
            if let Ok(results) = self.query_misp_attribute("domain", domain).await {
                hits.extend(results);
            }
        }
        
        for hash in &indicators.file_hashes {
            if let Ok(results) = self.query_misp_attribute("sha256", hash).await {
                hits.extend(results);
            }
        }
        
        Ok(hits)
    }
    
    fn source_name(&self) -> &str {
        "MISP"
    }
    
    fn source_reliability(&self) -> f64 {
        0.9 // High reliability for MISP
    }
    
    async fn health_check(&self) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/servers/getVersion", self.config.url))
            .header("Authorization", &self.config.api_key)
            .send()
            .await?;
        
        Ok(response.status().is_success())
    }
}
```

## 🧪 **Testing Strategy**

### **Threat Detection Testing**
```rust
#[cfg(test)]
mod threat_detection_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_login_anomaly_detection() {
        let config = create_test_threat_config();
        let threat_engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        // Create suspicious login event
        let suspicious_event = SecurityEvent {
            id: "test-event-123".to_string(),
            event_type: SecurityEventType::Login,
            timestamp: Utc::now(),
            user_id: "testuser".to_string(),
            source_ip: Some("1.2.3.4".to_string()), // Foreign IP
            location: Some(Location {
                country: "RU".to_string(),
                city: "Moscow".to_string(),
                latitude: 55.7558,
                longitude: 37.6176,
            }),
            device_info: Some(DeviceInfo {
                device_id: "new-device-456".to_string(),
                os: "Android".to_string(),
                browser: "Chrome".to_string(),
                is_known_device: false,
            }),
            metadata: HashMap::new(),
        };
        
        let analysis_result = threat_engine.analyze_security_event(suspicious_event).await.unwrap();
        
        assert!(analysis_result.threat_level >= ThreatLevel::Medium);
        assert!(!analysis_result.threat_indicators.is_empty());
        assert!(analysis_result.risk_score > 0.5);
    }
    
    #[tokio::test]
    async fn test_automated_response() {
        let config = create_test_threat_config();
        let threat_engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        // Create high-severity threat event
        let high_threat_event = SecurityEvent {
            event_type: SecurityEventType::DataExfiltration,
            threat_level: ThreatLevel::Critical,
            // ... other fields
        };
        
        let analysis_result = threat_engine.analyze_security_event(high_threat_event).await.unwrap();
        
        // Verify automated response was triggered
        assert_eq!(analysis_result.threat_level, ThreatLevel::Critical);
        // Additional assertions for response execution...
    }
    
    #[tokio::test]
    async fn test_threat_intelligence_lookup() {
        let config = create_test_threat_config();
        let threat_engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        // Create event with known malicious indicators
        let malicious_event = SecurityEvent {
            source_ip: Some("192.0.2.1".to_string()), // Known malicious IP
            // ... other fields
        };
        
        let analysis_result = threat_engine.analyze_security_event(malicious_event).await.unwrap();
        
        let has_intel_hit = analysis_result.threat_indicators
            .iter()
            .any(|indicator| indicator.threat_type == ThreatType::ThreatIntelligence);
        
        assert!(has_intel_hit);
    }
}
```

---

**Next Steps**: Implement advanced ML models, threat hunting capabilities, and integration with external SIEM/SOAR platforms. 