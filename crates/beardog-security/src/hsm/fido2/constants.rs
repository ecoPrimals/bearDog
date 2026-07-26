// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2/CTAP2 Protocol Constants
//!
//! This module defines constants for the FIDO2 CTAP2 protocol implementation.
//! All magic numbers are centralized here for maintainability and clarity.

/// Maximum number of attempts to read keepalive packets before timing out.
/// CTAP2 spec allows up to 30s for user presence — at 200ms per poll this is 150 attempts.
pub const MAX_KEEPALIVE_ATTEMPTS: usize = 150;

/// HID read poll interval in milliseconds (sleep between non-blocking reads)
pub const HID_READ_TIMEOUT_MS: u64 = 200;

/// Post-CTAPHID_INIT settle time before sending commands (milliseconds)
pub const CTAPHID_CHANNEL_SETTLE_MS: u64 = 50;

/// Standard HID packet size in bytes
pub const HID_PACKET_SIZE: usize = 64;

/// Minimum HID response header size (`CID` + `CMD` + `LEN_H` + `LEN_L` + `DATA`)
pub const HID_MIN_RESPONSE_SIZE: usize = 5;

/// Maximum response preview size for debug logging (bytes)
pub const DEBUG_PREVIEW_SIZE: usize = 32;

/// CTAP2 HID Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtapHidCommand {
    /// Keepalive packet indicator
    Keepalive = 0xBB,
    /// Error packet indicator
    Error = 0xBF,
}

impl CtapHidCommand {
    /// Convert command to byte representation
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
