// SPDX-License-Identifier: AGPL-3.0-or-later

//! # BTSP Unified - Secure Protocol Provider Types
//!
//! **BearDog Tunnel Security Protocol (BTSP) - Unified Architecture**
//!
//! This module provides type definitions for BTSP Unified, which supports
//! both internal (primal-to-primal) and external (HTTPS API) secure communication
//! through a single, consistent API.
//!
//! ## Architecture Overview
//!
//! BTSP Unified consolidates two communication patterns:
//! - **Internal Mode**: Genetic lineage verification (primal-to-primal)
//! - **External Mode**: Certificate verification (external HTTPS)
//!
//! The key insight: **trust mode** is the fundamental difference, not the protocol itself!
//!
//! ## Core Types
//!
//! ### Trust Modes
//!
//! - `TrustMode::GeneticLineage`: Verify cryptographic family trees (internal)
//! - `TrustMode::Certificate`: Verify X.509 certificate chains (external)
//!
//! ### Protocols
//!
//! - `TunnelProtocol::BtspNative`: Custom protocol for internal primals
//! - `TunnelProtocol::TlsHttp`: Standard TLS 1.3 + HTTP/2 for external APIs
//!
//! ### Transports
//!
//! - `Transport::UnixSocket`: Unix domain sockets (local primals)
//! - `Transport::TcpSocket`: TCP sockets (remote servers)
//!
//! ## Example: Internal Mode (Primal-to-Primal)
//!
//! ```rust
//! use beardog_types::btsp::{TunnelEstablishParams, TrustMode, TunnelProtocol};
//!
//! // Establish tunnel with another primal (existing usage unchanged!)
//! let params = TunnelEstablishParams {
//!     peer_id: "peer-registry-nat0".into(),
//!     peer_endpoint: "unix:///path/from/capability/discovery.sock".into(),
//!     trust_mode: None, // Defaults to GeneticLineage
//!     protocol: None,   // Defaults to BtspNative
//! };
//!
//! assert!(params.is_internal());
//! ```
//!
//! ## Example: External Mode (HTTPS API)
//!
//! ```rust
//! use beardog_types::btsp::{
//!     TunnelEstablishParams, TrustMode, TunnelProtocol, CaBundle
//! };
//!
//! // Establish tunnel with external API
//! let params = TunnelEstablishParams {
//!     peer_id: "api.anthropic.com".into(),
//!     peer_endpoint: "tcp://api.anthropic.com:443".into(),
//!     trust_mode: Some(TrustMode::Certificate {
//!         server_name: "api.anthropic.com".into(),
//!         verify_chain: true,
//!         root_ca_bundle: CaBundle::Mozilla,
//!         allow_self_signed: false,
//!     }),
//!     protocol: Some(TunnelProtocol::TlsHttp {
//!         tls_version: "1.3".into(),
//!         http_version: "2".into(),
//!         alpn_protocols: vec!["h2".into(), "http/1.1".into()],
//!     }),
//! };
//!
//! assert!(params.is_external());
//! ```
//!
//! ## Benefits
//!
//! - ✅ **Single Abstraction**: "Use BTSP for all secure communication"
//! - ✅ **47% Smaller API**: 9 methods (vs. 17 in two-pattern approach)
//! - ✅ **Code Reuse**: Same crypto foundation (X25519, ChaCha20, Ed25519)
//! - ✅ **Simpler Mental Model**: Trust mode is just a parameter
//! - ✅ **Backward Compatible**: Existing code works unchanged (defaults to internal)
//!
//! ## RPC Methods
//!
//! ### Core (6 methods, existing)
//!
//! - `btsp.contact_exchange` - Genetic lineage routing
//! - `btsp.tunnel_establish` - **EXTENDED** with mode parameters
//! - `btsp.tunnel_encrypt` - Encrypt data through tunnel
//! - `btsp.tunnel_decrypt` - Decrypt data from tunnel
//! - `btsp.tunnel_status` - Health check + statistics
//! - `btsp.tunnel_close` - Graceful shutdown
//!
//! ### New (3 methods, for unified support)
//!
//! - `btsp.configure_tls` - TLS-specific setup (external mode)
//! - `btsp.verify_peer` - Unified trust verification
//! - `btsp.tunnel_send_http` - HTTP request wrapper (external mode)
//!
//! **Total**: 9 public methods (down from 17 in two-pattern architecture!)

mod protocol;
mod rpc;
mod transport;
mod trust_mode;

// Re-export all public types
pub use protocol::TunnelProtocol;
pub use rpc::{
    ConfigureTlsParams, SessionCreateParams, SessionCreateResponse, SessionExportKeysParams,
    SessionExportKeysResponse, SessionNegotiateParams, SessionNegotiateResponse,
    SessionVerifyParams, SessionVerifyResponse, TunnelEstablishParams, TunnelEstablishResponse,
    TunnelSendHttpParams, TunnelSendHttpResponse, VerifyPeerParams, VerifyPeerResponse,
};
pub use transport::{Transport, TransportEndpoint};
pub use trust_mode::{CaBundle, TrustMode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_mode_workflow() {
        // Simulate internal mode (primal-to-primal)
        let params = TunnelEstablishParams {
            peer_id: "peer-nat0".into(),
            peer_endpoint: "unix:///tmp/peer.sock".into(),
            trust_mode: None, // Defaults to genetic lineage
            protocol: None,   // Defaults to btsp_native
        };

        // Verify defaults
        assert!(params.is_internal());
        assert!(params.trust_mode().is_internal());
        assert!(params.protocol().is_internal());

        // Verify transport
        let transport = params.transport().expect("Failed to parse transport");
        assert!(transport.is_unix_socket());
    }

    #[test]
    fn test_external_mode_workflow() {
        // Simulate external mode (HTTPS API)
        let params = TunnelEstablishParams {
            peer_id: "api.anthropic.com".into(),
            peer_endpoint: "tcp://api.anthropic.com:443".into(),
            trust_mode: Some(TrustMode::Certificate {
                server_name: "api.anthropic.com".into(),
                verify_chain: true,
                root_ca_bundle: CaBundle::Mozilla,
                allow_self_signed: false,
            }),
            protocol: Some(TunnelProtocol::TlsHttp {
                tls_version: "1.3".into(),
                http_version: "2".into(),
                alpn_protocols: vec!["h2".into()],
            }),
        };

        // Verify external mode
        assert!(params.is_external());
        assert!(params.trust_mode().is_external());
        assert!(params.protocol().is_external());

        // Verify transport
        let transport = params.transport().expect("Failed to parse transport");
        assert!(transport.is_tcp_socket());
        assert_eq!(transport.host(), Some("api.anthropic.com"));
        assert_eq!(transport.port(), Some(443));
    }

    #[test]
    fn test_backward_compatibility() {
        // Old-style BTSP call (minimal parameters)
        let json = r#"{
            "peer_id": "beardog-nat0",
            "peer_endpoint": "unix:///tmp/beardog.sock"
        }"#;

        let params: TunnelEstablishParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        // Should work unchanged with defaults
        assert!(params.is_internal());
        assert_eq!(params.peer_id, "beardog-nat0");
    }

    #[test]
    fn test_type_exports() {
        // Ensure all types are properly exported
        let _trust: TrustMode;
        let _protocol: TunnelProtocol;
        let _transport: Transport;
        let _bundle: CaBundle;
        let _establish_params: TunnelEstablishParams;
        let _establish_response: TunnelEstablishResponse;
        let _configure_tls: ConfigureTlsParams;
        let _verify_peer: VerifyPeerParams;
        let _verify_response: VerifyPeerResponse;
        let _http_params: TunnelSendHttpParams;
        let _http_response: TunnelSendHttpResponse;
    }
}
