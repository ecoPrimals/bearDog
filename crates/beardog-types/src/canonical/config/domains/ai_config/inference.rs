//! Inference Configuration
//!
//! Type-safe ML model inference and serving configuration with builder pattern.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

type Result<T> = BearDogResult<T>;

/// ML model inference configuration
///
/// # Examples
///
/// ```ignore
/// let config = InferenceConfig::builder()
///     .endpoint("http://model-server:8080")
///     .max_batch_size(64)?
///     .timeout(Duration::from_secs(30))
///     .enable_caching(true)
///     .build()?;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceConfig {
    enabled: bool,
    endpoints: Vec<String>,
    max_batch_size: usize,
    timeout: Duration,
    enable_caching: bool,
    cache_size_limit: usize,
    enable_gpu: bool,
}

impl InferenceConfig {
    /// Create a new builder
    pub fn builder() -> InferenceConfigBuilder {
        InferenceConfigBuilder::default()
    }

    /// Check if inference is enabled
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get serving endpoints
    pub fn endpoints(&self) -> &[String] {
        &self.endpoints
    }

    /// Get maximum batch size
    pub const fn max_batch_size(&self) -> usize {
        self.max_batch_size
    }

    /// Get inference timeout
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Check if caching is enabled
    pub const fn is_caching_enabled(&self) -> bool {
        self.enable_caching
    }

    /// Get cache size limit
    pub const fn cache_size_limit(&self) -> usize {
        self.cache_size_limit
    }

    /// Check if GPU acceleration is enabled
    pub const fn is_gpu_enabled(&self) -> bool {
        self.enable_gpu
    }
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoints: Vec::new(),
            max_batch_size: std::env::var("BEARDOG_AI_MAX_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_AI_INFERENCE_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            enable_caching: true,
            cache_size_limit: std::env::var("BEARDOG_AI_CACHE_SIZE_BYTES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024 * 1024 * 1024), // 1GB
            enable_gpu: false,
        }
    }
}

/// Builder for `InferenceConfig`
#[derive(Debug, Clone, Default)]
pub struct InferenceConfigBuilder {
    enabled: bool,
    endpoints: Vec<String>,
    max_batch_size: Option<usize>,
    timeout: Option<Duration>,
    enable_caching: bool,
    cache_size_limit: Option<usize>,
    enable_gpu: bool,
}

impl InferenceConfigBuilder {
    /// Enable or disable inference
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a serving endpoint
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoints.push(endpoint.into());
        self
    }

    /// Add multiple serving endpoints
    pub fn endpoints(mut self, endpoints: impl IntoIterator<Item = String>) -> Self {
        self.endpoints.extend(endpoints);
        self
    }

    /// Set maximum batch size
    pub fn max_batch_size(mut self, size: usize) -> Result<Self> {
        if size == 0 {
            return Err(BearDogError::Business {
                message: "Maximum batch size must be non-zero".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        self.max_batch_size = Some(size);
        Ok(self)
    }

    /// Set inference timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Enable or disable caching
    pub fn enable_caching(mut self, enabled: bool) -> Self {
        self.enable_caching = enabled;
        self
    }

    /// Set cache size limit in bytes
    pub fn cache_size_limit(mut self, limit: usize) -> Self {
        self.cache_size_limit = Some(limit);
        self
    }

    /// Enable or disable GPU acceleration
    pub fn enable_gpu(mut self, enabled: bool) -> Self {
        self.enable_gpu = enabled;
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<InferenceConfig> {
        // Validate at least one endpoint if inference is enabled
        if self.enabled && self.endpoints.is_empty() {
            return Err(BearDogError::Business {
                message: "At least one endpoint required when inference is enabled".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Validate timeout is reasonable
        if let Some(timeout) = self.timeout {
            if timeout.as_secs() > 300 {
                return Err(BearDogError::Business {
                    message: "Inference timeout should not exceed 5 minutes (300 seconds)"
                        .to_string(),
                    category: beardog_errors::BusinessErrorCategory::Validation,
                });
            }
        }

        Ok(InferenceConfig {
            enabled: self.enabled,
            endpoints: self.endpoints,
            max_batch_size: self.max_batch_size.unwrap_or(32),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            enable_caching: self.enable_caching,
            cache_size_limit: self.cache_size_limit.unwrap_or(1024 * 1024 * 1024),
            enable_gpu: self.enable_gpu,
        })
    }

    /// Preset: Fast inference configuration
    pub fn fast() -> Self {
        Self::default()
            .enabled(true)
            .enable_caching(true)
            .enable_gpu(true)
    }

    /// Preset: Standard inference configuration
    pub fn standard() -> Self {
        Self::default().enabled(true).enable_caching(true)
    }

    /// Preset: CPU-only inference configuration
    pub fn cpu_only() -> Self {
        Self::default()
            .enabled(true)
            .enable_caching(true)
            .enable_gpu(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_validation() {
        // Should fail without endpoints when enabled
        let result = InferenceConfig::builder().enabled(true).build();
        assert!(result.is_err());

        // Should succeed with endpoint
        let result = InferenceConfig::builder()
            .enabled(true)
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .endpoint("http://localhost:8080")
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_size_validation() {
        let result = InferenceConfig::builder().max_batch_size(0);
        assert!(result.is_err());

        let result = InferenceConfig::builder().max_batch_size(32);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(result.is_ok());
    }

    #[test]
    fn test_timeout_validation() {
        let result = InferenceConfig::builder()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .endpoint("http://localhost:8080")
            .enabled(true)
            .timeout(Duration::from_secs(400))
            .build();
        assert!(result.is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_presets() {
        let fast = InferenceConfigBuilder::fast()
            .endpoint("http://localhost:8080")
            .build();
        assert!(fast.is_ok());
        assert!(fast?.is_gpu_enabled());

        let cpu = InferenceConfigBuilder::cpu_only()
            .endpoint("http://localhost:8080")
            .build();
        assert!(cpu.is_ok());
        assert!(!cpu?.is_gpu_enabled());
    }
}
