// # Beardog Core Library
//
// Core functionality for the Beardog ecosystem including AI-powered hybrid intelligence,
// biome sovereignty, genetic algorithms, and universal service discovery.
//
// This crate provides the foundational components for:
// - AI-human hybrid decision making systems
// - Sovereign cryptographic key management
// - Genetic algorithm-based key evolution
// - Universal service discovery and orchestration
// - Zero-copy memory optimization
// - 100% safe Rust implementation
//
// ## Features
//
// - **Hybrid Intelligence**: AI-assisted human decision making
// - **Biome Sovereignty**: Human-owned entropy and key management
// - **Service Discovery**: Universal network service orchestration
// - **Zero Unsafe Code**: Complete memory safety without unsafe blocks
//
// ## Example
//
// ```rust
// use beardog_core::BearDogConfig;
//
// let config = BearDogConfig::default();
// println!("Beardog {} running in {} environment",
//          config.version, config.environment);
// ```

/// Core functionality
/// Core functionality
pub mod core;
/// Shared types used across the Beardog system
pub mod types;

pub mod ai;
pub mod biome_sovereignty;
/// Ecosystem integration and primal service coordination
pub mod ecosystem;
/// Ecosystem integration capabilities
pub mod ecosystem_integration;
pub mod ecosystem_storage;
/// External function integrations
pub mod external_functions;
/// Primal sovereignty implementation
pub mod primal_sovereignty;
/// Universal service discovery and network orchestration
pub mod universal_discovery;
// TODO: Fix syntax errors in universal_optimization module
// pub mod universal_optimization;
/// Zero-cost architecture patterns and optimizations
pub mod zero_cost_architecture;
// TODO: Fix remaining async/type issues in zero_knowledge_bootstrap module
// pub mod zero_knowledge_bootstrap;

// Re-export key components
pub use core::*;
pub use types::BearDogConfig;
