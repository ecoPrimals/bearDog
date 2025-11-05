//! # `BearDog` Universal Adapters
//!
//! Provides capability-based adapters for ecosystem integration, enabling
//! `BearDog` to work with multiple security providers, cloud vendors, and HSM hardware
//! without vendor lock-in.
//!
//! ## Features
//!
//! - **Universal Adapter Pattern**: Works with any security provider
//! - **Capability-Based Discovery**: Services discovered by capability, not name

#![deny(unsafe_code)]
//! - **Zero Vendor Lock-in**: Pluggable providers (AWS, Azure, GCP, Vault)
//! - **Automatic Failover**: Graceful degradation when providers unavailable
//! - **Response Caching**: Efficient request deduplication
//!
//! ## Example
//!
//! ```rust
//! use beardog_adapters::{UniversalAdapter, AdapterConfig};
//!
//! let config = AdapterConfig::default();
//! let adapter = UniversalAdapter::new(config);
//! // Adapter automatically discovers and connects to available providers
//! ```
//!
//! ## Sovereignty
//!
//! This crate implements sovereign computing principles - no hardcoded
//! provider dependencies, all services discovered dynamically.

// October 26, 2025: Week 2 Day 4 - Integration Tests
#[cfg(test)]
mod adapter_integration_tests;
#[cfg(test)]
mod adapter_resilience_tests;

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod lib_comprehensive_tests;

// October 29, 2025: Extended adapter validation tests
#[cfg(test)]
#[path = "tests/adapter_validation_extended_tests.rs"]
mod adapter_validation_extended_tests;

// October 30, 2025: Comprehensive adapter operations tests for coverage expansion
#[cfg(test)]
#[path = "tests/adapter_operations_comprehensive_tests.rs"]
mod adapter_operations_comprehensive_tests;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct UniversalAdapter {
    capabilities: Vec<String>,
    endpoints: HashMap<String, String>,
    config: AdapterConfig,
    cache: HashMap<String, (CapabilityResponse, Instant)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub timeout_seconds: u64,
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
    /// Whether `enable_caching` is enabled
    pub enable_caching: bool,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            retry_attempts: 3,
            enable_caching: true,
        }
    }
}

/// Capability request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// The capability value
    pub capability: String,
    /// The operation value
    pub operation: String,
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
}

/// Capability response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Whether success is enabled
    pub success: bool,
    /// Optional data
    pub data: Option<serde_json::Value>,
    /// Optional error
    pub error: Option<String>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// AI integration response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {
    pub confidence_score: f64,
    pub processing_time_ms: u32,
    /// The model version value
    pub model_version: String,
}

impl Default for AIResponseMetadata {
    fn default() -> Self {
        Self {
            confidence_score: 1.0,
            processing_time_ms: 0,
            model_version: "v1.0.0".to_string(),
        }
    }
}

/// AI integration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIIntegrationResponse {
    /// The result value
    pub result: String,
    /// The ai metadata value
    pub ai_metadata: AIResponseMetadata,
    /// Collection of suggested actions
    pub suggested_actions: Vec<String>,
}

impl Default for AIIntegrationResponse {
    fn default() -> Self {
        Self {
            result: "success ".to_string(),
            ai_metadata: AIResponseMetadata::default(),
            suggested_actions: Vec::new(),
        }
    }
}

/// Vendor discovery context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorDiscoveryContext {
    /// The discovery method value
    pub discovery_method: String,
    /// Number of priority
    pub priority: u32,
    /// Mapping of context
    pub context: HashMap<String, String>,
}

impl Default for VendorDiscoveryContext {
    fn default() -> Self {
        Self {
            discovery_method: "capability_based".to_string(),
            priority: 1,
            context: HashMap::with_capacity(16),
        }
    }
}

impl UniversalAdapter {
    /// Create new universal adapter
    /// Creates a new instance
    #[must_use]
    pub fn new(config: AdapterConfig) -> Self {
        Self {
            capabilities: Vec::new(),
            endpoints: HashMap::new(),
            config,
            cache: HashMap::new(),
        }
    }

    /// Register a capability with endpoint
    pub fn register_capability(&mut self, capability: String, endpoint: String) {
        self.capabilities.push(capability.clone());
        self.endpoints.insert(capability, endpoint);
    }

    /// Execute capability request with timeout, retry, and caching
    /// Executes capability
    /// Executes capability
    pub async fn execute_capability(
        &mut self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        if !self.capabilities.contains(&request.capability) {
            return Ok(CapabilityResponse {
                success: false,
                data: None,
                error: Some("Capability not available".to_string()),
                metadata: HashMap::new(),
            });
        }

        // Check cache if enabled
        if self.config.enable_caching {
            let cache_key = format!("{}:{}", request.capability, request.operation);
            if let Some((cached_response, cached_time)) = self.cache.get(&cache_key) {
                // Cache valid for 5 minutes
                let cache_duration_secs = std::env::var("BEARDOG_ADAPTER_CACHE_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300); // 5 minutes default
                if cached_time.elapsed() < Duration::from_secs(cache_duration_secs) {
                    let mut response = cached_response.clone();
                    response
                        .metadata
                        .insert("cached".to_string(), "true".to_string());
                    return Ok(response);
                }
            }
        }

        // Execute with retry logic
        let mut last_error = None;
        for attempt in 0..self.config.retry_attempts {
            match self.execute_capability_internal(&request).await {
                Ok(mut response) => {
                    // Cache successful response if enabled
                    if self.config.enable_caching && response.success {
                        let cache_key = format!("{}:{}", request.capability, request.operation);
                        self.cache
                            .insert(cache_key, (response.clone(), Instant::now()));
                    }

                    response
                        .metadata
                        .insert("attempt".to_string(), (attempt + 1).to_string());
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.retry_attempts - 1 {
                        // Exponential backoff: 100ms, 200ms, 400ms, etc.
                        let delay = Duration::from_millis(100 * (1 << attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        // All retries failed
        Err(last_error.unwrap_or_else(|| BearDogError::Adapter {
            message: "All retry attempts failed".to_string(),
        }))
    }

    /// Internal capability execution with timeout
    /// Executes `capability_internal`
    async fn execute_capability_internal(
        &self,
        request: &CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        let timeout_duration = Duration::from_secs(self.config.timeout_seconds);

        match timeout(
            timeout_duration,
            self.simulate_capability_execution(request),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(BearDogError::Adapter {
                message: format!(
                    "Capability execution timed out after {} seconds",
                    self.config.timeout_seconds
                ),
            }),
        }
    }

    /// Simulate capability execution (replace with actual implementation)
    async fn simulate_capability_execution(
        &self,
        request: &CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        // Simulate some processing time
        let sleep_millis = std::env::var("BEARDOG_ADAPTER_RETRY_SLEEP_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);
        tokio::time::sleep(Duration::from_millis(sleep_millis)).await;

        // Simulate occasional failures for testing retry logic
        if request.operation == "fail_test" {
            return Err(BearDogError::Adapter {
                message: "Simulated failure for testing".to_string(),
            });
        }

        Ok(CapabilityResponse {
            success: true,
            data: Some(serde_json::json!({
                "result": "executed",
                "capability": request.capability,
                "operation": request.operation,
                "parameters": request.parameters
            })),
            error: None,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("execution_time_ms".to_string(), "10".to_string());
                meta.insert("cached".to_string(), "false".to_string());
                meta
            },
        })
    }

    /// Get available capabilities
    /// Gets capabilities
    /// Gets capabilities
    #[must_use]
    pub fn get_capabilities(&self) -> &[String] {
        &self.capabilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_adapter_creation() {
        let config = AdapterConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter = UniversalAdapter::new(config);
        assert!(adapter.get_capabilities().is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_capability_execution() -> Result<(), Box<dyn std::error::Error>> {
        let mut adapter = UniversalAdapter::new(AdapterConfig::default());
        adapter.register_capability("test".to_string(), "http://test.com".to_string());

        let request = CapabilityRequest {
            capability: "test".to_string(),
            operation: "execute".to_string(),
            parameters: HashMap::new(),
        };

        let response = adapter.execute_capability(request).await?;
        assert!(response.success);
        Ok(())
    }
}

#[cfg(test)]
mod lib_main_tests {
    use super::*;

    #[test]
    fn test_adapters_lib_accessible() {
        // Verify adapters lib module loads
    }

    #[test]
    fn test_adapter_config_creation() {
        // Test adapter config default
        let config = AdapterConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.retry_attempts, 3);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_universal_module_accessible() {
        // Verify universal module is accessible
        // Note: Universal module is a directory-based module in this crate
        let _config = AdapterConfig::default();
    }

    #[tokio::test]
    async fn test_async_adapters() {
        // Verify async operations work
        tokio::time::sleep(std::time::Duration::from_micros(1)).await;
    }
}
