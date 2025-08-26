

pub mod ai_first_responses;
pub mod primal_interface;  // Now modular structure
pub mod primal_types;
pub mod primal_trait;
pub mod service_registration;

pub use ai_first_responses::{AIFirstResponse, AIFirstResponseBuilder};
use beardog_types::canonical::HealthStatus;
pub use primal_interface::EcoPrimal as `BearDog`EcoPrimalImplementation;
pub use primal_trait::EcoPrimal;
pub use primal_types::{
    AuthenticationResult, AttestationVerificationResult, CapabilityHealthStatus,
    EndpointHealth, HealthStatus as PrimalHealthStatus, HsmHealthStatus,
    KeyOperationStatus, PrimalConfig, PrimalError, PrimalHealth,
    PrimalIntegrationConfig, PrimalMetadata, PrimalRequest, PrimalResponse,
    PrimalType, PrimalCapability, PrimalDependency,
    ResourceUsageInfo as PrimalResourceUsage, ResponseTimeMetrics,
};
pub use service_registration::{
    HealthStatus as ServiceHealthStatus, ResourceSpec, ResourceUsage, ServiceCapability,
    ServiceMetadata, UniversalServiceRegistration, UniversalServiceRegistry,
