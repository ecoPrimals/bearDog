// SPDX-License-Identifier: AGPL-3.0-only

//! HSM provider implementations
//!
//! This module contains the main HSM provider enum and implementations.

use super::configs::{AndroidHsmConfig, IosHsmConfig, SoftwareHsmConfig, StrongBoxHsmConfig};
use crate::constants::time;
use crate::canonical::providers_unified::traits::consolidated::{
    ConsolidatedProvider, ProviderInfo, ProviderHealth, ProviderMetrics, 
    ProviderType, HealthStatus, ProviderCapability
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::SystemTime;

/// **Unified HSM Provider**
///
/// Single provider that handles all HSM operations across different platforms and implementations.
/// This eliminates the need for multiple scattered HSM provider structs.
#[derive(Debug, Clone)]
pub enum HsmUnifiedProvider {
    /// Android HSM with `StrongBox` support
    Android(AndroidHsmConfig),
    /// iOS HSM with Secure Enclave support
    Ios(IosHsmConfig),
    /// Software HSM for development and testing
    Software(SoftwareHsmConfig),
    /// Dedicated `StrongBox` HSM
    StrongBox(StrongBoxHsmConfig),
}

impl ConsolidatedProvider for HsmUnifiedProvider {
    type Error = BearDogError;
    type Config = AndroidHsmConfig; // Simplified for now
    type Data = Vec<u8>;
    fn provider_info(&self) -> ProviderInfo {
        match self {
            HsmUnifiedProvider::Android(_) => ProviderInfo {
                id: "android_hsm".to_string(),
                name: "Android HSM".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Hsm,
                supported_capabilities: vec![
                    "KeyGeneration".to_string(),
                    "Encryption".to_string(),
                    "Signing".to_string(),
                ],
            },
            HsmUnifiedProvider::Ios(_) => ProviderInfo {
                id: "ios_hsm".to_string(),
                name: "iOS HSM".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Hsm,
                supported_capabilities: vec![
                    "KeyGeneration".to_string(),
                    "Encryption".to_string(),
                    "Signing".to_string(),
                ],
            },
            HsmUnifiedProvider::Software(_) => ProviderInfo {
                id: "software_hsm".to_string(),
                name: "Software HSM".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Hsm,
                supported_capabilities: vec![
                    "KeyGeneration".to_string(),
                    "Encryption".to_string(),
                    "Signing".to_string(),
                ],
            },
            HsmUnifiedProvider::StrongBox(_) => ProviderInfo {
                id: "strongbox_hsm".to_string(),
                name: "StrongBox HSM".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Hsm,
                supported_capabilities: vec![
                    "KeyGeneration".to_string(),
                    "Encryption".to_string(),
                    "Signing".to_string(),
                ],
            },
        }
    }

    async fn health_check(&self) -> Result<ProviderHealth, Self::Error> {
        Ok(ProviderHealth {
            status: HealthStatus::Healthy,
            last_check: SystemTime::now(),
            error_message: None,
            uptime_seconds: time::SECONDS_PER_HOUR, // 1 hour example
            response_time_ms: 10,
        })
    }

    fn provider_version(&self) -> &'static str {
        "1.0.0"
    }

    fn capabilities(&self) -> Vec<ProviderCapability> {
        vec![
            ProviderCapability {
                name: "KeyGeneration".to_string(),
                version: "1.0.0".to_string(),
                description: "Generate cryptographic keys".to_string(),
                required_config: vec!["key_type".to_string(), "key_size".to_string()],
            },
            ProviderCapability {
                name: "Encryption".to_string(),
                version: "1.0.0".to_string(),
                description: "Encrypt and decrypt data".to_string(),
                required_config: vec!["algorithm".to_string()],
            },
            ProviderCapability {
                name: "Signing".to_string(),
                version: "1.0.0".to_string(),
                description: "Create and verify digital signatures".to_string(),
                required_config: vec!["signature_algorithm".to_string()],
            },
        ]
    }

    async fn initialize(
        &mut self,
        _config: Self::Config,
    ) -> Result<(), Self::Error> { Ok(()) }

    async fn metrics(&self) -> Result<ProviderMetrics, Self::Error> {
        Ok(ProviderMetrics {
            timestamp: SystemTime::now(),
            metrics: HashMap::new(),
            metadata: HashMap::new(),
        })
    }

    async fn shutdown(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn validate_config(_config: &Self::Config) -> Result<(), Self::Error> {
        Ok(())
    }
} 