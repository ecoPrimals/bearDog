// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// BearDog Core Library
///
/// This crate provides the core functionality for the BearDog security platform.

// Core modules - canonical modernization complete
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
// Re-export common types (using local definitions to avoid circular deps)
pub use beardog_errors::{BearDogError, BearDogResult};
pub use types::*;
// Re-export key external crate types that modules expect
pub use beardog_adapters;
pub use beardog_auth;
pub use beardog_compliance;
// ✅ MODERNIZATION: All modules use canonical patterns - no special handling needed
// Make sure serde macros are available everywhere
// Note: async_trait usage being phased out in favor of native async fn
pub use serde::{Deserialize, Serialize};
// Re-export core functionality
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
// Core traits and types - modernized with native async fn
/// **ZERO-COST ASYNC OPTIMIZATION** - Native async methods eliminate boxing overhead
/// 
/// This trait now uses native async fn in trait definitions (available in Rust 1.75+)
/// which eliminates the Box<dyn Future> allocation overhead from async_trait.
#[allow(async_fn_in_trait)]
pub trait BearDogService: Send + Sync {
    /// Start the service using native async fn (zero-cost)
    fn start(&mut self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
    
    /// Stop the service using native async fn (zero-cost)
    fn stop(&mut self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
    
    /// Perform health check using native async fn (zero-cost)
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
