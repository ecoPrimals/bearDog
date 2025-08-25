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


/// # Songbird Client Modules
///
/// **MODULAR SONGBIRD SERVICE MESH CLIENT SYSTEM**
/// This module splits the large 894-line songbird_client.rs file into focused, maintainable modules
/// following the 2000-line limit principle and single responsibility pattern.
/// ## Module Organization:
/// - `types` - Service mesh info, registration info, and data structures
/// - `traits` - UniversalServiceMesh trait definitions
/// - `client` - Main service mesh client implementation
/// - `discovery` - Service discovery and mesh finding logic
/// - `operations` - Service registration, lookup, and management operations

// Focused songbird client modules
pub mod client;
pub mod discovery;
pub mod operations;
pub mod traits;
pub mod types;
// Re-export songbird types
pub use types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceMeshCapability,
    ServiceMeshInfo, ServiceRegistrationRequest, ServiceUpdateRequest,
};
pub use traits::UniversalServiceMesh;
pub use client::UniversalCommunicationMeshClient;
pub use traits::{
    ServiceHealthOps, ServiceLookupOps, ServiceMeshDiscovery, ServiceRegistrationOps,
