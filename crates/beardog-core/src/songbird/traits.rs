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


/// # Songbird Service Mesh Traits
///
/// **SERVICE MESH TRAIT DEFINITIONS - MODERNIZED ✅**
/// Contains trait definitions for universal service mesh operations
/// extracted from the monolithic songbird_client.rs file.
/// 
/// **ZERO-COST ASYNC OPTIMIZATION** - Native async methods eliminate boxing overhead
/// This module now uses native async fn in trait definitions (available in Rust 1.75+)
/// which eliminates the Box<dyn Future> allocation overhead from async_trait.

use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use beardog_types::providers::ServiceHealth;
use super::types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshInfo, ServiceMeshStats,
    ServiceRegistrationRequest, ServiceUpdateRequest,
};

/// Universal service mesh interface that works with any service mesh primal
/// **ZERO-COST ASYNC** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait UniversalServiceMesh: Send + Sync {
    /// Discover available service mesh primals in the ecosystem
    async fn discover_service_meshes(&self) -> BearDogResult<Vec<ServiceMeshInfo>>;
    
    /// Connect to a specific service mesh
    async fn connect_to_mesh(&self, mesh_info: &ServiceMeshInfo) -> BearDogResult<()>;
    
    /// Register `BearDog` as a service with the active service mesh
    async fn register_service(
        &self,
        request: ServiceRegistrationRequest,
    ) -> BearDogResult<RegistrationInfo>;
    
    /// Deregister `BearDog` from the service mesh
    async fn deregister_service(&self) -> BearDogResult<()>;
    
    /// Look up services in the mesh by criteria
    async fn lookup_services(
        &self,
        request: ServiceLookupRequest,
    ) -> BearDogResult<Vec<DiscoveredService>>;
    
    /// Update service registration information
    async fn update_service(&self, request: ServiceUpdateRequest) -> BearDogResult<()>;
    
    /// Get health status of a specific service
    async fn get_service_health(&self, service_id: &str) -> BearDogResult<ServiceHealth>;
    
    /// Get current registration info
    async fn get_registration(&self) -> Option<RegistrationInfo>;
    
    /// Get information about the active service mesh
    async fn get_active_mesh(&self) -> Option<ServiceMeshInfo>;
    
    /// Failover to alternative service mesh if current one fails
    async fn failover_to_alternative(&self) -> BearDogResult<ServiceMeshInfo>;
    
    /// Get service mesh statistics
    async fn get_mesh_stats(&self) -> BearDogResult<ServiceMeshStats>;
    
    /// Refresh service discovery cache
    async fn refresh_service_cache(&self) -> BearDogResult<()>;
    
    /// Check if connected to a service mesh
    async fn is_connected(&self) -> bool;
    
    /// Get list of all registered services
    async fn list_services(&self) -> BearDogResult<Vec<DiscoveredService>>;
}

/// Service mesh discovery trait for finding available meshes
/// **ZERO-COST ASYNC** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait ServiceMeshDiscovery: Send + Sync {
    /// Discover service meshes using various methods
    async fn discover(&self) -> BearDogResult<Vec<ServiceMeshInfo>>;
    
    /// Check health of discovered meshes
    async fn health_check(&self, mesh: &ServiceMeshInfo) -> BearDogResult<ServiceHealth>;
    
    /// Rank meshes by preference/priority
    async fn rank_meshes(
        &self,
        meshes: Vec<ServiceMeshInfo>,
    ) -> BearDogResult<Vec<ServiceMeshInfo>>;
}

/// Service registration operations trait
/// **ZERO-COST ASYNC** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait ServiceRegistrationOps: Send + Sync {
    /// Register a service
    async fn register(
        &self,
        request: ServiceRegistrationRequest,
    ) -> BearDogResult<RegistrationInfo>;
    
    /// Update service registration
    async fn update(&self, request: ServiceUpdateRequest) -> BearDogResult<()>;
    
    /// Deregister a service
    async fn deregister(&self, service_id: &str) -> BearDogResult<()>;
    
    /// Refresh service registration (heartbeat)
    async fn refresh(&self, service_id: &str) -> BearDogResult<()>;
}

/// Service lookup operations trait
/// **ZERO-COST ASYNC** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait ServiceLookupOps: Send + Sync {
    /// Look up services by criteria
    async fn lookup(&self, request: ServiceLookupRequest) -> BearDogResult<Vec<DiscoveredService>>;
    
    /// Get service by ID
    async fn get_service(&self, service_id: &str) -> BearDogResult<Option<DiscoveredService>>;
    
    /// Search services by name pattern
    async fn search(&self, pattern: &str) -> BearDogResult<Vec<DiscoveredService>>;
}

/// Service health operations trait
/// **ZERO-COST ASYNC** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait ServiceHealthOps: Send + Sync {
    /// Check health of a service
    async fn check_health(&self, service_id: &str) -> BearDogResult<ServiceHealth>;
    
    /// Get health status of all services
    async fn get_all_health(&self) -> BearDogResult<Vec<(String, ServiceHealth)>>;
    
    /// Update health status
    async fn update_health(&self, service_id: &str, health: ServiceHealth) -> BearDogResult<()>;
}
