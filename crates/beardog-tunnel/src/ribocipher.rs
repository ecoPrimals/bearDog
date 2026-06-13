// SPDX-License-Identifier: AGPL-3.0-or-later

//! riboCipher — Transport Signal Standard (Wave 111)
//!
//! Deterministic protocol routing via intentional signal prefix. Replaces
//! fragile peek-and-guess detection where servers read the first byte and
//! hope to classify the connection.
//!
//! # Wire Format
//!
//! ## Tier 1: Clear Signal (local same-gate IPC)
//! ```text
//! [0xEC][protocol_type: u8]
//! ```
//!
//! ## Tier 2: Mito-Obfuscated (cross-gate / WAN)
//! ```text
//! [0xED][hmac_tag: [u8; 4]]
//! ```
//!
//! ## Tier 3: Nuclear-Sealed (privileged)
//! ```text
//! [0xEE][encrypted_payload: [u8; 6]]
//! ```
//!
//! ## Legacy (deprecated)
//! Any connection NOT starting with 0xEC/0xED/0xEE logs WARN and falls
//! through to existing peek logic during the deprecation window.

pub use beardog_types::constants::domains::network::ribocipher::{
    PROTO_BTSP_BINARY, PROTO_BTSP_JSONLINE, PROTO_DARK_FOREST_BEACON, PROTO_ENCRYPTED_RESUME,
    PROTO_HTTP, PROTO_MESH_RELAY, PROTO_NDJSON_JSONRPC, PROTO_PROBE, SIGNAL_CLEAR, SIGNAL_MITO,
    SIGNAL_NUCLEAR, clear_signal,
};

/// Result of riboCipher signal detection on a connection's first bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalResult {
    /// Clear signal detected — protocol type decoded.
    Clear(u8),
    /// Mito-obfuscated signal detected (4-byte HMAC tag follows).
    /// Decoding deferred to caller with family seed.
    Mito([u8; 4]),
    /// Nuclear-sealed signal detected (6-byte encrypted payload follows).
    Nuclear([u8; 6]),
    /// No riboCipher signal — legacy unsignalled connection.
    Legacy(u8),
}

/// Check whether the first byte is a riboCipher signal prefix.
#[must_use]
pub const fn is_signal_byte(byte: u8) -> bool {
    matches!(byte, SIGNAL_CLEAR | SIGNAL_MITO | SIGNAL_NUCLEAR)
}

/// Human-readable name for a protocol type byte.
#[must_use]
pub const fn protocol_name(proto: u8) -> &'static str {
    match proto {
        PROTO_PROBE => "probe",
        PROTO_NDJSON_JSONRPC => "ndjson-jsonrpc",
        PROTO_BTSP_BINARY => "btsp-binary",
        PROTO_BTSP_JSONLINE => "btsp-jsonline",
        PROTO_HTTP => "http",
        PROTO_ENCRYPTED_RESUME => "encrypted-resume",
        PROTO_DARK_FOREST_BEACON => "dark-forest-beacon",
        PROTO_MESH_RELAY => "mesh-relay",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_bytes_are_sequential_and_non_ascii() {
        assert_eq!(SIGNAL_CLEAR, 0xEC);
        assert_eq!(SIGNAL_MITO, 0xED);
        assert_eq!(SIGNAL_NUCLEAR, 0xEE);
        assert!(!SIGNAL_CLEAR.is_ascii());
        assert!(!SIGNAL_MITO.is_ascii());
        assert!(!SIGNAL_NUCLEAR.is_ascii());
    }

    #[test]
    fn signal_bytes_never_collide_with_json_or_http() {
        assert_ne!(SIGNAL_CLEAR, b'{');
        assert_ne!(SIGNAL_CLEAR, b'G');
        assert_ne!(SIGNAL_CLEAR, b'P');
        assert_ne!(SIGNAL_CLEAR, b'H');
        assert_ne!(SIGNAL_CLEAR, 0x00);
    }

    #[test]
    fn is_signal_byte_detects_all_tiers() {
        assert!(is_signal_byte(SIGNAL_CLEAR));
        assert!(is_signal_byte(SIGNAL_MITO));
        assert!(is_signal_byte(SIGNAL_NUCLEAR));
        assert!(!is_signal_byte(b'{'));
        assert!(!is_signal_byte(0x00));
        assert!(!is_signal_byte(b'G'));
    }

    #[test]
    fn clear_signal_builds_correct_prefix() {
        assert_eq!(clear_signal(PROTO_NDJSON_JSONRPC), [0xEC, 0x01]);
        assert_eq!(clear_signal(PROTO_BTSP_BINARY), [0xEC, 0x02]);
    }

    #[test]
    fn protocol_names_cover_known_types() {
        assert_eq!(protocol_name(PROTO_NDJSON_JSONRPC), "ndjson-jsonrpc");
        assert_eq!(protocol_name(PROTO_BTSP_BINARY), "btsp-binary");
        assert_eq!(protocol_name(0xFF), "unknown");
    }
}
