// SPDX-License-Identifier: AGPL-3.0-or-later

//! Protocol Detection for Unix Socket IPC
//!
//! Detects which protocol a client is using based on initial bytes.
//! Supports: JSON-RPC (PRIMARY), HTTP (LEGACY)

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::unix::OwnedReadHalf;
use tracing::debug;

use super::types::Protocol;

/// Protocol detector for Unix socket connections
pub struct ProtocolDetector;

impl ProtocolDetector {
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    /// Detect protocol from initial connection bytes
    ///
    /// Reads the first line/bytes from the connection to determine protocol.
    /// This is non-destructive - returns both the protocol and the first line
    /// so it can be used by the appropriate handler.
    ///
    /// # Returns
    ///
    /// `(Protocol, first_line)` tuple where:
    /// - `Protocol`: Detected protocol type
    /// - `first_line`: The initial data read (for protocol handlers to process)
    pub async fn detect(
        reader: &mut BufReader<OwnedReadHalf>,
    ) -> std::io::Result<(Protocol, String)> {
        let mut first_line = String::new();
        reader.read_line(&mut first_line).await?;

        let protocol = Protocol::detect_from_bytes(first_line.as_bytes());

        debug!(
            "🔍 Protocol detected: {} (security level: {})",
            protocol.name(),
            protocol.security_level()
        );

        Ok((protocol, first_line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detection_logic() {
        // HTTP detection
        assert_eq!(
            Protocol::detect_from_bytes(b"GET /health HTTP/1.1\r\n"),
            Protocol::Http
        );
        assert_eq!(
            Protocol::detect_from_bytes(b"POST /api HTTP/1.1\r\n"),
            Protocol::Http
        );

        // JSON-RPC detection
        assert_eq!(
            Protocol::detect_from_bytes(b"{\"jsonrpc\":\"2.0\"}"),
            Protocol::JsonRpc
        );

        // Empty/unknown defaults to JSON-RPC
        assert_eq!(Protocol::detect_from_bytes(b""), Protocol::JsonRpc);
    }
}
