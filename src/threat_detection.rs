//! Threat Detection and Response Engine
//! 
//! Provides real-time security monitoring and automated incident response.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use tokio::fs;
use tracing::{info, warn, error, debug};

use crate::error::{BearDogResult, BearDogError};

/// Threat Detection Engine
pub struct ThreatDetectionEngine {
    config: Arc<ThreatDetectionConfig>,
    rule_engine: Arc<RuleEngine>,
    file_monitor: Arc<FileIntegrityMonitor>,
    alert_system: Arc<AlertSystem>,
    threat_cache: Arc<RwLock<ThreatCache>>,
    detection_metrics: Arc<RwLock<DetectionMetrics>>,
}

/// Threat detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable threat detection
    pub enabled: bool,
    /// Detection rules path
    pub rules_path: Option<PathBuf>,
    /// File monitoring paths
    pub monitor_paths: Vec<PathBuf>,
    /// Alert thresholds
    pub alert_threshold: ThreatLevel,
    /// Cache size for threat indicators
    pub cache_size: usize,
    /// Monitoring interval in seconds
    pub monitoring_interval: u64,
}

/// Security event for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Unique event ID
    pub id: String,
    /// Event type
    pub event_type: EventType,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Source IP address (if applicable)
    pub source_ip: Option<String>,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Resource affected
    pub resource: Option<String>,
    /// Event metadata
    pub metadata: HashMap<String, String>,
}

/// Types of security events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    /// Login attempt
    LoginAttempt,
    /// File access
    FileAccess,
    /// File modification
    FileModification,
    /// Network connection
    NetworkConnection,
    /// API request
    ApiRequest,
    /// Configuration change
    ConfigChange,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Privileged operation
    PrivilegedOperation,
    /// Log analysis event
    LogAnalysis,
    /// Suspicious activity
    SuspiciousActivity,
    /// Failed login
    FailedLogin,
    /// Failed authentication
    FailedAuthentication,
    /// System command
    SystemCommand,
    /// Failed login (specific variant)
    LoginFailure,
    /// Encryption key operation
    EncryptionKey,
}

/// Threat level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// No threat detected
    None = 0,
    /// Low threat level
    Low = 1,
    /// Medium threat level
    Medium = 2,
    /// High threat level
    High = 3,
    /// Critical threat level
    Critical = 4,
}

/// Threat analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    /// Event ID
    pub event_id: String,
    /// Analysis ID
    pub analysis_id: String,
    /// Detected threat level
    pub threat_level: ThreatLevel,
    /// Threat indicators
    pub threat_indicators: Vec<ThreatIndicator>,
    /// Risk score (0.0-1.0)
    pub risk_score: f64,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
}

/// Individual threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator ID
    pub id: String,
    /// Indicator type
    pub indicator_type: IndicatorType,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Severity
    pub severity: ThreatLevel,
    /// Description
    pub description: String,
    /// Evidence
    pub evidence: Vec<String>,
    /// MITRE ATT&CK techniques
    pub mitre_techniques: Vec<String>,
}

/// Types of threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorType {
    /// Behavioral anomaly
    BehavioralAnomaly,
    /// Pattern match
    PatternMatch,
    /// Statistical anomaly
    StatisticalAnomaly,
    /// File integrity violation
    FileIntegrityViolation,
    /// Rule violation
    RuleViolation,
    /// Threat intelligence match
    ThreatIntelligence,
}

/// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    /// Alert ID
    pub id: String,
    /// Alert title
    pub title: String,
    /// Alert description
    pub description: String,
    /// Threat level
    pub threat_level: ThreatLevel,
    /// Related event ID
    pub event_id: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert metadata
    pub metadata: HashMap<String, String>,
}

/// Rule-based detection engine
pub struct RuleEngine {
    rules: Arc<RwLock<Vec<DetectionRule>>>,
}

/// Detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule pattern
    pub pattern: RulePattern,
    /// Severity level
    pub severity: ThreatLevel,
    /// MITRE ATT&CK techniques
    pub mitre_techniques: Vec<String>,
    /// Rule enabled
    pub enabled: bool,
}

/// Rule pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RulePattern {
    /// Regular expression pattern
    Regex { pattern: String },
    /// Multiple failed logins
    FailedLogins { threshold: u32, window_minutes: u32 },
    /// Suspicious file access
    SuspiciousFileAccess { patterns: Vec<String> },
    /// Network anomaly
    NetworkAnomaly { threshold: f64 },
}

/// File integrity monitoring
pub struct FileIntegrityMonitor {
    monitored_paths: Arc<RwLock<HashMap<PathBuf, FileIntegrityRecord>>>,
    config: Arc<ThreatDetectionConfig>,
}

/// File integrity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIntegrityRecord {
    /// File path
    pub path: PathBuf,
    /// File hash
    pub hash: String,
    /// Last modified time
    pub last_modified: DateTime<Utc>,
    /// File size
    pub size: u64,
    /// Last check time
    pub last_checked: DateTime<Utc>,
}

/// Alert system
pub struct AlertSystem {
    alert_handlers: Vec<Box<dyn AlertHandler>>,
}

/// Alert handler trait
#[async_trait::async_trait]
pub trait AlertHandler: Send + Sync {
    /// Handle a security alert
    async fn handle_alert(&self, alert: &SecurityAlert) -> BearDogResult<()>;
}

/// Console alert handler
pub struct ConsoleAlertHandler;

#[async_trait::async_trait]
impl AlertHandler for ConsoleAlertHandler {
    async fn handle_alert(&self, alert: &SecurityAlert) -> BearDogResult<()> {
        match alert.threat_level {
            ThreatLevel::Critical => error!("🚨 CRITICAL ALERT: {} - {}", alert.title, alert.description),
            ThreatLevel::High => warn!("⚠️  HIGH ALERT: {} - {}", alert.title, alert.description),
            ThreatLevel::Medium => warn!("⚠️  MEDIUM ALERT: {} - {}", alert.title, alert.description),
            ThreatLevel::Low => info!("ℹ️  LOW ALERT: {} - {}", alert.title, alert.description),
            ThreatLevel::None => debug!("ℹ️  INFO: {} - {}", alert.title, alert.description),
        }
        Ok(())
    }
}

/// Threat cache for storing recent analysis results
pub struct ThreatCache {
    cache: VecDeque<ThreatAnalysisResult>,
    max_size: usize,
}

/// Detection metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMetrics {
    /// Total events processed
    pub events_processed: u64,
    /// Threats detected
    pub threats_detected: u64,
    /// Alerts generated
    pub alerts_generated: u64,
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    /// False positive rate
    pub false_positive_rate: f64,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules_path: None,
            monitor_paths: vec![
                PathBuf::from("/etc"),
                PathBuf::from("/var/log"),
                PathBuf::from("/tmp"),
            ],
            alert_threshold: ThreatLevel::Medium,
            cache_size: 1000,
            monitoring_interval: 60,
        }
    }
}

impl ThreatDetectionEngine {
    /// Create a new threat detection engine
    pub async fn new(config: ThreatDetectionConfig) -> BearDogResult<Self> {
        let rule_engine = Arc::new(RuleEngine::new().await?);
        let file_monitor = Arc::new(FileIntegrityMonitor::new(&config).await?);
        let alert_system = Arc::new(AlertSystem::new().await?);
        
        Ok(Self {
            config: Arc::new(config),
            rule_engine,
            file_monitor,
            alert_system,
            threat_cache: Arc::new(RwLock::new(ThreatCache::new(1000))),
            detection_metrics: Arc::new(RwLock::new(DetectionMetrics::default())),
        })
    }
    
    /// Analyze a security event for threats
    pub async fn analyze_event(&self, event: SecurityEvent) -> BearDogResult<ThreatAnalysisResult> {
        let start_time = std::time::Instant::now();
        
        // Initialize threat indicators
        let mut threat_indicators = Vec::new();
        
        // Check rules
        let rule_results = self.rule_engine.check_rules(&event).await?;
        for rule_result in rule_results {
            if rule_result.matches {
                threat_indicators.push(ThreatIndicator {
                    id: format!("rule-{}", uuid::Uuid::new_v4()),
                    indicator_type: IndicatorType::RuleViolation,
                    confidence: rule_result.confidence,
                    severity: rule_result.severity,
                    description: format!("Rule violation: {}", rule_result.rule_name),
                    evidence: rule_result.evidence,
                    mitre_techniques: rule_result.mitre_techniques,
                });
            }
        }
        
        // Behavioral analysis
        if let Some(user_id) = &event.user_id {
            let behavioral_indicator = self.analyze_user_behavior(user_id, &event).await?;
            if let Some(indicator) = behavioral_indicator {
                threat_indicators.push(indicator);
            }
        }
        
        // System command analysis
        if event.event_type == EventType::SystemCommand {
            if let Some(command) = &event.resource {
                let mut command_risk = 0.0;
                let mut evidence = Vec::new();
                
                // Check for dangerous commands
                if command.contains("rm -rf") {
                    command_risk += 0.7;
                    evidence.push("Dangerous file deletion command detected".to_string());
                }
                if command.contains("sudo") {
                    command_risk += 0.3;
                    evidence.push("Privilege escalation command detected".to_string());
                }
                if command.contains("/etc/passwd") || command.contains("/etc/shadow") {
                    command_risk += 0.5;
                    evidence.push("System file access in command".to_string());
                }
                if command.contains("chmod 777") || command.contains("chmod +x") {
                    command_risk += 0.4;
                    evidence.push("File permission modification detected".to_string());
                }
                
                if command_risk > 0.3 {
                    threat_indicators.push(ThreatIndicator {
                        id: format!("syscmd-{}", uuid::Uuid::new_v4()),
                        indicator_type: IndicatorType::PatternMatch,
                        confidence: command_risk,
                        severity: if command_risk > 0.6 { ThreatLevel::High } else { ThreatLevel::Medium },
                        description: "Suspicious system command detected".to_string(),
                        evidence,
                        mitre_techniques: vec!["T1059".to_string()], // Command and Scripting Interpreter
                    });
                }
            }
        }
        
        // File integrity check (if applicable)
        if event.event_type == EventType::FileAccess || event.event_type == EventType::FileModification {
            if let Some(resource) = &event.resource {
                if let Ok(path) = std::path::Path::new(resource).canonicalize() {
                    if let Ok(integrity_result) = self.file_monitor.check_integrity(&path).await {
                        if !integrity_result.intact {
                            threat_indicators.push(ThreatIndicator {
                                id: format!("integrity-{}", uuid::Uuid::new_v4()),
                                indicator_type: IndicatorType::FileIntegrityViolation,
                                confidence: 0.9,
                                severity: ThreatLevel::High,
                                description: "File integrity violation detected".to_string(),
                                evidence: vec![format!("File: {}", resource)],
                                mitre_techniques: vec!["T1565.001".to_string()],
                            });
                        }
                    }
                }
            }
        }
        
        // Calculate risk score based on event type and context
        let base_risk_score = match event.event_type {
            EventType::FailedAuthentication => 0.6,
            EventType::SystemCommand => {
                if let Some(resource) = &event.resource {
                    if resource.contains("rm -rf") || resource.contains("sudo") {
                        0.8
                    } else {
                        0.4
                    }
                } else {
                    0.3
                }
            },
            EventType::FileAccess => {
                if let Some(resource) = &event.resource {
                    if resource.contains("/etc/") || resource.contains("passwd") {
                        0.5
                    } else {
                        0.2
                    }
                } else {
                    0.1
                }
            },
            EventType::PrivilegeEscalation => 0.9,
            EventType::SuspiciousActivity => 0.7,
            _ => 0.2,
        };
        
        // Add indicator-based risk
        let indicator_risk = self.calculate_risk_score(&threat_indicators);
        let final_risk_score = (base_risk_score + indicator_risk).min(1.0);
        
        // Determine threat level based on risk score
        let threat_level = if final_risk_score >= 0.8 {
            ThreatLevel::Critical
        } else if final_risk_score >= 0.6 {
            ThreatLevel::High
        } else if final_risk_score >= 0.4 {
            ThreatLevel::Medium
        } else if final_risk_score >= 0.1 {
            ThreatLevel::Low
        } else {
            ThreatLevel::None
        };
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&threat_indicators);
        
        let analysis_result = ThreatAnalysisResult {
            event_id: event.id.clone(),
            analysis_id: format!("analysis-{}", uuid::Uuid::new_v4()),
            threat_level,
            threat_indicators,
            risk_score: final_risk_score,
            recommended_actions: recommendations,
            analyzed_at: Utc::now(),
        };
        
        // Cache the result
        self.cache_analysis_result(&analysis_result).await?;
        
        // Update metrics
        let processing_time = start_time.elapsed().as_millis() as f64;
        self.update_metrics(processing_time).await?;
        
        // Generate alert if necessary
        if analysis_result.threat_level >= self.config.alert_threshold {
            self.generate_alert(&event, &analysis_result).await?;
        }
        
        Ok(analysis_result)
    }
    
    /// Start continuous monitoring
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        info!("Starting threat detection monitoring");
        
        // Start file integrity monitoring
        let file_monitor = self.file_monitor.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = file_monitor.scan_all().await {
                    error!("File integrity scan failed: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Get current detection metrics
    pub async fn get_metrics(&self) -> BearDogResult<DetectionMetrics> {
        let metrics = self.detection_metrics.read().await;
        Ok(metrics.clone())
    }
    
    /// Calculate risk score based on threat indicators
    fn calculate_risk_score(&self, indicators: &[ThreatIndicator]) -> f64 {
        if indicators.is_empty() {
            return 0.0;
        }
        
        let total_score: f64 = indicators.iter()
            .map(|i| i.confidence * (i.severity as u8 as f64 / 4.0))
            .sum();
        
        (total_score / indicators.len() as f64).min(1.0)
    }
    
    /// Generate recommended actions
    fn generate_recommendations(&self, indicators: &[ThreatIndicator]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for indicator in indicators {
            match indicator.indicator_type {
                IndicatorType::RuleViolation => {
                    recommendations.push("Review security rules and policies".to_string());
                    recommendations.push("Monitor user activity closely".to_string());
                }
                IndicatorType::BehavioralAnomaly => {
                    recommendations.push("Investigate user behavior patterns".to_string());
                    recommendations.push("Consider additional authentication requirements".to_string());
                }
                IndicatorType::FileIntegrityViolation => {
                    recommendations.push("Restore file from backup".to_string());
                    recommendations.push("Investigate file modification source".to_string());
                }
                IndicatorType::StatisticalAnomaly => {
                    recommendations.push("Analyze statistical patterns".to_string());
                }
                IndicatorType::PatternMatch => {
                    recommendations.push("Review pattern matching rules".to_string());
                }
                IndicatorType::ThreatIntelligence => {
                    recommendations.push("Cross-reference with threat intelligence feeds".to_string());
                }
            }
        }
        
        // Always provide at least one recommendation if there are no specific ones
        if recommendations.is_empty() && !indicators.is_empty() {
            recommendations.push("Monitor system activity for suspicious behavior".to_string());
            recommendations.push("Review security logs for additional context".to_string());
        }
        
        // Remove duplicates
        recommendations.sort();
        recommendations.dedup();
        
        recommendations
    }
    
    /// Cache analysis result
    async fn cache_analysis_result(&self, result: &ThreatAnalysisResult) -> BearDogResult<()> {
        let mut cache = self.threat_cache.write().await;
        cache.add(result.clone());
        Ok(())
    }
    
    /// Update detection metrics
    async fn update_metrics(&self, processing_time_ms: f64) -> BearDogResult<()> {
        let mut metrics = self.detection_metrics.write().await;
        metrics.events_processed += 1;
        metrics.avg_processing_time_ms = 
            (metrics.avg_processing_time_ms * (metrics.events_processed - 1) as f64 + processing_time_ms) 
            / metrics.events_processed as f64;
        metrics.last_updated = Utc::now();
        Ok(())
    }
    
    /// Generate security alert
    async fn generate_alert(&self, event: &SecurityEvent, analysis: &ThreatAnalysisResult) -> BearDogResult<()> {
        let alert = SecurityAlert {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Security Threat Detected - {:?}", analysis.threat_level),
            description: format!("Threat detected in event {} with {} indicators", 
                               event.id, analysis.threat_indicators.len()),
            threat_level: analysis.threat_level,
            event_id: event.id.clone(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };
        
        self.alert_system.send_alert(&alert).await?;
        
        // Update metrics
        let mut metrics = self.detection_metrics.write().await;
        metrics.alerts_generated += 1;
        if analysis.threat_level > ThreatLevel::None {
            metrics.threats_detected += 1;
        }
        
        Ok(())
    }

    async fn analyze_user_behavior(&self, user_id: &str, event: &SecurityEvent) -> BearDogResult<Option<ThreatIndicator>> {
        // Simple behavioral analysis - check for anomalous IP or resource access
        let mut anomaly_score = 0.0;
        let mut evidence = Vec::new();
        
        // Check for unusual IP
        if let Some(source_ip) = &event.source_ip {
            if !source_ip.starts_with("192.168.") && !source_ip.starts_with("10.0.") {
                anomaly_score += 0.3;
                evidence.push(format!("Unusual source IP: {}", source_ip));
            }
        }
        
        // Check for sensitive file access
        if let Some(resource) = &event.resource {
            if resource.contains("/etc/") || resource.contains("passwd") || resource.contains("shadow") {
                anomaly_score += 0.4;
                evidence.push(format!("Sensitive resource access: {}", resource));
            }
        }
        
        if anomaly_score > 0.2 {
            Ok(Some(ThreatIndicator {
                id: format!("behavior-{}", uuid::Uuid::new_v4()),
                indicator_type: IndicatorType::BehavioralAnomaly,
                confidence: anomaly_score,
                severity: if anomaly_score > 0.5 { ThreatLevel::High } else { ThreatLevel::Medium },
                description: "Behavioral anomaly detected".to_string(),
                evidence,
                mitre_techniques: vec!["T1078".to_string()],
            }))
        } else {
            Ok(None)
        }
    }

    /// Create a placeholder instance for initialization
    pub fn placeholder() -> Self {
        let config = ThreatDetectionConfig::default();
        Self {
            config: Arc::new(config.clone()),
            rule_engine: Arc::new(RuleEngine { rules: Arc::new(RwLock::new(Vec::new())) }),
            file_monitor: Arc::new(FileIntegrityMonitor {
                monitored_paths: Arc::new(RwLock::new(HashMap::new())),
                config: Arc::new(config),
            }),
            alert_system: Arc::new(AlertSystem { alert_handlers: Vec::new() }),
            threat_cache: Arc::new(RwLock::new(ThreatCache::new(1000))),
            detection_metrics: Arc::new(RwLock::new(DetectionMetrics::default())),
        }
    }
}

/// Rule check result
pub struct RuleCheckResult {
    pub matches: bool,
    pub rule_name: String,
    pub confidence: f64,
    pub severity: ThreatLevel,
    pub evidence: Vec<String>,
    pub mitre_techniques: Vec<String>,
}

impl RuleEngine {
    /// Create a new rule engine
    pub async fn new() -> BearDogResult<Self> {
        let rules = vec![
            DetectionRule {
                id: "failed_login_threshold".to_string(),
                name: "Multiple Failed Logins".to_string(),
                description: "Detects multiple failed login attempts".to_string(),
                pattern: RulePattern::FailedLogins { threshold: 5, window_minutes: 10 },
                severity: ThreatLevel::Medium,
                mitre_techniques: vec!["T1110".to_string()], // Brute Force
                enabled: true,
            },
            DetectionRule {
                id: "suspicious_file_access".to_string(),
                name: "Suspicious File Access".to_string(),
                description: "Detects access to sensitive files".to_string(),
                pattern: RulePattern::SuspiciousFileAccess { 
                    patterns: vec![
                        "/etc/passwd".to_string(),
                        "/etc/shadow".to_string(),
                        "*.key".to_string(),
                        "*.pem".to_string(),
                    ]
                },
                severity: ThreatLevel::High,
                mitre_techniques: vec!["T1005".to_string()], // Data from Local System
                enabled: true,
            },
        ];
        
        Ok(Self {
            rules: Arc::new(RwLock::new(rules)),
        })
    }
    
    /// Check event against all rules
    pub async fn check_rules(&self, event: &SecurityEvent) -> BearDogResult<Vec<RuleCheckResult>> {
        let rules = self.rules.read().await;
        let mut results = Vec::new();
        
        for rule in rules.iter() {
            if !rule.enabled {
                continue;
            }
            
            let matches = self.matches_pattern(event, &rule.pattern);
            
            if matches {
                results.push(RuleCheckResult {
                    matches: true,
                    rule_name: rule.name.clone(),
                    confidence: 0.8,
                    severity: rule.severity,
                    evidence: vec![format!("Rule '{}' matched event {}", rule.name, event.id)],
                    mitre_techniques: rule.mitre_techniques.clone(),
                });
            }
        }
        
        Ok(results)
    }

    fn matches_pattern(&self, event: &SecurityEvent, pattern: &RulePattern) -> bool {
        match pattern {
            RulePattern::FailedLogins { threshold, window_minutes: _ } => {
                if event.event_type == EventType::LoginFailure {
                    // Check if this user has exceeded the threshold within the time window
                    let user_id = event.metadata.get("user_id").and_then(|v| Some(v.as_str())).unwrap_or("");
                    // For now, check against event metadata for login failures
                    if let Some(failure_count) = event.metadata.get("failure_count") {
                        if let Ok(count) = failure_count.parse::<u32>() {
                            count >= *threshold
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            RulePattern::SuspiciousFileAccess { patterns } => {
                if event.event_type == EventType::FileAccess {
                    let file_path = event.metadata.get("file_path").and_then(|v| Some(v.as_str())).unwrap_or("");
                    patterns.iter().any(|pattern| file_path.contains(pattern))
                } else {
                    false
                }
            },
            RulePattern::NetworkAnomaly { threshold } => {
                if event.event_type == EventType::NetworkConnection {
                    // Check connection patterns for anomalies
                    let bytes_transferred = event.metadata.get("bytes_transferred")
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(0);
                    bytes_transferred > (*threshold as u64)
                } else {
                    false
                }
            },
            RulePattern::Regex { pattern } => {
                // Apply regex pattern to event metadata
                if let Ok(regex) = regex::Regex::new(pattern) {
                    event.metadata.values().any(|v| regex.is_match(v))
                } else {
                    false
                }
            },
        }
    }
}

/// File integrity result
pub struct FileIntegrityResult {
    pub intact: bool,
    pub current_hash: String,
    pub expected_hash: Option<String>,
}

impl FileIntegrityMonitor {
    /// Create a new file integrity monitor
    pub async fn new(config: &ThreatDetectionConfig) -> BearDogResult<Self> {
        let monitor = Self {
            monitored_paths: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(config.clone()),
        };
        
        // Initialize monitoring for configured paths
        monitor.initialize_monitoring().await?;
        
        Ok(monitor)
    }
    
    /// Initialize monitoring for configured paths
    async fn initialize_monitoring(&self) -> BearDogResult<()> {
        for path in &self.config.monitor_paths {
            if path.exists() {
                if let Ok(record) = self.create_integrity_record(path).await {
                    let mut monitored = self.monitored_paths.write().await;
                    monitored.insert(path.clone(), record);
                }
            }
        }
        Ok(())
    }
    
    /// Check integrity of a specific file
    pub async fn check_integrity(&self, path: &Path) -> BearDogResult<FileIntegrityResult> {
        let current_hash = self.calculate_file_hash(path).await?;
        
        let monitored = self.monitored_paths.read().await;
        if let Some(record) = monitored.get(path) {
            Ok(FileIntegrityResult {
                intact: current_hash == record.hash,
                current_hash,
                expected_hash: Some(record.hash.clone()),
            })
        } else {
            Ok(FileIntegrityResult {
                intact: true, // No baseline to compare against
                current_hash,
                expected_hash: None,
            })
        }
    }
    
    /// Scan all monitored files
    pub async fn scan_all(&self) -> BearDogResult<Vec<PathBuf>> {
        let mut changed_files = Vec::new();
        let paths: Vec<PathBuf> = {
            let monitored = self.monitored_paths.read().await;
            monitored.keys().cloned().collect()
        };
        
        for path in paths {
            match self.check_integrity(&path).await {
                Ok(result) => {
                    if !result.intact {
                        changed_files.push(path.clone());
                        warn!("File integrity violation detected: {}", path.display());
                    }
                }
                Err(e) => {
                    error!("Failed to check integrity for {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(changed_files)
    }
    
    /// Create integrity record for a file
    async fn create_integrity_record(&self, path: &Path) -> BearDogResult<FileIntegrityRecord> {
        let metadata = fs::metadata(path).await
            .map_err(|e| BearDogError::IoError(e.to_string()))?;
        
        let hash = self.calculate_file_hash(path).await?;
        
        Ok(FileIntegrityRecord {
            path: path.to_path_buf(),
            hash,
            last_modified: DateTime::from_timestamp(
                metadata.modified()
                    .map_err(|e| BearDogError::IoError(e.to_string()))?
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| BearDogError::IoError(e.to_string()))?
                    .as_secs() as i64, 0
            ).unwrap_or_else(Utc::now),
            size: metadata.len(),
            last_checked: Utc::now(),
        })
    }
    
    /// Calculate SHA-256 hash of a file
    async fn calculate_file_hash(&self, path: &Path) -> BearDogResult<String> {
        let content = fs::read(path).await
            .map_err(|e| BearDogError::IoError(e.to_string()))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }
}

impl AlertSystem {
    /// Create a new alert system
    pub async fn new() -> BearDogResult<Self> {
        let alert_handlers: Vec<Box<dyn AlertHandler>> = vec![
            Box::new(ConsoleAlertHandler),
        ];
        
        Ok(Self {
            alert_handlers,
        })
    }
    
    /// Send alert to all handlers
    pub async fn send_alert(&self, alert: &SecurityAlert) -> BearDogResult<()> {
        for handler in &self.alert_handlers {
            if let Err(e) = handler.handle_alert(alert).await {
                error!("Alert handler failed: {}", e);
            }
        }
        Ok(())
    }
}

impl ThreatCache {
    /// Create a new threat cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: VecDeque::new(),
            max_size,
        }
    }
    
    /// Add analysis result to cache
    pub fn add(&mut self, result: ThreatAnalysisResult) {
        if self.cache.len() >= self.max_size {
            self.cache.pop_front();
        }
        self.cache.push_back(result);
    }
    
    /// Get recent analysis results
    pub fn get_recent(&self, limit: usize) -> Vec<&ThreatAnalysisResult> {
        self.cache.iter().rev().take(limit).collect()
    }
}

impl Default for DetectionMetrics {
    fn default() -> Self {
        Self {
            events_processed: 0,
            threats_detected: 0,
            alerts_generated: 0,
            avg_processing_time_ms: 0.0,
            false_positive_rate: 0.0,
            last_updated: Utc::now(),
        }
    }
}

/// Placeholder for actual threat detection implementation
#[allow(dead_code)]
pub struct ThreatDetectionPlaceholder;

impl ThreatDetectionPlaceholder {
    /// Create a new placeholder instance
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    fn create_test_config() -> ThreatDetectionConfig {
        ThreatDetectionConfig {
            enabled: true,
            rules_path: Some(std::path::PathBuf::from("test_rules.json")),
            monitor_paths: vec![std::path::PathBuf::from("/test")],
            alert_threshold: ThreatLevel::Medium,
            cache_size: 1000,
            monitoring_interval: 300, // 5 minutes in seconds
        }
    }
    
    fn create_test_security_event() -> SecurityEvent {
        SecurityEvent {
            id: "test-event-001".to_string(),
            event_type: EventType::FileAccess,
            timestamp: Utc::now(),
            source_ip: Some("192.168.1.100".to_string()),
            user_id: Some("test-user".to_string()),
            resource: Some("/etc/passwd".to_string()),
            metadata: HashMap::new(),
        }
    }
    
    #[tokio::test]
    async fn test_threat_detection_engine_creation() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await;
        assert!(engine.is_ok());
    }
    
    #[tokio::test]
    async fn test_analyze_file_access_event() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let event = create_test_security_event();
        let result = engine.analyze_event(event).await.unwrap();
        
        assert!(result.risk_score >= 0.0);
        assert!(result.risk_score <= 1.0);
        assert!(!result.threat_indicators.is_empty());
    }
    
    #[tokio::test]
    async fn test_analyze_failed_login_event() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let mut event = create_test_security_event();
        event.event_type = EventType::FailedAuthentication;
        event.resource = Some("login_portal".to_string());
        
        let result = engine.analyze_event(event).await.unwrap();
        
        assert!(result.risk_score > 0.0);
        assert!(result.threat_level >= ThreatLevel::Low);
    }
    
    #[tokio::test]
    async fn test_analyze_system_command_event() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let mut event = create_test_security_event();
        event.event_type = EventType::SystemCommand;
        event.resource = Some("rm -rf /important/data".to_string()); // More suspicious command
        
        let result = engine.analyze_event(event).await.unwrap();
        
        assert!(result.threat_level >= ThreatLevel::Low);
        assert!(result.risk_score > 0.0);
        assert!(!result.recommended_actions.is_empty());
    }
    
    #[tokio::test]
    async fn test_threat_level_escalation() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        // Test different threat levels
        let low_threat_event = SecurityEvent {
            id: "low-threat".to_string(),
            event_type: EventType::FileAccess,
            timestamp: Utc::now(),
            source_ip: Some("192.168.1.10".to_string()),
            user_id: Some("normal-user".to_string()),
            resource: Some("/home/user/document.txt".to_string()),
            metadata: HashMap::new(),
        };
        
        let high_threat_event = SecurityEvent {
            id: "high-threat".to_string(),
            event_type: EventType::SystemCommand,
            timestamp: Utc::now(),
            source_ip: Some("192.168.1.100".to_string()),
            user_id: Some("suspicious-user".to_string()),
            resource: Some("rm -rf /".to_string()),
            metadata: HashMap::new(),
        };
        
        let low_result = engine.analyze_event(low_threat_event).await.unwrap();
        let high_result = engine.analyze_event(high_threat_event).await.unwrap();
        
        assert!(low_result.risk_score < high_result.risk_score);
        assert!(low_result.threat_level < high_result.threat_level);
    }
    
    #[tokio::test]
    async fn test_behavioral_analysis() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let user_id = "test-user-behavior";
        
        // Simulate normal behavior pattern
        for i in 0..10 {
            let event = SecurityEvent {
                id: format!("behavior-event-{}", i),
                event_type: EventType::FileAccess,
                timestamp: Utc::now(),
                source_ip: Some("192.168.1.50".to_string()),
                user_id: Some(user_id.to_string()),
                resource: Some(format!("/home/{}/file{}.txt", user_id, i)),
                metadata: HashMap::new(),
            };
            
            let _result = engine.analyze_event(event).await.unwrap();
        }
        
        // Now test anomalous behavior
        let anomalous_event = SecurityEvent {
            id: "anomalous-behavior".to_string(),
            event_type: EventType::FileAccess,
            timestamp: Utc::now(),
            source_ip: Some("10.0.0.1".to_string()), // Different IP
            user_id: Some(user_id.to_string()),
            resource: Some("/etc/passwd".to_string()), // Sensitive file
            metadata: HashMap::new(),
        };
        
        let result = engine.analyze_event(anomalous_event).await.unwrap();
        
        // Should detect higher risk due to behavioral change
        assert!(result.risk_score > 0.3);
    }
    
    #[tokio::test]
    async fn test_get_metrics() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        // Process some events
        for i in 0..5 {
            let mut event = create_test_security_event();
            event.id = format!("metrics-test-{}", i);
            let _result = engine.analyze_event(event).await.unwrap();
        }
        
        let metrics = engine.get_metrics().await.unwrap();
        assert_eq!(metrics.events_processed, 5);
        assert!(metrics.threats_detected >= 0);
    }
    
    #[tokio::test]
    async fn test_different_event_types() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let event_types = vec![
            EventType::FileAccess,
            EventType::FailedAuthentication,
            EventType::SystemCommand,
        ];
        
        for event_type in event_types {
            let mut event = create_test_security_event();
            event.event_type = event_type;
            let result = engine.analyze_event(event).await.unwrap();
            
            assert!(result.risk_score >= 0.0);
            assert!(result.risk_score <= 1.0);
        }
    }
    
    #[tokio::test]
    async fn test_threat_detection_with_metadata() {
        let config = create_test_config();
        let engine = ThreatDetectionEngine::new(config).await.unwrap();
        
        let mut event = create_test_security_event();
        event.metadata.insert("severity".to_string(), "high".to_string());
        event.metadata.insert("category".to_string(), "security".to_string());
        
        let result = engine.analyze_event(event).await.unwrap();
        
        assert!(result.risk_score >= 0.0);
        assert!(!result.threat_indicators.is_empty());
    }
    
    #[tokio::test]
    async fn test_concurrent_threat_analysis() {
        let config = create_test_config();
        let engine = std::sync::Arc::new(ThreatDetectionEngine::new(config).await.unwrap());
        
        let mut handles = vec![];
        
        for i in 0..10 {
            let engine_clone = engine.clone();
            let handle = tokio::spawn(async move {
                let mut event = SecurityEvent {
                    id: format!("concurrent-test-{}", i),
                    event_type: EventType::FileAccess,
                    timestamp: Utc::now(),
                    source_ip: Some(format!("192.168.1.{}", 100 + i)),
                    user_id: Some(format!("user-{}", i)),
                    resource: Some(format!("/test/file{}.txt", i)),
                    metadata: HashMap::new(),
                };
                
                let result = engine_clone.analyze_event(event).await.unwrap();
                assert!(result.risk_score >= 0.0);
                assert!(result.risk_score <= 1.0);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
    }
    
    #[test]
    fn test_threat_level_ordering() {
        assert!(ThreatLevel::Low < ThreatLevel::Medium);
        assert!(ThreatLevel::Medium < ThreatLevel::High);
        assert!(ThreatLevel::High < ThreatLevel::Critical);
    }
    
    #[test]
    fn test_security_event_creation() {
        let event = create_test_security_event();
        assert!(!event.id.is_empty());
        assert!(event.source_ip.is_some());
        assert!(event.user_id.is_some());
        assert!(event.resource.is_some());
    }
} 