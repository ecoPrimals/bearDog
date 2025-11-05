//! # Primal Interface Module
//!
//! Provides the standard interface for primal-to-primal communication
//! and ecosystem integration within the BearDog sovereignty architecture.
//!
//! ## Overview
//!
//! The primal interface defines how primals (ecosystem services) communicate:
//! - **API Endpoints**: Standard REST/gRPC interfaces
//! - **Ecosystem Integration**: Service discovery and coordination
//! - **HSM Management**: Hardware security module interfaces
//! - **Trait Implementations**: Core primal behavior implementations
//!
//! ## Key Components
//!
//! - [`api_endpoints`] - API endpoint definitions and handlers
//! - [`ecosystem_integration`] - Ecosystem service integration
//! - [`hsm_management`] - HSM management and key operations
//! - [`trait_impl`] - Implementation of core primal traits
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem::primal_interface::api_endpoints::PrimalApi;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Primals communicate through standard interfaces
//! let api = PrimalApi::new();
//! api.register_endpoints().await?;
//! # Ok(())
//! # }
//! ```

/// API endpoint definitions and handlers
pub mod api_endpoints;

/// Ecosystem service integration
pub mod ecosystem_integration;

/// HSM management and key operations
pub mod hsm_management;

/// Implementation of core primal traits
pub mod trait_impl;
