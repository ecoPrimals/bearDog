// SPDX-License-Identifier: AGPL-3.0-or-later

//! CTAPHID framing and CTAP2 command I/O over HID.

use super::super::constants::{
    DEBUG_PREVIEW_SIZE, HID_MIN_RESPONSE_SIZE, HID_PACKET_SIZE, HID_READ_TIMEOUT_MS,
    MAX_KEEPALIVE_ATTEMPTS,
};
use super::types::{Ctap2Command, Ctap2Status, CtapHidCommand};
use beardog_errors::BearDogError;
use tracing::{debug, info, warn};

/// Initialize CTAPHID channel (Pure Rust) and get channel ID
///
/// This MUST be called before sending any CTAP2 commands.
/// Returns the channel ID (CID) to use for all subsequent commands.
///
/// # Errors
///
/// Returns [`BearDogError`] on HID I/O failure, timeout, or invalid INIT response.
#[cfg(feature = "fido2")]
pub async fn ctaphid_init<D: beardog_hid::HidDevice + ?Sized>(
    device: &mut D,
) -> Result<u32, BearDogError> {
    use rand::RngCore;

    info!("🔗 Initializing CTAPHID channel...");

    let mut nonce = [0u8; 8];
    rand::rng().fill_bytes(&mut nonce);

    debug!("Generated nonce: {:?}", hex::encode(nonce));

    // Build CTAPHID_INIT packet
    // Format: [CID: 0xFFFFFFFF] [CMD: 0x86] [LEN_H] [LEN_L] [NONCE (8 bytes)]
    let mut packet = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Broadcast CID
    packet.push(CtapHidCommand::Init as u8); // INIT command
    packet.push(0x00); // Length high byte (8 bytes)
    packet.push(0x08); // Length low byte
    packet.extend_from_slice(&nonce);

    // Pad to 64 bytes
    while packet.len() < 64 {
        packet.push(0);
    }

    // Send packet
    device
        .write(&packet)
        .await
        .map_err(|e| BearDogError::system(format!("CTAPHID_INIT write failed: {e}")))?;

    debug!("📤 Sent CTAPHID_INIT");

    // Read response (with poll loop for non-blocking HID)
    let mut response = vec![0u8; 64];
    let mut bytes_read = 0;
    for _poll in 0..25 {
        bytes_read = device
            .read(&mut response)
            .await
            .map_err(|e| BearDogError::system(format!("CTAPHID_INIT read failed: {e}")))?;
        if bytes_read > 0 {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(HID_READ_TIMEOUT_MS)).await;
    }

    if bytes_read == 0 {
        return Err(BearDogError::system("CTAPHID_INIT timeout".to_string()));
    }

    debug!("📥 Received {} bytes response", bytes_read);

    // Parse response
    // Format: [CID (4)] [CMD] [LEN_H] [LEN_L] [NONCE (8)] [NEW_CID (4)] [PROTOCOL_VERSION] [...]
    if bytes_read < 17 {
        return Err(BearDogError::system(format!(
            "CTAPHID_INIT response too short: {bytes_read} bytes"
        )));
    }

    // Check command byte
    if response[4] != CtapHidCommand::Init as u8 {
        return Err(BearDogError::system(format!(
            "Unexpected response command: 0x{:02X}",
            response[4]
        )));
    }

    // Verify nonce echo
    let echoed_nonce = &response[7..15];
    if echoed_nonce != nonce {
        return Err(BearDogError::system(
            "CTAPHID_INIT nonce mismatch".to_string(),
        ));
    }

    // Extract new channel ID (4 bytes after nonce)
    let cid = u32::from_be_bytes([response[15], response[16], response[17], response[18]]);

    info!("✅ Channel initialized: CID = 0x{:08X}", cid);

    // Give device a moment to process channel initialization
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    Ok(cid)
}

/// Send a CTAP2 command to a FIDO2 device using an established channel
///
/// # Protocol
///
/// CTAP2 over HID uses the following packet format:
/// - Initialization packet: \[CID (4 bytes)] \[CMD] \[BCNTH] \[BCNTL] \[DATA (up to 57 bytes)]
/// - Continuation packets: \[CID (4 bytes)] \[SEQ] \[DATA (up to 59 bytes)]
///
/// Uses `CTAPHID_CBOR` (0x90) command which directly carries CTAP2 commands.
///
/// # Errors
///
/// Returns [`BearDogError`] on HID I/O failure, device error packets, timeout, or CTAP2 error status.
#[cfg(feature = "fido2")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "CTAP2 HID payload length fits u16 per protocol framing"
)]
pub async fn send_ctap2_command<D: beardog_hid::HidDevice + ?Sized>(
    device: &mut D,
    cid: u32,
    command: Ctap2Command,
    payload: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    debug!(
        "Sending CTAP2 command: {:?} ({} bytes payload) on CID 0x{:08X}",
        command,
        payload.len(),
        cid
    );

    // Build CTAP2 packet
    let mut packet = Vec::with_capacity(1 + payload.len());
    packet.push(command as u8);
    packet.extend_from_slice(payload);

    debug!("CTAP2 packet: {:02x?}", &packet);

    // Wrap in CTAPHID_CBOR frame
    // Format: [CID (4 bytes)] [CMD: 0x90] [LEN_H] [LEN_L] [DATA...]
    // CTAPHID_CBOR (0x90) is the correct channel for CTAP2 commands (ClientPIN, etc).
    let cid_bytes = cid.to_be_bytes();
    let mut hid_packet = vec![cid_bytes[0], cid_bytes[1], cid_bytes[2], cid_bytes[3]];
    hid_packet.push(CtapHidCommand::Cbor as u8); // CTAPHID_CBOR

    let len = packet.len() as u16;
    hid_packet.push((len >> 8) as u8); // Length high byte
    hid_packet.push((len & 0xFF) as u8); // Length low byte

    hid_packet.extend_from_slice(&packet);

    // Pad to HID report size (64 bytes)
    while hid_packet.len() < 64 {
        hid_packet.push(0);
    }

    debug!("HID packet (first 16 bytes): {:02x?}", &hid_packet[..16]);

    // Send the packet
    device
        .write(&hid_packet)
        .await
        .map_err(|e| BearDogError::system(format!("HID write failed: {e}")))?;

    debug!("✅ Sent {} bytes to device", hid_packet.len());

    // Read response with multiple attempts (device might send keepalive)
    debug!("📥 Reading response (with retry for keepalive)...");

    let mut total_response = Vec::new();

    for attempt in 1..=MAX_KEEPALIVE_ATTEMPTS {
        let mut response_buf = vec![0u8; HID_PACKET_SIZE];
        let bytes_read = device
            .read(&mut response_buf)
            .await
            .map_err(|e| BearDogError::system(format!("HID read failed: {e}")))?;

        if bytes_read == 0 {
            debug!(
                "   Attempt {}/{}: No data yet — polling in {}ms",
                attempt, MAX_KEEPALIVE_ATTEMPTS, HID_READ_TIMEOUT_MS
            );
            tokio::time::sleep(tokio::time::Duration::from_millis(HID_READ_TIMEOUT_MS)).await;
            continue;
        }

        debug!(
            "   Attempt {}/{}: Got {} bytes",
            attempt, MAX_KEEPALIVE_ATTEMPTS, bytes_read
        );
        debug!(
            "   Data: {:02x?}",
            &response_buf[..bytes_read.min(DEBUG_PREVIEW_SIZE)]
        );

        // Check command byte
        if bytes_read >= HID_MIN_RESPONSE_SIZE {
            let response_cmd = response_buf[4];

            // Check for keepalive (authenticator still processing, awaiting user touch)
            if response_cmd == CtapHidCommand::Keepalive.as_u8() {
                debug!("   📡 Keepalive — authenticator awaiting user presence");
                tokio::time::sleep(tokio::time::Duration::from_millis(HID_READ_TIMEOUT_MS)).await;
                continue;
            }

            // Check for error
            if response_cmd == CtapHidCommand::Error.as_u8() {
                warn!("   ❌ Device returned error packet");
                if bytes_read >= 8 {
                    let error_code = response_buf[7];
                    return Err(BearDogError::system(format!(
                        "Device error: 0x{error_code:02X}"
                    )));
                }
            }

            // Got actual response
            total_response = response_buf[..bytes_read].to_vec();
            debug!("✅ Got response after {} attempt(s)", attempt);
            break;
        }
    }

    if total_response.is_empty() {
        warn!(
            "⏱️  Device timeout - no response after {} attempts",
            MAX_KEEPALIVE_ATTEMPTS
        );
        return Err(BearDogError::system(
            "Device timeout - no response received".to_string(),
        ));
    }

    let bytes_read = total_response.len();
    let response_buf = total_response;

    // Parse HID response
    // Format: [CID (4)] [CMD] [LEN_H] [LEN_L] [DATA...]
    if bytes_read < 7 {
        return Err(BearDogError::system(format!(
            "Response too short: {bytes_read} bytes"
        )));
    }

    // Extract length
    let response_len = ((response_buf[5] as usize) << 8) | (response_buf[6] as usize);

    // Extract CTAP2 response (skip HID header)
    let ctap_response = &response_buf[7..std::cmp::min(7 + response_len, bytes_read)];

    if ctap_response.is_empty() {
        return Err(BearDogError::system("Empty CTAP2 response".to_string()));
    }

    // First byte is status code
    let status = Ctap2Status::from_byte(ctap_response[0]);
    debug!("CTAP2 Status: {:?} (0x{:02X})", status, ctap_response[0]);

    if !status.is_success() {
        return Err(BearDogError::system(format!(
            "CTAP2 error: {} (0x{:02X})",
            status.to_error_message(),
            ctap_response[0]
        )));
    }

    // Return payload (everything after status byte)
    Ok(ctap_response[1..].to_vec())
}
