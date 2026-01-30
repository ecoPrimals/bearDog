//! Unix filesystem socket implementation (Linux, macOS)
//!
//! **Platforms:** Linux (non-Android), macOS, BSD
//! **Transport:** Filesystem-based Unix domain sockets
//! **Path Format:** `/run/user/{UID}/biomeos/beardog.sock`
//!
//! ## XDG Base Directory Compliance
//!
//! Uses `XDG_RUNTIME_DIR` or `BIOMEOS_SOCKET_DIR` for socket placement,
//! falling back to `current_dir()` if neither is available.
//!
//! **Priority:**
//! 1. `BEARDOG_SOCKET` env var (highest - exact path override)
//! 2. `BIOMEOS_SOCKET_DIR` env var (shared standard)
//! 3. `XDG_RUNTIME_DIR/biomeos/` (XDG-compliant)
//! 4. `current_dir()` (fallback)
//!
//! ## Filesystem Benefits
//!
//! - **Permissions:** Use filesystem ACLs for security
//! - **Discovery:** Can use `ls` to see socket files
//! - **Compatibility:** Works on all Unix systems
//! - **Performance:** ~5μs latency, 10GB/s throughput
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (zero unsafe code)
//! - ✅ Zero C dependencies
//! - ✅ XDG-compliant (standard paths)
//! - ✅ No hardcoding (environment-driven)

use super::{PlatformSocket, SocketEndpoint};
use std::path::PathBuf;
use tokio::net::UnixListener;
use tracing::{debug, info, warn};

/// Unix filesystem socket implementation
pub struct UnixSocket;

impl PlatformSocket for UnixSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        // Priority 1: Exact override via BEARDOG_SOCKET
        if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
            info!("Using BEARDOG_SOCKET override: {}", socket_path);
            return Ok(SocketEndpoint::Filesystem(PathBuf::from(socket_path)));
        }
        
        // Priority 2: BIOMEOS_SOCKET_DIR (shared standard)
        let socket_dir = if let Ok(biomeos_dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
            debug!("Using BIOMEOS_SOCKET_DIR: {}", biomeos_dir);
            PathBuf::from(biomeos_dir)
        } else if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
            // Priority 3: XDG_RUNTIME_DIR (standard)
            debug!("Using XDG_RUNTIME_DIR: {}", xdg_runtime);
            PathBuf::from(xdg_runtime).join("biomeos")
        } else {
            // Priority 4: Fallback to current directory
            warn!("XDG_RUNTIME_DIR not set, using current directory fallback");
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("biomeos")
        };
        
        // Ensure directory exists
        if let Err(e) = std::fs::create_dir_all(&socket_dir) {
            warn!("Failed to create socket directory {}: {}", socket_dir.display(), e);
        }
        
        let socket_path = socket_dir.join(format!("{}.sock", primal_name));
        
        info!("🐧 Unix socket path (filesystem): {}", socket_path.display());
        
        Ok(SocketEndpoint::Filesystem(socket_path))
    }
    
    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<UnixListener> {
        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                debug!("Binding filesystem socket: {}", path.display());
                
                // Remove stale socket file if exists
                if path.exists() {
                    debug!("Removing stale socket file: {}", path.display());
                    std::fs::remove_file(path)?;
                }
                
                let listener = UnixListener::bind(path)?;
                
                info!("✅ Unix socket bound: {}", path.display());
                
                Ok(listener)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "UnixSocket requires Filesystem endpoint",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    
    #[test]
    #[serial] // Environment variables need serial execution
    fn test_filesystem_socket_format() {
        // Clear environment for clean test
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_DIR");
        
        let endpoint = UnixSocket::create_endpoint("beardog").unwrap();
        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                let path_str = path.to_string_lossy();
                assert!(path_str.ends_with("beardog.sock"), 
                    "Path should end with beardog.sock, got: {}", path_str);
                println!("✅ Filesystem socket: {}", path.display());
            }
            _ => panic!("Expected Filesystem endpoint"),
        }
    }
    
    #[test]
    #[serial] // Environment variables need serial execution
    fn test_environment_variable_override() {
        // Test BEARDOG_SOCKET override
        std::env::set_var("BEARDOG_SOCKET", "/tmp/test_override.sock");
        let endpoint = UnixSocket::create_endpoint("beardog").unwrap();
        std::env::remove_var("BEARDOG_SOCKET");
        
        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                assert_eq!(path.to_string_lossy(), "/tmp/test_override.sock");
                println!("✅ BEARDOG_SOCKET override works");
            }
            _ => panic!("Expected Filesystem endpoint"),
        }
    }
    
    #[test]
    #[serial] // Environment variables need serial execution
    fn test_biomeos_socket_dir() {
        // Clear BEARDOG_SOCKET to test BIOMEOS_SOCKET_DIR
        std::env::remove_var("BEARDOG_SOCKET");
        
        // Test BIOMEOS_SOCKET_DIR
        std::env::set_var("BIOMEOS_SOCKET_DIR", "/tmp/biomeos_test");
        let endpoint = UnixSocket::create_endpoint("beardog").unwrap();
        std::env::remove_var("BIOMEOS_SOCKET_DIR");
        
        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                let path_str = path.to_string_lossy();
                assert!(path_str.contains("/tmp/biomeos_test"), 
                    "Path should contain /tmp/biomeos_test, got: {}", path_str);
                assert!(path_str.ends_with("beardog.sock"));
                println!("✅ BIOMEOS_SOCKET_DIR works: {}", path.display());
            }
            _ => panic!("Expected Filesystem endpoint"),
        }
    }
    
    #[tokio::test]
    async fn test_socket_binding() {
        use std::env;
        
        // Use temp directory for test
        let test_dir = env::temp_dir().join("beardog_test");
        env::set_var("BIOMEOS_SOCKET_DIR", &test_dir);
        
        let test_name = format!("test_beardog_{}", std::process::id());
        let endpoint = UnixSocket::create_endpoint(&test_name).unwrap();
        
        let listener = UnixSocket::bind(&endpoint);
        assert!(listener.is_ok(), "Socket binding failed");
        
        if let Ok(listener) = listener {
            println!("✅ Socket bound successfully");
            drop(listener);
            
            // Cleanup
            if let SocketEndpoint::Filesystem(path) = endpoint {
                let _ = std::fs::remove_file(&path);
            }
        }
        
        env::remove_var("BIOMEOS_SOCKET_DIR");
        let _ = std::fs::remove_dir_all(&test_dir);
    }
}
