//! HSM Discovery Module
//!
//! Provides functionality for discovering HSMs across different platforms and protocols

use super::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

// Submodules
pub mod cloud_discoverer;
pub mod mobile_discoverer;
pub mod network_discoverer;
pub mod pkcs11_discoverer;
pub mod platform_discoverer;
pub mod software_discoverer;
pub mod usb_discoverer;

// Re-exports
pub use cloud_discoverer::CloudDiscoverer;
pub use mobile_discoverer::MobileDiscoverer;
pub use network_discoverer::NetworkDiscoverer;
pub use pkcs11_discoverer::Pkcs11Discoverer;
pub use platform_discoverer::PlatformDiscoverer;
pub use software_discoverer::SoftwareDiscoverer;
pub use usb_discoverer::UsbDiscoverer;

/// Discovery engine that coordinates all discoverers
#[derive(Debug, Clone)]
pub struct DiscoveryEngine {
    /// PKCS#11 discoverer
    pkcs11_discoverer: Pkcs11Discoverer,
    /// Cloud HSM discoverer
    cloud_discoverer: CloudDiscoverer,
    /// Mobile HSM discoverer
    mobile_discoverer: MobileDiscoverer,
    /// Platform discoverer
    platform_discoverer: PlatformDiscoverer,
    /// Network discoverer
    network_discoverer: NetworkDiscoverer,
    /// USB discoverer
    usb_discoverer: UsbDiscoverer,
    /// Software HSM discoverer
    software_discoverer: SoftwareDiscoverer,
}

impl DiscoveryEngine {
    /// Create a new discovery engine
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        info!("Creating HSM discovery engine");

        Ok(Self {
            pkcs11_discoverer: Pkcs11Discoverer::new()?,
            cloud_discoverer: CloudDiscoverer::new()?,
            mobile_discoverer: MobileDiscoverer::new()?,
            platform_discoverer: PlatformDiscoverer::new()?,
            network_discoverer: NetworkDiscoverer::new()?,
            usb_discoverer: UsbDiscoverer::new()?,
            software_discoverer: SoftwareDiscoverer::new()?,
        })
    }

    /// Discover all HSMs
    ///
    /// Runs all discovery methods in parallel and aggregates results:
    /// - Platform HSMs (local hardware: TPM, Secure Enclave, StrongBox)
    /// - Mobile HSMs (Android/iOS security modules)
    /// - Cloud HSMs (AWS KMS, Azure Key Vault, GCP KMS)
    /// - Network HSMs (mDNS, known endpoints, via Songbird)
    /// - USB HSMs (YubiKey, Nitrokey, etc.)
    /// - Software HSM (always available fallback)
    ///
    /// # Errors
    /// Returns an error if critical discovery failures occur
    pub async fn discover_all(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Starting universal HSM discovery across all providers");

        let mut discovered = Vec::new();

        // ✅ Run all discoverers in sequence (could be parallelized with tokio::join!)
        // Each discoverer is resilient - failures are logged but don't stop discovery

        // 1. Platform HSMs (local hardware)
        match self.platform_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ Platform discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ Platform discovery failed (non-fatal): {}", e);
            }
        }

        // 2. Mobile HSMs (Android StrongBox, iOS Secure Enclave)
        match self.mobile_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ Mobile discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ Mobile discovery failed (non-fatal): {}", e);
            }
        }

        // 3. Cloud HSMs (AWS KMS, Azure, GCP)
        match self.cloud_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ Cloud discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ Cloud discovery failed (non-fatal): {}", e);
            }
        }

        // 4. Network HSMs (mDNS, Songbird, known endpoints)
        match self.network_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ Network discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ Network discovery failed (non-fatal): {}", e);
            }
        }

        // 5. USB HSMs (YubiKey, Nitrokey, etc.)
        match self.usb_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ USB discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ USB discovery failed (non-fatal): {}", e);
            }
        }

        // 6. Software HSM (always available)
        match self.software_discoverer.discover().await {
            Ok(mut hsms) => {
                info!("✓ Software HSM discovery: {} HSMs found", hsms.len());
                discovered.append(&mut hsms);
            }
            Err(e) => {
                warn!("⚠ Software HSM discovery failed (non-fatal): {}", e);
            }
        }

        info!(
            "✅ Discovery complete: {} total HSMs discovered across all providers",
            discovered.len()
        );

        // Return all discovered HSMs
        // Note: Deduplication and prioritization happen in the tier manager
        Ok(discovered)
    }
}

// Note: Default implementation removed - use DiscoveryEngine::new() instead
// Default trait cannot be implemented safely since new() returns Result
// If you need a fallback, handle the Result explicitly:
//   let engine = DiscoveryEngine::new().unwrap_or_else(|e| { /* handle error */ });
//
// impl Default for DiscoveryEngine {
//     fn default() -> Self {
//         Self::new()?
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = DiscoveryEngine::new();
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_discovery() -> Result<(), BearDogError> {
        let engine = DiscoveryEngine::new()?;
        let hsms = engine.discover_all().await?;
        assert!(hsms.is_empty()); // PHASE-2(Discovery): Will find HSMs when probers implemented
        Ok(())
    }
}
