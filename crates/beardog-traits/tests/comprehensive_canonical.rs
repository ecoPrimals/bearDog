// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::manual_async_fn)]

//! Integration tests (part 3): canonical `BaseProvider`, platform, and DTO sweep.

use beardog_errors::BearDogError;
use beardog_traits::canonical::{
    ai::{AnalysisResult, ClassificationResult, ModelConfig, ModelInfo, ModelStatus, SearchResult},
    base::{
        BaseProvider, ConnectionStatus, HealthStatus, PlatformProvider, ProviderInfo,
        ProviderMetrics, ServiceHealth,
    },
    cache::{AdvancedCacheStats, CacheStats},
    crypto::{AsymmetricAlgorithm, SignatureAlgorithm, SymmetricAlgorithm},
    database::QueryResult,
    hsm::{HsmKeyInfo, HsmKeyType, HsmTier, KeyPolicy, KeyUsageLog},
    monitoring::AlertSeverity,
    security::{ClientInfo, SecureSession as CanonicalSecureSession, SecurityEvent},
    universal::OperationResult,
    workflow::{
        WorkflowDefinition, WorkflowExecution, WorkflowInfo, WorkflowParams, WorkflowResult,
    },
};
use beardog_traits::unified::core::ValidationUtils;
use beardog_types::canonical::hsm::KeyMetadata;
use beardog_types::canonical::providers_unified::{
    CanonicalProviderConfig as ProviderConfig, ProviderStatus,
};
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// canonical/base.rs — BaseProvider default methods
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct MockBase {
    id: String,
}

impl BaseProvider for MockBase {
    fn provider_id(&self) -> &str {
        &self.id
    }

    fn initialize(
        &mut self,
        _config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send {
        async {
            Ok(HealthStatus {
                last_check: Utc::now(),
                uptime_seconds: 0,
                resource_usage: HashMap::new(),
            })
        }
    }

    fn capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send {
        async { Ok(vec![]) }
    }

    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, BearDogError>> + Send {
        async { Ok(HashMap::new()) }
    }

    fn validate_config(
        &self,
        _config: &ProviderConfig,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send {
        async { Ok(true) }
    }

    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderStatus, BearDogError>> + Send {
        async { Ok(ProviderStatus::Active) }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn reload_config(
        &self,
        _config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }
}

#[derive(Debug)]
struct MockBaseCaps;

impl BaseProvider for MockBaseCaps {
    fn provider_id(&self) -> &'static str {
        "x"
    }

    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "n".into(),
            version: "v".into(),
            description: "d".into(),
            provider_type: "t".into(),
            capabilities: vec!["cap".into()],
            metadata: HashMap::new(),
        }
    }

    fn initialize(
        &mut self,
        _config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send {
        async {
            Ok(HealthStatus {
                last_check: Utc::now(),
                uptime_seconds: 1,
                resource_usage: HashMap::new(),
            })
        }
    }

    fn capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send {
        async { Ok(vec![]) }
    }

    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, BearDogError>> + Send {
        async { Ok(HashMap::new()) }
    }

    fn validate_config(
        &self,
        _config: &ProviderConfig,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send {
        async { Ok(true) }
    }

    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderStatus, BearDogError>> + Send {
        async { Ok(ProviderStatus::Active) }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn reload_config(
        &self,
        _config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }
}

#[tokio::test]
async fn base_provider_defaults_and_supports_capability() {
    let m = MockBase { id: "p".into() };
    assert_eq!(m.get_info().name, "BearDog Provider");
    assert_eq!(m.provider_info().version, "3.0.0");
    assert_eq!(m.id(), m.provider_id());
    assert_eq!(m.connection_status(), ConnectionStatus::Connected);
    assert_eq!(m.version(), "3.0.0");
    let _ = m.service_health();
    let c = MockBaseCaps;
    assert!(c.supports_capability("cap"));
    assert!(!c.supports_capability("missing"));
}

#[derive(Debug)]
struct Plat;

impl PlatformProvider for Plat {
    fn platform_info(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send
    {
        async { Ok(HashMap::new()) }
    }

    fn initialize_platform(
        &self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn platform_capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send {
        async { Ok(vec![]) }
    }
}

#[tokio::test]
async fn platform_provider_mock_smoke() {
    let p = Plat;
    assert!(p.platform_info().await.unwrap().is_empty());
    p.initialize_platform().await.unwrap();
    assert!(p.platform_capabilities().await.unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// canonical DTO sweep (serialization + common constructors)
// ---------------------------------------------------------------------------

#[test]
fn canonical_connection_and_health_dtos() {
    for s in [
        ConnectionStatus::Connected,
        ConnectionStatus::Disconnected,
        ConnectionStatus::Connecting,
        ConnectionStatus::Error,
    ] {
        serde_json::to_string(&s).unwrap();
    }
    let sh = ServiceHealth {
        status: "ok".into(),
        uptime: 0,
        last_check: Utc::now(),
    };
    serde_json::to_string(&sh).unwrap();
}

#[test]
fn canonical_security_and_client_dtos() {
    let ev = SecurityEvent {
        event_type: "t".into(),
        timestamp: Utc::now(),
        user_id: None,
        severity: "low".into(),
        metadata: HashMap::new(),
    };
    serde_json::to_string(&ev).unwrap();
    let ci = ClientInfo {
        client_id: "c".into(),
        ip_address: "127.0.0.1".into(),
        user_agent: "ua".into(),
        platform: "linux".into(),
    };
    serde_json::to_string(&ci).unwrap();
    let ss = CanonicalSecureSession {
        session_id: "s".into(),
        created_at: Utc::now(),
        expires_at: Utc::now(),
    };
    serde_json::to_string(&ss).unwrap();
}

#[test]
fn canonical_hsm_and_crypto_enums() {
    let tiers = [
        HsmTier::Software,
        HsmTier::Tpm,
        HsmTier::Cloud,
        HsmTier::Hybrid,
        HsmTier::Hardware,
    ];
    let mut sorted = tiers;
    sorted.sort();
    assert_eq!(sorted.first().unwrap(), &HsmTier::Software);
    for kt in [
        HsmKeyType::Symmetric,
        HsmKeyType::AsymmetricPublic,
        HsmKeyType::AsymmetricPrivate,
        HsmKeyType::Signing,
        HsmKeyType::Encryption,
    ] {
        serde_json::to_string(&kt).unwrap();
    }
    let ki = HsmKeyInfo {
        key_id: "k".into(),
        key_type: HsmKeyType::Signing,
        metadata: KeyMetadata::default(),
        usage_count: 0,
    };
    serde_json::to_string(&ki).unwrap();
    let kp = KeyPolicy {
        allowed_operations: vec![],
        expiration: None,
        minimum_key_size: 0,
    };
    serde_json::to_string(&kp).unwrap();
    let kul = KeyUsageLog {
        operation: "op".into(),
        timestamp: Utc::now(),
        user_id: "u".into(),
        result: "ok".into(),
    };
    serde_json::to_string(&kul).unwrap();
    for a in [
        SymmetricAlgorithm::Aes256,
        SymmetricAlgorithm::ChaCha20,
        SymmetricAlgorithm::Aes128,
    ] {
        serde_json::to_string(&a).unwrap();
    }
    for a in [
        SignatureAlgorithm::Ed25519,
        SignatureAlgorithm::EcdsaP256,
        SignatureAlgorithm::Rsa2048,
    ] {
        serde_json::to_string(&a).unwrap();
    }
    for a in [
        AsymmetricAlgorithm::Ed25519,
        AsymmetricAlgorithm::X25519,
        AsymmetricAlgorithm::EcdsaP256,
        AsymmetricAlgorithm::Rsa2048,
    ] {
        serde_json::to_string(&a).unwrap();
    }
}

#[test]
fn canonical_workflow_ai_cache_db_monitoring_universal() {
    let wp = WorkflowParams {
        parameters: HashMap::from([("a".into(), json!(1))]),
    };
    serde_json::to_string(&wp).unwrap();
    let wr = WorkflowResult {
        result: json!({}),
        status: "ok".into(),
    };
    serde_json::to_string(&wr).unwrap();
    let wi = WorkflowInfo {
        id: "i".into(),
        name: "n".into(),
        status: "s".into(),
    };
    serde_json::to_string(&wi).unwrap();
    let we = WorkflowExecution {
        id: "i".into(),
        workflow_id: "w".into(),
        status: "s".into(),
        result: None,
    };
    serde_json::to_string(&we).unwrap();
    let wd = WorkflowDefinition {
        name: "n".into(),
        steps: vec![json!(1)],
        metadata: HashMap::new(),
    };
    serde_json::to_string(&wd).unwrap();
    let ar = AnalysisResult {
        analysis: json!({}),
        confidence: 0.5,
    };
    serde_json::to_string(&ar).unwrap();
    let mc = ModelConfig {
        model_type: "t".into(),
        parameters: HashMap::new(),
    };
    serde_json::to_string(&mc).unwrap();
    let ms = ModelStatus {
        status: "s".into(),
        accuracy: 1.0,
        last_updated: Utc::now(),
    };
    serde_json::to_string(&ms).unwrap();
    let mi = ModelInfo {
        id: "i".into(),
        name: "n".into(),
        version: "v".into(),
        capabilities: vec![],
    };
    serde_json::to_string(&mi).unwrap();
    let sr = SearchResult {
        id: "i".into(),
        content: "c".into(),
        relevance_score: 0.0,
    };
    serde_json::to_string(&sr).unwrap();
    let cr = ClassificationResult {
        category: "c".into(),
        confidence: 0.0,
        metadata: HashMap::new(),
    };
    serde_json::to_string(&cr).unwrap();
    let cs = CacheStats {
        memory_usage_bytes: 0,
        eviction_count: 0,
        hit_rate_percent: 0.0,
        average_ttl_seconds: 0.0,
        hot_keys: vec![],
    };
    serde_json::to_string(&cs).unwrap();
    let acs = AdvancedCacheStats {
        hit_rate: 0.0,
        miss_rate: 0.0,
        eviction_count: 0,
        memory_usage: 0,
    };
    serde_json::to_string(&acs).unwrap();
    let qr = QueryResult {
        rows: vec![],
        affected_rows: 0,
    };
    serde_json::to_string(&qr).unwrap();
    for sev in [
        AlertSeverity::Low,
        AlertSeverity::Medium,
        AlertSeverity::High,
        AlertSeverity::Critical,
    ] {
        serde_json::to_string(&sev).unwrap();
    }
    let op = OperationResult {
        success: true,
        result: json!({}),
        message: "m".into(),
    };
    serde_json::to_string(&op).unwrap();
}

// ---------------------------------------------------------------------------
// Property-style boundary checks (no external proptest dependency)
// ---------------------------------------------------------------------------

#[test]
fn validation_id_randomized_lengths() {
    for len in [3usize, 10, 100, 255] {
        let id: String = (0..len).map(|i| ['a', 'b', '-', '_'][i % 4]).collect();
        assert!(ValidationUtils::validate_id(&id).is_ok());
    }
}
