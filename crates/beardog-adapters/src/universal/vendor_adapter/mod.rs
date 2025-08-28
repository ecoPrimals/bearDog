

pub mod core;
pub mod discovery;
pub mod handlers;
pub mod monitoring;
pub mod routing;

pub use self::core::{
    CapabilityConfig, CapabilityHandler, CapabilityHealth, CapabilityMetadata, ComplianceProfile,
    CostProfile, PerformanceProfile, QualityProfile, ResourceRequirements, UniversalVendorRequest,
    UniversalVendorResponse,
};
pub use self::discovery::{VendorDiscoveryEngine, VendorDiscoveryStrategy};
pub use self::routing::{RoutingStrategy, UniversalRequestRouter};
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug)]
pub enum CapabilityHandlerType {
    Hsm(HsmCapabilityHandler),
    Network(NetworkCapabilityHandler),
    Storage(StorageCapabilityHandler),
    Compute(ComputeCapabilityHandler),
}

type CapabilityHandlersMap = HashMap<CapabilityType, Vec<CapabilityHandlerType>>;

pub struct UniversalVendorAdapter {

    #[allow(clippy::type_complexity)]
    capability_handlers: Arc<RwLock<CapabilityHandlersMap>>,

    discovery_engine: Arc<VendorDiscoveryEngine>,

    request_router: Arc<UniversalRequestRouter>,

    metrics: Arc<monitoring::VendorMetricsCollector>,

    config: UniversalAdapterConfig,

    adapter_id: Uuid,
}

#[derive(Debug, Clone)]

impl Default for UniversalAdapterConfig {}

    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval_seconds: 300, // 5 minutes
            max_concurrent_operations: 100,
            default_timeout_seconds: 30,
            enable_monitoring: true,
            enable_health_checks: true,
            health_check_interval_seconds: 60, // 1 minute
        }
    }
impl UniversalVendorAdapter {

    pub async fn new(config: UniversalAdapterConfig) -> Result<Self, BearDogError> {
        let adapter_id = Uuid::new_v4();
        info!("🌌 Creating Universal Vendor Adapter: {}", adapter_id);
        let discovery_engine = Arc::new(
            VendorDiscoveryEngine::new(discovery::DiscoveryEngineConfig::default()).await?,
        );
        let request_router =
            Arc::new(UniversalRequestRouter::new(routing::RouterConfig::default()).await?);
        let metrics = Arc::new(monitoring::VendorMetricsCollector::new().await?);
        let adapter = Self {
            capability_handlers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            discovery_engine,
            request_router,
            metrics,
            config,
            adapter_id,
        };

        if adapter.config.auto_discovery {
            adapter.start_auto_discovery().await?;
        info!("✅ Universal Vendor Adapter created successfully");
        Ok(adapter)

    #[must_use] pub const fn adapter_id(&self) -> Uuid {
        self.adapter_id

    pub async fn register_capability_handler<T>(&self, handler: T) -> Result<(), BearDogError>
    where
        T: CapabilityHandler + 'static,
    {
        let capability_type = handler.capability_type();
        let handler_metadata = handler.get_metadata();
        info!(
            "📋 Registering capability handler: {:?} (instance: {})",
            capability_type, handler_metadata.instance_id
        let mut handlers = self.capability_handlers.write().await;
        handlers
            .entry(capability_type.clone())
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
        info!("✅ Capability handler registered: {:?}", capability_type);
        Ok(())

    pub async fn execute_request(
        &self,
        request: UniversalVendorRequest,
    ) -> Result<UniversalVendorResponse, BearDogError> {
        let _start_time = std::time::Instant::now();
            "🎯 Executing universal vendor request: {} (capability: {:?})",
            request.request_id, request.required_capability

        self.metrics.record_request(&request).await;

        let handler_boxes = self
            .get_handlers_for_capability(&request.required_capability)
            .await?;
        if handler_boxes.is_empty() {
            warn!(
                "❌ No handlers available for capability: {:?}",
                request.required_capability
            );
            return Err(BearDogError::configuration(format!(
                    "No capability handlers registered for: {:?)",
                    request.required_capability
                ),
            });

        let handlers: Vec<(Box<dyn CapabilityHandler>, f64)> = handler_boxes.into_iter().collect();

        if let Some(handler_index) = self
            .request_router
            .route_request(&request, &handlers)
            .await?
        {

            let selected_handler = &handlers[handler_index];
            let response = selected_handler.0.execute(request.clone()).await?;
            self.metrics.record_success(&request, &response).await;
            Ok(response)
        } else {
            warn!("⚠️ No suitable handlers found for request");
            Err(BearDogError::internal("No capable handlers found"))

    async fn get_handlers_for_capability(
        _capability: &CapabilityType,
    ) -> Result<Vec<(Box<dyn CapabilityHandler>, BearDogError>, f64)>> {
        let matching_handlers = Vec::new();

        Ok(matching_handlers)

    async fn start_auto_discovery(&self) -> Result<(), BearDogError> {
        info!("🔍 Starting automatic capability discovery");
        let discovery_engine = self.discovery_engine.clone();
        let _capability_handlers = self.capability_handlers.clone();
        let interval = self.config.discovery_interval_seconds;
        tokio::spawn(async move {
            let mut interval_timer =
                tokio::time::interval(std::time::Duration::from_secs(interval));
            loop {
                interval_timer.tick().await;
                match discovery_engine.discover_all_capabilities().await {
                    Ok(discovered_capabilities) => {
                        info!(
                            "🔍 Discovered {} capabilities",
                            discovered_capabilities.len()
                        );

                    }
                    Err(e) => {
                        warn!("⚠️ Discovery failed: {:?}", e);
                }
            }
        });

    pub async fn get_statistics(&self) -> Result<AdapterStatistics, BearDogError> {
        let handlers = self.capability_handlers.read().await;
        let total_handlers: usize = handlers.values().map(|v| v.len()).sum();
        let metrics = self.metrics.get_summary().await?;
        Ok(AdapterStatistics {
            adapter_id: self.adapter_id,
            total_capabilities: handlers.len(),
            total_handlers,
            total_requests: metrics.total_requests,
            successful_requests: metrics.successful_requests,
            failed_requests: metrics.failed_requests,
            average_response_time_ms: metrics.average_response_time_ms,
        })

pub struct AdapterStatistics {
    pub adapter_id: Uuid,
    pub total_capabilities: usize,
    pub total_handlers: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
