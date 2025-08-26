

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::primal_registry::{global_registry, PrimalId, PrimalRegistration};
use super::traits::*;
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UniversalAdapter {

    config: UniversalAdapterConfig,

    target_primal: Arc<RwLock<Option<PrimalRegistration>>>,

    client: reqwest::Client,

    connection_state: Arc<RwLock<ConnectionState>>,

#[derive(Debug, Clone)]
pub struct ConnectionState {

    pub connected: bool,

    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,

    pub last_error: Option<String>,

    pub metrics: ConnectionMetrics,

#[derive(Debug, Clone, Default)]
pub struct ConnectionMetrics {

    pub total_requests: u64,

    pub successful_requests: u64,

    pub failed_requests: u64,

    pub avg_response_time_ms: u64,}

impl UniversalAdapter {

    pub async fn new(config: UniversalAdapterConfig) -> BearDogResult<Self> {

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

    pub async fn connect(&self) -> BearDogResult<()> {

        let registry = global_registry().await;
        let primal_registration = registry
            .discover_primal(&self.config.target_primal.id)
            .await?;
        if let Some(registration) = primal_registration {

            {
                let mut target_primal = self.target_primal.write().await;
                *target_primal = Some(registration.clone());
            }

            let health_endpoint = format_args!("{}/health", registration.discovery_endpoint).to_string();
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
                    let error = format_args!("Health check failed with status: {}", resp.status().to_string());
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

    pub async fn send_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {

        {
            let state = self.connection_state.read().await;
            if !state.connected {
                return Err(BearDogError::network("Not connected to target primal".to_string(),
                ));

        let endpoint = {
            let target_primal = self.target_primal.read().await;
            if let Some(ref registration) = *target_primal {
                registration.discovery_endpoint.clone()
            } else {
                return Err(BearDogError::not_found("Target primal not registered".to_string(),
                ));
        };

        let request_url = format!("{endpoint}/api/v1/request");
        let mut http_request = self.client.post(&request_url);

        match &self.config.auth.auth_type {
            AuthType::ApiKey => {
                if let Some(ref api_key) = self.config.auth.api_key {
                    http_request = http_request.header("X-API-Key", api_key);
            AuthType::TlsCert => {

            AuthType::Custom(_custom_type) => {

                for (key, value) in &self.config.auth.custom_auth {
                    http_request = http_request.header(key, value);
            AuthType::None => {

        let start_time = std::time::Instant::now();
        let response = http_request.json(&request).send().await;
        let response_time = start_time.elapsed().as_millis() as u64;

            let mut state = self.connection_state.write().await;
            state.metrics.total_requests += 1;

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
                let error = format_args!("Request failed with status: {}", resp.status().to_string());
                state.last_error = Some(error.clone());
                Err(BearDogError::network(error ))
            Err(e) => {
                let error = format!("Network error: {e}");

    pub async fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.read().await.clone()

    pub async fn get_target_primal(&self) -> Option<PrimalRegistration> {
        self.target_primal.read().await.clone()

    pub async fn disconnect(&self) -> BearDogResult<()> {
        let mut state = self.connection_state.write().await;
        state.connected = false;
        state.last_connected = None;
        Ok(())

pub struct UniversalAdapterFactory;
impl UniversalAdapterFactory {

    pub async fn create_adapter(
        target_primal: PrimalId,
        endpoint: &str,
        auth: UnifiedAuthConfig,
    ) -> BearDogResult<UniversalAdapter> {
        let config = UniversalAdapterConfig {
            target_primal,
            endpoint,
            auth,
            timeout_seconds: 30,
            max_retries: 3,
            custom_config: HashMap::with_capacity(16),
        UniversalAdapter::new(config).await

    pub async fn create_songbird_adapter(
        api_key: &str,
        let auth = AuthConfig {
            auth_type: AuthType::ApiKey,
            api_key: Some(api_key),
            cert_path: None,
            custom_auth: HashMap::with_capacity(16),
        Self::create_adapter(PrimalId::songbird(), endpoint, auth).await

    pub async fn create_nestgate_adapter(
        Self::create_adapter(PrimalId::nestgate(), endpoint, auth).await

    pub async fn create_custom_adapter(
        primal_id: &str,
        primal_name: &str,
        let primal = PrimalId::new(primal_id, primal_name, "1.0.0");
        Self::create_adapter(primal, endpoint, auth).await}

#[cfg(test)]
mod tests {
    use super::*;

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
