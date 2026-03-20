// SPDX-License-Identifier: AGPL-3.0-only

//! Transport Types for BTSP Unified
//!
//! This module defines transport layers for tunnel communication:
//! - Unix Socket: For local primal communication
//! - TCP Socket: For remote server communication

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Transport layer for tunnel communication
///
/// BTSP Unified supports two transport types:
/// - **Unix Socket**: For local inter-primal communication
/// - **TCP Socket**: For remote server communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Transport {
    /// Unix domain socket (local primals)
    ///
    /// Used for high-performance local communication between primals
    /// on the same machine. Zero network overhead, secure by default.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "unix_socket",
    ///   "path": "/tmp/beardog-nat0.sock"
    /// }
    /// ```
    #[serde(rename = "unix_socket")]
    UnixSocket {
        /// Socket path
        ///
        /// Absolute path to the Unix domain socket file.
        /// Example: `/tmp/beardog-nat0.sock`
        path: PathBuf,
    },

    /// TCP socket (remote servers)
    ///
    /// Used for network communication with remote servers.
    /// Supports both IPv4 and IPv6.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "tcp_socket",
    ///   "host": "api.anthropic.com",
    ///   "port": 443
    /// }
    /// ```
    #[serde(rename = "tcp_socket")]
    TcpSocket {
        /// Hostname or IP address
        ///
        /// Can be a domain name (e.g., "api.anthropic.com"),
        /// IPv4 address (e.g., "192.168.1.1"),
        /// or IPv6 address (e.g., "::1").
        host: String,

        /// Port number
        ///
        /// Valid range: 1-65535
        /// Common ports: 443 (HTTPS), 80 (HTTP), 8080 (alternate HTTP)
        port: u16,
    },
}

impl Transport {
    /// Parse from endpoint URI
    ///
    /// Supports two URI schemes:
    /// - `unix://` - Unix domain socket
    /// - `tcp://` - TCP socket
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_types::btsp::Transport;
    ///
    /// // Unix socket
    /// let transport = Transport::from_endpoint("unix:///tmp/beardog.sock").unwrap();
    /// assert!(matches!(transport, Transport::UnixSocket { .. }));
    ///
    /// // TCP socket
    /// let transport = Transport::from_endpoint("tcp://api.example.com:443").unwrap();
    /// assert!(matches!(transport, Transport::TcpSocket { .. }));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - URI scheme is not `unix://` or `tcp://`
    /// - TCP URI doesn't have host:port format
    /// - Port number is invalid
    pub fn from_endpoint(endpoint: &str) -> Result<Self, String> {
        if let Some(path) = endpoint.strip_prefix("unix://") {
            Ok(Self::UnixSocket {
                path: PathBuf::from(path),
            })
        } else if let Some(rest) = endpoint.strip_prefix("tcp://") {
            // Find the last colon to handle IPv6 addresses (e.g., ::1:8080)
            let colon_pos = rest.rfind(':').ok_or_else(|| {
                format!("Invalid TCP endpoint format (expected host:port): {rest}")
            })?;

            let host = &rest[..colon_pos];
            let port_str = &rest[colon_pos + 1..];

            if host.is_empty() {
                return Err("TCP endpoint host cannot be empty".into());
            }

            let port = port_str
                .parse::<u16>()
                .map_err(|e| format!("Invalid port number '{port_str}': {e}"))?;

            Ok(Self::TcpSocket {
                host: host.to_string(),
                port,
            })
        } else {
            Err(format!(
                "Unknown endpoint scheme (expected unix:// or tcp://): {endpoint}"
            ))
        }
    }

    /// Check if this is a Unix socket transport
    pub const fn is_unix_socket(&self) -> bool {
        matches!(self, Self::UnixSocket { .. })
    }

    /// Check if this is a TCP socket transport
    pub const fn is_tcp_socket(&self) -> bool {
        matches!(self, Self::TcpSocket { .. })
    }

    /// Get the socket path (for Unix sockets)
    pub const fn socket_path(&self) -> Option<&PathBuf> {
        match self {
            Self::UnixSocket { path } => Some(path),
            Self::TcpSocket { .. } => None,
        }
    }

    /// Get the host (for TCP sockets)
    pub fn host(&self) -> Option<&str> {
        match self {
            Self::TcpSocket { host, .. } => Some(host),
            Self::UnixSocket { .. } => None,
        }
    }

    /// Get the port (for TCP sockets)
    pub const fn port(&self) -> Option<u16> {
        match self {
            Self::TcpSocket { port, .. } => Some(*port),
            Self::UnixSocket { .. } => None,
        }
    }

    /// Convert to endpoint URI string
    pub fn to_endpoint(&self) -> String {
        match self {
            Self::UnixSocket { path } => {
                format!("unix://{}", path.display())
            }
            Self::TcpSocket { host, port } => {
                format!("tcp://{host}:{port}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unix_endpoint() {
        let transport =
            Transport::from_endpoint("unix:///tmp/beardog.sock").expect("Failed to parse");

        match &transport {
            Transport::UnixSocket { path } => {
                assert_eq!(path, &PathBuf::from("/tmp/beardog.sock"));
            }
            _ => panic!("Expected UnixSocket"),
        }

        assert!(transport.is_unix_socket());
        assert!(!transport.is_tcp_socket());
        assert_eq!(
            transport.socket_path(),
            Some(&PathBuf::from("/tmp/beardog.sock"))
        );
        assert_eq!(transport.host(), None);
        assert_eq!(transport.port(), None);
    }

    #[test]
    fn test_parse_tcp_endpoint() {
        let transport =
            Transport::from_endpoint("tcp://api.anthropic.com:443").expect("Failed to parse");

        match &transport {
            Transport::TcpSocket { host, port } => {
                assert_eq!(host, "api.anthropic.com");
                assert_eq!(*port, 443);
            }
            _ => panic!("Expected TcpSocket"),
        }

        assert!(!transport.is_unix_socket());
        assert!(transport.is_tcp_socket());
        assert_eq!(transport.host(), Some("api.anthropic.com"));
        assert_eq!(transport.port(), Some(443));
        assert_eq!(transport.socket_path(), None);
    }

    #[test]
    fn test_parse_tcp_ipv4() {
        let transport =
            Transport::from_endpoint("tcp://192.168.1.1:8080").expect("Failed to parse");

        if let Transport::TcpSocket { ref host, port } = transport {
            assert_eq!(host, "192.168.1.1");
            assert_eq!(port, 8080);
        } else {
            panic!("Expected TcpSocket");
        }
    }

    #[test]
    fn test_parse_tcp_ipv6() {
        let transport = Transport::from_endpoint("tcp://::1:8080").expect("Failed to parse");

        if let Transport::TcpSocket { ref host, port } = transport {
            assert_eq!(host, "::1");
            assert_eq!(port, 8080);
        } else {
            panic!("Expected TcpSocket");
        }
    }

    #[test]
    fn test_invalid_scheme() {
        let result = Transport::from_endpoint("http://example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown endpoint scheme"));
    }

    #[test]
    fn test_invalid_tcp_format() {
        // Missing port
        let result = Transport::from_endpoint("tcp://example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid TCP endpoint format"));

        // Invalid port number
        let result = Transport::from_endpoint("tcp://example.com:invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid port number"));

        // Port out of range
        let result = Transport::from_endpoint("tcp://example.com:999999");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid port number"));
    }

    #[test]
    fn test_empty_host() {
        let result = Transport::from_endpoint("tcp://:443");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("host cannot be empty"));
    }

    #[test]
    fn test_transport_serialization() {
        // Unix socket
        let unix = Transport::UnixSocket {
            path: PathBuf::from("/tmp/test.sock"),
        };
        let json = serde_json::to_string(&unix).expect("Serialization failed");
        assert!(json.contains("unix_socket"));
        let parsed: Transport = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(unix, parsed);

        // TCP socket
        let tcp = Transport::TcpSocket {
            host: "example.com".into(),
            port: 443,
        };
        let json = serde_json::to_string(&tcp).expect("Serialization failed");
        assert!(json.contains("tcp_socket"));
        let parsed: Transport = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(tcp, parsed);
    }

    #[test]
    fn test_to_endpoint() {
        let unix = Transport::UnixSocket {
            path: PathBuf::from("/tmp/beardog.sock"),
        };
        assert_eq!(unix.to_endpoint(), "unix:///tmp/beardog.sock");

        let tcp = Transport::TcpSocket {
            host: "api.example.com".into(),
            port: 443,
        };
        assert_eq!(tcp.to_endpoint(), "tcp://api.example.com:443");
    }

    #[test]
    fn test_roundtrip() {
        let endpoints = vec![
            "unix:///tmp/beardog.sock",
            "tcp://api.anthropic.com:443",
            "tcp://192.168.1.1:8080",
            "tcp://localhost:3000",
        ];

        for endpoint in endpoints {
            let transport = Transport::from_endpoint(endpoint).expect("Failed to parse");
            let reconstructed = transport.to_endpoint();
            let parsed_again =
                Transport::from_endpoint(&reconstructed).expect("Failed to re-parse");
            assert_eq!(transport, parsed_again, "Roundtrip failed for {}", endpoint);
        }
    }
}
