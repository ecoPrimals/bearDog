//! # BearDog Authentication and Authorization
//!
//! Secure authentication and authorization capabilities for BearDog applications,
//! featuring decentralized identity management, proof verification, and human-centric auth.
//!
//! ## Features
//!
//! - **Decentralized Identity**: Self-sovereign identity without central authority
//! - **Proof Verification**: Cryptographic proof systems for authentication
//! - **Human-Centric Auth**: Biometric and behavioral authentication
//! - **Zero-Knowledge Proofs**: Privacy-preserving authentication
//! - **Multi-Factor Auth**: Layered security with multiple proof types
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_auth::auth::AuthEngine;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize authentication engine
//! let auth_engine = AuthEngine::new()?;
//!
//! // Authenticate user with credentials
//! let token = auth_engine.authenticate("user_id", "credentials").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The auth system is built on decentralized principles:
//! - **No Central Authority**: Users control their own identity
//! - **Cryptographic Proofs**: Mathematical verification of identity
//! - **Privacy-First**: Zero-knowledge proofs protect user data
//! - **Human Dignity**: No surveillance or data extraction
//!
//! ## Safety
//!
//! All authentication operations are memory-safe with zero unsafe code.

/// Core authentication functionality
///
/// Provides the main authentication engine and related authentication operations.
pub mod auth;

/// Identity verification and proof systems
///
/// Cryptographic proof verification for decentralized identity management.
pub mod verification;
