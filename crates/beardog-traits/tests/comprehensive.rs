// SPDX-License-Identifier: AGPL-3.0-only
//! Integration tests targeting line coverage across `beardog-traits` (traits, DTOs, helpers).

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
use beardog_traits::unified::core::{
    Configurable, HealthMonitored, Identifiable, Lifecycle, MetricsCollector, Serializable,
    Validatable, ValidationResult, ValidationUtils, Versionable,
};
use beardog_traits::unified::genetics::{
    AuthorizationResult, BiomeGeneticsData, EntropyClass, EvolutionEngineImpl,
    EvolutionEngineUtils, EvolutionStats, GeneticParameters, GeneticsProviderImpl,
    GeneticsProviderUtils, create_evolution_engine, create_genetics_provider,
};
use beardog_traits::unified::hsm_multi_credential::{
    CredentialHierarchy, CredentialIdConverter, CredentialInfo, CredentialNode,
    CredentialReplicationData, CredentialRequest, HsmProtocol, MultiCredentialCapabilities,
    MultiCredentialHsmProvider, PermissionMapper,
};
use beardog_traits::unified::identity::{
    ExtendedIdentity, IdentityInfo, IdentityValidation, IdentityWithLineage,
};
use beardog_traits::unified::monitoring::Observable;
use beardog_traits::unified::providers::BearDogProvider;
use beardog_traits::unified::security::{
    AuditQuery, AuditStats, AuthCredentials, AuthenticationResult, PolicyContext, SecureSession,
    SecurityAuditEvent, SecurityResult, SecuritySeverity, UnifiedSecurityConfig,
};
use beardog_traits::unified::workflow::{WorkflowContext, WorkflowStatus, WorkflowStep};
use beardog_traits::unified::{
    BearDogComponent, BearDogCore, BearDogService, ComponentHealth, ServiceMetrics, ServiceStatus,
    UnifiedTraitError,
};
use beardog_types::canonical::config::r#trait::BearDogConfig;
use beardog_types::canonical::hsm::KeyMetadata;
use beardog_types::canonical::providers_unified::traits::{
    CustomMetric, HealthStatus as TraitHealthStatus, NetworkIoMetrics, ProviderCapability,
    ProviderHealth, ProviderMetrics as UnifiedProviderMetrics, ResourceUsage, SystemMetrics,
};
use beardog_types::canonical::providers_unified::{
    CanonicalProviderConfig as ProviderConfig, ProviderStatus,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

// ---------------------------------------------------------------------------
// Env mutation guard (serializes tests that touch process environment)
// ---------------------------------------------------------------------------

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

// ---------------------------------------------------------------------------
// unified/mod.rs — core component traits & DTOs
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct CoreComponent {
    id: String,
}

impl BearDogCore for CoreComponent {
    type Error = UnifiedTraitError;

    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> &str {
        "2.0.0"
    }

    fn initialize(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ComponentHealth, Self::Error>> + Send {
        async {
            Ok(ComponentHealth {
                is_healthy: true,
                status: "OK".to_string(),
                last_check: std::time::SystemTime::UNIX_EPOCH,
                details: HashMap::new(),
            })
        }
    }
}

impl BearDogComponent for CoreComponent {
    type Parameters = ();

    fn execute(
        &self,
        _params: Self::Parameters,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }
}

#[tokio::test]
async fn bear_dog_core_default_name_and_metadata() {
    let c = CoreComponent {
        id: "entity-001".to_string(),
    };
    assert_eq!(c.name(), c.id());
    assert!(c.metadata().is_empty());
}

#[tokio::test]
async fn bear_dog_component_default_dependencies_and_stateless() {
    let c = CoreComponent {
        id: "x".to_string(),
    };
    assert!(c.dependencies().is_empty());
    assert!(c.is_stateless());
}

#[derive(Debug)]
struct ServiceThing {
    id: String,
    cfg: UnifiedSecurityConfig,
    running: bool,
}

impl BearDogCore for ServiceThing {
    type Error = UnifiedTraitError;

    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn initialize(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ComponentHealth, Self::Error>> + Send {
        async {
            Ok(ComponentHealth {
                is_healthy: true,
                status: "up".to_string(),
                last_check: std::time::SystemTime::now(),
                details: HashMap::new(),
            })
        }
    }
}

impl BearDogService for ServiceThing {
    type ServiceConfig = UnifiedSecurityConfig;

    fn start(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            self.running = true;
            Ok(())
        }
    }

    fn stop(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            self.running = false;
            Ok(())
        }
    }

    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ServiceStatus, Self::Error>> + Send {
        async move { Ok(ServiceStatus::Running) }
    }

    fn get_config(&self) -> &Self::ServiceConfig {
        &self.cfg
    }

    fn update_config(
        &mut self,
        config: Self::ServiceConfig,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            self.cfg = config;
            Ok(())
        }
    }

    fn get_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ServiceMetrics, Self::Error>> + Send {
        async move {
            Ok(ServiceMetrics {
                uptime_seconds: 0,
                requests_processed: 0,
                errors_encountered: 0,
                memory_usage_bytes: 0,
                cpu_usage_percent: 0.0,
                custom_metrics: HashMap::new(),
            })
        }
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

#[tokio::test]
async fn bear_dog_service_restart_default_runs_stop_then_start() {
    let mut s = ServiceThing {
        id: "svc".to_string(),
        cfg: UnifiedSecurityConfig::default(),
        running: true,
    };
    s.restart().await.unwrap();
    assert!(s.is_running());
}

#[test]
fn service_status_and_component_health_roundtrip_json() {
    let statuses = [
        ServiceStatus::Starting,
        ServiceStatus::Running,
        ServiceStatus::Stopping,
        ServiceStatus::Stopped,
        ServiceStatus::Error("e".to_string()),
        ServiceStatus::Maintenance,
    ];
    for st in statuses {
        let j = serde_json::to_string(&st).unwrap();
        let back: ServiceStatus = serde_json::from_str(&j).unwrap();
        assert_eq!(st, back);
    }
    let h = ComponentHealth {
        is_healthy: false,
        status: "".to_string(),
        last_check: std::time::SystemTime::UNIX_EPOCH,
        details: HashMap::from([("k".into(), "v".into())]),
    };
    let j = serde_json::to_string(&h).unwrap();
    let back: ComponentHealth = serde_json::from_str(&j).unwrap();
    assert_eq!(h, back);
}

#[test]
fn unified_trait_error_display_and_into_bear_dog_error() {
    let cases = vec![
        UnifiedTraitError::Configuration {
            message: "m".into(),
        },
        UnifiedTraitError::Validation {
            field: "f".into(),
            message: "bad".into(),
        },
        UnifiedTraitError::NotSupported {
            operation: "op".into(),
        },
        UnifiedTraitError::NotFound {
            resource: "r".into(),
        },
        UnifiedTraitError::PermissionDenied {
            action: "a".into(),
            resource: "r".into(),
        },
        UnifiedTraitError::Internal {
            message: "i".into(),
        },
    ];
    for e in cases {
        let s = e.to_string();
        assert!(!s.is_empty());
        let bde: BearDogError = e.clone().into();
        let _ = format!("{bde:?}");
    }
}

// ---------------------------------------------------------------------------
// unified/core.rs — identity, validation, lifecycle defaults
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct IdEnt {
    id: String,
    ety: String,
}

impl Identifiable for IdEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

#[test]
fn identifiable_defaults() {
    let e = IdEnt {
        id: "abc".into(),
        ety: "t".into(),
    };
    assert_eq!(e.name(), e.id());
    assert!(e.metadata().is_empty());
}

#[derive(Debug)]
struct VerEnt {
    id: String,
    ety: String,
    ver: String,
}

impl Identifiable for VerEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl Versionable for VerEnt {
    fn version(&self) -> &str {
        &self.ver
    }
}

#[test]
fn versionable_defaults() {
    let v = VerEnt {
        id: "id".into(),
        ety: "e".into(),
        ver: "1".into(),
    };
    assert_eq!(v.schema_version(), 1);
    assert!(v.is_compatible_with(1));
    assert!(!v.is_compatible_with(2));
}

#[derive(Debug)]
struct CfgEnt {
    id: String,
    ety: String,
    cfg: UnifiedSecurityConfig,
}

impl Identifiable for CfgEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl Configurable for CfgEnt {
    type Config = UnifiedSecurityConfig;

    fn configure(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async move {
            self.cfg = config;
            Ok(())
        }
    }

    fn get_config(&self) -> &Self::Config {
        &self.cfg
    }

    fn validate_config(
        &self,
        _config: &Self::Config,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn reload_config(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }
}

impl Validatable for CfgEnt {
    fn validate_deep(
        &self,
    ) -> impl std::future::Future<Output = Result<ValidationResult, BearDogError>> + Send {
        async {
            Ok(ValidationResult {
                valid: true,
                errors: vec![],
                warnings: vec![],
            })
        }
    }
}

#[tokio::test]
async fn validatable_batch_validate_default_warns() {
    let e = CfgEnt {
        id: "c".into(),
        ety: "t".into(),
        cfg: UnifiedSecurityConfig::default(),
    };
    let r = CfgEnt::batch_validate(vec![e.get_config().clone(), e.get_config().clone()])
        .await
        .unwrap();
    assert!(r.valid);
    assert_eq!(r.warnings.len(), 2);
    assert!(
        r.warnings
            .iter()
            .all(|w| w.contains("Batch validation not fully implemented"))
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct SerEnt {
    id: String,
    ety: String,
}

impl Identifiable for SerEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl Serializable for SerEnt {
    fn to_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self).map_err(|e| BearDogError::System {
            message: e.to_string(),
            category: beardog_errors::SystemErrorCategory::General,
        })
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        Ok(format!("id = {:?}\nentity_type = {:?}", self.id, self.ety))
    }

    fn to_binary(&self) -> Result<Vec<u8>, BearDogError> {
        Ok(self.to_json()?.into_bytes())
    }
}

#[test]
fn serializable_helpers() {
    let s = SerEnt {
        id: "i".into(),
        ety: "e".into(),
    };
    assert!(!s.to_json().unwrap().is_empty());
    assert!(!s.to_toml().unwrap().is_empty());
    assert!(!s.to_binary().unwrap().is_empty());
}

#[derive(Debug)]
struct LifeEnt {
    id: String,
    ety: String,
}

impl Identifiable for LifeEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl Lifecycle for LifeEnt {
    fn start(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }

    fn pause(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async { Ok(()) }
    }
}

#[tokio::test]
async fn lifecycle_default_stop_and_restart() {
    let mut l = LifeEnt {
        id: "l".into(),
        ety: "e".into(),
    };
    l.stop().await.unwrap();
    l.restart().await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct SimpleHealth {
    ok: bool,
}

impl HealthMonitored for LifeEnt {
    type Health = SimpleHealth;

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Health, BearDogError>> + Send {
        async { Ok(SimpleHealth { ok: true }) }
    }
}

#[tokio::test]
async fn health_monitored_default_is_ready() {
    let h = LifeEnt {
        id: "h".into(),
        ety: "e".into(),
    };
    assert!(h.is_ready().await.unwrap());
}

#[derive(Debug, Serialize, Deserialize)]
struct MMetrics {
    v: u32,
}

#[derive(Debug)]
struct McEnt {
    id: String,
    ety: String,
}

impl Identifiable for McEnt {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl MetricsCollector for McEnt {
    type Metrics = MMetrics;

    fn collect_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Metrics, BearDogError>> + Send {
        async { Ok(MMetrics { v: 1 }) }
    }

    fn performance_score(
        &self,
    ) -> impl std::future::Future<Output = Result<f64, BearDogError>> + Send {
        async { Ok(1.0) }
    }
}

#[tokio::test]
async fn metrics_collector_smoke() {
    let m = McEnt {
        id: "m".into(),
        ety: "e".into(),
    };
    assert_eq!(m.collect_metrics().await.unwrap().v, 1);
    assert_eq!(m.performance_score().await.unwrap(), 1.0);
}

#[test]
fn validation_utils_exhaustive() {
    assert!(ValidationUtils::validate_id("abc").is_ok());
    assert!(ValidationUtils::validate_id("ab").is_err());
    assert!(ValidationUtils::validate_id("").is_err());
    assert!(ValidationUtils::validate_id("a\0b").is_err());
    assert!(ValidationUtils::validate_id(&"a".repeat(256)).is_err());
    assert!(ValidationUtils::validate_id("bad space").is_err());
    assert!(ValidationUtils::validate_id("bad.dot").is_err());
    assert!(ValidationUtils::validate_id("ok-id_1").is_ok());
    assert!(ValidationUtils::validate_id("ab\tc").is_err());
}

// ---------------------------------------------------------------------------
// unified/identity.rs — extended traits
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct ExtId {
    id: String,
    ety: String,
}

impl Identifiable for ExtId {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl ExtendedIdentity for ExtId {}

#[test]
fn extended_identity_defaults() {
    let e = ExtId {
        id: "e".into(),
        ety: "t".into(),
    };
    assert!(e.created_at().is_none());
    assert!(e.updated_at().is_none());
    assert!(e.tags().is_empty());
    assert!(e.extended_metadata().is_empty());
}

#[derive(Debug)]
struct LinId {
    id: String,
    ety: String,
}

impl Identifiable for LinId {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.ety
    }
}

impl IdentityWithLineage for LinId {}

#[test]
fn identity_with_lineage_defaults() {
    let l = LinId {
        id: "l".into(),
        ety: "t".into(),
    };
    assert!(l.parent_ids().is_empty());
    assert_eq!(l.generation(), 0);
    assert!(l.lineage_hash().is_none());
}

#[test]
fn identity_info_serde_and_validation_struct() {
    let mut info = IdentityInfo {
        id: "id".into(),
        entity_type: "et".into(),
        name: "n".into(),
        version: "0.0.1".into(),
        created_at: None,
        updated_at: None,
        tags: vec!["t".into()],
        metadata: HashMap::new(),
    };
    let j = serde_json::to_string(&info).unwrap();
    let back: IdentityInfo = serde_json::from_str(&j).unwrap();
    assert_eq!(info, back);
    let v = IdentityValidation {
        valid: false,
        errors: vec!["e".into()],
        warnings: vec!["w".into()],
    };
    let j2 = serde_json::to_string(&v).unwrap();
    let _: IdentityValidation = serde_json::from_str(&j2).unwrap();
    let d = IdentityInfo::default();
    assert!(!d.id.is_empty());
    assert_ne!(info, d);
    info.id = d.id.clone();
}

// ---------------------------------------------------------------------------
// unified/monitoring.rs — Observable defaults
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Obs;

impl Observable for Obs {
    type Event = u32;
}

#[test]
fn observable_defaults() {
    let o = Obs;
    assert!(!o.has_subscribers());
    assert_eq!(o.subscriber_count(), 0);
}

// ---------------------------------------------------------------------------
// unified/genetics.rs
// ---------------------------------------------------------------------------

#[test]
fn genetics_dtos_and_defaults() {
    let mut gp = GeneticParameters::default();
    assert_eq!(gp.signature_length, 64);
    gp.signature_length = 16;
    let es = EvolutionStats::default();
    assert_eq!(es.generation, 0);
    let ar = AuthorizationResult::default();
    assert!(!ar.authorized);
    for ec in [
        EntropyClass::Human,
        EntropyClass::Hardware,
        EntropyClass::Environmental,
        EntropyClass::Quantum,
        EntropyClass::Hybrid,
        EntropyClass::Unknown,
    ] {
        let j = serde_json::to_string(&ec).unwrap();
        let _: EntropyClass = serde_json::from_str(&j).unwrap();
    }
    let bd = BiomeGeneticsData {
        biome_id: "b".into(),
        signature: vec![],
        trust_level: 0.0,
        health_status: "".into(),
        metadata: HashMap::new(),
    };
    serde_json::to_string(&bd).unwrap();
}

#[tokio::test]
async fn genetics_utils_and_factories() {
    let v = GeneticsProviderUtils::create_basic_provider(json!({})).unwrap();
    assert!(v.to_string().contains("basic_genetics"));
    let bh = GeneticsProviderUtils::create_biome_handler(json!({"biome_id": "x"})).unwrap();
    assert_eq!(bh.biome_id, "x");
    let bh2 = GeneticsProviderUtils::create_biome_handler(json!({})).unwrap();
    assert_eq!(bh2.biome_id, "default");
    let warns = GeneticsProviderUtils::validate_genetic_params(&GeneticParameters {
        signature_length: 16,
        ..Default::default()
    })
    .await
    .unwrap();
    assert!(!warns.is_empty());
    let eng = EvolutionEngineUtils::create_basic_engine(json!({})).unwrap();
    assert!(eng.to_string().contains("evolution"));
    let biome = EvolutionEngineUtils::create_biome_data("z".into()).unwrap();
    assert_eq!(biome.biome_id, "z");
    assert!(matches!(
        create_genetics_provider("basic", json!({})),
        Ok(GeneticsProviderImpl::Basic)
    ));
    assert!(create_genetics_provider("nope", json!({})).is_err());
    assert!(matches!(
        create_evolution_engine("basic", json!({})),
        Ok(EvolutionEngineImpl::Basic)
    ));
    assert!(create_evolution_engine("other", json!({})).is_err());
}

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
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(s: &str) -> Result<Vec<u8>, BearDogError> {
    if s.len() % 2 != 0 {
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
        details: HashMap::new(),
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
        events_by_type: HashMap::new(),
        events_by_severity: HashMap::new(),
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
        environment: HashMap::new(),
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
        metadata: HashMap::new(),
    };
    serde_json::to_string(&ar).unwrap();
    let ss = SecureSession {
        session_id: "s".into(),
        user_id: "u".into(),
        created_at: Utc::now(),
        expires_at: Utc::now(),
        permissions: vec![],
        metadata: HashMap::new(),
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
    let _g = env_lock();
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
    let _g = env_lock();
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
        details: HashMap::new(),
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
            disk_io: HashMap::new(),
        },
        last_error: None,
    }
}

fn sample_trait_provider_metrics() -> UnifiedProviderMetrics {
    UnifiedProviderMetrics {
        timestamp: SystemTime::UNIX_EPOCH,
        performance: HashMap::new(),
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

    fn provider_id(&self) -> &str {
        "mini"
    }

    fn provider_version(&self) -> &str {
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
    fn provider_id(&self) -> &str {
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
        let id: String = (0..len)
            .map(|i| ['a', 'b', '-', '_'][(i % 4) as usize])
            .collect();
        assert!(ValidationUtils::validate_id(&id).is_ok());
    }
}
