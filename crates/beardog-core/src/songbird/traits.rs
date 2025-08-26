

use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use beardog_types::providers::ServiceHealth;
use super::types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshInfo, ServiceMeshStats,
    ServiceRegistrationRequest, ServiceUpdateRequest,
};

#[allow(async_fn_in_trait)]
pub trait UniversalServiceMesh: Send + Sync {

    async fn discover_service_meshes(&self) -> BearDogResult<Vec<ServiceMeshInfo>>;

    async fn connect_to_mesh(&self, mesh_info: &ServiceMeshInfo) -> BearDogResult<()>;

    async fn register_service(
        &self,
        request: ServiceRegistrationRequest,
    ) -> BearDogResult<RegistrationInfo>;

    async fn deregister_service(&self) -> BearDogResult<()>;

    async fn lookup_services(
        &self,
        request: ServiceLookupRequest,
    ) -> BearDogResult<Vec<DiscoveredService>>;

    async fn update_service(&self, request: ServiceUpdateRequest) -> BearDogResult<()>;

    async fn get_service_health(&self, service_id: &str) -> BearDogResult<ServiceHealth>;

    async fn get_registration(&self) -> Option<RegistrationInfo>;

    async fn get_active_mesh(&self) -> Option<ServiceMeshInfo>;

    async fn failover_to_alternative(&self) -> BearDogResult<ServiceMeshInfo>;

    async fn get_mesh_stats(&self) -> BearDogResult<ServiceMeshStats>;

    async fn refresh_service_cache(&self) -> BearDogResult<()>;

    async fn is_connected(&self) -> bool;

    async fn list_services(&self) -> BearDogResult<Vec<DiscoveredService>>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceMeshDiscovery: Send + Sync {

    async fn discover(&self) -> BearDogResult<Vec<ServiceMeshInfo>>;

    async fn health_check(&self, mesh: &ServiceMeshInfo) -> BearDogResult<ServiceHealth>;

    async fn rank_meshes(
        &self,
        meshes: Vec<ServiceMeshInfo>,
    ) -> BearDogResult<Vec<ServiceMeshInfo>>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceRegistrationOps: Send + Sync {

    async fn register(
        &self,
        request: ServiceRegistrationRequest,
    ) -> BearDogResult<RegistrationInfo>;

    async fn update(&self, request: ServiceUpdateRequest) -> BearDogResult<()>;

    async fn deregister(&self, service_id: &str) -> BearDogResult<()>;

    async fn refresh(&self, service_id: &str) -> BearDogResult<()>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceLookupOps: Send + Sync {

    async fn lookup(&self, request: ServiceLookupRequest) -> BearDogResult<Vec<DiscoveredService>>;

    async fn get_service(&self, service_id: &str) -> BearDogResult<Option<DiscoveredService>>;

    async fn search(&self, pattern: &str) -> BearDogResult<Vec<DiscoveredService>>;
}

#[allow(async_fn_in_trait)]
pub trait ServiceHealthOps: Send + Sync {

    async fn check_health(&self, service_id: &str) -> BearDogResult<ServiceHealth>;

    async fn get_all_health(&self) -> BearDogResult<Vec<(String, ServiceHealth)>>;

    async fn update_health(&self, service_id: &str, health: ServiceHealth) -> BearDogResult<()>;
}
