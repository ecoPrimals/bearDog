// SPDX-License-Identifier: AGPL-3.0-only

//! FIDO2 Cryptographic Operations
//!
//! Low-level CTAP2 protocol operations for FIDO2 devices.

use beardog_errors::BearDogError;

/// CTAP2 command codes
#[repr(u8)]
pub enum Ctap2Command {
    MakeCredential = 0x01,
    GetAssertion = 0x02,
    GetInfo = 0x04,
    ClientPin = 0x06,
    Reset = 0x07,
    GetNextAssertion = 0x08,
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

/// Query device information via CTAP2 GetInfo
///
/// Note: Phase 2 implementation pending
#[cfg(feature = "fido2")]
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
