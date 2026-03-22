// SPDX-License-Identifier: AGPL-3.0-only
//! Integration tests (part 1): unified `mod`, core, identity, monitoring, genetics.

use beardog_errors::BearDogError;
use beardog_traits::unified::core::{
    Configurable, HealthMonitored, Identifiable, Lifecycle, MetricsCollector, Serializable,
    Validatable, ValidationResult, ValidationUtils, Versionable,
};
use beardog_traits::unified::genetics::{
    AuthorizationResult, BiomeGeneticsData, EntropyClass, EvolutionEngineImpl,
    EvolutionEngineUtils, EvolutionStats, GeneticParameters, GeneticsProviderImpl,
    GeneticsProviderUtils, create_evolution_engine, create_genetics_provider,
};
use beardog_traits::unified::identity::{
    ExtendedIdentity, IdentityInfo, IdentityValidation, IdentityWithLineage,
};
use beardog_traits::unified::monitoring::Observable;
use beardog_traits::unified::security::UnifiedSecurityConfig;
use beardog_traits::unified::{
    BearDogComponent, BearDogCore, BearDogService, ComponentHealth, ServiceMetrics, ServiceStatus,
    UnifiedTraitError,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

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
