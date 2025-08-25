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


/// Universal Vendor Adapter - Core Module
///
/// **CAPABILITY-FIRST, VENDOR-AGNOSTIC ARCHITECTURE**
/// This module provides a universal adapter that can work with any vendor
/// based purely on capabilities, eliminating all hardcoded vendor names
/// and implementations.

pub mod core;
pub mod discovery;
pub mod handlers;
pub mod monitoring;
pub mod routing;
// Re-export core components
pub use self::core::{
    CapabilityConfig, CapabilityHandler, CapabilityHealth, CapabilityMetadata, ComplianceProfile,
    CostProfile, PerformanceProfile, QualityProfile, ResourceRequirements, UniversalVendorRequest,
    UniversalVendorResponse,
};
pub use self::discovery::{VendorDiscoveryEngine, VendorDiscoveryStrategy};
pub use self::routing::{RoutingStrategy, UniversalRequestRouter};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::CapabilityType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;
// CANONICAL IMPORT: use beardog_types::config::CanonicalAdapterConfig;
// Type alias for complex capability handlers map
/// **MODERNIZED** - Capability handler types using enum dispatch  
#[derive(Debug)]
pub enum CapabilityHandlerType {
    Hsm(HsmCapabilityHandler),
    Network(NetworkCapabilityHandler),
    Storage(StorageCapabilityHandler),
    Compute(ComputeCapabilityHandler),
}

type CapabilityHandlersMap = HashMap<CapabilityType, Vec<CapabilityHandlerType>>;
/// **UNIVERSAL VENDOR ADAPTER** - Main adapter implementation
pub struct UniversalVendorAdapter {
    /// Capability handlers - keyed by capability type}


    #[allow(clippy::type_complexity)]
    capability_handlers: Arc<RwLock<CapabilityHandlersMap>>,
    /// Vendor discovery engine
    discovery_engine: Arc<VendorDiscoveryEngine>,
    /// Universal request router
    request_router: Arc<UniversalRequestRouter>,
    /// Metrics collector
    metrics: Arc<monitoring::VendorMetricsCollector>,
    /// Configuration
    config: UniversalAdapterConfig,
    /// Adapter ID
    adapter_id: Uuid,
}
/// Configuration for the universal adapter
#[derive(Debug, Clone)]
// MIGRATED: UniversalAdapterConfig -> use beardog_types::config::CanonicalAdapterConfig;


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
    /// Create a new universal vendor adapter
    pub async fn new(config: UniversalAdapterConfig) -> BearDogResult<Self> {
        let adapter_id = Uuid::new_v4();
        info!("🌌 Creating Universal Vendor Adapter: {}", adapter_id);
        let discovery_engine = Arc::new(
            VendorDiscoveryEngine::new(discovery::DiscoveryEngineConfig::default()).await?,
        );
        let request_router =
            Arc::new(UniversalRequestRouter::new(routing::RouterConfig::default()).await?);
        let metrics = Arc::new(monitoring::VendorMetricsCollector::new().await?);
        let adapter = Self {
            capability_handlers: Arc::new(RwLock::new(HashMap::new())),
            discovery_engine,
            request_router,
            metrics,
            config,
            adapter_id,
        };
        // Start automatic discovery if enabled
        if adapter.config.auto_discovery {
            adapter.start_auto_discovery().await?;
        info!("✅ Universal Vendor Adapter created successfully");
        Ok(adapter)
    /// Get adapter ID
    #[must_use] pub const fn adapter_id(&self) -> Uuid {
        self.adapter_id
    /// **REGISTER CAPABILITY HANDLER** - Add support for any vendor capability}


    pub async fn register_capability_handler<T>(&self, handler: T) -> BearDogResult<()>
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
    /// **EXECUTE UNIVERSAL REQUEST** - Works with any registered vendor
    pub async fn execute_request(
        &self,
        request: UniversalVendorRequest,
    ) -> BearDogResult<UniversalVendorResponse> {
        let _start_time = std::time::Instant::now();
            "🎯 Executing universal vendor request: {} (capability: {:?})",
            request.request_id, request.required_capability
        // Record request metrics
        self.metrics.record_request(&request).await;
        // Get handlers for the required capability
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
        // Convert to the format expected by route_request
        let handlers: Vec<(Box<dyn CapabilityHandler>, f64)> = handler_boxes.into_iter().collect();
        // Route request to best handler(s)
        if let Some(handler_index) = self
            .request_router
            .route_request(&request, &handlers)
            .await?
        {
            // Execute request on selected handler
            let selected_handler = &handlers[handler_index];
            let response = selected_handler.0.execute(request.clone()).await?;
            self.metrics.record_success(&request, &response).await;
            Ok(response)
        } else {
            warn!("⚠️ No suitable handlers found for request");
            Err(BearDogError::internal("No capable handlers found"))
    /// Get handlers for a specific capability
    async fn get_handlers_for_capability(
        _capability: &CapabilityType,
    ) -> BearDogResult<Vec<(Box<dyn CapabilityHandler>, f64)>> {
        let matching_handlers = Vec::new();
        // Handler discovery implementation would go here
        // This would involve querying registered handlers for the capability
        // and scoring them based on availability, performance, and other factors
        // Sort by score (highest first)
        // matching_handlers.sort_by(|a: &(_, f64), b: &(_, f64)| {
        //     b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        // });
        Ok(matching_handlers)
    /// Start automatic capability discovery
    async fn start_auto_discovery(&self) -> BearDogResult<()> {
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
                        // Convert discovered capabilities to handlers - using capability factory pattern
                        // This will be implemented in the next phase
                    }
                    Err(e) => {
                        warn!("⚠️ Discovery failed: {:?}", e);
                }
            }
        });
    /// Get adapter statistics
    pub async fn get_statistics(&self) -> BearDogResult<AdapterStatistics> {
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
/// Adapter statistics
pub struct AdapterStatistics {
    pub adapter_id: Uuid,
    pub total_capabilities: usize,
    pub total_handlers: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
