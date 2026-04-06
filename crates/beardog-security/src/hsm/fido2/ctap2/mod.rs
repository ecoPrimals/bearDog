// SPDX-License-Identifier: AGPL-3.0-or-later

//! CTAP2 Protocol Implementation
//!
//! Implements the Client to Authenticator Protocol 2 (CTAP2) for FIDO2 devices.
//!
//! # Protocol Overview
//!
//! CTAP2 uses HID transport with CBOR (Concise Binary Object Representation) encoding.
//! Commands are sent as:
//! ```text
//! [Command Byte] [CBOR Payload]
//! ```
//!
//! Responses are:
//! ```text
//! [Status Byte] [CBOR Payload]
//! ```

pub mod parse;
pub mod transport;
pub mod types;

pub use parse::parse_get_info_cbor;
pub use types::*;

#[cfg(feature = "fido2")]
pub use transport::{ctaphid_init, send_ctap2_command};

use beardog_errors::BearDogError;
#[cfg(feature = "fido2")]
use tracing::info;

/// Query device information via CTAP2 `GetInfo` (Pure Rust)
///
/// This is the first command you should send to any FIDO2 device.
/// It returns capabilities, supported features, and device metadata.
///
/// # Note
///
/// This function automatically initializes the CTAPHID channel if needed.
///
/// # Errors
///
/// Returns [`BearDogError`] on channel init failure, CTAP2 command failure, or CBOR parse errors.
#[cfg(feature = "fido2")]
pub async fn ctap2_get_info(
    device: &mut Box<dyn beardog_hid::HidDevice>,
) -> Result<Ctap2DeviceInfo, BearDogError> {
    info!("🔍 Querying device capabilities (CTAP2 GetInfo)...");

    let cid = ctaphid_init(device).await?;
    let response_bytes = send_ctap2_command(device, cid, Ctap2Command::GetInfo, &[]).await?;
    let info = parse_get_info_cbor(&response_bytes)?;

    info!("✅ Device capabilities:");
    info!("   Versions: {}", info.versions.join(", "));
    info!("   Extensions: {}", info.extensions.join(", "));
    info!("   Options: {:?}", info.options);

    Ok(info)
}

#[cfg(test)]
mod tests;
