// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP handshake message types (length-prefixed JSON on the wire).

use serde::{Deserialize, Serialize};

/// Protocol version for the BTSP handshake.
pub const BTSP_HANDSHAKE_VERSION: u32 = 1;

// ── Step 1: Client → Server ────────────────────────────────────────────

/// First message from the connecting client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHello {
    /// Protocol version the client supports.
    pub version: u32,
    /// Base64-encoded X25519 ephemeral public key (32 bytes).
    pub client_ephemeral_pub: String,
}

// ── Step 2: Server → Client ────────────────────────────────────────────

/// Server's response with its own ephemeral key and a random challenge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHello {
    /// Protocol version the server supports.
    pub version: u32,
    /// Base64-encoded X25519 ephemeral public key (32 bytes).
    pub server_ephemeral_pub: String,
    /// Base64-encoded random challenge (32 bytes).
    pub challenge: String,
}

// ── Step 3: Client → Server ────────────────────────────────────────────

/// Client proves family membership via HMAC and requests a cipher suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    /// Base64-encoded `HMAC-SHA256(handshake_key, challenge || client_pub || server_pub)`.
    pub response: String,
    /// Preferred cipher: `chacha20_poly1305`, `hmac_plain`, or `null`.
    pub preferred_cipher: String,
}

// ── Step 4: Server → Client ────────────────────────────────────────────

/// Sent after successful verification; both sides derive session keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeComplete {
    /// Negotiated cipher suite name.
    pub cipher: String,
    /// Hex-encoded random session ID (16 bytes → 32 hex chars).
    pub session_id: String,
    /// HMAC proving server knows the family seed (mutual authentication)
    #[serde(default)]
    pub server_proof: String,
}

// ── Error ──────────────────────────────────────────────────────────────

/// Sent when the handshake fails (connection is closed immediately after).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeError {
    /// Machine-readable error code (e.g. `"version_mismatch"`).
    pub error: String,
    /// Human-readable explanation.
    pub reason: String,
}

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]

    use super::*;

    #[test]
    fn wire_messages_roundtrip_json() {
        let hello = ClientHello {
            version: BTSP_HANDSHAKE_VERSION,
            client_ephemeral_pub: "AAA".into(),
        };
        let j = serde_json::to_string(&hello).expect("ser");
        let back: ClientHello = serde_json::from_str(&j).expect("de");
        assert_eq!(back.version, hello.version);

        let sh = ServerHello {
            version: 1,
            server_ephemeral_pub: "BBB".into(),
            challenge: "CCC".into(),
        };
        let j2 = serde_json::to_string(&sh).expect("ser");
        let back2: ServerHello = serde_json::from_str(&j2).expect("de");
        assert_eq!(back2.challenge, sh.challenge);

        let cr = ChallengeResponse {
            response: "DDD".into(),
            preferred_cipher: "chacha20_poly1305".into(),
        };
        let j3 = serde_json::to_string(&cr).expect("ser");
        let back3: ChallengeResponse = serde_json::from_str(&j3).expect("de");
        assert_eq!(back3.preferred_cipher, cr.preferred_cipher);

        let hc = HandshakeComplete {
            cipher: "chacha20_poly1305".into(),
            session_id: "aabb".into(),
            server_proof: String::new(),
        };
        let j4 = serde_json::to_string(&hc).expect("ser");
        let back4: HandshakeComplete = serde_json::from_str(&j4).expect("de");
        assert_eq!(back4.session_id, hc.session_id);

        let he = HandshakeError {
            error: "e".into(),
            reason: "r".into(),
        };
        let j5 = serde_json::to_string(&he).expect("ser");
        let back5: HandshakeError = serde_json::from_str(&j5).expect("de");
        assert_eq!(back5.reason, he.reason);
    }
}
