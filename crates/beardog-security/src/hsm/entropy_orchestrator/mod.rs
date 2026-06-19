// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Entropy Orchestrator
//!
//! This module connects BearDog's entropy hierarchy with all available
//! hardware security modules (FIDO2, Android StrongBox, iOS Secure Enclave).
//!
//! ## Current behavior (Phase 1)
//!
//! Device discovery and selection run against real HSM providers, but entropy
//! bytes are currently drawn from the operating-system CSPRNG (`rand` OS RNG) as
//! a **software fallback**. Results are labeled honestly with `source: "os_rng"`,
//! `device_used: "os_rng_fallback"`, and `hardware_backed: false`.
//!
//! ## Future (Phase 2)
//!
//! When FIDO2, Android StrongBox, and iOS Secure Enclave providers are wired,
//! `generate_from_hsm()` will request hardware RNG from the selected device and
//! report hardware tier/quality metadata only when that path succeeds.
//!
//! # Architecture
//!
//! ```text
//! Human Identity
//!      ↓
//! Device Selection (iPhone/Pixel/Security Key)
//!      ↓
//! Entropy Provider (OS RNG fallback today; hardware RNG in Phase 2)
//!      ↓
//! Entropy Mixing Engine
//!      ↓
//! Entropy Classification (Tier 0 fallback / Tier 1-3 hardware)
//!      ↓
//! Entropy Hierarchy Manager
//!      ↓
//! BearDog Applications (AI/Crypto/Network/etc.)
//! ```
//!
//! # Example
//!
//! ```rust,ignore
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
