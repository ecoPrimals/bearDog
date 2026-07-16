// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::manual_async_fn)]

//! Integration tests (part 2): unified HSM multi-credential, security, workflow, providers.

mod common;

use beardog_errors::BearDogError;
use beardog_traits::unified::hsm_multi_credential::{
    CredentialHierarchy, CredentialIdConverter, CredentialInfo, CredentialNode,
    CredentialReplicationData, CredentialRequest, HsmProtocol, MultiCredentialCapabilities,
    MultiCredentialHsmProvider, PermissionMapper,
};
use beardog_traits::unified::providers::BearDogProvider;
use beardog_traits::unified::security::{
    AuditQuery, AuditStats, AuthCredentials, AuthenticationResult, PolicyContext, SecureSession,
    SecurityAuditEvent, SecurityResult, SecuritySeverity, UnifiedSecurityConfig,
};
use beardog_traits::unified::workflow::{WorkflowContext, WorkflowStatus, WorkflowStep};
use beardog_types::canonical::config::r#trait::BearDogConfig;
use beardog_types::canonical::providers_unified::traits::{
    CustomMetric, HealthStatus as TraitHealthStatus, NetworkIoMetrics, ProviderCapability,
    ProviderHealth, ProviderMetrics as UnifiedProviderMetrics, ResourceUsage, SystemMetrics,
};
use chrono::Utc;
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;

// ---------------------------------------------------------------------------
// unified/hsm_multi_credential.rs — trait smoke + helpers
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct MockHsm;

impl MultiCredentialHsmProvider for MockHsm {
    type Error = std::io::Error;

    fn create_credential(
        &self,
        request: CredentialRequest,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send {
        async move {
            Ok(CredentialInfo {
                credential_id: "id".into(),
                role: request.role,
                display_name: None,
                permissions: vec![],
                public_key: vec![],
                algorithm: "ES256".into(),
                created_at: Utc::now(),
                last_used: None,
                use_count: 0,
                parent_credential_id: None,
                metadata: HashMap::new(),
                requires_user_presence: false,
                requires_user_verification: false,
            })
        }
    }

    fn list_credentials(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<CredentialInfo>, Self::Error>> + Send {
        async { Ok(vec![]) }
    }

    fn delete_credential(
        &self,
        _credential_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn get_credential_info(
        &self,
        _credential_id: &str,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send {
        async { Err(std::io::Error::new(std::io::ErrorKind::NotFound, "missing")) }
    }

    fn sign_with_credential(
        &self,
        _credential_id: &str,
        _data: &[u8],
        _require_user_presence: bool,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send {
        async { Ok(vec![1, 2, 3]) }
    }

    fn generate_hardware_entropy(
        &self,
        size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send {
        async move { Ok(vec![0u8; size]) }
    }

    fn derive_child_credential(
        &self,
        _parent_credential_id: &str,
        request: CredentialRequest,
    ) -> impl std::future::Future<Output = Result<CredentialInfo, Self::Error>> + Send {
        self.create_credential(request)
    }

    fn get_credential_hierarchy(
        &self,
    ) -> impl std::future::Future<Output = Result<CredentialHierarchy, Self::Error>> + Send {
        async { Ok(CredentialHierarchy { roots: vec![] }) }
    }

    fn get_multi_credential_capabilities(&self) -> MultiCredentialCapabilities {
        MultiCredentialCapabilities {
            max_credentials: None,
            current_credentials: 0,
            supports_hierarchical_credentials: false,
            supports_deterministic_derivation: false,
            supports_hardware_entropy: true,
            max_entropy_bytes: Some(32),
            supported_algorithms: vec![],
            supports_user_presence: false,
            supports_user_verification: false,
            supports_metadata: false,
            protocol: HsmProtocol::Proprietary("p".into()),
        }
    }

    fn prepare_credential_replication(
        &self,
        _credential_id: &str,
        _shared_entropy: &[u8],
    ) -> impl std::future::Future<Output = Result<CredentialReplicationData, Self::Error>> + Send
    {
        async {
            Ok(CredentialReplicationData {
                source_credential: CredentialInfo {
                    credential_id: "s".into(),
                    role: "r".into(),
                    display_name: None,
                    permissions: vec![],
                    public_key: vec![],
                    algorithm: "x".into(),
                    created_at: Utc::now(),
                    last_used: None,
                    use_count: 0,
                    parent_credential_id: None,
                    metadata: HashMap::new(),
                    requires_user_presence: false,
                    requires_user_verification: false,
                },
                derivation_path: vec![0],
                entropy_hash: vec![],
                target_request: CredentialRequest {
                    role: "r".into(),
                    display_name: None,
                    permissions: vec![],
                    require_user_presence: false,
                    require_user_verification: false,
                    parent_credential: None,
                    metadata: HashMap::new(),
                    algorithm: None,
                },
            })
        }
    }
}

#[derive(Debug)]
struct IdConv;

fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn hex_decode(s: &str) -> Result<Vec<u8>, BearDogError> {
    if !s.len().is_multiple_of(2) {
        return Err(BearDogError::System {
            message: "invalid hex length".into(),
            category: beardog_errors::SystemErrorCategory::General,
        });
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in (0..s.len()).step_by(2) {
        let byte = u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| BearDogError::System {
            message: e.to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        })?;
        out.push(byte);
    }
    Ok(out)
}

impl CredentialIdConverter for IdConv {
    fn to_universal_id(&self, protocol_id: &[u8]) -> String {
        hex_encode(protocol_id)
    }

    fn from_universal_id(&self, universal_id: &str) -> Result<Vec<u8>, BearDogError> {
        hex_decode(universal_id)
    }
}

#[derive(Debug)]
struct PermMap;

impl PermissionMapper for PermMap {
    fn map_permissions(&self, permissions: &[String]) -> HashMap<String, serde_json::Value> {
        permissions
            .iter()
            .map(|p| (p.clone(), json!(true)))
            .collect()
    }

    fn unmap_permissions(&self, attributes: &HashMap<String, serde_json::Value>) -> Vec<String> {
        attributes.keys().cloned().collect()
    }
}

#[tokio::test]
async fn hsm_multi_credential_smoke() {
    let h = MockHsm;
    let req = CredentialRequest {
        role: "admin".into(),
        display_name: Some("A".into()),
        permissions: vec!["*".into()],
        require_user_presence: true,
        require_user_verification: false,
        parent_credential: None,
        metadata: HashMap::new(),
        algorithm: Some("ES256".into()),
    };
    let _ = h.create_credential(req.clone()).await.unwrap();
    assert!(h.list_credentials().await.unwrap().is_empty());
    let _ = h.delete_credential("x").await;
    assert!(h.get_credential_info("missing").await.is_err());
    assert_eq!(
        h.sign_with_credential("c", b"data", true).await.unwrap(),
        vec![1, 2, 3]
    );
    assert_eq!(h.generate_hardware_entropy(0).await.unwrap().len(), 0);
    let _ = h.derive_child_credential("p", req).await.unwrap();
    let _ = h.get_credential_hierarchy().await.unwrap();
    let _ = h.prepare_credential_replication("c", b"e").await.unwrap();
    let caps = h.get_multi_credential_capabilities();
    assert_eq!(caps.protocol, HsmProtocol::Proprietary("p".into()));
    for p in [
        HsmProtocol::Fido2,
        HsmProtocol::Pkcs11,
        HsmProtocol::Tpm2,
        HsmProtocol::OpenPgp,
        HsmProtocol::AndroidStrongBox,
        HsmProtocol::IosSecureEnclave,
    ] {
        serde_json::to_string(&p).unwrap();
    }
    let node = CredentialNode {
        credential: CredentialInfo {
            credential_id: "c".into(),
            role: "r".into(),
            display_name: None,
            permissions: vec![],
            public_key: vec![],
            algorithm: "a".into(),
            created_at: Utc::now(),
            last_used: None,
            use_count: 0,
            parent_credential_id: None,
            metadata: HashMap::new(),
            requires_user_presence: false,
            requires_user_verification: false,
        },
        children: vec![],
    };
    serde_json::to_string(&node).unwrap();
    let conv = IdConv;
    assert_eq!(conv.to_universal_id(&[0, 255]), "00ff");
    assert_eq!(conv.from_universal_id("00ff").unwrap(), vec![0, 255]);
    let pm = PermMap;
    let m = pm.map_permissions(&["a".into(), "b".into()]);
    assert_eq!(pm.unmap_permissions(&m).len(), 2);
}

// ---------------------------------------------------------------------------
// unified/security.rs — config & DTOs
// ---------------------------------------------------------------------------

#[test]
fn unified_security_config_bear_dog_config() {
    let c = UnifiedSecurityConfig::default();
    assert_eq!(UnifiedSecurityConfig::domain(), "security");
    assert_eq!(<UnifiedSecurityConfig as BearDogConfig>::version(), 1);
    assert!(c.validate().is_ok());
    let toml = c.to_toml().unwrap();
    assert!(toml.contains("session_timeout"));
    let mut bad = c.clone();
    bad.session_timeout_minutes = 0;
    assert!(bad.validate().is_err());
    let mut bad2 = UnifiedSecurityConfig::default();
    bad2.max_failed_attempts = 0;
    assert!(bad2.validate().is_err());
    let mut bad3 = UnifiedSecurityConfig::default();
    bad3.supported_auth_methods.clear();
    assert!(bad3.validate().is_err());
    let merged = c
        .merge(&UnifiedSecurityConfig {
            require_mfa: true,
            ..Default::default()
        })
        .unwrap();
    assert!(merged.require_mfa);
}

#[test]
fn unified_security_config_merge_branches() {
    let base = UnifiedSecurityConfig::default();
    let other = UnifiedSecurityConfig {
        session_timeout_minutes: 120,
        max_failed_attempts: 10,
        lockout_duration_minutes: 30,
        supported_auth_methods: vec!["x".into()],
        crypto_algorithms: vec!["y".into()],
        ..Default::default()
    };
    let m = base.merge(&other).unwrap();
    assert_eq!(m.session_timeout_minutes, 120);
    assert_eq!(m.max_failed_attempts, 10);
    assert_eq!(m.lockout_duration_minutes, 30);
    assert_eq!(m.supported_auth_methods, vec!["x".to_string()]);
    assert_eq!(m.crypto_algorithms, vec!["y".to_string()]);
}

#[test]
fn security_dtos_serde() {
    let ev = SecurityAuditEvent {
        event_id: "1".into(),
        event_type: "t".into(),
        timestamp: Utc::now(),
        user_id: None,
        session_id: None,
        resource: None,
        action: None,
        result: SecurityResult::Denied,
        details: BTreeMap::new(),
        severity: SecuritySeverity::Critical,
    };
    serde_json::to_string(&ev).unwrap();
    for r in [
        SecurityResult::Success,
        SecurityResult::Failure,
        SecurityResult::Denied,
        SecurityResult::Error,
    ] {
        serde_json::to_string(&r).unwrap();
    }
    for s in [
        SecuritySeverity::Low,
        SecuritySeverity::Medium,
        SecuritySeverity::High,
        SecuritySeverity::Critical,
    ] {
        let mut h = std::collections::HashSet::new();
        h.insert(s.clone());
        assert_eq!(h.len(), 1);
    }
    let q = AuditQuery {
        start_time: None,
        end_time: None,
        user_id: None,
        event_type: None,
        severity: None,
        limit: Some(0),
        offset: Some(0),
    };
    serde_json::to_string(&q).unwrap();
    let st = AuditStats {
        total_events: 0,
        events_by_type: BTreeMap::new(),
        events_by_severity: BTreeMap::new(),
        success_rate: 0.0,
        average_events_per_day: 0.0,
        last_event_time: None,
    };
    serde_json::to_string(&st).unwrap();
    let pc = PolicyContext {
        user_id: "u".into(),
        session_id: None,
        resource: "r".into(),
        action: "a".into(),
        environment: BTreeMap::new(),
        timestamp: Utc::now(),
    };
    serde_json::to_string(&pc).unwrap();
    let ar = AuthenticationResult {
        success: true,
        user_id: Some("u".into()),
        session_id: None,
        permissions: vec![],
        expires_at: None,
        error_message: None,
        metadata: BTreeMap::new(),
    };
    serde_json::to_string(&ar).unwrap();
    let ss = SecureSession {
        session_id: "s".into(),
        user_id: "u".into(),
        created_at: Utc::now(),
        expires_at: Utc::now(),
        permissions: vec![],
        metadata: BTreeMap::new(),
        is_active: true,
    };
    serde_json::to_string(&ss).unwrap();
    let ac = AuthCredentials::MultiFactorAuth {
        primary: Box::new(AuthCredentials::Password {
            username: "u".into(),
            password: "p".into(),
        }),
        secondary: vec![AuthCredentials::Token { token: "t".into() }],
    };
    serde_json::to_string(&ac).unwrap();
    let cert = AuthCredentials::Certificate {
        certificate: vec![1, 2],
        private_key: vec![3, 4],
    };
    serde_json::to_string(&cert).unwrap();
    let bio = AuthCredentials::Biometric {
        user_id: "u".into(),
        biometric_data: vec![],
        biometric_type: "fingerprint".into(),
    };
    serde_json::to_string(&bio).unwrap();
}

#[test]
fn unified_security_config_from_env_no_panic() {
    let _g = common::env_lock();
    for v in [
        "BEARDOG_SECURITY_ENABLE_AUTH",
        "BEARDOG_SECURITY_SESSION_TIMEOUT",
        "BEARDOG_SECURITY_REQUIRE_MFA",
        "BEARDOG_SECURITY_HSM_ENABLED",
    ] {
        beardog_errors::process_env::remove_var(v);
    }
    let cfg = UnifiedSecurityConfig::from_env().unwrap();
    assert!(cfg.validate().is_ok());
}

#[test]
fn unified_security_config_from_env_parses_overrides() {
    let _g = common::env_lock();
    beardog_errors::process_env::set_var("BEARDOG_SECURITY_ENABLE_AUTH", "false");
    beardog_errors::process_env::set_var("BEARDOG_SECURITY_SESSION_TIMEOUT", "99");
    beardog_errors::process_env::set_var("BEARDOG_SECURITY_REQUIRE_MFA", "true");
    beardog_errors::process_env::set_var("BEARDOG_SECURITY_HSM_ENABLED", "true");
    let cfg = UnifiedSecurityConfig::from_env().unwrap();
    assert!(!cfg.enable_authentication);
    assert_eq!(cfg.session_timeout_minutes, 99);
    assert!(cfg.require_mfa);
    assert!(cfg.hsm_enabled);
    for v in [
        "BEARDOG_SECURITY_ENABLE_AUTH",
        "BEARDOG_SECURITY_SESSION_TIMEOUT",
        "BEARDOG_SECURITY_REQUIRE_MFA",
        "BEARDOG_SECURITY_HSM_ENABLED",
    ] {
        beardog_errors::process_env::remove_var(v);
    }
}

// ---------------------------------------------------------------------------
// unified/workflow.rs — WorkflowStep defaults
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Step;

impl WorkflowStep for Step {
    type StepResult = ();

    fn execute_step(
        &self,
        _context: &WorkflowContext,
    ) -> impl std::future::Future<Output = Result<Self::StepResult, BearDogError>> + Send {
        async { Ok(()) }
    }
}

#[tokio::test]
async fn workflow_step_default_rollback_and_validate() {
    let s = Step;
    let ctx = WorkflowContext::default();
    s.rollback(&ctx).await.unwrap();
    assert!(s.validate_preconditions(&ctx).await.unwrap());
}

#[test]
fn workflow_status_display_all_variants() {
    for (st, expect) in [
        (WorkflowStatus::Pending, "Pending"),
        (WorkflowStatus::Running, "Running"),
        (WorkflowStatus::Paused, "Paused"),
        (WorkflowStatus::Completed, "Completed"),
        (WorkflowStatus::Failed, "Failed"),
        (WorkflowStatus::Cancelled, "Cancelled"),
    ] {
        assert_eq!(st.to_string(), expect);
    }
}

// ---------------------------------------------------------------------------
// unified/providers.rs — BearDogProvider smoke
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct MiniProv;

fn sample_trait_provider_health() -> ProviderHealth {
    ProviderHealth {
        status: TraitHealthStatus::Healthy,
        timestamp: SystemTime::UNIX_EPOCH,
        details: BTreeMap::new(),
        resource_usage: ResourceUsage {
            cpu_percent: 0.0,
            memory_bytes: 0,
            memory_percent: 0.0,
            network_io: NetworkIoMetrics {
                bytes_sent: 0,
                bytes_received: 0,
                packets_sent: 0,
                packets_received: 0,
            },
            disk_io: BTreeMap::new(),
        },
        last_error: None,
    }
}

fn sample_trait_provider_metrics() -> UnifiedProviderMetrics {
    UnifiedProviderMetrics {
        timestamp: SystemTime::UNIX_EPOCH,
        performance: BTreeMap::new(),
        custom_metrics: Vec::<CustomMetric>::new(),
        system_metrics: SystemMetrics {
            uptime_seconds: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            active_connections: 0,
            error_rate: 0.0,
        },
    }
}

impl BearDogProvider for MiniProv {
    type Error = std::io::Error;
    type Config = ();

    fn provider_id(&self) -> &'static str {
        "mini"
    }

    fn provider_version(&self) -> &'static str {
        "0.0.1"
    }

    fn capabilities(&self) -> Vec<ProviderCapability> {
        vec![]
    }

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderHealth, Self::Error>> + Send {
        async { Ok(sample_trait_provider_health()) }
    }

    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<UnifiedProviderMetrics, Self::Error>> + Send {
        async { Ok(sample_trait_provider_metrics()) }
    }
}

#[tokio::test]
async fn mini_provider_smoke() {
    let p = MiniProv;
    assert_eq!(
        p.health_check().await.unwrap().status,
        TraitHealthStatus::Healthy
    );
    let _ = p.metrics().await.unwrap();
}
