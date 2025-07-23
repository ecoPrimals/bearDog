//! Core BearDog Security Manager functionality
//!
//! This crate provides the main BearDog orchestration engine and core functionality.

pub mod biome_yaml_parser;
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

use async_trait::async_trait;
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};

// Re-export core functionality
pub use crate::biome_yaml_parser::{
    BiomeEnvironment, BiomeManifest, BiomeMetadata, BiomeSecurityContext, BiomeYamlParser,
    PrimalConfig, PrimalResourceRequirements, ServiceDefinition,
};
pub use crate::core::BearDogCore;
pub use crate::ecosystem_integration::{EcosystemRequest, EcosystemResponse};
pub use crate::ecosystem_simple::BearDogEcosystemProvider as SimpleBearDogEcosystemProvider;
pub use crate::ecosystem_storage::{
    AuditConfig, EcosystemDataStore, EcosystemStorageConfig, EcosystemStorageService,
    RetentionPolicy, SnapshotConfig, StorageHealthReport,
};
// Use universal ecosystem integration from beardog-adapters
pub use crate::local_optimizer::BearDogLocalOptimizer;
pub use crate::songbird_client::{
    DiscoveredService, RegistrationInfo, RegistrationStatus, ResponseStatus, ServiceMeshInfo,
    ServiceRequest, ServiceResponse, UniversalServiceMesh, UniversalServiceMeshClient,
};
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

// Core traits and types
#[async_trait]
pub trait BearDogService: Send + Sync {
    async fn start(&mut self) -> BearDogResult<()>;
    async fn stop(&mut self) -> BearDogResult<()>;
    async fn health_check(&self) -> BearDogResult<HealthStatus>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: HealthStatus,
}
