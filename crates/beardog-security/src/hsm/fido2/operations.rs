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
#[expect(
    dead_code,
    reason = "Phase-2 CTAP2 HID integration: remove when first caller lands"
)]
pub async fn send_ctap2_command<D: beardog_hid::HidDevice + ?Sized>(
    _device: &mut D,
    _command: Ctap2Command,
    _payload: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    Err(BearDogError::requires_capability(
        "fido2-ctap2",
        "CTAP2 command protocol requires Phase-2 HID transport integration",
    ))
}

/// Query device information via CTAP2 `GetInfo`
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
#[expect(
    dead_code,
    reason = "Phase-2 CTAP2 HID integration: remove when first caller lands"
)]
pub async fn get_device_info<D: beardog_hid::HidDevice + ?Sized>(
    _device: &mut D,
) -> Result<super::types::Fido2DeviceInfo, BearDogError> {
    Err(BearDogError::requires_capability(
        "fido2-ctap2",
        "CTAP2 GetInfo requires Phase-2 HID transport integration",
    ))
}

/// Generate entropy using hmac-secret extension
///
/// The hmac-secret extension allows deriving cryptographic material from the device.
/// This is perfect for entropy generation.
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
#[expect(
    dead_code,
    reason = "Phase-2 CTAP2 HID integration: remove when first caller lands"
)]
pub async fn generate_entropy_via_hmac_secret<D: beardog_hid::HidDevice + ?Sized>(
    _device: &mut D,
    _size: usize,
) -> Result<Vec<u8>, BearDogError> {
    Err(BearDogError::requires_capability(
        "fido2-ctap2",
        "hmac-secret entropy generation requires Phase-2 HID transport integration",
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
