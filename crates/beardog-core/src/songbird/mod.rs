

pub mod client;
pub mod discovery;
pub mod operations;
pub mod traits;
pub mod types;

pub use types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshCapability,
    ServiceMeshInfo, ServiceRegistrationRequest, ServiceUpdateRequest,
};
pub use traits::UniversalServiceMesh;
pub use client::UniversalCommunicationMeshClient;
pub use traits::{
    ServiceHealthOps, ServiceLookupOps, ServiceMeshDiscovery, ServiceRegistrationOps,
