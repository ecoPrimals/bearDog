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


/// Universal Adapter Factory
///
/// **Dynamic adapter creation for any primal type**
/// This module provides a universal adapter factory that can create adapters
/// for any primal type dynamically, eliminating the need for specific adapter
/// implementations like SongBirdAdapter or NestGateAdapter.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::primal_registry::{global_registry, PrimalId, PrimalRegistration};
use super::traits::*;
use beardog_errors::{BearDogError, BearDogResult};
/// Universal adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
// MIGRATED: UniversalAdapterConfig -> use beardog_types::config::CanonicalAdapterConfig;
/// Authentication configuration for universal adapters
// MIGRATED: AuthConfig -> use beardog_types::config::UnifiedSecurityConfig;


pub struct UniversalAdapter {
    /// Configuration
    config: UniversalAdapterConfig,
    /// Target primal registration
    target_primal: Arc<RwLock<Option<PrimalRegistration>>>,
    /// HTTP client for communication
    client: reqwest::Client,
    /// Connection state
    connection_state: Arc<RwLock<ConnectionState>>,
/// Connection state for universal adapters
#[derive(Debug, Clone)]
pub struct ConnectionState {
    /// Whether the adapter is connected
    pub connected: bool,
    /// Last successful connection timestamp
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,
    /// Last error encountered
    pub last_error: Option<String>,
    /// Connection metrics
    pub metrics: ConnectionMetrics,
/// Connection metrics
#[derive(Debug, Clone, Default)]
pub struct ConnectionMetrics {
    /// Total requests made
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: u64,}


impl UniversalAdapter {
    /// Create a new universal adapter for any primal type
    pub async fn new(config: UniversalAdapterConfig) -> BearDogResult<Self> {
        // Create HTTP client with timeout
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {e)"),
            })?;
        let connection_state = Arc::new(RwLock::new(ConnectionState {
            connected: false,
            last_connected: None,
            last_error: None,
            metrics: ConnectionMetrics::default(),
        }));
        Ok(Self {
            config,
            target_primal: Arc::new(RwLock::new(None)),
            client,
            connection_state,
        })
    }
    /// Connect to the target primal
    pub async fn connect(&self) -> BearDogResult<()> {
        // Discover the target primal
        let registry = global_registry().await;
        let primal_registration = registry
            .discover_primal(&self.config.target_primal.id)
            .await?;
        if let Some(registration) = primal_registration {
            // Update the target primal
            {
                let mut target_primal = self.target_primal.write().await;
                *target_primal = Some(registration.clone());
            }
            // Test connectivity
            let health_endpoint = format!("{}/health", registration.discovery_endpoint);
            let response = self.client.get(&health_endpoint).send().await;
            match response {
                Ok(resp) if resp.status().is_success() => {
                    let mut state = self.connection_state.write().await;
                    state.connected = true;
                    state.last_connected = Some(chrono::Utc::now());
                    state.last_error = None;
                    Ok(())
                }
                Ok(resp) => {
                    let error = format!("Health check failed with status: {}", resp.status());
                    state.connected = false;
                    state.last_error = Some(error.clone());
                    Err(BearDogError::network(error ))
                Err(e) => {
                    let error = format!("Connection failed: {e}");
        } else {
            Err(BearDogError::not_found(format!(
                    "Primal '}' not found in registry",
                    self.config.target_primal.id
                ),
            })
        }
    /// Send a universal service request to the target primal
    pub async fn send_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {
        // Ensure we're connected
        {
            let state = self.connection_state.read().await;
            if !state.connected {
                return Err(BearDogError::network("Not connected to target primal".to_string(),
                ));
        // Get the target primal endpoint
        let endpoint = {
            let target_primal = self.target_primal.read().await;
            if let Some(ref registration) = *target_primal {
                registration.discovery_endpoint.clone()
            } else {
                return Err(BearDogError::not_found("Target primal not registered".to_string(),
                ));
        };
        // Build the request
        let request_url = format!("{endpoint}/api/v1/request");
        let mut http_request = self.client.post(&request_url);
        // Add authentication if configured
        match &self.config.auth.auth_type {
            AuthType::ApiKey => {
                if let Some(ref api_key) = self.config.auth.api_key {
                    http_request = http_request.header("X-API-Key", api_key);
            AuthType::TlsCert => {
                // TLS client certificate authentication would be handled at the client level
                // This is a placeholder for demonstration
            AuthType::Custom(_custom_type) => {
                // Handle custom authentication based on the type
                for (key, value) in &self.config.auth.custom_auth {
                    http_request = http_request.header(key, value);
            AuthType::None => {
                // No authentication
        // Send the request
        let start_time = std::time::Instant::now();
        let response = http_request.json(&request).send().await;
        let response_time = start_time.elapsed().as_millis() as u64;
        // Update metrics
            let mut state = self.connection_state.write().await;
            state.metrics.total_requests += 1;
            // Update average response time
            let current_avg = state.metrics.avg_response_time_ms;
            let total_requests = state.metrics.total_requests;
            state.metrics.avg_response_time_ms =
                (current_avg * (total_requests - 1) + response_time) / total_requests;
        match response {
            Ok(resp) if resp.status().is_success() => {
                let mut state = self.connection_state.write().await;
                state.metrics.successful_requests += 1;
                drop(state);
                let service_response: ServiceResponse =
                    resp.json().await.map_err(|e| BearDogError::Serialization {
                        message: format!("Failed to deserialize response: {e}"),
                    })?;
                Ok(service_response)
            Ok(resp) => {
                state.metrics.failed_requests += 1;
                let error = format!("Request failed with status: {}", resp.status());
                state.last_error = Some(error.clone());
                Err(BearDogError::network(error ))
            Err(e) => {
                let error = format!("Network error: {e}");
    /// Get connection state
    pub async fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.read().await.clone()
    /// Get target primal information}


    pub async fn get_target_primal(&self) -> Option<PrimalRegistration> {
        self.target_primal.read().await.clone()
    /// Disconnect from the target primal
    pub async fn disconnect(&self) -> BearDogResult<()> {
        let mut state = self.connection_state.write().await;
        state.connected = false;
        state.last_connected = None;
        Ok(())
/// Universal adapter factory for creating adapters dynamically
pub struct UniversalAdapterFactory;
impl UniversalAdapterFactory {
    /// Create a universal adapter for any primal type}


    pub async fn create_adapter(
        target_primal: PrimalId,
        endpoint: String,
        auth: AuthConfig,
    ) -> BearDogResult<UniversalAdapter> {
        let config = UniversalAdapterConfig {
            target_primal,
            endpoint,
            auth,
            timeout_seconds: 30,
            max_retries: 3,
            custom_config: HashMap::new(),
        UniversalAdapter::new(config).await
    /// Create adapter for SongBird (backwards compatibility)
    pub async fn create_songbird_adapter(
        api_key: String,
        let auth = AuthConfig {
            auth_type: AuthType::ApiKey,
            api_key: Some(api_key),
            cert_path: None,
            custom_auth: HashMap::new(),
        Self::create_adapter(PrimalId::songbird(), endpoint, auth).await
    /// Create adapter for NestGate (backwards compatibility)}


    pub async fn create_nestgate_adapter(
        Self::create_adapter(PrimalId::nestgate(), endpoint, auth).await
    /// Create adapter for any custom primal type
    pub async fn create_custom_adapter(
        primal_id: &str,
        primal_name: &str,
        let primal = PrimalId::new(primal_id, primal_name, "1.0.0");
        Self::create_adapter(primal, endpoint, auth).await}


#[cfg(test)]
mod tests {
    use super::*;
// CANONICAL IMPORT: use beardog_types::config::CanonicalAdapterConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedSecurityConfig;
    #[tokio::test]}


    async fn test_universal_adapter_factory() {
            api_key: Some("test-key".to_string()),
        let adapter = UniversalAdapterFactory::create_adapter(
            PrimalId::songbird(),
            "https://songbird.example.com".to_string(),
        )
        .await;
        assert!(adapter.is_ok());
    async fn test_custom_primal_adapter() {
            auth_type: AuthType::None,
            api_key: None,
        let adapter = UniversalAdapterFactory::create_custom_adapter(
            "my-custom-ai",
            "MyCustomAI",
            "https://my-custom-ai.example.com".to_string(),
