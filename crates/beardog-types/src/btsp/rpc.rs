// SPDX-License-Identifier: AGPL-3.0-only

//! RPC Parameter Types for BTSP Unified
//!
//! This module defines request/response parameters for BTSP RPC methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Transport, TrustMode, TunnelProtocol};

/// Parameters for `btsp.tunnel_establish` (extended for unified BTSP)
///
/// This method establishes a secure tunnel with either:
/// - Internal mode: Another primal using genetic lineage
/// - External mode: External server using certificate verification
///
/// # Example (Internal Mode)
///
/// ```json
/// {
///   "peer_id": "songbird-nat0",
///   "peer_endpoint": "unix:///tmp/songbird-nat0.sock"
/// }
/// ```
///
/// # Example (External Mode)
///
/// ```json
/// {
///   "peer_id": "api.anthropic.com",
///   "peer_endpoint": "tcp://api.anthropic.com:443",
///   "trust_mode": {
///     "type": "certificate",
///     "server_name": "api.anthropic.com"
///   },
///   "protocol": {
///     "type": "tls_http",
///     "tls_version": "1.3",
///     "alpn_protocols": ["h2"]
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelEstablishParams {
    /// Peer identifier (primal ID or server hostname)
    ///
    /// - Internal mode: Primal ID (e.g., "songbird-nat0")
    /// - External mode: Server hostname (e.g., "api.anthropic.com")
    pub peer_id: String,

    /// Peer endpoint (URI: unix:// or tcp://)
    ///
    /// - Internal mode: `unix:///tmp/primal.sock`
    /// - External mode: `tcp://hostname:port`
    pub peer_endpoint: String,

    /// Trust verification mode (optional, defaults to genetic_lineage)
    ///
    /// If not specified, defaults to `GeneticLineage` for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_mode: Option<TrustMode>,

    /// Communication protocol (optional, defaults to btsp_native)
    ///
    /// If not specified, defaults to `BtspNative` for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TunnelProtocol>,
}

impl TunnelEstablishParams {
    /// Get trust mode (with default)
    ///
    /// Returns the specified trust mode, or defaults to `GeneticLineage`
    /// for backward compatibility with existing internal BTSP usage.
    pub fn trust_mode(&self) -> TrustMode {
        self.trust_mode.clone().unwrap_or({
            TrustMode::GeneticLineage {
                required_family: None,
                required_generation: None,
                verify_ancestry: true,
            }
        })
    }

    /// Get protocol (with default)
    ///
    /// Returns the specified protocol, or defaults to `BtspNative`
    /// for backward compatibility with existing internal BTSP usage.
    pub fn protocol(&self) -> TunnelProtocol {
        self.protocol
            .clone()
            .unwrap_or_else(|| TunnelProtocol::BtspNative {
                version: "2.0".into(),
                features: vec![],
            })
    }

    /// Parse transport from endpoint
    ///
    /// Extracts the transport layer (Unix socket or TCP socket)
    /// from the `peer_endpoint` URI.
    pub fn transport(&self) -> Result<Transport, String> {
        Transport::from_endpoint(&self.peer_endpoint)
    }

    /// Check if this is internal mode
    pub fn is_internal(&self) -> bool {
        self.trust_mode().is_internal() && self.protocol().is_internal()
    }

    /// Check if this is external mode
    pub fn is_external(&self) -> bool {
        self.trust_mode().is_external() && self.protocol().is_external()
    }
}

/// Response from `btsp.tunnel_establish`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelEstablishResponse {
    /// Tunnel ID (UUID)
    pub tunnel_id: String,

    /// Peer ID
    pub peer_id: String,

    /// Tunnel mode ("internal" or "external")
    pub mode: String,

    /// Protocol type ("btsp_native" or "tls_http")
    pub protocol: String,

    /// Timestamp when tunnel was established (RFC 3339)
    pub established_at: String,
}

/// Parameters for `btsp.configure_tls` (new, external mode only)
///
/// Configures TLS-specific parameters for an external mode tunnel.
///
/// # Example
///
/// ```json
/// {
///   "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
///   "server_name": "api.anthropic.com",
///   "sni_enabled": true,
///   "alpn_protocols": ["h2"],
///   "min_tls_version": "1.3"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureTlsParams {
    /// Tunnel ID
    pub tunnel_id: String,

    /// Server name for SNI (Server Name Indication)
    pub server_name: String,

    /// Enable SNI (default: true)
    #[serde(default = "default_true")]
    pub sni_enabled: bool,

    /// ALPN protocols to negotiate
    #[serde(default)]
    pub alpn_protocols: Vec<String>,

    /// Minimum TLS version (default: "1.3")
    #[serde(default = "default_min_tls")]
    pub min_tls_version: String,
}

/// Parameters for `btsp.verify_peer` (new, unified trust verification)
///
/// Verifies peer trust using the appropriate method:
/// - Genetic lineage for internal mode
/// - Certificate chain for external mode
///
/// # Example (Certificate Mode)
///
/// ```json
/// {
///   "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
///   "trust_mode": "certificate",
///   "certificate_chain": ["base64_cert1", "base64_cert2"]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyPeerParams {
    /// Tunnel ID
    pub tunnel_id: String,

    /// Trust mode ("genetic_lineage" or "certificate")
    pub trust_mode: String,

    /// Certificate chain (for certificate mode, base64-encoded DER)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<Vec<String>>,
}

/// Response from `btsp.verify_peer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyPeerResponse {
    /// Whether peer is trusted
    pub valid: bool,

    /// Trust level (for genetic lineage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<String>,

    /// Certificate subject (for certificate mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_subject: Option<String>,

    /// Error message (if validation failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parameters for `btsp.tunnel_send_http` (new, external mode only)
///
/// Sends an HTTP request through an external mode tunnel.
///
/// # Example
///
/// ```json
/// {
///   "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
///   "method": "POST",
///   "path": "/v1/messages",
///   "headers": {
///     "content-type": "application/json",
///     "x-api-key": "sk-ant-..."
///   },
///   "body": "base64_encoded_json"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelSendHttpParams {
    /// Tunnel ID
    pub tunnel_id: String,

    /// HTTP method (GET, POST, PUT, DELETE, etc.)
    pub method: String,

    /// Request path (e.g., "/v1/messages")
    pub path: String,

    /// HTTP headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Request body (base64-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Response from `btsp.tunnel_send_http`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelSendHttpResponse {
    /// HTTP status code
    pub status: u16,

    /// Response headers
    pub headers: HashMap<String, String>,

    /// Response body (base64-encoded)
    pub body: String,
}

fn default_true() -> bool {
    true
}

fn default_min_tls() -> String {
    "1.3".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_establish_params_defaults() {
        let params = TunnelEstablishParams {
            peer_id: "songbird-nat0".into(),
            peer_endpoint: "unix:///tmp/songbird.sock".into(),
            trust_mode: None,
            protocol: None,
        };

        // Should default to genetic_lineage
        assert!(params.trust_mode().is_internal());
        assert!(params.is_internal());
        assert!(!params.is_external());

        // Should default to btsp_native
        assert!(params.protocol().is_internal());
    }

    #[test]
    fn test_tunnel_establish_params_external() {
        let params = TunnelEstablishParams {
            peer_id: "api.anthropic.com".into(),
            peer_endpoint: "tcp://api.anthropic.com:443".into(),
            trust_mode: Some(TrustMode::Certificate {
                server_name: "api.anthropic.com".into(),
                verify_chain: true,
                root_ca_bundle: super::super::CaBundle::Mozilla,
                allow_self_signed: false,
            }),
            protocol: Some(TunnelProtocol::TlsHttp {
                tls_version: "1.3".into(),
                http_version: "2".into(),
                alpn_protocols: vec!["h2".into()],
            }),
        };

        assert!(params.is_external());
        assert!(!params.is_internal());
        assert!(params.trust_mode().is_external());
        assert!(params.protocol().is_external());

        let transport = params.transport().expect("Failed to parse transport");
        assert!(transport.is_tcp_socket());
        assert_eq!(transport.host(), Some("api.anthropic.com"));
        assert_eq!(transport.port(), Some(443));
    }

    #[test]
    fn test_tunnel_establish_params_serialization() {
        let params = TunnelEstablishParams {
            peer_id: "test-peer".into(),
            peer_endpoint: "tcp://example.com:443".into(),
            trust_mode: Some(TrustMode::Certificate {
                server_name: "example.com".into(),
                verify_chain: true,
                root_ca_bundle: super::super::CaBundle::Mozilla,
                allow_self_signed: false,
            }),
            protocol: Some(TunnelProtocol::TlsHttp {
                tls_version: "1.3".into(),
                http_version: "2".into(),
                alpn_protocols: vec!["h2".into()],
            }),
        };

        let json = serde_json::to_string(&params).expect("Serialization failed");
        assert!(json.contains("test-peer"));
        assert!(json.contains("example.com"));

        let parsed: TunnelEstablishParams =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(parsed.peer_id, params.peer_id);
    }

    #[test]
    fn test_configure_tls_params() {
        let params = ConfigureTlsParams {
            tunnel_id: "test-tunnel".into(),
            server_name: "api.anthropic.com".into(),
            sni_enabled: true,
            alpn_protocols: vec!["h2".into()],
            min_tls_version: "1.3".into(),
        };

        let json = serde_json::to_value(&params).expect("Serialization failed");
        assert_eq!(json["server_name"], "api.anthropic.com");
        assert_eq!(json["sni_enabled"], true);
    }

    #[test]
    fn test_verify_peer_params() {
        let params = VerifyPeerParams {
            tunnel_id: "test-tunnel".into(),
            trust_mode: "certificate".into(),
            certificate_chain: Some(vec!["cert1".into(), "cert2".into()]),
        };

        let json = serde_json::to_string(&params).expect("Serialization failed");
        assert!(json.contains("certificate"));
        assert!(json.contains("cert1"));
    }

    #[test]
    fn test_tunnel_send_http_params() {
        let mut headers = HashMap::new();
        headers.insert("content-type".into(), "application/json".into());

        let params = TunnelSendHttpParams {
            tunnel_id: "test-tunnel".into(),
            method: "POST".into(),
            path: "/v1/messages".into(),
            headers,
            body: Some("base64_encoded_data".into()),
        };

        let json = serde_json::to_string(&params).expect("Serialization failed");
        assert!(json.contains("POST"));
        assert!(json.contains("/v1/messages"));
        assert!(json.contains("application/json"));
    }

    #[test]
    fn test_tunnel_send_http_response() {
        let mut headers = HashMap::new();
        headers.insert("content-type".into(), "application/json".into());

        let response = TunnelSendHttpResponse {
            status: 200,
            headers,
            body: "base64_response".into(),
        };

        let json = serde_json::to_string(&response).expect("Serialization failed");
        assert!(json.contains("200"));
        assert!(json.contains("base64_response"));
    }

    #[test]
    fn test_backward_compatibility() {
        // Old-style BTSP call (no trust_mode or protocol specified)
        let json = r#"{
            "peer_id": "songbird-nat0",
            "peer_endpoint": "unix:///tmp/songbird.sock"
        }"#;

        let params: TunnelEstablishParams =
            serde_json::from_str(json).expect("Deserialization failed");

        // Should default to internal mode
        assert!(params.is_internal());
        assert!(params.trust_mode().is_internal());
        assert!(params.protocol().is_internal());
    }
}
