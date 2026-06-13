// SPDX-License-Identifier: AGPL-3.0-or-later

//! Per-connection protocol handlers for the Unix socket IPC server.
//!
//! Handles JSON-RPC (NDJSON), HTTP, and BTSP (both length-prefixed and
//! JSON-line framed) connections after protocol detection in [`super::server`].
//!
//! After a successful `btsp.negotiate` response (Phase 3), the connection
//! transparently transitions from NDJSON to encrypted frame I/O.

use super::server::{IPC_READ_TIMEOUT, UnixSocketIpcServer};
use crate::btsp_handshake::{self, BtspSession, Phase3Session};
use crate::method_gate::CallerContext;
use crate::platform::PlatformStream;
use crate::ribocipher;
use anyhow::Result;
use beardog_config::env_keys;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, warn};

impl UnixSocketIpcServer {
    /// Route a connection that sent a riboCipher signal prefix.
    ///
    /// Reads the tier-specific payload after the signal byte, resolves the
    /// protocol type, then dispatches to the appropriate handler.
    pub(super) async fn handle_ribocipher_signal(
        &self,
        mut stream: Box<dyn PlatformStream>,
        signal_byte: u8,
        family_seed: &btsp_handshake::FamilySeed,
    ) -> Result<()> {
        match signal_byte {
            ribocipher::SIGNAL_CLEAR => {
                let mut proto_buf = [0u8; 1];
                stream.read_exact(&mut proto_buf).await?;
                let protocol_type = proto_buf[0];

                info!(
                    protocol = ribocipher::protocol_name(protocol_type),
                    byte = format!("0x{:02X}", protocol_type),
                    "riboCipher: clear signal — routing"
                );

                match protocol_type {
                    ribocipher::PROTO_NDJSON_JSONRPC => {
                        self.handle_jsonrpc_universal("", stream).await
                    }
                    ribocipher::PROTO_BTSP_BINARY => {
                        match btsp_handshake::perform_server_handshake(
                            &mut stream,
                            family_seed,
                        )
                        .await
                        {
                            Ok(session) => {
                                info!(
                                    session_id = %session.session_id,
                                    cipher = %session.cipher.wire_name(),
                                    "riboCipher→BTSP handshake succeeded"
                                );
                                self.handle_jsonrpc_btsp(stream, session).await
                            }
                            Err(e) => {
                                warn!(error = %e, "riboCipher→BTSP handshake failed");
                                Ok(())
                            }
                        }
                    }
                    ribocipher::PROTO_BTSP_JSONLINE => {
                        let mut buf = Vec::with_capacity(512);
                        let mut buf_reader = BufReader::new(stream);
                        buf_reader.read_until(b'\n', &mut buf).await?;
                        let line = String::from_utf8_lossy(&buf);
                        if let Ok(hello) =
                            serde_json::from_str::<btsp_handshake::ClientHello>(line.trim())
                        {
                            let stream = buf_reader.into_inner();
                            self.handle_btsp_jsonline_connection(stream, &hello, family_seed)
                                .await
                        } else {
                            warn!("riboCipher: BTSP JSON-line signal but invalid ClientHello");
                            Ok(())
                        }
                    }
                    ribocipher::PROTO_HTTP => {
                        self.handle_http_universal("", stream).await
                    }
                    ribocipher::PROTO_PROBE => {
                        let response = serde_json::json!({
                            "status": "ok",
                            "primal": "bearDog",
                            "signal": "riboCipher-v1"
                        });
                        let msg = serde_json::to_string(&response)?;
                        stream.write_all(format!("{msg}\n").as_bytes()).await?;
                        stream.flush().await?;
                        Ok(())
                    }
                    _ => {
                        warn!(
                            protocol_type = format!("0x{:02X}", protocol_type),
                            "riboCipher: unknown protocol type in clear signal"
                        );
                        Ok(())
                    }
                }
            }
            ribocipher::SIGNAL_MITO => {
                let mut tag = [0u8; 4];
                stream.read_exact(&mut tag).await?;
                info!(
                    tag = format!("{:02X}{:02X}{:02X}{:02X}", tag[0], tag[1], tag[2], tag[3]),
                    "riboCipher: mito-obfuscated signal received (Tier 2 — future expansion)"
                );
                // Tier 2 decode: iterate known protocol types, HMAC-verify tag
                // For Wave 111: acknowledge and reject gracefully
                warn!("riboCipher: mito-tier not yet implemented — closing connection");
                Ok(())
            }
            ribocipher::SIGNAL_NUCLEAR => {
                let mut payload = [0u8; 6];
                stream.read_exact(&mut payload).await?;
                info!("riboCipher: nuclear-sealed signal received (Tier 3 — future expansion)");
                warn!("riboCipher: nuclear-tier not yet implemented — closing connection");
                Ok(())
            }
            _ => unreachable!("is_signal_byte guards this branch"),
        }
    }

    /// Handle JSON-RPC requests using universal platform stream.
    ///
    /// After writing a successful `btsp.negotiate` response, transitions the
    /// connection to Phase 3 encrypted frame I/O.
    pub(super) async fn handle_jsonrpc_universal(
        &self,
        first_line: &str,
        stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let mut caller = CallerContext::from_unix_with_peer(stream.peer_credentials());
        let mut buf_stream = BufReader::new(stream);

        if let Some(response) = self
            .handle_one_jsonrpc_request_universal(first_line, &mut caller)
            .await?
        {
            buf_stream.get_mut().write_all(response.as_bytes()).await?;
            buf_stream.get_mut().write_all(b"\n").await?;

            if let Some(session) = try_phase3_upgrade(first_line, &response) {
                buf_stream.get_mut().flush().await?;
                let stream = buf_stream.into_inner();
                return self.handle_jsonrpc_phase3(stream, session).await;
            }
        }

        let mut line_buf = Vec::with_capacity(1024);

        loop {
            line_buf.clear();

            let read_result = tokio::time::timeout(
                *IPC_READ_TIMEOUT,
                buf_stream.read_until(b'\n', &mut line_buf),
            )
            .await;

            match read_result {
                Err(_elapsed) => {
                    debug!(
                        timeout_secs = IPC_READ_TIMEOUT.as_secs(),
                        "IPC read timed out — closing idle connection"
                    );
                    return Ok(());
                }
                Ok(Ok(0)) => {
                    debug!("Client disconnected gracefully");
                    return Ok(());
                }
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    error!(error = %e, "Read error");
                    return Err(anyhow::anyhow!("Read failed: {e}"));
                }
            }

            let line = String::from_utf8_lossy(&line_buf);
            if line.trim().is_empty() {
                continue;
            }

            match self
                .handle_one_jsonrpc_request_universal(&line, &mut caller)
                .await
            {
                Ok(Some(response)) => {
                    if let Err(e) = buf_stream.get_mut().write_all(response.as_bytes()).await {
                        warn!(error = %e, "Failed to write response");
                        break;
                    }
                    if let Err(e) = buf_stream.get_mut().write_all(b"\n").await {
                        warn!(error = %e, "Failed to write newline");
                        break;
                    }

                    if let Some(session) = try_phase3_upgrade(&line, &response) {
                        let _ = buf_stream.get_mut().flush().await;
                        let stream = buf_stream.into_inner();
                        return self.handle_jsonrpc_phase3(stream, session).await;
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    warn!(error = %e, "Error handling request");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle HTTP requests using universal platform stream.
    pub(super) async fn handle_http_universal(
        &self,
        _first_line: &str,
        mut stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let response = b"HTTP/1.1 501 Not Implemented\r\nContent-Length: 50\r\n\r\nHTTP deprecated - use JSON-RPC over Unix sockets\n";
        stream.write_all(response).await?;
        Ok(())
    }

    /// Handle a UDS connection that began with a JSON-line BTSP `ClientHello`.
    ///
    /// Completes the 4-step handshake (steps 2–4) using newline-delimited JSON,
    /// then transitions to the appropriate post-handshake handler:
    /// - `Null` cipher → plain NDJSON JSON-RPC loop
    /// - Encrypted cipher → length-prefixed encrypted frame loop
    pub(super) async fn handle_btsp_jsonline_connection(
        &self,
        mut stream: Box<dyn PlatformStream>,
        client_hello: &btsp_handshake::ClientHello,
        family_seed: &btsp_handshake::FamilySeed,
    ) -> Result<()> {
        match btsp_handshake::continue_server_handshake_jsonline(
            &mut stream,
            client_hello,
            family_seed,
        )
        .await
        {
            Ok(session) => {
                info!(
                    session_id = %session.session_id,
                    cipher = %session.cipher.wire_name(),
                    "BTSP handshake succeeded (JSON-line framed)"
                );
                if session.cipher == btsp_handshake::BtspCipher::Null {
                    self.handle_jsonrpc_ndjson_loop(stream).await
                } else {
                    self.handle_jsonrpc_btsp(stream, session).await
                }
            }
            Err(e) => {
                warn!(error = %e, "BTSP handshake failed (JSON-line)");
                Ok(())
            }
        }
    }

    /// Read NDJSON JSON-RPC requests in a loop (post-handshake, null cipher).
    ///
    /// Transitions to Phase 3 encrypted frame I/O if `btsp.negotiate`
    /// succeeds with a non-null cipher.
    pub(super) async fn handle_jsonrpc_ndjson_loop(
        &self,
        stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let mut caller = CallerContext::from_unix_with_peer(stream.peer_credentials());
        let mut buf_stream = BufReader::new(stream);
        let mut line_buf = Vec::with_capacity(1024);

        loop {
            line_buf.clear();

            let read_result = tokio::time::timeout(
                *IPC_READ_TIMEOUT,
                buf_stream.read_until(b'\n', &mut line_buf),
            )
            .await;

            match read_result {
                Err(_) => {
                    debug!("Post-handshake NDJSON read timed out — closing connection");
                    return Ok(());
                }
                Ok(Ok(0)) => {
                    debug!("Client disconnected after BTSP handshake");
                    return Ok(());
                }
                Ok(Ok(_)) => {}
                Ok(Err(e)) => return Err(anyhow::anyhow!("Post-handshake read: {e}")),
            }

            let line = String::from_utf8_lossy(&line_buf);
            if line.trim().is_empty() {
                continue;
            }

            if let Some(response) = self
                .handle_one_jsonrpc_request_universal(line.as_ref(), &mut caller)
                .await?
            {
                buf_stream.get_mut().write_all(response.as_bytes()).await?;
                buf_stream.get_mut().write_all(b"\n").await?;

                if let Some(session) = try_phase3_upgrade(&line, &response) {
                    let _ = buf_stream.get_mut().flush().await;
                    let stream = buf_stream.into_inner();
                    return self.handle_jsonrpc_phase3(stream, session).await;
                }
            }
        }
    }

    /// Handle JSON-RPC over BTSP encrypted frames (Phase 2 — counter nonces).
    ///
    /// Each frame is decrypted → parsed as JSON-RPC → processed → encrypted → sent.
    pub(super) async fn handle_jsonrpc_btsp(
        &self,
        mut stream: Box<dyn PlatformStream>,
        mut session: BtspSession,
    ) -> Result<()> {
        let mut caller = CallerContext::from_unix_with_peer(stream.peer_credentials());
        caller.btsp_family_verified = true;
        loop {
            let frame = match btsp_handshake::read_frame(&mut stream).await {
                Ok(f) => f,
                Err(e) => {
                    debug!(error = %e, "BTSP frame read ended");
                    return Ok(());
                }
            };

            let plaintext = match session.decrypt_frame(&frame) {
                Ok(p) => p,
                Err(e) => {
                    warn!(error = %e, "BTSP frame decrypt failed — dropping connection");
                    return Ok(());
                }
            };

            let line = match String::from_utf8(plaintext) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "BTSP frame not valid UTF-8");
                    continue;
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            if let Some(response_str) = self
                .handle_one_jsonrpc_request_universal(&line, &mut caller)
                .await?
            {
                let encrypted = session
                    .encrypt_frame(response_str.as_bytes())
                    .map_err(|e| anyhow::anyhow!("BTSP encrypt failed: {e}"))?;

                btsp_handshake::write_frame(&mut stream, &encrypted)
                    .await
                    .map_err(|e| anyhow::anyhow!("BTSP frame write failed: {e}"))?;
            }
        }
    }

    /// Handle JSON-RPC over Phase 3 encrypted frames (random nonces).
    ///
    /// Entered after a successful `btsp.negotiate` response. Each subsequent
    /// message uses length-prefixed encrypted framing:
    /// `[4B len BE u32][12B random nonce][ciphertext + Poly1305 tag]`
    pub(super) async fn handle_jsonrpc_phase3(
        &self,
        mut stream: Box<dyn PlatformStream>,
        session: Phase3Session,
    ) -> Result<()> {
        let mut caller = CallerContext::from_unix_with_peer(stream.peer_credentials());
        info!("BTSP Phase 3: encrypted frame I/O active");
        loop {
            let frame = match btsp_handshake::read_frame(&mut stream).await {
                Ok(f) => f,
                Err(e) => {
                    debug!(error = %e, "Phase 3 frame read ended");
                    return Ok(());
                }
            };

            let plaintext = match session.decrypt_frame(&frame) {
                Ok(p) => p,
                Err(e) => {
                    warn!(error = %e, "Phase 3 frame decrypt failed — dropping connection");
                    return Ok(());
                }
            };

            let line = match String::from_utf8(plaintext) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "Phase 3 frame not valid UTF-8");
                    continue;
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            if let Some(response_str) = self
                .handle_one_jsonrpc_request_universal(&line, &mut caller)
                .await?
            {
                let encrypted = session
                    .encrypt_frame(response_str.as_bytes())
                    .map_err(|e| anyhow::anyhow!("Phase 3 encrypt failed: {e}"))?;

                btsp_handshake::write_frame(&mut stream, &encrypted)
                    .await
                    .map_err(|e| anyhow::anyhow!("Phase 3 frame write failed: {e}"))?;
            }
        }
    }
}

/// Detect a successful Phase 3 negotiate from a request/response pair and
/// derive session keys for the encrypted frame transition.
///
/// Returns `Some(Phase3Session)` when the request was `btsp.negotiate` and the
/// server selected a non-null cipher. Returns `None` for all other methods or
/// when the null cipher was selected.
fn try_phase3_upgrade(request_line: &str, response_str: &str) -> Option<Phase3Session> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;

    let req: serde_json::Value = serde_json::from_str(request_line.trim()).ok()?;
    if req.get("method")?.as_str()? != "btsp.negotiate" {
        return None;
    }

    let resp: serde_json::Value = serde_json::from_str(response_str.trim()).ok()?;
    let result = resp.get("result")?;
    let cipher = result.get("cipher")?.as_str()?;
    if cipher == "null" {
        return None;
    }

    let server_nonce_b64 = result.get("server_nonce")?.as_str()?;
    let client_nonce_b64 = req.get("params")?.get("client_nonce")?.as_str()?;

    let client_nonce = BASE64.decode(client_nonce_b64).ok()?;
    let server_nonce = BASE64.decode(server_nonce_b64).ok()?;

    let family_seed = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED)
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED))
        .ok()
        .filter(|s| s.len() >= 16)
        .map(String::into_bytes)?;

    let handshake_key = btsp_handshake::crypto::derive_handshake_key(&family_seed).ok()?;
    let keys = btsp_handshake::crypto::derive_phase3_session_keys(
        &handshake_key,
        &client_nonce,
        &server_nonce,
    )
    .ok()?;

    info!(
        cipher = cipher,
        "BTSP Phase 3: transitioning connection to encrypted frame I/O"
    );

    Some(Phase3Session::new(keys))
}
