// SPDX-License-Identifier: AGPL-3.0-only

//! Performance and Security Configuration
//!
//! AI performance optimization and security settings.

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

type Result<T> = Result<T>;

/// AI performance optimization configuration
///
/// # Examples
///
/// ```ignore
/// let config = AiPerformanceConfig::builder()
///     .cpu_threads(8)
///     .memory_limit_mb(8192)
///     .enable_mixed_precision(true)
///     .build()?;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiPerformanceConfig {
    enabled: bool,
    cpu_threads: usize,
    memory_limit_mb: usize,
    mixed_precision: bool,
    quantization: bool,
}

impl AiPerformanceConfig {
    /// Create a new builder
    pub fn builder() -> AiPerformanceConfigBuilder {
        AiPerformanceConfigBuilder::default()
    }

    /// Check if performance optimization is enabled
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get CPU thread pool size
    pub const fn cpu_threads(&self) -> usize {
        self.cpu_threads
    }

    /// Get memory limit in MB
    pub const fn memory_limit_mb(&self) -> usize {
        self.memory_limit_mb
    }

    /// Check if mixed precision training is enabled
    pub const fn is_mixed_precision_enabled(&self) -> bool {
        self.mixed_precision
    }

    /// Check if quantization is enabled
    pub const fn is_quantization_enabled(&self) -> bool {
        self.quantization
    }
}

impl Default for AiPerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cpu_threads: std::env::var("BEARDOG_AI_CPU_THREADS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4), // Default to 4 threads
            memory_limit_mb: std::env::var("BEARDOG_AI_MEMORY_LIMIT_MB")
                .ok()
                .and_then(|m| m.parse().ok())
                .unwrap_or(4096), // 4GB default
            mixed_precision: false,
            quantization: false,
        }
    }
}

/// Builder for `AiPerformanceConfig`
#[derive(Debug, Clone)]
pub struct AiPerformanceConfigBuilder {
    enabled: bool,
    cpu_threads: Option<usize>,
    memory_limit_mb: Option<usize>,
    mixed_precision: bool,
    quantization: bool,
}

impl Default for AiPerformanceConfigBuilder {
    fn default() -> Self {
        Self {
            enabled: false,
            cpu_threads: None,
            memory_limit_mb: None,
            mixed_precision: false,
            quantization: false,
        }
    }
}

impl AiPerformanceConfigBuilder {
    /// Enable or disable performance optimization
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set CPU thread pool size
    pub fn cpu_threads(mut self, threads: usize) -> Result<Self> {
        if threads == 0 {
            return Err(BearDogError::Business {
                message: "CPU threads must be at least 1".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        self.cpu_threads = Some(threads);
        Ok(self)
    }

    /// Set memory limit in MB
    pub fn memory_limit_mb(mut self, limit: usize) -> Result<Self> {
        if limit == 0 {
            return Err(BearDogError::Business {
                message: "Memory limit must be greater than 0".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        self.memory_limit_mb = Some(limit);
        Ok(self)
    }

    /// Enable or disable mixed precision training
    pub fn enable_mixed_precision(mut self, enabled: bool) -> Self {
        self.mixed_precision = enabled;
        self
    }

    /// Enable or disable quantization
    pub fn enable_quantization(mut self, enabled: bool) -> Self {
        self.quantization = enabled;
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<AiPerformanceConfig> {
        Ok(AiPerformanceConfig {
            enabled: self.enabled,
            cpu_threads: self.cpu_threads.unwrap_or(4),
            memory_limit_mb: self.memory_limit_mb.unwrap_or(
                std::env::var("BEARDOG_AI_MEMORY_LIMIT_MB")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(4096)
            ),
            mixed_precision: self.mixed_precision,
            quantization: self.quantization,
        })
    }

    /// Preset: High performance configuration
    pub fn high_performance() -> Self {
        Self::default()
            .enabled(true)
            .enable_mixed_precision(true)
            .enable_quantization(true)
    }

    /// Preset: Balanced configuration
    pub fn balanced() -> Self {
        Self::default()
            .enabled(true)
            .enable_mixed_precision(false)
            .enable_quantization(false)
    }
}

/// AI security and privacy configuration
///
/// # Examples
///
/// ```ignore
/// let config = AiSecurityConfig::builder()
///     .enable_adversarial_protection(true)
///     .enable_differential_privacy(true)
///     .build()?;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiSecurityConfig {
    enabled: bool,
    enable_data_encryption: bool,
    enable_model_encryption: bool,
    adversarial_protection: bool,
    enable_differential_privacy: bool,
}

impl AiSecurityConfig {
    /// Create a new builder
    pub fn builder() -> AiSecurityConfigBuilder {
        AiSecurityConfigBuilder::default()
    }

    /// Check if AI security is enabled
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if data encryption is enabled
    pub const fn is_data_encryption_enabled(&self) -> bool {
        self.enable_data_encryption
    }

    /// Check if model encryption is enabled
    pub const fn is_model_encryption_enabled(&self) -> bool {
        self.enable_model_encryption
    }

    /// Check if adversarial protection is enabled
    pub const fn is_adversarial_protection_enabled(&self) -> bool {
        self.adversarial_protection
    }

    /// Check if differential privacy is enabled
    pub const fn is_differential_privacy_enabled(&self) -> bool {
        self.enable_differential_privacy
    }
}

impl Default for AiSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            enable_data_encryption: true,
            enable_model_encryption: true,
            adversarial_protection: true,
            enable_differential_privacy: false,
        }
    }
}

/// Builder for `AiSecurityConfig`
#[derive(Debug, Clone, Default)]
pub struct AiSecurityConfigBuilder {
    enabled: bool,
    enable_data_encryption: bool,
    enable_model_encryption: bool,
    adversarial_protection: bool,
    enable_differential_privacy: bool,
}

impl AiSecurityConfigBuilder {
    /// Enable or disable AI security
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable or disable data encryption
    pub fn enable_data_encryption(mut self, enabled: bool) -> Self {
        self.enable_data_encryption = enabled;
        self
    }

    /// Enable or disable model encryption
    pub fn enable_model_encryption(mut self, enabled: bool) -> Self {
        self.enable_model_encryption = enabled;
        self
    }

    /// Enable or disable adversarial protection
    pub fn enable_adversarial_protection(mut self, enabled: bool) -> Self {
        self.adversarial_protection = enabled;
        self
    }

    /// Enable or disable differential privacy
    pub fn enable_differential_privacy(mut self, enabled: bool) -> Self {
        self.enable_differential_privacy = enabled;
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<AiSecurityConfig> {
        Ok(AiSecurityConfig {
            enabled: self.enabled,
            enable_data_encryption: self.enable_data_encryption,
            enable_model_encryption: self.enable_model_encryption,
            adversarial_protection: self.adversarial_protection,
            enable_differential_privacy: self.enable_differential_privacy,
        })
    }

    /// Preset: Maximum security configuration
    pub fn maximum_security() -> Self {
        Self {
            enabled: true,
            enable_data_encryption: true,
            enable_model_encryption: true,
            adversarial_protection: true,
            enable_differential_privacy: true,
        }
    }

    /// Preset: Production security configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            enable_data_encryption: true,
            enable_model_encryption: true,
            adversarial_protection: true,
            enable_differential_privacy: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_builder() {
        let config = AiPerformanceConfig::builder()
            .enabled(true)
            .cpu_threads(8)
            ?
            .memory_limit_mb(8192)
            ?
            .build()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            ?;

        assert!(config.is_enabled());
        assert_eq!(config.cpu_threads(), 8);
        assert_eq!(config.memory_limit_mb(), 8192);
    }

    #[test]
    fn test_cpu_threads_validation() {
        assert!(AiPerformanceConfig::builder().cpu_threads(0).is_err());
        assert!(AiPerformanceConfig::builder().cpu_threads(1).is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_security_builder() {
        let config = AiSecurityConfig::builder()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .enabled(true)
            .enable_adversarial_protection(true)
            .build()
            ?;

        assert!(config.is_enabled());
        assert!(config.is_adversarial_protection_enabled());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_security_presets() {
        let max_sec = AiSecurityConfigBuilder::maximum_security().build()?;
        assert!(max_sec.is_differential_privacy_enabled());

        let prod = AiSecurityConfigBuilder::production().build()?;
        assert!(!prod.is_differential_privacy_enabled());
    }
}
