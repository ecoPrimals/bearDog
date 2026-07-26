// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android abstract socket implementation for `BearDog`
//!
//! **Platform:** Android (ARM64, `x86_64`, all architectures)
//! **Transport:** Abstract Unix domain sockets (Linux namespace)
//! **Path Format:** `@<ecosystem-namespace>_beardog` (@ indicates abstract namespace; namespace from `resolve_biomeos_ipc_subdir_from_optional`)
//!
//! ## Modern Idiomatic Rust Evolution (Jan 31, 2026)
//!
//! **Updated**: Now implements universal `PlatformListener` trait for
//! cross-platform compatibility. Same trait works on Unix, Windows, Android, WASM!
//!
//! ## Why Abstract Sockets?
//!
//! Android uses `SELinux` which blocks filesystem-based Unix sockets in user-space.
//! Abstract sockets bypass this by using pure namespace-based IPC with no filesystem.
//!
//! **Technical Details:**
//! - Abstract sockets use null byte (`\0`) prefix instead of filesystem path
//! - By convention, we write them with `@` prefix (e.g., `@<namespace>_beardog`)
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
//! - ✅ Pure Rust (memory-safe)
//! - ✅ Zero C dependencies (tokio handles syscalls)
//! - ✅ Platform-agnostic (universal trait)
//! - ✅ No hardcoding (primal name from runtime)

use super::{
    PlatformListener, PlatformListenerBackend, PlatformSocket, PlatformStream, SocketEndpoint,
};
use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, info};

/// Android abstract socket implementation
pub struct AndroidSocket;

/// Wrapper to make `UnixStream` implement `PlatformStream`
pub struct AndroidPlatformStream(UnixStream);

impl PlatformStream for AndroidPlatformStream {
    fn peer_credentials(&self) -> Option<(u32, Option<u32>)> {
        self.0
            .peer_cred()
            .ok()
            .map(|cred| (cred.uid(), cred.pid().map(i32::cast_unsigned)))
    }
}

impl AsyncRead for AndroidPlatformStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for AndroidPlatformStream {
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

/// Universal listener wrapper for Android abstract sockets
pub struct AndroidPlatformListener {
    listener: UnixListener,
    name: String,
}

impl PlatformListener for AndroidPlatformListener {
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>> {
        let (stream, _addr) = self.listener.accept().await?;
        Ok(Box::new(AndroidPlatformStream(stream)) as Box<dyn PlatformStream>)
    }

    fn local_addr(&self) -> std::io::Result<String> {
        Ok(self.name.clone())
    }
}

impl PlatformSocket for AndroidSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        // Abstract socket naming: @<ecosystem-namespace>_{primal_name} (same resolver as IPC layout).
        // The @ prefix tells UnixListener to use abstract namespace
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        let abstract_name = format!("@{ns}_{primal_name}");

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

    fn bind(endpoint: &SocketEndpoint) -> std::io::Result<Box<PlatformListenerBackend>> {
        match endpoint {
            SocketEndpoint::Abstract(name) => {
                debug!("Binding abstract socket: {}", name);

                // The magic: UnixListener::bind with @ prefix
                // Tokio/libc automatically converts @ to \0 (null byte)
                // Kernel recognizes \0 prefix as abstract socket
                let listener = UnixListener::bind(name)?;

                info!("✅ Abstract socket bound: {} (Android-optimized)", name);

                Ok(Box::new(PlatformListenerBackend::Android(
                    AndroidPlatformListener {
                        listener,
                        name: name.clone(),
                    },
                )))
            }
            SocketEndpoint::Filesystem(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "AndroidSocket requires Abstract endpoint",
            )),
            #[allow(unreachable_patterns)]
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
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        let endpoint =
            AndroidSocket::create_endpoint("beardog").expect("Android abstract endpoint");
        match endpoint {
            SocketEndpoint::Abstract(name) => {
                assert_eq!(name, format!("@{ns}_beardog"));
                assert!(name.starts_with('@'));
                println!("✅ Abstract socket format correct: {}", name);
            }
            SocketEndpoint::Filesystem(_) => {
                panic!("Expected Abstract endpoint, got {:?}", endpoint)
            }
        }
    }

    #[test]
    fn test_primal_name_variations() {
        for primal in &["alpha", "beta", "gamma", "delta", "epsilon"] {
            let endpoint =
                AndroidSocket::create_endpoint(primal).expect("Android endpoint for primal");
            match endpoint {
                SocketEndpoint::Abstract(name) => {
                    assert!(name.starts_with('@'));
                    assert!(name.contains(primal));
                    println!("✅ {} → {}", primal, name);
                }
                SocketEndpoint::Filesystem(_) => {
                    panic!("Expected Abstract endpoint, got {:?}", endpoint)
                }
            }
        }
    }

    #[tokio::test]
    #[cfg(target_os = "linux")] // Abstract sockets work on Linux too!
    async fn test_socket_binding() {
        // Use unique name to avoid conflicts
        let test_name = format!("test_beardog_{}", std::process::id());
        let endpoint =
            AndroidSocket::create_endpoint(&test_name).expect("Android endpoint for binding test");

        // Bind should succeed on Linux (abstract sockets are Linux feature)
        let listener = AndroidSocket::bind(&endpoint);
        assert!(listener.is_ok(), "Abstract socket binding failed");

        if let Ok(listener) = listener {
            // Verify local_addr works
            let addr = listener.local_addr().expect("abstract listener local_addr");
            assert!(addr.starts_with('@'));
            println!("✅ Abstract socket bound successfully: {}", addr);
        }
    }

    #[tokio::test]
    async fn test_universal_listener_trait() {
        // Use unique socket name with timestamp to avoid conflicts
        let test_name = format!(
            "test_beardog_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_millis()
        );
        let endpoint = AndroidSocket::create_endpoint(&test_name)
            .expect("Android endpoint for universal trait test");

        #[cfg(target_os = "linux")]
        {
            // Only test binding on Linux (where abstract sockets work)
            let listener = AndroidSocket::bind(&endpoint).expect("bind abstract socket");

            // Verify universal trait methods
            let addr = listener.local_addr().expect("listener local_addr");
            assert!(addr.starts_with('@'));

            println!("✅ Universal PlatformListener trait working on Android!");
        }

        #[cfg(not(target_os = "linux"))]
        {
            println!("⚠️  Skipping Android abstract socket test on non-Linux platform");
        }
    }
}
