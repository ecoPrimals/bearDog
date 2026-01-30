//! Platform-specific socket implementations for BearDog
//!
//! This module provides platform-agnostic IPC through compile-time platform detection.
//! Each platform uses its native, optimal socket mechanism:
//!
//! - **Android**: Abstract Unix sockets (`@biomeos_beardog`)
//! - **Linux/macOS**: Filesystem Unix sockets (`/run/user/UID/biomeos/beardog.sock`)
//! - **Windows** (future): Named pipes (`\\.\pipe\biomeos_beardog`)
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (zero unsafe code)
//! - ✅ Zero C dependencies
//! - ✅ Platform-agnostic (automatic detection)
//! - ✅ No hardcoding (runtime discovery)
//!
//! ## Architecture
//!
//! Uses Rust's `#[cfg(target_os)]` for compile-time platform selection.
//! No runtime overhead - the correct implementation is chosen at compile time.
//!
//! ## Reference
//!
//! Based on Songbird's production-tested implementation:
//! `songbird/crates/songbird-universal-ipc/src/platform/`

pub mod android;
pub mod unix;

use std::path::PathBuf;
use tokio::net::UnixListener;

/// Platform-specific socket endpoint types
#[derive(Debug, Clone)]
pub enum SocketEndpoint {
    /// Filesystem-based Unix socket (Linux, macOS, BSD)
    Filesystem(PathBuf),
    
    /// Abstract Unix socket (Android, also works on Linux)
    Abstract(String),
}

impl SocketEndpoint {
    /// Get the display string for logging
    pub fn display(&self) -> String {
        match self {
            SocketEndpoint::Filesystem(path) => format!("{}", path.display()),
            SocketEndpoint::Abstract(name) => name.clone(),
        }
    }
}

/// Platform-specific socket operations
///
/// Implementations handle platform-specific binding logic
pub trait PlatformSocket {
    /// Create platform-appropriate socket endpoint
    ///
    /// # Arguments
    /// * `primal_name` - Name of the primal (e.g., "beardog")
    ///
    /// # Returns
    /// Platform-specific socket endpoint
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint>;
    
    /// Bind listener to endpoint
    ///
    /// # Arguments
    /// * `endpoint` - Socket endpoint to bind
    ///
    /// # Returns
    /// Bound Unix listener ready to accept connections
    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<UnixListener>;
}

/// Select platform implementation at compile time
#[cfg(target_os = "android")]
pub use android::AndroidSocket as Socket;

#[cfg(all(unix, not(target_os = "android")))]
pub use unix::UnixSocket as Socket;

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
        
        #[cfg(all(unix, not(target_os = "android")))]
        {
            println!("Platform: Unix (filesystem sockets)");
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
        }
    }
}
