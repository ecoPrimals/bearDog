// SPDX-License-Identifier: AGPL-3.0-or-later

//! Client Mode — JSON-RPC `BearDog` client
//!
//! Connects to the `BearDog` IPC server (Unix socket or TCP) and dispatches
//! a single command as a JSON-RPC 2.0 request.  When no command is given,
//! prints the available method catalogue and exits.

use beardog_core::socket_config::SocketConfig;
use std::io::{BufRead, BufReader, Write};
use tracing::info;

use crate::ribocipher;

/// Wrapper for platform-agnostic stream that implements both Read and Write.
enum ClientStream {
    Tcp(std::net::TcpStream),
    #[cfg(unix)]
    Unix(std::os::unix::net::UnixStream),
}

impl std::io::Read for ClientStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(s) => s.read(buf),
            #[cfg(unix)]
            Self::Unix(s) => s.read(buf),
        }
    }
}

impl std::io::Write for ClientStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(s) => s.write(buf),
            #[cfg(unix)]
            Self::Unix(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Tcp(s) => s.flush(),
            #[cfg(unix)]
            Self::Unix(s) => s.flush(),
        }
    }
}

/// Run `BearDog` in client mode.
///
/// Discovers the server endpoint via `SocketConfig`, then either dispatches
/// `command` as a JSON-RPC method or displays the help banner.
///
/// # Errors
///
/// Returns an error if socket connection or I/O fails.
pub fn run(endpoint: Option<String>, command: Option<String>) -> anyhow::Result<()> {
    info!("🐻 BearDog Client v{}", env!("CARGO_PKG_VERSION"));

    let socket_path = if let Some(ep) = endpoint {
        ep
    } else {
        let config = SocketConfig::from_env().map_err(|e| anyhow::anyhow!("{e}"))?;
        info!("Auto-discovered endpoint: {}", config.description());
        config.socket_path_string()
    };

    info!("Endpoint: {socket_path}");

    if let Some(method) = command {
        dispatch_rpc(&socket_path, &method)?;
    } else {
        print_help_banner(&socket_path);
    }

    Ok(())
}

/// Open a connection to the endpoint (Unix socket or TCP).
fn open_connection(endpoint: &str) -> anyhow::Result<ClientStream> {
    if endpoint.contains(':') && endpoint.split(':').last().is_some_and(|p| p.parse::<u16>().is_ok()) {
        let stream = std::net::TcpStream::connect(endpoint).map_err(|e| {
            anyhow::anyhow!("Cannot connect to TCP {endpoint}: {e}. Is the BearDog server running?")
        })?;
        return Ok(ClientStream::Tcp(stream));
    }

    #[cfg(unix)]
    {
        let stream = std::os::unix::net::UnixStream::connect(endpoint).map_err(|e| {
            anyhow::anyhow!("Cannot connect to {endpoint}: {e}. Is the BearDog server running?")
        })?;
        return Ok(ClientStream::Unix(stream));
    }

    #[cfg(not(unix))]
    Err(anyhow::anyhow!(
        "Unix domain sockets are not available on this platform. Use TCP endpoint (host:port) instead."
    ))
}

/// Send a JSON-RPC 2.0 request and print the response.
fn dispatch_rpc(endpoint: &str, method: &str) -> anyhow::Result<()> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": {},
        "id": 1
    });

    let mut stream = open_connection(endpoint)?;

    stream.write_all(&ribocipher::clear_signal(ribocipher::PROTO_NDJSON_JSONRPC))?;

    let mut payload = serde_json::to_string(&request)?;
    payload.push('\n');
    stream.write_all(payload.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line)?;

    if response_line.trim().is_empty() {
        println!("(server closed connection without response)");
    } else {
        match serde_json::from_str::<serde_json::Value>(response_line.trim()) {
            Ok(value) => println!("{}", serde_json::to_string_pretty(&value)?),
            Err(_) => println!("{}", response_line.trim()),
        }
    }

    Ok(())
}

fn print_help_banner(endpoint: &str) {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║              🐻 BearDog Interactive Client 🐻                     ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Endpoint: {endpoint}");
    println!();
    println!("Usage:  beardog client --command <method>");
    println!();
    println!("  health.liveness         Liveness probe");
    println!("  health.readiness        Readiness probe");
    println!("  health.check            Full health check");
    println!("  capabilities.list       List registered capabilities");
    println!("  crypto.keygen           Generate a new key pair");
    println!("  crypto.sign             Sign data");
    println!("  crypto.verify           Verify a signature");
    println!("  crypto.hash             Compute a BLAKE3 hash");
    println!();
}
