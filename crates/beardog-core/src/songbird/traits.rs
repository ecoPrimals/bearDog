

use beardog_errors::BearDogError;
use beardog_types::providers::ServiceHealth;
use super::types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshInfo, ServiceMeshStats,
    ServiceRegistrationRequest, ServiceUpdateRequest,
};

#[allow(async_fn_in_trait)]
pub trait UniversalServiceMesh: Send + Sync {

    async fn discover_service_meshes(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError>;

    async fn connect_to_mesh(&self, mesh_info: &ServiceMeshInfo) -> Result<(), BearDogError>;

    async fn register_service(
        &self,
        request: ServiceRegistrationRequest,
    ) -> Result<RegistrationInfo, BearDogError>;

    async fn deregister_service(&self) -> Result<(), BearDogError>;

    async fn lookup_services(
        &self,
        request: ServiceLookupRequest,
    ) -> Result<Vec<DiscoveredService>, BearDogError>;

    async fn update_service(&self, request: ServiceUpdateRequest) -> Result<(), BearDogError>;

    async fn get_service_health(&self, service_id: &str) -> Result<ServiceHealth, BearDogError>;

    async fn get_registration(&self) -> Option<RegistrationInfo>;

    async fn get_active_mesh(&self) -> Option<ServiceMeshInfo>;

    async fn failover_to_alternative(&self) -> Result<ServiceMeshInfo, BearDogError>;

    async fn get_mesh_stats(&self) -> Result<ServiceMeshStats, BearDogError>;

    async fn refresh_service_cache(&self) -> Result<(), BearDogError>;

    async fn is_connected(&self) -> bool;

    async fn list_services(&self) -> Result<Vec<DiscoveredService>, BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceMeshDiscovery: Send + Sync {

    async fn discover(&self) -> Result<Vec<ServiceMeshInfo>, BearDogError>;

    async fn health_check(&self, mesh: &ServiceMeshInfo) -> Result<ServiceHealth, BearDogError>;

    async fn rank_meshes(
        &self,
        meshes: Vec<ServiceMeshInfo>,
    ) -> Result<Vec<ServiceMeshInfo>, BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceRegistrationOps: Send + Sync {

    async fn register(
        &self,
        request: ServiceRegistrationRequest,
    ) -> Result<RegistrationInfo, BearDogError>;

    async fn update(&self, request: ServiceUpdateRequest) -> Result<(), BearDogError>;

    async fn deregister(&self, service_id: &str) -> Result<(), BearDogError>;

    async fn refresh(&self, service_id: &str) -> Result<(), BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceLookupOps: Send + Sync {

    async fn lookup(&self, request: ServiceLookupRequest) -> Result<Vec<DiscoveredService>, BearDogError>;

    async fn get_service(&self, service_id: &str) -> Result<Option<DiscoveredService>, BearDogError>;

    async fn search(&self, pattern: &str) -> Result<Vec<DiscoveredService>, BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceHealthOps: Send + Sync {

    async fn check_health(&self, service_id: &str) -> Result<ServiceHealth, BearDogError>;

    async fn get_all_health(&self) -> Result<Vec<(String, ServiceHealth)>>;

    async fn update_health(&self, service_id: &str, health: ServiceHealth) -> Result<(), BearDogError>;
}
