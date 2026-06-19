// SPDX-License-Identifier: AGPL-3.0-or-later

use super::TcpIpcServer;
use crate::btsp_handshake::{self, BtspSession};
use crate::btsp_provider::BeardogBtspProvider;
use crate::method_gate::{CallerContext, MethodGate, dispatch_auth_method};
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_errors::BearDogError;
use serde_json::Value;
use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::{debug, warn};

impl TcpIpcServer {
    /// Handle JSON-RPC over BTSP encrypted frames on a TCP stream.
    pub(super) async fn handle_jsonrpc_btsp_tcp(
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

            // JH-1: extract bearer token from _bearer_token param
            if let Some(token) = params
                .as_ref()
                .and_then(|p| p.get("_bearer_token"))
                .and_then(|v| v.as_str())
            {
                caller.bearer_token = Some(token.to_owned());
            }

            // JH-0/JH-1: intercept gate-handled methods
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

            // JH-0/JH-1: pre-dispatch authorization gate (real token verification)
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
