// SPDX-License-Identifier: AGPL-3.0-or-later

//! Per-connection protocol handlers for the Unix socket IPC server.
//!
//! Handles JSON-RPC (NDJSON), HTTP, and BTSP (both length-prefixed and
//! JSON-line framed) connections after protocol detection in [`super::server`].

use super::server::{IPC_READ_TIMEOUT, UnixSocketIpcServer};
use crate::btsp_handshake::{self, BtspSession};
use crate::platform::PlatformStream;
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, warn};

impl UnixSocketIpcServer {
    /// Handle JSON-RPC requests using universal platform stream.
    pub(super) async fn handle_jsonrpc_universal(
        &self,
        first_line: &str,
        stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let mut buf_stream = BufReader::new(stream);

        if let Some(response) = self
            .handle_one_jsonrpc_request_universal(first_line)
            .await?
        {
            buf_stream.get_mut().write_all(response.as_bytes()).await?;
            buf_stream.get_mut().write_all(b"\n").await?;
        }

        let mut line_buf = Vec::with_capacity(1024);

        loop {
            line_buf.clear();

            let read_result = tokio::time::timeout(
                IPC_READ_TIMEOUT,
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

            match self.handle_one_jsonrpc_request_universal(&line).await {
                Ok(Some(response)) => {
                    if let Err(e) = buf_stream.get_mut().write_all(response.as_bytes()).await {
                        warn!(error = %e, "Failed to write response");
                        break;
                    }
                    if let Err(e) = buf_stream.get_mut().write_all(b"\n").await {
                        warn!(error = %e, "Failed to write newline");
                        break;
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
    pub(super) async fn handle_jsonrpc_ndjson_loop(
        &self,
        stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let mut buf_stream = BufReader::new(stream);
        let mut line_buf = Vec::with_capacity(1024);

        loop {
            line_buf.clear();

            let read_result = tokio::time::timeout(
                IPC_READ_TIMEOUT,
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
                .handle_one_jsonrpc_request_universal(line.as_ref())
                .await?
            {
                buf_stream.get_mut().write_all(response.as_bytes()).await?;
                buf_stream.get_mut().write_all(b"\n").await?;
            }
        }
    }

    /// Handle JSON-RPC over BTSP encrypted frames (production mode).
    ///
    /// Each frame is decrypted → parsed as JSON-RPC → processed → encrypted → sent.
    pub(super) async fn handle_jsonrpc_btsp(
        &self,
        mut stream: Box<dyn PlatformStream>,
        mut session: BtspSession,
    ) -> Result<()> {
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

            if let Some(response_str) = self.handle_one_jsonrpc_request_universal(&line).await? {
                let encrypted = session
                    .encrypt_frame(response_str.as_bytes())
                    .map_err(|e| anyhow::anyhow!("BTSP encrypt failed: {e}"))?;

                btsp_handshake::write_frame(&mut stream, &encrypted)
                    .await
                    .map_err(|e| anyhow::anyhow!("BTSP frame write failed: {e}"))?;
            }
        }
    }
}
