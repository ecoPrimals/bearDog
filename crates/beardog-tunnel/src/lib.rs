// SPDX-License-Identifier: AGPL-3.0-or-later
#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    allow(
        dead_code,
        clippy::large_stack_arrays,
        clippy::bool_assert_comparison,
        clippy::bool_to_int_with_if,
        clippy::case_sensitive_file_extension_comparisons,
        clippy::collapsible_if,
        clippy::equatable_if_let,
        clippy::expect_used,
        clippy::float_cmp,
        clippy::ignored_unit_patterns,
        clippy::manual_let_else,
        clippy::needless_borrows_for_generic_args,
        clippy::needless_collect,
        clippy::or_fun_call,
        clippy::redundant_clone,
        clippy::redundant_closure_for_method_calls,
        clippy::semicolon_if_nothing_returned,
        clippy::should_implement_trait,
        clippy::single_match,
        clippy::single_match_else,
        clippy::uninlined_format_args,
        clippy::unnecessary_struct_initialization,
        clippy::unnecessary_unwrap,
        clippy::unnested_or_patterns,
        clippy::unreadable_literal,
        clippy::unwrap_used,
        clippy::useless_vec,
    )
)] // Pedantic/nursery clippy: relaxed only for `cfg(test)` builds (see workspace `Cargo.toml` lints).

//! # `BearDog` Tunnel - Secure Communication and HSM Integration
//!
//! Provides secure communication tunnels and Hardware Security Module (HSM) integration
//! for the `BearDog` ecosystem, enabling encrypted channels and cryptographic key management.
//!
//! ## Features
//!
//! - **Secure Tunnels**: Encrypted communication channels using the BSTP (`BearDog` Secure Tunnel Protocol)
//! - **HSM Integration**: Hardware security module support for `YubiKey`, TPM 2.0, and mobile secure hardware
//! - **Session Management**: Secure session lifecycle with automatic expiration and cleanup
//! - **Key Management**: Hardware-backed cryptographic operations (sign, verify, encrypt, decrypt)
//! - **Mobile HSM**: Native iOS Secure Enclave and Android `StrongBox` support
//! - **Zero-Copy Design**: Optimized for minimal memory allocation and maximum performance
//!
//! ## Core Types
//!
//! ### Session Management
//! - `SessionManager` - Manages secure session lifecycle and concurrent access
//! - `SecureSession` - Represents an active secure session with a peer
//! - [`crate::tunnel::session::SecurityGenetics`] - Security parameters for adaptive threat response
//! - [`crate::tunnel::session::GamingSecurityProfile`] - Optimized security profiles for low-latency applications
//!
//! ### Configuration
//! - `BStpConfig` - Main tunnel configuration (security level, timeouts, limits)
//! - [`crate::tunnel::config::PerformanceConfig`] - Performance tuning (latency, throughput, caching)
//! - [`crate::tunnel::config::SecurityConfig`] - Security settings (key storage, escrow thresholds)
//!
//! ### HSM Integration
//! - HSM provider abstraction for multiple backend types
//! - Hardware key generation, signing, and verification
//! - Secure key storage with hardware-backed protection
//!
//! ## Example Usage
//!
//! ### Basic Session Management
//!
//! ```no_run
//! # use beardog_tunnel::SessionManager;
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create a session manager
//! let manager = SessionManager::new();
//!
//! // Session management with secure tunnels
//! // - Hardware-backed encryption
//! // - Zero-knowledge authentication
//! // - Automatic key rotation
//! # Ok(())
//! # }
//! ```
//!
//! ### Configuring the Tunnel
//!
//! ```rust
//! use beardog_tunnel::{BStpConfig, SecurityLevel};
//!
//! // Create custom configuration
//! let mut config = BStpConfig::default();
//! config.security_level = SecurityLevel::Critical;
//! config.session_timeout_seconds = 7200; // 2 hours
//! config.max_concurrent_sessions = 5000;
//! ```
//!
//! ## Example
//!
//! ```rust
//! use beardog_tunnel::SessionManager;
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize session manager
//! let session_mgr = SessionManager::new();
//!
//! // Session manager is ready for secure sessions
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The tunnel system provides multi-layered security:
//! - **BSTP Protocol**: `BearDog` Secure Tunnel Protocol
//! - **Hardware Security**: HSM-backed key operations
//! - **Zero-Trust**: All connections verified cryptographically
//! - **Quantum-Resistant**: Post-quantum cryptography support
//!
//! ## Safety
//!
//! All tunnel operations are memory-safe with full memory safety.

/// Core tunnel and session management functionality
pub mod tunnel;

/// Production diagnostics module (FOSSIL RECORD principle)
///
/// Enable with: `cargo build --features diagnostics`
/// All diagnostic logging preserved here, not deleted.
pub mod diagnostics;

// Operational modes for UniBin architecture
pub mod modes;

// NOTE: universal_hsm module - RE-ENABLED for Phase 1.2 rebuild (Nov 7, 2025)
// NOTE: Tunnel module stabilized - proceeding with Phase 2 implementations
pub mod universal_hsm;

// BTSP handshake enforcement for socket listeners (Phase 2, BTSP_PROTOCOL_STANDARD.md)
pub mod btsp_handshake;

// BTSP Provider - internet deployment via discovered transport peer
pub mod btsp_provider;

// Generic IPC server - capability-based, primal-agnostic
pub mod ipc_server;

// Unix socket IPC server (PRIMARY inter-primal communication, JSON-RPC 2.0)
pub mod unix_socket_ipc;

// Platform-specific socket implementations (Android abstract sockets, Unix filesystem)
// Enables TRUE ecoBin v2.0 platform-agnostic IPC
pub mod platform;

// TCP IPC for universal platform support (Android, Windows, cross-device)
// Tier 2 transport when Unix sockets are not available
pub mod tcp_ipc;

// Graph Security for Collaborative Intelligence
pub mod graph_security;

// Multi-Transport Server - Universal IPC (Phase 3: Deep Debt Evolution)
// Binds ALL available transports simultaneously for universal deployment
pub mod multi_transport_server;

// Pre-dispatch capability gate for JSON-RPC methods (JH-0 ecosystem standard)
pub mod method_gate;

// Ed25519-signed ionic capability tokens (JH-1)
pub mod ionic_token;
pub mod ionic_token_handlers;

// Cross-gate trust management (trust_issuer, exchange_trust, events.poll, trusted_issuers)
pub mod trust_handlers;

// Cross-gate trusted issuer registry (Wave 135: covalent mesh trust)
pub mod trusted_issuer_registry;

// Mesh join orchestration — reciprocal trust exchange over BTSP
pub mod mesh_join;

// Auth event bus for cross-gate trust provenance (Wave 138)
pub mod auth_event_bus;

// riboCipher transport signal standard (Wave 111)
// Deterministic protocol routing via intentional signal prefix.
pub mod ribocipher;

#[cfg(test)]
mod btsp_handshake_tests;
#[cfg(test)]
mod unix_socket_ipc_btsp_tests;
#[cfg(test)]
mod unix_socket_ipc_logic_tests;
#[cfg(test)]
mod unix_socket_ipc_schema_tests;

#[cfg(test)]
mod test_helpers;

// Re-export key types
pub use tunnel::{BStpConfig, SecureSession, SecurityLevel, SessionManager};

// Re-export BTSP types for internet deployment interop
pub use btsp_provider::{
    BeardogBtspProvider, Direction, PeerInfo, SecurityContext, TunnelHandle, TunnelStatus,
};

// Re-export HSM discovery for CLI usage
pub use tunnel::hsm::{
    DiscoveredHsm, DiscoveryEngine, DiscoveryHsmHealthStatus, HsmConnectionInfo, HsmInterfaceType,
    UniversalHsmCapabilities,
};

pub use beardog_errors::BearDogError;

#[cfg(test)]
mod coverage_boost;
#[cfg(test)]
mod crypto_fault_injection_tests;
#[cfg(test)]
mod tests;
