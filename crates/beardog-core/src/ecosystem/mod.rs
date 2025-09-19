// Ecosystem integration and primal service coordination
//
// This module provides the core functionality for BearDog to interact with
// other primals in the ecosystem through capability-based discovery and
// sovereign service coordination.

pub mod ai_first_responses;
/// Primal interface definitions and implementations
pub mod primal_interface; // Now modular structure
pub mod primal_trait;
pub mod primal_types;
pub mod self_discovery;
/// Service registration and capability-based discovery
pub mod service_registration; // Self-discovery for capability-based ecosystem

pub use ai_first_responses::{AIFirstResponse, AIFirstResponseBuilder};
pub use primal_trait::EcoPrimal as BearDogEcoPrimalImplementation;
pub use primal_trait::EcoPrimal;
pub use primal_types::{
    AttestationVerificationResult,
    AuthenticationResult,
    CapabilityHealthStatus,
    EndpointHealth,
    HealthStatus as PrimalHealthStatus,
    KeyOperationStatus,
    PrimalCapability,
    PrimalConfig,
    // PrimalDependency removed - use ServiceDependency instead
    PrimalError,
    PrimalHealth,
    // PrimalIntegrationConfig removed - use UniversalIntegrationConfig instead
    PrimalMetadata,
    PrimalRequest,
    PrimalResponse,
    // PrimalType removed - use capability-based discovery instead
    ResponseTimeMetrics,
    // New capability-based types
    ServiceCapabilityType,
    ServiceDependency,
    UniversalIntegrationConfig,
};
pub use self_discovery::{DiscoveredService, HealthStatus, SelfDiscoveryManager, SelfIdentity};
pub use service_registration::EcosystemRegistration;
