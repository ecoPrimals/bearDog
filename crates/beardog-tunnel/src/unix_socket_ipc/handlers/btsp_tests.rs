// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use base64::Engine;

#[tokio::test]
async fn test_btsp_handler_methods() {
    let handler = BtspHandler::new();
    let methods = handler.methods();

    // - 6 core operations x 4 aliases each = 24
    // - 4 unified methods (configure_tls, verify_peer, trust.seed, tunnel_send_http) = 4
    // - 5 server methods (create_session, verify, export_keys, negotiate, status) = 5
    // - 1 Phase 3 method (btsp.negotiate) = 1
    // - 3 legacy session aliases (session.create, session.verify, session.negotiate) = 3
    // - 1 Tower Atomic method (enrollment.verify) = 1
    // Total = 38 methods
    assert_eq!(methods.len(), 38);

    // Semantic domain.operation names (primary in registry)
    assert!(methods.contains(&"btsp.contact.exchange"));
    assert!(methods.contains(&"btsp.tunnel.establish"));
    assert!(methods.contains(&"btsp.tunnel.encrypt"));
    assert!(methods.contains(&"btsp.tunnel.decrypt"));
    assert!(methods.contains(&"btsp.tunnel.status"));
    assert!(methods.contains(&"btsp.tunnel.close"));

    // Check core operations have their aliases
    assert!(methods.contains(&"btsp.contact_exchange"));
    assert!(methods.contains(&"btsp.tunnel_establish"));
    assert!(methods.contains(&"btsp.tunnel_encrypt"));
    assert!(methods.contains(&"btsp.tunnel_decrypt"));
    assert!(methods.contains(&"btsp.tunnel_status"));
    assert!(methods.contains(&"btsp.tunnel_close"));

    // Check new unified methods
    assert!(methods.contains(&"btsp.configure_tls"));
    assert!(methods.contains(&"btsp.verify_peer"));
    assert!(methods.contains(&"btsp.tunnel_send_http"));

    // Phase 3 encrypted channel negotiation
    assert!(methods.contains(&"btsp.negotiate"));

    // Check BTSP session methods (handshake-as-a-service)
    assert!(methods.contains(&"btsp.session.create"));
    assert!(methods.contains(&"btsp.session.verify"));
    assert!(methods.contains(&"btsp.session.negotiate"));

    // Tower Atomic enrollment
    assert!(methods.contains(&"enrollment.verify"));
}

// Note: Full integration tests require a working BTSP provider
// These would be added as integration tests in the tunnel crate

#[tokio::test]
async fn btsp_handle_unknown_method() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.no_such_method", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Unknown BTSP method"));
}

#[tokio::test]
async fn btsp_contact_exchange_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.contact_exchange", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Missing params") || err.contains("Missing"));
}

#[tokio::test]
async fn btsp_tunnel_establish_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.tunnel_establish", None, &provider)
        .await
        .unwrap_err();
    assert!(!err.is_empty());
}

#[tokio::test]
async fn btsp_configure_tls_returns_external_mode_guidance() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "t1",
        "server_name": "example.com",
    });
    let err = handler
        .handle("btsp.configure_tls", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("external mode") || err.contains("not implemented in BearDog"));
}

#[tokio::test]
async fn btsp_routes_namespaced_contact_exchange_alias() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "target_peer_id": "peer-1",
        "requester_lineage": "L1",
    });
    let err = handler
        .handle("beardog./btsp/contact/exchange", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(
        !err.is_empty(),
        "expected routing to contact_exchange error path: {err}"
    );
}

#[tokio::test]
async fn btsp_semantic_dot_contact_exchange_routes_same_as_underscore() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err_dot = handler
        .handle("btsp.contact.exchange", None, &provider)
        .await
        .unwrap_err();
    let err_us = handler
        .handle("btsp.contact_exchange", None, &provider)
        .await
        .unwrap_err();
    assert_eq!(err_dot, err_us);
}

#[tokio::test]
async fn btsp_tunnel_encrypt_invalid_base64() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let tunnel = serde_json::json!({
        "id": "t1",
        "peer_id": "p",
        "established_at": "2020-01-01T00:00:00Z",
    });
    let params = serde_json::json!({
        "tunnel": tunnel,
        "data": "not!!!valid_b64",
    });
    let err = handler
        .handle("btsp.tunnel_encrypt", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("base64") || err.contains("Invalid"), "{err}");
}

#[tokio::test]
async fn btsp_tunnel_establish_external_routes_to_external_mode_message() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "peer_id": "api.example.com",
        "peer_endpoint": "tcp://api.example.com:443",
        "trust_mode": {
            "type": "certificate",
            "server_name": "api.example.com",
            "verify_chain": true,
            "root_ca_bundle": "mozilla",
            "allow_self_signed": false
        },
        "protocol": {
            "type": "tls_http",
            "tls_version": "1.3",
            "http_version": "2",
            "alpn_protocols": ["h2"]
        }
    });
    let err = handler
        .handle("btsp.tunnel_establish", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("external") || err.contains("not implemented in BearDog"));
}

#[tokio::test]
async fn btsp_tunnel_establish_mixed_mode_rejected() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "peer_id": "p",
        "peer_endpoint": "unix:///tmp/x.sock",
        "trust_mode": {
            "type": "genetic_lineage",
            "verify_ancestry": true
        },
        "protocol": {
            "type": "tls_http",
            "tls_version": "1.3",
            "http_version": "2",
            "alpn_protocols": ["h2"]
        }
    });
    let err = handler
        .handle("btsp.tunnel_establish", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Invalid mode") || err.contains("mismatch"));
}

#[tokio::test]
async fn btsp_verify_peer_tunnel_not_found() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "no-such-tunnel",
        "trust_mode": "genetic_lineage",
    });
    let err = handler
        .handle("btsp.verify_peer", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("not found") || err.contains("Tunnel"));
}

#[tokio::test]
async fn btsp_verify_peer_certificate_mode_returns_external_mode_guidance() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "t1",
        "trust_mode": "certificate",
    });
    let err = handler
        .handle("btsp.verify_peer", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("certificate") || err.contains("external mode"));
}

#[tokio::test]
async fn btsp_verify_peer_unknown_trust_mode() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "t1",
        "trust_mode": "unknown_mode_xyz",
    });
    let err = handler
        .handle("btsp.verify_peer", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Unknown trust_mode"));
}

#[tokio::test]
async fn btsp_tunnel_status_by_tunnel_id_only() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "ghost-tunnel",
    });
    let err = handler
        .handle("btsp.tunnel_status", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(!err.is_empty());
}

#[tokio::test]
async fn btsp_verify_peer_genetic_lineage_uses_tunnel_and_trust_db() {
    use crate::tunnel::hsm::SoftwareHsmConfig;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider};
    use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
    use std::sync::Arc;

    let mut hsm = HsmManager::new();
    let software_hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("software hsm");
    hsm.register_hsm_provider(
        crate::tunnel::hsm::types::HsmTier::Software,
        Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
            software_hsm,
        )),
    )
    .expect("register");
    let hsm = Arc::new(hsm);
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("provider"),
    );

    let handle = provider
        .establish_tunnel(PeerEndpoint {
            id: "peer-verify-test".to_string(),
            endpoint: "unix:///tmp/btsp-verify.sock".to_string(),
            public_key: Some(vec![9u8; 32]),
        })
        .await
        .expect("establish");

    let handler = BtspHandler::new();
    let params = serde_json::json!({
        "tunnel_id": handle.id,
        "trust_mode": "genetic_lineage",
    });
    let v = handler
        .handle("btsp.verify_peer", Some(&params), &provider)
        .await
        .expect("verify result");
    assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(false));
    assert_eq!(
        v.get("peer_id").and_then(|x| x.as_str()),
        Some("peer-verify-test")
    );
}

#[tokio::test]
async fn btsp_contact_exchange_uses_peer_id_alias() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "peer_id": "peer-alias",
        "requester_lineage": "L1",
    });
    let err = handler
        .handle("btsp.contact_exchange", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(!err.is_empty());
}

#[tokio::test]
async fn btsp_tunnel_decrypt_missing_tunnel() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "data": base64::engine::general_purpose::STANDARD.encode(b"x"),
    });
    let err = handler
        .handle("btsp.tunnel_decrypt", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("tunnel") || err.contains("Missing"));
}

#[tokio::test]
async fn btsp_tunnel_close_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.tunnel_close", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Missing") || err.contains("params"));
}

#[tokio::test]
async fn btsp_configure_tls_invalid_json() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle(
            "btsp.configure_tls",
            Some(&serde_json::json!({})),
            &provider,
        )
        .await
        .unwrap_err();
    assert!(err.contains("Invalid") || err.contains("configure_tls"));
}

#[tokio::test]
async fn btsp_tunnel_send_http_valid_shape_returns_external_mode_guidance() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel_id": "550e8400-e29b-41d4-a716-446655440000",
        "method": "GET",
        "path": "/v1/x",
        "headers": {},
    });
    let err = handler
        .handle("btsp.tunnel_send_http", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("external") || err.contains("not implemented in BearDog"));
}

#[tokio::test]
async fn btsp_routes_namespaced_tunnel_encrypt_contains_path() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let tunnel = serde_json::json!({
        "id": "t1",
        "peer_id": "p",
        "established_at": "2020-01-01T00:00:00Z",
    });
    let params = serde_json::json!({
        "tunnel": tunnel,
        "data": base64::engine::general_purpose::STANDARD.encode(b"hi"),
    });
    let res = handler
        .handle("beardog./btsp/tunnel/encrypt", Some(&params), &provider)
        .await;
    match &res {
        Ok(_) => {}
        Err(e) => {
            assert!(!e.contains("Unknown BTSP method"), "routing failed: {e}");
        }
    }
}

#[tokio::test]
async fn btsp_contact_exchange_missing_target_peer() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "requester_lineage": "L1",
    });
    let err = handler
        .handle("btsp.contact_exchange", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("target_peer") || err.contains("peer"));
}

#[tokio::test]
async fn btsp_contact_exchange_missing_lineage() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "target_peer_id": "p1",
    });
    let err = handler
        .handle("btsp.contact_exchange", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("lineage") || err.contains("requester"));
}

#[tokio::test]
async fn btsp_tunnel_encrypt_missing_data() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let tunnel = serde_json::json!({
        "id": "t1",
        "peer_id": "p",
        "established_at": "2020-01-01T00:00:00Z",
    });
    let params = serde_json::json!({ "tunnel": tunnel });
    let err = handler
        .handle("btsp.tunnel_encrypt", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("data") || err.contains("Missing"));
}

#[tokio::test]
async fn btsp_tunnel_status_missing_tunnel_and_id() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle(
            "btsp.tunnel_status",
            Some(&serde_json::json!({})),
            &provider,
        )
        .await
        .unwrap_err();
    assert!(err.contains("tunnel") || err.contains("Missing"));
}

#[tokio::test]
async fn btsp_tunnel_close_invalid_tunnel_json() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel": "not-an-object",
    });
    let err = handler
        .handle("btsp.tunnel_close", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(!err.is_empty());
}

#[tokio::test]
async fn btsp_configure_tls_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.configure_tls", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Missing") || err.contains("params"));
}

#[tokio::test]
async fn btsp_verify_peer_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.verify_peer", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Missing") || err.contains("params"));
}

#[tokio::test]
async fn btsp_tunnel_send_http_missing_params() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("btsp.tunnel_send_http", None, &provider)
        .await
        .unwrap_err();
    assert!(err.contains("Missing") || err.contains("params"));
}

#[tokio::test]
async fn btsp_tunnel_decrypt_invalid_tunnel_handle() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "tunnel": { "bad": "shape" },
        "data": base64::engine::general_purpose::STANDARD.encode(b"x"),
    });
    let err = handler
        .handle("btsp.tunnel_decrypt", Some(&params), &provider)
        .await
        .unwrap_err();
    assert!(err.contains("tunnel") || err.contains("Invalid") || err.contains("deserialize"));
}

#[tokio::test]
async fn btsp_routes_tunnel_establish_legacy_namespaced() {
    let handler = BtspHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let params = serde_json::json!({
        "id": "peer-x",
        "endpoint": "unix:///tmp/x.sock",
        "public_key": null,
    });
    let res = handler
        .handle("beardog./btsp/tunnel/establish", Some(&params), &provider)
        .await;
    let v = res.expect("namespaced tunnel establish");
    assert!(v.get("id").is_some());
    assert!(v.get("peer_id").is_some());
}

#[tokio::test]
async fn btsp_tunnel_establish_unified_internal_returns_response_shape() {
    use crate::tunnel::hsm::SoftwareHsmConfig;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
    use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
    use std::sync::Arc;

    let mut hsm = HsmManager::new();
    let software_hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("software hsm");
    hsm.register_hsm_provider(
        crate::tunnel::hsm::types::HsmTier::Software,
        Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
            software_hsm,
        )),
    )
    .expect("register");
    let hsm = Arc::new(hsm);
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("provider"),
    );

    let handler = BtspHandler::new();
    let params = serde_json::json!({
        "peer_id": "peer-unified-internal",
        "peer_endpoint": "unix:///tmp/btsp-unified-internal.sock",
        "trust_mode": { "type": "genetic_lineage", "verify_ancestry": true },
        "protocol": { "type": "btsp_native", "version": "2.0", "features": [] },
    });
    let v = handler
        .handle("btsp.tunnel_establish", Some(&params), &provider)
        .await
        .expect("unified internal establish");
    assert_eq!(v.get("mode").and_then(|x| x.as_str()), Some("internal"));
    assert_eq!(
        v.get("protocol").and_then(|x| x.as_str()),
        Some("btsp_native")
    );
    assert!(v.get("tunnel_id").is_some());
}

#[tokio::test]
async fn btsp_tunnel_encrypt_decrypt_status_close_roundtrip() {
    use crate::tunnel::hsm::SoftwareHsmConfig;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider};
    use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
    use std::sync::Arc;

    let mut hsm = HsmManager::new();
    let software_hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("software hsm");
    hsm.register_hsm_provider(
        crate::tunnel::hsm::types::HsmTier::Software,
        Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
            software_hsm,
        )),
    )
    .expect("register");
    let hsm = Arc::new(hsm);
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("provider"),
    );

    let handle = provider
        .establish_tunnel(PeerEndpoint {
            id: "peer-roundtrip".to_string(),
            endpoint: "unix:///tmp/btsp-roundtrip.sock".to_string(),
            public_key: Some(vec![7u8; 32]),
        })
        .await
        .expect("establish");

    let handler = BtspHandler::new();
    let tunnel_json = serde_json::to_value(&handle).expect("tunnel json");
    let plain_b64 = base64::engine::general_purpose::STANDARD.encode(b"hello-tunnel");
    let enc_params = serde_json::json!({
        "tunnel": tunnel_json,
        "data": plain_b64,
    });
    let enc = handler
        .handle("btsp.tunnel.encrypt", Some(&enc_params), &provider)
        .await
        .expect("tunnel encrypt");
    let cipher_b64 = enc
        .get("ciphertext")
        .and_then(|x| x.as_str())
        .expect("ciphertext b64");

    let dec_params = serde_json::json!({
        "tunnel": tunnel_json,
        "data": cipher_b64,
    });
    let dec = handler
        .handle("btsp.tunnel.decrypt", Some(&dec_params), &provider)
        .await
        .expect("tunnel decrypt");
    let out_b64 = dec
        .get("plaintext")
        .and_then(|x| x.as_str())
        .expect("plaintext b64");
    let round = base64::engine::general_purpose::STANDARD
        .decode(out_b64)
        .expect("decode roundtrip plaintext");
    assert_eq!(round, b"hello-tunnel");

    let st = handler
        .handle(
            "btsp.tunnel.status",
            Some(&serde_json::json!({ "tunnel": tunnel_json })),
            &provider,
        )
        .await
        .expect("tunnel status");
    assert!(st.get("tunnel_id").is_some() || st.get("active").is_some());

    let close = handler
        .handle(
            "btsp.tunnel.close",
            Some(&serde_json::json!({ "tunnel_id": handle.id })),
            &provider,
        )
        .await
        .expect("tunnel close");
    assert_eq!(close.get("success").and_then(|x| x.as_bool()), Some(true));
}
