

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub mod authentication;
pub mod protocols;
pub mod providers;
pub mod transformers;
pub use authentication::*;
pub use protocols::*;
pub use providers::*;
pub use transformers::*;

pub struct UniversalExternalAdapter {
    providers: HashMap<String, Box<dyn UniversalProvider>>,
    config: UniversalAdapterConfig,
}

#[derive(Debug, Clone)]

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub system_id: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub timeout_override: Option<u64>,

pub struct UniversalResponse {
    pub success: bool,
    pub processing_time_ms: u64,

#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
pub trait ExternalSystemProvider: Send + Sync {

    fn system_id(&self) -> &str;

    fn protocol(&self) -> &dyn Protocol;

    fn authentication(&self) -> &dyn Authentication;

    async fn execute(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> BearDogResult<serde_json::Value>;

    async fn health_check(&self) -> BearDogResult<bool>;

    fn capabilities(&self) -> Vec<String>;}

impl UniversalExternalAdapter {

    pub fn new(config: UniversalAdapterConfig) -> Self {
            providers: HashMap::with_capacity(16),
            config,

    pub fn register_provider(&mut self, provider: Box<dyn UniversalProvider>) {
        let system_id = provider.system_id().to_string();
        info!("🔌 Registering external system provider: {}", system_id);
        self.providers.insert(system_id, provider);

    pub async fn execute(&self, request: UniversalRequest) -> BearDogResult<UniversalResponse> {
        let start_time = std::time::Instant::now();
        debug!(
            "🌐 Universal adapter executing: {} -> {}",
            request.system_id, request.operation
        );
        let provider = self.providers.get(&request.system_id).ok_or_else(|| {
            BearDogError::configuration(format_args!("External system provider not found: {}", request.system_id).to_string())
        })?;

        let timeout = Duration::from_secs(
            request
                .timeout_override
                .unwrap_or(self.config.timeout_seconds),

        let result = self
            .execute_with_retry(provider.as_ref(), &request, timeout)
            .await;
        let processing_time = start_time.elapsed().as_millis() as u64;
        match result {
            Ok(payload) => Ok(UniversalResponse {
                success: true,
                payload,
                metadata: HashMap::with_capacity(16),
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
                    metadata: HashMap::with_capacity(16),
                    processing_time_ms: processing_time,
                    system_id: request.system_id,
                    operation: request.operation,
                })
            }

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
                    let timeout_error = BearDogError::network(format_args!("External system timeout after {}s", timeout.as_secs().to_string()));
                    warn!("⏰ External system timeout on attempt {}", attempt);
                    last_error = Some(timeout_error);

            if attempt < self.config.retry_attempts {
                tokio::time::sleep(Duration::from_millis(
                    self.config.retry_backoff_ms * attempt as u64,
                ))
                .await;

        Err(
            last_error.unwrap_or_else(|| BearDogError::internal(format_args!("All retry attempts failed for service: {}", request.system_id).to_string())),
        )

    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let mut results = HashMap::with_capacity(16);
        for (system_id, provider) in &self.providers {
            match provider.health_check().await {
                Ok(healthy) => {
                    results.insert(system_id.clone(), healthy);
                Err(e) => {
                    warn!("🏥 Health check failed for {}: {}", system_id, e);
                    results.insert(system_id.clone(), false);
        results

    pub fn get_systems_info(&self) -> HashMap<String, Vec<String>> {
        self.providers
            .iter()
            .map(|(id, provider)| (id.clone(), provider.capabilities()))
            .collect()

    pub fn unregister_provider(&mut self, system_id: &str) -> bool {
        match self.providers.remove(system_id) {
            Some(_) => {
                info!("🔌 Unregistered external system provider: {}", system_id);
                true
            None => {
                warn!("⚠️ Attempted to unregister unknown provider: {}", system_id);
                false
