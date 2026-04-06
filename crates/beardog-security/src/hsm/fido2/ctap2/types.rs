// SPDX-License-Identifier: AGPL-3.0-or-later

//! CTAP2 protocol types: status codes, HID/CTAP command codes, and `GetInfo` structures.

use ciborium::Value as CborValue;
use std::collections::BTreeMap;

/// CTAP2 (Client to Authenticator Protocol 2) status codes
///
/// These status codes are defined in the FIDO2/WebAuthn specification and are
/// returned by authenticators to indicate operation results or error conditions.
///
/// # Specification
/// Based on CTAP2 specification: <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html>
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctap2Status {
    /// Operation completed successfully
    Success = 0x00,
    /// The command is not a valid CTAP command
    InvalidCommand = 0x01,
    /// The command included an invalid parameter
    InvalidParameter = 0x02,
    /// Invalid message or item length
    InvalidLength = 0x03,
    /// Invalid message sequencing
    InvalidSeq = 0x04,
    /// Message timed out
    Timeout = 0x05,
    /// Channel busy - cannot process command at this time
    ChannelBusy = 0x06,
    /// Command requires channel lock
    LockRequired = 0x0A,
    /// Invalid channel identifier
    InvalidChannel = 0x0B,
    /// Unexpected CBOR type encountered
    CborUnexpected = 0x11,
    /// Error when parsing CBOR structure
    CborError = 0x12,
    /// Missing required parameter in the request
    MissingParameter = 0x14,
    /// Limit for number of items exceeded
    LimitExceeded = 0x15,
    /// Unsupported extension requested
    UnsupportedExtension = 0x16,
    /// Credential was excluded from the operation
    CredentialExcluded = 0x19,
    /// Authenticator is processing the request
    Processing = 0x21,
    /// The credential provided is invalid or not recognized
    InvalidCredential = 0x22,
    /// User action (e.g., touch, button press) is pending
    UserActionPending = 0x23,
    /// Operation is pending and needs to be completed
    OperationPending = 0x24,
    /// No operations are currently pending
    NoOperations = 0x25,
    /// The requested algorithm is not supported
    UnsupportedAlgorithm = 0x26,
    /// Operation was denied by user or policy
    OperationDenied = 0x27,
    /// Internal key storage is full
    KeyStoreFull = 0x28,
    /// Authenticator is not busy (no operation in progress)
    NotBusy = 0x29,
    /// No operation is currently pending
    NoOperationPending = 0x2A,
    /// The requested option is not supported
    UnsupportedOption = 0x2B,
    /// The option value provided is invalid
    InvalidOption = 0x2C,
    /// Keepalive was cancelled by user or timeout
    KeepaliveCancel = 0x2D,
    /// No credentials are available
    NoCredentials = 0x2E,
    /// User action timed out waiting for input
    UserActionTimeout = 0x2F,
    /// Operation not allowed
    NotAllowed = 0x30,
    /// PIN is invalid
    PinInvalid = 0x31,
    /// PIN is blocked due to too many attempts
    PinBlocked = 0x32,
    /// PIN authentication is invalid
    PinAuthInvalid = 0x33,
    /// PIN authentication is blocked
    PinAuthBlocked = 0x34,
    /// PIN is not set
    PinNotSet = 0x35,
    /// PIN is required for this operation
    PinRequired = 0x36,
    /// PIN policy violation
    PinPolicyViolation = 0x37,
    /// PIN token has expired
    PinTokenExpired = 0x38,
    /// Request is too large
    RequestTooLarge = 0x39,
    /// Action timed out
    ActionTimeout = 0x3A,
    /// User presence required
    UpRequired = 0x3B,
    /// User verification is blocked
    UvBlocked = 0x3C,
    /// Integrity check failure
    IntegrityFailure = 0x3D,
    /// Invalid subcommand
    InvalidSubcommand = 0x3E,
    /// User verification is invalid
    UvInvalid = 0x3F,
    /// Unauthorized permission
    UnauthorizedPermission = 0x40,
    /// Other error
    Other = 0xFF,
}

impl Ctap2Status {
    /// Convert byte to status code
    pub const fn from_byte(byte: u8) -> Self {
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
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Convert to error message
    pub const fn to_error_message(&self) -> &'static str {
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
    /// `CTAPHID_MSG` — Encapsulates CTAP command
    Msg = 0x83,
    /// `CTAPHID_CBOR` — CTAP CBOR command
    Cbor = 0x90,
    /// `CTAPHID_INIT` — Initialize channel
    Init = 0x86,
    /// `CTAPHID_PING` — Echo data
    Ping = 0x81,
    /// `CTAPHID_CANCEL` — Cancel pending request
    Cancel = 0x91,
    /// `CTAPHID_ERROR` — Error response
    Error = 0xBF,
    /// `CTAPHID_KEEPALIVE` — Keep connection alive
    Keepalive = 0xBB,
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
    /// Make a new credential (attestation)
    MakeCredential = 0x01,
    /// Get an assertion (authentication)
    GetAssertion = 0x02,
    /// Get authenticator info
    GetInfo = 0x04,
    /// Client PIN operations
    ClientPin = 0x06,
    /// Reset the authenticator
    Reset = 0x07,
    /// Get next assertion in multi-credential scenario
    GetNextAssertion = 0x08,
    /// Manage credentials
    CredentialManagement = 0x0A,
}

/// CTAP2 device information from `GetInfo` command
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
