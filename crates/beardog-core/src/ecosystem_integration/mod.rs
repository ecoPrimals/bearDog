

pub mod integration_engine;
pub mod license_manager;
pub mod types;
pub mod universal_adapter;
pub mod universal_hsm_provider;
pub mod songbird_service_discovery;
pub mod ecosystem_genetic_spawner;
pub mod toadstool_client;
pub mod examples;

pub use integration_engine::IntegrationEngine;
pub use license_manager::{LicenseConfig, LicenseManager};
pub use types::{EcosystemConfig, EcosystemRequest, EcosystemResponse};
pub use universal_adapter::{ProductionUniversalAdapter, ServiceEndpoint, UniversalAdapterConfig};

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

pub use songbird_service_discovery::{
    SongbirdServiceDiscovery,
    SongbirdServiceDiscoveryConfig,
    SongbirdServiceDiscoveryFactory,
    BearDogServiceInfo,
    ServiceRegistration,
    ServiceRegistryResponse,
};

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
