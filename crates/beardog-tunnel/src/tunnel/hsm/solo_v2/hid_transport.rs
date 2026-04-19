// SPDX-License-Identifier: AGPL-3.0-or-later

//! HID-backed CTAP2 transport using `beardog-hid` (pure Rust).
//!
//! CTAPHID framing follows FIDO Client to Authenticator Protocol (HID).

use std::future::Future;

use super::transport::Ctap2Transport;
use beardog_errors::BearDogError;
use beardog_hid::{HidDevice, HidDeviceBackend};
use rand::RngCore;
use tracing::{debug, warn};

const HID_PACKET_SIZE: usize = 64;
const MAX_KEEPALIVE_ATTEMPTS: usize = 32;
const FIRST_PAYLOAD_MAX: usize = 57;
const CONT_PAYLOAD_MAX: usize = 59;

#[repr(u8)]
#[derive(Clone, Copy)]
enum CtapHidCommand {
    Msg = 0x83,
    Init = 0x86,
    Error = 0xBF,
    Keepalive = 0xBB,
}

/// HID-backed CTAP2 transport using `beardog-hid` (pure Rust).
pub struct HidCtap2Transport {
    device: HidDeviceBackend,
    channel_id: Option<u32>,
}

impl HidCtap2Transport {
    /// Open a HID device by path (e.g. `/dev/hidrawN`).
    ///
    /// # Errors
    ///
    /// Returns an error if the device cannot be opened (path invalid, permissions, or I/O failure).
    pub async fn open(device_path: &str) -> Result<Self, BearDogError> {
        let device = beardog_hid::open_device(device_path).await?;
        Ok(Self {
            device,
            channel_id: None,
        })
    }

    async fn ensure_channel(&mut self) -> Result<u32, BearDogError> {
        if let Some(cid) = self.channel_id {
            return Ok(cid);
        }
        let cid = ctaphid_init(&mut self.device).await?;
        self.channel_id = Some(cid);
        Ok(cid)
    }
}

async fn hid_send_receive(
    transport: &mut HidCtap2Transport,
    command: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    let cid = transport.ensure_channel().await?;
    send_ctaphid_message(&mut transport.device, cid, command).await?;
    read_ctaphid_ctap_response(&mut transport.device, cid).await
}

impl Ctap2Transport for HidCtap2Transport {
    fn send_receive(
        &mut self,
        command: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        hid_send_receive(self, command)
    }
}

async fn ctaphid_init<D: HidDevice + ?Sized>(device: &mut D) -> Result<u32, BearDogError> {
    let mut nonce = [0u8; 8];
    rand::rng().fill_bytes(&mut nonce);

    let mut packet = vec![0xFF, 0xFF, 0xFF, 0xFF];
    packet.push(CtapHidCommand::Init as u8);
    packet.push(0x00);
    packet.push(0x08);
    packet.extend_from_slice(&nonce);
    while packet.len() < HID_PACKET_SIZE {
        packet.push(0);
    }

    device
        .write(&packet)
        .await
        .map_err(|e| BearDogError::system(format!("CTAPHID_INIT write failed: {e}")))?;

    let mut response = vec![0u8; HID_PACKET_SIZE];
    let bytes_read = device
        .read(&mut response)
        .await
        .map_err(|e| BearDogError::system(format!("CTAPHID_INIT read failed: {e}")))?;

    if bytes_read < 19 {
        return Err(BearDogError::system(format!(
            "CTAPHID_INIT response too short: {bytes_read} bytes"
        )));
    }
    if response[4] != CtapHidCommand::Init as u8 {
        return Err(BearDogError::system(format!(
            "Unexpected CTAPHID response cmd 0x{:02x}",
            response[4]
        )));
    }
    if response[7..15] != nonce {
        return Err(BearDogError::system(
            "CTAPHID_INIT nonce mismatch".to_string(),
        ));
    }

    let cid = u32::from_be_bytes([response[15], response[16], response[17], response[18]]);
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(cid)
}

async fn send_ctaphid_message<D: HidDevice + ?Sized>(
    device: &mut D,
    cid: u32,
    message: &[u8],
) -> Result<(), BearDogError> {
    let cid_b = cid.to_be_bytes();
    let total_len = message.len();
    if total_len > 7609 {
        return Err(BearDogError::system(format!(
            "CTAP inner message too large: {total_len}"
        )));
    }

    let first_chunk = std::cmp::min(FIRST_PAYLOAD_MAX, total_len);
    let bcnt_hi = u8::try_from((total_len >> 8) & 0xFF).map_err(|_| {
        BearDogError::system("CTAPHID message length high byte out of range".to_string())
    })?;
    let bcnt_lo = u8::try_from(total_len & 0xFF).map_err(|_| {
        BearDogError::system("CTAPHID message length low byte out of range".to_string())
    })?;
    let mut pkt = vec![
        cid_b[0],
        cid_b[1],
        cid_b[2],
        cid_b[3],
        CtapHidCommand::Msg as u8,
        bcnt_hi,
        bcnt_lo,
    ];
    pkt.extend_from_slice(&message[..first_chunk]);
    while pkt.len() < HID_PACKET_SIZE {
        pkt.push(0);
    }
    device
        .write(&pkt)
        .await
        .map_err(|e| BearDogError::system(format!("CTAPHID write failed: {e}")))?;

    let mut offset = first_chunk;
    let mut seq: u8 = 0;
    while offset < total_len {
        let take = std::cmp::min(CONT_PAYLOAD_MAX, total_len - offset);
        let mut cont = vec![cid_b[0], cid_b[1], cid_b[2], cid_b[3], seq];
        cont.extend_from_slice(&message[offset..offset + take]);
        while cont.len() < HID_PACKET_SIZE {
            cont.push(0);
        }
        device
            .write(&cont)
            .await
            .map_err(|e| BearDogError::system(format!("CTAPHID continuation write failed: {e}")))?;
        offset += take;
        seq = seq.wrapping_add(1);
    }

    Ok(())
}

/// Read a full CTAP2 payload (`[status][CBOR...]`) from `CTAPHID_MSG` response packets.
///
/// First packet: `[CID][0x83][BCNTH][BCNTL][payload ≤57]`. Continuation: `[CID][SEQ][payload ≤59]`.
async fn read_ctaphid_ctap_response<D: HidDevice + ?Sized>(
    device: &mut D,
    expected_cid: u32,
) -> Result<Vec<u8>, BearDogError> {
    let mut assembled: Vec<u8> = Vec::new();
    let mut total_len: Option<usize> = None;
    // Next expected continuation sequence (0 for first continuation frame).
    let mut next_seq: u8 = 0;

    for attempt in 1..=MAX_KEEPALIVE_ATTEMPTS {
        let mut buf = vec![0u8; HID_PACKET_SIZE];
        let n = device
            .read(&mut buf)
            .await
            .map_err(|e| BearDogError::system(format!("CTAPHID read failed: {e}")))?;

        if n < 5 {
            if n == 0 {
                debug!("CTAPHID read attempt {attempt}: empty");
                continue;
            }
            return Err(BearDogError::system(format!(
                "CTAPHID response too short: {n} bytes"
            )));
        }

        let cid = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if cid != expected_cid {
            debug!("skipping packet with unexpected CID 0x{cid:08x}");
            continue;
        }

        let b4 = buf[4];
        if b4 == CtapHidCommand::Keepalive as u8 {
            debug!("CTAPHID keepalive");
            continue;
        }
        if b4 == CtapHidCommand::Error as u8 {
            let code = if n > 7 { buf[7] } else { 0 };
            return Err(BearDogError::system(format!(
                "CTAPHID error packet: code 0x{code:02x}"
            )));
        }

        if assembled.is_empty() {
            if b4 != CtapHidCommand::Msg as u8 {
                return Err(BearDogError::system(format!(
                    "Expected CTAPHID MSG (0x83), got 0x{b4:02x}"
                )));
            }
            if n < 7 {
                return Err(BearDogError::system(
                    "CTAPHID MSG packet missing length".to_string(),
                ));
            }
            let bcnt = ((buf[5] as usize) << 8) | (buf[6] as usize);
            total_len = Some(bcnt);
            let take = std::cmp::min(FIRST_PAYLOAD_MAX, bcnt);
            if n < 7 + take {
                return Err(BearDogError::system(
                    "CTAPHID first packet shorter than declared payload".to_string(),
                ));
            }
            assembled.extend_from_slice(&buf[7..7 + take]);
            if assembled.len() == bcnt {
                return Ok(assembled);
            }
        } else {
            let bcnt = total_len.ok_or_else(|| {
                BearDogError::system("CTAPHID internal: missing total length".to_string())
            })?;
            if b4 != next_seq {
                return Err(BearDogError::system(format!(
                    "CTAPHID bad continuation seq: got 0x{b4:02x}, want 0x{next_seq:02x}"
                )));
            }
            let remaining = bcnt - assembled.len();
            let take = std::cmp::min(CONT_PAYLOAD_MAX, remaining);
            if n < 5 + take {
                return Err(BearDogError::system(
                    "CTAPHID continuation packet truncated".to_string(),
                ));
            }
            assembled.extend_from_slice(&buf[5..5 + take]);
            next_seq = next_seq.wrapping_add(1);
            if assembled.len() == bcnt {
                return Ok(assembled);
            }
        }

        if attempt == MAX_KEEPALIVE_ATTEMPTS {
            break;
        }
    }

    warn!("CTAPHID timeout waiting for response");
    Err(BearDogError::system(
        "CTAPHID timeout — no complete CTAP response".to_string(),
    ))
}

// --- Enum dispatch + test mock --------------------------------------------------------------------

/// Test-only mock CTAP2 transport (deterministic CBOR payloads for provider tests).
#[cfg(test)]
pub struct MockCtap2Transport {
    make_cred_response: Vec<u8>,
    get_assertion_response: Vec<u8>,
}

#[cfg(test)]
impl MockCtap2Transport {
    pub(crate) fn with_success_responses() -> Self {
        use super::ctap2_protocol::CTAP2_OK;
        use ciborium::Value as CborValue;

        let mut auth_data = vec![0u8; 32];
        auth_data.push(0x41);
        auth_data.extend([0u8; 4]);
        auth_data.extend([0u8; 16]);
        auth_data.push(0);
        auth_data.push(4);
        auth_data.extend_from_slice(&[1, 2, 3, 4]);
        auth_data.extend_from_slice(&[0xa1, 0x01, 0x18, 0x2b]);

        let mc = CborValue::Map(vec![
            (
                CborValue::Integer(1.into()),
                CborValue::Text("packed".to_string()),
            ),
            (CborValue::Integer(2.into()), CborValue::Bytes(auth_data)),
            (CborValue::Integer(3.into()), CborValue::Map(vec![])),
        ]);
        let mut mc_body = Vec::new();
        ciborium::into_writer(&mc, &mut mc_body).unwrap();
        let mut make_cred = vec![CTAP2_OK];
        make_cred.extend(mc_body);

        let ga = CborValue::Map(vec![
            (
                CborValue::Integer(2.into()),
                CborValue::Bytes(vec![0xcc; 37]),
            ),
            (
                CborValue::Integer(3.into()),
                CborValue::Bytes(vec![0xdd; 64]),
            ),
        ]);
        let mut ga_body = Vec::new();
        ciborium::into_writer(&ga, &mut ga_body).unwrap();
        let mut get_assert = vec![CTAP2_OK];
        get_assert.extend(ga_body);

        Self {
            make_cred_response: make_cred,
            get_assertion_response: get_assert,
        }
    }
}

#[cfg(test)]
async fn mock_send_receive(
    mock: &mut MockCtap2Transport,
    command: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    match command.first() {
        Some(&x) if x == CTAP2_MAKE_CREDENTIAL => Ok(mock.make_cred_response.clone()),
        Some(&x) if x == CTAP2_GET_ASSERTION => Ok(mock.get_assertion_response.clone()),
        _ => Err(BearDogError::system(
            "mock: unknown CTAP command".to_string(),
        )),
    }
}

#[cfg(test)]
impl Ctap2Transport for MockCtap2Transport {
    fn send_receive(
        &mut self,
        command: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        mock_send_receive(self, command)
    }
}

/// Enum dispatch for [`Ctap2Transport`].
pub enum Ctap2TransportBackend {
    /// HID-backed CTAP2 transport.
    Hid(HidCtap2Transport),
    #[cfg(test)]
    Mock(MockCtap2Transport),
}

async fn ctap2_backend_send_receive(
    backend: &mut Ctap2TransportBackend,
    command: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    match backend {
        Ctap2TransportBackend::Hid(t) => hid_send_receive(t, command).await,
        #[cfg(test)]
        Ctap2TransportBackend::Mock(t) => mock_send_receive(t, command).await,
    }
}

impl Ctap2Transport for Ctap2TransportBackend {
    fn send_receive(
        &mut self,
        command: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        ctap2_backend_send_receive(self, command)
    }
}
