// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ecosystem types for capability-based service discovery.
//!
//! This module provides capability-based types that replace hardcoded primal references
//! to maintain sovereignty compliance where primals only know themselves.

#![allow(
    deprecated,
    reason = "v0.10.0 migration target — deprecated types re-exported for backward compat"
)]

mod attestation;
mod discovery;
mod health;
mod identity;
mod io;

#[cfg(test)]
mod primal_types_tests;

pub use attestation::{
    AttestationVerificationChain, AttestationVerificationResult, AuthRequirements,
    AuthenticationResult, CapabilityHealthStatus, SecurityAttestation,
};
pub use beardog_types::canonical::HealthStatus;
pub use beardog_types::canonical::capabilities::ServiceCapabilityType;
pub use discovery::{
    CapabilityIntegrationConfig, CapabilityType, DiscoveredPrimal, PrimalMetadata,
    ServiceDependency, ServiceEndpoints, ServiceMetadata, UniversalEndpoint,
};
pub use health::{
    EndpointHealth, ErrorRateMetrics, KeyOperationStatus, LoadMetrics, PrimalHealth,
    PrimalHealthStatus, PrimalMetrics, ResourceUsageInfo, ResponseTimeMetrics,
};
pub use identity::{
    EndpointSecurityConfig, PrimalCapability, PrimalConfig, UniversalIntegrationConfig,
};
pub use io::{PrimalError, PrimalRequest, PrimalResponse, PrimalTypeMigrationHelper};
