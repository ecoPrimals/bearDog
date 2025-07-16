// 🛡️ BearDog Secure Tunnel Protocol (BSTP) Security Layer
// This module provides the security layer for gaming tunnels, working in conjunction with Songbird's network layer

/// # BearDog Secure Tunnel Protocol (BSTP) Security Layer
///
/// The BSTP security layer provides ultra-fast encryption and security management
/// for gaming tunnels with sub-100μs latency targets. This module implements:
///
/// ## Core Security Features
///
/// - **Ultra-Fast Gaming Encryption**: Sub-100μs encryption/decryption for competitive gaming
/// - **Genetic Security Adaptation**: Self-evolving security based on threat landscape
/// - **Session Management**: Secure session lifecycle with automatic key rotation
/// - **Performance Monitoring**: Real-time latency and throughput tracking
/// - **Threat Response**: Automated security adaptation to detected threats
///
/// ## Architecture
///
/// ```text
/// ┌─────────────────────────────────────────┐
/// │             SONGBIRD                    │ ← Network orchestration
/// │    (Network/Discovery/Load Balancing)  │
/// ├─────────────────────────────────────────┤
/// │          BSTP INTERFACE                 │ ← Clean API boundary
/// ├─────────────────────────────────────────┤
/// │             BEARDOG                     │ ← Security & encryption
/// │     (Security/Crypto/Compliance)       │
/// └─────────────────────────────────────────┘
/// ```
///
/// ## Module Overview
///
/// - [`security_provider`]: Core security provider interface and implementation
/// - [`session`]: Secure session management with genetic evolution
/// - [`gaming_crypto`]: Gaming-optimized encryption engine
/// - [`genetic_healing`]: Self-healing security with genetic algorithms
/// - [`performance`]: Performance monitoring and optimization
/// - [`events`]: Event system for BSTP security communications
/// - [`config`]: Configuration management for all BSTP components
/// - [`key_manager`]: Secure key generation, rotation, and management
///
/// ## Example Usage
///
/// ```rust,no_run
/// use beardog::tunnel::{BStpSecurityManager, BStpConfig, BStpKeyManager};
/// use std::sync::Arc;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Initialize configuration
///     let config = BStpConfig::competitive_gaming();
///     
///     // Create key manager
///     let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);
///     
///     // Initialize BSTP security manager
///     let security_manager = BStpSecurityManager::new(
///         encryption_engine,
///         genetics_engine,
///         key_manager,
///         config,
///         auth_engine,
///         threat_engine
///     ).await?;
///     
///     println!("🛡️ BSTP Security Layer initialized for gaming!");
///     Ok(())
/// }
/// ```
/// Security provider interface and implementation
/// Tunnel configuration management
pub mod config;
/// Event handling and processing
pub mod events;
/// Gaming cryptography implementations
pub mod gaming_crypto;
/// Genetic healing and recovery systems
pub mod genetic_healing;
/// Hardware Security Module integration
pub mod hsm;
/// Key management and lifecycle
pub mod key_manager;
/// Performance monitoring and metrics
pub mod performance;
/// Security provider implementations
pub mod security_provider;
/// Session management and lifecycle
pub mod session;

pub use config::*;
pub use events::*;
pub use gaming_crypto::*;
pub use genetic_healing::*;
pub use hsm::{types, AndroidStrongBoxHsm, HsmManager, HsmProvider, RustSoftwareHsm};
pub use key_manager::{BStpKeyManager, CryptoAlgorithm, CryptoKey, KeyRotationTask};
pub use performance::*;
pub use security_provider::*;
pub use session::*;
