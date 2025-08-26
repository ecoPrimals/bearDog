

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use songbird_orchestrator::traits::hooks::{
    EventHook, HookResult, HookConfig, OrchestratorEvent, HookContext,
    EventFilter, ExecutionConfig, RetryConfig
};
use songbird_orchestrator::traits::service::{ServiceInfo, ServiceRequest, ServiceResponse};
use songbird_orchestrator::errors::Result;

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

    fn is_security_related_error(&self, error_type: &str, error_message: &str) -> bool {
        let security_keywords = [
            "authentication", "authorization", "forbidden", "unauthorized",
            "token", "credential", "permission", "access denied",
            "invalid signature", "expired", "malformed",
        ];

        let error_lower = format_args!("{} {}", error_type, error_message).to_string().to_lowercase();
        security_keywords.iter().any(|keyword| error_lower.contains(keyword))
    }

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

        self.beardog_client.connect().await?;

        self.beardog_client.register_songbird_instance(
            &context.orchestrator_id,
            &self.config
        ).await?;

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

            OrchestratorEvent::RequestReceived { service_id, request, timestamp } => {
                result.log_messages.push(format_args!("Processing request {} for service {}", request.id, service_id).to_string());

                let security_context = self.extract_security_context(request).await;

                match self.beardog_client.monitor_request(service_id, &security_context).await {
                    Ok(monitoring_result) => {

                        match self.threat_detector.analyze_request(&security_context).await {
                            Ok(Some(threat)) => {
                                result.log_messages.push(format_args!("Threat detected: {} (confidence: {})", 
                                    threat.threat_type, threat.confidence).to_string());

                                if threat.confidence >= self.config.threat_confidence_threshold {

                                    if let Err(e) = self.incident_responder.handle_threat(
                                        service_id, 
                                        &security_context, 
                                        &threat
                                    ).await {
                                        result.log_messages.push(format_args!("Incident response failed: {}", e).to_string());
                                    }

                                    match threat.severity {
                                        ThreatSeverity::Critical | ThreatSeverity::High => {
                                            result.allow_operation = false;
                                            result.continue_chain = false;
                                            result.log_messages.push("Request blocked due to high threat level".to_string());
                                        },
                                        ThreatSeverity::Medium => {

                                            result.log_messages.push("Request allowed with extra monitoring".to_string());
                                        },
                                        ThreatSeverity::Low => {

                                            result.log_messages.push("Low threat detected, allowing with audit".to_string());
                                        },
                                    }
                                }
                            },
                            Ok(None) => {

                                result.log_messages.push("No threats detected".to_string());
                            },
                            Err(e) => {
                                result.log_messages.push(format_args!("Threat analysis failed: {}", e).to_string());

                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format_args!("BearDog monitoring failed: {}", e).to_string());

                    }
                }
            },

            OrchestratorEvent::ServiceRegistering { service_info, timestamp } => {
                result.log_messages.push(format_args!("Validating security for service: {}", service_info.name).to_string());

                match self.beardog_client.assess_service_security(service_info).await {
                    Ok(assessment) => {
                        if !assessment.is_compliant() {
                            result.allow_operation = false;
                            result.log_messages.push(format!(
                                "Service security assessment failed. Violations: {:?}", 
                                assessment.violations
                            ));

                            let incident = SecurityIncident {
                                incident_type: SecurityIncidentType::ServiceNonCompliance,
                                service_id: Some(service_info.id.clone()),
                                description: format_args!("Service {} failed security compliance", service_info.name).to_string(),
                                severity: IncidentSeverity::Medium,
                                details: assessment.to_details(),
                                timestamp: *timestamp,
                            };

                            if let Err(e) = self.incident_responder.create_incident(incident).await {
                                result.log_messages.push(format_args!("Failed to create incident: {}", e).to_string());
                            }
                        } else {
                            result.log_messages.push("Service security assessment passed".to_string());

                            if let Err(e) = self.beardog_client.log_compliance_success(
                                &service_info.id, 
                                &assessment
                            ).await {
                                result.log_messages.push(format_args!("Failed to log compliance: {}", e).to_string());
                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format_args!("Security assessment failed: {}", e).to_string());

                        if self.config.fail_closed_on_assessment_error {
                            result.allow_operation = false;
                            result.log_messages.push("Blocking service due to assessment failure".to_string());
                        }
                    }
                }
            },

            OrchestratorEvent::ErrorOccurred { error_type, error_message, service_id, context, timestamp } => {
                result.log_messages.push(format_args!("Processing error: {} - {}", error_type, error_message).to_string());

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
                            result.log_messages.push(format_args!("Security incident created: {}", incident_id).to_string());
                        },
                        Err(e) => {
                            result.log_messages.push(format_args!("Failed to create security incident: {}", e).to_string());
                        }
                    }
                }
            },

            OrchestratorEvent::ConfigurationChanged { config_section, old_config, new_config, timestamp } => {
                result.log_messages.push(format_args!("Validating config change for section: {}", config_section).to_string());

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
                            result.log_messages.push(format_args!("Config validation failed: {}", e).to_string());

                            result.allow_operation = false;
                            result.log_messages.push("Blocking config change due to validation failure".to_string());
                        }
                    }
                }
            },

            OrchestratorEvent::HealthCheckCompleted { service_id, healthy, details, timestamp } => {

                if !healthy {

                    if let Some(error_details) = details.get("error") {
                        if let Some(error_str) = error_details.as_str() {
                            if self.is_security_related_error("health_check", error_str) {
                                let incident = SecurityIncident {
                                    incident_type: SecurityIncidentType::ServiceSecurityFailure,
                                    service_id: Some(service_id.clone()),
                                    description: format_args!("Security-related health check failure: {}", error_str).to_string(),
                                    severity: IncidentSeverity::Medium,
                                    details: details.clone(),
                                    timestamp: *timestamp,
                                };

                                if let Err(e) = self.incident_responder.create_incident(incident).await {
                                    result.log_messages.push(format_args!("Failed to create health incident: {}", e).to_string());
                                }
                            }
                        }
                    }
                }
            },

            OrchestratorEvent::ServiceDiscovered { service_info, discovery_source, timestamp } => {
                result.log_messages.push(format_args!("Validating discovered service: {}", service_info.name).to_string());

                match self.beardog_client.validate_service_discovery(service_info, discovery_source).await {
                    Ok(validation_result) => {
                        if !validation_result.trusted {
                            result.log_messages.push(format!(
                                "Discovered service not trusted: {}", 
                                validation_result.reason
                            ));

                            let incident = SecurityIncident {
                                incident_type: SecurityIncidentType::UntrustedServiceDiscovery,
                                service_id: Some(service_info.id.clone()),
                                description: format_args!("Untrusted service discovered: {}", validation_result.reason).to_string(),
                                severity: IncidentSeverity::Medium,
                                details: {
                                    let mut details = HashMap::with_capacity(16);
                                    details.insert("discovery_source".to_string(), serde_json::to_value(discovery_source)?);
                                    details.insert("service_info".to_string(), serde_json::to_value(service_info)?);
                                    details
                                },
                                timestamp: *timestamp,
                            };

                            if let Err(e) = self.incident_responder.create_incident(incident).await {
                                result.log_messages.push(format_args!("Failed to create discovery incident: {}", e).to_string());
                            }
                        }
                    },
                    Err(e) => {
                        result.log_messages.push(format_args!("Service discovery validation failed: {}", e).to_string());
                    }
                }
            },

            _ => {

                result.log_messages.push(format_args!("Processed event: {:?}", event).to_string());
            }
        }

        let execution_time = start_time.elapsed().as_millis() as u64;
        result.execution_time_ms = execution_time;
        
        self.update_statistics(result.success, execution_time).await;

        Ok(result)
    }

    async fn cleanup(&self) -> Result<()> {

        self.beardog_client.disconnect().await?;

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
        let mut settings = HashMap::with_capacity(16);
        settings.insert("enabled".to_string(), serde_json::to_value(self.enabled).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);
        settings.insert("endpoint".to_string(), serde_json::to_value(&self.beardog_endpoint).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);
        settings.insert("threat_threshold".to_string(), serde_json::to_value(self.threat_confidence_threshold).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
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

pub struct BearDogMonitoringClient {
    endpoint: String,
    api_key: String,
}

impl BearDogMonitoringClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        Self { endpoint, api_key }
    }

    pub async fn connect(&self) -> Result<()> {

        tracing::info!("Connected to BearDog monitoring at {}", self.endpoint);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {

        tracing::info!("Disconnected from BearDog monitoring");
        Ok(())
    }

    pub async fn register_songbird_instance(
        &self, 
        orchestrator_id: &str, 
        config: &BearDogHookConfig
    ) -> Result<()> {

        tracing::info!("Registered Songbird instance {} with BearDog", orchestrator_id);
        Ok(())
    }

    pub async fn monitor_request(
        &self, 
        service_id: &str, 
        context: &SecurityContext
    ) -> Result<MonitoringResult> {

        Ok(MonitoringResult { 
            monitored: true, 
            risk_score: 0.1 
        })
    }

    pub async fn assess_service_security(&self, service_info: &ServiceInfo) -> Result<SecurityAssessment> {

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

        tracing::info!("Threat detector initialized");
        Ok(())
    }

    pub async fn analyze_request(&self, context: &SecurityContext) -> Result<Option<ThreatInfo>> {

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

        tracing::warn!("Handling threat {} for service {}", threat.threat_type, service_id);
        Ok(())
    }

    pub async fn create_incident(&self, incident: SecurityIncident) -> Result<String> {

        let incident_id = format_args!("INC-{}", Utc::now().to_string().timestamp());
        tracing::warn!("Created security incident {}: {:?}", incident_id, incident.incident_type);
        Ok(incident_id)
    }
}

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
        let mut details = HashMap::with_capacity(16);
        details.insert("compliant".to_string(), serde_json::to_value(self.compliant).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);
        details.insert("violations".to_string(), serde_json::to_value(&self.violations).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?);
        details.insert("risk_score".to_string(), serde_json::to_value(self.risk_score).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
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

pub async fn setup_beardog_security_integration() -> Result<Box<dyn EventHook>> {

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

    let beardog_client = Arc::new(BearDogMonitoringClient::new(
        config.beardog_endpoint.clone(),
        config.api_key.clone()
    ));

    beardog_client.connect().await?;

    let hook = BearDogSecurityMonitoringHook::new(config, beardog_client);

    Ok(Box::new(hook))
}

pub async fn register_beardog_hooks_with_songbird() -> Result<()> {

    let security_hook = setup_beardog_security_integration().await?;

    let mut orchestrator = songbird_orchestrator::Orchestrator::builder()
        .build()?;

    orchestrator.register_hook(security_hook).await?;

    orchestrator.start().await?;

    tracing::info!("Songbird Orchestrator started with BearDog security integration");

    Ok(())
} 