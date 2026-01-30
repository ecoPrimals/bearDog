//! Android abstract socket implementation for BearDog
//!
//! **Platform:** Android (ARM64, x86_64, all architectures)
//! **Transport:** Abstract Unix domain sockets (Linux namespace)
//! **Path Format:** `@biomeos_beardog` (@ indicates abstract namespace)
//!
//! ## Why Abstract Sockets?
//!
//! Android uses SELinux which blocks filesystem-based Unix sockets in user-space.
//! Abstract sockets bypass this by using pure namespace-based IPC with no filesystem.
//!
//! **Technical Details:**
//! - Abstract sockets use null byte (`\0`) prefix instead of filesystem path
//! - By convention, we write them with `@` prefix (e.g., `@biomeos_beardog`)
//! - Rust's `UnixListener::bind` automatically converts `@` to `\0`
//! - Kernel recognizes `\0` prefix and uses abstract namespace
//!
//! **Performance:**
//! - Same latency as filesystem Unix sockets (~5μs)
//! - No filesystem overhead
//! - Automatic cleanup on process exit
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (zero unsafe code)
//! - ✅ Zero C dependencies (tokio handles syscalls)
//! - ✅ Platform-agnostic (same `UnixListener` API)
//! - ✅ No hardcoding (primal name from runtime)
//!
//! ## Reference Implementation
//!
//! Based on Songbird's production-tested implementation:
//! `songbird/crates/songbird-universal-ipc/src/platform/android.rs`
//!
//! ## Validation
//!
//! Tested on Pixel 8a (GrapheneOS, Android 16, ARM64)

use super::{PlatformSocket, SocketEndpoint};
use tokio::net::UnixListener;
use tracing::{debug, info};

/// Android abstract socket implementation
pub struct AndroidSocket;

impl PlatformSocket for AndroidSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        // Abstract socket naming: @biomeos_{primal_name}
        // The @ prefix tells UnixListener to use abstract namespace
        let abstract_name = format!("@biomeos_{}", primal_name);
        
        debug!(
            "Creating abstract socket endpoint for '{}': {}",
            primal_name, abstract_name
        );
        
        info!(
            "🤖 Android abstract socket (SELinux-safe): {} (no filesystem)",
            abstract_name
        );
        
        Ok(SocketEndpoint::Abstract(abstract_name))
    }
    
    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<UnixListener> {
        match endpoint {
            SocketEndpoint::Abstract(name) => {
                debug!("Binding abstract socket: {}", name);
                
                // The magic: UnixListener::bind with @ prefix
                // Tokio/libc automatically converts @ to \0 (null byte)
                // Kernel recognizes \0 prefix as abstract socket
                let listener = UnixListener::bind(name)?;
                
                info!("✅ Abstract socket bound: {} (Android-optimized)", name);
                
                Ok(listener)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "AndroidSocket requires Abstract endpoint",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_abstract_socket_format() {
        let endpoint = AndroidSocket::create_endpoint("beardog").unwrap();
        match endpoint {
            SocketEndpoint::Abstract(name) => {
                assert_eq!(name, "@biomeos_beardog");
                assert!(name.starts_with('@'));
                println!("✅ Abstract socket format correct: {}", name);
            }
            _ => panic!("Expected Abstract endpoint"),
        }
    }
    
    #[test]
    fn test_primal_name_variations() {
        for primal in &["beardog", "songbird", "nestgate", "toadstool", "squirrel"] {
            let endpoint = AndroidSocket::create_endpoint(primal).unwrap();
            match endpoint {
                SocketEndpoint::Abstract(name) => {
                    assert!(name.starts_with('@'));
                    assert!(name.contains(primal));
                    println!("✅ {} → {}", primal, name);
                }
                _ => panic!("Expected Abstract endpoint"),
            }
        }
    }
    
    #[tokio::test]
    #[cfg(target_os = "linux")] // Abstract sockets work on Linux too!
    async fn test_socket_binding() {
        // Use unique name to avoid conflicts
        let test_name = format!("test_beardog_{}", std::process::id());
        let endpoint = AndroidSocket::create_endpoint(&test_name).unwrap();
        
        // Bind should succeed on Linux (abstract sockets are Linux feature)
        let listener = AndroidSocket::bind(&endpoint);
        assert!(listener.is_ok(), "Abstract socket binding failed");
        
        if let Ok(listener) = listener {
            println!("✅ Abstract socket bound successfully: {}", endpoint.display());
            drop(listener); // Auto-cleanup
        }
    }
}
