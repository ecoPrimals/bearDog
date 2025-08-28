

pub mod client;
pub mod discovery;
// pub mod operations; // Temporarily disabled due to syntax issues - needs refactoring
pub mod traits;
pub mod types;

pub use types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshCapability,
    ServiceMeshInfo, ServiceRegistrationRequest, ServiceUpdateRequest,
};
pub use traits::UniversalServiceMesh;
pub use client::UniversalCommunicationMeshClient;
pub use traits::{
    ServiceMeshDiscovery,
    // ServiceHealthOps, ServiceLookupOps, ServiceRegistrationOps, // From operations module - temporarily disabled
};
