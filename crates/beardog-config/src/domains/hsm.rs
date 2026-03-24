// SPDX-License-Identifier: AGPL-3.0-only

//! HSM configuration

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// HSM configuration for hardware and software providers
///
/// **Note**: Contains multiple boolean flags for fine-grained HSM control.
/// Each flag enables/disables a specific HSM provider type.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Auto-detect available HSMs
    #[serde(default = "default_true")]
    pub auto_detect: bool,

    /// Prefer hardware over software
    #[serde(default = "default_true")]
    pub prefer_hardware: bool,

    /// Enable `SoftHSM2` provider
    #[serde(default = "default_true")]
    pub enable_softhsm: bool,

    /// Enable `YubiHSM` provider
    #[serde(default)]
    pub enable_yubihsm: bool,

    /// Enable TPM provider
    #[serde(default)]
    pub enable_tpm: bool,

    /// Enable Android `StrongBox` (if on Android)
    #[serde(default)]
    pub enable_strongbox: bool,

    /// `SoftHSM2` configuration file path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softhsm_config: Option<PathBuf>,

    /// `YubiHSM` connector URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yubihsm_connector: Option<String>,

    /// Preferred provider order
    #[serde(default = "default_provider_order")]
    pub provider_order: Vec<String>,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            prefer_hardware: true,
            enable_softhsm: true,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            softhsm_config: None,
            yubihsm_connector: None,
            provider_order: default_provider_order(),
        }
    }
}

impl HsmConfig {
    /// Load HSM configuration from environment variables
    ///
    /// Reads configuration from environment variables:
    /// - `BEARDOG_HSM_AUTO_DETECT`: Auto-detect HSMs (default: true)
    /// - `BEARDOG_HSM_PREFER_HARDWARE`: Prefer hardware HSMs (default: true)
    /// - `BEARDOG_HSM_ENABLE_SOFTHSM`: Enable SoftHSM2 (default: true)
    /// - `BEARDOG_HSM_ENABLE_YUBIHSM`: Enable YubiHSM (default: false)
    /// - `BEARDOG_HSM_ENABLE_TPM`: Enable TPM (default: false)
    /// - `BEARDOG_HSM_ENABLE_STRONGBOX`: Enable StrongBox (default: false)
    /// - `SOFTHSM2_CONF`: SoftHSM2 config file path
    /// - `BEARDOG_YUBIHSM_CONNECTOR`: YubiHSM connector URL
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::default();

        Self {
            auto_detect: std::env::var("BEARDOG_HSM_AUTO_DETECT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.auto_detect),
            prefer_hardware: std::env::var("BEARDOG_HSM_PREFER_HARDWARE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.prefer_hardware),
            enable_softhsm: std::env::var("BEARDOG_HSM_ENABLE_SOFTHSM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.enable_softhsm),
            enable_yubihsm: std::env::var("BEARDOG_HSM_ENABLE_YUBIHSM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.enable_yubihsm),
            enable_tpm: std::env::var("BEARDOG_HSM_ENABLE_TPM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.enable_tpm),
            enable_strongbox: std::env::var("BEARDOG_HSM_ENABLE_STRONGBOX")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.enable_strongbox),
            softhsm_config: std::env::var("SOFTHSM2_CONF")
                .ok()
                .map(PathBuf::from)
                .or(defaults.softhsm_config),
            yubihsm_connector: std::env::var("BEARDOG_YUBIHSM_CONNECTOR")
                .ok()
                .or(defaults.yubihsm_connector),
            provider_order: defaults.provider_order,
        }
    }

    /// Validate HSM configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Check that at least one provider is enabled
        if !self.auto_detect
            && !self.enable_softhsm
            && !self.enable_yubihsm
            && !self.enable_tpm
            && !self.enable_strongbox
        {
            return Err(ConfigError::validation(
                "At least one HSM provider must be enabled or auto_detect must be true",
            ));
        }

        // Validate SoftHSM config path if specified
        if let Some(ref config) = self.softhsm_config
            && !config.exists()
        {
            return Err(ConfigError::PathNotFound(format!(
                "SoftHSM2 config not found: {}",
                config.display()
            )));
        }

        // Validate YubiHSM connector URL if specified
        if let Some(ref url) = self.yubihsm_connector
            && !url.starts_with("http://")
            && !url.starts_with("https://")
        {
            return Err(ConfigError::invalid_value(
                "hsm.yubihsm_connector",
                "Must be a valid HTTP(S) URL",
            ));
        }

        Ok(())
    }

    /// Get enabled providers in order of preference
    #[must_use]
    pub fn get_enabled_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();

        for provider in &self.provider_order {
            match provider.as_str() {
                "softhsm" if self.enable_softhsm => providers.push(provider.clone()),
                "yubihsm" if self.enable_yubihsm => providers.push(provider.clone()),
                "tpm" if self.enable_tpm => providers.push(provider.clone()),
                "strongbox" if self.enable_strongbox => providers.push(provider.clone()),
                _ => {}
            }
        }

        providers
    }
}

const fn default_true() -> bool {
    true
}

fn default_provider_order() -> Vec<String> {
    vec![
        "strongbox".to_string(),
        "tpm".to_string(),
        "yubihsm".to_string(),
        "softhsm".to_string(),
    ]
}

#[cfg(test)]
#[path = "hsm_comprehensive_tests.rs"]
mod hsm_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hsm_config() {
        let config = HsmConfig::default();
        assert!(config.validate().is_ok());
        assert!(config.auto_detect);
        assert!(config.prefer_hardware);
    }

    #[test]
    fn test_no_providers_enabled() {
        let config = HsmConfig {
            auto_detect: false,
            enable_softhsm: false,
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_enabled_providers() {
        let config = HsmConfig {
            enable_softhsm: true,
            enable_yubihsm: true,
            enable_tpm: false,
            enable_strongbox: false,
            ..Default::default()
        };

        let providers = config.get_enabled_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"softhsm".to_string()));
        assert!(providers.contains(&"yubihsm".to_string()));
    }

    #[test]
    fn test_from_env_creates_valid_config() {
        let config = HsmConfig::from_env();

        // Should have defaults
        assert!(config.auto_detect);
        assert!(config.prefer_hardware);
        assert!(config.enable_softhsm);
    }

    #[test]
    fn test_clone_hsm_config() {
        let config = HsmConfig::default();
        let cloned = config.clone();

        assert_eq!(config.auto_detect, cloned.auto_detect);
        assert_eq!(config.prefer_hardware, cloned.prefer_hardware);
    }

    #[test]
    fn test_debug_format() {
        let config = HsmConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("HsmConfig"));
        assert!(debug_str.contains("auto_detect"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = HsmConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: HsmConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.auto_detect, deserialized.auto_detect);
        assert_eq!(config.prefer_hardware, deserialized.prefer_hardware);
    }

    #[test]
    fn test_auto_detect_allows_no_explicit_providers() {
        let config = HsmConfig {
            auto_detect: true,
            enable_softhsm: false,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            ..Default::default()
        };

        // Should be valid because auto_detect is true
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_provider_order_default() {
        let config = HsmConfig::default();

        assert_eq!(config.provider_order.len(), 4);
        assert_eq!(config.provider_order[0], "strongbox");
        assert_eq!(config.provider_order[1], "tpm");
        assert_eq!(config.provider_order[2], "yubihsm");
        assert_eq!(config.provider_order[3], "softhsm");
    }

    #[test]
    fn test_get_enabled_providers_respects_order() {
        let config = HsmConfig {
            enable_softhsm: true,
            enable_yubihsm: true,
            enable_tpm: true,
            enable_strongbox: true,
            ..Default::default()
        };

        let providers = config.get_enabled_providers();
        assert_eq!(providers.len(), 4);
        // Should be in provider_order
        assert_eq!(providers[0], "strongbox");
        assert_eq!(providers[1], "tpm");
        assert_eq!(providers[2], "yubihsm");
        assert_eq!(providers[3], "softhsm");
    }

    #[test]
    fn test_get_enabled_providers_empty_when_none() {
        let config = HsmConfig {
            enable_softhsm: false,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            ..Default::default()
        };

        let providers = config.get_enabled_providers();
        assert_eq!(providers.len(), 0);
    }

    #[test]
    fn test_yubihsm_connector_validation_http() {
        let config = HsmConfig {
            yubihsm_connector: Some(std::env::var("YUBIHSM_CONNECTOR_URL").unwrap_or_else(|_| {
                "http://yubihsm-connector.ecosystem.internal:12345".to_string()
            })),
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_yubihsm_connector_validation_https() {
        let config = HsmConfig {
            yubihsm_connector: Some("https://yubihsm.example.com".to_string()),
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_yubihsm_connector_validation_invalid() {
        let config = HsmConfig {
            yubihsm_connector: Some("invalid-url".to_string()),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_custom_softhsm_config() {
        let config = HsmConfig {
            softhsm_config: Some(PathBuf::from("/custom/softhsm2.conf")),
            ..Default::default()
        };

        assert!(config.softhsm_config.is_some());
    }

    #[test]
    fn test_prefer_hardware_setting() {
        let mut config = HsmConfig::default();
        assert!(config.prefer_hardware);

        config.prefer_hardware = false;
        assert!(!config.prefer_hardware);
    }

    #[test]
    fn test_enable_tpm() {
        let config = HsmConfig {
            enable_tpm: true,
            ..Default::default()
        };

        let providers = config.get_enabled_providers();
        assert!(providers.contains(&"tpm".to_string()));
    }

    #[test]
    fn test_enable_strongbox() {
        let config = HsmConfig {
            enable_strongbox: true,
            ..Default::default()
        };

        let providers = config.get_enabled_providers();
        assert!(providers.contains(&"strongbox".to_string()));
    }

    #[test]
    fn test_all_providers_disabled_but_auto_detect() {
        let config = HsmConfig {
            auto_detect: true,
            enable_softhsm: false,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            ..Default::default()
        };

        assert!(config.validate().is_ok());
        let providers = config.get_enabled_providers();
        assert_eq!(providers.len(), 0);
    }
}
