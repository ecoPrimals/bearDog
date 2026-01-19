//! Client mode handler - interactive REPL
//!
//! Interactive client for connecting to BearDog server.

use crate::ClientArgs;
use beardog_errors::BearDogError;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{error, info};

/// Handle client command - interactive REPL
pub async fn handle_client(args: ClientArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Client Mode");
    info!("   Connecting to: {}", args.socket);
    info!("");

    // Connect to server
    let stream = UnixStream::connect(&args.socket)
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to connect to server: {}", e),
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
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

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
                        println!("{}", serde_json::to_string_pretty(&response).unwrap());
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
    // We need to create a new connection for the command
    // Unix streams don't support try_clone in the same way as TCP streams
    let socket_path = "/tmp/beardog.sock"; // Default path
    let new_stream = UnixStream::connect(socket_path)
        .await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to connect for command: {}", e),
            category: Default::default(),
        })?;
    
    let (reader, mut writer) = new_stream.into_split();
    let mut reader = BufReader::new(reader);

    let response = send_command(&mut writer, &mut reader, command).await?;
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    Ok(())
}

async fn send_command(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
    command: &str,
) -> Result<serde_json::Value, BearDogError> {
    // Parse command into JSON-RPC request
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

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    // Send request
    let request_str = serde_json::to_string(&request)
        .map_err(|e| BearDogError::System {
            message: format!("Failed to serialize request: {}", e),
            category: Default::default(),
        })?;
    
    writer.write_all(request_str.as_bytes()).await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write request: {}", e),
            category: Default::default(),
        })?;
    writer.write_all(b"\n").await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to write newline: {}", e),
            category: Default::default(),
        })?;

    // Read response
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await
        .map_err(|e| BearDogError::Network {
            message: format!("Failed to read response: {}", e),
            category: Default::default(),
        })?;

    let response: serde_json::Value = serde_json::from_str(&response_line)
        .map_err(|e| BearDogError::System {
            message: format!("Failed to parse response: {}", e),
            category: Default::default(),
        })?;

    if let Some(error) = response.get("error") {
        return Err(BearDogError::Api {
            message: format!("Server error: {}", error),
            category: Default::default(),
            status_code: None,
            endpoint: None,
        });
    }

    Ok(response["result"].clone())
}

fn print_help() {
    println!("");
    println!("Available Commands:");
    println!("  help                    - Show this help message");
    println!("  exit, quit              - Exit client");
    println!("");
    println!("Crypto Operations:");
    println!("  crypto.sign_ed25519     - Sign with Ed25519");
    println!("  crypto.verify_ed25519   - Verify Ed25519 signature");
    println!("  crypto.blake3_hash      - Blake3 hash");
    println!("  crypto.hmac_sha256      - HMAC-SHA256");
    println!("");
    println!("Discovery:");
    println!("  discovery.capabilities  - List available capabilities");
    println!("  discovery.health        - Server health check");
    println!("");
    println!("Examples:");
    println!("  beardog> crypto.blake3_hash");
    println!("  beardog> discovery.capabilities");
    println!("");
}

