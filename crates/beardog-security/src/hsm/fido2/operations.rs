// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2 Cryptographic Operations
//!
//! Low-level CTAP2 protocol operations for FIDO2 devices.
//! Phase 2 stubs - implementations pending.

use beardog_errors::BearDogError;

/// CTAP2 command codes (protocol layer, distinct from [`crate::hsm::fido2::ctap2::Ctap2Command`]).
#[repr(u8)]
pub enum Ctap2Command {
    /// Create a new credential (attestation)
    MakeCredential = 0x01,
    /// Authenticate with an existing credential
    GetAssertion = 0x02,
    /// Read authenticator metadata
    GetInfo = 0x04,
    /// PIN setup / verification
    ClientPin = 0x06,
    /// Factory reset
    Reset = 0x07,
    /// Next assertion in multi-credential authentication
    GetNextAssertion = 0x08,
    /// Credential management (resident keys)
    CredentialManagement = 0x0A,
}

/// Send a CTAP2 command to a FIDO2 device
///
/// # Arguments
///
/// * `device` - HID device handle
/// * `command` - CTAP2 command code
/// * `payload` - CBOR-encoded command payload
///
/// # Returns
///
/// CBOR-encoded response from the device
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
#[allow(
    dead_code,
    reason = "pub CTAP2 API reserved for Phase-2 HID integration"
)]
pub async fn send_ctap2_command(
    _device: &mut Box<dyn beardog_hid::HidDevice>,
    _command: Ctap2Command,
    _payload: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    // PHASE-2(CTAP2): Implement CTAP2 protocol operations
    // 1. Format command packet (command byte + CBOR payload)
    // 2. Send via HID
    // 3. Receive response
    // 4. Parse CTAP2 status code
    // 5. Return CBOR payload or error

    Err(BearDogError::system(
        "CTAP2 command sending not yet implemented (Phase 1 in progress)".to_string(),
    ))
}

/// Query device information via CTAP2 `GetInfo`
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
#[allow(
    dead_code,
    reason = "pub CTAP2 API reserved for Phase-2 HID integration"
)]
pub async fn get_device_info(
    _device: &mut Box<dyn beardog_hid::HidDevice>,
) -> Result<super::types::Fido2DeviceInfo, BearDogError> {
    // PHASE-2(CTAP2): Send GetInfo command and parse response
    Err(BearDogError::system(
        "CTAP2 GetInfo not yet implemented (Phase 1 in progress)".to_string(),
    ))
}

/// Generate entropy using hmac-secret extension
///
/// The hmac-secret extension allows deriving cryptographic material from the device.
/// This is perfect for entropy generation.
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
#[allow(
    dead_code,
    reason = "pub CTAP2 API reserved for Phase-2 HID integration"
)]
pub async fn generate_entropy_via_hmac_secret(
    _device: &mut Box<dyn beardog_hid::HidDevice>,
    _size: usize,
) -> Result<Vec<u8>, BearDogError> {
    // PHASE-2(CTAP2): Implement hmac-secret entropy generation
    // 1. Create a credential with hmac-secret extension
    // 2. Get assertion with salt to derive key material
    // 3. Use derived material as entropy
    // 4. Optionally hash/expand to requested size

    Err(BearDogError::system(
        "hmac-secret entropy generation not yet implemented (Phase 1 in progress)".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctap2_command_codes() {
        assert_eq!(Ctap2Command::MakeCredential as u8, 0x01);
        assert_eq!(Ctap2Command::GetAssertion as u8, 0x02);
        assert_eq!(Ctap2Command::GetInfo as u8, 0x04);
    }
}
