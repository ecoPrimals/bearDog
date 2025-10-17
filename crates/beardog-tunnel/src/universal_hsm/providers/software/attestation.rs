//! Software HSM attestation functionality

use beardog_errors::BearDogError;

/// Software attestation stub
///
/// NOTE: This is a minimal stub implementation.
/// Full software attestation implementation to be added later.
pub struct SoftwareAttestation;

impl SoftwareAttestation {
    /// Create new software attestation instance
    pub fn new() -> Self {
        Self
    }

    /// Perform software attestation
    ///
    /// This is a stub that always succeeds.
    pub fn attest(&self) -> Result<Vec<u8>, BearDogError> {
        // TODO: Implement actual software attestation
        Ok(vec![])
    }
}

impl Default for SoftwareAttestation {
    fn default() -> Self {
        Self::new()
    }
}
