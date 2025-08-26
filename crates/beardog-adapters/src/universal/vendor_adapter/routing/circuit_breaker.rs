

use super::traits::RoutingStrategy;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct CircuitBreakerRouting {
    pub name: String,
    pub config: CircuitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, requests rejected
    HalfOpen, // Testing if service recovered

pub struct CircuitConfig {
    pub failure_threshold: u32,
    pub recovery_timeout_ms: u64,
    pub success_threshold: u32,}

impl Default for CircuitBreakerRouting {}

    fn default() -> Self {
        Self {
            name: "CircuitBreaker".to_string(),
            config: CircuitConfig::default(),
        }
    }
impl Default for CircuitConfig {
            failure_threshold: 5,
            recovery_timeout_ms: 60000,
            success_threshold: 3,}

impl RoutingStrategy for CircuitBreakerRouting {}

    fn strategy_name(&self) -> &str {
        &self.name}

    async fn select_handler(
        &self,
        _request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)],
    ) -> BearDogResult<Option<usize>> {
        if available_handlers.is_empty() {
            return Ok(None);

        Ok(Some(0))
    async fn update_with_result(
        _handler_id: Uuid,
        _success: bool,
        _response_time_ms: u64,
        _error: Option<&str>,
    ) -> BearDogResult<()> {

        Ok(())}

    async fn get_statistics(&self) -> BearDogResult<serde_json::Value> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "config": self.config
        }))

#[must_use] pub fn create_circuit_breaker_routing() -> CircuitBreakerRouting {
    CircuitBreakerRouting::default()
