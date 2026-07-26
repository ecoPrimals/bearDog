// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Protocol Router for `BearDog` IPC
//!
//! **AUTOMATIC PROTOCOL DETECTION AND ROUTING** (v2.0.0)
//!
//! Detects incoming connection protocol and routes to appropriate handler:
//! - **JSON-RPC 2.0**: Primary protocol — flexible, human-readable, ecosystem standard
//! - **Binary frame**: Length-prefixed binary (detected for future evolution)
//! - **HTTP**: Legacy compatibility
//!
//! ## Protocol Detection Strategy
//! Uses first-bytes sniffing to identify protocol:
//! - JSON-RPC: `{` character (JSON object start)
//! - Binary: Length-prefixed binary frames (reserved for future zero-copy transport)
//! - HTTP: `GET`, `POST`, `PUT`, `DELETE`, etc.
//!
//! ## Architecture
//! JSON-RPC 2.0 over NDJSON is the primary inter-primal protocol.
//! All capability negotiation, method dispatch, and ecosystem communication
//! flows through JSON-RPC. Binary framing is detected but not actively
//! dispatched — reserved for future zero-copy `bytes::Bytes` evolution.

use std::io;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::debug;

/// Detected protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// JSON-RPC 2.0 (primary inter-primal protocol)
    JsonRpc,

    /// Length-prefixed binary frame (reserved for future zero-copy transport)
    BinaryFrame,

    /// HTTP/1.1 (legacy compatibility)
    Http,

    /// Unknown/unrecognized protocol
    Unknown,
}

impl Protocol {
    /// Human-readable name
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::JsonRpc => "json-rpc",
            Self::BinaryFrame => "binary-frame",
            Self::Http => "http",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this is the primary ecosystem protocol
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        matches!(self, Self::JsonRpc)
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
    #[must_use]
    pub const fn new() -> Self {
        Self { peek_size: 16 }
    }

    /// Create with custom peek size
    #[must_use]
    pub const fn with_peek_size(peek_size: usize) -> Self {
        Self { peek_size }
    }

    /// Detect protocol from first bytes (non-consuming peek)
    ///
    /// Returns (`detected_protocol`, `peeked_bytes`) so caller can prepend
    /// peeked bytes back to the stream if needed.
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] when reading from `stream` fails.
    pub async fn detect(&self, stream: &mut TcpStream) -> io::Result<(Protocol, Vec<u8>)> {
        let mut buf = vec![0u8; self.peek_size];

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
    #[must_use]
    pub fn detect_from_bytes(bytes: &[u8]) -> Protocol {
        if bytes.is_empty() {
            return Protocol::Unknown;
        }

        // JSON-RPC: starts with '{' or whitespace then '{'
        let mut trimmed = bytes
            .iter()
            .skip_while(|&&b| b == b' ' || b == b'\t' || b == b'\n' || b == b'\r');
        if let Some(&first_char) = trimmed.next()
            && first_char == b'{'
        {
            return Protocol::JsonRpc;
        }

        // HTTP methods
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

        if bytes.starts_with(b"HTTP/") {
            return Protocol::Http;
        }

        // Length-prefixed binary frame detection (reserved for future zero-copy transport)
        if bytes.len() >= 4 {
            let frame_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
            if frame_len > 0
                && frame_len < 16 * 1024 * 1024
                && bytes.len() > 4
                && !bytes[4].is_ascii_graphic()
            {
                return Protocol::BinaryFrame;
            }
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
    /// Enable JSON-RPC handling (primary protocol)
    pub enable_jsonrpc: bool,

    /// Enable HTTP handling (legacy compatibility)
    pub enable_http: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            enable_jsonrpc: true,
            enable_http: true,
        }
    }
}

impl RouterConfig {
    /// Create config with only JSON-RPC enabled (recommended for inter-primal IPC)
    #[must_use]
    pub const fn jsonrpc_only() -> Self {
        Self {
            enable_jsonrpc: true,
            enable_http: false,
        }
    }

    /// Get list of supported protocols
    #[must_use]
    pub fn supported_protocols(&self) -> Vec<Protocol> {
        let mut protocols = Vec::new();

        if self.enable_jsonrpc {
            protocols.push(Protocol::JsonRpc);
        }
        if self.enable_http {
            protocols.push(Protocol::Http);
        }

        protocols
    }

    /// Check if a protocol is supported
    #[must_use]
    pub const fn is_supported(&self, protocol: Protocol) -> bool {
        match protocol {
            Protocol::JsonRpc => self.enable_jsonrpc,
            Protocol::Http => self.enable_http,
            Protocol::BinaryFrame | Protocol::Unknown => false,
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

    /// Protocol version info
    pub versions: std::collections::HashMap<String, String>,
}

impl ProtocolCapabilities {
    /// Create capabilities from router config
    #[must_use]
    pub fn from_config(config: &RouterConfig) -> Self {
        let mut supported = Vec::new();
        let mut versions = std::collections::HashMap::new();

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
            recommended: "json-rpc".to_string(),
            versions,
        }
    }

    /// Create JSON representation for capability response
    #[must_use]
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
    pub const fn prefix_exhausted(&self) -> bool {
        self.prefix_pos >= self.prefix.len()
    }
}

impl tokio::io::AsyncRead for PrefixedStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        if self.prefix_pos < self.prefix.len() {
            let remaining = &self.prefix[self.prefix_pos..];
            let to_copy = std::cmp::min(remaining.len(), buf.remaining());
            buf.put_slice(&remaining[..to_copy]);
            self.prefix_pos += to_copy;
            return std::task::Poll::Ready(Ok(()));
        }

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
    fn test_router_config_supported() {
        let config = RouterConfig::default();
        assert!(config.is_supported(Protocol::JsonRpc));
        assert!(config.is_supported(Protocol::Http));
        assert!(!config.is_supported(Protocol::Unknown));
        assert!(!config.is_supported(Protocol::BinaryFrame));
    }

    #[test]
    fn test_router_config_jsonrpc_only() {
        let config = RouterConfig::jsonrpc_only();
        assert!(config.is_supported(Protocol::JsonRpc));
        assert!(!config.is_supported(Protocol::Http));
    }

    #[test]
    fn test_capabilities_from_config() {
        let config = RouterConfig::default();
        let caps = ProtocolCapabilities::from_config(&config);

        assert!(caps.supported.contains(&"json-rpc".to_string()));
        assert_eq!(caps.recommended, "json-rpc");
    }

    #[test]
    fn test_detect_json_without_jsonrpc_field() {
        let bytes = br#"{"method": "crypto.sign", "params": {}}"#;
        assert_eq!(
            ProtocolDetector::detect_from_bytes(bytes),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(format!("{}", Protocol::JsonRpc), "json-rpc");
        assert_eq!(format!("{}", Protocol::Http), "http");
        assert_eq!(format!("{}", Protocol::BinaryFrame), "binary-frame");
    }

    #[tokio::test]
    async fn test_prefixed_stream_new() {
        use tokio::net::{TcpListener, TcpStream};
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
    fn test_detect_binary_frame() {
        let bytes = vec![0x10, 0x00, 0x00, 0x00, 0x01];
        assert_eq!(
            ProtocolDetector::detect_from_bytes(&bytes),
            Protocol::BinaryFrame
        );
    }

    #[test]
    fn test_binary_frame_with_ascii_fifth_byte_not_detected() {
        let bytes = vec![0x10, 0x00, 0x00, 0x00, b'{'];
        assert_ne!(
            ProtocolDetector::detect_from_bytes(&bytes),
            Protocol::BinaryFrame
        );
    }

    #[test]
    fn test_protocol_properties() {
        assert_eq!(Protocol::Unknown.name(), "unknown");
        assert!(!Protocol::Http.is_primary());
        assert!(Protocol::JsonRpc.is_primary());
    }
}
