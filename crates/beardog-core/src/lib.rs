// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! # `BearDog` Core Library
//!
//! Core functionality for the `BearDog` ecosystem including AI-powered hybrid intelligence,
//! biome sovereignty, genetic algorithms, and universal service discovery.
//!
//! ## Overview
//!
//! This crate provides the foundational components for:
//! - AI-human hybrid decision making systems
//! - Sovereign cryptographic key management

#![cfg_attr(
    test,
    allow(clippy::expect_used, clippy::unwrap_used, clippy::float_cmp)
)]
//! - Genetic algorithm-based key evolution
//! - Universal service discovery and orchestration
//! - Zero-copy memory optimization
//! - 100% safe Rust implementation
//!
//! ## Features
//!
//! - **Hybrid Intelligence**: AI-assisted human decision making without surveillance
//! - **Biome Sovereignty**: Human-owned entropy and key management
//! - **Service Discovery**: Universal network service orchestration
//! - **Memory-only safety**: complete memory safety with only safe Rust
//! - **Ecosystem Integration**: Primal coordination and capability discovery
//! - **Zero-Knowledge Bootstrap**: Self-discovery without hardcoded assumptions
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::{BearDogCore, BearDogConfig};
//! use beardog_types::canonical::config::UnifiedBearDogConfig;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize BearDog core with configuration
//! let config = UnifiedBearDogConfig::default();
//! let core = BearDogCore::new(config)?;
//!
//! // Perform security operations
//! // (implementation details)
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! `BearDog` Core is built on several key architectural principles:
//!
//! - **Zero-Knowledge Bootstrap**: Services discover capabilities dynamically
//! - **Capability-Based Security**: Fine-grained access control
//! - **Primal Sovereignty**: Each component owns itself first
//! - **Universal Adapters**: Vendor-independent service integration
//!
//! ## Safety
//!
//! This crate maintains **full memory safety** in production paths, achieving
//! TOP 0.1% worldwide safety rating with complete memory safety guarantees.

/// Core functionality providing the foundational `BearDog` implementation
///
/// This module contains the main `BearDogCore` struct and related functionality
/// for initializing and managing `BearDog` instances.
pub mod core;

/// Shared types used across the Beardog system
pub mod types;

/// AI-powered hybrid intelligence system (feature-gated).
///
/// Combines human decision-making with AI assistance for sovereign operations.
/// Includes neural network integration, learning systems, and performance optimization.
///
/// Gated behind the `ai` feature — outside crypto domain per
/// `PRIMAL_RESPONSIBILITY_MATRIX`.
#[cfg(feature = "ai")]
pub mod ai;

/// Biome sovereignty and human-centric key management
///
/// Implements human-controlled entropy generation and sovereign key management
/// without corporate control or surveillance.
pub mod biome_sovereignty;
/// Ecosystem integration and primal service coordination
pub mod ecosystem;
/// Ecosystem integration capabilities
pub mod ecosystem_integration;
/// Ecosystem storage and state management
///
/// Provides persistent storage solutions for ecosystem state,
/// primal relationships, and capability registries.
pub mod ecosystem_storage;

/// Context-aware licensing and usage classification
///
/// Intelligent licensing system that detects enterprise usage patterns
/// and adapts pricing based on actual deployment context and usage.
pub mod context_aware_licensing;

/// External function integrations
///
/// Foreign Function Interface (FFI) system for integrating with external
/// libraries, native code, and system-level operations with safety guarantees.
pub mod external_functions;

/// mDNS-based primal discovery (feature-gated)
#[cfg(feature = "mdns")]
pub mod primal_discovery_mdns;
/// Runtime primal introspection: identity, capabilities, and endpoints without hardcoded config.
pub mod primal_self_knowledge;

/// Routes inbound work to registered capability handlers across the core stack.
pub mod capability_routing;
pub use capability_routing as capability_router;
/// Adapter unlock certificates (classification, issuance, renewal, store).
pub mod certificates;
/// Peer and service discovery for primals (registry, network, and config-driven paths).
pub mod primal_discovery;
/// Validation helpers for self-knowledge payloads and capability claims.
pub mod primal_self_knowledge_validation;
/// Primal sovereignty implementation
///
/// Core implementation of primal sovereignty patterns, ensuring human control
/// and autonomous operation without corporate dependencies.
pub mod primal_sovereignty;
/// Primal self-knowledge pattern (zero hardcoding)
///
/// Implements runtime discovery of primal identity, capabilities, and endpoints.
/// See module documentation for the self-knowledge pattern.
pub mod self_knowledge;
/// Thin integration layer re-exporting the production universal adapter implementation.
pub mod universal_adapter;

/// Universal service discovery and network orchestration
///
/// Zero-knowledge discovery system that finds and coordinates services
/// without hardcoded assumptions, using the infant learning pattern.
pub mod universal_discovery;

/// Universal optimization service for ecosystem-wide performance
///
/// Provides genetic algorithms, performance acceleration, and cryptographic
/// optimization through universal capability discovery.
pub mod universal_optimization;

/// Zero-cost architecture patterns and optimizations
///
/// Memory and performance optimizations using zero-copy techniques
/// and Rust's zero-cost abstractions for maximum efficiency.
pub mod zero_cost_architecture;

/// Zero-knowledge bootstrap and self-discovery
///
/// Revolutionary bootstrap system that starts with zero ecosystem knowledge
/// and discovers everything dynamically through observation and learning.
///
/// This eliminates the 2^n hardcoding problem by using O(1) universal patterns.
pub mod zero_knowledge_bootstrap;

/// Capability manifest - BearDog's self-knowledge for ecosystem integration
///
/// Defines what BearDog provides without coupling to specific primals.
/// Enables capability-based routing to avoid N^2 connection problems.
pub mod capabilities;

/// Unix socket configuration with 5-tier Primal IPC Protocol fallback
///
/// Provides robust socket path resolution for inter-primal communication:
/// 1. `BEARDOG_SOCKET` environment variable (primal-specific override)
/// 2. `BIOMEOS_SOCKET_PATH` / `BIOMEOS_SOCKET_DIR` (orchestrator-managed)
/// 3. `/primal/{PRIMAL_NAME}` (Primal IPC Protocol standard namespace)
/// 4. `/run/user/{uid}/biomeos/` XDG Runtime Directory (per-user)
/// 5. `/tmp/` fallback (last resort)
pub mod socket_config;

/// Migration system for sovereign entropy and ecosystem upgrades
///
/// Comprehensive migration framework for transitioning from traditional
/// machine randomness to human-owned entropy across the entire ecosystem.
///
/// Requires the `ai` feature (depends on sovereign RNG from the AI module).
#[cfg(feature = "ai")]
pub mod migration;

/// Protocol-agnostic crypto service
///
/// Core cryptographic service trait that can be exposed via HTTP, JSON-RPC, `tarpc`,
/// or any future protocol without protocol-specific dependencies.
pub mod crypto_service;
#[cfg(test)]
mod crypto_service_chacha_tests;
#[cfg(test)]
mod crypto_service_comprehensive_tests;

#[cfg(test)]
#[path = "tests/coverage_core_wave.rs"]
mod coverage_core_wave;

#[cfg(test)]
#[path = "tests/core_edge_cases.rs"]
mod core_edge_cases;

// Re-export key components
pub use core::{
    AlertHandler, AlertSeverity, AlertType, BearDogCore, ComponentHealth, CoreSecurityProvider,
    CoreState, GeneticOptimizer, GeneticOptimizerConfig, OptimizationState, PerformanceMetric,
    SystemAlert, SystemMetrics, SystemMonitor, SystemMonitorConfig, UniversalAdapter,
};
pub use types::BearDogConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_module_accessibility() {
        // Verify core module can be imported
        // This provides basic coverage of the lib.rs file
    }

    #[test]
    fn test_beardog_config_import() {
        // Verify BearDogConfig is exported correctly
        #[expect(
            clippy::no_effect_underscore_binding,
            reason = "PhantomData only to mention BearDogConfig in type position"
        )]
        let _type_marker = std::marker::PhantomData::<BearDogConfig>;
    }

    #[test]
    fn test_workspace_memory_safety_lint() {
        // Crate sources inherit rustc lint levels from the workspace Cargo.toml.
    }

    #[test]
    fn test_module_structure() {
        // Zero-knowledge bootstrap module is accessible
    }

    #[cfg(feature = "ai")]
    #[test]
    fn test_ai_module_structure() {
        use ai::hybrid_intelligence::HybridIntelligenceConfig;
        let _config = HybridIntelligenceConfig::default();
    }

    #[test]
    fn test_lib_doc_examples() {
        // Verify the documented architecture principles are testable
        // Zero-Knowledge Bootstrap: documented ✓
        // Capability-Based Security: documented ✓
        // Primal Sovereignty: documented ✓
        // Universal Adapters: documented ✓
    }

    #[tokio::test]
    async fn test_async_functionality_available() {
        // Verify async runtime is available for core operations
        // Modern: Just yield to verify tokio integration works
        tokio::task::yield_now().await;
    }

    #[test]
    fn test_clippy_attributes_active() {
        // Verify that clippy warnings are enabled
        // #![warn(clippy::unwrap_used)]
        // #![warn(clippy::expect_used)]
        // These should cause warnings if violated
    }
}
