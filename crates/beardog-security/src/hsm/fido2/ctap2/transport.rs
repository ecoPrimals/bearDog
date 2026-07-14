// SPDX-License-Identifier: AGPL-3.0-or-later

//! CTAPHID framing and CTAP2 command I/O over HID.

use super::super::constants::{HID_PACKET_SIZE, HID_READ_TIMEOUT_MS, MAX_KEEPALIVE_ATTEMPTS};
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

    // Send via CTAPHID_CBOR with multi-packet framing.
    // Init packet: [CID(4)][0x90][BCNTH][BCNTL][payload ≤57]
    // Continuation: [CID(4)][SEQ][payload ≤59]
    let cid_bytes = cid.to_be_bytes();
    let total_len = packet.len() as u16;

    // Build and send initialization packet (up to 57 bytes of payload)
    let first_payload_max = HID_PACKET_SIZE - 7; // 57
    let first_take = packet.len().min(first_payload_max);

    let mut init_pkt = Vec::with_capacity(HID_PACKET_SIZE);
    init_pkt.extend_from_slice(&cid_bytes);
    init_pkt.push(CtapHidCommand::Cbor as u8);
    init_pkt.push((total_len >> 8) as u8);
    init_pkt.push((total_len & 0xFF) as u8);
    init_pkt.extend_from_slice(&packet[..first_take]);
    while init_pkt.len() < HID_PACKET_SIZE {
        init_pkt.push(0);
    }

    debug!("   Init pkt ({} bytes): {:02x?}", init_pkt.len(), &init_pkt[..12.min(init_pkt.len())]);

    device
        .write(&init_pkt)
        .await
        .map_err(|e| BearDogError::system(format!("HID write failed: {e}")))?;

    // Send continuation packets if payload exceeds 57 bytes
    let mut offset = first_take;
    let mut seq: u8 = 0;
    let cont_payload_max = HID_PACKET_SIZE - 5; // 59

    while offset < packet.len() {

        let take = (packet.len() - offset).min(cont_payload_max);
        let mut cont_pkt = Vec::with_capacity(HID_PACKET_SIZE);
        cont_pkt.extend_from_slice(&cid_bytes);
        cont_pkt.push(seq);
        cont_pkt.extend_from_slice(&packet[offset..offset + take]);
        while cont_pkt.len() < HID_PACKET_SIZE {
            cont_pkt.push(0);
        }

        debug!(
            "   Cont pkt seq={} ({} bytes payload): {:02x?}",
            seq, take, &cont_pkt[..12.min(cont_pkt.len())]
        );

        device
            .write(&cont_pkt)
            .await
            .map_err(|e| BearDogError::system(format!("HID continuation write failed: {e}")))?;

        offset += take;
        seq = seq.wrapping_add(1);
    }

    debug!(
        "✅ Sent CTAP2 command ({} bytes, {} packet(s))",
        packet.len(),
        1 + u32::from(seq)
    );

    // Read multi-packet response with keepalive support.
    // Init packet: [CID(4)][CMD(1)][BCNTH(1)][BCNTL(1)][payload ≤57]
    // Continuation: [CID(4)][SEQ(1)][payload ≤59]
    debug!("📥 Reading response (multi-packet + keepalive)...");

    let mut assembled: Vec<u8> = Vec::new();
    let mut total_len: Option<usize> = None;
    let mut next_seq: u8 = 0;

    for attempt in 1..=MAX_KEEPALIVE_ATTEMPTS {
        let mut buf = vec![0u8; HID_PACKET_SIZE];
        let n = device
            .read(&mut buf)
            .await
            .map_err(|e| BearDogError::system(format!("HID read failed: {e}")))?;

        if n == 0 {
            debug!(
                "   Attempt {}/{}: No data — polling in {}ms",
                attempt, MAX_KEEPALIVE_ATTEMPTS, HID_READ_TIMEOUT_MS
            );
            tokio::time::sleep(tokio::time::Duration::from_millis(HID_READ_TIMEOUT_MS)).await;
            continue;
        }

        if n < 5 {
            return Err(BearDogError::system(format!(
                "CTAPHID response too short: {n} bytes"
            )));
        }

        let b4 = buf[4];

        // Keepalive — authenticator still processing / awaiting user presence
        if b4 == CtapHidCommand::Keepalive.as_u8() {
            debug!("   📡 Keepalive — awaiting user presence");
            tokio::time::sleep(tokio::time::Duration::from_millis(HID_READ_TIMEOUT_MS)).await;
            continue;
        }

        // Error packet
        if b4 == CtapHidCommand::Error.as_u8() {
            let code = if n > 7 { buf[7] } else { 0 };
            return Err(BearDogError::system(format!(
                "CTAPHID error: 0x{code:02X}"
            )));
        }

        if assembled.is_empty() {
            // First (initialization) packet
            if b4 != CtapHidCommand::Cbor.as_u8() {
                return Err(BearDogError::system(format!(
                    "Expected CTAPHID_CBOR (0x90), got 0x{b4:02x}"
                )));
            }
            if n < 7 {
                return Err(BearDogError::system(
                    "CTAPHID init packet missing length".to_string(),
                ));
            }
            let bcnt = ((buf[5] as usize) << 8) | (buf[6] as usize);
            total_len = Some(bcnt);
            let first_payload_max = HID_PACKET_SIZE - 7; // 57 bytes
            let take = bcnt.min(first_payload_max).min(n - 7);
            assembled.extend_from_slice(&buf[7..7 + take]);
            debug!(
                "   Init packet: bcnt={bcnt}, got {take} bytes (attempt {attempt})"
            );
            if assembled.len() >= bcnt {
                break;
            }
        } else {
            // Continuation packet: [CID(4)][SEQ(1)][payload ≤59]
            let bcnt = total_len.unwrap_or(0);
            if b4 != next_seq {
                return Err(BearDogError::system(format!(
                    "CTAPHID bad seq: got 0x{b4:02x}, want 0x{next_seq:02x}"
                )));
            }
            let remaining = bcnt.saturating_sub(assembled.len());
            let cont_payload_max = HID_PACKET_SIZE - 5; // 59 bytes
            let take = remaining.min(cont_payload_max).min(n - 5);
            assembled.extend_from_slice(&buf[5..5 + take]);
            next_seq = next_seq.wrapping_add(1);
            debug!(
                "   Continuation seq={}: +{take} bytes, total={}/{}",
                b4,
                assembled.len(),
                bcnt
            );
            if assembled.len() >= bcnt {
                break;
            }
        }
    }

    if assembled.is_empty() {
        return Err(BearDogError::system(
            "CTAPHID timeout — no response received".to_string(),
        ));
    }

    let total = total_len.unwrap_or(assembled.len());
    if assembled.len() < total {
        return Err(BearDogError::system(format!(
            "CTAPHID incomplete: got {} of {total} bytes",
            assembled.len()
        )));
    }

    // First byte of assembled payload is the CTAP2 status code
    let status = Ctap2Status::from_byte(assembled[0]);
    debug!("CTAP2 Status: {:?} (0x{:02X})", status, assembled[0]);

    if !status.is_success() {
        return Err(BearDogError::system(format!(
            "CTAP2 error: {} (0x{:02X})",
            status.to_error_message(),
            assembled[0]
        )));
    }

    // Return payload (everything after status byte)
    Ok(assembled[1..].to_vec())
}
