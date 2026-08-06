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

    /// tarpc binary RPC (bincode over serde-transport framing)
    Tarpc,

    /// Length-prefixed binary frame (legacy detection alias for tarpc framing)
    BinaryFrame,

    /// `PROTOCOLS:` negotiation greeting (G65 Phase 3)
    Negotiation,

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
            Self::Tarpc => "tarpc",
            Self::BinaryFrame => "binary-frame",
            Self::Negotiation => "negotiation",
            Self::Http => "http",
            Self::Unknown => "unknown",
        }
    }

    /// Wire name used in `PROTOCOLS:` / `PROTOCOL:` negotiation lines.
    #[must_use]
    pub const fn wire_name(&self) -> &'static str {
        match self {
            Self::JsonRpc => "jsonrpc",
            Self::Tarpc | Self::BinaryFrame => "tarpc",
            Self::Negotiation => "negotiation",
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

        // G65 Protocol Negotiation: `PROTOCOLS: tarpc,jsonrpc\n`
        if bytes.starts_with(b"PROTOCOLS:") || bytes.starts_with(b"PROTOCOLS ") {
            return Protocol::Negotiation;
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

        // Length-prefixed binary frame detection (tarpc serde-transport uses this framing)
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

    /// Enable tarpc binary RPC (G65 Phase 3)
    pub enable_tarpc: bool,

    /// Enable HTTP handling (legacy compatibility)
    pub enable_http: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            enable_jsonrpc: true,
            enable_tarpc: true,
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
            enable_tarpc: false,
            enable_http: false,
        }
    }

    /// Get list of supported protocols
    #[must_use]
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
    #[must_use]
    pub const fn is_supported(&self, protocol: Protocol) -> bool {
        match protocol {
            Protocol::JsonRpc => self.enable_jsonrpc,
            Protocol::Tarpc => self.enable_tarpc,
            Protocol::Http => self.enable_http,
            Protocol::BinaryFrame | Protocol::Negotiation | Protocol::Unknown => false,
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

        if config.enable_tarpc {
            supported.push("tarpc".to_string());
            versions.insert("tarpc".to_string(), "0.37".to_string());
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
            recommended: if config.enable_tarpc {
                "tarpc".to_string()
            } else {
                "json-rpc".to_string()
            },
            versions,
        }
    }

    /// Create JSON representation for capability response
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// G65 Protocol Negotiation (Phase 3 — single-socket protocol selection).
///
/// Replaces the C2 dual-socket pattern (`.sock` + `.tarpc.sock`) with a
/// single-socket negotiation handshake:
///
/// ```text
/// Client → Server:  PROTOCOLS: tarpc,jsonrpc\n
/// Server → Client:  PROTOCOL: tarpc\n
/// ```
///
/// - Client sends `PROTOCOLS:` followed by comma-separated protocol names
///   in preference order (highest priority first).
/// - Server selects the best mutual match and responds with `PROTOCOL:`.
/// - No negotiation (first byte is `{` or binary frame) = legacy fallback
///   (JSON-RPC or tarpc respectively). Backward-compatible.
#[derive(Debug, Clone)]
pub struct ProtocolNegotiator {
    /// Protocols this server supports, in priority order
    server_supported: Vec<Protocol>,
}

impl ProtocolNegotiator {
    /// Create a negotiator with the default bearDog protocol preferences.
    ///
    /// Preference order: tarpc (highest throughput) > jsonrpc (universal).
    #[must_use]
    pub fn new() -> Self {
        Self {
            server_supported: vec![Protocol::Tarpc, Protocol::JsonRpc],
        }
    }

    /// Create a negotiator with custom protocol preferences.
    #[must_use]
    pub fn with_protocols(protocols: Vec<Protocol>) -> Self {
        Self {
            server_supported: protocols,
        }
    }

    /// Parse a `PROTOCOLS:` line from the client.
    ///
    /// Accepts: `PROTOCOLS: tarpc,jsonrpc\n` or `PROTOCOLS:tarpc,jsonrpc\n`
    /// Returns the client's requested protocols in preference order.
    #[must_use]
    pub fn parse_client_greeting(line: &str) -> Vec<Protocol> {
        let trimmed = line.trim();
        let payload = if let Some(rest) = trimmed.strip_prefix("PROTOCOLS:") {
            rest.trim()
        } else if let Some(rest) = trimmed.strip_prefix("PROTOCOLS ") {
            rest.trim()
        } else {
            return Vec::new();
        };

        payload
            .split(',')
            .filter_map(|name| match name.trim() {
                "tarpc" => Some(Protocol::Tarpc),
                "jsonrpc" | "json-rpc" => Some(Protocol::JsonRpc),
                "http" => Some(Protocol::Http),
                _ => None,
            })
            .collect()
    }

    /// Get the list of protocols this server supports.
    #[must_use]
    pub fn supported_protocols(&self) -> &[Protocol] {
        &self.server_supported
    }

    /// Select the best mutual protocol from client preferences.
    ///
    /// Iterates client preferences in order; the first protocol that the
    /// server also supports wins. Returns `None` if no mutual protocol exists.
    #[must_use]
    pub fn negotiate(&self, client_prefs: &[Protocol]) -> Option<Protocol> {
        client_prefs
            .iter()
            .find(|p| self.server_supported.contains(p))
            .copied()
    }

    /// Format the server response line.
    ///
    /// ```text
    /// PROTOCOL: tarpc\n
    /// ```
    #[must_use]
    pub fn format_response(selected: Protocol) -> String {
        format!("PROTOCOL: {}\n", selected.wire_name())
    }

    /// Format an error response when no mutual protocol exists.
    #[must_use]
    pub fn format_error(server_supported: &[Protocol]) -> String {
        let names: Vec<&str> = server_supported.iter().map(Protocol::wire_name).collect();
        format!("PROTOCOL: NONE supported={}\n", names.join(","))
    }
}

impl Default for ProtocolNegotiator {
    fn default() -> Self {
        Self::new()
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
    fn test_detect_negotiation_colon() {
        let bytes = b"PROTOCOLS: tarpc,jsonrpc\n";
        assert_eq!(
            ProtocolDetector::detect_from_bytes(bytes),
            Protocol::Negotiation
        );
    }

    #[test]
    fn test_detect_negotiation_space() {
        let bytes = b"PROTOCOLS tarpc\n";
        assert_eq!(
            ProtocolDetector::detect_from_bytes(bytes),
            Protocol::Negotiation
        );
    }

    #[test]
    fn test_router_config_supported() {
        let config = RouterConfig::default();
        assert!(config.is_supported(Protocol::JsonRpc));
        assert!(config.is_supported(Protocol::Tarpc));
        assert!(config.is_supported(Protocol::Http));
        assert!(!config.is_supported(Protocol::Unknown));
        assert!(!config.is_supported(Protocol::BinaryFrame));
        assert!(!config.is_supported(Protocol::Negotiation));
    }

    #[test]
    fn test_router_config_jsonrpc_only() {
        let config = RouterConfig::jsonrpc_only();
        assert!(config.is_supported(Protocol::JsonRpc));
        assert!(!config.is_supported(Protocol::Tarpc));
        assert!(!config.is_supported(Protocol::Http));
    }

    #[test]
    fn test_capabilities_from_config() {
        let config = RouterConfig::default();
        let caps = ProtocolCapabilities::from_config(&config);

        assert!(caps.supported.contains(&"json-rpc".to_string()));
        assert!(caps.supported.contains(&"tarpc".to_string()));
        assert_eq!(caps.recommended, "tarpc");
    }

    #[test]
    fn test_capabilities_jsonrpc_only_recommends_jsonrpc() {
        let config = RouterConfig::jsonrpc_only();
        let caps = ProtocolCapabilities::from_config(&config);

        assert!(caps.supported.contains(&"json-rpc".to_string()));
        assert!(!caps.supported.contains(&"tarpc".to_string()));
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
        assert_eq!(format!("{}", Protocol::Tarpc), "tarpc");
        assert_eq!(format!("{}", Protocol::Negotiation), "negotiation");
        assert_eq!(format!("{}", Protocol::Http), "http");
        assert_eq!(format!("{}", Protocol::BinaryFrame), "binary-frame");
    }

    #[test]
    fn test_protocol_wire_names() {
        assert_eq!(Protocol::JsonRpc.wire_name(), "jsonrpc");
        assert_eq!(Protocol::Tarpc.wire_name(), "tarpc");
        assert_eq!(Protocol::BinaryFrame.wire_name(), "tarpc");
        assert_eq!(Protocol::Http.wire_name(), "http");
    }

    // ── G65 Protocol Negotiation tests ──

    #[test]
    fn test_parse_client_greeting_tarpc_jsonrpc() {
        let prefs = ProtocolNegotiator::parse_client_greeting("PROTOCOLS: tarpc,jsonrpc\n");
        assert_eq!(prefs, vec![Protocol::Tarpc, Protocol::JsonRpc]);
    }

    #[test]
    fn test_parse_client_greeting_jsonrpc_only() {
        let prefs = ProtocolNegotiator::parse_client_greeting("PROTOCOLS: jsonrpc\n");
        assert_eq!(prefs, vec![Protocol::JsonRpc]);
    }

    #[test]
    fn test_parse_client_greeting_json_rpc_alias() {
        let prefs = ProtocolNegotiator::parse_client_greeting("PROTOCOLS: json-rpc\n");
        assert_eq!(prefs, vec![Protocol::JsonRpc]);
    }

    #[test]
    fn test_parse_client_greeting_unknown_ignored() {
        let prefs =
            ProtocolNegotiator::parse_client_greeting("PROTOCOLS: grpc,tarpc,quic,jsonrpc\n");
        assert_eq!(prefs, vec![Protocol::Tarpc, Protocol::JsonRpc]);
    }

    #[test]
    fn test_parse_client_greeting_empty() {
        let prefs = ProtocolNegotiator::parse_client_greeting("NOT_PROTOCOLS: tarpc\n");
        assert!(prefs.is_empty());
    }

    #[test]
    fn test_negotiate_mutual_tarpc_preferred() {
        let neg = ProtocolNegotiator::new();
        let client = vec![Protocol::Tarpc, Protocol::JsonRpc];
        assert_eq!(neg.negotiate(&client), Some(Protocol::Tarpc));
    }

    #[test]
    fn test_negotiate_mutual_jsonrpc_fallback() {
        let neg = ProtocolNegotiator::new();
        let client = vec![Protocol::JsonRpc];
        assert_eq!(neg.negotiate(&client), Some(Protocol::JsonRpc));
    }

    #[test]
    fn test_negotiate_no_mutual() {
        let neg = ProtocolNegotiator::with_protocols(vec![Protocol::Tarpc]);
        let client = vec![Protocol::Http];
        assert_eq!(neg.negotiate(&client), None);
    }

    #[test]
    fn test_negotiate_client_priority_honored() {
        let neg = ProtocolNegotiator::new();
        let client = vec![Protocol::JsonRpc, Protocol::Tarpc];
        assert_eq!(
            neg.negotiate(&client),
            Some(Protocol::JsonRpc),
            "client prefers jsonrpc"
        );
    }

    #[test]
    fn test_format_response() {
        assert_eq!(
            ProtocolNegotiator::format_response(Protocol::Tarpc),
            "PROTOCOL: tarpc\n"
        );
        assert_eq!(
            ProtocolNegotiator::format_response(Protocol::JsonRpc),
            "PROTOCOL: jsonrpc\n"
        );
    }

    #[test]
    fn test_format_error() {
        let err =
            ProtocolNegotiator::format_error(&[Protocol::Tarpc, Protocol::JsonRpc]);
        assert!(err.starts_with("PROTOCOL: NONE"));
        assert!(err.contains("tarpc"));
        assert!(err.contains("jsonrpc"));
    }

    // ── PrefixedStream tests ──

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
        assert!(!Protocol::Tarpc.is_primary());
        assert!(Protocol::JsonRpc.is_primary());
    }
}
