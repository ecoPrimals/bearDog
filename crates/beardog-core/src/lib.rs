

pub mod context_aware_licensing;
pub mod core;
pub mod ecosystem_integration;
pub mod ecosystem_simple;
pub mod ecosystem_storage;
pub mod external_functions;
pub mod licensing;
pub mod local_optimizer;
pub mod node_registry;
pub mod songbird_client;
pub mod types;
pub mod universal_discovery;
pub mod universal_optimization;
pub mod universal_primal_provider;

pub use beardog_errors::{BearDogError, BearDogResult};
pub use types::*;

pub use beardog_adapters;
pub use beardog_auth;
pub use beardog_compliance;

pub use serde::{Deserialize, Serialize};

pub use crate::core::BearDogCore;
pub use crate::ecosystem_integration::{EcosystemRequest, EcosystemResponse};
pub use crate::ecosystem_simple::BearDogEcosystemProvider as SimpleBearDogEcosystemProvider;
pub use crate::ecosystem_storage::{
    AuditConfig, EcosystemDataStore, EcosystemStorageConfig, EcosystemStorageService,
    RetentionPolicy, SnapshotConfig, StorageHealthReport,
};

pub use crate::local_optimizer::BearDogLocalOptimizer;
pub use crate::songbird_client::{
    DiscoveredService, RegistrationInfo, RegistrationStatus, ResponseStatus, ServiceMeshInfo,
    ServiceRequest, ServiceResponse, UniversalServiceMesh, UniversalServiceMeshClient,};

pub use crate::universal_discovery::{
    CapabilityType, EcosystemCapabilityDiscovery, ModuleInstance, PerformanceRequirements,
    QualityRequirements, UniversalCapabilityDiscovery, UniversalModuleRequest,
    UniversalModuleResponse,
};

pub use crate::universal_optimization::{
    EcosystemOptimizationService, GeneticTarget, LocalOptimizer, OptimizationRequest,
    OptimizationResponse, PerformanceMetrics as UniversalPerformanceMetrics,
    UniversalOptimizationService, WorkloadType,
};

pub use crate::universal_primal_provider::{
    EcosystemRole, PrimalCapability, PrimalIdentity, PrimalMetadata, PrimalService, PrimalType,
    SecurityContext, ServiceContext, ServiceEndpoint, ServiceHealth, UniversalPrimalProvider,
};
pub use beardog_adapters::EcosystemIntegration;

#[allow(async_fn_in_trait)]
pub trait BearDogService: Send + Sync {

    fn start(&mut self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn stop(&mut self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: HealthStatus,
}
