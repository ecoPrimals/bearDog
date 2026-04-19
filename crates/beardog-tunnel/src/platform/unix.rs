// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unix filesystem socket implementation for `BearDog`  
//!
//! **Platform:** Linux, BSD, Solaris (all Unix-like systems)
//! **Transport:** Filesystem Unix domain sockets
//! **Path Format:** `/run/user/$UID/biomeos/beardog.sock` (XDG-compliant)
//!
//! ## Modern Idiomatic Rust Evolution (Jan 31, 2026)
//!
//! **Updated**: Now implements universal `PlatformListener` trait for
//! cross-platform compatibility. Same code works on Unix, Windows, WASM!
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (memory-safe)
//! - ✅ Zero C dependencies (tokio handles syscalls)
//! - ✅ Platform-agnostic (universal trait)
//! - ✅ No hardcoding (XDG Base Directory + runtime discovery)

use super::{
    PlatformListener, PlatformListenerBackend, PlatformSocket, PlatformStream, SocketEndpoint,
};
use beardog_types::constants::domains::network::ipc_discovery as ipc_layout;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, info};

/// Unix filesystem socket implementation
pub struct UnixSocket;

/// Wrapper to make `UnixStream` implement `PlatformStream`
pub struct UnixPlatformStream(UnixStream);

impl PlatformStream for UnixPlatformStream {}

impl AsyncRead for UnixPlatformStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for UnixPlatformStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

/// Universal listener wrapper for Unix sockets
pub struct UnixPlatformListener {
    listener: UnixListener,
    path: String,
}

impl UnixPlatformListener {
    pub(super) fn new(listener: UnixListener, path: String) -> Self {
        Self { listener, path }
    }
}

impl PlatformListener for UnixPlatformListener {
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>> {
        let (stream, _addr) = self.listener.accept().await?;
        Ok(Box::new(UnixPlatformStream(stream)) as Box<dyn PlatformStream>)
    }

    fn local_addr(&self) -> std::io::Result<String> {
        Ok(self.path.clone())
    }
}

/// Injected paths for Unix listener setup (tests avoid mutating process env).
#[derive(Debug, Clone, Default)]
pub struct UnixListenHints {
    /// `BEARDOG_SOCKET` when set.
    pub beardog_socket: Option<String>,
    /// `XDG_RUNTIME_DIR` when set.
    pub xdg_runtime_dir: Option<String>,
    /// `BIOMEOS_IPC_NAMESPACE` override (directory under runtime / tmp).
    pub ipc_namespace: Option<String>,
}

impl UnixListenHints {
    /// Read `BEARDOG_SOCKET` and `XDG_RUNTIME_DIR`.
    pub fn from_env() -> Self {
        Self {
            beardog_socket: beardog_errors::process_env::var("BEARDOG_SOCKET").ok(),
            xdg_runtime_dir: beardog_errors::process_env::var("XDG_RUNTIME_DIR").ok(),
            ipc_namespace: beardog_errors::process_env::var(ipc_layout::ENV_BIOMEOS_IPC_NAMESPACE)
                .ok(),
        }
    }
}

impl UnixSocket {
    /// Create a filesystem socket endpoint using explicit path hints.
    ///
    /// # Errors
    ///
    /// Returns an error if socket directories cannot be created or the endpoint path is invalid.
    pub fn create_endpoint_with(
        primal_name: &str,
        hints: &UnixListenHints,
    ) -> std::io::Result<SocketEndpoint> {
        Self::create_endpoint_inner(primal_name, hints)
    }

    fn create_endpoint_inner(
        primal_name: &str,
        hints: &UnixListenHints,
    ) -> std::io::Result<SocketEndpoint> {
        // NOTE: This function contains blocking filesystem operations (std::fs::create_dir_all)
        // during directory creation. This is acceptable as:
        // 1. It runs only during initialization (not in hot path)
        // 2. Directory creation is infrequent (usually already exists)
        // 3. Making the trait async would require larger refactoring
        //
        // Phase 3 plan: Consider making PlatformSocket trait async for full non-blocking operation,
        // allowing create_endpoint to use async filesystem APIs.

        if let Some(ref custom_socket) = hints.beardog_socket {
            info!("📡 Using BEARDOG_SOCKET override: {}", custom_socket);
            return Ok(SocketEndpoint::Filesystem(custom_socket.into()));
        }

        // Priority 2: XDG Base Directory (standard Linux/Unix)
        let socket_path = if let Some(ref runtime_dir) = hints.xdg_runtime_dir {
            // XDG compliant: /run/user/$UID/<namespace>/<primal>.sock
            let biomeos_dir = std::path::PathBuf::from(runtime_dir).join(
                ipc_layout::resolve_biomeos_ipc_subdir_from_optional(
                    hints.ipc_namespace.as_deref(),
                ),
            );

            // Ensure directory exists (BLOCKING - acceptable for initialization)
            if !biomeos_dir.exists() {
                std::fs::create_dir_all(&biomeos_dir)?;
            }

            biomeos_dir.join(format!("{primal_name}.sock"))
        } else {
            // Priority 3: temp fallback (compatibility)
            let tmp_dir = ipc_layout::biomeos_tmp_socket_root().join(
                ipc_layout::resolve_biomeos_ipc_subdir_from_optional(
                    hints.ipc_namespace.as_deref(),
                ),
            );

            // Ensure directory exists (BLOCKING - acceptable for initialization)
            if !tmp_dir.exists() {
                std::fs::create_dir_all(&tmp_dir)?;
            }

            tmp_dir.join(format!("{primal_name}.sock"))
        };

        debug!("Creating Unix filesystem socket: {}", socket_path.display());
        info!(
            "🐧 Unix filesystem socket (XDG-compliant): {} (automatic cleanup)",
            socket_path.display()
        );

        Ok(SocketEndpoint::Filesystem(socket_path))
    }
}

impl PlatformSocket for UnixSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        Self::create_endpoint_inner(primal_name, &UnixListenHints::from_env())
    }

    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<Box<PlatformListenerBackend>> {
        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                // Clean up stale socket file if it exists
                if path.exists() {
                    debug!("Removing stale socket: {}", path.display());
                    std::fs::remove_file(path)?;
                }

                debug!("Binding Unix socket: {}", path.display());

                let listener = UnixListener::bind(path)?;
                let path_str = path.display().to_string();

                info!(
                    "✅ Unix socket bound: {} (filesystem-based)",
                    path.display()
                );

                Ok(Box::new(PlatformListenerBackend::Unix(
                    UnixPlatformListener::new(listener, path_str),
                )))
            }
            SocketEndpoint::Abstract(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "UnixSocket requires Filesystem endpoint",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xdg_socket_path() {
        let temp_dir = std::env::temp_dir();
        let xdg_runtime = temp_dir.join(format!("xdg_test_{}", std::process::id()));
        std::fs::create_dir_all(&xdg_runtime).ok();

        let hints = UnixListenHints {
            beardog_socket: None,
            xdg_runtime_dir: Some(xdg_runtime.to_string_lossy().into_owned()),
            ipc_namespace: None,
        };
        let endpoint = UnixSocket::create_endpoint_with("beardog", &hints)
            .expect("create_endpoint_with valid hints");

        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                let path_str = path.to_str().expect("socket path is valid UTF-8");
                assert!(path_str.contains("biomeos"));
                assert!(path_str.ends_with("beardog.sock"));
                println!("✅ XDG-compliant path: {}", path.display());
            }
            SocketEndpoint::Abstract(_) => panic!("Expected Filesystem endpoint"),
        }

        std::fs::remove_dir_all(&xdg_runtime).ok();
    }

    #[test]
    fn test_environment_override() {
        let hints = UnixListenHints {
            beardog_socket: Some("/custom/path/beardog.sock".to_string()),
            xdg_runtime_dir: None,
            ipc_namespace: None,
        };
        let endpoint = UnixSocket::create_endpoint_with("beardog", &hints)
            .expect("create_endpoint_with beardog_socket override");

        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                assert_eq!(
                    path.to_str().expect("socket path is valid UTF-8"),
                    "/custom/path/beardog.sock"
                );
                println!("✅ Environment override works");
            }
            SocketEndpoint::Abstract(_) => panic!("Expected Filesystem endpoint"),
        }
    }

    #[test]
    fn test_primal_name_variations() {
        for primal in &["alpha", "beta", "gamma", "delta", "epsilon"] {
            let endpoint = UnixSocket::create_endpoint_with(primal, &UnixListenHints::default())
                .expect("create_endpoint_with default hints");
            match endpoint {
                SocketEndpoint::Filesystem(path) => {
                    assert!(
                        path.to_str()
                            .expect("socket path is UTF-8")
                            .contains(primal)
                    );
                    println!("✅ {} → {}", primal, path.display());
                }
                SocketEndpoint::Abstract(_) => panic!("Expected Filesystem endpoint"),
            }
        }
    }

    #[tokio::test]
    async fn test_universal_listener_trait() {
        // Create unique socket for test
        let test_socket = format!("/tmp/test_beardog_{}.sock", std::process::id());
        let endpoint = SocketEndpoint::Filesystem(test_socket.clone().into());

        // Bind using universal trait
        let listener = UnixSocket::bind(&endpoint).expect("bind test unix socket");

        // Verify local_addr works
        let addr = listener.local_addr().expect("listener local_addr");
        assert_eq!(addr, test_socket);

        println!("✅ Universal PlatformListener trait working!");

        // Cleanup
        let _ = std::fs::remove_file(&test_socket);
    }
}
