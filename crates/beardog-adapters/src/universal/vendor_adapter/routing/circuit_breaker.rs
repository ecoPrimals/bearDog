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


/// Circuit Breaker Routing Strategies
///
/// **EXTRACTED FROM**: strategies.rs (950 lines → focused module)
/// Contains routing strategies with circuit breaker patterns for fault tolerance.

use super::traits::RoutingStrategy;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use async_trait::async_trait;
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// **CIRCUIT BREAKER ROUTING** - Fault-tolerant routing with circuit breakers
#[derive(Debug)]
pub struct CircuitBreakerRouting {
    pub name: String,
    pub config: CircuitConfig,
}
/// Circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, requests rejected
    HalfOpen, // Testing if service recovered
/// Circuit breaker configuration}


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
        // Circuit breaker logic would be implemented here
        // This would track handler failures and open circuits when failure thresholds are exceeded
        // For now, select the first available handler
        Ok(Some(0))
    async fn update_with_result(
        _handler_id: Uuid,
        _success: bool,
        _response_time_ms: u64,
        _error: Option<&str>,
    ) -> BearDogResult<()> {
        // Circuit state update logic would be implemented here
        // This would track success/failure rates and update circuit states accordingly
        Ok(())}


    async fn get_statistics(&self) -> BearDogResult<serde_json::Value> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "config": self.config
        }))
/// Create a circuit breaker routing strategy
#[must_use] pub fn create_circuit_breaker_routing() -> CircuitBreakerRouting {
    CircuitBreakerRouting::default()
