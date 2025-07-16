//! Universal Ecosystem Manager
//!
//! **Central orchestration for universal ecosystem integration**
//!
//! This manager coordinates all PrimalProvider implementations across the
//! ecosystem, enabling universal integration patterns that work for any
//! ecosystem component regardless of domain.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use uuid::Uuid;

use super::discovery::EcosystemDiscovery;
use super::registry::CapabilityRegistry;
use super::traits::*;
use crate::{BearDogCore, BearDogError, BearDogResult};

/// Universal Ecosystem Manager
///
/// Central orchestration system that manages all PrimalProvider implementations,
/// enabling universal integration patterns across the entire ecoPrimals ecosystem.
pub struct UniversalEcosystemManager {
    /// Core BearDog instance
    core: Arc<BearDogCore>,

    /// Registered PrimalProvider implementations
    providers: Arc<RwLock<HashMap<String, Box<dyn PrimalProvider>>>>,

    /// Capability registry for ecosystem-wide discovery
    capability_registry: Arc<CapabilityRegistry>,

    /// Ecosystem discovery service
    ecosystem_discovery: Arc<EcosystemDiscovery>,

    /// Active service requests
    active_requests: Arc<RwLock<HashMap<Uuid, ServiceRequest>>>,

    /// Provider configurations
    provider_configs: Arc<RwLock<HashMap<String, ProviderConfig>>>,

    /// Manager status
    manager_status: Arc<RwLock<EcosystemManagerStatus>>,
}

/// Status of the Universal Ecosystem Manager
#[derive(Debug, Clone)]
pub struct EcosystemManagerStatus {
    /// Total registered providers
    pub total_providers: u32,
    /// Active providers
    pub active_providers: u32,
    /// Healthy providers
    pub healthy_providers: u32,
    /// Degraded providers
    pub degraded_providers: u32,
    /// Unhealthy providers
    pub unhealthy_providers: u32,
    /// Total capabilities
    pub total_capabilities: u32,
    /// Active service requests
    pub active_requests: u32,
    /// Last health check time
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

impl UniversalEcosystemManager {
    /// Create a new Universal Ecosystem Manager
    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        info!("🌐 Initializing Universal Ecosystem Manager");

        let capability_registry = Arc::new(CapabilityRegistry::new().await?);
        let ecosystem_discovery = Arc::new(EcosystemDiscovery::new(core.clone()).await?);

        let manager = Self {
            core,
            providers: Arc::new(RwLock::new(HashMap::new())),
            capability_registry,
            ecosystem_discovery,
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            provider_configs: Arc::new(RwLock::new(HashMap::new())),
            manager_status: Arc::new(RwLock::new(EcosystemManagerStatus {
                total_providers: 0,
                active_providers: 0,
                healthy_providers: 0,
                degraded_providers: 0,
                unhealthy_providers: 0,
                total_capabilities: 0,
                active_requests: 0,
                last_health_check: chrono::Utc::now(),
            })),
        };

        info!("✅ Universal Ecosystem Manager initialized successfully");
        Ok(manager)
    }

    /// Register a PrimalProvider with the ecosystem
    pub async fn register_provider<T: PrimalProvider + 'static>(
        &self,
        mut provider: T,
        config: ProviderConfig,
    ) -> BearDogResult<EcosystemRegistration> {
        let ecosystem_id = provider.ecosystem_id().to_string();
        let instance_id = provider.instance_id().to_string();

        info!(
            "🔌 Registering PrimalProvider: {} ({})",
            ecosystem_id, instance_id
        );

        // Initialize the provider
        provider.initialize(config.clone()).await?;

        // Register capabilities with the capability registry
        self.capability_registry
            .register_capabilities(&ecosystem_id, &instance_id, provider.capabilities())
            .await?;

        // Register with ecosystem discovery
        let registration = provider.register_with_ecosystem().await?;

        // Store the provider and configuration
        {
            let mut providers = self.providers.write().await;
            let provider_key = format!("{ecosystem_id}:{instance_id}");
            providers.insert(provider_key, Box::new(provider));
        }

        {
            let mut configs = self.provider_configs.write().await;
            let config_key = format!("{ecosystem_id}:{instance_id}");
            configs.insert(config_key, config);
        }

        // Update manager status
        self.update_manager_status().await?;

        info!(
            "✅ Successfully registered PrimalProvider: {} ({})",
            ecosystem_id, instance_id
        );
        Ok(registration)
    }

    /// Unregister a PrimalProvider from the ecosystem
    pub async fn unregister_provider(
        &self,
        ecosystem_id: &str,
        instance_id: &str,
    ) -> BearDogResult<()> {
        info!(
            "🔌 Unregistering PrimalProvider: {} ({})",
            ecosystem_id, instance_id
        );

        let provider_key = format!("{ecosystem_id}:{instance_id}");

        // Remove from providers and shutdown
        {
            let mut providers = self.providers.write().await;
            if let Some(mut provider) = providers.remove(&provider_key) {
                if let Err(e) = provider.shutdown().await {
                    error!("Error shutting down provider {}: {}", provider_key, e);
                }
            }
        }

        // Remove configuration
        {
            let mut configs = self.provider_configs.write().await;
            configs.remove(&provider_key);
        }

        // Unregister capabilities
        self.capability_registry
            .unregister_capabilities(ecosystem_id, instance_id)
            .await?;

        // Update manager status
        self.update_manager_status().await?;

        info!(
            "✅ Successfully unregistered PrimalProvider: {} ({})",
            ecosystem_id, instance_id
        );
        Ok(())
    }

    /// Route a service request to the appropriate provider
    pub async fn route_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {
        debug!(
            "🔄 Routing request: {} ({})",
            request.request_type, request.request_id
        );

        // Store active request
        {
            let mut active_requests = self.active_requests.write().await;
            active_requests.insert(request.request_id, request.clone());
        }

        // Find capable provider
        let provider_key = self.find_capable_provider(&request).await?;

        // Route to provider
        let response = {
            let providers = self.providers.read().await;
            if let Some(provider) = providers.get(&provider_key) {
                provider.handle_request(request.clone()).await
            } else {
                Err(BearDogError::internal(format!(
                    "Provider {provider_key} not found after capability check"
                )))
            }
        };

        // Remove from active requests
        {
            let mut active_requests = self.active_requests.write().await;
            active_requests.remove(&request.request_id);
        }

        response
    }

    /// Find a provider capable of handling the request
    async fn find_capable_provider(&self, request: &ServiceRequest) -> BearDogResult<String> {
        let providers = self.providers.read().await;

        // First, try to find a provider that explicitly can handle this request
        for (provider_key, provider) in providers.iter() {
            if provider.can_handle_request(request) {
                return Ok(provider_key.clone());
            }
        }

        // If no explicit handler, try to find by capability
        if let Some(target_ecosystem) = &request.context.target_ecosystem {
            // Look for specific ecosystem
            for (provider_key, _) in providers.iter() {
                if provider_key.starts_with(target_ecosystem) {
                    return Ok(provider_key.clone());
                }
            }
        }

        // If still no match, try to find by request type
        if request.request_type.starts_with("security.") {
            // Look for security providers
            for (provider_key, provider) in providers.iter() {
                if provider.ecosystem_id() == "beardog" {
                    return Ok(provider_key.clone());
                }
            }
        }

        Err(BearDogError::internal(format!(
            "No capable provider found for request type: {}",
            request.request_type
        )))
    }

    /// Get all registered providers
    pub async fn get_providers(&self) -> Vec<ProviderInfo> {
        let providers = self.providers.read().await;

        let mut provider_infos = Vec::new();
        for (_, provider) in providers.iter() {
            provider_infos.push(ProviderInfo {
                ecosystem_id: provider.ecosystem_id().to_string(),
                instance_id: provider.instance_id().to_string(),
                service_name: provider.service_name().to_string(),
                service_version: provider.service_version().to_string(),
                capabilities: provider.capabilities(),
                dependencies: provider.dependencies(),
                endpoints: provider.endpoints(),
                health_status: provider.health_check().await,
                metadata: provider.metadata(),
            });
        }

        provider_infos
    }

    /// Get provider by ecosystem and instance ID
    pub async fn get_provider(
        &self,
        ecosystem_id: &str,
        instance_id: &str,
    ) -> Option<ProviderInfo> {
        let providers = self.providers.read().await;
        let provider_key = format!("{ecosystem_id}:{instance_id}");

        if let Some(provider) = providers.get(&provider_key) {
            Some(ProviderInfo {
                ecosystem_id: provider.ecosystem_id().to_string(),
                instance_id: provider.instance_id().to_string(),
                service_name: provider.service_name().to_string(),
                service_version: provider.service_version().to_string(),
                capabilities: provider.capabilities(),
                dependencies: provider.dependencies(),
                endpoints: provider.endpoints(),
                health_status: provider.health_check().await,
                metadata: provider.metadata(),
            })
        } else {
            None
        }
    }

    /// Get all capabilities across all providers
    pub async fn get_all_capabilities(&self) -> Vec<Capability> {
        let providers = self.providers.read().await;

        let mut all_capabilities = Vec::new();
        for (_, provider) in providers.iter() {
            all_capabilities.extend(provider.capabilities());
        }

        all_capabilities
    }

    /// Get capabilities by category
    pub async fn get_capabilities_by_category(
        &self,
        category: CapabilityCategory,
    ) -> Vec<Capability> {
        let all_capabilities = self.get_all_capabilities().await;
        all_capabilities
            .into_iter()
            .filter(|cap| cap.category == category)
            .collect()
    }

    /// Perform health check on all providers
    pub async fn health_check_all(&self) -> BearDogResult<EcosystemHealthReport> {
        let providers = self.providers.read().await;

        let mut health_reports = Vec::new();
        for (provider_key, provider) in providers.iter() {
            let health_status = provider.health_check().await;
            health_reports.push(ProviderHealthReport {
                provider_key: provider_key.clone(),
                ecosystem_id: provider.ecosystem_id().to_string(),
                instance_id: provider.instance_id().to_string(),
                health_status,
                last_check: chrono::Utc::now(),
            });
        }

        // Calculate overall health
        let total_providers = health_reports.len() as u32;
        let healthy_providers = health_reports
            .iter()
            .filter(|report| matches!(report.health_status, HealthStatus::Healthy))
            .count() as u32;
        let degraded_providers = health_reports
            .iter()
            .filter(|report| matches!(report.health_status, HealthStatus::Degraded { .. }))
            .count() as u32;
        let unhealthy_providers = health_reports
            .iter()
            .filter(|report| matches!(report.health_status, HealthStatus::Unhealthy { .. }))
            .count() as u32;

        let overall_health = if unhealthy_providers > 0 {
            EcosystemHealthStatus::Unhealthy
        } else if degraded_providers > 0 {
            EcosystemHealthStatus::Degraded
        } else {
            EcosystemHealthStatus::Healthy
        };

        Ok(EcosystemHealthReport {
            overall_health,
            total_providers,
            healthy_providers,
            degraded_providers,
            unhealthy_providers,
            provider_reports: health_reports,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get manager status
    pub async fn get_status(&self) -> EcosystemManagerStatus {
        let status = self.manager_status.read().await;
        status.clone()
    }

    /// Update manager status
    async fn update_manager_status(&self) -> BearDogResult<()> {
        let providers = self.providers.read().await;
        let active_requests = self.active_requests.read().await;
        let total_capabilities = self.get_all_capabilities().await.len() as u32;

        let mut healthy_providers = 0;
        let mut degraded_providers = 0;
        let mut unhealthy_providers = 0;

        for (_, provider) in providers.iter() {
            match provider.health_check().await {
                HealthStatus::Healthy => healthy_providers += 1,
                HealthStatus::Degraded { .. } => degraded_providers += 1,
                HealthStatus::Unhealthy { .. } => unhealthy_providers += 1,
                _ => {}
            }
        }

        let mut status = self.manager_status.write().await;
        status.total_providers = providers.len() as u32;
        status.active_providers = providers.len() as u32;
        status.healthy_providers = healthy_providers;
        status.degraded_providers = degraded_providers;
        status.unhealthy_providers = unhealthy_providers;
        status.total_capabilities = total_capabilities;
        status.active_requests = active_requests.len() as u32;
        status.last_health_check = chrono::Utc::now();

        Ok(())
    }

    /// Shutdown all providers and the manager
    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 Shutting down Universal Ecosystem Manager");

        let mut providers = self.providers.write().await;
        for (provider_key, provider) in providers.iter_mut() {
            info!("🛑 Shutting down provider: {}", provider_key);
            if let Err(e) = provider.shutdown().await {
                error!("Error shutting down provider {}: {}", provider_key, e);
            }
        }
        providers.clear();

        info!("✅ Universal Ecosystem Manager shutdown complete");
        Ok(())
    }
}

/// Information about a registered provider
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    /// Ecosystem identifier
    pub ecosystem_id: String,
    /// Instance identifier
    pub instance_id: String,
    /// Human-readable service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Capabilities offered by this provider
    pub capabilities: Vec<Capability>,
    /// Dependencies required by this provider
    pub dependencies: Vec<Dependency>,
    /// Service endpoints for communication
    pub endpoints: ServiceEndpoints,
    /// Current health status
    pub health_status: HealthStatus,
    /// Provider metadata
    pub metadata: ProviderMetadata,
}

/// Health report for a single provider
#[derive(Debug, Clone)]
pub struct ProviderHealthReport {
    /// Provider key (ecosystem_id:instance_id)
    pub provider_key: String,
    /// Ecosystem identifier
    pub ecosystem_id: String,
    /// Instance identifier
    pub instance_id: String,
    /// Current health status
    pub health_status: HealthStatus,
    /// When the health check was performed
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Overall ecosystem health report
#[derive(Debug, Clone)]
pub struct EcosystemHealthReport {
    /// Overall health status of the ecosystem
    pub overall_health: EcosystemHealthStatus,
    /// Total number of providers
    pub total_providers: u32,
    /// Number of healthy providers
    pub healthy_providers: u32,
    /// Number of degraded providers
    pub degraded_providers: u32,
    /// Number of unhealthy providers
    pub unhealthy_providers: u32,
    /// Individual provider health reports
    pub provider_reports: Vec<ProviderHealthReport>,
    /// When this report was generated
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Overall ecosystem health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemHealthStatus {
    /// All providers are healthy
    Healthy,
    /// Some providers are degraded
    Degraded,
    /// Some providers are unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}
