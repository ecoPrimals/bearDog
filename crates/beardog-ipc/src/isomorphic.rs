// SPDX-License-Identifier: AGPL-3.0-or-later

//! Isomorphic IPC Client Discovery
//!
//! This module provides automatic discovery of `BearDog` IPC endpoints,
//! supporting both Unix sockets and TCP fallback transparently.
//!
//! ## Isomorphic Pattern (Client-Side)
//!
//! The client automatically discovers and connects to whichever transport
//! the server is using - no configuration needed!
//!
//! ### Discovery Priority
//!
//! 1. **Unix Socket** (optimal - tries first)
//! 2. **TCP Discovery File** (fallback - automatic)
//!
//! ## Deep Debt Principles
//!
//! - ✅ **Runtime Discovery**: Detects available transport
//! - ✅ **Zero Configuration**: No environment variables required
//! - ✅ **Platform Agnostic**: Works on all platforms
//! - ✅ **Pure Rust**: Zero external dependencies

use anyhow::{Context, Result};
use beardog_config::env_keys;
use beardog_core::self_knowledge::{
    IdentityInputs, PrimalIdentity, discovered_simple_capabilities,
};
use beardog_core::socket_config::ipc_capability_domain_stems_resolved;
use beardog_types::constants::domains::network::ipc_discovery as ipc_layout;
use beardog_types::constants::domains::network::ribocipher;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::Poll;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::{TcpStream, UnixStream};
use tracing::{debug, info};

/// IPC endpoint types (Unix socket or TCP)
///
/// This enum enables polymorphic connections - same client code
/// works with either transport!
#[derive(Debug, Clone)]
pub enum IpcEndpoint {
    /// Unix domain socket (optimal on Linux/macOS)
    UnixSocket(PathBuf),

    /// TCP on localhost (fallback for Android/constraints)
    TcpLocal(SocketAddr),
}

impl IpcEndpoint {
    /// Get display string for logging
    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::UnixSocket(path) => format!("unix:{}", path.display()),
            Self::TcpLocal(addr) => format!("tcp:{addr}"),
        }
    }

    /// Check if this is the optimal transport (Unix socket)
    #[must_use]
    pub const fn is_optimal(&self) -> bool {
        matches!(self, Self::UnixSocket(_))
    }
}

/// Polymorphic stream trait for IPC
///
/// This trait allows both `UnixStream` and `TcpStream` to be used
/// interchangeably - TRUE universal abstraction!
pub trait AsyncStream: AsyncRead + AsyncWrite + Send + Unpin {}

// Implement for both Unix and TCP streams
impl AsyncStream for UnixStream {}
impl AsyncStream for TcpStream {}

/// Concrete enum dispatch for IPC streams (replaces `Box<dyn AsyncStream>`).
///
/// Both variants are `Unpin + Send`, so the enum delegates directly.
#[derive(Debug)]
pub enum IpcStream {
    /// Unix domain socket transport.
    Unix(UnixStream),
    /// TCP transport (localhost fallback).
    Tcp(TcpStream),
}

impl AsyncRead for IpcStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Unix(s) => Pin::new(s).poll_read(cx, buf),
            Self::Tcp(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for IpcStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            Self::Unix(s) => Pin::new(s).poll_write(cx, buf),
            Self::Tcp(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Unix(s) => Pin::new(s).poll_flush(cx),
            Self::Tcp(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Unix(s) => Pin::new(s).poll_shutdown(cx),
            Self::Tcp(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

impl AsyncStream for IpcStream {}

/// Discover `BearDog` IPC endpoint (Unix or TCP)
///
/// **Isomorphic Discovery** (zero configuration):
/// 1. Tries Unix socket paths (optimal)
/// 2. Falls back to TCP discovery file (automatic)
///
/// This enables the same client code to work on Linux (Unix) and
/// Android (TCP fallback) without any configuration!
///
/// ## Example
///
/// ```no_run
/// use beardog_ipc::discover_beardog_endpoint;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let endpoint = discover_beardog_endpoint().await?;
///     println!("Found BearDog at: {}", endpoint.display());
///     Ok(())
/// }
/// ```
///
/// ## Error Handling
///
/// Returns error only if BOTH Unix and TCP discovery fail.
/// This means `BearDog` is not running or unreachable.
///
/// # Errors
///
/// Same as above: returns an error when no reachable IPC endpoint is found.
pub async fn discover_beardog_endpoint() -> Result<IpcEndpoint> {
    // 1. Try Unix socket paths first (optimal)
    debug!("🔍 Discovering BearDog IPC endpoint...");
    debug!("   Step 1: Trying Unix socket paths (optimal)");

    let socket_paths = get_unix_socket_paths();
    for path in socket_paths {
        if path.exists() {
            info!("✅ Found Unix socket: {}", path.display());
            return Ok(IpcEndpoint::UnixSocket(path));
        }
    }

    debug!("   Unix sockets not found, trying TCP discovery...");

    // 2. Try TCP discovery file (fallback)
    debug!("   Step 2: Trying TCP discovery file (fallback)");

    if let Ok(endpoint) = discover_tcp_endpoint().await {
        info!(
            "✅ Found TCP endpoint via discovery file: {}",
            endpoint.display()
        );
        return Ok(endpoint);
    }

    Err(anyhow::anyhow!(
        "Could not discover BearDog IPC endpoint (tried Unix sockets and TCP discovery)"
    ))
}

/// Hints for Unix socket path discovery (injectable; no I/O in [`Default`]).
#[derive(Debug, Clone, Default)]
pub struct UnixSocketPathHints {
    /// `BEARDOG_SOCKET` override.
    pub beardog_socket: Option<String>,
    /// `XDG_RUNTIME_DIR` for default socket layout.
    pub xdg_runtime_dir: Option<String>,
    /// `BIOMEOS_IPC_NAMESPACE` override (directory under runtime dir).
    pub ipc_namespace: Option<String>,
    /// Socket filename stem (e.g. `PRIMAL_NAME`); default matches [`PrimalIdentity`] resolution.
    pub primal_socket_stem: Option<String>,
}

/// Get Unix socket path candidates from explicit hints (tests avoid env mutation).
#[must_use]
pub fn get_unix_socket_paths_with(hints: &UnixSocketPathHints) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(ref path) = hints.beardog_socket {
        paths.push(PathBuf::from(path));
    }

    let stem = hints
        .primal_socket_stem
        .clone()
        .or_else(|| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME).ok())
        .or_else(|| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME_PREFIXED).ok())
        .unwrap_or_else(|| PrimalIdentity::from_inputs(&IdentityInputs::from_env()).name);

    let subdir =
        ipc_layout::resolve_biomeos_ipc_subdir_from_optional(hints.ipc_namespace.as_deref());

    if let Some(ref runtime_dir) = hints.xdg_runtime_dir {
        let mut p = PathBuf::from(runtime_dir);
        p.push(&subdir);
        p.push(format!("{stem}.sock"));
        paths.push(p);

        // wateringHole IPC v3.1: try capability-domain symlinks (e.g. crypto.sock → beardog.sock)
        for cap_stem in ipc_capability_domain_stems_resolved(&discovered_simple_capabilities()) {
            let mut q = PathBuf::from(runtime_dir);
            q.push(&subdir);
            q.push(format!("{cap_stem}.sock"));
            paths.push(q);
        }
    }

    paths.push(std::env::temp_dir().join(format!("{stem}.sock")));

    paths
}

/// Get Unix socket path candidates (XDG-compliant)
///
/// **Discovery Priority**:
/// 1. `BEARDOG_SOCKET` env var (operator override)
/// 2. `$XDG_RUNTIME_DIR/biomeos/beardog.sock` (XDG standard)
/// 3. `{std::env::temp_dir()}/beardog.sock` (fallback)
///
/// **Zero Hardcoding**: Uses XDG Base Directory specification
fn get_unix_socket_paths() -> Vec<PathBuf> {
    get_unix_socket_paths_with(&UnixSocketPathHints {
        beardog_socket: beardog_errors::process_env::var(env_keys::ENV_SOCKET).ok(),
        xdg_runtime_dir: beardog_errors::process_env::var(env_keys::ENV_XDG_RUNTIME_DIR).ok(),
        ipc_namespace: beardog_errors::process_env::var(ipc_layout::ENV_BIOMEOS_IPC_NAMESPACE).ok(),
        primal_socket_stem: None,
    })
}

/// Discover TCP endpoint from discovery file
///
/// **Discovery File Format**: `tcp:<host>:<port>` (parsed as [`SocketAddr`]; typically loopback from the daemon’s written discovery file)
///
/// **Search Paths** (XDG-compliant):
/// 1. `$XDG_RUNTIME_DIR/beardog-ipc-port`
/// 2. `$HOME/.local/share/beardog-ipc-port`
/// 3. `{std::env::temp_dir()}/beardog-ipc-port`
///
/// **Zero Hardcoding**: Uses XDG Base Directory specification
async fn discover_tcp_endpoint() -> Result<IpcEndpoint> {
    let discovery_files = get_tcp_discovery_file_candidates();

    for file in discovery_files {
        if let Ok(contents) = tokio::fs::read_to_string(&file).await {
            // Parse format: tcp:<SocketAddr>
            if let Some(addr_str) = contents.trim().strip_prefix("tcp:")
                && let Ok(addr) = addr_str.parse::<SocketAddr>()
            {
                debug!("📁 Found TCP discovery file: {} -> {}", file, addr);
                return Ok(IpcEndpoint::TcpLocal(addr));
            }
        }
    }

    Err(anyhow::anyhow!("No TCP discovery file found"))
}

/// Hints for TCP discovery file search paths.
#[derive(Debug, Clone, Default)]
pub struct TcpDiscoveryPathHints {
    /// `XDG_RUNTIME_DIR`.
    pub xdg_runtime_dir: Option<String>,
    /// `HOME`.
    pub home: Option<String>,
}

/// TCP discovery file candidates from explicit hints.
#[must_use]
pub fn get_tcp_discovery_file_candidates_with(hints: &TcpDiscoveryPathHints) -> Vec<String> {
    let mut files = Vec::new();

    if let Some(ref runtime_dir) = hints.xdg_runtime_dir {
        files.push(format!(
            "{runtime_dir}/{}",
            ipc_layout::BEARDOG_TCP_DISCOVERY_FILENAME
        ));
    }

    if let Some(ref home) = hints.home {
        files.push(format!(
            "{home}/.local/share/{}",
            ipc_layout::BEARDOG_TCP_DISCOVERY_FILENAME
        ));
    }

    files.push(
        std::env::temp_dir()
            .join("beardog-ipc-port")
            .to_string_lossy()
            .into_owned(),
    );

    files
}

/// Get TCP discovery file path candidates (XDG-compliant)
fn get_tcp_discovery_file_candidates() -> Vec<String> {
    get_tcp_discovery_file_candidates_with(&TcpDiscoveryPathHints {
        xdg_runtime_dir: beardog_errors::process_env::var(env_keys::ENV_XDG_RUNTIME_DIR).ok(),
        home: beardog_errors::process_env::var(env_keys::ENV_HOME).ok(),
    })
}

/// Connect to `BearDog` IPC endpoint (polymorphic)
///
/// **Isomorphic Connection** (automatic adaptation):
/// - Unix socket → Uses `UnixStream`
/// - TCP → Uses `TcpStream`
/// - Returns [`IpcStream`] (enum dispatch, zero heap allocation)
///
/// ## Example
///
/// ```no_run
/// use beardog_ipc::connect_beardog;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut stream = connect_beardog().await?;
///     // Use stream - works with Unix OR TCP transparently!
///     Ok(())
/// }
/// ```
///
/// Connect to any primal via a structured [`TransportEndpoint`].
///
/// This is the ecosystem-standard outbound connection path: the caller
/// provides a [`TransportEndpoint`] (from `ipc.resolve`, `TRANSPORT_ENDPOINT`
/// env var, or capability discovery) and gets back a transport-agnostic
/// [`IpcStream`].
///
/// # Errors
///
/// Returns an error if the connection cannot be established, or if the
/// endpoint transport type is not supported for outbound connections
/// (e.g. `mesh_relay` without a relay client).
pub async fn connect_transport(
    endpoint: &beardog_types::btsp::TransportEndpoint,
) -> Result<IpcStream> {
    use beardog_types::btsp::TransportEndpoint;

    info!(endpoint = %endpoint, "connecting via TransportEndpoint");

    match endpoint {
        TransportEndpoint::Uds { path } => {
            let mut stream = UnixStream::connect(path)
                .await
                .context(format!("Failed to connect to UDS: {}", path.display()))?;
            stream
                .write_all(&ribocipher::clear_signal(ribocipher::PROTO_NDJSON_JSONRPC))
                .await
                .context("Failed to send riboCipher signal")?;
            info!(path = %path.display(), "connected via UDS");
            Ok(IpcStream::Unix(stream))
        }
        TransportEndpoint::Tcp { host, port } => {
            let addr = format!("{host}:{port}");
            let mut stream = TcpStream::connect(&addr)
                .await
                .context(format!("Failed to connect to TCP: {addr}"))?;
            stream
                .write_all(&ribocipher::clear_signal(ribocipher::PROTO_NDJSON_JSONRPC))
                .await
                .context("Failed to send riboCipher signal")?;
            info!(addr = %addr, "connected via TCP");
            Ok(IpcStream::Tcp(stream))
        }
        TransportEndpoint::MeshRelay {
            peer_id,
            capability,
        } => Err(anyhow::anyhow!(
            "mesh_relay transport not yet supported for outbound connections (peer={peer_id}, cap={capability})"
        )),
    }
}

/// ## Error Handling
///
/// Returns error if:
/// - Discovery fails (`BearDog` not running)
/// - Connection fails (network error)
///
/// # Errors
///
/// Propagates [`discover_beardog_endpoint`] failures, or I/O errors when opening the socket.
pub async fn connect_beardog() -> Result<IpcStream> {
    let endpoint = discover_beardog_endpoint().await?;
    let te = ipc_endpoint_to_transport(&endpoint);
    connect_transport(&te).await
}

/// Convert a discovered [`IpcEndpoint`] into a [`TransportEndpoint`].
#[must_use]
pub fn ipc_endpoint_to_transport(ep: &IpcEndpoint) -> beardog_types::btsp::TransportEndpoint {
    use beardog_types::btsp::TransportEndpoint;
    match ep {
        IpcEndpoint::UnixSocket(path) => TransportEndpoint::Uds { path: path.clone() },
        IpcEndpoint::TcpLocal(addr) => TransportEndpoint::Tcp {
            host: addr.ip().to_string(),
            port: addr.port(),
        },
    }
}

/// Open a raw transport connection without sending any protocol prefix.
///
/// Use this when the caller manages its own `riboCipher` signal or sends
/// a different protocol prefix (e.g. health probes, registry clients).
/// For the standard JSON-RPC path, prefer [`connect_transport`] which
/// includes the `riboCipher` signal automatically.
///
/// # Errors
///
/// Returns an error if the connection cannot be established, or if the
/// endpoint transport type is not supported for outbound connections.
pub async fn connect_raw(endpoint: &beardog_types::btsp::TransportEndpoint) -> Result<IpcStream> {
    use beardog_types::btsp::TransportEndpoint;

    debug!(endpoint = %endpoint, "raw transport connect (no protocol prefix)");

    match endpoint {
        TransportEndpoint::Uds { path } => {
            let stream = UnixStream::connect(path)
                .await
                .context(format!("Failed to connect to UDS: {}", path.display()))?;
            Ok(IpcStream::Unix(stream))
        }
        TransportEndpoint::Tcp { host, port } => {
            let addr = format!("{host}:{port}");
            let stream = TcpStream::connect(&addr)
                .await
                .context(format!("Failed to connect to TCP: {addr}"))?;
            Ok(IpcStream::Tcp(stream))
        }
        TransportEndpoint::MeshRelay {
            peer_id,
            capability,
        } => Err(anyhow::anyhow!(
            "mesh_relay transport not yet supported for outbound connections (peer={peer_id}, cap={capability})"
        )),
    }
}

/// Connect to a primal via a Unix socket path (convenience wrapper).
///
/// # Errors
///
/// Returns an error if the socket cannot be opened.
pub async fn connect_unix(path: impl AsRef<std::path::Path>) -> Result<IpcStream> {
    let te = beardog_types::btsp::TransportEndpoint::Uds {
        path: path.as_ref().to_path_buf(),
    };
    connect_transport(&te).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_display() {
        let unix = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/test.sock"));
        assert_eq!(unix.display(), "unix:/tmp/test.sock");
        assert!(unix.is_optimal());

        // Test-only: arbitrary loopback port for display string shape
        let tcp = IpcEndpoint::TcpLocal(
            "127.0.0.1:65000"
                .parse()
                .expect("127.0.0.1:65000 parses as SocketAddr"),
        );
        assert_eq!(tcp.display(), "tcp:127.0.0.1:65000");
        assert!(!tcp.is_optimal());
    }

    #[test]
    fn test_endpoint_clone_debug() {
        let ep = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/x.sock"));
        let cloned = ep.clone();
        assert_eq!(ep.display(), cloned.display());
        assert!(format!("{ep:?}").contains("UnixSocket"));
    }

    #[test]
    fn test_unix_socket_paths() {
        let paths = get_unix_socket_paths();
        assert!(!paths.is_empty());

        // Should always have /tmp fallback
        assert!(paths.iter().any(|p| p.starts_with("/tmp")));
    }

    #[test]
    fn test_unix_socket_paths_env_override() {
        let paths = get_unix_socket_paths_with(&UnixSocketPathHints {
            beardog_socket: Some("/custom/beardog.sock".to_string()),
            ..Default::default()
        });
        assert!(!paths.is_empty());
        assert!(paths[0].to_string_lossy().contains("custom"));
    }

    #[test]
    fn test_tcp_discovery_files() {
        let files = get_tcp_discovery_file_candidates();
        assert!(!files.is_empty());

        // Should always have /tmp fallback
        assert!(files.iter().any(|f| f.starts_with("/tmp")));
    }

    #[test]
    fn test_async_stream_trait_objects() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<UnixStream>();
        assert_send_sync::<TcpStream>();
    }

    #[tokio::test]
    async fn test_discover_beardog_endpoint_with_nonexistent_socket() {
        beardog_errors::process_env::set_var(
            "BEARDOG_SOCKET",
            "/tmp/.beardog_test_nonexistent_socket_path_42.sock",
        );
        let result = discover_beardog_endpoint().await;
        beardog_errors::process_env::remove_var("BEARDOG_SOCKET");

        // With a non-existent explicit socket and no TCP discovery file,
        // discovery should fail — unless the environment has a real socket
        // at a default path. Either outcome is valid for this unit test.
        if let Err(e) = &result {
            assert!(e.to_string().contains("Could not discover"));
        }
    }
}
