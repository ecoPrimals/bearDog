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
pub mod security_provider;

/// Secure session management with genetic evolution
pub mod session;

/// Gaming-optimized encryption engine with sub-100μs performance
pub mod gaming_crypto;

/// Self-healing security with genetic algorithms
pub mod genetic_healing;

/// Performance monitoring and latency optimization
pub mod performance;

/// Event system for BSTP security communications
pub mod events;

/// Configuration management for all BSTP components
pub mod config;

/// Secure key generation, rotation, and management
pub mod key_manager;

// Re-export core types
pub use config::{
    BStpConfig, GamingConfig, GeneticHealingConfig, KeyManagementConfig, PerformanceConfig,
};
pub use events::{
    NetworkSecurityEvent, SecurityEvolution, SecurityLevel, SecurityMetrics, SecurityNetworkEvent,
    SecurityResponse,
};
pub use gaming_crypto::{CryptoChoice, EncryptedPacket, GamingCryptoEngine};
pub use genetic_healing::{GeneticSecurityHealing, HealingResult, SecurityIssue};
pub use key_manager::{BStpKeyManager, CryptoAlgorithm, CryptoKey, KeyRotationTask};
pub use performance::{BStpPerformanceTargets, GamingSecurityProfile, LatencyMonitor};
pub use security_provider::{
    BStpSecurityManager, BStpSecurityProvider, TrustLevel, VerificationResult,
};
pub use session::{SecureSession, SecurityGenetics, SessionManager};
