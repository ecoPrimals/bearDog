//! # Biome Discovery Module
//!
//! Provides comprehensive biome (network environment) discovery and assessment
//! capabilities for understanding the operational context of BearDog nodes.
//!
//! ## Overview
//!
//! Biome discovery automatically detects and evaluates the network environment,
//! security posture, and available resources. This enables BearDog to:
//! - Adapt to different deployment environments
//! - Assess trust levels of discovered services
//! - Register with appropriate discovery mechanisms
//! - Scan and map network topology
//!
//! ## Key Components
//!
//! - [`DiscoveryEngine`] - Core discovery orchestration
//! - [`AssessmentEngine`] - Environment and security assessment
//! - [`RegistrationManager`] - Service registration coordination
//! - [`NetworkScanner`] - Network topology discovery
//! - [`TrustEvaluator`] - Trust level evaluation
//! - [`BiomeDiscoveryProtocol`] - Discovery protocol implementation
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::biome_discovery::{DiscoveryEngine, BiomeDiscoveryProtocol};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize discovery engine
//! let engine = DiscoveryEngine::new();
//!
//! // Discover biome characteristics
//! let biome = engine.discover_biome().await?;
//! println!("Discovered biome: {:?}", biome);
//! # Ok(())
//! # }
//! ```

/// Biome discovery types and data structures
pub mod types;

/// Core discovery engine for biome detection
pub mod discovery_engine;

/// Environment and security assessment
pub mod assessment_engine;

/// Service registration management
pub mod registration_manager;

/// Network topology scanning
pub mod network_scanner;

/// Trust level evaluation for discovered services
pub mod trust_evaluator;

/// Biome discovery protocol implementation
pub mod protocol;

pub use types::*;
pub use discovery_engine::DiscoveryEngine;
pub use assessment_engine::AssessmentEngine;
pub use registration_manager::RegistrationManager;
pub use network_scanner::NetworkScanner;
pub use trust_evaluator::TrustEvaluator;
pub use protocol::BiomeDiscoveryProtocol; 
