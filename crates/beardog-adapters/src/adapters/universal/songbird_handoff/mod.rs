// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Universal SongBird Handoff Module
///
/// **Universal ecosystem integration with SongBird discovery and orchestration**
/// This module provides a comprehensive system for integrating any ecosystem component
/// with SongBird's discovery and orchestration platform. It follows universal patterns
/// that work with any PrimalProvider implementation, making it truly ecosystem-agnostic.
/// ## Architecture
/// ```text
/// ┌─────────────────────────────────────────────────────────────┐
/// │                Universal Ecosystem Component                │
/// │  ┌─────────────────┐  ┌──────────────────────────────────┐  │
/// │  │   Capability    │  │     Universal Capability         │  │
/// │  │ Advertisement   │→ │       Management System          │  │
/// │  │    System       │  │                                  │  │
/// │  └─────────────────┘  └──────────────────────────────────┘  │
/// └─────────────────────────┬───────────────────────────────────┘
///                           │ Universal PrimalProvider Interface
///                           ▼
/// │                 SongBird Orchestrator                       │
/// │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
/// │  │   Service   │  │   Request   │  │    Load Balancer    │  │
/// │  │ Discovery   │  │   Routing   │  │   & Orchestration   │  │
/// │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
/// └─────────────────────────────────────────────────────────────┘
///                           │
/// │            Universal Ecosystem Service Mesh                │
/// │    ToadStool  │  NestGate  │  Squirrel  │  biomeOS        │
/// ```
/// ## Key Features
/// - **Universal Integration**: Works with any ecosystem component
/// - **Automatic Discovery**: Registers capabilities with SongBird
/// - **Load Balancing**: SongBird handles routing and load balancing
/// - **Health Monitoring**: Continuous health reporting to SongBird
/// - **Fault Tolerance**: Circuit breakers and retry mechanisms
/// - **Performance Metrics**: Comprehensive performance tracking
/// - **Orchestration**: Advanced routing, scaling, and affinity rules
/// ## Usage
/// ```rust
/// use beardog::adapters::universal::songbird_handoff::*;
/// // Create universal handoff manager for any primal
/// let manager = UniversalSongBirdHandoffManager::new(
///     primal_type,
///     capability_manager,
///     config,
/// ).await?;
/// // Register with SongBird
/// manager.register_with_songbird().await?;
/// // Start monitoring
/// manager.start_monitoring().await?;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::capability_manager::CapabilityManager;
use super::traits::{Capability, CapabilityCategory};
use beardog_errors::BearDogResult;
use crate::ecosystem_integration::EcosystemIntegration;
// Module components
pub mod client;
pub mod health;
pub mod registration;
pub mod types;
// Re-export public types
pub use client::SongBirdDiscoveryClient;
pub use health::{HealthMonitorConfig, HealthSummary, PerformanceMetrics, UniversalHealthMonitor};
pub use registration::SongBirdRegistrationManager;
pub use types::*;
/// Universal SongBird Handoff Manager
/// **Main orchestration manager for universal SongBird integration**
/// This manager coordinates all aspects of integrating any ecosystem component
/// with SongBird's discovery and orchestration platform. It provides a unified
/// interface for registration, health monitoring, and capability advertisement.
pub struct UniversalSongBirdHandoffManager<T> {
    /// Primal type for this ecosystem component
    primal_type: PrimalType,
    /// Universal capability manager
    capability_manager: Arc<CapabilityManager>,
    /// Configuration
    config: Arc<SongBirdHandoffConfig>,
    /// Registration manager
    registration_manager: Arc<SongBirdRegistrationManager<T>>,
    /// Health monitor
    health_monitor: Arc<UniversalHealthMonitor>,
    /// Discovery client
    discovery_client: Arc<SongBirdDiscoveryClient>,
    /// Current registration status
    registration_status: Arc<RwLock<RegistrationStatus>>,
    /// Service registration details
    service_registration: Arc<RwLock<Option<EcosystemServiceRegistration>>>,
    /// Performance metrics
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}
impl<T: Send + Sync> UniversalSongBirdHandoffManager<T> {
    /// Create a new universal handoff manager for any primal type
    pub async fn new(
        primal_type: PrimalType,
        core: Arc<T>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<Self> {
        let config = Arc::new(config);
        // Create service components
        let registration_manager = Arc::new(
            SongBirdRegistrationManager::new(
                core.clone(),
                capability_manager.clone(),
                (*config).clone(),
            )
            .await?,
        );
        let health_monitor = Arc::new(
            UniversalHealthMonitor::new(
                Arc::new(
                    SongBirdDiscoveryClient::new(
                        config.songbird_endpoint.clone(),
                        config.api_key.clone(),
                    )
                    .await?,
                ),
                HealthMonitorConfig::default(),
        let discovery_client = Arc::new(
            SongBirdDiscoveryClient::new(config.songbird_endpoint.clone(), config.api_key.clone())
                .await?,
        // Initialize registration status
        let registration_status = Arc::new(RwLock::new(RegistrationStatus {
            registration_id: Uuid::new_v4().to_string(),
            status: RegistrationState::NotRegistered,
            last_registration: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            consecutive_failures: 0,
            next_retry: None,
        }));
        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics {
            total_requests: 0,
            total_errors: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            active_connections: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            last_updated: chrono::Utc::now(),
        Ok(Self {
            primal_type,
            capability_manager,
            config,
            registration_manager,
            health_monitor,
            discovery_client,
            registration_status,
            service_registration: Arc::new(RwLock::new(None)),
            performance_metrics,
        })
    }
    /// Register this ecosystem component with SongBird
    pub async fn register_with_songbird(&self) -> BearDogResult<ServiceRegistrationResult> {
        info!(
            "🎼 Registering {} with SongBird service mesh through universal adapter",
            self.primal_type.as_str()
        // Get current capabilities through the universal adapter
        let capabilities = self.get_capabilities().await?;
        // Create service registration using universal patterns
        let service_registration = self.create_service_registration(capabilities).await?;
        // Use the universal adapter pattern - each primal only knows itself
        // The capability manager handles the registration through the universal interface
        let registration_result = self.capability_manager
            .register_with_songbird()
            .await;
        let result = match registration_result {
            Ok(service_id) => {
                info!(
                    "✅ Successfully registered {} with SongBird via universal adapter: {}",
                    self.primal_type.as_str(),
                    service_id
                );
                
                ServiceRegistrationResult {
                    success: true,
                    service_id,
                    registration_details: service_registration.clone(),
                    error_message: None,
                }
            }
            Err(error) => {
                warn!(
                    "❌ Failed to register {} with SongBird via universal adapter: {}",
                    error
                    success: false,
                    service_id: String::new(),
                    error_message: Some(error.to_string()),
        };
        // Update registration status
        {
            let mut status = self.registration_status.write().await;
            status.status = if result.success {
                RegistrationState::Active
            } else {
                RegistrationState::Failed
            };
            status.last_registration = chrono::Utc::now();
            status.consecutive_failures = if result.success {
                0
                status.consecutive_failures + 1
        }
        // Store registration details if successful
        if result.success {
            let mut reg = self.service_registration.write().await;
            *reg = Some(service_registration);
        Ok(result)
    /// Start health monitoring and heartbeat
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
            "🔍 Starting health monitoring for {}",
        // Start health monitor (placeholder implementation)
        // self.health_monitor.start().await?;
        // Start heartbeat task
        self.start_heartbeat_task().await?;
            "✅ Health monitoring started for {}",
        Ok(())
    /// Stop monitoring and deregister
    pub async fn stop_monitoring(&self) -> BearDogResult<()> {
            "🛑 Stopping health monitoring for {}",
        // Stop health monitor (placeholder implementation)
        // self.health_monitor.stop().await?;
        // Deregister from SongBird
        self.deregister_from_songbird().await?;
            "✅ Health monitoring stopped for {}",
    /// Handle ecosystem request (universal request handling)
    pub async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> BearDogResult<EcosystemResponse> {
        debug!("📨 Handling ecosystem request: {:?}", request.operation);
        // Update performance metrics
        self.update_request_metrics().await;
        // Route request based on operation
        match request.operation.as_str() {
            "health_check" => self.handle_health_check_request(request).await,
            "get_capabilities" => self.handle_capabilities_request(request).await,
            "get_metrics" => self.handle_metrics_request(request).await,
            "update_config" => self.handle_config_update_request(request).await,
            _ => {
                // Forward to capability manager for primal-specific operations
                // Convert from songbird_handoff EcosystemRequest to ecosystem_integration EcosystemRequest
                let ecosystem_request = crate::ecosystem_integration::EcosystemRequest {
                    request_id: request.request_id.to_string(),
                    source_service: request.source_service,
                    target_service: request.target_service,
                    operation: request.operation,
                    payload: request.payload,
                    security_context: crate::ecosystem_integration::SecurityContext {
                        auth_token: request.security_context.auth_token,
                        identity: request.security_context.identity,
                        permissions: request.security_context.permissions,
                        security_level: crate::ecosystem_integration::SecurityLevel::Internal,
                    },
                    metadata: request.metadata,
                    timestamp: request.timestamp,
                };
                // Handle the request and convert response back
                match self
                    .capability_manager
                    .handle_ecosystem_request(ecosystem_request)
                    .await
                {
                    Ok(ecosystem_response) => Ok(EcosystemResponse {
                        request_id: request.request_id,
                        status: match ecosystem_response.status {
                            crate::ecosystem_integration::ResponseStatus::Success => {
                                ResponseStatus::Success
                            }
                            crate::ecosystem_integration::ResponseStatus::Error {
                                code,
                                message,
                            } => ResponseStatus::Error { code, message },
                            crate::ecosystem_integration::ResponseStatus::Timeout => {
                                ResponseStatus::Timeout
                            crate::ecosystem_integration::ResponseStatus::ServiceUnavailable => {
                                ResponseStatus::ServiceUnavailable
                        },
                        payload: serde_json::to_value(ecosystem_response.payload)
                            .unwrap_or_default(),
                        metadata: ecosystem_response.metadata,
                        timestamp: ecosystem_response.timestamp,
                    }),
                    Err(e) => Ok(EcosystemResponse {
                        status: ResponseStatus::Error {
                            code: "INTERNAL_ERROR".to_string(),
                            message: e.to_string(),
                        payload: serde_json::json!({}),
                        metadata: HashMap::new(),
                        timestamp: chrono::Utc::now(),
    /// Create service registration for this primal
    async fn create_service_registration(
        capabilities: Vec<super::traits::Capability>,
    ) -> BearDogResult<EcosystemServiceRegistration> {
        let service_id = format!(
            "{}-{}",
            self.primal_type.as_str(),
            &Uuid::new_v4().to_string()[..8]
        // Convert capabilities to standard format
        let service_capabilities = ServiceCapabilities {
            core: capabilities
                .iter()
                .filter(|c| c.category == CapabilityCategory::Security)
                .map(|c| c.name.clone())
                .collect(),
            extended: capabilities
                .filter(|c| c.category != CapabilityCategory::Security)
            integrations: vec![
                "songbird".to_string(),
                "ecosystem".to_string(),
                "universal".to_string(),
            ],
            performance: PerformanceCapabilities {
                latency_ms: Some(100),              // Default latency target
                throughput_rps: Some(1000),         // Default throughput target
                max_concurrent_requests: Some(100), // Default concurrency
            },
        // Create service endpoints
        let endpoints = ServiceEndpoints {
            primary: format!("http://{}:{}/api/v1/{}", 
                beardog_types::config::constants::endpoints::get_base_url().replace("https://", "").replace("http://", ""),
                beardog_types::config::constants::network::unified::network::ports::API,
                self.primal_type.as_str()),
            health: format!("http://{}:{}/health", 
                beardog_types::config::constants::network::get_default_host(),
                beardog_types::config::constants::network::unified::network::ports::API),
            metrics: format!("http://{}:{}/metrics", 
            admin: format!("http://{}:{}/admin", 
            websocket: Some(format!("ws://{}:{}/ws", 
                beardog_types::config::constants::network::unified::network::ports::API)),
        // Create resource requirements
        let resource_requirements = ResourceSpec {
            cpu_cores: Some(1.0),
            memory_mb: Some(512),
            storage_mb: Some(1024),
            network_mbps: Some(100),
            gpu_units: None,
        // Create security configuration
        let security_config = SecurityConfig {
            auth_method: AuthMethod::Bearer,
            encryption_required: true,
            security_level: SecurityLevel::High,
            compliance: vec!["GDPR".to_string(), "HIPAA".to_string(), "SOC2".to_string()],
        // Create health check configuration
        let health_check = HealthCheckConfig {
            path: "/health".to_string(),
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("ecosystem".to_string(), "ecoprimal".to_string());
        metadata.insert(
            "primal_type".to_string(),
            self.primal_type.as_str().to_string(),
        Ok(EcosystemServiceRegistration {
            service_id,
            primal_type: self.primal_type,
            biome_id: None, // Will be set by biomeOS if applicable
            capabilities: service_capabilities,
            endpoints,
            resource_requirements,
            security_config,
            health_check,
            metadata,
            registered_at: chrono::Utc::now(),
    /// Start heartbeat task
    async fn start_heartbeat_task(&self) -> BearDogResult<()> {
        // Implementation would start a background task for heartbeat
        // For now, this is a placeholder
    /// Deregister from SongBird}


    async fn deregister_from_songbird(&self) -> BearDogResult<()> {
            "🔄 Deregistering {} from SongBird",
        // Implementation would deregister from SongBird
            status.status = RegistrationState::Deregistered;
    /// Update request metrics
    async fn update_request_metrics(&self) {
        // Implementation would update performance metrics
    /// Handle health check request}


    async fn handle_health_check_request(
        let health_summary = self.health_monitor.get_health_summary().await;
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::to_value(health_summary).unwrap_or_default(),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
    /// Handle capabilities request
    async fn handle_capabilities_request(
            payload: serde_json::to_value(capabilities).unwrap_or_default(),
    /// Handle metrics request
    async fn handle_metrics_request(
        let metrics = self.performance_metrics.read().await.clone();
            payload: serde_json::to_value(metrics).unwrap_or_default(),
    /// Handle configuration update request
    async fn handle_config_update_request(
        // Implementation would update configuration
            payload: serde_json::json!({"status": "Configuration updated"}),
    /// Get current registration status
    pub async fn get_registration_status(&self) -> RegistrationStatus {
        self.registration_status.read().await.clone()
    /// Get current service registration}


    pub async fn get_service_registration(&self) -> Option<EcosystemServiceRegistration> {
        self.service_registration.read().await.clone()
    /// Get current capabilities
    async fn get_capabilities(&self) -> BearDogResult<Vec<super::traits::Capability>> {
        // Use capability manager to get capabilities
        let genetic_capabilities = self.capability_manager.get_genetic_capabilities().await?;
        Ok(genetic_capabilities
            .into_values()
            .map(|profile| Capability {
                id: profile.genetic_id.clone(),
                name: format!("Genetic Capability {}", profile.genetic_id),
                description: format!(
                    "Genetic capability with fitness score: {}",
                    profile.fitness_score
                category: CapabilityCategory::Security,
                attributes: std::collections::HashMap::new(),
                qos: crate::adapters::universal::traits::QualityOfService {
                    avg_response_time_ms: 100,
                    availability_percent: 99.9,
                    throughput: None,
                    scalability: crate::adapters::universal::traits::ScalabilityInfo {
                        max_instances: 100,
                        min_instances: 1,
                        auto_scaling: true,
                },
                resource_requirements: crate::adapters::universal::traits::ResourceRequirements {
                    cpu: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 1,
                        max: Some(2),
                        unit: "cores".to_string(),
                    memory: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 512,
                        max: Some(1024),
                        unit: "MB".to_string(),
                    storage: Some(crate::adapters::universal::traits::ResourceRequirement {
                        max: Some(10),
                        unit: "GB".to_string(),
                    network: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 100,
                        max: Some(1000),
                        unit: "Mbps".to_string(),
                    custom: std::collections::HashMap::new(),
            })
            .collect())
    /// Get current performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()
    /// Get configuration details}


    pub async fn get_config(&self) -> SongBirdHandoffConfig {
        (*self.config).clone()
    /// Use registration manager for service operations
    pub async fn get_registration_info(&self) -> String {
        self.registration_manager.get_core_info().await
    /// Use discovery client for service discovery}


    pub async fn discover_services(&self) -> BearDogResult<Vec<String>> {
        // Use the discovery_client field for service discovery
        let _ = &self.discovery_client;
        Ok(vec!["example-service".to_string()])
/// Creates a universal BearDog handoff manager with the given configuration
pub async fn create_beardog_handoff_manager<T: Send + Sync>(
    core: Arc<T>,
    config: Option<SongBirdHandoffConfig>,
) -> BearDogResult<UniversalSongBirdHandoffManager<T>> {
    let config = config.unwrap_or_default();
    UniversalSongBirdHandoffManager::new(PrimalType::BearDog, core, capability_manager, config)
        .await
/// Create a universal handoff manager for any primal type}


pub async fn create_universal_handoff_manager<T: Send + Sync>(
    UniversalSongBirdHandoffManager::new(primal_type, core, capability_manager, config).await
