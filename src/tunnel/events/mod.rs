//! # BSTP Events - BearDog Secure Tunnel Protocol Event System
//!
//! This module defines the critical communication interface between BearDog's security layer
//! and the Songbird network layer. These events enable real-time security adaptation and
//! threat response in the forest protection network.
//!
//! ## Forest Protection Communication Model
//!
//! BearDog protects the digital forest through a sophisticated event-driven architecture:
//!
//! ```text
//! ┌─────────────────┐    Events    ┌─────────────────┐
//! │  Songbird       │◄────────────►│   BearDog       │
//! │  Network Layer  │              │  Security Layer │
//! │                 │              │                 │
//! │ • Routing       │              │ • Threat Detection │
//! │ • Performance   │              │ • Access Control   │
//! │ • Optimization  │              │ • Compliance       │
//! └─────────────────┘              └─────────────────┘
//! ```
//!
//! ## Module Structure
//!
//! * **types**: Core event types and enumerations
//! * **network**: Network-related event definitions
//! * **security**: Security-related event definitions
//! * **threat**: Threat detection and response events
//! * **compliance**: Compliance and policy events
//! * **capabilities**: Peer capability and trust structures

// Re-export all event types and structures
pub use network::*;
pub use security::*;
pub use threat::*;
pub use compliance::*;
pub use capabilities::*;
pub use types::*;

// Module declarations
pub mod network;
pub mod security;
pub mod threat;
pub mod compliance;
pub mod capabilities;
pub mod types; 