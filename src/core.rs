//! Core BearDog orchestration engine
//! 
//! Manages the lifecycle and health of all BearDog security components.

use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::config::BearDogConfig;
use crate::error::{BearDogResult, BearDogError};
use crate::workflows::MultiPartyWorkflowEngine;
use crate::encryption::EncryptionEngine;
use crate::audit::AuditEngine;
use crate::threat_detection::ThreatDetectionEngine;
use crate::compliance::ComplianceEngine;
use crate::security_provider::BearDogSecurityProvider;

/// Core BearDog orchestration engine
#[derive(Clone)]
pub struct BearDogCore {
    config: Arc<BearDogConfig>,
    state: Arc<RwLock<CoreState>>,
    workflow_engine: Arc<MultiPartyWorkflowEngine>,
    encryption_engine: Arc<EncryptionEngine>,
    audit_engine: Arc<AuditEngine>,
    threat_detection_engine: Arc<ThreatDetectionEngine>,
    compliance_engine: Arc<ComplianceEngine>,
    security_provider: Arc<BearDogSecurityProvider>,
    startup_time: std::time::Instant,
    component_status: HashMap<String, ComponentStatus>,
}

/// Internal state of the BearDog core
#[derive(Debug)]
struct CoreState {
    /// When the core was started
    start_time: Option<DateTime<Utc>>,
    /// Status of individual components
    component_status: HashMap<String, ComponentStatus>,
    /// Overall system health
    health_status: HealthStatus,
    /// Performance metrics
    metrics: SystemMetrics,
}

/// Status of individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Whether the component is healthy
    pub healthy: bool,
    /// Last health check time
    pub last_check: DateTime<Utc>,
    /// Optional error message if unhealthy
    pub error_message: Option<String>,
    /// Component uptime
    pub uptime: Option<Duration>,
}

/// Overall system health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// All components are healthy
    Healthy,
    /// Some components have issues but system is operational
    Degraded,
    /// Critical components are failing
    Unhealthy,
    /// System is starting up
    Starting,
    /// System is shutting down
    Stopping,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage (0.0-100.0)
    pub cpu_usage_percent: f64,
    /// Active connections count
    pub active_connections: u32,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Error rate percentage (0.0-100.0)
    pub error_rate_percent: f64,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Overall system status
    pub status: HealthStatus,
    /// System uptime
    pub uptime: Option<Duration>,
    /// Individual component statuses
    pub components: Vec<ComponentStatus>,
    /// System metrics
    pub metrics: SystemMetrics,
    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,
}



impl Default for CoreState {
    fn default() -> Self {
        Self {
            start_time: None,
            component_status: HashMap::new(),
            health_status: HealthStatus::Starting,
            metrics: SystemMetrics {
                memory_usage_bytes: 0,
                cpu_usage_percent: 0.0,
                active_connections: 0,
                requests_per_second: 0.0,
                avg_response_time_ms: 0.0,
                error_rate_percent: 0.0,
            },
        }
    }
}

impl BearDogCore {
    /// Create a new BearDog core instance
    pub async fn new(config: BearDogConfig) -> BearDogResult<Self> {
        info!("🐻 Initializing BearDog Security Manager");
        
        // Initialize encryption engine
        let encryption_engine = Arc::new(EncryptionEngine::new(config.encryption.clone()).await?);
        
        // Initialize audit engine
        let audit_engine = Arc::new(AuditEngine::new().await);
        
        // Convert config types to module-specific types
        let threat_config = crate::threat_detection::ThreatDetectionConfig {
            enabled: config.threat_detection.enabled,
            rules_path: None,
            monitor_paths: vec![],
            alert_threshold: crate::threat_detection::ThreatLevel::Medium,
            cache_size: 1000,
            monitoring_interval: 30,
        };
        
        let compliance_config = crate::compliance::ComplianceConfig {
            enabled_standards: vec![
                crate::compliance::ComplianceStandard::GDPR,
                crate::compliance::ComplianceStandard::SOX,
                crate::compliance::ComplianceStandard::PCI_DSS,
            ],
            monitoring_interval: chrono::Duration::from_std(config.compliance.monitoring_interval).unwrap_or(chrono::Duration::minutes(5)),
            audit_retention: chrono::Duration::from_std(config.compliance.audit_retention).unwrap_or(chrono::Duration::days(365)),
            dashboard_refresh_interval: chrono::Duration::minutes(1),
            reporting: crate::compliance::ReportingConfig::default(),
        };
        
        // Initialize threat detection engine
        let threat_detection_engine = Arc::new(ThreatDetectionEngine::new(threat_config).await?);
        
        // Initialize compliance engine
        let compliance_engine = Arc::new(ComplianceEngine::new(compliance_config).await?);
        
        // Initialize workflow engine
        let workflow_engine = Arc::new(MultiPartyWorkflowEngine::new(config.workflows.clone()).await?);
        
        // Initialize security provider with placeholder for now
        let security_provider = Arc::new(crate::security_provider::BearDogSecurityProvider::new_placeholder());
        
        let core = Self {
            config: Arc::new(config),
            encryption_engine,
            audit_engine,
            threat_detection_engine,
            compliance_engine,
            workflow_engine,
            security_provider,
            startup_time: std::time::Instant::now(),
            component_status: HashMap::new(),
            state: Arc::new(RwLock::new(CoreState::default())),
        };
        
        info!("✅ BearDog Security Manager initialized successfully");
        Ok(core)
    }
    
    /// Create a placeholder core for initialization
    pub fn new_placeholder() -> Self {
        Self {
            config: Arc::new(BearDogConfig::default()),
            encryption_engine: Arc::new(EncryptionEngine::placeholder()),
            audit_engine: Arc::new(AuditEngine::placeholder()),
            threat_detection_engine: Arc::new(ThreatDetectionEngine::placeholder()),
            compliance_engine: Arc::new(ComplianceEngine::placeholder()),
            workflow_engine: Arc::new(MultiPartyWorkflowEngine::placeholder()),
            security_provider: Arc::new(crate::security_provider::BearDogSecurityProvider::new_placeholder()),
            startup_time: std::time::Instant::now(),
            component_status: HashMap::new(),
            state: Arc::new(RwLock::new(CoreState::default())),
        }
    }

    /// Create a core without security provider to break circular dependency
    pub fn new_without_security_provider() -> Self {
        Self {
            config: Arc::new(BearDogConfig::default()),
            encryption_engine: Arc::new(EncryptionEngine::placeholder()),
            audit_engine: Arc::new(AuditEngine::placeholder()),
            threat_detection_engine: Arc::new(ThreatDetectionEngine::placeholder()),
            compliance_engine: Arc::new(ComplianceEngine::placeholder()),
            workflow_engine: Arc::new(MultiPartyWorkflowEngine::placeholder()),
            // Use a bare minimum security provider that doesn't reference core
            security_provider: Arc::new(crate::security_provider::BearDogSecurityProvider::new_placeholder()),
            startup_time: std::time::Instant::now(),
            component_status: HashMap::new(),
            state: Arc::new(RwLock::new(CoreState::default())),
        }
    }

    /// Start the BearDog core and all components
    pub async fn start(&self) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.start_time = Some(Utc::now());
        state.health_status = HealthStatus::Starting;

        info!("Starting BearDog core components...");

        // Initialize components in order
        self.register_component(&mut state, "encryption", true, None).await;
        self.register_component(&mut state, "threat_detection", true, None).await;
        self.register_component(&mut state, "compliance", true, None).await;
        self.register_component(&mut state, "audit", true, None).await;
        self.register_component(&mut state, "workflows", true, None).await;
        self.register_component(&mut state, "nestgate_adapter", true, None).await;
        self.register_component(&mut state, "songbird_adapter", true, None).await;

        state.health_status = HealthStatus::Healthy;
        info!("BearDog core started successfully");

        Ok(())
    }

    /// Stop the BearDog core and all components
    pub async fn stop(&self) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.health_status = HealthStatus::Stopping;

        info!("Stopping BearDog core...");

        // Clear component statuses
        state.component_status.clear();
        
        info!("BearDog core stopped");
        Ok(())
    }

    /// Get current health status
    pub async fn health_check(&self) -> BearDogResult<HealthCheck> {
        let state = self.state.read().await;
        
        let uptime = state.start_time.map(|start_time| Utc::now() - start_time);

        let components: Vec<ComponentStatus> = state.component_status.values().cloned().collect();

        Ok(HealthCheck {
            status: state.health_status.clone(),
            uptime,
            components,
            metrics: state.metrics.clone(),
            timestamp: Utc::now(),
        })
    }

    /// Update system metrics
    pub async fn update_metrics(&self, metrics: SystemMetrics) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.metrics = metrics;
        Ok(())
    }

    /// Register a component with the core
    async fn register_component(
        &self,
        state: &mut CoreState,
        name: &str,
        healthy: bool,
        error_message: Option<String>,
    ) {
        let component_status = ComponentStatus {
            name: name.to_string(),
            healthy,
            last_check: Utc::now(),
            error_message,
            uptime: state.start_time.map(|start_time| Utc::now() - start_time),
        };

        state.component_status.insert(name.to_string(), component_status);
    }

    /// Update component status
    pub async fn update_component_status(
        &self,
        component_name: &str,
        healthy: bool,
        error_message: Option<String>,
    ) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        let start_time = state.start_time; // Get start_time before mutable borrow
        
        if let Some(status) = state.component_status.get_mut(component_name) {
            status.healthy = healthy;
            status.last_check = Utc::now();
            status.error_message = error_message;
            status.uptime = start_time.map(|st| Utc::now() - st);
        } else {
            return Err(BearDogError::not_found("component", component_name));
        }

        // Update overall health status based on component health
        let all_healthy = state.component_status.values().all(|s| s.healthy);
        let any_healthy = state.component_status.values().any(|s| s.healthy);

        state.health_status = if all_healthy {
            HealthStatus::Healthy
        } else if any_healthy {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };

        Ok(())
    }

    /// Get configuration reference
    pub fn config(&self) -> &BearDogConfig {
        &self.config
    }

    /// Get access to the workflow engine
    pub fn workflow_engine(&self) -> &MultiPartyWorkflowEngine {
        &self.workflow_engine
    }

    /// Get reference to the encryption engine
    pub fn encryption_engine(&self) -> &EncryptionEngine {
        &self.encryption_engine
    }

    /// Get reference to the audit engine
    pub fn audit_engine(&self) -> &AuditEngine {
        &self.audit_engine
    }

    /// Get reference to the threat detection engine
    pub fn threat_detection_engine(&self) -> &ThreatDetectionEngine {
        &self.threat_detection_engine
    }

    /// Get reference to the compliance engine
    pub fn compliance_engine(&self) -> &ComplianceEngine {
        &self.compliance_engine
    }

    /// Get reference to the security provider
    pub fn security_provider(&self) -> &BearDogSecurityProvider {
        &self.security_provider
    }


} 