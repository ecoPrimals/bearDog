//! Software HSM Capability Prober
//!
//! Provides capability detection for software-based HSMs

use super::super::*;
use crate::tunnel::hsm::types::capability::HsmCapabilities;
use beardog_errors::BearDogError;
use tracing::debug;

/// Software HSM capability prober
#[derive(Debug, Clone)]
pub struct SoftwareHsmCapabilityProber;

impl SoftwareHsmCapabilityProber {
    /// Create new Software HSM capability prober
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }

    /// Probe capabilities
    ///
    /// # Errors
    /// Returns an error if probing fails
    pub async fn probe_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
        debug!("Probing software HSM capabilities");
        
        // TODO: Implement actual software HSM capability detection
        Ok(HsmCapabilities::default())
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used ? which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prober_creation() {
        let prober = SoftwareHsmCapabilityProber::new();
        assert!(prober.is_ok());
    }
}
