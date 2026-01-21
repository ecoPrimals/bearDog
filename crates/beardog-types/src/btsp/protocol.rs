//! Protocol Types for BTSP Unified
//!
//! This module defines communication protocols for tunnels:
//! - BTSP Native: For internal primal-to-primal communication
//! - TLS HTTP: For external HTTPS communication

use serde::{Deserialize, Serialize};

/// Communication protocol for tunnel
///
/// BTSP Unified supports two protocol modes:
/// - **BTSP Native**: Custom protocol for internal primals
/// - **TLS HTTP**: Standard TLS 1.3 + HTTP/2 for external APIs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TunnelProtocol {
    /// Native BTSP protocol (internal primals)
    ///
    /// Custom protocol optimized for inter-primal communication:
    /// - Genetic lineage verification
    /// - X25519 key exchange
    /// - ChaCha20-Poly1305 encryption
    /// - Long-lived secure channels
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "btsp_native",
    ///   "version": "2.0",
    ///   "features": ["compression", "multiplexing"]
    /// }
    /// ```
    #[serde(rename = "btsp_native")]
    BtspNative {
        /// Protocol version (e.g., "2.0")
        ///
        /// Defaults to "2.0" if not specified.
        #[serde(default = "default_version")]
        version: String,

        /// Optional features (compression, multiplexing, etc.)
        ///
        /// Supported features:
        /// - `"compression"`: Enable data compression
        /// - `"multiplexing"`: Multiple streams per tunnel
        /// - `"keep_alive"`: Automatic heartbeat
        #[serde(default)]
        features: Vec<String>,
    },

    /// TLS 1.3 + HTTP (external APIs)
    ///
    /// Standard protocol for external HTTPS communication:
    /// - TLS 1.3 handshake with certificate verification
    /// - HTTP/2 or HTTP/1.1
    /// - ALPN protocol negotiation
    /// - Per-request sessions
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "tls_http",
    ///   "tls_version": "1.3",
    ///   "http_version": "2",
    ///   "alpn_protocols": ["h2", "http/1.1"]
    /// }
    /// ```
    #[serde(rename = "tls_http")]
    TlsHttp {
        /// TLS version (e.g., "1.3")
        ///
        /// Only TLS 1.3 is supported for security reasons.
        /// Defaults to "1.3" if not specified.
        #[serde(default = "default_tls_version")]
        tls_version: String,

        /// HTTP version (e.g., "2", "1.1")
        ///
        /// Prefers HTTP/2 for multiplexing support.
        /// Defaults to "2" if not specified.
        #[serde(default = "default_http_version")]
        http_version: String,

        /// ALPN protocols (Application-Layer Protocol Negotiation)
        ///
        /// Specifies preferred protocols during TLS handshake:
        /// - `"h2"`: HTTP/2
        /// - `"http/1.1"`: HTTP/1.1
        ///
        /// Defaults to `["h2", "http/1.1"]` if not specified.
        #[serde(default = "default_alpn")]
        alpn_protocols: Vec<String>,
    },
}

impl TunnelProtocol {
    /// Check if this is internal mode (BTSP native)
    pub fn is_internal(&self) -> bool {
        matches!(self, TunnelProtocol::BtspNative { .. })
    }

    /// Check if this is external mode (TLS HTTP)
    pub fn is_external(&self) -> bool {
        matches!(self, TunnelProtocol::TlsHttp { .. })
    }

    /// Get the protocol version string
    pub fn version(&self) -> &str {
        match self {
            TunnelProtocol::BtspNative { version, .. } => version,
            TunnelProtocol::TlsHttp { tls_version, .. } => tls_version,
        }
    }

    /// Get HTTP version (for external mode)
    pub fn http_version(&self) -> Option<&str> {
        match self {
            TunnelProtocol::TlsHttp { http_version, .. } => Some(http_version),
            _ => None,
        }
    }

    /// Get ALPN protocols (for external mode)
    pub fn alpn_protocols(&self) -> Option<&[String]> {
        match self {
            TunnelProtocol::TlsHttp {
                alpn_protocols, ..
            } => Some(alpn_protocols),
            _ => None,
        }
    }

    /// Check if a feature is enabled (for internal mode)
    pub fn has_feature(&self, feature: &str) -> bool {
        match self {
            TunnelProtocol::BtspNative { features, .. } => features.contains(&feature.to_string()),
            _ => false,
        }
    }
}

fn default_version() -> String {
    "2.0".into()
}

fn default_tls_version() -> String {
    "1.3".into()
}

fn default_http_version() -> String {
    "2".into()
}

fn default_alpn() -> Vec<String> {
    vec!["h2".into(), "http/1.1".into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btsp_native_serialization() {
        let protocol = TunnelProtocol::BtspNative {
            version: "2.0".into(),
            features: vec!["compression".into(), "multiplexing".into()],
        };

        let json = serde_json::to_string(&protocol).expect("Serialization failed");
        assert!(json.contains("btsp_native"));
        assert!(json.contains("2.0"));
        assert!(json.contains("compression"));

        let parsed: TunnelProtocol =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(protocol, parsed);
    }

    #[test]
    fn test_btsp_native_defaults() {
        let json = r#"{"type":"btsp_native"}"#;
        let protocol: TunnelProtocol = serde_json::from_str(json).expect("Deserialization failed");

        match protocol {
            TunnelProtocol::BtspNative { version, features } => {
                assert_eq!(version, "2.0"); // Should default to "2.0"
                assert!(features.is_empty()); // Should default to empty
            }
            _ => panic!("Expected BtspNative"),
        }
    }

    #[test]
    fn test_tls_http_serialization() {
        let protocol = TunnelProtocol::TlsHttp {
            tls_version: "1.3".into(),
            http_version: "2".into(),
            alpn_protocols: vec!["h2".into()],
        };

        let json = serde_json::to_string(&protocol).expect("Serialization failed");
        assert!(json.contains("tls_http"));
        assert!(json.contains("1.3"));
        assert!(json.contains("h2"));

        let parsed: TunnelProtocol =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(protocol, parsed);
    }

    #[test]
    fn test_tls_http_defaults() {
        let json = r#"{"type":"tls_http"}"#;
        let protocol: TunnelProtocol = serde_json::from_str(json).expect("Deserialization failed");

        match protocol {
            TunnelProtocol::TlsHttp {
                tls_version,
                http_version,
                alpn_protocols,
            } => {
                assert_eq!(tls_version, "1.3"); // Should default to "1.3"
                assert_eq!(http_version, "2"); // Should default to "2"
                assert_eq!(alpn_protocols, vec!["h2", "http/1.1"]); // Should default to both
            }
            _ => panic!("Expected TlsHttp"),
        }
    }

    #[test]
    fn test_protocol_helpers() {
        let internal = TunnelProtocol::BtspNative {
            version: "2.0".into(),
            features: vec!["compression".into()],
        };

        assert!(internal.is_internal());
        assert!(!internal.is_external());
        assert_eq!(internal.version(), "2.0");
        assert!(internal.has_feature("compression"));
        assert!(!internal.has_feature("multiplexing"));
        assert_eq!(internal.http_version(), None);
        assert_eq!(internal.alpn_protocols(), None);

        let external = TunnelProtocol::TlsHttp {
            tls_version: "1.3".into(),
            http_version: "2".into(),
            alpn_protocols: vec!["h2".into()],
        };

        assert!(!external.is_internal());
        assert!(external.is_external());
        assert_eq!(external.version(), "1.3");
        assert_eq!(external.http_version(), Some("2"));
        assert_eq!(external.alpn_protocols(), Some(&vec!["h2".to_string()][..]));
        assert!(!external.has_feature("compression"));
    }

    #[test]
    fn test_tls_version_security() {
        // Ensure we default to TLS 1.3 (not 1.2 or earlier)
        let json = r#"{"type":"tls_http"}"#;
        let protocol: TunnelProtocol = serde_json::from_str(json).expect("Deserialization failed");

        if let TunnelProtocol::TlsHttp { tls_version, .. } = protocol {
            assert_eq!(
                tls_version, "1.3",
                "Should default to TLS 1.3 for security"
            );
        }
    }

    #[test]
    fn test_http2_preference() {
        // Ensure we prefer HTTP/2 by default
        let json = r#"{"type":"tls_http"}"#;
        let protocol: TunnelProtocol = serde_json::from_str(json).expect("Deserialization failed");

        if let TunnelProtocol::TlsHttp {
            http_version,
            alpn_protocols,
            ..
        } = protocol
        {
            assert_eq!(http_version, "2", "Should default to HTTP/2");
            assert_eq!(
                alpn_protocols.first().map(|s| s.as_str()),
                Some("h2"),
                "Should prefer h2 in ALPN"
            );
        }
    }
}

