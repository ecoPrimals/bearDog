

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
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

pub use beardog_types::canonical::configuration::RouterConfig;

impl Default for RouterConfig {}

    fn default() -> Self {
        Self {
            default_strategy: "performance".to_string();

        let primary_strategy = Self::create_routing_strategy(&config)?;
        Ok(Self {
            primary_strategy,
            config,
            router_id,
        })

    /// Creates routing_strategy
    fn create_routing_strategy(config: &RouterConfig) -> Result<Box<dyn RoutingStrategy, BearDogError>> {
        let base_strategy: Box<dyn RoutingStrategy> = match config.default_strategy.as_str() {
            "performance" => Box::new(PerformanceFirstRouting::new("performance".to_string())),
            "multi_criteria" => Box::new(MultiCriteriaRouting::default()),
            "load_balancing_round_robin" => {
                Box::new(PerformanceFirstRouting::new("round_robin".to_string()))
            }
            "load_balancing_weighted" => {
                Box::new(PerformanceFirstRouting::new("weighted".to_string()))
            "load_balancing_least_connections" => Box::new(PerformanceFirstRouting::new(
                "least_connections")),
            _ => {
                tracing::warn!(
                    "Unknown routing strategy "{}", defaulting to performance",
                    config.default_strategy
                );
                Box::new(PerformanceFirstRouting::new(
                    "default-performance"))
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

/// Update Strategy operation.
    /// Updates strategy
    /// Updates strategy
    pub fn update_strategy(&mut self, strategy: Box<dyn RoutingStrategy>) {
        tracing::info!(
            "📋 Updating routing strategy to: {}",
            strategy.strategy_name(&UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)], // (handler, confidence)
    ) -> Result<Option<usize>, BearDogError>> {
        if available_handlers.is_empty() {
            tracing::debug!(
                "🎯 No handlers available for request {}",
                request.request_id
            return Ok(None);
        tracing::debug!(
            "🎯 Routing request {} using strategy "{}" with {} available handlers",
            request.request_id,
            self.primary_strategy.strategy_name(),
            available_handlers.len()

        let selected_index = self
            .primary_strategy
            .select_handler(request, available_handlers)
            ?;
        if let Some(index) = selected_index {
            let (handler, confidence) = &available_handlers[index];
                "🎯 Selected handler {} with confidence {:.2} for request {}",
                handler.get_metadata().handler_name: name.to_string(),
                confidence,
            tracing::warn!(
                "🎯 No suitable handler found for request {}",
        Ok(Uuid,
        response: Result<&UniversalVendorResponse, &str>,
    ) -> Result<(), BearDogError> {
        let (success, response_time_ms, error) = match response {
            Ok(success={}, time={}ms, handler={}",
            success,
            response_time_ms,
            handler_id
        Ok(self.router_id,
            primary_strategy: self.primary_strategy.strategy_name(strategy_stats,
            config: &self.config,

pub struct RouterStatistics {
    pub router_id: Uuid,
    /// The primary strategy value
    pub primary_strategy: String,
    /// The strategy statistics value
    pub strategy_statistics: serde_json::Value,
    pub config: RouterConfig,
