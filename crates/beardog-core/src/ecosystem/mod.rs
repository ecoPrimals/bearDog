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


pub mod ai_first_responses;
pub mod primal_interface;  // Now modular structure
pub mod primal_types;
pub mod primal_trait;
pub mod service_registration;

// Re-export specific items to avoid ambiguous glob re-exports
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
