// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys::resolve_primal_name;
use beardog_errors::BearDogError;
use tracing::info;

/// Lightweight plaintext health socket for monitoring probes.
///
/// Accepts connections, optionally consumes riboCipher prefix, reads one
/// JSON-RPC request, responds with health status. No BTSP, no auth, no
/// method gate — just a liveness signal for cellMembrane and orchestration.
pub(super) async fn run_health_socket(path: &str) -> Result<(), BearDogError> {
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;

    let _ = std::fs::remove_file(path);
    let listener = UnixListener::bind(path)
        .map_err(|e| BearDogError::system(format!("health socket bind failed: {e}")))?;
    info!(path = %path, "health socket listening");

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut first = [0u8; 1];
            if stream.read_exact(&mut first).await.is_err() {
                return;
            }

            // Tolerate riboCipher prefix: consume second byte and read JSON
            let is_signal = matches!(first[0], 0xEC..=0xEE);
            if is_signal {
                let mut _proto = [0u8; 1];
                let _ = stream.read_exact(&mut _proto).await;
            }

            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            if is_signal {
                if reader.read_line(&mut line).await.is_err() || line.is_empty() {
                    return;
                }
            } else {
                // First byte was part of the JSON payload
                line.push(first[0] as char);
                let mut rest = String::new();
                if reader.read_line(&mut rest).await.is_err() {
                    return;
                }
                line.push_str(&rest);
                if line.trim().is_empty() {
                    return;
                }
            }

            // Extract request id for proper JSON-RPC correlation
            let req_id = serde_json::from_str::<serde_json::Value>(line.trim())
                .ok()
                .and_then(|v| v.get("id").cloned())
                .unwrap_or(serde_json::Value::Null);

            let primal = resolve_primal_name();
            let version = env!("CARGO_PKG_VERSION");
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "result": {
                    "status": "alive",
                    "primal": primal,
                    "version": version,
                },
                "id": req_id
            });

            let mut resp_bytes = serde_json::to_vec(&response).unwrap_or_default();
            resp_bytes.push(b'\n');
            let _ = stream.write_all(&resp_bytes).await;
        });
    }
}
