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

use super::constants::*;
use beardog_errors::BearDogError;
use ciborium::Value as CborValue;
use std::collections::BTreeMap;
use tracing::{debug, info, warn};

/// CTAP2 status codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctap2Status {
    Success = 0x00,
    InvalidCommand = 0x01,
    InvalidParameter = 0x02,
    InvalidLength = 0x03,
    InvalidSeq = 0x04,
    Timeout = 0x05,
    ChannelBusy = 0x06,
    LockRequired = 0x0A,
    InvalidChannel = 0x0B,
    CborUnexpected = 0x11,
    CborError = 0x12,
    MissingParameter = 0x14,
    LimitExceeded = 0x15,
    UnsupportedExtension = 0x16,
    CredentialExcluded = 0x19,
    Processing = 0x21,
    InvalidCredential = 0x22,
    UserActionPending = 0x23,
    OperationPending = 0x24,
    NoOperations = 0x25,
    UnsupportedAlgorithm = 0x26,
    OperationDenied = 0x27,
    KeyStoreFull = 0x28,
    NotBusy = 0x29,
    NoOperationPending = 0x2A,
    UnsupportedOption = 0x2B,
    InvalidOption = 0x2C,
    KeepaliveCancel = 0x2D,
    NoCredentials = 0x2E,
    UserActionTimeout = 0x2F,
    NotAllowed = 0x30,
    PinInvalid = 0x31,
    PinBlocked = 0x32,
    PinAuthInvalid = 0x33,
    PinAuthBlocked = 0x34,
    PinNotSet = 0x35,
    PinRequired = 0x36,
    PinPolicyViolation = 0x37,
    PinTokenExpired = 0x38,
    RequestTooLarge = 0x39,
    ActionTimeout = 0x3A,
    UpRequired = 0x3B,
    UvBlocked = 0x3C,
    IntegrityFailure = 0x3D,
    InvalidSubcommand = 0x3E,
    UvInvalid = 0x3F,
    UnauthorizedPermission = 0x40,
    Other = 0xFF,
}

impl Ctap2Status {
    /// Convert byte to status code
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Self::Success,
            0x01 => Self::InvalidCommand,
            0x02 => Self::InvalidParameter,
            0x11 => Self::CborUnexpected,
            0x12 => Self::CborError,
            0x14 => Self::MissingParameter,
            0x21 => Self::Processing,
            0x23 => Self::UserActionPending,
            0x2E => Self::NoCredentials,
            0x35 => Self::PinNotSet,
            0x36 => Self::PinRequired,
            0x3B => Self::UpRequired,
            _ => Self::Other,
        }
    }

    /// Check if status indicates success
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Convert to error message
    pub fn to_error_message(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::InvalidCommand => "Invalid command",
            Self::InvalidParameter => "Invalid parameter",
            Self::CborUnexpected => "Unexpected CBOR type",
            Self::CborError => "CBOR parsing error",
            Self::MissingParameter => "Missing required parameter",
            Self::Processing => "Device is processing",
            Self::UserActionPending => "User action required (touch button)",
            Self::NoCredentials => "No credentials found",
            Self::PinNotSet => "PIN not set",
            Self::PinRequired => "PIN required",
            Self::UpRequired => "User presence required",
            _ => "Unknown error",
        }
    }
}

/// CTAPHID command codes (HID layer)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CtapHidCommand {
    Msg = 0x83,       // CTAPHID_MSG - Encapsulates CTAP command
    Cbor = 0x90,      // CTAPHID_CBOR - CTAP CBOR command
    Init = 0x86,      // CTAPHID_INIT - Initialize channel
    Ping = 0x81,      // CTAPHID_PING - Echo data
    Cancel = 0x91,    // CTAPHID_CANCEL - Cancel pending request
    Error = 0xBF,     // CTAPHID_ERROR - Error response
    Keepalive = 0xBB, // CTAPHID_KEEPALIVE - Keep connection alive
}

impl CtapHidCommand {
    /// Convert command to u8 value
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// CTAP2 command codes (Protocol layer)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Ctap2Command {
    MakeCredential = 0x01,
    GetAssertion = 0x02,
    GetInfo = 0x04,
    ClientPin = 0x06,
    Reset = 0x07,
    GetNextAssertion = 0x08,
    CredentialManagement = 0x0A,
}

/// CTAP2 device information from GetInfo command
#[derive(Debug, Clone)]
pub struct Ctap2DeviceInfo {
    /// List of supported CTAP protocol versions
    pub versions: Vec<String>,

    /// List of supported extensions
    pub extensions: Vec<String>,

    /// AAGUID (Authenticator Attestation GUID)
    pub aaguid: Vec<u8>,

    /// Supported options (e.g., "rk", "up", "uv", "plat")
    pub options: BTreeMap<String, bool>,

    /// Maximum message size
    pub max_msg_size: Option<u64>,

    /// List of supported PIN protocols
    pub pin_protocols: Option<Vec<u64>>,

    /// Maximum number of credentials in credentialID list
    pub max_credential_count_in_list: Option<u64>,

    /// Maximum credential ID length
    pub max_credential_id_length: Option<u64>,

    /// List of supported transports
    pub transports: Option<Vec<String>>,

    /// List of supported algorithms
    pub algorithms: Option<Vec<BTreeMap<String, CborValue>>>,

    /// Maximum size of serialized large-blob array
    pub max_serialized_large_blob_array: Option<u64>,

    /// Whether forcePINChange is required
    pub force_pin_change: Option<bool>,

    /// Minimum PIN length
    pub min_pin_length: Option<u64>,

    /// Firmware version
    pub firmware_version: Option<u64>,

    /// Maximum credential blob length
    pub max_cred_blob_length: Option<u64>,

    /// Maximum number of RPs for setMinPINLength
    pub max_rpids_for_set_min_pin_length: Option<u64>,

    /// Preferred platform UV attempts
    pub preferred_platform_uv_attempts: Option<u64>,

    /// UV modality
    pub uv_modality: Option<u64>,

    /// Certifications
    pub certifications: Option<BTreeMap<String, u64>>,

    /// Remaining discoverable credentials
    pub remaining_discoverable_credentials: Option<u64>,

    /// Vendor prototype config commands
    pub vendor_prototype_config_commands: Option<Vec<u64>>,
}

/// Initialize CTAPHID channel and get channel ID
///
/// This MUST be called before sending any CTAP2 commands.
/// Returns the channel ID (CID) to use for all subsequent commands.
#[cfg(feature = "fido2")]
pub async fn ctaphid_init(device: &hidapi::HidDevice) -> Result<u32, BearDogError> {
    info!("🔗 Initializing CTAPHID channel...");

    // Generate random nonce (8 bytes)
    use rand::RngCore;
    let mut nonce = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut nonce);

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
        .map_err(|e| BearDogError::system(format!("CTAPHID_INIT write failed: {e}")))?;

    debug!("📤 Sent CTAPHID_INIT");

    // Read response
    let mut response = vec![0u8; 64];
    let bytes_read = device
        .read_timeout(&mut response, 5000)
        .map_err(|e| BearDogError::system(format!("CTAPHID_INIT read failed: {e}")))?;

    if bytes_read == 0 {
        return Err(BearDogError::system("CTAPHID_INIT timeout".to_string()));
    }

    debug!("📥 Received {} bytes response", bytes_read);

    // Parse response
    // Format: [CID (4)] [CMD] [LEN_H] [LEN_L] [NONCE (8)] [NEW_CID (4)] [PROTOCOL_VERSION] [...]
    if bytes_read < 17 {
        return Err(BearDogError::system(format!(
            "CTAPHID_INIT response too short: {} bytes",
            bytes_read
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
/// - Initialization packet: [CID (4 bytes)] [CMD] [BCNTH] [BCNTL] [DATA (up to 57 bytes)]
/// - Continuation packets: [CID (4 bytes)] [SEQ] [DATA (up to 59 bytes)]
///
/// Uses CTAPHID_CBOR (0x90) command which directly carries CTAP2 commands.
#[cfg(feature = "fido2")]
pub async fn send_ctap2_command(
    device: &hidapi::HidDevice,
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

    // Wrap in CTAPHID_MSG frame
    // Format: [CID (4 bytes)] [CMD: 0x83] [LEN_H] [LEN_L] [DATA...]
    // Note: Using CTAPHID_MSG (0x83) as it's more widely supported than CTAPHID_CBOR (0x90)
    let cid_bytes = cid.to_be_bytes();
    let mut hid_packet = vec![cid_bytes[0], cid_bytes[1], cid_bytes[2], cid_bytes[3]];
    hid_packet.push(CtapHidCommand::Msg as u8); // CTAPHID_MSG

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
        .map_err(|e| BearDogError::system(format!("HID write failed: {e}")))?;

    debug!("✅ Sent {} bytes to device", hid_packet.len());

    // Read response with multiple attempts (device might send keepalive)
    debug!("📥 Reading response (with retry for keepalive)...");

    let mut total_response = Vec::new();

    for attempt in 1..=MAX_KEEPALIVE_ATTEMPTS {
        let mut response_buf = vec![0u8; HID_PACKET_SIZE];
        let bytes_read = device
            .read_timeout(
                &mut response_buf,
                HID_READ_TIMEOUT_MS.try_into().unwrap_or(1000),
            )
            .map_err(|e| BearDogError::system(format!("HID read failed: {e}")))?;

        if bytes_read == 0 {
            debug!(
                "   Attempt {}/{}: No data (timeout)",
                attempt, MAX_KEEPALIVE_ATTEMPTS
            );
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

            // Check for keepalive
            if response_cmd == CtapHidCommand::Keepalive.as_u8() {
                debug!("   📡 Keepalive packet - waiting for actual response...");
                continue; // Keep reading
            }

            // Check for error
            if response_cmd == CtapHidCommand::Error.as_u8() {
                warn!("   ❌ Device returned error packet");
                if bytes_read >= 8 {
                    let error_code = response_buf[7];
                    return Err(BearDogError::system(format!(
                        "Device error: 0x{:02X}",
                        error_code
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
            "Response too short: {} bytes",
            bytes_read
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

/// Query device information via CTAP2 GetInfo
///
/// This is the first command you should send to any FIDO2 device.
/// It returns capabilities, supported features, and device metadata.
///
/// # Note
///
/// This function automatically initializes the CTAPHID channel if needed.
#[cfg(feature = "fido2")]
pub async fn ctap2_get_info(device: &hidapi::HidDevice) -> Result<Ctap2DeviceInfo, BearDogError> {
    info!("🔍 Querying device capabilities (CTAP2 GetInfo)...");

    // Initialize channel first
    let cid = ctaphid_init(device).await?;

    // GetInfo has no parameters - empty CBOR payload
    let response_bytes = send_ctap2_command(device, cid, Ctap2Command::GetInfo, &[]).await?;

    // Parse CBOR response
    let cbor_value: CborValue = ciborium::from_reader(&response_bytes[..])
        .map_err(|e| BearDogError::system(format!("CBOR parse error: {e}")))?;

    // GetInfo response is a CBOR map
    let map = match cbor_value {
        CborValue::Map(m) => m,
        _ => {
            return Err(BearDogError::system(
                "GetInfo response not a CBOR map".to_string(),
            ))
        }
    };

    // Extract fields from CBOR map
    let mut info = Ctap2DeviceInfo {
        versions: Vec::new(),
        extensions: Vec::new(),
        aaguid: Vec::new(),
        options: BTreeMap::new(),
        max_msg_size: None,
        pin_protocols: None,
        max_credential_count_in_list: None,
        max_credential_id_length: None,
        transports: None,
        algorithms: None,
        max_serialized_large_blob_array: None,
        force_pin_change: None,
        min_pin_length: None,
        firmware_version: None,
        max_cred_blob_length: None,
        max_rpids_for_set_min_pin_length: None,
        preferred_platform_uv_attempts: None,
        uv_modality: None,
        certifications: None,
        remaining_discoverable_credentials: None,
        vendor_prototype_config_commands: None,
    };

    for (key, value) in map {
        let key_int: Option<i128> = match key {
            CborValue::Integer(n) => Some(n.into()),
            _ => None,
        };

        match key_int {
            Some(1) => {
                // versions
                if let CborValue::Array(arr) = value {
                    for v in arr {
                        if let CborValue::Text(s) = v {
                            info.versions.push(s);
                        }
                    }
                }
            }
            Some(2) => {
                // extensions
                if let CborValue::Array(arr) = value {
                    for v in arr {
                        if let CborValue::Text(s) = v {
                            info.extensions.push(s);
                        }
                    }
                }
            }
            Some(3) => {
                // aaguid
                if let CborValue::Bytes(b) = value {
                    info.aaguid = b;
                }
            }
            Some(4) => {
                // options
                if let CborValue::Map(opts) = value {
                    for (opt_key, opt_val) in opts {
                        if let (CborValue::Text(key_str), CborValue::Bool(val_bool)) =
                            (opt_key, opt_val)
                        {
                            info.options.insert(key_str, val_bool);
                        }
                    }
                }
            }
            Some(5) => {
                // maxMsgSize
                if let CborValue::Integer(n) = value {
                    info.max_msg_size = Some(n.try_into().unwrap_or(0));
                }
            }
            Some(6) => {
                // pinProtocols
                if let CborValue::Array(arr) = value {
                    let protocols: Vec<u64> = arr
                        .iter()
                        .filter_map(|v| {
                            if let CborValue::Integer(n) = v {
                                (*n).try_into().ok()
                            } else {
                                None
                            }
                        })
                        .collect();
                    info.pin_protocols = Some(protocols);
                }
            }
            _ => {
                // Other fields...
                debug!("Unhandled GetInfo field: {:?}", key);
            }
        }
    }

    info!("✅ Device capabilities:");
    info!("   Versions: {}", info.versions.join(", "));
    info!("   Extensions: {}", info.extensions.join(", "));
    info!("   Options: {:?}", info.options);

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctap2_status_conversion() {
        assert_eq!(Ctap2Status::from_byte(0x00), Ctap2Status::Success);
        assert_eq!(Ctap2Status::from_byte(0x01), Ctap2Status::InvalidCommand);
        assert!(Ctap2Status::Success.is_success());
        assert!(!Ctap2Status::InvalidCommand.is_success());
    }

    #[test]
    fn test_ctap2_command_codes() {
        assert_eq!(Ctap2Command::GetInfo as u8, 0x04);
        assert_eq!(Ctap2Command::MakeCredential as u8, 0x01);
        assert_eq!(Ctap2Command::GetAssertion as u8, 0x02);
    }
}
