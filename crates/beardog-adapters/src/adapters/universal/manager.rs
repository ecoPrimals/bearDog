

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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
    pub active_providers: u32,


    pub healthy_providers: u32,


    pub degraded_providers: u32,


    pub unhealthy_providers: u32,

    /// Number of total_capabilities
    pub total_capabilities: u32,
    /// Number of active_requests
    pub active_requests: u32,

    /// The last health check value
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

impl<T: Send + Sync> UniversalEcosystemManager<T> {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(core: Arc<T>) -> Result<Self, BearDogError> {
        info!("🌐 Initializing Universal Ecosystem Manager");
        let capability_registry = Arc::new(CapabilityRegistry::new()?);
        let ecosystem_discovery = Arc::new(&EcosystemDiscovery::new(core)?);
        let manager = Self {
            core,
            providers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            capability_registry,
            ecosystem_discovery,
            active_requests: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            provider_configs: Arc::new(RwLock::new(HashMap::with_capacity(Arc::new(RwLock::new(EcosystemManagerStatus {
                total_providers: 0,
                active_providers: 0,
                healthy_providers: 0,
                degraded_providers: 0,
                unhealthy_providers: 0,
                total_capabilities: 0,
                active_requests: 0,
                last_health_check: chrono::Utc::now(PrimalProvider + 'static>(
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

        provider.initialize(&config)?;

        self.capability_registry
            .register_capabilities(&ecosystem_id, &instance_id, provider.capabilities())
            ?;

        let registration = provider.register_with_ecosystem()?;

        {
            let mut providers = self.providers.write();
            let provider_key = format!("{ecosystem_id}:{instance_id}");
            providers.insert(provider_key, Box::new(provider));
        }
            let mut configs = self.provider_configs.write();
            let config_key = format!("{ecosystem_id}:{instance_id}");
            configs.insert(config_key, config);

        self.update_manager_status()?;
            "✅ Successfully registered PrimalProvider: {} ({})",
        Ok(&str,
        instance_id: &str,
    ) -> Result<(), BearDogError> {
            "🔌 Unregistering PrimalProvider: {} ({})",
        let provider_key = format!("{ecosystem_id}:{instance_id}");

            if let Some({}", provider_key, e);
                }
            }

            configs.remove(&provider_key);

            .unregister_capabilities(ecosystem_id, instance_id)
            "✅ Successfully unregistered PrimalProvider: {} ({})",
        Ok(())

/// Route Request operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn route_request(&self, request: ServiceRequest) -> Result<ServiceResponse, BearDogError> {
        debug!(
            "🔄 Routing request: {} ({})",
            request.request_type, request.id

            let mut active_requests = self.active_requests.write();
            active_requests.insert(request.id.clone(), request.clone());

        let provider_key = self.find_capable_provider(&request)?;

        let response = {
            let providers = self.providers.read();
            if let Some(provider) = providers.get(&provider_key) {
                provider.handle_request(&request)
            } else {
                Err(BearDogError::internal(format!(
                    "Provider {provider_key} not found after capability check"
                )))

            active_requests.remove(&request.id);
        response


    fn find_capable_provider(&self, request: &ServiceRequest) -> Result<String, BearDogError> {
        let providers = self.providers.read();

        for (provider_key, provider) in providers.iter() {
            if provider.can_handle_request(request) {
                return Ok(provider_key);

        if !request.target.is_empty() {

            for (provider_key, _) in providers.iter() {
                if provider_key.starts_with(&request.target) {
                    return Ok(provider_key);

        if request.request_type.starts_with("security.") {

            for (provider_key, provider) in providers.iter() {
                if provider.ecosystem_id() == "beardog " {
        Err(BearDogError::internal({}",
            request.request_type
        )))

/// Get Providers operation.
    /// Gets providers
    /// Gets providers
    pub fn get_providers(&self) -> Vec<ProviderInfo> {
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
                health_status: provider.health_check(),
                metadata: provider.metadata(),
            });
        provider_infos

/// Get Provider operation.
    /// Gets provider
    /// Gets provider
    pub fn get_provider(
    ) -> Option<ProviderInfo> {
        if let Some(provider) = providers.get(&provider_key) {
            Some(ProviderInfo {
            })
        } else {
            None

/// Get All Capabilities operation.
    /// Gets all_capabilities
    /// Gets all_capabilities
    pub fn get_all_capabilities(&self) -> Vec<Capability> {
        let mut all_capabilities = Vec::new(CapabilityCategory,
    ) -> Vec<Capability> {
        let all_capabilities = self.get_all_capabilities();
            .into_iter()
            .filter(|cap| cap.category == category)
            .collect()

/// Health Check All operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check_all(&self) -> Result<EcosystemHealthReport, BearDogError> {
        let mut health_reports = Vec::new();
            let health_status = provider.health_check();
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
        Ok(health_reports,
            timestamp: chrono::Utc::now(),
        })

/// Get Status operation.
    /// Gets status
    /// Gets status
    pub fn get_status(&self) -> EcosystemManagerStatus {
        let status = self.manager_status.read();
        status.clone()

    /// Updates manager_status
    fn update_manager_status(&self) -> Result<(), BearDogError> {
        let active_requests = self.active_requests.read();
        let total_capabilities = self.get_all_capabilities().len() as u32;
        let mut healthy_providers = 0;
        let mut degraded_providers = 0;
        let mut unhealthy_providers = 0;
            match provider.health_check() {
                HealthStatus::Healthy => healthy_providers += 1,
                HealthStatus::Degraded { .. } => degraded_providers += 1,
                HealthStatus::Unhealthy { .. } => unhealthy_providers += 1,
                _ => {}
        let mut status = self.manager_status.write();
        status.total_providers = providers.len() as u32;
        status.active_providers = providers.len() as u32;
        status.healthy_providers = healthy_providers;
        status.degraded_providers = degraded_providers;
        status.unhealthy_providers = unhealthy_providers;
        status.total_capabilities = total_capabilities;
        status.active_requests = active_requests.len() as u32;
        status.last_health_check = chrono::Utc::now({}", provider_key);
            if let Err({}", provider_key, e);
        providers.clear(String,


    pub instance_id: String,

    /// Name of the service
    pub service_name: String,

    /// The service version value
    pub service_version: String,

    /// Collection of capabilities
    pub capabilities: Vec<Capability>,

    /// Collection of dependencies
    pub dependencies: Vec<Dependency>,

    /// The endpoints value
    pub endpoints: ServiceEndpoints,

    /// Current status of the health
    pub health_status: HealthStatus,

    /// The metadata value
    pub metadata: ProviderMetadata,

pub struct ProviderHealthReport {


    pub provider_key: String,

    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,

pub struct EcosystemHealthReport {

    /// The overall health value
    pub overall_health: EcosystemHealthStatus,


    pub provider_reports: Vec<ProviderHealthReport>,


    pub timestamp: chrono::DateTime<chrono::Utc>,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemHealthStatus {


    /// Represents healthy variant
    Healthy,


    /// State indicating degraded
    Degraded,


    /// Represents unhealthy variant
    Unhealthy,


    /// Unknown or undefined state
    Unknown,
}

impl<T> UniversalEcosystemManager<T> {

/// Initialize Ecosystem Discovery operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize_ecosystem_discovery
    /// Initializes componentialize_ecosystem_discovery
    pub fn initialize_ecosystem_discovery(&self) -> Result<(), BearDogError> {
        info!("🔄 Initializing ecosystem discovery with core integration");

        let _core_ref = Arc::clone(&self.core);

        self.ecosystem_discovery
            .start_background_discovery()

            .initialize_default_compatibility_rules()
            ;
        info!("✅ Ecosystem discovery initialized successfully");

/// Get Core operation.
    /// Gets core
    /// Gets core
    pub fn get_core(&self) -> Arc<T> {
        Arc::clone({}", ecosystem_id);

        let services = self
            .ecosystem_discovery
            .discover_ecosystem_services(ecosystem_id)

        for service in &services {
            for capability in &service.capabilities {
                let compatible_ids: Vec<String> =
                    service.capabilities.iter({}",
            services.len(&str,
        capability_b: &str,
    ) -> Result<bool, BearDogError> {

        let compatible = self
            .capability_registry
            .are_capabilities_compatible(&EcosystemRegistration,
        info!("📝 Registering with ecosystem: {}", ecosystem_id);

            .register_with_ecosystem(ecosystem_id, registration)

        for capability in &registration.capabilities {
            let compatible_ids: Vec<String> = registration
                .capabilities
                .iter({}",
