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


/// Ecosystem Integration for BearDog
///
/// Provides integration with external systems through licensed external functions.
/// This is the core revenue-generating capability of BearDog.
/// 
/// ## Phase 4: Universal HSM Architecture
/// 
/// This module now includes the Universal HSM Architecture implementation,
/// providing vendor-agnostic HSM operations across the entire ecosystem
/// through Songbird service mesh integration.

pub mod integration_engine;
pub mod license_manager;
pub mod types;
pub mod universal_adapter;
pub mod universal_hsm_provider;
pub mod songbird_service_discovery;
pub mod ecosystem_genetic_spawner;
pub mod toadstool_client;
pub mod examples;

// Re-export main types and structs
pub use integration_engine::IntegrationEngine;
pub use license_manager::{LicenseConfig, LicenseManager};
pub use types::{EcosystemConfig, EcosystemRequest, EcosystemResponse};
pub use universal_adapter::{ProductionUniversalAdapter, ServiceEndpoint, UniversalAdapterConfig};

// Re-export Universal HSM Architecture components
pub use universal_hsm_provider::{
    UniversalHsmProvider, 
    EcosystemHsmProvider, 
    EcosystemServiceDiscovery,
    ProviderHealthStatus,
    ProviderSelectionStrategy,
    HsmFailoverManager,
    IntelligentProviderSelector,
    ProviderMetrics,
};

// Re-export Songbird service discovery components
pub use songbird_service_discovery::{
    SongbirdServiceDiscovery,
    SongbirdServiceDiscoveryConfig,
    SongbirdServiceDiscoveryFactory,
    BearDogServiceInfo,
    ServiceRegistration,
    ServiceRegistryResponse,
};

// Re-export Ecosystem Genetic Spawning components
pub use ecosystem_genetic_spawner::{
    EcosystemGeneticSpawner,
    EcosystemCapability,
    EcosystemGeneticContribution,
    EcosystemGeneticBlueprint,
    EcosystemHybridNode,
    EcosystemSpawningRequirements,
    EcosystemResourceAllocation,
    EcosystemSpawningOperation,
    EcosystemSpawningStatistics,
    EcosystemPrimalClient,
    GeneticTrait,
    TraitCategory,
    SpawningStatus,
    SpawningStage,
    NodeHealthStatus,
};

// Re-export ToadStool client components
pub use toadstool_client::{
    ToadStoolComputeClient,
    ToadStoolClientConfig,
    ToadStoolClientFactory,
    ToadStoolComputeGenetics,
    ToadStoolComputeRequest,
    ToadStoolComputeResponse,
    ComputeArchitecture,
    ProcessingCapability,
    OptimizationType,
    ComputePriority,
};
