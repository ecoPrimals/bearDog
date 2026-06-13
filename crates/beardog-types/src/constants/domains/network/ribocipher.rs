// SPDX-License-Identifier: AGPL-3.0-or-later

//! riboCipher Transport Signal Constants
//!
//! Wire-format constants for the riboCipher deterministic protocol routing
//! standard. These are shared across all bearDog crates that initiate or
//! accept IPC connections. Detection logic lives in `beardog-tunnel`.

/// Clear signal prefix — readable by any ecosystem participant.
pub const SIGNAL_CLEAR: u8 = 0xEC;

/// Mito-obfuscated signal — only decodable by family seed holders.
pub const SIGNAL_MITO: u8 = 0xED;

/// Nuclear-sealed signal — per-peer encrypted protocol selection.
pub const SIGNAL_NUCLEAR: u8 = 0xEE;

/// Protocol type: lightweight health probe.
pub const PROTO_PROBE: u8 = 0x00;

/// Protocol type: NDJSON JSON-RPC (standard ecosystem IPC).
pub const PROTO_NDJSON_JSONRPC: u8 = 0x01;

/// Protocol type: BTSP binary (length-prefixed handshake).
pub const PROTO_BTSP_BINARY: u8 = 0x02;

/// Protocol type: BTSP JSON-line (`ClientHello` as first JSON line).
pub const PROTO_BTSP_JSONLINE: u8 = 0x03;

/// Protocol type: HTTP/1.1 over UDS.
pub const PROTO_HTTP: u8 = 0x04;

/// Protocol type: encrypted session resume (post-BTSP).
pub const PROTO_ENCRYPTED_RESUME: u8 = 0x05;

/// Protocol type: dark forest beacon packet.
pub const PROTO_DARK_FOREST_BEACON: u8 = 0x06;

/// Protocol type: songBird mesh relay frame.
pub const PROTO_MESH_RELAY: u8 = 0x07;

/// Build a 2-byte clear signal for outbound connections.
#[must_use]
pub const fn clear_signal(protocol_type: u8) -> [u8; 2] {
    [SIGNAL_CLEAR, protocol_type]
}
