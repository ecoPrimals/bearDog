// SPDX-License-Identifier: AGPL-3.0-or-later

//! Platform-specific socket implementations for `BearDog`
//!
//! This module provides platform-agnostic IPC through compile-time platform detection.
//! Each platform uses its native, optimal socket mechanism:
//!
//! - **Android**: Abstract Unix sockets (`@biomeos_beardog`)
//! - **Linux/macOS**: Filesystem Unix sockets (`/run/user/UID/biomeos/beardog.sock`)
//! - **Windows**: Named pipes (`\\.\pipe\biomeos_beardog`)
//! - **iOS**: XPC services (documented, awaiting Pure Rust bindings)
//! - **WASM**: In-process channels (`BroadcastChannel`)
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (memory-safe)
//! - ✅ Zero C dependencies
//! - ✅ Platform-agnostic (automatic detection)
//! - ✅ No hardcoding (runtime discovery)
//! - ✅ Universal trait-based abstraction
//!
//! ## Architecture
//!
//! Uses Rust's `#[cfg(target_os)]` for compile-time platform selection with
//! universal trait-based abstraction for runtime polymorphism.
//!
//! ## Modern Idiomatic Rust Evolution (Jan 31, 2026)
//!
//! **Problem**: Original `PlatformSocket` trait returned `UnixListener` which
//! is incompatible with Windows `NamedPipeServer`.
//!
//! **Solution**: Generic `PlatformListener` and `PlatformStream` traits that
//! work across all platforms - true universal abstraction!
//!
//! ## Reference
//!
//! Follows the biomeOS cross-platform IPC layout for primal-agnostic sockets.

pub mod android;
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod ios;

#[cfg(target_family = "wasm")]
pub mod wasm;

use std::path::PathBuf;
use tokio::io::{AsyncRead, AsyncWrite};

use beardog_types::constants::domains::config::system::DEFAULT_SYSTEM_NAME;
use beardog_types::constants::domains::network::ipc_discovery;

/// Platform-specific socket endpoint types
#[derive(Debug, Clone)]
pub enum SocketEndpoint {
    /// Filesystem-based Unix socket (Linux, macOS, BSD)
    Filesystem(PathBuf),

    /// Abstract Unix socket (Android, also works on Linux)
    Abstract(String),

    /// Windows named pipe (Windows all architectures)
    #[cfg(windows)]
    NamedPipe(String),

    /// iOS XPC service (iOS only, documented for future)
    #[cfg(target_os = "ios")]
    XPC(String),

    /// In-process channel (WASM only, no true IPC in browser)
    #[cfg(target_family = "wasm")]
    InProcess(String),
}

impl SocketEndpoint {
    /// Get the display string for logging
    pub fn display(&self) -> String {
        match self {
            Self::Filesystem(path) => format!("{}", path.display()),
            Self::Abstract(name) => name.clone(),
            #[cfg(windows)]
            SocketEndpoint::NamedPipe(name) => name.clone(),
            #[cfg(target_os = "ios")]
            SocketEndpoint::XPC(service) => service.clone(),
            #[cfg(target_family = "wasm")]
            SocketEndpoint::InProcess(channel) => channel.clone(),
        }
    }
}

/// Get platform-native default socket endpoint
///
/// **Deep Debt Principle #4 & #5**: Runtime discovery, platform-agnostic
///
/// This function returns the optimal socket type for the current platform:
/// - **Android**: Abstract sockets (bypasses SELinux)
/// - **Linux/macOS**: Filesystem Unix sockets
/// - **Windows**: Named pipes
/// - **iOS**: XPC services
/// - **WASM**: In-process channels
///
/// ## Evolution from Hardcoding
///
/// **Before**: Hardcoded `/tmp/beardog.sock` everywhere
/// **After**: Platform-native discovery at runtime
///
/// This enables universal deployment without user intervention.
#[cfg(target_os = "android")]
pub fn default_socket_endpoint() -> SocketEndpoint {
    default_socket_endpoint_for_primal(
        beardog_errors::process_env::var("PRIMAL_NAME")
            .ok()
            .as_deref(),
    )
}

/// Android abstract socket: `@{namespace}_{primal}` (namespace from [`beardog_types::constants::domains::network::ipc_discovery`]).
#[cfg(target_os = "android")]
pub fn default_socket_endpoint_for_primal(primal_name: Option<&str>) -> SocketEndpoint {
    let primal_name = primal_name
        .map(str::to_string)
        .or_else(|| beardog_errors::process_env::var("PRIMAL_NAME").ok())
        .or_else(|| beardog_errors::process_env::var("BEARDOG_PRIMAL_NAME").ok())
        .unwrap_or_else(|| DEFAULT_SYSTEM_NAME.to_string());
    let ns = beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional(None);
    SocketEndpoint::Abstract(format!("@{ns}_{primal_name}"))
}

/// Returns the default socket endpoint for the current Unix platform
#[cfg(all(unix, not(target_os = "android")))]
pub fn default_socket_endpoint() -> SocketEndpoint {
    default_socket_endpoint_for_primal(None)
}

/// Default Unix filesystem socket using an optional primal name (`None` → env / `beardog`).
#[cfg(all(unix, not(target_os = "android")))]
pub fn default_socket_endpoint_for_primal(primal_name: Option<&str>) -> SocketEndpoint {
    let primal_name = primal_name
        .map(str::to_string)
        .or_else(|| beardog_errors::process_env::var("PRIMAL_NAME").ok())
        .or_else(|| beardog_errors::process_env::var("BEARDOG_PRIMAL_NAME").ok())
        .unwrap_or_else(|| DEFAULT_SYSTEM_NAME.to_string());
    SocketEndpoint::Filesystem(
        ipc_discovery::biomeos_ipc_socket_dir_from_env().join(format!("{primal_name}.sock")),
    )
}

/// Same as [`default_socket_endpoint_for_primal`] but reads `PRIMAL_NAME` from the environment.
#[cfg(all(unix, not(target_os = "android")))]
pub fn default_socket_endpoint_from_env() -> SocketEndpoint {
    default_socket_endpoint_for_primal(
        beardog_errors::process_env::var("PRIMAL_NAME")
            .ok()
            .as_deref(),
    )
}

#[cfg(windows)]
pub fn default_socket_endpoint() -> SocketEndpoint {
    let primal_name = beardog_errors::process_env::var("PRIMAL_NAME")
        .or_else(|_| beardog_errors::process_env::var("BEARDOG_PRIMAL_NAME"))
        .unwrap_or_else(|_| DEFAULT_SYSTEM_NAME.to_string());
    windows::create_endpoint_with(
        &primal_name,
        beardog_errors::process_env::var("BEARDOG_PIPE")
            .ok()
            .as_deref(),
        beardog_errors::process_env::var("BIOMEOS_PIPE_DIR")
            .ok()
            .as_deref(),
    )
    .expect("Windows named pipe path")
}

#[cfg(target_os = "ios")]
pub fn default_socket_endpoint() -> SocketEndpoint {
    // iOS: Use XPC services (Apple's recommended IPC)
    SocketEndpoint::XPC("com.ecoprimals.beardog".to_string())
}

#[cfg(target_family = "wasm")]
pub fn default_socket_endpoint() -> SocketEndpoint {
    default_socket_endpoint_for_primal(None)
}

#[cfg(target_family = "wasm")]
pub fn default_socket_endpoint_for_primal(primal_name: Option<&str>) -> SocketEndpoint {
    let primal_name = primal_name
        .map(str::to_string)
        .or_else(|| beardog_errors::process_env::var("PRIMAL_NAME").ok())
        .or_else(|| beardog_errors::process_env::var("BEARDOG_PRIMAL_NAME").ok())
        .unwrap_or_else(|| DEFAULT_SYSTEM_NAME.to_string());
    SocketEndpoint::InProcess(primal_name)
}

#[cfg(target_family = "wasm")]
pub fn default_socket_endpoint_from_env() -> SocketEndpoint {
    default_socket_endpoint_for_primal(
        beardog_errors::process_env::var("PRIMAL_NAME")
            .ok()
            .as_deref(),
    )
}

/// Convert socket endpoint to string path (for CLI defaults)
///
/// **Deep Debt Principle #4**: Capability-based, not hardcoded
pub fn default_socket_path() -> String {
    default_socket_endpoint().display()
}

/// Universal platform stream trait (replaces platform-specific types)
///
/// **Modern Idiomatic Rust**: This trait abstracts over platform-specific stream types:
/// - `tokio::net::UnixStream` (Unix, Android, macOS)
/// - `tokio::net::windows::named_pipe::NamedPipeServer` (Windows)
/// - `web_sys::MessagePort` (WASM)
///
/// All platforms provide async read/write through this unified interface.
pub trait PlatformStream: AsyncRead + AsyncWrite + Send + Sync + Unpin {}

/// Universal platform listener trait (replaces `UnixListener`)
///
/// **Modern Idiomatic Rust**: This trait abstracts over platform-specific listener types:
/// - `tokio::net::UnixListener` (Unix, Android, macOS)
/// - `tokio::net::windows::named_pipe::ServerOptions` (Windows)  
/// - Custom `BroadcastChannel` listener (WASM)
///
/// **Philosophy**: "1 unified codebase" - same API works everywhere!
#[async_trait::async_trait]
pub trait PlatformListener: Send + Sync {
    /// Accept incoming connection
    ///
    /// Returns a boxed stream that can be used for bidirectional communication.
    /// Platform-specific implementations provide their native stream type.
    ///
    /// # Errors
    ///
    /// Returns an error if accepting the next connection fails.
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>>;

    /// Get the local address/identifier of this listener
    ///
    /// Returns platform-appropriate identifier:
    /// - Unix: File path
    /// - Android: Abstract socket name
    /// - Windows: Named pipe path
    /// - WASM: Channel identifier
    ///
    /// # Errors
    ///
    /// Returns an error if the local address cannot be read from the listener.
    fn local_addr(&self) -> std::io::Result<String>;
}

/// Platform-specific socket operations
///
/// **Modern Idiomatic Rust Evolution**: Now returns generic `PlatformListener`
/// instead of Unix-specific `UnixListener`. This enables true universal support!
///
/// Implementations handle platform-specific binding logic while exposing
/// a unified interface through trait objects.
pub trait PlatformSocket {
    /// Create platform-appropriate socket endpoint
    ///
    /// # Arguments
    /// * `primal_name` - Name of the primal (e.g., "beardog")
    ///
    /// # Returns
    /// Platform-specific socket endpoint
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint path or configuration cannot be created.
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint>;

    /// Bind listener to endpoint (EVOLVED: Now universal!)
    ///
    /// **Modern Idiomatic Rust**: Returns `Box<dyn PlatformListener>` which works
    /// across all platforms. Callers don't need to know about platform specifics!
    ///
    /// # Arguments
    /// * `endpoint` - Socket endpoint to bind
    ///
    /// # Returns
    /// Boxed platform listener ready to accept connections
    ///
    /// # Example
    /// ```ignore
    /// use beardog_tunnel::platform::{Socket, PlatformSocket};
    ///
    /// let endpoint = Socket::create_endpoint("beardog")?;
    /// let mut listener = Socket::bind(&endpoint)?;
    ///
    /// // Works on Unix, Windows, Android, iOS, WASM!
    /// let stream = listener.accept().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the listener cannot be bound to the endpoint.
    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<Box<dyn PlatformListener>>;
}

/// Select platform implementation at compile time
#[cfg(target_os = "android")]
pub use android::AndroidSocket as Socket;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use ios::IOSSocket as Socket;

#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "macos"),
    not(target_os = "ios")
))]
pub use unix::UnixSocket as Socket;

#[cfg(windows)]
pub use windows::WindowsSocket as Socket;

#[cfg(target_family = "wasm")]
pub use wasm::WASMSocket as Socket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_selection() {
        // Verify correct platform is selected at compile time
        #[cfg(target_os = "android")]
        {
            println!("Platform: Android (abstract sockets)");
        }

        #[cfg(target_os = "macos")]
        {
            println!("Platform: macOS (filesystem sockets via iOS module)");
        }

        #[cfg(target_os = "ios")]
        {
            println!("Platform: iOS (XPC documented, awaiting Pure Rust bindings)");
        }

        #[cfg(all(
            unix,
            not(target_os = "android"),
            not(target_os = "macos"),
            not(target_os = "ios")
        ))]
        {
            println!("Platform: Unix (filesystem sockets)");
        }

        #[cfg(windows)]
        {
            println!("Platform: Windows (named pipes)");
        }

        #[cfg(target_family = "wasm")]
        {
            println!("Platform: WASM (in-process channels, no true IPC)");
        }
    }

    #[test]
    fn test_endpoint_creation() {
        let endpoint = Socket::create_endpoint("test_beardog").expect("create_endpoint");

        match endpoint {
            SocketEndpoint::Abstract(name) => {
                assert!(name.starts_with('@'), "Abstract socket should start with @");
                println!("Abstract endpoint: {}", name);
            }
            SocketEndpoint::Filesystem(path) => {
                let s = path.to_str().expect("utf-8 path");
                assert!(s.contains("test_beardog"));
                println!("Filesystem endpoint: {}", path.display());
            }
            #[cfg(windows)]
            SocketEndpoint::NamedPipe(name) => {
                assert!(name.contains("test_beardog"));
                println!("Named pipe endpoint: {}", name);
            }
            #[cfg(target_os = "ios")]
            SocketEndpoint::XPC(service) => {
                assert!(service.contains("test_beardog"));
                assert!(service.starts_with("org.biomeos."));
                println!("XPC service endpoint: {}", service);
            }
            #[cfg(target_family = "wasm")]
            SocketEndpoint::InProcess(channel) => {
                assert!(channel.contains("test_beardog"));
                println!("In-process channel endpoint: {}", channel);
            }
        }
    }

    #[test]
    fn socket_endpoint_display_matches_variant() {
        let fs = SocketEndpoint::Filesystem(PathBuf::from("/tmp/x.sock"));
        assert_eq!(fs.display(), "/tmp/x.sock");
        let abs = SocketEndpoint::Abstract("@ns_primal".to_string());
        assert_eq!(abs.display(), "@ns_primal");
    }

    #[cfg(all(unix, not(target_os = "android")))]
    #[test]
    fn default_socket_endpoint_for_primal_sets_path() {
        let ep = default_socket_endpoint_for_primal(Some("myprimal"));
        match ep {
            SocketEndpoint::Filesystem(p) => {
                let s = p.to_str().expect("utf-8 path");
                assert!(s.contains("myprimal"), "path={s}");
            }
            other @ SocketEndpoint::Abstract(_) => {
                panic!("expected filesystem socket on this platform: {other:?}")
            }
        }
    }

    #[test]
    fn default_socket_path_is_non_empty() {
        let p = default_socket_path();
        assert!(!p.is_empty());
    }
}
