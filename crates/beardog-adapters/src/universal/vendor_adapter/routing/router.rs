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


/// Universal Request Router Implementation

use beardog_errors::BearDogResult;
use uuid::Uuid;
use crate::universal::vendor_adapter::{
    CapabilityHandler, UniversalVendorRequest, UniversalVendorResponse,
};
use super::{
    circuit_breaker::CircuitBreakerRouting, multi_criteria::MultiCriteriaRouting,
    performance::PerformanceFirstRouting, RoutingStrategy,
/// **UNIVERSAL REQUEST ROUTER** - Routes by capability, not vendor
pub struct UniversalRequestRouter {
    /// Primary routing strategy
    primary_strategy: Box<dyn RoutingStrategy>,
    /// Configuration
    config: RouterConfig,
    /// Router ID
    router_id: Uuid,
}
/// Configuration for the request router
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Default routing strategy
    pub default_strategy: String,
    /// Enable load balancing
    pub enable_load_balancing: bool,
    /// Enable circuit breakers
    pub enable_circuit_breakers: bool,
    /// Circuit breaker failure threshold
    pub circuit_breaker_failure_threshold: u32,
    /// Circuit breaker timeout in seconds
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
    /// Create a new request router
    pub async fn new(config: RouterConfig) -> BearDogResult<Self> {
        let router_id = Uuid::new_v4();
        tracing::info!("🎯 Creating Universal Request Router: {}", router_id);
        // Create the primary routing strategy based on configuration
        let primary_strategy = Self::create_routing_strategy(&config)?;
        Ok(Self {
            primary_strategy,
            config,
            router_id,
        })
    /// Create the appropriate routing strategy based on configuration
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
        // Wrap with circuit breaker if enabled
        let final_strategy = if config.enable_circuit_breakers {
            tracing::info!(
                "🔄 Enabling circuit breaker with {} failure threshold",
                config.circuit_breaker_failure_threshold
            );
            Box::new(CircuitBreakerRouting::default())
        } else {
            base_strategy
        Ok(final_strategy)
    /// Get router ID
    #[must_use] pub const fn router_id(&self) -> Uuid {
        self.router_id
    /// Update routing strategy}


    pub fn update_strategy(&mut self, strategy: Box<dyn RoutingStrategy>) {
        tracing::info!(
            "📋 Updating routing strategy to: {}",
            strategy.strategy_name()
        );
        self.primary_strategy = strategy;
    /// Route a request to the best handler
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
        // Use the primary routing strategy to select the best handler
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
    /// Update routing strategy with execution results
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
    /// Get router statistics
    pub async fn get_statistics(&self) -> BearDogResult<RouterStatistics> {
        let strategy_stats = self.primary_strategy.get_statistics().await?;
        Ok(RouterStatistics {
            router_id: self.router_id,
            primary_strategy: self.primary_strategy.strategy_name().to_string(),
            strategy_statistics: strategy_stats,
            config: self.config.clone(),
/// Router statistics
pub struct RouterStatistics {
    pub router_id: Uuid,
    pub primary_strategy: String,
    pub strategy_statistics: serde_json::Value,
    pub config: RouterConfig,
