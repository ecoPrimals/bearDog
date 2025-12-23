//! Universal HSM Entropy Orchestrator
//!
//! This module connects BearDog's entropy hierarchy with all available
//! hardware security modules (FIDO2, Android StrongBox, iOS Secure Enclave).
//!
//! # Architecture
//!
//! ```text
//! Human Identity
//!      ↓
//! Device Selection (iPhone/Pixel/Security Key)
//!      ↓
//! HSM Entropy Provider (hardware RNG)
//!      ↓
//! Entropy Mixing Engine
//!      ↓
//! Entropy Classification (Tier 1/2/3)
//!      ↓
//! Entropy Hierarchy Manager
//!      ↓
//! BearDog Applications (AI/Crypto/Network/etc.)
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_security::hsm::entropy_orchestrator::HsmEntropyOrchestrator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), beardog_errors::BearDogError> {
//!     // Initialize and discover all available HSMs
//!     let mut orchestrator = HsmEntropyOrchestrator::new().await?;
//!     
//!     // List available devices
//!     let devices = orchestrator.list_available_devices().await;
//!     println!("Found {} HSM device(s)", devices.len());
//!     
//!     // Generate human entropy from best available device
//!     let seed_id = orchestrator.generate_human_entropy(256, None).await?;
//!     
//!     println!("Generated entropy seed: {}", seed_id);
//!     Ok(())
//! }
//! ```

pub mod orchestrator;
pub mod types;

pub use orchestrator::HsmEntropyOrchestrator;
pub use types::{
    EntropyGenerationRequest, EntropyGenerationResult, HsmDeviceInfo, HsmDeviceType,
    HumanEntropyInput,
};
