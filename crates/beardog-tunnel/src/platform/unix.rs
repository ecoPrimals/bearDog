//! Unix filesystem socket implementation for BearDog  
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
//! - ✅ Pure Rust (zero unsafe code)
//! - ✅ Zero C dependencies (tokio handles syscalls)
//! - ✅ Platform-agnostic (universal trait)
//! - ✅ No hardcoding (XDG Base Directory + runtime discovery)

use super::{PlatformListener, PlatformSocket, PlatformStream, SocketEndpoint};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, info};

/// Unix filesystem socket implementation
pub struct UnixSocket;

/// Wrapper to make UnixStream implement PlatformStream
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

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

/// Universal listener wrapper for Unix sockets
pub struct UnixPlatformListener {
    listener: UnixListener,
    path: String,
}

#[async_trait::async_trait]
impl PlatformListener for UnixPlatformListener {
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>> {
        let (stream, _addr) = self.listener.accept().await?;
        Ok(Box::new(UnixPlatformStream(stream)))
    }

    fn local_addr(&self) -> std::io::Result<String> {
        Ok(self.path.clone())
    }
}

impl PlatformSocket for UnixSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        // NOTE: This function contains blocking filesystem operations (std::fs::create_dir_all)
        // during directory creation. This is acceptable as:
        // 1. It runs only during initialization (not in hot path)
        // 2. Directory creation is infrequent (usually already exists)
        // 3. Making the trait async would require larger refactoring
        // TODO(Phase 3): Consider making PlatformSocket trait async for full non-blocking operation
        
        // Priority 1: Environment variable (operator control)
        if let Ok(custom_socket) = std::env::var("BEARDOG_SOCKET") {
            info!("📡 Using BEARDOG_SOCKET override: {}", custom_socket);
            return Ok(SocketEndpoint::Filesystem(custom_socket.into()));
        }

        // Priority 2: XDG Base Directory (standard Linux/Unix)
        let socket_path = if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            // XDG compliant: /run/user/$UID/biomeos/beardog.sock
            let biomeos_dir = std::path::PathBuf::from(&runtime_dir).join("biomeos");
            
            // Ensure directory exists (BLOCKING - acceptable for initialization)
            if !biomeos_dir.exists() {
                std::fs::create_dir_all(&biomeos_dir)?;
            }
            
            biomeos_dir.join(format!("{}.sock", primal_name))
        } else {
            // Priority 3: /tmp fallback (compatibility)
            let tmp_dir = std::path::PathBuf::from("/tmp/biomeos");
            
            // Ensure directory exists (BLOCKING - acceptable for initialization)
            if !tmp_dir.exists() {
                std::fs::create_dir_all(&tmp_dir)?;
            }
            
            tmp_dir.join(format!("{}.sock", primal_name))
        };

        debug!(
            "Creating Unix filesystem socket: {}",
            socket_path.display()
        );
        info!(
            "🐧 Unix filesystem socket (XDG-compliant): {} (automatic cleanup)",
            socket_path.display()
        );

        Ok(SocketEndpoint::Filesystem(socket_path))
    }

    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<Box<dyn PlatformListener>> {
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

                Ok(Box::new(UnixPlatformListener {
                    listener,
                    path: path_str,
                }))
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

    #[test]
    fn test_xdg_socket_path() {
        // Set XDG_RUNTIME_DIR
        std::env::set_var("XDG_RUNTIME_DIR", "/run/user/1000");
        
        let endpoint = UnixSocket::create_endpoint("beardog").unwrap();
        
        std::env::remove_var("XDG_RUNTIME_DIR");

        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                assert!(path.to_str().unwrap().contains("/run/user/1000/biomeos"));
                assert!(path.to_str().unwrap().ends_with("beardog.sock"));
                println!("✅ XDG-compliant path: {}", path.display());
            }
            _ => panic!("Expected Filesystem endpoint"),
        }
    }

    #[test]
    fn test_environment_override() {
        std::env::set_var("BEARDOG_SOCKET", "/custom/path/beardog.sock");
        
        let endpoint = UnixSocket::create_endpoint("beardog").unwrap();
        
        std::env::remove_var("BEARDOG_SOCKET");

        match endpoint {
            SocketEndpoint::Filesystem(path) => {
                assert_eq!(path.to_str().unwrap(), "/custom/path/beardog.sock");
                println!("✅ Environment override works");
            }
            _ => panic!("Expected Filesystem endpoint"),
        }
    }

    #[test]
    fn test_primal_name_variations() {
        for primal in &["beardog", "songbird", "nestgate", "toadstool", "squirrel"] {
            let endpoint = UnixSocket::create_endpoint(primal).unwrap();
            match endpoint {
                SocketEndpoint::Filesystem(path) => {
                    assert!(path.to_str().unwrap().contains(primal));
                    println!("✅ {} → {}", primal, path.display());
                }
                _ => panic!("Expected Filesystem endpoint"),
            }
        }
    }

    #[tokio::test]
    async fn test_universal_listener_trait() {
        // Create unique socket for test
        let test_socket = format!("/tmp/test_beardog_{}.sock", std::process::id());
        let endpoint = SocketEndpoint::Filesystem(test_socket.clone().into());

        // Bind using universal trait
        let mut listener = UnixSocket::bind(&endpoint).unwrap();

        // Verify local_addr works
        let addr = listener.local_addr().unwrap();
        assert_eq!(addr, test_socket);

        println!("✅ Universal PlatformListener trait working!");

        // Cleanup
        let _ = std::fs::remove_file(&test_socket);
    }
}
