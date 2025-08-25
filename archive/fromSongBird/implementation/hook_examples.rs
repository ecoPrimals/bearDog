//! BearDog Hook System Integration Examples
//!
//! This file provides comprehensive examples of how to implement security
//! hooks for BearDog Security Manager integration with Songbird Orchestrator.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

// Songbird imports (these would be actual imports in real implementation)
use songbird_orchestrator::traits::hooks::{
    EventHook, HookResult, HookConfig, OrchestratorEvent, HookContext,
    EventFilter, ExecutionConfig, RetryConfig
};
use songbird_orchestrator::traits::service::{ServiceInfo, ServiceRequest, ServiceResponse};
use songbird_orchestrator::errors::Result;

/// BearDog Security Monitoring Hook
/// 
/// This hook provides real-time security monitoring, threat detection,
/// and incident response for all Songbird orchestrator events.
pub struct BearDogSecurityMonitoringHook {
    name: String,
    config: BearDogHookConfig,
    beardog_client: Arc<BearDogMonitoringClient>,
    threat_detector: Arc<ThreatDetector>,
    incident_responder: Arc<IncidentResponder>,
    statistics: Arc<Mutex<HookStatistics>>,
}

impl BearDogSecurityMonitoringHook {
    pub fn new(
        config: BearDogHookConfig,
        beardog_client: Arc<BearDogMonitoringClient>,
    ) -> Self {
        Self {
            name: "beardog-security-monitoring".to_string(),
            config,
            beardog_client: beardog_client.clone(),
            threat_detector: Arc::new(ThreatDetector::new(beardog_client.clone())),
            incident_responder: Arc::new(IncidentResponder::new(beardog_client)),
            statistics: Arc::new(Mutex::new(HookStatistics::default())),
        }
    }

    /// Extract security context from a request
    async fn extract_security_context(&self, request: &ServiceRequest) -> SecurityContext {
        SecurityContext {
            request_id: request.id.clone(),
            client_ip: request.headers.get("X-Real-IP")
                .or_else(|| request.headers.get("X-Forwarded-For"))
                .cloned(),
            user_agent: request.headers.get("User-Agent").cloned(),
            authorization: request.headers.get("Authorization").cloned(),
            content_type: request.headers.get("Content-Type").cloned(),
            content_length: request.body.len(),
            timestamp: Utc::now(),
        }
    }

    /// Check if an error is security-related
    fn is_security_related_error(&self, error_type: &str, error_message: &str) -> bool {
        let security_keywords = [
            "authentication", "authorization", "forbidden", "unauthorized",
            "token", "credential", "permission", "access denied",
            "invalid signature", "expired", "malformed",
        ];

        let error_lower = format!("{} {}", error_type, error_message).to_lowercase();
        security_keywords.iter().any(|keyword| error_lower.contains(keyword))
    }

    /// Classify security incident type
    fn classify_security_incident(&self, error_type: &str) -> SecurityIncidentType {
        match error_type.to_lowercase().as_str() {
            t if t.contains("auth") => SecurityIncidentType::AuthenticationFailure,
            t if t.contains("permission") || t.contains("forbidden") => SecurityIncidentType::UnauthorizedAccess,
            t if t.contains("token") => SecurityIncidentType::TokenViolation,
            t if t.contains("rate") || t.contains("throttle") => SecurityIncidentType::RateLimitViolation,
            t if t.contains("malicious") || t.contains("attack") => SecurityIncidentType::SuspiciousActivity,
            _ => SecurityIncidentType::GeneralSecurityError,
        }
    }

    /// Update hook statistics
    async fn update_statistics(&self, success: bool, execution_time_ms: u64) {
        let mut stats = self.statistics.lock().await;
        stats.total_executions += 1;
        stats.total_execution_time_ms += execution_time_ms;
        
        if success {
            stats.successful_executions += 1;
        } else {
            stats.failed_executions += 1;
        }
        
        stats.average_execution_time_ms = 
            stats.total_execution_time_ms as f64 / stats.total_executions as f64;
        stats.last_execution = Some(Utc::now());
    }
}

#[async_trait]
impl EventHook for BearDogSecurityMonitoringHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn priority(&self) -> u32 {
        100 // High priority for security
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    async fn initialize(&mut self, context: &HookContext) -> Result<()> {
        // Initialize connection to BearDog
        self.beardog_client.connect().await?;
        
        // Register this Songbird instance with BearDog
        self.beardog_client.register_songbird_instance(
            &context.orchestrator_id,
            &self.config
        ).await?;
        
        // Initialize threat detection models
        self.threat_detector.initialize().await?;
        
        tracing::info!("BearDog Security Monitoring Hook initialized successfully");
        Ok(())
    }

    async fn handle_event(&self, event: &OrchestratorEvent) -> Result<HookResult> {
        let start_time = Instant::now();
        let mut result = HookResult {
            success: true,
            continue_chain: true,
            allow_operation: true,
            modifications: None,
            log_messages: Vec::new(),
            execution_time_ms: 0,
            error: None,
        };

        match event {
            // ========== REQUEST SECURITY MONITORING ==========
            OrchestratorEvent::RequestReceived { service_id, request, timestamp } => {
                result.log_messages.push(format!("Processing request {} for service {}", request.id, service_id));

                // Extract security context from request
                let security_context = self.extract_security_context(request).await;

                // Send request to BearDog for real-time analysis
                match self.beardog_client.monitor_request(service_id, &security_context).await {
                    Ok(monitoring_result) => {
                        // Analyze for threats using ML models
                        match self.threat_detector.analyze_request(&security_context).await {
                            Ok(Some(threat)) => {
                                result.log_messages.push(format!("Threat detected: {} (confidence: {})", 
                                    threat.threat_type, threat.confidence));

                                // Handle high-confidence threats
                                if threat.confidence >= self.config.threat_confidence_threshold {
                                    // Trigger incident response
                                    if let Err(e) = self.incident_responder.handle_threat(
                                        service_id, 
                                        &security_context, 
                                        &threat
                                    ).await {
                                        result.log_messages.push(format!("Incident response failed: {}", e));
                                    }

                                    // Block high-threat requests
                                    match threat.severity {
                                        ThreatSeverity::Critical | ThreatSeverity::High => {
                                            result.allow_operation = false;
                                            result.continue_chain = false;
                                            result.log_messages.push("Request blocked due to high threat level".to_string());
                                        },
                                        ThreatSeverity::Medium => {
                                            // Allow but add extra monitoring
                                            result.log_messages.push("Request allowed with extra monitoring".to_string());
                                        },
                                        ThreatSeverity::Low => {
                                            // Allow with logging
                                            result.log_messages.push("Low threat detected, allowing with audit".to_string());
                                        },
                                    }
                                }
                            },
                            Ok(None) => {
                                // No threat detected
                                result.log_messages.push("No threats detected".to_string());
                            },
                            Err(e) => {
                                result.log_messages.push(format!("Threat analysis failed: {}", e));
                                // Continue processing on analysis failure (fail open for availability)
                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format!("BearDog monitoring failed: {}", e));
                        // Continue processing on monitoring failure (fail open)
                    }
                }
            },

            // ========== SERVICE LIFECYCLE SECURITY ==========
            OrchestratorEvent::ServiceRegistering { service_info, timestamp } => {
                result.log_messages.push(format!("Validating security for service: {}", service_info.name));

                // Perform security compliance check
                match self.beardog_client.assess_service_security(service_info).await {
                    Ok(assessment) => {
                        if !assessment.is_compliant() {
                            result.allow_operation = false;
                            result.log_messages.push(format!(
                                "Service security assessment failed. Violations: {:?}", 
                                assessment.violations
                            ));

                            // Create security incident for non-compliant service
                            let incident = SecurityIncident {
                                incident_type: SecurityIncidentType::ServiceNonCompliance,
                                service_id: Some(service_info.id.clone()),
                                description: format!("Service {} failed security compliance", service_info.name),
                                severity: IncidentSeverity::Medium,
                                details: assessment.to_details(),
                                timestamp: *timestamp,
                            };

                            if let Err(e) = self.incident_responder.create_incident(incident).await {
                                result.log_messages.push(format!("Failed to create incident: {}", e));
                            }
                        } else {
                            result.log_messages.push("Service security assessment passed".to_string());
                            
                            // Log successful compliance check
                            if let Err(e) = self.beardog_client.log_compliance_success(
                                &service_info.id, 
                                &assessment
                            ).await {
                                result.log_messages.push(format!("Failed to log compliance: {}", e));
                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format!("Security assessment failed: {}", e));
                        
                        // In production, you might want to fail closed (block) on assessment failure
                        if self.config.fail_closed_on_assessment_error {
                            result.allow_operation = false;
                            result.log_messages.push("Blocking service due to assessment failure".to_string());
                        }
                    }
                }
            },

            // ========== ERROR AND INCIDENT MONITORING ==========
            OrchestratorEvent::ErrorOccurred { error_type, error_message, service_id, context, timestamp } => {
                result.log_messages.push(format!("Processing error: {} - {}", error_type, error_message));

                // Check if this is a security-related error
                if self.is_security_related_error(error_type, error_message) {
                    let incident_type = self.classify_security_incident(error_type);
                    
                    let incident = SecurityIncident {
                        incident_type,
                        service_id: service_id.clone(),
                        description: error_message.clone(),
                        severity: match error_type.to_lowercase().as_str() {
                            t if t.contains("critical") || t.contains("breach") => IncidentSeverity::Critical,
                            t if t.contains("unauthorized") || t.contains("forbidden") => IncidentSeverity::High,
                            _ => IncidentSeverity::Medium,
                        },
                        details: context.clone(),
                        timestamp: *timestamp,
                    };

                    match self.incident_responder.create_incident(incident).await {
                        Ok(incident_id) => {
                            result.log_messages.push(format!("Security incident created: {}", incident_id));
                        },
                        Err(e) => {
                            result.log_messages.push(format!("Failed to create security incident: {}", e));
                        }
                    }
                }
            },

            // ========== CONFIGURATION SECURITY ==========
            OrchestratorEvent::ConfigurationChanged { config_section, old_config, new_config, timestamp } => {
                result.log_messages.push(format!("Validating config change for section: {}", config_section));

                // Security-sensitive configuration sections
                let sensitive_sections = ["security", "auth", "tls", "oauth", "encryption"];
                
                if sensitive_sections.iter().any(|&section| config_section.contains(section)) {
                    match self.beardog_client.validate_security_config(
                        config_section, 
                        old_config, 
                        new_config
                    ).await {
                        Ok(validation_result) => {
                            if !validation_result.is_valid() {
                                result.allow_operation = false;
                                result.log_messages.push(format!(
                                    "Security configuration validation failed: {:?}", 
                                    validation_result.violations
                                ));
                            }
                        },
                        Err(e) => {
                            result.log_messages.push(format!("Config validation failed: {}", e));
                            
                            // Fail closed on validation errors for security configs
                            result.allow_operation = false;
                            result.log_messages.push("Blocking config change due to validation failure".to_string());
                        }
                    }
                }
            },

            // ========== HEALTH AND METRICS MONITORING ==========
            OrchestratorEvent::HealthCheckCompleted { service_id, healthy, details, timestamp } => {
                // Monitor for security-related health issues
                if !healthy {
                    // Check if health failure might be security-related
                    if let Some(error_details) = details.get("error") {
                        if let Some(error_str) = error_details.as_str() {
                            if self.is_security_related_error("health_check", error_str) {
                                let incident = SecurityIncident {
                                    incident_type: SecurityIncidentType::ServiceSecurityFailure,
                                    service_id: Some(service_id.clone()),
                                    description: format!("Security-related health check failure: {}", error_str),
                                    severity: IncidentSeverity::Medium,
                                    details: details.clone(),
                                    timestamp: *timestamp,
                                };

                                if let Err(e) = self.incident_responder.create_incident(incident).await {
                                    result.log_messages.push(format!("Failed to create health incident: {}", e));
                                }
                            }
                        }
                    }
                }
            },

            // ========== DISCOVERY SECURITY ==========
            OrchestratorEvent::ServiceDiscovered { service_info, discovery_source, timestamp } => {
                result.log_messages.push(format!("Validating discovered service: {}", service_info.name));

                // Validate that discovered services meet security requirements
                match self.beardog_client.validate_service_discovery(service_info, discovery_source).await {
                    Ok(validation_result) => {
                        if !validation_result.trusted {
                            result.log_messages.push(format!(
                                "Discovered service not trusted: {}", 
                                validation_result.reason
                            ));

                            // Create incident for untrusted service discovery
                            let incident = SecurityIncident {
                                incident_type: SecurityIncidentType::UntrustedServiceDiscovery,
                                service_id: Some(service_info.id.clone()),
                                description: format!("Untrusted service discovered: {}", validation_result.reason),
                                severity: IncidentSeverity::Medium,
                                details: {
                                    let mut details = HashMap::new();
                                    details.insert("discovery_source".to_string(), serde_json::to_value(discovery_source)?);
                                    details.insert("service_info".to_string(), serde_json::to_value(service_info)?);
                                    details
                                },
                                timestamp: *timestamp,
                            };

                            if let Err(e) = self.incident_responder.create_incident(incident).await {
                                result.log_messages.push(format!("Failed to create discovery incident: {}", e));
                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format!("Service discovery validation failed: {}", e));
                    }
                }
            },

            _ => {
                // Handle other events with basic logging
                result.log_messages.push(format!("Processed event: {:?}", event));
            }
        }

        // Update execution metrics
        let execution_time = start_time.elapsed().as_millis() as u64;
        result.execution_time_ms = execution_time;
        
        self.update_statistics(result.success, execution_time).await;

        Ok(result)
    }

    async fn cleanup(&self) -> Result<()> {
        // Disconnect from BearDog
        self.beardog_client.disconnect().await?;
        
        // Log final statistics
        let stats = self.statistics.lock().await;
        tracing::info!("BearDog Security Hook final stats: {:?}", *stats);
        
        Ok(())
    }

    fn get_config(&self) -> HookConfig {
        HookConfig {
            settings: self.config.to_settings(),
            event_filter: EventFilter {
                event_types: vec![
                    "RequestReceived".to_string(),
                    "ServiceRegistering".to_string(),
                    "ErrorOccurred".to_string(),
                    "ConfigurationChanged".to_string(),
                    "HealthCheckCompleted".to_string(),
                    "ServiceDiscovered".to_string(),
                ],
                service_ids: vec![], // Monitor all services
                conditions: vec![],
            },
            execution: ExecutionConfig {
                async_execution: true,
                timeout_ms: 5000, // 5 second timeout
                log_execution: true,
                measure_performance: true,
            },
            retry: RetryConfig {
                enabled: true,
                max_attempts: 3,
                retry_delay_ms: 1000,
                backoff_multiplier: 2.0,
            },
        }
    }
}

// ============== SUPPORT STRUCTURES ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogHookConfig {
    pub enabled: bool,
    pub beardog_endpoint: String,
    pub api_key: String,
    pub threat_confidence_threshold: f64,
    pub fail_closed_on_assessment_error: bool,
    pub enable_real_time_monitoring: bool,
    pub enable_incident_response: bool,
}

impl Default for BearDogHookConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            beardog_endpoint: "https://beardog.security.internal".to_string(),
            api_key: std::env::var("BEARDOG_API_KEY").unwrap_or_default(),
            threat_confidence_threshold: 0.7,
            fail_closed_on_assessment_error: false,
            enable_real_time_monitoring: true,
            enable_incident_response: true,
        }
    }
}

impl BearDogHookConfig {
    pub fn to_settings(&self) -> HashMap<String, serde_json::Value> {
        let mut settings = HashMap::new();
        settings.insert("enabled".to_string(), serde_json::to_value(self.enabled).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        settings.insert("endpoint".to_string(), serde_json::to_value(&self.beardog_endpoint).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        settings.insert("threat_threshold".to_string(), serde_json::to_value(self.threat_confidence_threshold).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        settings
    }
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub request_id: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub authorization: Option<String>,
    pub content_type: Option<String>,
    pub content_length: usize,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ThreatInfo {
    pub threat_type: String,
    pub confidence: f64,
    pub severity: ThreatSeverity,
    pub description: String,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SecurityIncident {
    pub incident_type: SecurityIncidentType,
    pub service_id: Option<String>,
    pub description: String,
    pub severity: IncidentSeverity,
    pub details: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum SecurityIncidentType {
    AuthenticationFailure,
    UnauthorizedAccess,
    TokenViolation,
    RateLimitViolation,
    SuspiciousActivity,
    ServiceNonCompliance,
    ServiceSecurityFailure,
    UntrustedServiceDiscovery,
    GeneralSecurityError,
}

#[derive(Debug, Clone)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Default)]
pub struct HookStatistics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_execution_time_ms: u64,
    pub average_execution_time_ms: f64,
    pub last_execution: Option<DateTime<Utc>>,
}

// ============== MOCK CLIENTS FOR REFERENCE ==============

pub struct BearDogMonitoringClient {
    endpoint: String,
    api_key: String,
}

impl BearDogMonitoringClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self { endpoint, api_key }
    }

    pub async fn connect(&self) -> Result<()> {
        // Implementation: Connect to BearDog monitoring service
        tracing::info!("Connected to BearDog monitoring at {}", self.endpoint);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        // Implementation: Disconnect from BearDog
        tracing::info!("Disconnected from BearDog monitoring");
        Ok(())
    }

    pub async fn register_songbird_instance(
        &self, 
        orchestrator_id: &str, 
        config: &BearDogHookConfig
    ) -> Result<()> {
        // Implementation: Register this Songbird instance with BearDog
        tracing::info!("Registered Songbird instance {} with BearDog", orchestrator_id);
        Ok(())
    }

    pub async fn monitor_request(
        &self, 
        service_id: &str, 
        context: &SecurityContext
    ) -> Result<MonitoringResult> {
        // Implementation: Send request to BearDog for monitoring
        Ok(MonitoringResult { 
            monitored: true, 
            risk_score: 0.1 
        })
    }

    pub async fn assess_service_security(&self, service_info: &ServiceInfo) -> Result<SecurityAssessment> {
        // Implementation: Assess service security compliance
        Ok(SecurityAssessment {
            compliant: true,
            violations: vec![],
            risk_score: 0.2,
        })
    }

    pub async fn validate_security_config(
        &self,
        section: &str,
        old_config: &serde_json::Value,
        new_config: &serde_json::Value,
    ) -> Result<ConfigValidationResult> {
        // Implementation: Validate security configuration changes
        Ok(ConfigValidationResult {
            valid: true,
            violations: vec![],
        })
    }

    pub async fn validate_service_discovery(
        &self,
        service_info: &ServiceInfo,
        discovery_source: &str,
    ) -> Result<DiscoveryValidationResult> {
        // Implementation: Validate discovered services
        Ok(DiscoveryValidationResult {
            trusted: true,
            reason: "Service from trusted source".to_string(),
        })
    }

    pub async fn log_compliance_success(
        &self,
        service_id: &str,
        assessment: &SecurityAssessment,
    ) -> Result<()> {
        // Implementation: Log successful compliance check
        tracing::info!("Logged compliance success for service {}", service_id);
        Ok(())
    }
}

pub struct ThreatDetector {
    client: Arc<BearDogMonitoringClient>,
}

impl ThreatDetector {
    pub fn new(client: Arc<BearDogMonitoringClient>) -> Self {
        Self { client }
    }

    pub async fn initialize(&self) -> Result<()> {
        // Implementation: Initialize threat detection models
        tracing::info!("Threat detector initialized");
        Ok(())
    }

    pub async fn analyze_request(&self, context: &SecurityContext) -> Result<Option<ThreatInfo>> {
        // Implementation: Analyze request for threats using ML models
        // This is a mock implementation - real implementation would use BearDog's ML models
        Ok(None) // No threat detected
    }
}

pub struct IncidentResponder {
    client: Arc<BearDogMonitoringClient>,
}

impl IncidentResponder {
    pub fn new(client: Arc<BearDogMonitoringClient>) -> Self {
        Self { client }
    }

    pub async fn handle_threat(
        &self,
        service_id: &str,
        context: &SecurityContext,
        threat: &ThreatInfo,
    ) -> Result<()> {
        // Implementation: Handle detected threats
        tracing::warn!("Handling threat {} for service {}", threat.threat_type, service_id);
        Ok(())
    }

    pub async fn create_incident(&self, incident: SecurityIncident) -> Result<String> {
        // Implementation: Create security incident in BearDog
        let incident_id = format!("INC-{}", Utc::now().timestamp());
        tracing::warn!("Created security incident {}: {:?}", incident_id, incident.incident_type);
        Ok(incident_id)
    }
}

// ============== SUPPORT TYPES ==============

#[derive(Debug)]
pub struct MonitoringResult {
    pub monitored: bool,
    pub risk_score: f64,
}

#[derive(Debug)]
pub struct SecurityAssessment {
    pub compliant: bool,
    pub violations: Vec<String>,
    pub risk_score: f64,
}

impl SecurityAssessment {
    pub fn is_compliant(&self) -> bool {
        self.compliant
    }

    pub fn to_details(&self) -> HashMap<String, serde_json::Value> {
        let mut details = HashMap::new();
        details.insert("compliant".to_string(), serde_json::to_value(self.compliant).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        details.insert("violations".to_string(), serde_json::to_value(&self.violations).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        details.insert("risk_score".to_string(), serde_json::to_value(self.risk_score).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        details
    }
}

#[derive(Debug)]
pub struct ConfigValidationResult {
    pub valid: bool,
    pub violations: Vec<String>,
}

impl ConfigValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

#[derive(Debug)]
pub struct DiscoveryValidationResult {
    pub trusted: bool,
    pub reason: String,
}

// ============== INTEGRATION EXAMPLE ==============

/// Complete example of setting up BearDog security hooks with Songbird
pub async fn setup_beardog_security_integration() -> Result<Box<dyn EventHook>> {
    // Create BearDog configuration
    let config = BearDogHookConfig {
        enabled: true,
        beardog_endpoint: "https://beardog.security.internal".to_string(),
        api_key: std::env::var("BEARDOG_API_KEY")
            .map_err(|_| "BEARDOG_API_KEY environment variable not set")?,
        threat_confidence_threshold: 0.7,
        fail_closed_on_assessment_error: false,
        enable_real_time_monitoring: true,
        enable_incident_response: true,
    };

    // Create BearDog client
    let beardog_client = Arc::new(BearDogMonitoringClient::new(
        config.beardog_endpoint.clone(),
        config.api_key.clone()
    ));

    // Test connectivity
    beardog_client.connect().await?;

    // Create security monitoring hook
    let hook = BearDogSecurityMonitoringHook::new(config, beardog_client);

    Ok(Box::new(hook))
}

/// Example of registering the hook with Songbird Orchestrator
pub async fn register_beardog_hooks_with_songbird() -> Result<()> {
    // Set up the security hook
    let security_hook = setup_beardog_security_integration().await?;

    // Create Songbird orchestrator
    let mut orchestrator = songbird_orchestrator::Orchestrator::builder()
        .build()?;

    // Register the BearDog security hook
    orchestrator.register_hook(security_hook).await?;

    // Start the orchestrator
    orchestrator.start().await?;

    tracing::info!("Songbird Orchestrator started with BearDog security integration");

    Ok(())
} 