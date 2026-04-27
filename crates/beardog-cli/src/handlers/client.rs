// SPDX-License-Identifier: AGPL-3.0-or-later

//! Client mode handler - interactive REPL
//!
//! Interactive client for connecting to BearDog server.

use crate::ClientArgs;
use beardog_errors::{
    ApiErrorCategory, BearDogError, BusinessErrorCategory, NetworkErrorCategory,
    SystemErrorCategory,
};
use serde_json::json;
use std::path::Path;
use std::sync::OnceLock;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{error, info};

fn default_local_socket_parent_dir() -> std::path::PathBuf {
    beardog_errors::process_env::var("BEARDOG_LOCAL_SOCKET_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from)
}

/// Store the active socket path for command execution
static ACTIVE_SOCKET: OnceLock<String> = OnceLock::new();

/// Get the socket path using self-knowledge pattern.
/// Priority: `ACTIVE_SOCKET` > `BEARDOG_SOCKET` > PRIMAL_NAME-based > default
fn discover_socket_path() -> String {
    if let Some(path) = ACTIVE_SOCKET.get() {
        return path.clone();
    }
    discover_socket_path_with(|key| std::env::var(key).ok())
}

fn discover_socket_path_with(get: impl Fn(&str) -> Option<String>) -> String {
    if let Some(path) = get("BEARDOG_SOCKET") {
        return path;
    }

    let primal_name = get("PRIMAL_NAME")
        .or_else(|| get("BEARDOG_NAME"))
        .unwrap_or_else(|| "beardog".to_string());

    default_local_socket_parent_dir()
        .join(format!("{primal_name}.sock"))
        .display()
        .to_string()
}

/// Handle client command - interactive REPL
///
/// # Errors
///
/// Returns an error if the Unix socket cannot be opened, or sending/receiving IPC commands fails.
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
            category: NetworkErrorCategory::default(),
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
            category: NetworkErrorCategory::default(),
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
            category: BusinessErrorCategory::default(),
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
        category: SystemErrorCategory::default(),
    })?;

    writer
        .write_all(request_str.as_bytes())
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write request: {e}"),
            category: NetworkErrorCategory::default(),
        })?;
    writer
        .write_all(b"\n")
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write newline: {e}"),
            category: NetworkErrorCategory::default(),
        })?;

    // Read response
    let mut response_line = String::new();
    reader
        .read_line(&mut response_line)
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to read response: {e}"),
            category: NetworkErrorCategory::default(),
        })?;

    let response: serde_json::Value =
        serde_json::from_str(&response_line).map_err(|e| BearDogError::System {
            message: format!("Failed to parse response: {e}"),
            category: SystemErrorCategory::default(),
        })?;

    if let Some(error) = response.get("error") {
        return Err(BearDogError::Api {
            message: format!("Server error: {error}"),
            category: ApiErrorCategory::default(),
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
    use super::{build_jsonrpc_request, discover_socket_path_with, print_help, send_command};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Mutex;

    static CLIENT_ENV_LOCK: Mutex<()> = Mutex::new(());
    static HANDLE_CLIENT_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_build_jsonrpc_request_method_only() {
        let v =
            build_jsonrpc_request("crypto.blake3_hash").expect("build_jsonrpc_request method only");
        assert_eq!(v["jsonrpc"], "2.0");
        assert_eq!(v["method"], "crypto.blake3_hash");
        assert_eq!(v["params"], json!({}));
        assert_eq!(v["id"], 1);
    }

    #[test]
    fn test_build_jsonrpc_request_with_args() {
        let v = build_jsonrpc_request("crypto.sign_ed25519 msg1 msg2")
            .expect("build_jsonrpc_request with args");
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
    fn test_discover_socket_path_defaults_primal_name_to_beardog() {
        let _guard = CLIENT_ENV_LOCK
            .lock()
            .expect("client env test lock poisoned");
        beardog_errors::process_env::remove_var("BEARDOG_LOCAL_SOCKET_DIR");
        let map: HashMap<String, String> = HashMap::new();
        let get = |k: &str| map.get(k).cloned();
        let expected = std::env::temp_dir()
            .join("beardog.sock")
            .display()
            .to_string();
        assert_eq!(discover_socket_path_with(get), expected);
    }

    #[test]
    fn test_discover_socket_path_primal_name() {
        let _guard = CLIENT_ENV_LOCK
            .lock()
            .expect("client env test lock poisoned");
        beardog_errors::process_env::remove_var("BEARDOG_LOCAL_SOCKET_DIR");
        let mut map = HashMap::new();
        map.insert("PRIMAL_NAME".to_string(), "myprimal".to_string());
        let get = |k: &str| map.get(k).cloned();
        let expected = std::env::temp_dir()
            .join("myprimal.sock")
            .display()
            .to_string();
        assert_eq!(discover_socket_path_with(get), expected);
    }

    #[test]
    fn test_discover_socket_path_beardog_name_fallback() {
        let _guard = CLIENT_ENV_LOCK
            .lock()
            .expect("client env test lock poisoned");
        beardog_errors::process_env::remove_var("BEARDOG_LOCAL_SOCKET_DIR");
        let mut map = HashMap::new();
        map.insert("BEARDOG_NAME".to_string(), "other".to_string());
        let get = |k: &str| map.get(k).cloned();
        let expected = std::env::temp_dir()
            .join("other.sock")
            .display()
            .to_string();
        assert_eq!(discover_socket_path_with(get), expected);
    }

    #[test]
    fn test_discover_socket_path_uses_beardog_local_socket_dir_env() {
        let _guard = CLIENT_ENV_LOCK
            .lock()
            .expect("client env test lock poisoned");
        let dir = tempfile::tempdir().expect("tempdir for BEARDOG_LOCAL_SOCKET_DIR test");
        beardog_errors::process_env::set_var("BEARDOG_LOCAL_SOCKET_DIR", dir.path().as_os_str());
        let mut map = HashMap::new();
        map.insert("PRIMAL_NAME".to_string(), "sockname".to_string());
        let get = |k: &str| map.get(k).cloned();
        let expected = dir.path().join("sockname.sock").display().to_string();
        assert_eq!(discover_socket_path_with(get), expected);
        beardog_errors::process_env::remove_var("BEARDOG_LOCAL_SOCKET_DIR");
    }

    #[test]
    fn test_print_help_smoke() {
        print_help();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_send_command_success() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (client, mut server) =
            tokio::net::UnixStream::pair().expect("UnixStream::pair for test");
        tokio::spawn(async move {
            let mut line = String::new();
            let mut reader = BufReader::new(&mut server);
            reader
                .read_line(&mut line)
                .await
                .expect("server read request line");
            let response = r#"{"jsonrpc":"2.0","result":{"ok":true},"id":1}"#;
            server
                .write_all(response.as_bytes())
                .await
                .expect("server write response");
            server.write_all(b"\n").await.expect("server write newline");
        });

        let (read_half, mut write_half) = client.into_split();
        let mut reader = BufReader::new(read_half);
        let out = send_command(&mut write_half, &mut reader, "crypto.blake3_hash")
            .await
            .expect("send_command success path");
        assert_eq!(out["ok"], true);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_send_command_server_error_field() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (client, mut server) =
            tokio::net::UnixStream::pair().expect("UnixStream::pair for error test");
        tokio::spawn(async move {
            let mut line = String::new();
            let mut reader = BufReader::new(&mut server);
            reader
                .read_line(&mut line)
                .await
                .expect("server read request line");
            let response = r#"{"jsonrpc":"2.0","error":{"code":-1},"id":1}"#;
            server
                .write_all(response.as_bytes())
                .await
                .expect("server write error response");
            server.write_all(b"\n").await.expect("server write newline");
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
    async fn test_send_command_invalid_json_response() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (client, mut server) =
            tokio::net::UnixStream::pair().expect("UnixStream::pair for invalid JSON test");
        tokio::spawn(async move {
            let mut line = String::new();
            let mut reader = BufReader::new(&mut server);
            reader
                .read_line(&mut line)
                .await
                .expect("server read request line");
            server
                .write_all(b"not-json\n")
                .await
                .expect("server write bad response");
        });

        let (read_half, mut write_half) = client.into_split();
        let mut reader = BufReader::new(read_half);
        let err = send_command(&mut write_half, &mut reader, "crypto.blake3_hash")
            .await
            .expect_err("parse response");
        assert!(
            err.to_string().contains("parse") || err.to_string().contains("Failed"),
            "{err}"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_execute_command_on_socket_roundtrip() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let dir = tempfile::tempdir().expect("tempdir for unix socket test");
        let sock_path = dir.path().join("beardog-client-test.sock");
        let _ = std::fs::remove_file(&sock_path);
        let listener = tokio::net::UnixListener::bind(&sock_path).expect("bind test unix listener");

        tokio::spawn(async move {
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("listener accept in test server");
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .await
                .expect("read jsonrpc line");
            let response = r#"{"jsonrpc":"2.0","result":{"echo":"pong"},"id":1}"#;
            stream
                .write_all(response.as_bytes())
                .await
                .expect("write response");
            stream.write_all(b"\n").await.expect("write newline");
        });

        super::execute_command_on_socket(&sock_path, "discovery.capabilities arg1")
            .await
            .expect("execute_command_on_socket roundtrip");
    }

    #[cfg(unix)]
    #[tokio::test]
    #[allow(
        clippy::await_holding_lock,
        reason = "serializes handle_client tests against shared env"
    )]
    async fn test_handle_client_runs_one_shot_command() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let _guard = HANDLE_CLIENT_LOCK
            .lock()
            .expect("handle_client test lock poisoned");

        let dir = tempfile::tempdir().expect("tempdir for handle_client test");
        let sock_path = dir.path().join("handle_client.sock");
        let _ = std::fs::remove_file(&sock_path);
        let listener = tokio::net::UnixListener::bind(&sock_path)
            .expect("bind unix listener for handle_client test");

        tokio::spawn(async move {
            for _ in 0..2u32 {
                let (mut stream, _) = listener.accept().await.expect("accept client connection");
                tokio::spawn(async move {
                    let mut reader = BufReader::new(&mut stream);
                    let mut line = String::new();
                    let _ = reader.read_line(&mut line).await;
                    let response = r#"{"jsonrpc":"2.0","result":{"handled":true},"id":1}"#;
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.write_all(b"\n").await;
                });
            }
        });

        tokio::task::yield_now().await;

        let args = crate::ClientArgs {
            socket: sock_path.display().to_string(),
            command: Some("crypto.blake3_hash".to_string()),
        };

        super::handle_client(args)
            .await
            .expect("handle_client one-shot command mode");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_execute_command_on_socket_fails_when_socket_missing() {
        let err = super::execute_command_on_socket(
            "/tmp/beardog-cli-nonexistent-socket-9f3a.sock",
            "crypto.blake3_hash",
        )
        .await
        .expect_err("connect to missing socket must fail");
        assert!(
            err.to_string().contains("connect") || err.to_string().contains("Failed"),
            "{err}"
        );
    }

    #[test]
    fn test_build_jsonrpc_request_whitespace_only_parts() {
        let v = build_jsonrpc_request("  method_name  ")
            .expect("single token with surrounding whitespace");
        assert_eq!(v["method"], "method_name");
        assert_eq!(v["params"], json!({}));
    }
}
