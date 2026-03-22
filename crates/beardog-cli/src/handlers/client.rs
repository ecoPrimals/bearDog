// SPDX-License-Identifier: AGPL-3.0-only

//! Client mode handler - interactive REPL
//!
//! Interactive client for connecting to BearDog server.

use crate::ClientArgs;
use beardog_errors::BearDogError;
use serde_json::json;
use std::path::Path;
use std::sync::OnceLock;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{error, info};

/// Last-resort Unix socket directory when `BEARDOG_SOCKET` is unset (prefer `BIOMEOS_SOCKET_DIR` / XDG in production).
const DEFAULT_LOCAL_SOCKET_TMP_DIR: &str = "/tmp/";

/// Store the active socket path for command execution
static ACTIVE_SOCKET: OnceLock<String> = OnceLock::new();

/// Get the socket path using self-knowledge pattern.
/// Priority: ACTIVE_SOCKET > BEARDOG_SOCKET > PRIMAL_NAME-based > default
fn discover_socket_path() -> String {
    discover_socket_path_with(|key| std::env::var(key).ok())
}

fn discover_socket_path_with(get: impl Fn(&str) -> Option<String>) -> String {
    if let Some(path) = ACTIVE_SOCKET.get() {
        return path.clone();
    }

    if let Some(path) = get("BEARDOG_SOCKET") {
        return path;
    }

    let primal_name = get("PRIMAL_NAME")
        .or_else(|| get("BEARDOG_NAME"))
        .unwrap_or_else(|| "beardog".to_string());

    format!("{DEFAULT_LOCAL_SOCKET_TMP_DIR}{primal_name}.sock")
}

/// Handle client command - interactive REPL
pub async fn handle_client(args: ClientArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Client Mode");
    info!("   Connecting to: {}", args.socket);
    info!("");

    // Store the socket path for later use
    let _ = ACTIVE_SOCKET.set(args.socket.clone());

    // Connect to server
    let stream = UnixStream::connect(&args.socket)
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to connect to server: {e}"),
            category: Default::default(),
        })?;

    info!("✅ Connected to BearDog server");
    info!("");

    // If command provided, execute it and exit
    if let Some(command) = &args.command {
        return execute_command(&stream, command.as_str()).await;
    }

    // Interactive REPL
    info!("╔════════════════════════════════════════════════════════════════╗");
    info!("║                                                                ║");
    info!("║        🐻🐕 BearDog Interactive Client                         ║");
    info!("║                                                                ║");
    info!("╚════════════════════════════════════════════════════════════════╝");
    info!("");
    info!("Type 'help' for available commands, 'exit' to quit");
    info!("");

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin);

    loop {
        // Print prompt
        print!("beardog> ");
        if std::io::Write::flush(&mut std::io::stdout()).is_err() {
            // Flush failure is non-fatal, continue
        }

        // Read user input
        let mut input = String::new();
        match stdin_reader.read_line(&mut input).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "exit" || input == "quit" {
                    info!("👋 Goodbye!");
                    break;
                }

                if input == "help" {
                    print_help();
                    continue;
                }

                // Send command to server
                match send_command(&mut writer, &mut reader, input).await {
                    Ok(response) => {
                        let output = serde_json::to_string_pretty(&response).unwrap_or_else(|e| {
                            format!("{{\"error\": \"JSON serialization failed: {e}\"}}")
                        });
                        println!("{output}");
                    }
                    Err(e) => {
                        error!("❌ Error: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("❌ Failed to read input: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn execute_command(_stream: &UnixStream, command: &str) -> Result<(), BearDogError> {
    execute_command_on_socket(&discover_socket_path(), command).await
}

/// Run one JSON-RPC command on a concrete Unix socket path (used by tests; same behavior as `execute_command`).
async fn execute_command_on_socket(
    socket_path: impl AsRef<Path>,
    command: &str,
) -> Result<(), BearDogError> {
    let new_stream = UnixStream::connect(socket_path.as_ref())
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to connect for command: {e}"),
            category: Default::default(),
        })?;

    let (reader, mut writer) = new_stream.into_split();
    let mut reader = BufReader::new(reader);

    let response = send_command(&mut writer, &mut reader, command).await?;
    let output = serde_json::to_string_pretty(&response)
        .unwrap_or_else(|e| format!("{{\"error\": \"JSON serialization failed: {e}\"}}"));
    println!("{output}");

    Ok(())
}

/// Build a JSON-RPC 2.0 request from a whitespace-separated CLI-style command string.
fn build_jsonrpc_request(command: &str) -> Result<serde_json::Value, BearDogError> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(BearDogError::Business {
            message: "Empty command".to_string(),
            category: Default::default(),
        });
    }

    let method = parts[0];
    let params = if parts.len() > 1 {
        json!({ "args": parts[1..].to_vec() })
    } else {
        json!({})
    };

    Ok(json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    }))
}

async fn send_command(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
    command: &str,
) -> Result<serde_json::Value, BearDogError> {
    let request = build_jsonrpc_request(command)?;

    // Send request
    let request_str = serde_json::to_string(&request).map_err(|e| BearDogError::System {
        message: format!("Failed to serialize request: {e}"),
        category: Default::default(),
    })?;

    writer
        .write_all(request_str.as_bytes())
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write request: {e}"),
            category: Default::default(),
        })?;
    writer
        .write_all(b"\n")
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write newline: {e}"),
            category: Default::default(),
        })?;

    // Read response
    let mut response_line = String::new();
    reader
        .read_line(&mut response_line)
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to read response: {e}"),
            category: Default::default(),
        })?;

    let response: serde_json::Value =
        serde_json::from_str(&response_line).map_err(|e| BearDogError::System {
            message: format!("Failed to parse response: {e}"),
            category: Default::default(),
        })?;

    if let Some(error) = response.get("error") {
        return Err(BearDogError::Api {
            message: format!("Server error: {error}"),
            category: Default::default(),
            status_code: None,
            endpoint: None,
        });
    }

    Ok(response["result"].clone())
}

fn print_help() {
    println!();
    println!("Available Commands:");
    println!("  help                    - Show this help message");
    println!("  exit, quit              - Exit client");
    println!();
    println!("Crypto Operations:");
    println!("  crypto.sign_ed25519     - Sign with Ed25519");
    println!("  crypto.verify_ed25519   - Verify Ed25519 signature");
    println!("  crypto.blake3_hash      - Blake3 hash");
    println!("  crypto.hmac_sha256      - HMAC-SHA256");
    println!();
    println!("Discovery:");
    println!("  discovery.capabilities  - List available capabilities");
    println!("  discovery.health        - Server health check");
    println!();
    println!("Examples:");
    println!("  beardog> crypto.blake3_hash");
    println!("  beardog> discovery.capabilities");
    println!();
}

#[cfg(test)]
mod client_handler_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_build_jsonrpc_request_method_only() {
        let v = build_jsonrpc_request("crypto.blake3_hash").unwrap();
        assert_eq!(v["jsonrpc"], "2.0");
        assert_eq!(v["method"], "crypto.blake3_hash");
        assert_eq!(v["params"], json!({}));
        assert_eq!(v["id"], 1);
    }

    #[test]
    fn test_build_jsonrpc_request_with_args() {
        let v = build_jsonrpc_request("crypto.sign_ed25519 msg1 msg2").unwrap();
        assert_eq!(v["method"], "crypto.sign_ed25519");
        assert_eq!(v["params"], json!({ "args": ["msg1", "msg2"] }));
    }

    #[test]
    fn test_build_jsonrpc_request_empty_command() {
        assert!(build_jsonrpc_request("   ").is_err());
        assert!(build_jsonrpc_request("").is_err());
    }

    #[test]
    fn test_discover_socket_path_beardog_socket() {
        let mut map = HashMap::new();
        map.insert(
            "BEARDOG_SOCKET".to_string(),
            "/custom/beardog.sock".to_string(),
        );
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(discover_socket_path_with(get), "/custom/beardog.sock");
    }

    #[test]
    fn test_discover_socket_path_primal_name() {
        let mut map = HashMap::new();
        map.insert("PRIMAL_NAME".to_string(), "myprimal".to_string());
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(discover_socket_path_with(get), "/tmp/myprimal.sock");
    }

    #[test]
    fn test_discover_socket_path_beardog_name_fallback() {
        let mut map = HashMap::new();
        map.insert("BEARDOG_NAME".to_string(), "other".to_string());
        let get = |k: &str| map.get(k).cloned();
        assert_eq!(discover_socket_path_with(get), "/tmp/other.sock");
    }

    #[test]
    fn test_print_help_smoke() {
        print_help();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_send_command_success() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (client, mut server) = tokio::net::UnixStream::pair().unwrap();
        tokio::spawn(async move {
            let mut line = String::new();
            let mut reader = BufReader::new(&mut server);
            reader.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","result":{"ok":true},"id":1}"#;
            server.write_all(response.as_bytes()).await.unwrap();
            server.write_all(b"\n").await.unwrap();
        });

        let (read_half, mut write_half) = client.into_split();
        let mut reader = BufReader::new(read_half);
        let out = send_command(&mut write_half, &mut reader, "crypto.blake3_hash")
            .await
            .unwrap();
        assert_eq!(out["ok"], true);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_send_command_server_error_field() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (client, mut server) = tokio::net::UnixStream::pair().unwrap();
        tokio::spawn(async move {
            let mut line = String::new();
            let mut reader = BufReader::new(&mut server);
            reader.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","error":{"code":-1},"id":1}"#;
            server.write_all(response.as_bytes()).await.unwrap();
            server.write_all(b"\n").await.unwrap();
        });

        let (read_half, mut write_half) = client.into_split();
        let mut reader = BufReader::new(read_half);
        let err = send_command(&mut write_half, &mut reader, "crypto.blake3_hash")
            .await
            .expect_err("api error");
        assert!(err.to_string().contains("Server error") || err.to_string().contains("error"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_execute_command_on_socket_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let sock_path = dir.path().join("beardog-client-test.sock");
        let _ = std::fs::remove_file(&sock_path);
        let listener = tokio::net::UnixListener::bind(&sock_path).unwrap();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","result":{"echo":"pong"},"id":1}"#;
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.write_all(b"\n").await.unwrap();
        });

        super::execute_command_on_socket(&sock_path, "discovery.capabilities arg1")
            .await
            .unwrap();
    }
}
