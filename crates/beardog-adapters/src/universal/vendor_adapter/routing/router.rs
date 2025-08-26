

use beardog_errors::BearDogResult;
use uuid::Uuid;
use crate::universal::vendor_adapter::{
    CapabilityHandler, UniversalVendorRequest, UniversalVendorResponse,
};
use super::{
    circuit_breaker::CircuitBreakerRouting, multi_criteria::MultiCriteriaRouting,
    performance::PerformanceFirstRouting, RoutingStrategy,

pub struct UniversalRequestRouter {

    primary_strategy: Box<dyn RoutingStrategy>,

    config: RouterConfig,

    router_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct RouterConfig {

    pub default_strategy: String,

    pub enable_load_balancing: bool,

    pub enable_circuit_breakers: bool,

    pub circuit_breaker_failure_threshold: u32,

    pub circuit_breaker_timeout_seconds: u64,}

impl Default for RouterConfig {}

    fn default() -> Self {
        Self {
            default_strategy: "performance".to_string(),
            enable_load_balancing: true,
            enable_circuit_breakers: true,
            circuit_breaker_failure_threshold: 5,
            circuit_breaker_timeout_seconds: 60,
        }
    }
impl UniversalRequestRouter {

    pub async fn new(config: RouterConfig) -> BearDogResult<Self> {
        let router_id = Uuid::new_v4();
        tracing::info!("🎯 Creating Universal Request Router: {}", router_id);

        let primary_strategy = Self::create_routing_strategy(&config)?;
        Ok(Self {
            primary_strategy,
            config,
            router_id,
        })

    fn create_routing_strategy(config: &RouterConfig) -> BearDogResult<Box<dyn RoutingStrategy>> {
        let base_strategy: Box<dyn RoutingStrategy> = match config.default_strategy.as_str() {
            "performance" => Box::new(PerformanceFirstRouting::new("performance".to_string())),
            "multi_criteria" => Box::new(MultiCriteriaRouting::default()),
            "load_balancing_round_robin" => {
                Box::new(PerformanceFirstRouting::new("round_robin".to_string()))
            }
            "load_balancing_weighted" => {
                Box::new(PerformanceFirstRouting::new("weighted".to_string()))
            "load_balancing_least_connections" => Box::new(PerformanceFirstRouting::new(
                "least_connections".to_string(),
            )),
            _ => {
                tracing::warn!(
                    "Unknown routing strategy '{}', defaulting to performance",
                    config.default_strategy
                );
                Box::new(PerformanceFirstRouting::new(
                    "default-performance".to_string(),
                ))
        };

        let final_strategy = if config.enable_circuit_breakers {
            tracing::info!(
                "🔄 Enabling circuit breaker with {} failure threshold",
                config.circuit_breaker_failure_threshold
            );
            Box::new(CircuitBreakerRouting::default())
        } else {
            base_strategy
        Ok(final_strategy)

    #[must_use] pub const fn router_id(&self) -> Uuid {
        self.router_id

    pub fn update_strategy(&mut self, strategy: Box<dyn RoutingStrategy>) {
        tracing::info!(
            "📋 Updating routing strategy to: {}",
            strategy.strategy_name()
        );
        self.primary_strategy = strategy;

    pub async fn route_request(
        &self,
        request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)], // (handler, confidence)
    ) -> BearDogResult<Option<usize>> {
        if available_handlers.is_empty() {
            tracing::debug!(
                "🎯 No handlers available for request {}",
                request.request_id
            return Ok(None);
        tracing::debug!(
            "🎯 Routing request {} using strategy '{}' with {} available handlers",
            request.request_id,
            self.primary_strategy.strategy_name(),
            available_handlers.len()

        let selected_index = self
            .primary_strategy
            .select_handler(request, available_handlers)
            .await?;
        if let Some(index) = selected_index {
            let (handler, confidence) = &available_handlers[index];
                "🎯 Selected handler {} with confidence {:.2} for request {}",
                handler.get_metadata().handler_name,
                confidence,
            tracing::warn!(
                "🎯 No suitable handler found for request {}",
        Ok(selected_index)

    pub async fn update_with_result(
        handler_id: Uuid,
        response: Result<&UniversalVendorResponse, &str>,
    ) -> BearDogResult<()> {
        let (success, response_time_ms, error) = match response {
            Ok(resp) => (true, resp.performance.processing_time_ms, None),
            Err(err) => (false, 0, Some(err)),
        self.primary_strategy
            .update_with_result(handler_id, request, success, response_time_ms, error)
            "🎯 Updated routing strategy with result: success={}, time={}ms, handler={}",
            success,
            response_time_ms,
            handler_id
        Ok(())

    pub async fn get_statistics(&self) -> BearDogResult<RouterStatistics> {
        let strategy_stats = self.primary_strategy.get_statistics().await?;
        Ok(RouterStatistics {
            router_id: self.router_id,
            primary_strategy: self.primary_strategy.strategy_name().to_string(),
            strategy_statistics: strategy_stats,
            config: self.config.clone(),

pub struct RouterStatistics {
    pub router_id: Uuid,
    pub primary_strategy: String,
    pub strategy_statistics: serde_json::Value,
    pub config: RouterConfig,
