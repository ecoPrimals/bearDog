// SPDX-License-Identifier: AGPL-3.0-or-later

//! Vendor-specific adapter configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Vendor-specific adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendorConfig {
    /// KMS configuration
    pub kms: KmsConfig,

    /// Cloud provider configurations
    pub cloud_providers: HashMap<String, CloudProviderConfig>,

    /// Enable vendor abstraction
    pub abstraction_enabled: bool,
}

/// KMS (Key Management Service) configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KmsConfig {
    /// KMS provider type (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub provider: Arc<str>,

    /// KMS endpoint
    pub endpoint: Option<String>,

    /// KMS region
    pub region: Option<String>,

    /// KMS key ID
    pub key_id: Option<String>,

    /// Enable hardware backing
    pub hardware_backed: bool,
}

/// Cloud provider specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudProviderConfig {
    /// Provider name (AWS, GCP, Azure, etc.) - Arc for fast cloning
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub name: Arc<str>,

    /// Provider region (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub region: Arc<str>,

    /// Authentication configuration
    pub auth: HashMap<String, String>,

    /// Provider-specific settings
    pub settings: HashMap<String, String>,
}

impl Default for VendorConfig {
    fn default() -> Self {
        Self {
            kms: KmsConfig::default(),
            cloud_providers: HashMap::new(),
            abstraction_enabled: true,
        }
    }
}

impl Default for KmsConfig {
    fn default() -> Self {
        Self {
            provider: Arc::from("software"),
            endpoint: None,
            region: None,
            key_id: None,
            hardware_backed: false,
        }
    }
}
