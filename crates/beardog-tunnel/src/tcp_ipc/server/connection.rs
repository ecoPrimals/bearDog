// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP connection handling: BTSP handshake, plaintext NDJSON, and encrypted JSON-RPC.

use super::{TCP_HANDSHAKE_DETECT_TIMEOUT, TCP_READ_TIMEOUT, TcpIpcServer};
use crate::btsp_handshake::{self, BtspSecurityMode, BtspSession};
use crate::btsp_provider::BeardogBtspProvider;
use crate::method_gate::{CallerContext, MethodGate, dispatch_auth_method};
use crate::ribocipher;
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_errors::BearDogError;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tracing::{debug, error, info, warn};

impl TcpIpcServer {
    /// Handle a single TCP connection
    pub(crate) async fn handle_connection(
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

        if let BtspSecurityMode::Production { ref family_seed } = security_mode {
            // ── riboCipher signal detection (Wave 111) ──────────────────
            //
            // Peek first byte. If riboCipher signal prefix, consume and route
            // deterministically. Otherwise ERROR (deprecated unsignalled) and
            // fall through to legacy peek-and-guess.
            let mut peek_buf = [0u8; 1];
            match tokio::time::timeout(*TCP_HANDSHAKE_DETECT_TIMEOUT, stream.peek(&mut peek_buf))
                .await
            {
                Ok(Ok(1)) if ribocipher::is_signal_byte(peek_buf[0]) => {
                    // Consume the signal byte (peek doesn't consume)
                    let mut consume = [0u8; 1];
                    stream
                        .read_exact(&mut consume)
                        .await
                        .map_err(|e| BearDogError::system(format!("riboCipher read: {e}")))?;

                    return Self::handle_tcp_ribocipher(
                        stream,
                        consume[0],
                        family_seed,
                        &registry,
                        &btsp_provider,
                        &gate,
                        &peer_addr,
                    )
                    .await;
                }
                Ok(Ok(1)) if peek_buf[0] == b'{' => {
                    error!(
                        peer = %peer_addr,
                        first_byte = "0x7B",
                        "DEPRECATED: unsignalled connection — use riboCipher signal [0xEC, 0x01] for JSON-RPC"
                    );
                    debug!(
                        peer = %peer_addr,
                        "TCP peek: JSON-RPC detected (0x7B) — bypassing BTSP for local composition"
                    );
                }
                _ => {
                    error!(
                        peer = %peer_addr,
                        first_byte = format!("0x{:02X}", peek_buf[0]),
                        "DEPRECATED: unsignalled connection — use riboCipher signal [0xEC, 0x02] for BTSP"
                    );
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

    /// Route a TCP connection that sent a riboCipher signal prefix.
    async fn handle_tcp_ribocipher(
        mut stream: TcpStream,
        signal_byte: u8,
        family_seed: &btsp_handshake::FamilySeed,
        registry: &Arc<HandlerRegistry>,
        btsp_provider: &Arc<BeardogBtspProvider>,
        gate: &Arc<MethodGate>,
        peer_addr: &SocketAddr,
    ) -> Result<(), BearDogError> {
        match signal_byte {
            ribocipher::SIGNAL_CLEAR => {
                let mut proto_buf = [0u8; 1];
                stream
                    .read_exact(&mut proto_buf)
                    .await
                    .map_err(|e| BearDogError::system(format!("riboCipher proto read: {e}")))?;
                let protocol_type = proto_buf[0];

                info!(
                    peer = %peer_addr,
                    protocol = ribocipher::protocol_name(protocol_type),
                    byte = format!("0x{:02X}", protocol_type),
                    "riboCipher: clear signal — routing"
                );

                match protocol_type {
                    ribocipher::PROTO_NDJSON_JSONRPC => {
                        let mut caller = caller_context_from_addr(peer_addr);
                        let (reader, writer) = stream.into_split();
                        Self::handle_plaintext_connection(
                            reader,
                            writer,
                            *peer_addr,
                            Arc::clone(registry),
                            Arc::clone(btsp_provider),
                            gate,
                            &mut caller,
                        )
                        .await
                    }
                    ribocipher::PROTO_BTSP_BINARY => {
                        match btsp_handshake::perform_server_handshake(&mut stream, family_seed)
                            .await
                        {
                            Ok(mut session) => {
                                info!(
                                    peer = %peer_addr,
                                    session_id = %session.session_id,
                                    cipher = %session.cipher.wire_name(),
                                    "riboCipher→BTSP TCP handshake succeeded"
                                );
                                let mut caller = caller_context_from_addr(peer_addr);
                                caller.btsp_family_verified = true;
                                Self::handle_jsonrpc_btsp_tcp(
                                    &mut stream,
                                    &mut session,
                                    registry,
                                    btsp_provider,
                                    gate,
                                    &mut caller,
                                )
                                .await
                            }
                            Err(e) => {
                                warn!(
                                    peer = %peer_addr,
                                    error = %e,
                                    "riboCipher→BTSP TCP handshake failed"
                                );
                                Ok(())
                            }
                        }
                    }
                    ribocipher::PROTO_PROBE => {
                        let response = serde_json::json!({
                            "status": "ok",
                            "primal": "bearDog",
                            "signal": "riboCipher-v1"
                        });
                        let msg = serde_json::to_string(&response)
                            .map_err(|e| BearDogError::system(format!("serialize: {e}")))?;
                        stream
                            .write_all(format!("{msg}\n").as_bytes())
                            .await
                            .map_err(|e| BearDogError::system(format!("write: {e}")))?;
                        Ok(())
                    }
                    _ => {
                        warn!(
                            peer = %peer_addr,
                            protocol_type = format!("0x{:02X}", protocol_type),
                            "riboCipher: unknown protocol type in clear signal"
                        );
                        Ok(())
                    }
                }
            }
            ribocipher::SIGNAL_MITO => {
                let mut tag = [0u8; 4];
                stream
                    .read_exact(&mut tag)
                    .await
                    .map_err(|e| BearDogError::system(format!("riboCipher mito read: {e}")))?;
                info!(
                    peer = %peer_addr,
                    "riboCipher: mito-obfuscated signal (Tier 2 — future expansion)"
                );
                warn!("riboCipher: mito-tier not yet implemented — closing connection");
                Ok(())
            }
            ribocipher::SIGNAL_NUCLEAR => {
                let mut payload = [0u8; 6];
                stream
                    .read_exact(&mut payload)
                    .await
                    .map_err(|e| BearDogError::system(format!("riboCipher nuclear read: {e}")))?;
                info!(
                    peer = %peer_addr,
                    "riboCipher: nuclear-sealed signal (Tier 3 — future expansion)"
                );
                warn!("riboCipher: nuclear-tier not yet implemented — closing connection");
                Ok(())
            }
            _ => unreachable!("is_signal_byte guards this branch"),
        }
    }

    /// Handle a plaintext NDJSON connection over any async reader/writer.
    pub(crate) async fn handle_plaintext_connection<R, W>(
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

    /// Handle JSON-RPC over BTSP encrypted frames on a TCP stream.
    async fn handle_jsonrpc_btsp_tcp(
        stream: &mut TcpStream,
        session: &mut BtspSession,
        registry: &Arc<HandlerRegistry>,
        btsp_provider: &Arc<BeardogBtspProvider>,
        gate: &MethodGate,
        caller: &mut CallerContext,
    ) -> Result<(), BearDogError> {
        loop {
            let frame = match btsp_handshake::read_frame(stream).await {
                Ok(f) => f,
                Err(e) => {
                    debug!(error = %e, "BTSP TCP frame read ended");
                    return Ok(());
                }
            };

            let plaintext = match session.decrypt_frame(&frame) {
                Ok(p) => p,
                Err(e) => {
                    warn!(error = %e, "BTSP TCP frame decrypt failed");
                    return Ok(());
                }
            };

            let request_str = match String::from_utf8(plaintext) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "BTSP TCP frame not valid UTF-8");
                    continue;
                }
            };

            let trimmed = request_str.trim();
            if trimmed.is_empty() {
                continue;
            }

            let request: Value = match serde_json::from_str(trimmed) {
                Ok(req) => req,
                Err(e) => {
                    let err_resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": {"code": -32700, "message": format!("Parse error: {e}")},
                        "id": null
                    });
                    if let Ok(s) = serde_json::to_string(&err_resp) {
                        let encrypted = session
                            .encrypt_frame(s.as_bytes())
                            .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
                        let _ = btsp_handshake::write_frame(stream, &encrypted).await;
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

            if let Some(result) = dispatch_auth_method(method, gate, caller, params.as_ref()) {
                let json_response = serde_json::json!({"jsonrpc":"2.0","result":result,"id":id});
                let resp_str = serde_json::to_string(&json_response)
                    .map_err(|e| BearDogError::system(format!("Serialize: {e}")))?;
                let encrypted = session
                    .encrypt_frame(resp_str.as_bytes())
                    .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
                btsp_handshake::write_frame(stream, &encrypted)
                    .await
                    .map_err(|e| BearDogError::system(format!("BTSP write: {e}")))?;
                continue;
            }

            if let Err(gate_err) = gate.check(method, caller) {
                let json_response = serde_json::json!({
                    "jsonrpc":"2.0",
                    "error":{"code":gate_err.code,"message":gate_err.message,"data":gate_err.data},
                    "id":id
                });
                let resp_str = serde_json::to_string(&json_response)
                    .map_err(|e| BearDogError::system(format!("Serialize: {e}")))?;
                let encrypted = session
                    .encrypt_frame(resp_str.as_bytes())
                    .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
                btsp_handshake::write_frame(stream, &encrypted)
                    .await
                    .map_err(|e| BearDogError::system(format!("BTSP write: {e}")))?;
                continue;
            }

            let response = registry.route(method, params.as_ref(), btsp_provider).await;

            let json_response = match response {
                Ok(result) => serde_json::json!({"jsonrpc":"2.0","result":result,"id":id}),
                Err(e) => serde_json::json!({
                    "jsonrpc":"2.0",
                    "error":{"code":-32601,"message":format!("Business error: {e}")},
                    "id":id
                }),
            };

            let resp_str = serde_json::to_string(&json_response)
                .map_err(|e| BearDogError::system(format!("Serialize response: {e}")))?;
            let encrypted = session
                .encrypt_frame(resp_str.as_bytes())
                .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
            btsp_handshake::write_frame(stream, &encrypted)
                .await
                .map_err(|e| BearDogError::system(format!("BTSP frame write: {e}")))?;
        }
    }
}

fn caller_context_from_addr(addr: &SocketAddr) -> CallerContext {
    if addr.ip().is_loopback() {
        CallerContext::loopback()
    } else {
        CallerContext::remote()
    }
}
