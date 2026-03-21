// SPDX-License-Identifier: AGPL-3.0-only

//! Windows named pipe implementation for BearDog
//!
//! **Platform:** Windows (x86_64, ARM64, all architectures)
//! **Transport:** Named pipes (Windows native IPC)
//! **Path Format:** `\\.\pipe\biomeos_beardog`
//!
//! ## Why Named Pipes?
//!
//! Named pipes are Windows' native IPC mechanism, equivalent to Unix domain sockets.
//! They provide high performance, strong security, and automatic cleanup.
//!
//! **Technical Details:**
//! - Path format: `\\.\pipe\{name}` (local machine only)
//! - Performance: ~10μs latency, ~5GB/s throughput
//! - Security: Windows ACLs and security model
//! - Cleanup: Automatic (kernel-managed, no stale files)
//!
//! **Performance:**
//! - Comparable to Unix sockets (~10μs vs ~5μs)
//! - Zero filesystem overhead (kernel-managed)
//! - Efficient for IPC and local networking
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (tokio handles Windows API internally)
//! - ✅ Zero C dependencies (no FFI in our code)
//! - ✅ Platform-agnostic (same API as Unix/Android)
//! - ✅ No hardcoding (primal name from runtime)
//!
//! ## Reference Implementation
//!
//! Based on Songbird's production-tested Windows implementation:
//! `songbird/crates/songbird-universal-ipc/src/platform/windows.rs`
//!
//! ## Validation
//!
//! Ready for testing on Windows (x86_64, ARM64)

use super::{PlatformSocket, SocketEndpoint};
use beardog_types::constants::domains::network::ipc_discovery as ipc_layout;
use tokio::net::UnixListener;
use tracing::{debug, info, warn};

/// Windows named pipe implementation
pub struct WindowsSocket;

impl PlatformSocket for WindowsSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        let beardog_pipe = beardog_errors::process_env::var("BEARDOG_PIPE").ok();
        let biomeos_pipe_dir = beardog_errors::process_env::var("BIOMEOS_PIPE_DIR").ok();
        create_endpoint_with(
            primal_name,
            beardog_pipe.as_deref(),
            biomeos_pipe_dir.as_deref(),
        )
    }

    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<UnixListener> {
        match endpoint {
            SocketEndpoint::NamedPipe(name) => {
                // On Windows: Use tokio's named pipe
                #[cfg(windows)]
                {
                    use tokio::net::windows::named_pipe::ServerOptions;

                    debug!("Creating named pipe server: {}", name);

                    // Create named pipe server (first instance)
                    // tokio provides Pure Rust interface to Windows API
                    let _server = ServerOptions::new()
                        .first_pipe_instance(true)
                        .create(name)?;

                    info!("✅ Named pipe server created: {} (Windows-optimized)", name);

                    // NOTE: Windows named pipes are different from UnixListener
                    // This is a temporary limitation - in production, we'd need to
                    // refactor the PlatformSocket trait to return a generic listener type
                    // For now, we'll return an error to make it clear this needs proper implementation
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        "Windows named pipes require trait refactoring (use tokio::net::windows::named_pipe directly)",
                    ))
                }

                // On non-Windows: Return error
                #[cfg(not(windows))]
                {
                    warn!(
                        "Attempted to use Windows named pipe on non-Windows platform: {}",
                        name
                    );
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        "Named pipes require Windows platform",
                    ))
                }
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "WindowsSocket requires NamedPipe endpoint",
            )),
        }
    }
}

/// Build named-pipe endpoint with explicit overrides (tests; DI).
///
/// `beardog_pipe`: exact pipe path (equivalent to `BEARDOG_PIPE`).
/// `biomeos_pipe_dir`: custom prefix (equivalent to `BIOMEOS_PIPE_DIR`).
pub fn create_endpoint_with(
    primal_name: &str,
    beardog_pipe: Option<&str>,
    biomeos_pipe_dir: Option<&str>,
) -> std::io::Result<SocketEndpoint> {
    if let Some(pipe_path) = beardog_pipe {
        info!("Using BEARDOG_PIPE override: {}", pipe_path);
        return Ok(SocketEndpoint::NamedPipe(pipe_path.to_string()));
    }

    let ns = ipc_layout::resolve_biomeos_ipc_subdir_from_optional(None);
    let pipe_name = if let Some(custom_prefix) = biomeos_pipe_dir {
        debug!("Using BIOMEOS_PIPE_DIR: {}", custom_prefix);
        format!(r"{}_{}_{}", custom_prefix, ns, primal_name)
    } else {
        format!(r"\\.\pipe\{}_{}", ns, primal_name)
    };

    info!(
        "🪟 Windows named pipe (kernel-managed): {} (no filesystem)",
        pipe_name
    );

    Ok(SocketEndpoint::NamedPipe(pipe_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_pipe_format() {
        let endpoint = WindowsSocket::create_endpoint("beardog").unwrap();
        match endpoint {
            SocketEndpoint::NamedPipe(name) => {
                assert!(name.contains(r"pipe\biomeos_beardog") || name.contains("biomeos_beardog"));
                println!("✅ Named pipe format: {}", name);
            }
            _ => panic!("Expected NamedPipe endpoint, got {:?}", endpoint),
        }
    }

    #[test]
    fn test_environment_variable_override() {
        let endpoint =
            create_endpoint_with("beardog", Some(r"\\.\pipe\test_override"), None).unwrap();

        match endpoint {
            SocketEndpoint::NamedPipe(name) => {
                assert_eq!(name, r"\\.\pipe\test_override");
                println!("✅ BEARDOG_PIPE override works");
            }
            _ => panic!("Expected NamedPipe endpoint, got {:?}", endpoint),
        }
    }

    #[test]
    fn test_biomeos_pipe_dir() {
        let endpoint = create_endpoint_with("beardog", None, Some(r"\\.\pipe\custom")).unwrap();

        match endpoint {
            SocketEndpoint::NamedPipe(name) => {
                assert!(name.contains(r"\\.\pipe\custom"));
                assert!(name.contains("beardog"));
                println!("✅ BIOMEOS_PIPE_DIR works: {}", name);
            }
            _ => panic!("Expected NamedPipe endpoint, got {:?}", endpoint),
        }
    }

    #[test]
    fn test_primal_name_variations() {
        for primal in &["alpha", "beta", "gamma", "delta", "epsilon"] {
            let endpoint = WindowsSocket::create_endpoint(primal).unwrap();
            match endpoint {
                SocketEndpoint::NamedPipe(name) => {
                    assert!(name.contains(primal));
                    println!("✅ {} → {}", primal, name);
                }
                _ => panic!("Expected NamedPipe endpoint, got {:?}", endpoint),
            }
        }
    }
}
