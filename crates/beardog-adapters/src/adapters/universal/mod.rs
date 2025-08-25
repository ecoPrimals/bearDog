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


/// Universal External System Adapter
///
/// **Agnostic adapter for ALL external (non-ecoPrimals) systems**
/// This module provides a universal adapter pattern that treats all external systems
/// (Kubernetes, Prometheus, Grafana, AWS, Azure, Splunk, etc.) as extensions through
/// a consistent, agnostic interface.
/// ## Architecture Principles
/// - **ecoPrimals First-Class**: Direct integration (SongBird, NestGate, ToadStool, Squirrel, biomeOS)
/// - **External as Extensions**: All others go through this universal adapter
/// - **Protocol Agnostic**: HTTP, gRPC, WebSocket, TCP, UDP support
/// - **Authentication Agnostic**: API keys, OAuth, certificates, tokens
/// - **Format Agnostic**: JSON, XML, YAML, binary, custom protocols
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};
// CANONICAL IMPORT: use beardog_types::config::CanonicalAdapterConfig;

pub mod authentication;
pub mod protocols;
pub mod providers;
pub mod transformers;
pub use authentication::*;
pub use protocols::*;
pub use providers::*;
pub use transformers::*;
/// Universal adapter for external systems
pub struct UniversalExternalAdapter {
    providers: HashMap<String, Box<dyn ExternalSystemProvider>>,
    config: UniversalAdapterConfig,
}
/// Configuration for universal external adapter
#[derive(Debug, Clone)]
// MIGRATED: UniversalAdapterConfig -> use beardog_types::config::CanonicalAdapterConfig;


impl Default for UniversalAdapterConfig {}


    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            retry_attempts: 3,
            retry_backoff_ms: 1000,
            max_concurrent_connections: 100,
            enable_circuit_breaker: true,
            circuit_breaker_failure_threshold: 5,
            circuit_breaker_reset_timeout_seconds: 60,
        }
    }
/// Request to external system through universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub system_id: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub timeout_override: Option<u64>,
/// Response from external system through universal adapter
pub struct UniversalResponse {
    pub success: bool,
    pub processing_time_ms: u64,
/// External system provider trait - all external integrations implement this
// Uses native async fn from trait definition - no async_trait needed
pub trait ExternalSystemProvider: Send + Sync {
    /// System identifier (e.g., "kubernetes", "prometheus", "aws_kms")}


    fn system_id(&self) -> &str;
    /// Protocol used by this system
    fn protocol(&self) -> &dyn Protocol;
    /// Authentication method used
    fn authentication(&self) -> &dyn Authentication;
    /// Execute operation on external system
    async fn execute(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value>;
    /// Health check for external system
    async fn health_check(&self) -> BearDogResult<bool>;
    /// Get system capabilities
    fn capabilities(&self) -> Vec<String>;}


impl UniversalExternalAdapter {
    /// Create new universal external adapter}


    pub fn new(config: UniversalAdapterConfig) -> Self {
            providers: HashMap::new(),
            config,
    /// Register an external system provider}


    pub fn register_provider(&mut self, provider: Box<dyn ExternalSystemProvider>) {
        let system_id = provider.system_id().to_string();
        info!("🔌 Registering external system provider: {}", system_id);
        self.providers.insert(system_id, provider);
    /// Execute request on external system
    pub async fn execute(&self, request: UniversalRequest) -> BearDogResult<UniversalResponse> {
        let start_time = std::time::Instant::now();
        debug!(
            "🌐 Universal adapter executing: {} -> {}",
            request.system_id, request.operation
        );
        let provider = self.providers.get(&request.system_id).ok_or_else(|| {
            BearDogError::configuration(format!("External system provider not found: {}", request.system_id))
        })?;
        // Apply timeout override if provided
        let timeout = Duration::from_secs(
            request
                .timeout_override
                .unwrap_or(self.config.timeout_seconds),
        // Execute with retry logic
        let result = self
            .execute_with_retry(provider.as_ref(), &request, timeout)
            .await;
        let processing_time = start_time.elapsed().as_millis() as u64;
        match result {
            Ok(payload) => Ok(UniversalResponse {
                success: true,
                payload,
                metadata: HashMap::new(),
                processing_time_ms: processing_time,
                system_id: request.system_id,
                operation: request.operation,
            }),
            Err(e) => {
                error!("🚨 Universal adapter execution failed: {}", e);
                Ok(UniversalResponse {
                    success: false,
                    payload: serde_json::json!({
                        "error": e.to_string(),
                        "error_type": "external_system_error"
                    }),
                    metadata: HashMap::new(),
                    processing_time_ms: processing_time,
                    system_id: request.system_id,
                    operation: request.operation,
                })
            }
    /// Execute with retry logic and circuit breaker
    async fn execute_with_retry(
        provider: &dyn ExternalSystemProvider,
        request: &UniversalRequest,
        timeout: Duration,
    ) -> BearDogResult<serde_json::Value> {
        let mut last_error = None;
        for attempt in 1..=self.config.retry_attempts {
            match tokio::time::timeout(
                timeout,
                provider.execute(&request.operation, request.payload.clone()),
            )
            .await
            {
                Ok(Ok(result)) => {
                    if attempt > 1 {
                        info!("✅ External system recovered on attempt {}", attempt);
                    }
                    return Ok(result);
                }
                Ok(Err(e)) => {
                    warn!("⚠️ External system attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                Err(_) => {
                    let timeout_error = BearDogError::network(format!("External system timeout after {}s", timeout.as_secs()));
                    warn!("⏰ External system timeout on attempt {}", attempt);
                    last_error = Some(timeout_error);
            // Wait before retry (except on last attempt)
            if attempt < self.config.retry_attempts {
                tokio::time::sleep(Duration::from_millis(
                    self.config.retry_backoff_ms * attempt as u64,
                ))
                .await;
        // All attempts failed
        Err(
            last_error.unwrap_or_else(|| BearDogError::internal(format!("All retry attempts failed for service: {}", request.system_id))),
        )
    /// Get health status of all registered external systems
    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        for (system_id, provider) in &self.providers {
            match provider.health_check().await {
                Ok(healthy) => {
                    results.insert(system_id.clone(), healthy);
                Err(e) => {
                    warn!("🏥 Health check failed for {}: {}", system_id, e);
                    results.insert(system_id.clone(), false);
        results
    /// Get all registered external systems and their capabilities
    pub fn get_systems_info(&self) -> HashMap<String, Vec<String>> {
        self.providers
            .iter()
            .map(|(id, provider)| (id.clone(), provider.capabilities()))
            .collect()
    /// Remove external system provider}


    pub fn unregister_provider(&mut self, system_id: &str) -> bool {
        match self.providers.remove(system_id) {
            Some(_) => {
                info!("🔌 Unregistered external system provider: {}", system_id);
                true
            None => {
                warn!("⚠️ Attempted to unregister unknown provider: {}", system_id);
                false
