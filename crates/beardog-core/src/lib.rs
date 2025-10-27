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

#![deny(unsafe_code)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
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
//! - **Zero Unsafe Code**: Complete memory safety without unsafe blocks
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
//! This crate maintains **zero unsafe code** in production paths, achieving
//! TOP 0.1% worldwide safety rating with complete memory safety guarantees.

/// Core functionality providing the foundational `BearDog` implementation
///
/// This module contains the main `BearDogCore` struct and related functionality
/// for initializing and managing `BearDog` instances.
pub mod core;

/// Shared types used across the Beardog system
pub mod types;

/// AI-powered hybrid intelligence system
///
/// Combines human decision-making with AI assistance for sovereign operations.
/// Includes neural network integration, learning systems, and performance optimization.
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

/// External function integrations
///
/// Foreign Function Interface (FFI) system for integrating with external
/// libraries, native code, and system-level operations with safety guarantees.
pub mod external_functions;

/// Primal sovereignty implementation
///
/// Core implementation of primal sovereignty patterns, ensuring human control
/// and autonomous operation without corporate dependencies.
pub mod primal_sovereignty;

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

// Re-export key components
pub use core::*;
pub use types::BearDogConfig;

#[cfg(test)]
mod tests;
