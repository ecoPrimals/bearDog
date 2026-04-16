// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::primal_self_knowledge::{PrimalIdentity, PrimalIdentityEnvInputs};
use beardog_types::canonical::discovery::PerformanceProfile;
#[derive(Debug)]
struct MockDiscoveryService {
    mock_primals: Vec<UniversalServiceDescriptor>,
}

impl PrimalDiscoveryService for MockDiscoveryService {
    async fn discover_by_capability(
        &self,
        _capability: UniversalCapabilityType,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        Ok(self.mock_primals.clone())
    }

    async fn send_request(
        &self,
        _service: &UniversalServiceDescriptor,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({}))
    }
}

#[tokio::test]
async fn test_messenger_creation() {
    let mock_service = Arc::new(MockDiscoveryService {
        mock_primals: vec![],
    });

    let result = SecureCrossPrimalMessenger::new(mock_service);
    assert!(result.is_ok());

    let messenger = result.unwrap();
    let expected = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::from_env())
        .expect("identity")
        .name;
    assert_eq!(messenger.our_identity, expected);
}

#[tokio::test]
async fn test_no_hardcoded_primal_names() {
    let mock_service = Arc::new(MockDiscoveryService {
        mock_primals: vec![],
    });

    let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();

    let expected = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::from_env())
        .expect("identity")
        .name;
    assert_eq!(messenger.our_identity, expected);

    let result = messenger
        .send_to_network_primal(b"test", HashMap::new())
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("No network-capable primals discovered")
    );
}

#[tokio::test]
async fn test_metrics_tracking() {
    let mock_service = Arc::new(MockDiscoveryService {
        mock_primals: vec![],
    });

    let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();

    let metrics = messenger.get_metrics().await;
    assert_eq!(metrics.messages_sent, 0);
    assert_eq!(metrics.sessions_established, 0);
}

fn sample_network_descriptor(id: &str) -> UniversalServiceDescriptor {
    use beardog_types::canonical::discovery::{
        AuthenticationMethod, NetworkFunction, ServiceEndpoint, UniversalCapabilityType,
    };
    UniversalServiceDescriptor {
        service_id: id.to_string(),
        capabilities: vec![UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        }],
        endpoint: ServiceEndpoint {
            protocol: "http".to_string(),
            host: "peer.local".to_string(),
            port: 80,
            path: None,
            parameters: std::collections::HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.9,
    }
}

fn sample_security_descriptor(id: &str) -> UniversalServiceDescriptor {
    use beardog_types::canonical::discovery::{
        AuthenticationMethod, SecurityService, UniversalCapabilityType,
    };
    UniversalServiceDescriptor {
        service_id: id.to_string(),
        capabilities: vec![UniversalCapabilityType::Security {
            services: vec![SecurityService::KeyManagement],
        }],
        endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
            protocol: "https".to_string(),
            host: "sec.local".to_string(),
            port: 443,
            path: None,
            parameters: std::collections::HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.95,
    }
}

#[tokio::test]
async fn send_to_network_primal_deserialize_error_is_reported() {
    #[derive(Debug)]
    struct BadJsonNetwork(MockDiscoveryService);

    impl PrimalDiscoveryService for BadJsonNetwork {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.0.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({"not": "SecurePrimalResponse"}))
        }
    }

    let inner = MockDiscoveryService {
        mock_primals: vec![sample_network_descriptor("net-1")],
    };
    let svc = Arc::new(BadJsonNetwork(inner));
    let messenger = SecureCrossPrimalMessenger::new(svc).unwrap();
    let err = messenger
        .send_to_network_primal(b"hi", HashMap::new())
        .await
        .expect_err("bad json");
    assert!(
        err.to_string().contains("deserialize")
            || err.to_string().contains("Failed to deserialize"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn send_to_network_primal_round_trip_updates_metrics() {
    use beardog_types::canonical::discovery::{NetworkFunction, UniversalCapabilityType};

    #[derive(Debug)]
    struct RoundTrip(Arc<MockDiscoveryService>);

    impl PrimalDiscoveryService for RoundTrip {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.0.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            service: &UniversalServiceDescriptor,
            payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            if payload.get("type").and_then(|v| v.as_str()) == Some("key_exchange_request") {
                let peer_sk =
                    x25519_dalek::EphemeralSecret::random_from_rng(chacha20poly1305::aead::OsRng);
                let peer_pk = x25519_dalek::PublicKey::from(&peer_sk);
                return Ok(serde_json::json!({
                    "peer_public_key": hex::encode(peer_pk.as_bytes()),
                }));
            }

            let expected_sender = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::from_env())
                .expect("identity")
                .name;
            let msg: SecurePrimalMessage = serde_json::from_value(payload)
                .map_err(|e| BearDogError::serialization(&format!("parse msg: {e}")))?;
            assert_eq!(msg.sender_id, expected_sender);

            let resp = SecurePrimalResponse {
                ciphertext: msg.ciphertext,
                responder_id: service.service_id.clone(),
                capability_used: UniversalCapabilityType::Network {
                    functions: vec![NetworkFunction::TrafficRouting],
                },
                processing_time_ms: 1,
            };
            serde_json::to_value(resp).map_err(|e| BearDogError::serialization(&e.to_string()))
        }
    }

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![sample_network_descriptor("net-rt")],
    });
    let svc = Arc::new(RoundTrip(inner));
    let messenger = SecureCrossPrimalMessenger::new(svc).unwrap();
    let out = messenger
        .send_to_network_primal(b"ping", HashMap::new())
        .await
        .expect("send");
    assert_eq!(out.responder_id, "net-rt");
    let m = messenger.get_metrics().await;
    assert!(m.messages_sent >= 1);
}

#[tokio::test]
async fn establish_secure_session_errors_when_peer_key_missing() {
    #[derive(Debug)]
    struct NoKey(Arc<MockDiscoveryService>);

    impl PrimalDiscoveryService for NoKey {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.0.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({}))
        }
    }

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![sample_security_descriptor("sec-1")],
    });
    let messenger = SecureCrossPrimalMessenger::new(Arc::new(NoKey(inner))).unwrap();
    let err = messenger
        .establish_secure_session("high")
        .await
        .expect_err("no peer key");
    assert!(
        err.to_string().contains("public key") || err.to_string().contains("Peer"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn establish_secure_session_errors_on_invalid_peer_key_hex() {
    #[derive(Debug)]
    struct BadHex(Arc<MockDiscoveryService>);

    impl PrimalDiscoveryService for BadHex {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.0.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({
                "peer_public_key": "not-hex!!!",
            }))
        }
    }

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![sample_security_descriptor("sec-2")],
    });
    let messenger = SecureCrossPrimalMessenger::new(Arc::new(BadHex(inner))).unwrap();
    let err = messenger
        .establish_secure_session("high")
        .await
        .expect_err("bad hex");
    assert!(
        err.to_string().contains("Invalid") || err.to_string().contains("peer"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn establish_secure_session_errors_when_peer_key_wrong_length() {
    #[derive(Debug)]
    struct ShortKey(Arc<MockDiscoveryService>);

    impl PrimalDiscoveryService for ShortKey {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.0.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({
                "peer_public_key": hex::encode([7u8; 16]),
            }))
        }
    }

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![sample_security_descriptor("sec-3")],
    });
    let messenger = SecureCrossPrimalMessenger::new(Arc::new(ShortKey(inner))).unwrap();
    let err = messenger
        .establish_secure_session("high")
        .await
        .expect_err("short key");
    assert!(
        err.to_string().contains("32 bytes") || err.to_string().contains("Peer"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn establish_secure_session_succeeds_with_valid_peer_public_key() {
    #[derive(Debug)]
    struct GoodKey {
        inner: Arc<MockDiscoveryService>,
        hex: String,
    }

    impl PrimalDiscoveryService for GoodKey {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            self.inner.discover_by_capability(capability).await
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            let hex = self.hex.clone();
            Ok(serde_json::json!({
                "peer_public_key": hex,
            }))
        }
    }

    let peer_sk = x25519_dalek::EphemeralSecret::random_from_rng(chacha20poly1305::aead::OsRng);
    let peer_pk = x25519_dalek::PublicKey::from(&peer_sk);

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![sample_security_descriptor("sec-ok")],
    });
    let messenger = SecureCrossPrimalMessenger::new(Arc::new(GoodKey {
        inner,
        hex: hex::encode(peer_pk.as_bytes()),
    }))
    .unwrap();

    let session = messenger
        .establish_secure_session("high")
        .await
        .expect("session");
    assert!(!session.session_id.is_empty());
    assert_eq!(session.peer_id, "sec-ok");

    let loaded = messenger
        .get_session(&session.session_id)
        .await
        .expect("get");
    assert_eq!(loaded.session_id, session.session_id);
}

#[tokio::test]
async fn get_session_not_found() {
    let mock_service = Arc::new(MockDiscoveryService {
        mock_primals: vec![],
    });
    let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();
    let err = messenger.get_session("nope").await.expect_err("missing");
    assert!(err.to_string().contains("Session not found"));
}

#[tokio::test]
async fn send_to_compute_and_storage_primals_happy_path() {
    use beardog_types::canonical::discovery::{
        AuthenticationMethod, ComputeAbility, ServiceEndpoint, StorageCharacteristic,
        UniversalCapabilityType,
    };

    #[derive(Debug)]
    struct Multi(Arc<MockDiscoveryService>);

    impl PrimalDiscoveryService for Multi {
        async fn discover_by_capability(
            &self,
            capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            Ok(self
                .0
                .mock_primals
                .iter()
                .filter(|d| d.capabilities.contains(&capability))
                .cloned()
                .collect())
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({ "ok": true }))
        }
    }

    let compute = UniversalServiceDescriptor {
        service_id: "cmp".to_string(),
        capabilities: vec![UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        }],
        endpoint: ServiceEndpoint {
            protocol: "grpc".to_string(),
            host: "c".to_string(),
            port: 9,
            path: None,
            parameters: std::collections::HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.8,
    };

    let storage = UniversalServiceDescriptor {
        service_id: "sto".to_string(),
        capabilities: vec![UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Encrypted],
        }],
        endpoint: ServiceEndpoint {
            protocol: "https".to_string(),
            host: "s".to_string(),
            port: 8,
            path: None,
            parameters: std::collections::HashMap::new(),
        },
        auth_method: AuthenticationMethod::None,
        performance_profile: PerformanceProfile::default(),
        trust_score: 0.85,
    };

    let inner = Arc::new(MockDiscoveryService {
        mock_primals: vec![compute.clone(), storage.clone()],
    });
    let messenger = SecureCrossPrimalMessenger::new(Arc::new(Multi(inner))).unwrap();

    let cr = messenger
        .send_to_compute_primal(serde_json::json!({ "q": 1 }))
        .await
        .expect("compute");
    assert_eq!(cr.responder_id, "cmp");

    let sr = messenger
        .send_to_storage_primal(b"blob")
        .await
        .expect("storage");
    assert_eq!(sr.responder_id, "sto");
}

#[tokio::test]
async fn send_to_compute_errors_when_empty_discovery() {
    let mock_service = Arc::new(MockDiscoveryService {
        mock_primals: vec![],
    });
    let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();
    let err = messenger
        .send_to_compute_primal(serde_json::json!({}))
        .await
        .expect_err("empty");
    assert!(err.to_string().contains("compute-capable"));
}
