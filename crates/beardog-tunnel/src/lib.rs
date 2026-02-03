//! # BearDog Tunnel - Secure Communication and HSM Integration
//!
//! Provides secure communication tunnels and Hardware Security Module (HSM) integration
//! for the BearDog ecosystem, enabling encrypted channels and cryptographic key management.

#![deny(unsafe_code)]
#![warn(missing_docs)]
// Production code must use proper error handling - deny panicking methods
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// Allow expect in tests - test panics are appropriate failure modes
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
//!
//! ## Features
//!
//! - **Secure Tunnels**: Encrypted communication channels using the BSTP (BearDog Secure Tunnel Protocol)
//! - **HSM Integration**: Hardware security module support for YubiKey, TPM 2.0, and mobile secure hardware
//! - **Session Management**: Secure session lifecycle with automatic expiration and cleanup
//! - **Key Management**: Hardware-backed cryptographic operations (sign, verify, encrypt, decrypt)
//! - **Mobile HSM**: Native iOS Secure Enclave and Android StrongBox support
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

#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

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
//! All tunnel operations are memory-safe with zero unsafe code.

/// Core tunnel and session management functionality
pub mod tunnel;

/// Production diagnostics module (FOSSIL RECORD principle)
///
/// Enable with: `cargo build --features diagnostics`
/// All diagnostic logging preserved here, not deleted.
pub mod diagnostics;

// Operational modes for UniBin architecture
pub mod modes;

// Simple HSM client for CLI usage (PKCS#11 - optional)
// Disabled until module is created
// #[cfg(feature = "pkcs11")]
// pub mod simple_hsm_client;

// NOTE: universal_hsm module - RE-ENABLED for Phase 1.2 rebuild (Nov 7, 2025)
// NOTE: Tunnel module stabilized - proceeding with Phase 2 implementations
pub mod universal_hsm;

// BTSP Provider - Songbird integration for internet deployment
pub mod btsp_provider;

// Generic IPC server - capability-based, primal-agnostic
pub mod ipc_server;

// Unix socket IPC server (PRIMARY inter-primal communication)
// tarpc_service removed - JSON-RPC first architecture (see TARPC_REMOVAL_RATIONALE_JAN_29_2026.md)
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

#[cfg(test)]
mod unix_socket_ipc_btsp_tests;
#[cfg(test)]
mod unix_socket_ipc_logic_tests;
#[cfg(test)]
mod unix_socket_ipc_schema_tests;

#[cfg(test)]
mod test_helpers;

// Re-export key types
// #[cfg(feature = "pkcs11")]
// pub use simple_hsm_client::SimplePkcs11Client;
pub use tunnel::{BStpConfig, SecureSession, SecurityLevel, SessionManager};

// Re-export BTSP types for Songbird integration
pub use btsp_provider::{
    BeardogBtspProvider, BtspProvider, Direction, PeerInfo, SecurityContext, TunnelHandle,
    TunnelStatus,
};

// Re-export HSM discovery for CLI usage
pub use tunnel::hsm::{
    DiscoveredHsm, DiscoveryEngine, DiscoveryHsmHealthStatus, HsmConnectionInfo, HsmInterfaceType,
    UniversalHsmCapabilities,
};

pub use beardog_errors::BearDogError;

#[cfg(test)]
mod tests;
