// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Client for `BearDog`
//!
//! Universal JSON-RPC client over TCP. Cleartext [`TcpIpcClient`] serves local
//! biomeOS composition; [`BtspTcpClient`] performs the production BTSP handshake
//! for WAN mesh connections.

use std::borrow::Cow;
use std::net::SocketAddr;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info, warn};

use crate::btsp_handshake::{
    FamilySeed,
    crypto::{
        compute_challenge_hmac, derive_handshake_key, derive_session_keys,
        generate_ephemeral_keypair, x25519_shared_secret,
    },
    framing::{read_frame, write_frame},
    session::{BtspCipher, BtspSession},
    types::{
        BTSP_HANDSHAKE_VERSION, ChallengeResponse, ClientHello, HandshakeComplete,
        HandshakeError, ServerHello,
    },
};
use crate::ribocipher;

/// TCP IPC Client
pub struct TcpIpcClient {
    server_addr: SocketAddr,
}

impl TcpIpcClient {
    /// Create new TCP client
    pub const fn new(server_addr: SocketAddr) -> Self {
        Self { server_addr }
    }

    /// Call a JSON-RPC method
    ///
    /// # Errors
    ///
    /// Returns an error if the TCP connection fails, request/response I/O fails, JSON serialization
    /// or parsing fails, or the RPC returns an error.
    pub async fn call(&self, method: &str, params: Option<Value>) -> Result<Value, BearDogError> {
        debug!("📞 Calling {}", method);

        // Connect to server
        let stream = TcpStream::connect(self.server_addr).await.map_err(|e| {
            BearDogError::system(format!("Failed to connect to {}: {}", self.server_addr, e))
        })?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = tokio::io::BufReader::new(reader);

        // riboCipher: signal clear NDJSON JSON-RPC before payload
        writer
            .write_all(&ribocipher::clear_signal(ribocipher::PROTO_NDJSON_JSONRPC))
            .await
            .map_err(|e| BearDogError::system(format!("Failed to send riboCipher signal: {e}")))?;

        // Build JSON-RPC request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        // Send request (serialization of valid json! macro value is infallible, but handle gracefully)
        let request_str = serde_json::to_string(&request)
            .map_err(|e| BearDogError::system(format!("Failed to serialize request: {e}")))?
            + "\n";
        writer
            .write_all(request_str.as_bytes())
            .await
            .map_err(|e| BearDogError::system(format!("Failed to send request: {e}")))?;

        // Read response
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read response: {e}")))?;

        // Parse response
        let response: Value = serde_json::from_str(&response_line)
            .map_err(|e| BearDogError::system(format!("Invalid JSON response: {e}")))?;

        // Check for error
        if let Some(error) = response.get("error") {
            return Err(BearDogError::system(format!("RPC error: {error}")));
        }

        // Return result
        response
            .get("result")
            .cloned()
            .ok_or_else(|| BearDogError::system("No result in response".to_string()))
    }
}

/// BTSP-encrypted TCP client for production WAN mesh connections.
pub struct BtspTcpClient {
    server_addr: SocketAddr,
    family_seed: FamilySeed,
}

impl BtspTcpClient {
    /// Create a new BTSP TCP client.
    #[must_use]
    pub fn new(server_addr: SocketAddr, family_seed: FamilySeed) -> Self {
        Self {
            server_addr,
            family_seed,
        }
    }

    /// Connect to the server and complete the 4-step BTSP handshake.
    ///
    /// # Errors
    ///
    /// Returns an error if the TCP connection fails, any handshake step fails,
    /// or the server rejects family membership.
    pub async fn connect_and_handshake(&self) -> Result<BtspConnection, BearDogError> {
        info!(addr = %self.server_addr, "BTSP TCP client connecting");
        let mut stream = TcpStream::connect(self.server_addr).await.map_err(|e| {
            BearDogError::system(format!("Failed to connect to {}: {}", self.server_addr, e))
        })?;
        let session = perform_client_handshake(&mut stream, &self.family_seed).await?;
        info!(
            session_id = %session.session_id,
            cipher = %session.cipher.wire_name(),
            "BTSP TCP handshake complete"
        );
        Ok(BtspConnection { stream, session })
    }
}

/// Established BTSP-encrypted TCP connection after a successful handshake.
pub struct BtspConnection {
    stream: TcpStream,
    session: BtspSession,
}

impl BtspConnection {
    /// BTSP session identifier from the completed handshake.
    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session.session_id
    }

    /// Call a JSON-RPC method over the encrypted BTSP frame channel.
    ///
    /// # Errors
    ///
    /// Returns an error if frame I/O, encryption/decryption, JSON serialization or parsing fails,
    /// or the RPC returns an error response.
    pub async fn call(
        &mut self,
        method: Cow<'static, str>,
        params: Option<Value>,
    ) -> Result<Value, BearDogError> {
        debug!(method = %method, "BTSP encrypted RPC call");
        encrypted_jsonrpc_call(&mut self.stream, &mut self.session, method, params).await
    }

    /// Shut down the TCP connection cleanly.
    ///
    /// # Errors
    ///
    /// Returns an error if the TCP shutdown syscall fails.
    pub async fn close(mut self) -> Result<(), BearDogError> {
        use tokio::io::AsyncWriteExt;
        self.stream
            .shutdown()
            .await
            .map_err(|e| BearDogError::system(format!("BTSP TCP shutdown failed: {e}")))
    }
}

/// Run the client side of the BTSP handshake on `stream`.
///
/// # Errors
///
/// Returns an error if any handshake step fails (I/O, crypto, or server rejection).
pub(crate) async fn perform_client_handshake<S>(
    stream: &mut S,
    family_seed: &FamilySeed,
) -> Result<BtspSession, BearDogError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let handshake_key = derive_handshake_key(family_seed.as_bytes())?;

    let (client_secret, client_pub) = generate_ephemeral_keypair();
    let hello = ClientHello {
        version: BTSP_HANDSHAKE_VERSION,
        client_ephemeral_pub: BASE64.encode(client_pub.as_bytes()),
    };
    let hello_json = serde_json::to_vec(&hello)
        .map_err(|e| BearDogError::system(format!("BTSP ClientHello serialize: {e}")))?;
    write_frame(stream, &hello_json).await?;
    debug!("BTSP client: sent ClientHello");

    let server_hello_bytes = read_frame(stream).await?;
    let server_hello: ServerHello = serde_json::from_slice(&server_hello_bytes)
        .map_err(|e| BearDogError::system(format!("BTSP ServerHello parse failed: {e}")))?;

    if server_hello.version != BTSP_HANDSHAKE_VERSION {
        return Err(BearDogError::system(format!(
            "BTSP version mismatch: client={}, server={}",
            BTSP_HANDSHAKE_VERSION, server_hello.version
        )));
    }

    let server_pub_bytes = BASE64
        .decode(&server_hello.server_ephemeral_pub)
        .map_err(|e| BearDogError::system(format!("BTSP server ephemeral key decode: {e}")))?;
    if server_pub_bytes.len() != 32 {
        return Err(BearDogError::system(
            "BTSP server ephemeral key is not 32 bytes".to_string(),
        ));
    }

    let challenge = BASE64
        .decode(&server_hello.challenge)
        .map_err(|e| BearDogError::system(format!("BTSP challenge decode: {e}")))?;

    let hmac = compute_challenge_hmac(
        &handshake_key,
        &challenge,
        client_pub.as_bytes(),
        &server_pub_bytes,
    )?;

    let response = ChallengeResponse {
        response: BASE64.encode(hmac),
        preferred_cipher: BtspCipher::ChaCha20Poly1305.wire_name().to_string(),
    };
    let response_json = serde_json::to_vec(&response)
        .map_err(|e| BearDogError::system(format!("BTSP ChallengeResponse serialize: {e}")))?;
    write_frame(stream, &response_json).await?;
    debug!("BTSP client: sent ChallengeResponse");

    let complete_bytes = read_frame(stream).await?;
    if let Ok(err) = serde_json::from_slice::<HandshakeError>(&complete_bytes) {
        warn!(error = %err.error, reason = %err.reason, "BTSP handshake rejected by server");
        return Err(BearDogError::security(format!(
            "BTSP handshake failed: {} ({})",
            err.error, err.reason
        )));
    }

    let complete: HandshakeComplete = serde_json::from_slice(&complete_bytes).map_err(|e| {
        BearDogError::system(format!("BTSP HandshakeComplete parse failed: {e}"))
    })?;

    // Verify server's family membership proof (mutual auth)
    if !complete.server_proof.is_empty() {
        let expected_proof = compute_challenge_hmac(
            &handshake_key,
            &challenge,
            &server_pub_bytes,
            client_pub.as_bytes(),
        )?;
        let server_proof_bytes = BASE64
            .decode(&complete.server_proof)
            .map_err(|e| BearDogError::system(format!("BTSP server proof decode: {e}")))?;
        if server_proof_bytes.as_slice() != expected_proof.as_slice() {
            return Err(BearDogError::system(
                "BTSP mutual auth failed: server proof invalid".to_string(),
            ));
        }
        debug!("BTSP mutual auth: server family membership verified");
    }

    let server_pub_array: [u8; 32] = server_pub_bytes
        .try_into()
        .map_err(|_| BearDogError::system("BTSP server key length mismatch".to_string()))?;
    let their_pub = x25519_dalek::PublicKey::from(server_pub_array);
    let shared_secret = x25519_shared_secret(&client_secret, &their_pub);
    let keys = derive_session_keys(&shared_secret, complete.session_id.as_bytes())?;

    let cipher = BtspCipher::from_wire_name(&complete.cipher)
        .unwrap_or(BtspCipher::ChaCha20Poly1305);

    Ok(BtspSession::new_client(
        complete.session_id,
        cipher,
        keys.client_to_server,
        keys.server_to_client,
    ))
}

async fn encrypted_jsonrpc_call<S>(
    stream: &mut S,
    session: &mut BtspSession,
    method: Cow<'static, str>,
    params: Option<Value>,
) -> Result<Value, BearDogError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let request_bytes = serde_json::to_vec(&request)
        .map_err(|e| BearDogError::system(format!("Failed to serialize request: {e}")))?;

    let ciphertext = session
        .encrypt_frame(&request_bytes)
        .map_err(|e| BearDogError::system(format!("BTSP encrypt request: {e}")))?;
    write_frame(stream, &ciphertext).await?;

    let response_frame = read_frame(stream).await?;
    let plaintext = session
        .decrypt_frame(&response_frame)
        .map_err(|e| BearDogError::system(format!("BTSP decrypt response: {e}")))?;

    let response_str = String::from_utf8(plaintext)
        .map_err(|e| BearDogError::system(format!("BTSP response not valid UTF-8: {e}")))?;
    let response: Value = serde_json::from_str(response_str.trim())
        .map_err(|e| BearDogError::system(format!("Invalid JSON response: {e}")))?;

    if let Some(error) = response.get("error") {
        return Err(BearDogError::system(format!("RPC error: {error}")));
    }

    response
        .get("result")
        .cloned()
        .ok_or_else(|| BearDogError::system("No result in response".to_string()))
}

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    use super::*;
    use crate::btsp_handshake::perform_server_handshake;
    use tokio::io::duplex;

    #[tokio::test]
    async fn btsp_client_handshake_roundtrip() {
        let seed = b"test-family-seed-for-handshake!!";
        let family_seed = FamilySeed::new(seed.to_vec());
        let (mut client_stream, mut server_stream) = duplex(16384);

        let server_fut = perform_server_handshake(&mut server_stream, &family_seed);
        let client_fut = perform_client_handshake(&mut client_stream, &family_seed);

        let (server_result, client_result) = tokio::join!(server_fut, client_fut);
        let mut server_session = server_result.expect("server handshake");
        let mut client_session = client_result.expect("client handshake");

        assert_eq!(server_session.session_id, client_session.session_id);
        assert_eq!(server_session.cipher, client_session.cipher);

        let msg = b"hello from server";
        let encrypted = server_session.encrypt_frame(msg).expect("encrypt");
        let decrypted = client_session.decrypt_frame(&encrypted).expect("decrypt");
        assert_eq!(decrypted, msg);

        let msg2 = b"hello from client";
        let encrypted2 = client_session.encrypt_frame(msg2).expect("encrypt");
        let decrypted2 = server_session.decrypt_frame(&encrypted2).expect("decrypt");
        assert_eq!(decrypted2, msg2);
    }

    #[tokio::test]
    async fn btsp_client_call_encrypted() {
        let seed_bytes = b"test-family-seed-for-handshake!!";
        let family_seed = FamilySeed::new(seed_bytes.to_vec());
        let (mut client_stream, mut server_stream) = duplex(16384);

        let server_fut = async {
            let mut session = perform_server_handshake(&mut server_stream, &family_seed).await?;

            let frame = read_frame(&mut server_stream).await?;
            let plain = session.decrypt_frame(&frame)?;
            let request: Value = serde_json::from_slice(&plain)
                .map_err(|e| BearDogError::system(format!("JSON parse: {e}")))?;
            assert_eq!(request["method"], "test.echo");

            let resp = serde_json::json!({"jsonrpc":"2.0","result":{"echoed":true},"id":1});
            let enc = session.encrypt_frame(resp.to_string().as_bytes())?;
            write_frame(&mut server_stream, &enc).await?;
            Ok::<(), BearDogError>(())
        };

        let client_fut = async {
            let mut session = perform_client_handshake(&mut client_stream, &family_seed).await?;
            let result = encrypted_jsonrpc_call(
                &mut client_stream,
                &mut session,
                Cow::Borrowed("test.echo"),
                Some(serde_json::json!({"msg": "hi"})),
            )
            .await?;
            assert_eq!(result["echoed"], true);
            Ok::<(), BearDogError>(())
        };

        let (server_result, client_result) = tokio::join!(server_fut, client_fut);
        server_result.expect("server side");
        client_result.expect("client call");
    }
}
