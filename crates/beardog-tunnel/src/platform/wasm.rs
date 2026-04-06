// SPDX-License-Identifier: AGPL-3.0-or-later

//! WASM (WebAssembly) platform implementation for BearDog
//!
//! **Platform:** WebAssembly (browser, Node.js, Deno, embedded WASM runtimes)
//! **Transport:** In-process channels (no true IPC in browser sandbox)
//!
//! ## WASM Constraints
//!
//! WebAssembly runs in a sandboxed environment with no direct access to:
//! - Filesystem (no Unix sockets)
//! - Network sockets (browser security model)
//! - OS-level IPC mechanisms
//!
//! ## Implementation Strategy
//!
//! **Browser WASM:**
//! - In-process message channels (PostMessage API)
//! - SharedArrayBuffer for zero-copy communication (when available)
//! - WebSocket to server-side primals (for true inter-primal IPC)
//!
//! **Server-Side WASM (Node.js, Deno):**
//! - May have access to OS primitives via host bindings
//! - Can potentially use TCP localhost
//! - Fallback to in-process channels
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Pure Rust (memory-safe)
//! - ✅ Platform-agnostic (works in any WASM runtime)
//! - ⚠️  No true IPC (WASM limitation, not code limitation)
//! - ✅ In-process channels for single-binary deployment
//! - ✅ WebSocket option for distributed deployment
//!
//! ## Use Cases
//!
//! 1. **Browser-Based Primals:**
//!    - Web-based BearDog UI
//!    - Browser crypto operations
//!    - Local-first applications
//!
//! 2. **Edge/Serverless:**
//!    - Cloudflare Workers
//!    - Fastly Compute@Edge
//!    - AWS Lambda (with WASM runtime)
//!
//! 3. **Embedded Systems:**
//!    - IoT devices with WASM runtime
//!    - Microcontrollers with WASM support
//!    - Constrained environments
//!
//! ## Implementation Notes
//!
//! This module provides a **documented skeleton** for WASM deployment.
//! Full implementation requires:
//! - `wasm-bindgen` for browser integration
//! - `web-sys` for Web APIs
//! - PostMessage channel implementation
//! - Optional: WebSocket for distributed primals
//!
//! ## References
//!
//! - WASM IPC patterns: Limited by sandbox, use message passing
//! - wasm-bindgen: https://rustwasm.github.io/wasm-bindgen/
//! - Songbird reference: songbird-universal-ipc/src/platform/wasm.rs

use super::{PlatformSocket, SocketEndpoint};
use tokio::net::UnixListener;
use tracing::{info, warn};

/// WASM platform implementation
///
/// **Browser**: In-process channels (no true IPC)
/// **Server-side WASM**: May have access to OS primitives
pub struct WASMSocket;

impl PlatformSocket for WASMSocket {
    fn create_endpoint(primal_name: &str) -> std::io::Result<SocketEndpoint> {
        let channel_name = format!("beardog_channel_{}", primal_name);

        info!("🌐 WASM in-process channel: {}", channel_name);
        warn!("⚠️  WASM deployment uses in-process channels (no true IPC)");
        warn!("   Browser limitation: No access to filesystem or network sockets");
        warn!("   Options:");
        warn!("   1. In-process channels (single binary deployment)");
        warn!("   2. PostMessage API (browser window communication)");
        warn!("   3. WebSocket to server-side primal (distributed deployment)");
        warn!("   4. SharedArrayBuffer for zero-copy (when available)");

        // Document the in-process channel for future implementation
        Ok(SocketEndpoint::InProcess(channel_name))
    }

    fn bind(_endpoint: &SocketEndpoint) -> std::io::Result<UnixListener> {
        warn!("WASM socket binding not implemented");
        warn!("WebAssembly cannot bind traditional sockets due to sandbox");
        warn!("Implementation requires:");
        warn!("  1. wasm-bindgen for browser integration");
        warn!("  2. In-process message channels (mpsc, broadcast)");
        warn!("  3. PostMessage API bindings (for browser)");
        warn!("  4. WebSocket client (for distributed primals)");

        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "WASM deployment requires in-process channels or WebSocket (traditional sockets not available in browser sandbox)",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_channel_format() {
        let endpoint = WASMSocket::create_endpoint("beardog").expect("WASM in-process endpoint");
        match endpoint {
            SocketEndpoint::InProcess(channel) => {
                assert!(channel.contains("beardog"));
                assert!(channel.starts_with("beardog_channel_"));
                println!("✅ WASM channel: {}", channel);
            }
            _ => panic!("Expected InProcess endpoint for WASM, got {:?}", endpoint),
        }
    }

    #[test]
    fn test_primal_name_variations() {
        for primal in &["alpha", "beta", "gamma", "delta", "epsilon"] {
            let endpoint = WASMSocket::create_endpoint(primal).expect("WASM endpoint for primal");
            match endpoint {
                SocketEndpoint::InProcess(channel) => {
                    assert!(channel.contains(primal));
                    println!("✅ WASM {} → {}", primal, channel);
                }
                _ => panic!("Expected InProcess endpoint for WASM, got {:?}", endpoint),
            }
        }
    }

    #[test]
    fn test_wasm_binding_returns_unsupported() {
        let endpoint = WASMSocket::create_endpoint("beardog").expect("WASM endpoint for bind test");
        let result = WASMSocket::bind(&endpoint);

        assert!(result.is_err(), "WASM binding should return error");
        let err = result.expect_err("WASM bind should fail in sandbox");
        assert_eq!(err.kind(), std::io::ErrorKind::Unsupported);
        println!("✅ WASM correctly returns Unsupported (expected for browser sandbox)");
    }
}
