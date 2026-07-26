// SPDX-License-Identifier: AGPL-3.0-or-later

use super::TcpIpcServer;
use crate::btsp_handshake::{self, BtspSecurityMode};
use crate::btsp_provider::BeardogBtspProvider;
use crate::method_gate::{CallerContext, MethodGate, dispatch_auth_method};
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_errors::BearDogError;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tracing::{debug, error, info, warn};

use super::TCP_READ_TIMEOUT;

impl TcpIpcServer {
    /// Handle a single TCP connection
    pub(super) async fn handle_connection(
        mut stream: TcpStream,
        registry: Arc<HandlerRegistry>,
        btsp_provider: Arc<BeardogBtspProvider>,
        security_mode: BtspSecurityMode,
        gate: Arc<MethodGate>,
    ) -> Result<(), BearDogError> {
        let peer_addr = stream
            .peer_addr()
            .map_err(|e| BearDogError::system(format!("Failed to get peer address: {e}")))?;

        let mut caller = caller_context_from_addr(&peer_addr);

        debug!("Handling connection from: {}", peer_addr);

        // ── BTSP production mode with protocol auto-detection ──────────
        //
        // Peek the first byte to distinguish BTSP binary framing from plain
        // JSON-RPC text. BTSP frames start with a 4-byte big-endian length;
        // JSON-RPC starts with '{' (0x7B). This allows biomeOS (the local
        // composition substrate) to forward capability.call via plain JSON-RPC
        // over TCP without requiring BTSP client implementation, while external
        // connections still get full BTSP enforcement.
        if let BtspSecurityMode::Production { ref family_seed } = security_mode {
            let mut peek_buf = [0u8; 1];
            match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                stream.peek(&mut peek_buf),
            )
            .await
            {
                Ok(Ok(1)) if peek_buf[0] == b'{' => {
                    debug!(
                        peer = %peer_addr,
                        "TCP peek: JSON-RPC detected (0x7B) — bypassing BTSP for local composition"
                    );
                }
                _ => {
                    debug!(peer = %peer_addr, "BTSP production: initiating TCP handshake");
                    match btsp_handshake::perform_server_handshake(&mut stream, family_seed).await {
                        Ok(mut session) => {
                            info!(
                                peer = %peer_addr,
                                session_id = %session.session_id,
                                cipher = %session.cipher.wire_name(),
                                "BTSP TCP handshake succeeded"
                            );
                            caller.btsp_family_verified = true;
                            caller.peer_id = Some(session.session_id.clone());
                            return Self::handle_jsonrpc_btsp_tcp(
                                &mut stream,
                                &mut session,
                                &registry,
                                &btsp_provider,
                                &gate,
                                &mut caller,
                            )
                            .await;
                        }
                        Err(e) => {
                            warn!(peer = %peer_addr, error = %e, "BTSP TCP handshake failed");
                            return Ok(());
                        }
                    }
                }
            }
        }

        // ── Plain NDJSON (development mode or JSON-RPC auto-detected) ──
        let (reader, writer) = stream.into_split();
        Self::handle_plaintext_connection(
            reader,
            writer,
            peer_addr,
            registry,
            btsp_provider,
            &gate,
            &mut caller,
        )
        .await
    }

    /// Handle a plaintext NDJSON connection over any async reader/writer.
    ///
    /// Shared by both cleartext TCP and TLS-terminated connections.
    pub(super) async fn handle_plaintext_connection<R, W>(
        reader: R,
        mut writer: W,
        peer_addr: SocketAddr,
        registry: Arc<HandlerRegistry>,
        btsp_provider: Arc<BeardogBtspProvider>,
        gate: &MethodGate,
        caller: &mut CallerContext,
    ) -> Result<(), BearDogError>
    where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();

            let read_result =
                tokio::time::timeout(*TCP_READ_TIMEOUT, reader.read_line(&mut line)).await;

            let bytes_read = match read_result {
                Err(_elapsed) => {
                    warn!(
                        peer = %peer_addr,
                        timeout_secs = TCP_READ_TIMEOUT.as_secs(),
                        "TCP read timed out — closing idle connection"
                    );
                    break;
                }
                Ok(Err(e)) => {
                    warn!("Read error from {}: {}", peer_addr, e);
                    break;
                }
                Ok(Ok(n)) => n,
            };

            match bytes_read {
                0 => {
                    debug!("Connection closed by peer: {}", peer_addr);
                    break;
                }
                n => {
                    debug!("Received {} bytes from {}", n, peer_addr);

                    let request_str = line.trim();
                    if request_str.is_empty() {
                        continue;
                    }

                    let request: Value = match serde_json::from_str(request_str) {
                        Ok(req) => req,
                        Err(e) => {
                            warn!("Invalid JSON from {}: {}", peer_addr, e);
                            let error_response = serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                    "code": -32700,
                                    "message": format!("Parse error: {}", e)
                                },
                                "id": null
                            });
                            if let Ok(response) = serde_json::to_string(&error_response) {
                                let response = response + "\n";
                                if let Err(e) = writer.write_all(response.as_bytes()).await {
                                    warn!("Failed to write error response to client: {}", e);
                                }
                            }
                            continue;
                        }
                    };

                    let method = request["method"].as_str().unwrap_or("");
                    let params = request.get("params").cloned();
                    let id = request.get("id").cloned();

                    if let Some(token) = params
                        .as_ref()
                        .and_then(|p| p.get("_bearer_token"))
                        .and_then(|v| v.as_str())
                    {
                        caller.bearer_token = Some(token.to_owned());
                    }

                    debug!("Request: {} (id: {:?})", method, id);

                    if let Some(result) =
                        dispatch_auth_method(method, gate, caller, params.as_ref())
                    {
                        let json_response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": result,
                            "id": id
                        });
                        if let Ok(s) = serde_json::to_string(&json_response) {
                            let _ = writer.write_all((s + "\n").as_bytes()).await;
                        }
                        continue;
                    }

                    if let Err(gate_err) = gate.check(method, caller) {
                        let json_response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": gate_err.code,
                                "message": gate_err.message,
                                "data": gate_err.data,
                            },
                            "id": id
                        });
                        if let Ok(s) = serde_json::to_string(&json_response) {
                            let _ = writer.write_all((s + "\n").as_bytes()).await;
                        }
                        continue;
                    }

                    let response = registry
                        .route(method, params.as_ref(), &btsp_provider)
                        .await;

                    let json_response = match response {
                        Ok(result) => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": result,
                            "id": id
                        }),
                        Err(e) => serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32601,
                                "message": format!("Business error: {}", e)
                            },
                            "id": id
                        }),
                    };

                    let response_str = match serde_json::to_string(&json_response) {
                        Ok(s) => s + "\n",
                        Err(e) => {
                            error!("Failed to serialize response: {}", e);
                            continue;
                        }
                    };
                    if let Err(e) = writer.write_all(response_str.as_bytes()).await {
                        error!("Failed to write response to {}: {}", peer_addr, e);
                        break;
                    }

                    debug!("Response sent to {}", peer_addr);
                }
            }
        }

        debug!("Connection handler finished: {}", peer_addr);
        Ok(())
    }
}

/// Create a `CallerContext` based on the TCP peer address.
const fn caller_context_from_addr(addr: &SocketAddr) -> CallerContext {
    if addr.ip().is_loopback() {
        CallerContext::loopback()
    } else {
        CallerContext::remote()
    }
}
