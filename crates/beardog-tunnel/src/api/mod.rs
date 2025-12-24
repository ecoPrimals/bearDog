//! BearDog Unified API Server
//!
//! Modern, capability-based HTTP API exposing:
//! - BTSP (Secure Tunnels)
//! - Genesis (Physical Bootstrap)
//! - BirdSong (Privacy-Preserving Broadcasts)
//! - Lineage (Cryptographic Proofs)
//!
//! Multi-primal coordination:
//! - UPA (Universal Port Authority) client for Songbird integration

pub mod birdsong;
pub mod btsp;
pub mod genesis;
pub mod lineage;
pub mod server;
pub mod types;
pub mod upa_client;

pub use server::{BearDogApiServer, BearDogApiServerConfig};
pub use types::*;
pub use upa_client::{UpaClient, UpaClientConfig};
