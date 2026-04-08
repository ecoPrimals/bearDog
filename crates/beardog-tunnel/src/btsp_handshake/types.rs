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
