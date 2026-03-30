// SPDX-License-Identifier: AGPL-3.0-only

//! Cloud Provider Types - Canonical Definition
//!
//! This module provides the single source of truth for cloud provider identifications
//! used across the BearDog HSM system.
//!
//! ## Design Philosophy
//!
//! While BearDog's architecture favors capability-based discovery over hardcoded vendor
//! names, cloud provider identification remains necessary for:
//! - Cloud HSM discovery and configuration
//! - Region-specific endpoint resolution
//! - Provider-specific API authentication
//! - Telemetry and cost attribution
//!
//! ## Usage
//!
//! ```rust
//! use beardog_types::canonical::hsm_unified::cloud::CloudProvider;
//!
//! let provider = CloudProvider::Aws;
//! let service_name = provider.default_hsm_service_name();
//! ```
//!
//! ## Migration Note
//!
//! This consolidates duplicate CloudProvider definitions from:
//! - `beardog-tunnel/src/universal_hsm_discovery/discovery/cloud_discoverer.rs`
//! - `beardog-tunnel/src/universal_hsm/providers/factory.rs`

use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical cloud provider identification
///
/// This enum provides a unified way to identify cloud providers across the `BearDog` ecosystem.
/// While capability-based discovery is preferred, explicit provider identification is sometimes
/// necessary for discovery, configuration, and telemetry purposes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    /// Amazon Web Services (AWS)
    /// Services: AWS KMS, AWS `CloudHSM`
    Aws,

    /// Microsoft Azure
    /// Services: Azure Key Vault, Azure Managed HSM
    Azure,

    /// Google Cloud Platform (GCP)
    /// Services: Google Cloud KMS, Google Cloud HSM
    Gcp,

    /// Oracle Cloud Infrastructure (OCI)
    /// Services: Oracle Key Management, Oracle Cloud HSM
    Oci,

    /// IBM Cloud
    /// Services: IBM Key Protect, IBM Cloud HSM
    Ibm,

    /// Alibaba Cloud
    /// Services: Alibaba Cloud KMS
    Alibaba,

    /// Custom/Other cloud provider
    Custom {
        /// Provider name
        name: String,
    },
}

impl CloudProvider {
    /// Get the default HSM service name for this provider
    ///
    /// Returns the most common HSM service identifier used by this provider.
    #[must_use]
    pub const fn default_hsm_service_name(&self) -> &'static str {
        match self {
            Self::Aws => "AWS KMS",
            Self::Azure => "Azure Key Vault",
            Self::Gcp => "Google Cloud KMS",
            Self::Oci => "OCI Key Management",
            Self::Ibm => "IBM Key Protect",
            Self::Alibaba => "Alibaba Cloud KMS",
            Self::Custom { .. } => "Custom HSM",
        }
    }

    /// Get common region prefixes for this provider
    ///
    /// Returns typical region identifier prefixes to help with region discovery.
    #[must_use]
    pub const fn region_prefixes(&self) -> &'static [&'static str] {
        match self {
            Self::Aws => &["us-", "eu-", "ap-", "ca-", "sa-"],
            Self::Azure => &["eastus", "westus", "northeurope", "westeurope"],
            Self::Gcp => &["us-", "europe-", "asia-"],
            Self::Oci => &["us-", "eu-", "ap-"],
            Self::Ibm => &["us-", "eu-", "ap-"],
            Self::Alibaba => &["cn-", "us-", "eu-"],
            Self::Custom { .. } => &[],
        }
    }

    /// Check if this provider is a major cloud platform
    ///
    /// Returns true for AWS, Azure, and GCP (the "big three").
    #[must_use]
    pub const fn is_major_platform(&self) -> bool {
        matches!(self, Self::Aws | Self::Azure | Self::Gcp)
    }

    /// Get provider identifier string (lowercase)
    ///
    /// Useful for configuration keys, environment variables, etc.
    #[must_use]
    pub const fn identifier(&self) -> &'static str {
        match self {
            Self::Aws => "aws",
            Self::Azure => "azure",
            Self::Gcp => "gcp",
            Self::Oci => "oci",
            Self::Ibm => "ibm",
            Self::Alibaba => "alibaba",
            Self::Custom { .. } => "custom",
        }
    }

    /// Parse provider from string identifier
    ///
    /// Accepts various string formats (case-insensitive).
    pub fn from_identifier(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "aws" | "amazon" | "awskms" => Some(Self::Aws),
            "azure" | "microsoft" | "azurekeyvault" => Some(Self::Azure),
            "gcp" | "google" | "googlecloud" | "gcpkms" => Some(Self::Gcp),
            "oci" | "oracle" => Some(Self::Oci),
            "ibm" => Some(Self::Ibm),
            "alibaba" | "alibabacloud" => Some(Self::Alibaba),
            _ => None, // Return None for unrecognized; caller can create Custom if needed
        }
    }
}

impl fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aws => write!(f, "AWS"),
            Self::Azure => write!(f, "Azure"),
            Self::Gcp => write!(f, "GCP"),
            Self::Oci => write!(f, "OCI"),
            Self::Ibm => write!(f, "IBM Cloud"),
            Self::Alibaba => write!(f, "Alibaba Cloud"),
            Self::Custom { name } => write!(f, "Custom ({name})"),
        }
    }
}

/// Cloud HSM service types
///
/// Identifies specific HSM services within a cloud provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudHsmService {
    /// AWS Key Management Service
    AwsKms,
    /// AWS `CloudHSM` (dedicated hardware)
    AwsCloudHsm,

    /// Azure Key Vault (standard)
    AzureKeyVault,
    /// Azure Managed HSM (dedicated hardware)
    AzureManagedHsm,

    /// Google Cloud Key Management Service
    GcpKms,
    /// Google Cloud HSM (dedicated hardware)
    GcpCloudHsm,

    /// Oracle Key Management
    OciKeyManagement,

    /// IBM Key Protect
    IbmKeyProtect,

    /// Alibaba Cloud KMS
    AlibabaKms,

    /// Custom cloud HSM service
    Custom {
        /// Provider
        provider: CloudProvider,
        /// Service name
        service_name: String,
    },
}

impl CloudHsmService {
    /// Get the cloud provider for this service
    #[must_use]
    pub fn provider(&self) -> CloudProvider {
        match self {
            Self::AwsKms | Self::AwsCloudHsm => CloudProvider::Aws,
            Self::AzureKeyVault | Self::AzureManagedHsm => CloudProvider::Azure,
            Self::GcpKms | Self::GcpCloudHsm => CloudProvider::Gcp,
            Self::OciKeyManagement => CloudProvider::Oci,
            Self::IbmKeyProtect => CloudProvider::Ibm,
            Self::AlibabaKms => CloudProvider::Alibaba,
            Self::Custom { provider, .. } => provider.clone(),
        }
    }

    /// Check if this is a dedicated HSM service (vs. shared/multi-tenant)
    #[must_use]
    pub const fn is_dedicated_hsm(&self) -> bool {
        matches!(
            self,
            Self::AwsCloudHsm | Self::AzureManagedHsm | Self::GcpCloudHsm
        )
    }
}

impl fmt::Display for CloudHsmService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwsKms => write!(f, "AWS KMS"),
            Self::AwsCloudHsm => write!(f, "AWS CloudHSM"),
            Self::AzureKeyVault => write!(f, "Azure Key Vault"),
            Self::AzureManagedHsm => write!(f, "Azure Managed HSM"),
            Self::GcpKms => write!(f, "Google Cloud KMS"),
            Self::GcpCloudHsm => write!(f, "Google Cloud HSM"),
            Self::OciKeyManagement => write!(f, "OCI Key Management"),
            Self::IbmKeyProtect => write!(f, "IBM Key Protect"),
            Self::AlibabaKms => write!(f, "Alibaba Cloud KMS"),
            Self::Custom { service_name, .. } => write!(f, "{service_name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_provider_identifier() {
        assert_eq!(CloudProvider::Aws.identifier(), "aws");
        assert_eq!(CloudProvider::Azure.identifier(), "azure");
        assert_eq!(CloudProvider::Gcp.identifier(), "gcp");
    }

    #[test]
    fn test_cloud_provider_from_identifier() {
        assert_eq!(
            CloudProvider::from_identifier("aws"),
            Some(CloudProvider::Aws)
        );
        assert_eq!(
            CloudProvider::from_identifier("AWS"),
            Some(CloudProvider::Aws)
        );
        assert_eq!(
            CloudProvider::from_identifier("amazon"),
            Some(CloudProvider::Aws)
        );
        assert_eq!(
            CloudProvider::from_identifier("azure"),
            Some(CloudProvider::Azure)
        );
        assert_eq!(
            CloudProvider::from_identifier("gcp"),
            Some(CloudProvider::Gcp)
        );
        assert_eq!(
            CloudProvider::from_identifier("google"),
            Some(CloudProvider::Gcp)
        );
    }

    #[test]
    fn test_major_platform() {
        assert!(CloudProvider::Aws.is_major_platform());
        assert!(CloudProvider::Azure.is_major_platform());
        assert!(CloudProvider::Gcp.is_major_platform());
        assert!(!CloudProvider::Oci.is_major_platform());
        assert!(!CloudProvider::Ibm.is_major_platform());
    }

    #[test]
    fn test_cloud_hsm_service_provider() {
        assert_eq!(CloudHsmService::AwsKms.provider(), CloudProvider::Aws);
        assert_eq!(
            CloudHsmService::AzureKeyVault.provider(),
            CloudProvider::Azure
        );
        assert_eq!(CloudHsmService::GcpKms.provider(), CloudProvider::Gcp);
    }

    #[test]
    fn test_dedicated_hsm() {
        assert!(!CloudHsmService::AwsKms.is_dedicated_hsm());
        assert!(CloudHsmService::AwsCloudHsm.is_dedicated_hsm());
        assert!(!CloudHsmService::AzureKeyVault.is_dedicated_hsm());
        assert!(CloudHsmService::AzureManagedHsm.is_dedicated_hsm());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", CloudProvider::Aws), "AWS");
        assert_eq!(format!("{}", CloudHsmService::AwsKms), "AWS KMS");
        assert_eq!(
            format!("{}", CloudHsmService::AzureKeyVault),
            "Azure Key Vault"
        );
    }
}
