// SPDX-License-Identifier: AGPL-3.0-or-later

//! CTAP2 transport port (hexagonal architecture).
//!
//! Implementations live in `hid_transport` (hardware) or test mocks.

use beardog_errors::BearDogError;

/// Thin port for CTAP2 device communication.
/// The ONLY part that needs real hardware to exercise end-to-end.
#[async_trait::async_trait]
pub trait Ctap2Transport: Send + Sync {
    /// Send a framed CTAP2 HID message and receive the raw CTAP2 response.
    ///
    /// The `command` buffer is the inner CTAP message: `[CTAP command byte][CBOR payload]`
    /// as produced by [`super::ctap2_protocol`] builders. The transport is responsible for
    /// CTAPHID framing, channel setup, and reassembly.
    async fn send_receive(&mut self, command: &[u8]) -> Result<Vec<u8>, BearDogError>;
}
