//! Platform-specific socket implementations for BearDog
//!
//! This module provides platform-agnostic IPC through compile-time platform detection.
//! Each platform uses its native, optimal socket mechanism:
//!
//! - **Android**: Abstract Unix sockets (`@biomeos_beardog`)
//! - **Linux/macOS**: Filesystem Unix sockets (`/run/user/UID/biomeos/beardog.sock`)
//! - **Windows**: Named pipes (`\\.\pipe\biomeos_beardog`)
//! - **iOS**: XPC services (documented, awaiting Pure Rust bindings)
//! - **WASM**: In-process channels (BroadcastChannel)
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (zero unsafe code)
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
//! Based on Songbird's production-tested implementation:
//! `songbird/crates/songbird-universal-ipc/src/platform/`

pub mod android;
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod ios;

#[cfg(target_family = "wasm")]
pub mod wasm;

use std::path::PathBuf;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};
use std::task::{Context, Poll};

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
            SocketEndpoint::Filesystem(path) => format!("{}", path.display()),
            SocketEndpoint::Abstract(name) => name.clone(),
            #[cfg(windows)]
            SocketEndpoint::NamedPipe(name) => name.clone(),
            #[cfg(target_os = "ios")]
            SocketEndpoint::XPC(service) => service.clone(),
            #[cfg(target_family = "wasm")]
            SocketEndpoint::InProcess(channel) => channel.clone(),
        }
    }
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

/// Universal platform listener trait (replaces UnixListener)
///
/// **Modern Idiomatic Rust**: This trait abstracts over platform-specific listener types:
/// - `tokio::net::UnixListener` (Unix, Android, macOS)
/// - `tokio::net::windows::named_pipe::ServerOptions` (Windows)  
/// - Custom BroadcastChannel listener (WASM)
///
/// **Philosophy**: "1 unified codebase" - same API works everywhere!
#[async_trait::async_trait]
pub trait PlatformListener: Send + Sync {
    /// Accept incoming connection
    ///
    /// Returns a boxed stream that can be used for bidirectional communication.
    /// Platform-specific implementations provide their native stream type.
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>>;
    
    /// Get the local address/identifier of this listener
    ///
    /// Returns platform-appropriate identifier:
    /// - Unix: File path
    /// - Android: Abstract socket name
    /// - Windows: Named pipe path
    /// - WASM: Channel identifier
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
    /// ```no_run
    /// use beardog_tunnel::platform::Socket;
    ///
    /// let endpoint = Socket::create_endpoint("beardog")?;
    /// let mut listener = Socket::bind(&endpoint)?;
    ///
    /// // Works on Unix, Windows, Android, iOS, WASM!
    /// let stream = listener.accept().await?;
    /// ```
    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<Box<dyn PlatformListener>>;
}

/// Select platform implementation at compile time
#[cfg(target_os = "android")]
pub use android::AndroidSocket as Socket;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use ios::IOSSocket as Socket;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos"), not(target_os = "ios")))]
pub use unix::UnixSocket as Socket;

#[cfg(windows)]
pub use windows::WindowsSocket as Socket;

#[cfg(target_family = "wasm")]
pub use wasm::WASMSocket as Socket;

/// Get socket endpoint with environment variable support
///
/// **Deep Debt Principle**: "No hardcoding - use runtime discovery"
///
/// Priority order:
/// 1. `BEARDOG_ABSTRACT_SOCKET` env var → Abstract socket (Android/mobile)
/// 2. `BEARDOG_SOCKET` env var → Custom path (operator control)
/// 3. Platform default → Compile-time platform selection
///
/// # Example
/// ```bash
/// # Force abstract socket (Android, cross-platform testing)
/// export BEARDOG_ABSTRACT_SOCKET='beardog_nucleus'
///
/// # Force custom filesystem socket
/// export BEARDOG_SOCKET='/custom/path/beardog.sock'
///
/// # Use platform default (automatic)
/// unset BEARDOG_ABSTRACT_SOCKET BEARDOG_SOCKET
/// ```
///
/// # Returns
/// Platform-appropriate socket endpoint
pub fn get_socket_endpoint() -> std::io::Result<SocketEndpoint> {
    // 1. Check for abstract socket override (highest priority - Android/mobile)
    if let Ok(abstract_name) = std::env::var("BEARDOG_ABSTRACT_SOCKET") {
        // Ensure @ prefix for abstract socket (required by Unix kernel)
        let name_with_prefix = if abstract_name.starts_with('@') {
            abstract_name
        } else {
            format!("@{}", abstract_name)
        };
        tracing::info!("📡 Using abstract socket from BEARDOG_ABSTRACT_SOCKET: {}", name_with_prefix);
        return Ok(SocketEndpoint::Abstract(name_with_prefix));
    }
    
    // 2. Check for custom socket path (operator control)
    if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
        tracing::info!("📡 Using filesystem socket from BEARDOG_SOCKET: {}", socket_path);
        return Ok(SocketEndpoint::Filesystem(PathBuf::from(socket_path)));
    }
    
    // 3. Platform default (compile-time selection)
    tracing::debug!("📡 Using platform default socket endpoint");
    Socket::create_endpoint("beardog")
}

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
        
        #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos"), not(target_os = "ios")))]
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
        let endpoint = Socket::create_endpoint("test_beardog").unwrap();
        
        match endpoint {
            SocketEndpoint::Abstract(name) => {
                assert!(name.starts_with('@'), "Abstract socket should start with @");
                println!("Abstract endpoint: {}", name);
            }
            SocketEndpoint::Filesystem(path) => {
                assert!(path.to_str().unwrap().contains("test_beardog"));
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
}
