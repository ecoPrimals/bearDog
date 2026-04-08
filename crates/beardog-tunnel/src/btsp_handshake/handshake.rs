// SPDX-License-Identifier: AGPL-3.0-or-later

//! Server-side BTSP handshake orchestrator.
//!
//! Runs the 4-step handshake on a raw stream before any JSON-RPC traffic.

use super::FamilySeed;
use super::crypto::{
    derive_handshake_key, derive_session_keys, generate_ephemeral_keypair,
    verify_challenge_response, x25519_shared_secret,
};
use super::framing::{read_frame, write_frame};
use super::session::{BtspCipher, BtspSession};
use super::types::{
    BTSP_HANDSHAKE_VERSION, ChallengeResponse, ClientHello, HandshakeComplete, HandshakeError,
    ServerHello,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use rand::RngCore;
use tracing::{debug, warn};

/// Run the server side of the BTSP handshake on `stream`.
///
/// On success returns a [`BtspSession`] ready for encrypted frame I/O.
/// On failure writes a [`HandshakeError`] frame and returns `Err`.
///
/// # Errors
///
/// Returns an error if any step of the handshake fails (I/O, crypto, or
/// verification). The caller should close the connection.
pub async fn perform_server_handshake<S>(
    stream: &mut S,
    family_seed: &FamilySeed,
) -> Result<BtspSession, BearDogError>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    // ── Derive handshake key from family seed ──────────────────────────
    let handshake_key = derive_handshake_key(family_seed.as_bytes())?;

    // ── Step 1: Read ClientHello ───────────────────────────────────────
    let client_hello_bytes = read_frame(stream).await?;
    let client_hello: ClientHello = serde_json::from_slice(&client_hello_bytes)
        .map_err(|e| BearDogError::system(format!("BTSP ClientHello parse failed: {e}")))?;

    if client_hello.version != BTSP_HANDSHAKE_VERSION {
        send_error(stream, "handshake_failed", "unsupported_version").await;
        return Err(BearDogError::system(format!(
            "BTSP version mismatch: client={}, server={}",
            client_hello.version, BTSP_HANDSHAKE_VERSION
        )));
    }

    let client_pub_bytes = BASE64
        .decode(&client_hello.client_ephemeral_pub)
        .map_err(|e| {
            BearDogError::system(format!("BTSP client ephemeral key decode failed: {e}"))
        })?;
    if client_pub_bytes.len() != 32 {
        send_error(stream, "handshake_failed", "invalid_key_length").await;
        return Err(BearDogError::system(
            "BTSP client ephemeral key is not 32 bytes".to_string(),
        ));
    }

    debug!("BTSP handshake: received ClientHello");

    // ── Step 2: Send ServerHello with challenge ────────────────────────
    let (server_secret, server_pub) = generate_ephemeral_keypair();
    let mut challenge = [0u8; 32];
    rand::rng().fill_bytes(&mut challenge);

    let server_hello = ServerHello {
        version: BTSP_HANDSHAKE_VERSION,
        server_ephemeral_pub: BASE64.encode(server_pub.as_bytes()),
        challenge: BASE64.encode(challenge),
    };

    let server_hello_json = serde_json::to_vec(&server_hello)
        .map_err(|e| BearDogError::system(format!("BTSP ServerHello serialize: {e}")))?;
    write_frame(stream, &server_hello_json).await?;

    debug!("BTSP handshake: sent ServerHello");

    // ── Step 3: Read ChallengeResponse ─────────────────────────────────
    let response_bytes = read_frame(stream).await?;
    let response: ChallengeResponse = serde_json::from_slice(&response_bytes)
        .map_err(|e| BearDogError::system(format!("BTSP ChallengeResponse parse failed: {e}")))?;

    let client_response = BASE64
        .decode(&response.response)
        .map_err(|e| BearDogError::system(format!("BTSP challenge response decode: {e}")))?;

    // Verify family membership
    if let Err(e) = verify_challenge_response(
        &handshake_key,
        &challenge,
        &client_pub_bytes,
        server_pub.as_bytes(),
        &client_response,
    ) {
        send_error(stream, "handshake_failed", "family_verification").await;
        warn!("BTSP handshake failed: family verification");
        return Err(e);
    }

    debug!("BTSP handshake: challenge verified");

    // ── Negotiate cipher ───────────────────────────────────────────────
    let cipher = BtspCipher::from_wire_name(&response.preferred_cipher)
        .unwrap_or(BtspCipher::ChaCha20Poly1305);

    // ── Generate session ID ────────────────────────────────────────────
    let mut session_id_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut session_id_bytes);
    let mut session_id = String::with_capacity(32);
    for b in &session_id_bytes {
        use std::fmt::Write;
        let _ = write!(session_id, "{b:02x}");
    }

    // ── Step 4: Send HandshakeComplete ─────────────────────────────────
    let complete = HandshakeComplete {
        cipher: cipher.wire_name().to_string(),
        session_id: session_id.clone(),
    };
    let complete_json = serde_json::to_vec(&complete)
        .map_err(|e| BearDogError::system(format!("BTSP HandshakeComplete serialize: {e}")))?;
    write_frame(stream, &complete_json).await?;

    debug!(session_id = %session_id, cipher = %cipher.wire_name(), "BTSP handshake complete");

    // ── Derive session keys from X25519 shared secret ──────────────────
    let client_pub_array: [u8; 32] = client_pub_bytes
        .try_into()
        .map_err(|_| BearDogError::system("BTSP client key length mismatch".to_string()))?;
    let their_pub = x25519_dalek::PublicKey::from(client_pub_array);
    let shared_secret = x25519_shared_secret(&server_secret, &their_pub);
    let keys = derive_session_keys(&shared_secret, session_id.as_bytes())?;

    Ok(BtspSession::new_server(
        session_id,
        cipher,
        keys.server_to_client,
        keys.client_to_server,
    ))
}

/// Best-effort: send a handshake error frame and swallow any I/O error.
async fn send_error<S: tokio::io::AsyncWrite + Unpin>(stream: &mut S, error: &str, reason: &str) {
    let msg = HandshakeError {
        error: error.to_string(),
        reason: reason.to_string(),
    };
    if let Ok(json) = serde_json::to_vec(&msg) {
        let _ = write_frame(stream, &json).await;
    }
}
