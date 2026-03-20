// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem Integration and Primal Service Coordination
//!
//! This module provides the core functionality for BearDog to interact with
//! other primals in the ecosystem through capability-based discovery and
//! sovereign service coordination.
//!
//! # Key Features
//!
//! - **Capability-Based Discovery**: Find services by capability, not by name
//! - **Primal Interface**: Standard interface for ecosystem primals
//! - **Service Registration**: Dynamic service registration and discovery
//! - **AI-First Responses**: Intelligent response caching and optimization
//! - **Self-Discovery**: Automatic capability detection and advertisement
//!
//! # Architecture
//!
//! The ecosystem integration follows the primal sovereignty pattern:
//! - No hardcoded service names or endpoints
//! - Capability-based service discovery
//! - Health-based routing and failover
//! - Zero-knowledge bootstrap capability
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_core::ecosystem::{PrimalCapability, EcoPrimal};
//!
//! // Discover services by capability
//! let compute_services = PrimalCapability::Compute;
//! // Services are discovered dynamically at runtime
//! ```

/// AI-first response system for intelligent caching and optimization
pub mod ai_first_responses;
/// Primal interface definitions and implementations
pub mod primal_interface;
/// Core primal trait for ecosystem integration
pub mod primal_trait;
/// Shared types for primal operations
pub mod primal_types;
/// Self-discovery and capability detection
pub mod self_discovery;
/// Service registration and capability-based discovery
pub mod service_registration;

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

#[cfg(test)]
mod tests;
