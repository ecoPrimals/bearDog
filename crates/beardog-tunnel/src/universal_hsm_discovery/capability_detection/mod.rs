// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Capability Detection Module
//!
//! Provides functionality for detecting and probing HSM capabilities
//! across different providers and platforms.

use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities, HumanEntropyCapabilities,
    KeyGenerationCapabilities, KeyManagementCapabilities, PerformanceCapabilities,
    SecurityCapabilities,
};
use super::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, error, info, warn};

// Submodules
pub mod cloud_kms_prober;
pub mod mobile_hsm_prober;
pub mod performance_benchmarker;
pub mod pkcs11_prober;
pub mod software_hsm_prober;

use cloud_kms_prober::CloudKmsCapabilityProber;
use mobile_hsm_prober::MobileHsmCapabilityProber;
use performance_benchmarker::PerformanceBenchmarker;
use pkcs11_prober::Pkcs11CapabilityProber;
use software_hsm_prober::SoftwareHsmCapabilityProber;

/// Capability detector for HSMs
#[derive(Debug, Clone)]
pub struct CapabilityDetector {
    /// PKCS#11 capability prober
    pkcs11_prober: Pkcs11CapabilityProber,
    /// Cloud KMS capability prober
    cloud_kms_prober: CloudKmsCapabilityProber,
    /// Mobile HSM capability prober
    mobile_hsm_prober: MobileHsmCapabilityProber,
    /// Software HSM capability prober
    software_hsm_prober: SoftwareHsmCapabilityProber,
    /// Performance benchmarker
    performance_benchmarker: PerformanceBenchmarker,
}

impl CapabilityDetector {
    /// Create a new capability detector
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        info!("Creating HSM capability detector");
        
        Ok(Self {
            pkcs11_prober: Pkcs11CapabilityProber::new()?,
            cloud_kms_prober: CloudKmsCapabilityProber::new()?,
            mobile_hsm_prober: MobileHsmCapabilityProber::new()?,
            software_hsm_prober: SoftwareHsmCapabilityProber::new()?,
            performance_benchmarker: PerformanceBenchmarker::new()?,
        })
    }

    /// Detect capabilities for a given HSM
    ///
    /// # Errors
    /// Returns an error if detection fails
    pub async fn detect_capabilities(
        &self,
        hsm_type: &HsmType,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("Detecting capabilities for HSM type: {:?}", hsm_type);

        match hsm_type {
            HsmType::Pkcs11 => self.pkcs11_prober.probe_capabilities().await,
            HsmType::Cloud => self.cloud_kms_prober.probe_capabilities().await,
            HsmType::Smartphone => self.mobile_hsm_prober.probe_capabilities().await,
            HsmType::Software => self.software_hsm_prober.probe_capabilities().await,
            _ => {
                warn!("Unsupported HSM type for capability detection: {:?}", hsm_type);
                Ok(HsmCapabilities::default())
            }
        }
    }

    /// Benchmark HSM performance
    ///
    /// # Errors
    /// Returns an error if benchmarking fails
    pub async fn benchmark_performance(
        &self,
        hsm_type: &HsmType,
    ) -> Result<PerformanceCapabilities, BearDogError> {
        debug!("Benchmarking performance for HSM type: {:?}", hsm_type);
        self.performance_benchmarker
            .benchmark(hsm_type)
            .await
    }

    /// Detect all capabilities including performance
    ///
    /// # Errors
    /// Returns an error if detection fails
    pub async fn detect_all_capabilities(
        &self,
        hsm_type: &HsmType,
    ) -> Result<HsmCapabilities, BearDogError> {
        let mut capabilities = self.detect_capabilities(hsm_type).await?;
        
        // Add performance metrics
        if let Ok(perf) = self.benchmark_performance(hsm_type).await {
            capabilities.performance = perf;
        }

        Ok(capabilities)
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used ? which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_detector_creation() {
        let detector = CapabilityDetector::new();
        assert!(detector.is_ok());
    }

    #[tokio::test]
    async fn test_software_hsm_detection() -> Result<(), BearDogError> {
        let detector = CapabilityDetector::new()?;
        let capabilities = detector.detect_capabilities(&HsmType::Software).await?;
        assert!(capabilities.supports_key_generation);
        Ok(())
    }
}
