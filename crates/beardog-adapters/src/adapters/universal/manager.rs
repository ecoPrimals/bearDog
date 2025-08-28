

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use super::discovery::{EcosystemDiscovery, EcosystemService};
use super::registry::CapabilityRegistry;
use super::traits::*;
use beardog_errors::BearDogError;

pub struct UniversalEcosystemManager<T> {

    core: Arc<T>,

    providers: Arc<RwLock<HashMap<String, Box<dyn PrimalProvider>>>>,

    capability_registry: Arc<CapabilityRegistry>,

    ecosystem_discovery: Arc<EcosystemDiscovery<T>>,

    active_requests: Arc<RwLock<HashMap<String, ServiceRequest>>>,

    provider_configs: Arc<RwLock<HashMap<String, ProviderConfig>>>,

    manager_status: Arc<RwLock<EcosystemManagerStatus>>,
}

#[derive(Debug, Clone)]
pub struct EcosystemManagerStatus {

    pub total_providers: u32,

    pub active_providers: u32,

    pub healthy_providers: u32,

    pub degraded_providers: u32,

    pub unhealthy_providers: u32,

    pub total_capabilities: u32,
    pub active_requests: u32,

    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

impl<T: Send + Sync> UniversalEcosystemManager<T> {

    pub async fn new(core: Arc<T>) -> Result<Self, BearDogError> {
        info!("🌐 Initializing Universal Ecosystem Manager");
        let capability_registry = Arc::new(CapabilityRegistry::new().await?);
        let ecosystem_discovery = Arc::new(EcosystemDiscovery::new(core.clone()).await?);
        let manager = Self {
            core,
            providers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            capability_registry,
            ecosystem_discovery,
            active_requests: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            provider_configs: Arc::new(RwLock::new(HashMap::with_capacity(16))),
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

    pub async fn register_provider<P: PrimalProvider + 'static>(
        &self,
        mut provider: P,
        config: ProviderConfig,
    ) -> Result<EcosystemRegistration, BearDogError> {
        let ecosystem_id = provider.ecosystem_id().to_string();
        let instance_id = provider.instance_id().to_string();
        info!(
            "🔌 Registering PrimalProvider: {} ({})",
            ecosystem_id, instance_id
        );

        provider.initialize(config.clone()).await?;

        self.capability_registry
            .register_capabilities(&ecosystem_id, &instance_id, provider.capabilities())
            .await?;

        let registration = provider.register_with_ecosystem().await?;

        {
            let mut providers = self.providers.write().await;
            let provider_key = format!("{ecosystem_id}:{instance_id}");
            providers.insert(provider_key, Box::new(provider));
        }
            let mut configs = self.provider_configs.write().await;
            let config_key = format!("{ecosystem_id}:{instance_id}");
            configs.insert(config_key, config);

        self.update_manager_status().await?;
            "✅ Successfully registered PrimalProvider: {} ({})",
        Ok(registration)

    pub async fn unregister_provider(
        ecosystem_id: &str,
        instance_id: &str,
    ) -> Result<(), BearDogError> {
            "🔌 Unregistering PrimalProvider: {} ({})",
        let provider_key = format!("{ecosystem_id}:{instance_id}");

            if let Some(mut provider) = providers.remove(&provider_key) {
                if let Err(e) = provider.shutdown().await {
                    error!("Error shutting down provider {}: {}", provider_key, e);
                }
            }

            configs.remove(&provider_key);

            .unregister_capabilities(ecosystem_id, instance_id)
            "✅ Successfully unregistered PrimalProvider: {} ({})",
        Ok(())

    pub async fn route_request(&self, request: ServiceRequest) -> Result<ServiceResponse, BearDogError> {
        debug!(
            "🔄 Routing request: {} ({})",
            request.request_type, request.id

            let mut active_requests = self.active_requests.write().await;
            active_requests.insert(request.id.clone(), request.clone());

        let provider_key = self.find_capable_provider(&request).await?;

        let response = {
            let providers = self.providers.read().await;
            if let Some(provider) = providers.get(&provider_key) {
                provider.handle_request(request.clone()).await
            } else {
                Err(BearDogError::internal(format!(
                    "Provider {provider_key} not found after capability check"
                )))

            active_requests.remove(&request.id);
        response

    async fn find_capable_provider(&self, request: &ServiceRequest) -> Result<String, BearDogError> {
        let providers = self.providers.read().await;

        for (provider_key, provider) in providers.iter() {
            if provider.can_handle_request(request) {
                return Ok(provider_key.clone());

        if !request.target.is_empty() {

            for (provider_key, _) in providers.iter() {
                if provider_key.starts_with(&request.target) {
                    return Ok(provider_key.clone());

        if request.request_type.starts_with("security.") {

            for (provider_key, provider) in providers.iter() {
                if provider.ecosystem_id() == "beardog" {
        Err(BearDogError::internal(format!(
            "No capable provider found for request type: {}",
            request.request_type
        )))

    pub async fn get_providers(&self) -> Vec<ProviderInfo> {
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
        provider_infos

    pub async fn get_provider(
    ) -> Option<ProviderInfo> {
        if let Some(provider) = providers.get(&provider_key) {
            Some(ProviderInfo {
            })
        } else {
            None

    pub async fn get_all_capabilities(&self) -> Vec<Capability> {
        let mut all_capabilities = Vec::new();
            all_capabilities.extend(provider.capabilities());
        all_capabilities

    pub async fn get_capabilities_by_category(
        category: CapabilityCategory,
    ) -> Vec<Capability> {
        let all_capabilities = self.get_all_capabilities().await;
            .into_iter()
            .filter(|cap| cap.category == category)
            .collect()

    pub async fn health_check_all(&self) -> Result<EcosystemHealthReport, BearDogError> {
        let mut health_reports = Vec::new();
            let health_status = provider.health_check().await;
            health_reports.push(ProviderHealthReport {
                provider_key: provider_key.clone(),
                health_status,
                last_check: chrono::Utc::now(),

        let total_providers = health_reports.len() as u32;
        let healthy_providers = health_reports
            .iter()
            .filter(|report| matches!(report.health_status, HealthStatus::Healthy))
            .count() as u32;
        let degraded_providers = health_reports
            .filter(|report| matches!(report.health_status, HealthStatus::Degraded { .. }))
        let unhealthy_providers = health_reports
            .filter(|report| matches!(report.health_status, HealthStatus::Unhealthy { .. }))
        let overall_health = if unhealthy_providers > 0 {
            EcosystemHealthStatus::Unhealthy
        } else if degraded_providers > 0 {
            EcosystemHealthStatus::Degraded
            EcosystemHealthStatus::Healthy
        Ok(EcosystemHealthReport {
            overall_health,
            total_providers,
            healthy_providers,
            degraded_providers,
            unhealthy_providers,
            provider_reports: health_reports,
            timestamp: chrono::Utc::now(),
        })

    pub async fn get_status(&self) -> EcosystemManagerStatus {
        let status = self.manager_status.read().await;
        status.clone()

    async fn update_manager_status(&self) -> Result<(), BearDogError> {
        let active_requests = self.active_requests.read().await;
        let total_capabilities = self.get_all_capabilities().await.len() as u32;
        let mut healthy_providers = 0;
        let mut degraded_providers = 0;
        let mut unhealthy_providers = 0;
            match provider.health_check().await {
                HealthStatus::Healthy => healthy_providers += 1,
                HealthStatus::Degraded { .. } => degraded_providers += 1,
                HealthStatus::Unhealthy { .. } => unhealthy_providers += 1,
                _ => {}
        let mut status = self.manager_status.write().await;
        status.total_providers = providers.len() as u32;
        status.active_providers = providers.len() as u32;
        status.healthy_providers = healthy_providers;
        status.degraded_providers = degraded_providers;
        status.unhealthy_providers = unhealthy_providers;
        status.total_capabilities = total_capabilities;
        status.active_requests = active_requests.len() as u32;
        status.last_health_check = chrono::Utc::now();

    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        info!("🛑 Shutting down Universal Ecosystem Manager");
        let mut providers = self.providers.write().await;
        for (provider_key, provider) in providers.iter_mut() {
            info!("🛑 Shutting down provider: {}", provider_key);
            if let Err(e) = provider.shutdown().await {
                error!("Error shutting down provider {}: {}", provider_key, e);
        providers.clear();
        info!("✅ Universal Ecosystem Manager shutdown complete");

pub struct ProviderInfo {

    pub ecosystem_id: String,

    pub instance_id: String,

    pub service_name: String,

    pub service_version: String,

    pub capabilities: Vec<Capability>,

    pub dependencies: Vec<Dependency>,

    pub endpoints: ServiceEndpoints,

    pub health_status: HealthStatus,

    pub metadata: ProviderMetadata,

pub struct ProviderHealthReport {

    pub provider_key: String,

    pub last_check: chrono::DateTime<chrono::Utc>,

pub struct EcosystemHealthReport {

    pub overall_health: EcosystemHealthStatus,

    pub provider_reports: Vec<ProviderHealthReport>,

    pub timestamp: chrono::DateTime<chrono::Utc>,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemHealthStatus {

    Healthy,

    Degraded,

    Unhealthy,

    Unknown,
}

impl<T> UniversalEcosystemManager<T> {

    pub async fn initialize_ecosystem_discovery(&self) -> Result<(), BearDogError> {
        info!("🔄 Initializing ecosystem discovery with core integration");

        let _core_ref = Arc::clone(&self.core);

        self.ecosystem_discovery
            .start_background_discovery()

            .initialize_default_compatibility_rules()
            .await;
        info!("✅ Ecosystem discovery initialized successfully");

    pub fn get_core(&self) -> Arc<T> {
        Arc::clone(&self.core)

    pub async fn discover_ecosystem_services(
    ) -> Result<Vec<EcosystemService>, BearDogError>> {
        info!("🔍 Discovering services for ecosystem: {}", ecosystem_id);

        let services = self
            .ecosystem_discovery
            .discover_ecosystem_services(ecosystem_id)

        for service in &services {
            for capability in &service.capabilities {
                let compatible_ids: Vec<String> =
                    service.capabilities.iter().map(|c| c.id.clone()).collect();

                self.capability_registry
                    .add_compatibility_rule(capability.id.clone(), compatible_ids)
                    .await;
            "✅ Discovered {} services for ecosystem: {}",
            services.len(),
            ecosystem_id
        Ok(services)

    pub async fn check_capability_compatibility(
        capability_a: &str,
        capability_b: &str,
    ) -> Result<bool, BearDogError> {

        let compatible = self
            .capability_registry
            .are_capabilities_compatible(capability_a, capability_b)
        if compatible {
            info!(
                "✅ Capabilities '{}' and '{}' are compatible",
                capability_a, capability_b
            );
                "❌ Capabilities '{}' and '{}' are not compatible",
        Ok(compatible)

    pub async fn register_with_ecosystem(
        registration: &EcosystemRegistration,
        info!("📝 Registering with ecosystem: {}", ecosystem_id);

            .register_with_ecosystem(ecosystem_id, registration)

        for capability in &registration.capabilities {
            let compatible_ids: Vec<String> = registration
                .capabilities
                .iter()
                .map(|c| c.id.clone())
                .collect();
            self.capability_registry
                .add_compatibility_rule(capability.id.clone(), compatible_ids)
                .await;
            "✅ Successfully registered with ecosystem: {}",
