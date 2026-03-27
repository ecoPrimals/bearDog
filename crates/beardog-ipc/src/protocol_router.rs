// SPDX-License-Identifier: AGPL-3.0-only

//! # Protocol Router for BearDog IPC
//!
//! **AUTOMATIC PROTOCOL DETECTION AND ROUTING** (v1.0.0)
//!
//! Detects incoming connection protocol and routes to appropriate handler:
//! - **tarpc** (binary): Highest performance (~10-20μs latency)
//! - **JSON-RPC**: Flexible, human-readable (~100-500μs latency)
//! - **HTTP**: Legacy compatibility
//!
//! ## Protocol Detection Strategy
//! Uses first-bytes sniffing to identify protocol:
//! - tarpc/binary: Length-prefixed binary frames
//! - JSON-RPC: `{` character (JSON object start)
//! - HTTP: `GET`, `POST`, `PUT`, `DELETE`, etc.
//!
//! ## Priority Order
//! tarpc > JSON-RPC > HTTP
//!
//! ## Philosophy: Walk → Run
//! New interactions start with JSON-RPC (walking - flexible, observable).
//! As patterns stabilize, they graduate to tarpc (running - fast, efficient).

use std::io;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::debug;

/// Detected protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// tarpc binary RPC (highest performance)
    Tarpc,

    /// JSON-RPC 2.0 (flexible, human-readable)
    JsonRpc,

    /// HTTP/1.1 (legacy compatibility)
    Http,

    /// Unknown/unrecognized protocol
    Unknown,
}

impl Protocol {
    /// Protocol priority (higher = preferred)
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Tarpc => 3, // Highest
            Self::JsonRpc => 2,
            Self::Http => 1,
            Self::Unknown => 0,
        }
    }

    /// Human-readable name
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Tarpc => "tarpc",
            Self::JsonRpc => "json-rpc",
            Self::Http => "http",
            Self::Unknown => "unknown",
        }
    }

    /// Is this a high-performance protocol?
    pub const fn is_high_performance(&self) -> bool {
        matches!(self, Self::Tarpc)
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Protocol detector for incoming connections
///
/// Sniffs first bytes to determine protocol without consuming the stream.
#[derive(Debug, Clone)]
pub struct ProtocolDetector {
    /// Maximum bytes to peek for detection
    peek_size: usize,
}

impl ProtocolDetector {
    /// Create a new protocol detector
    pub const fn new() -> Self {
        Self { peek_size: 16 }
    }

    /// Create with custom peek size
    pub const fn with_peek_size(peek_size: usize) -> Self {
        Self { peek_size }
    }

    /// Detect protocol from first bytes (non-consuming peek)
    ///
    /// Returns (detected_protocol, peeked_bytes) so caller can prepend
    /// peeked bytes back to the stream if needed.
    pub async fn detect(&self, stream: &mut TcpStream) -> io::Result<(Protocol, Vec<u8>)> {
        let mut buf = vec![0u8; self.peek_size];

        // Read first bytes (this consumes them from the stream)
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok((Protocol::Unknown, vec![]));
        }

        buf.truncate(n);
        let protocol = Self::detect_from_bytes(&buf);

        debug!("Detected protocol: {} from {} bytes", protocol, n);
        Ok((protocol, buf))
    }

    /// Detect protocol from bytes (pure function)
    pub fn detect_from_bytes(bytes: &[u8]) -> Protocol {
        if bytes.is_empty() {
            return Protocol::Unknown;
        }

        // Check for tarpc/binary (length-prefixed binary)
        // Length-prefixed frames start with a 4-byte length prefix (little-endian u32)
        // The length should be reasonable (< 16MB)
        if bytes.len() >= 4 {
            let frame_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

            // If frame length looks valid for tarpc (non-zero, < 16MB, not ASCII)
            // and first byte after length is not ASCII text
            if frame_len > 0 && frame_len < 16 * 1024 * 1024 {
                // Check if this looks like binary data (not ASCII text)
                if bytes.len() > 4 {
                    let fifth = bytes[4];
                    // tarpc binary payloads typically start with enum variant indices
                    // or struct field counts (small numbers), not ASCII printable chars
                    if !fifth.is_ascii_graphic() || fifth < 0x20 {
                        return Protocol::Tarpc;
                    }
                }
            }
        }

        // Check for JSON-RPC (starts with '{' or whitespace then '{')
        let mut trimmed = bytes
            .iter()
            .skip_while(|&&b| b == b' ' || b == b'\t' || b == b'\n' || b == b'\r');
        if let Some(&first_char) = trimmed.next()
            && first_char == b'{'
        {
            // Likely JSON - check for JSON-RPC fields
            if let Ok(text) = std::str::from_utf8(bytes) {
                if text.contains("jsonrpc") || text.contains("method") || text.contains("id") {
                    return Protocol::JsonRpc;
                }
                // Generic JSON, treat as JSON-RPC
                return Protocol::JsonRpc;
            }
        }

        // Check for HTTP (methods)
        let http_methods = [
            b"GET ".as_slice(),
            b"POST ".as_slice(),
            b"PUT ".as_slice(),
            b"DELETE ".as_slice(),
            b"PATCH ".as_slice(),
            b"HEAD ".as_slice(),
            b"OPTIONS ".as_slice(),
            b"CONNECT ".as_slice(),
        ];

        for method in &http_methods {
            if bytes.starts_with(method) {
                return Protocol::Http;
            }
        }

        // Could also be HTTP response
        if bytes.starts_with(b"HTTP/") {
            return Protocol::Http;
        }

        Protocol::Unknown
    }
}

impl Default for ProtocolDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Protocol router configuration
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Enable tarpc handling
    pub enable_tarpc: bool,

    /// Enable JSON-RPC handling
    pub enable_jsonrpc: bool,

    /// Enable HTTP handling
    pub enable_http: bool,

    /// Preferred protocol (for capability negotiation)
    pub preferred: Protocol,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            enable_tarpc: true,
            enable_jsonrpc: true,
            enable_http: true,
            preferred: Protocol::Tarpc, // Prefer high-performance
        }
    }
}

impl RouterConfig {
    /// Create config with only tarpc enabled
    pub const fn tarpc_only() -> Self {
        Self {
            enable_tarpc: true,
            enable_jsonrpc: false,
            enable_http: false,
            preferred: Protocol::Tarpc,
        }
    }

    /// Create config with only JSON-RPC enabled
    pub const fn jsonrpc_only() -> Self {
        Self {
            enable_tarpc: false,
            enable_jsonrpc: true,
            enable_http: false,
            preferred: Protocol::JsonRpc,
        }
    }

    /// Create config for development (all protocols, prefer JSON-RPC for debugging)
    pub const fn development() -> Self {
        Self {
            enable_tarpc: true,
            enable_jsonrpc: true,
            enable_http: true,
            preferred: Protocol::JsonRpc,
        }
    }

    /// Create config for production (all protocols, prefer tarpc for performance)
    pub fn production() -> Self {
        Self::default()
    }

    /// Get list of supported protocols
    pub fn supported_protocols(&self) -> Vec<Protocol> {
        let mut protocols = Vec::new();

        if self.enable_tarpc {
            protocols.push(Protocol::Tarpc);
        }
        if self.enable_jsonrpc {
            protocols.push(Protocol::JsonRpc);
        }
        if self.enable_http {
            protocols.push(Protocol::Http);
        }

        protocols
    }

    /// Check if a protocol is supported
    pub const fn is_supported(&self, protocol: Protocol) -> bool {
        match protocol {
            Protocol::Tarpc => self.enable_tarpc,
            Protocol::JsonRpc => self.enable_jsonrpc,
            Protocol::Http => self.enable_http,
            Protocol::Unknown => false,
        }
    }
}

/// Capability advertisement for protocol negotiation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtocolCapabilities {
    /// Supported protocols in priority order
    pub supported: Vec<String>,

    /// Recommended protocol for this connection
    pub recommended: String,

    /// High-performance protocols available
    pub high_performance: Vec<String>,

    /// Protocol version info
    pub versions: std::collections::HashMap<String, String>,
}

impl ProtocolCapabilities {
    /// Create capabilities from router config
    pub fn from_config(config: &RouterConfig) -> Self {
        let mut supported = Vec::new();
        let mut high_performance = Vec::new();
        let mut versions = std::collections::HashMap::new();

        if config.enable_tarpc {
            supported.push("tarpc".to_string());
            high_performance.push("tarpc".to_string());
            versions.insert("tarpc".to_string(), "0.34".to_string());
        }

        if config.enable_jsonrpc {
            supported.push("json-rpc".to_string());
            versions.insert("json-rpc".to_string(), "2.0".to_string());
        }

        if config.enable_http {
            supported.push("http".to_string());
            versions.insert("http".to_string(), "1.1".to_string());
        }

        Self {
            supported,
            recommended: config.preferred.name().to_string(),
            high_performance,
            versions,
        }
    }

    /// Create JSON representation for capability response
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Stream wrapper that prepends peeked bytes
///
/// When we peek bytes for protocol detection, we need to "unread" them
/// so the protocol handler sees the complete message.
pub struct PrefixedStream {
    prefix: Vec<u8>,
    prefix_pos: usize,
    inner: TcpStream,
}

impl PrefixedStream {
    /// Create a new prefixed stream
    pub const fn new(prefix: Vec<u8>, inner: TcpStream) -> Self {
        Self {
            prefix,
            prefix_pos: 0,
            inner,
        }
    }

    /// Consume into inner stream (after prefix is exhausted)
    pub fn into_inner(self) -> TcpStream {
        self.inner
    }

    /// Check if prefix has been fully read
    pub fn prefix_exhausted(&self) -> bool {
        self.prefix_pos >= self.prefix.len()
    }
}

impl tokio::io::AsyncRead for PrefixedStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        // First, serve from prefix
        if self.prefix_pos < self.prefix.len() {
            let remaining = &self.prefix[self.prefix_pos..];
            let to_copy = std::cmp::min(remaining.len(), buf.remaining());
            buf.put_slice(&remaining[..to_copy]);
            self.prefix_pos += to_copy;
            return std::task::Poll::Ready(Ok(()));
        }

        // Prefix exhausted, read from inner stream
        std::pin::Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl tokio::io::AsyncWrite for PrefixedStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        std::pin::Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::pin::Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jsonrpc() {
        let bytes = br#"{"jsonrpc": "2.0", "method": "test", "id": 1}"#;
        assert_eq!(
            ProtocolDetector::detect_from_bytes(bytes),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_detect_http_get() {
        // Host header is arbitrary; detection keys off request line only
        let bytes = b"GET / HTTP/1.1\r\nHost: discarded.test\r\n\r\n";
        assert_eq!(ProtocolDetector::detect_from_bytes(bytes), Protocol::Http);
    }

    #[test]
    fn test_detect_http_post() {
        let bytes = b"POST /api HTTP/1.1\r\nContent-Type: application/json\r\n\r\n";
        assert_eq!(ProtocolDetector::detect_from_bytes(bytes), Protocol::Http);
    }

    #[test]
    fn test_detect_empty() {
        assert_eq!(ProtocolDetector::detect_from_bytes(&[]), Protocol::Unknown);
    }

    #[test]
    fn test_protocol_priority() {
        assert!(Protocol::Tarpc.priority() > Protocol::JsonRpc.priority());
        assert!(Protocol::JsonRpc.priority() > Protocol::Http.priority());
        assert!(Protocol::Http.priority() > Protocol::Unknown.priority());
    }

    #[test]
    fn test_router_config_supported() {
        let config = RouterConfig::default();
        assert!(config.is_supported(Protocol::Tarpc));
        assert!(config.is_supported(Protocol::JsonRpc));
        assert!(config.is_supported(Protocol::Http));
        assert!(!config.is_supported(Protocol::Unknown));
    }

    #[test]
    fn test_router_config_tarpc_only() {
        let config = RouterConfig::tarpc_only();
        assert!(config.is_supported(Protocol::Tarpc));
        assert!(!config.is_supported(Protocol::JsonRpc));
        assert!(!config.is_supported(Protocol::Http));
    }

    #[test]
    fn test_capabilities_from_config() {
        let config = RouterConfig::default();
        let caps = ProtocolCapabilities::from_config(&config);

        assert!(caps.supported.contains(&"tarpc".to_string()));
        assert!(caps.supported.contains(&"json-rpc".to_string()));
        assert!(caps.high_performance.contains(&"tarpc".to_string()));
        assert_eq!(caps.recommended, "tarpc");
    }

    #[test]
    fn test_detect_json_without_jsonrpc_field() {
        // Plain JSON that's not explicitly JSON-RPC should still be treated as JSON-RPC
        let bytes = br#"{"method": "crypto.sign", "params": {}}"#;
        assert_eq!(
            ProtocolDetector::detect_from_bytes(bytes),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(format!("{}", Protocol::Tarpc), "tarpc");
        assert_eq!(format!("{}", Protocol::JsonRpc), "json-rpc");
        assert_eq!(format!("{}", Protocol::Http), "http");
    }

    #[tokio::test]
    async fn test_prefixed_stream_new() {
        use tokio::net::{TcpListener, TcpStream};
        // Ephemeral loopback — test-only
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind ephemeral TCP for prefixed stream test");
        let addr = listener
            .local_addr()
            .expect("ephemeral listener local_addr");
        let (client_stream, _) = tokio::join!(TcpStream::connect(addr), listener.accept());
        let client_stream = client_stream.expect("TCP connect in prefixed stream test");
        let prefixed = PrefixedStream::new(vec![1, 2, 3], client_stream);
        assert!(!prefixed.prefix_exhausted());
    }

    #[tokio::test]
    async fn test_prefixed_stream_prefix_exhausted_empty() {
        use tokio::net::{TcpListener, TcpStream};
        // Ephemeral loopback — test-only
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind ephemeral TCP for empty-prefix test");
        let addr = listener
            .local_addr()
            .expect("ephemeral listener local_addr");
        let (client_stream, _) = tokio::join!(TcpStream::connect(addr), listener.accept());
        let client_stream = client_stream.expect("TCP connect for empty-prefix test");
        let prefixed = PrefixedStream::new(vec![], client_stream);
        assert!(prefixed.prefix_exhausted());
    }

    #[tokio::test]
    async fn test_prefixed_stream_read_from_prefix() {
        use tokio::io::AsyncReadExt;
        use tokio::net::TcpListener;
        // Ephemeral loopback — test-only
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind ephemeral TCP for prefix read test");
        let addr = listener
            .local_addr()
            .expect("ephemeral listener local_addr");
        let (client_stream, _) = tokio::join!(TcpStream::connect(addr), listener.accept());
        let client_stream = client_stream.expect("TCP connect for prefix read test");
        let prefix = vec![0x01, 0x02, 0x03];
        let mut prefixed = PrefixedStream::new(prefix, client_stream);
        let mut buf = [0u8; 3];
        let n = prefixed
            .read(&mut buf)
            .await
            .expect("read prefix bytes from PrefixedStream");
        assert_eq!(n, 3);
        assert_eq!(buf, [0x01, 0x02, 0x03]);
        assert!(prefixed.prefix_exhausted());
    }

    #[test]
    fn test_protocol_detector_with_peek_size() {
        let d = ProtocolDetector::with_peek_size(64);
        assert!(format!("{d:?}").contains("64"));
    }

    #[test]
    fn test_detect_tarpc_ascii_graphic_fifth_byte() {
        let bytes = vec![0x10, 0x00, 0x00, 0x00, b'{'];
        let protocol = ProtocolDetector::detect_from_bytes(&bytes);
        assert_ne!(protocol, Protocol::Tarpc);
    }

    #[test]
    fn test_protocol_unknown_display_name_and_high_performance() {
        assert_eq!(format!("{}", Protocol::Unknown), "unknown");
        assert_eq!(Protocol::Unknown.name(), "unknown");
        assert_eq!(Protocol::Unknown.priority(), 0);
        assert!(!Protocol::JsonRpc.is_high_performance());
        assert!(Protocol::Tarpc.is_high_performance());
    }
}
