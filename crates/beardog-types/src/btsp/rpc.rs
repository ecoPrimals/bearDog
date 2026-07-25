// SPDX-License-Identifier: AGPL-3.0-or-later

//! RPC Parameter Types for BTSP Unified
//!
//! This module defines request/response parameters for BTSP RPC methods.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
///   "peer_id": "peer-instance-id",
///   "peer_endpoint": "unix:///path/from/discovery.sock"
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
    /// - Internal mode: Opaque primal / instance id from discovery (not a product name)
    /// - External mode: Server hostname (e.g., "api.anthropic.com")
    pub peer_id: String,

    /// Peer endpoint (URI: unix:// or tcp://)
    ///
    /// - Internal mode: `unix:///...` from capability / registry discovery
    /// - External mode: `tcp://hostname:port`
    pub peer_endpoint: String,

    /// Trust verification mode (optional, defaults to `genetic_lineage`)
    ///
    /// If not specified, defaults to `GeneticLineage` for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_mode: Option<TrustMode>,

    /// Communication protocol (optional, defaults to `btsp_native`)
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
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint URI cannot be parsed as a transport.
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

    /// Protocol type ("`btsp_native`" or "`tls_http`")
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

    /// Trust mode ("`genetic_lineage`" or "certificate")
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
    pub headers: BTreeMap<String, String>,

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
    pub headers: BTreeMap<String, String>,

    /// Response body (base64-encoded)
    pub body: String,
}

// ── BTSP Server Methods (handshake-as-a-service for other primals) ────

/// Parameters for `btsp.server.create_session`
///
/// Creates a server-side BTSP session context: generates an ephemeral keypair,
/// derives the handshake key from the provided family seed, and returns the
/// server's public key + a random challenge for the client to prove membership.
///
/// Legacy alias: `btsp.session.create` (accepted for backward compatibility).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateParams {
    /// Base64-encoded family seed (caller supplies their family's seed).
    pub family_seed: String,
}

/// Response from `btsp.server.create_session`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateResponse {
    /// Base64-encoded server ephemeral X25519 public key.
    pub server_ephemeral_pub: String,
    /// Base64-encoded random challenge (32 bytes).
    pub challenge: String,
    /// Opaque session token referencing the server-side state.
    pub session_token: String,
}

/// Parameters for `btsp.server.verify`
///
/// Verifies a client's challenge response and, if valid, derives session keys.
///
/// Legacy alias: `btsp.session.verify` (accepted for backward compatibility).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionVerifyParams {
    /// Session token returned by `btsp.server.create_session`.
    pub session_token: String,
    /// Base64-encoded client ephemeral X25519 public key.
    pub client_ephemeral_pub: String,
    /// Base64-encoded HMAC response from the client.
    pub response: String,
    /// Preferred cipher suite name (e.g. `"chacha20_poly1305"`).
    #[serde(default = "default_cipher")]
    pub preferred_cipher: String,
}

/// Response from `btsp.server.verify`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionVerifyResponse {
    /// Whether verification succeeded.
    pub verified: bool,
    /// Hex-encoded session ID (set only when `verified` is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Negotiated cipher suite name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    /// Error detail (set only on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parameters for `btsp.server.negotiate`
///
/// Negotiate (or re-negotiate) the cipher suite for an existing session.
///
/// Legacy alias: `btsp.session.negotiate` (accepted for backward compatibility).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionNegotiateParams {
    /// Session token or session ID.
    pub session_token: String,
    /// Requested cipher suite name.
    pub cipher: String,
}

/// Response from `btsp.server.negotiate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionNegotiateResponse {
    /// Whether the negotiation succeeded.
    pub accepted: bool,
    /// Negotiated cipher suite name (may differ from request).
    pub cipher: String,
}

/// Parameters for `btsp.server.export_keys`
///
/// Exports session keys for a verified BTSP session, encrypted under the
/// caller's X25519 ephemeral public key via ChaCha20-Poly1305. This
/// completes the relay path: after the relay primal
/// calls `btsp.server.verify`, it calls `export_keys` to obtain the
/// session keys needed for post-handshake stream encryption.
///
/// The keys are wrapped (encrypted) so they never appear as plaintext in
/// JSON-RPC responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionExportKeysParams {
    /// Session ID from `btsp.server.verify`.
    pub session_id: String,
    /// Base64-encoded X25519 public key of the caller. The session keys
    /// will be encrypted under a shared secret derived from this key and
    /// `BearDog`'s ephemeral key.
    pub caller_ephemeral_pub: String,
}

/// Response from `btsp.server.export_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionExportKeysResponse {
    /// Base64-encoded encrypted payload containing the two 32-byte session
    /// keys (server-to-client || client-to-server, 64 bytes total), plus a
    /// 12-byte nonce and 16-byte Poly1305 tag.
    pub wrapped_keys: String,
    /// Base64-encoded X25519 ephemeral public key that `BearDog` used for
    /// the key-wrapping DH. The caller uses this + their secret to derive
    /// the unwrapping key.
    pub wrapper_ephemeral_pub: String,
    /// Cipher used for the session (`"chacha20_poly1305"`, `"hmac_plain"`,
    /// or `"null"`).
    pub cipher: String,
}

/// Parameters for `enrollment.verify` — verify an HMAC proof from `mesh.enroll`.
///
/// songBird sends the structured enrollment fields; bearDog reconstructs the
/// HMAC message as `node_id || "|" || public_key || "|" || timestamp || "|" || seed_generation`
/// and verifies the proof against a key derived from `FAMILY_SEED` via HKDF
/// at the specified `seed_generation`.
///
/// ## Seed rotation (Wave 150x)
///
/// Enrollment keys are derived through the genetic HKDF hierarchy:
///
/// ```text
/// enrollment_key(gen) = HKDF-SHA256(
///     ikm  = FAMILY_SEED,
///     salt = FAMILY_ID (or "default"),
///     info = "enrollment-v{gen}"
/// )
/// ```
///
/// During a grace period after rotation, the verifier accepts proofs
/// keyed to the current generation **or** the previous one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentVerifyParams {
    /// The enrolling node's identifier (e.g. `"southGate"`).
    pub node_id: String,
    /// The enrolling node's public key (`WireGuard` or Ed25519, as string).
    pub public_key: String,
    /// Unix timestamp (seconds) when the enrollment was initiated.
    pub timestamp: u64,
    /// Base64-encoded HMAC-SHA256 proof over the enrollment message.
    pub proof: String,
    /// Seed generation used to derive the HMAC key (default 0 for
    /// backward compatibility with pre-rotation enrollments).
    #[serde(default)]
    pub seed_generation: u32,
}

/// Response from `enrollment.verify`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentVerifyResponse {
    /// Whether the HMAC proof is valid.
    pub verified: bool,
    /// Human-readable reason if `verified` is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The seed generation against which the proof was verified (if successful).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_generation: Option<u32>,
}

fn default_cipher() -> String {
    "chacha20_poly1305".into()
}

const fn default_true() -> bool {
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
            peer_id: "peer-nat0".into(),
            peer_endpoint: "unix:///tmp/peer.sock".into(),
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
        let mut headers = BTreeMap::new();
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
        let mut headers = BTreeMap::new();
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
            "peer_id": "peer-nat0",
            "peer_endpoint": "unix:///tmp/peer.sock"
        }"#;

        let params: TunnelEstablishParams =
            serde_json::from_str(json).expect("Deserialization failed");

        // Should default to internal mode
        assert!(params.is_internal());
        assert!(params.trust_mode().is_internal());
        assert!(params.protocol().is_internal());
    }
}
