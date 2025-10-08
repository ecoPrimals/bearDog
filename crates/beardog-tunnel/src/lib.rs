//! # BearDog Tunnel - Secure Communication and HSM Integration
//!
//! Secure communication tunnels and Hardware Security Module (HSM) integration
//! for the BearDog ecosystem, providing encrypted channels and key management.
//!
//! ## Features
//!
//! - **Secure Tunnels**: Encrypted communication channels (BSTP protocol)
//! - **HSM Integration**: Hardware security module support (YubiKey, TPM, StrongBox)
//! - **Session Management**: Secure session lifecycle management
//! - **Key Management**: Hardware-backed cryptographic key operations
//! - **Mobile HSM**: iOS Secure Enclave and Android StrongBox support
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_tunnel::{SessionManager, BStpConfig, SecurityLevel};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize session manager
//! let config = BStpConfig::default();
//! let session_mgr = SessionManager::new(config)?;
//!
//! // Create secure session
//! let session = session_mgr.create_session(SecurityLevel::High).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The tunnel system provides multi-layered security:
//! - **BSTP Protocol**: BearDog Secure Tunnel Protocol
//! - **Hardware Security**: HSM-backed key operations
//! - **Zero-Trust**: All connections verified cryptographically
//! - **Quantum-Resistant**: Post-quantum cryptography support
//!
//! ## Safety
//!
//! All tunnel operations are memory-safe with zero unsafe code.

/// Core tunnel and session management functionality
pub mod tunnel;

// Re-export key types
pub use tunnel::{BStpConfig, SecureSession, SecurityLevel, SessionManager};

pub use beardog_errors::BearDogError;
