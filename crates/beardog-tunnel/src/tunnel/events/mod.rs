// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # BSTP Events - BearDog Secure Tunnel Protocol Event System
///
/// This module defines the critical communication interface between BearDog's security layer
/// and the Songbird network layer. These events enable real-time security adaptation and
/// threat response in the forest protection network.
/// ## Forest Protection Communication Model
/// BearDog protects the digital forest through a sophisticated event-driven architecture:
/// ```text
/// ┌─────────────────┐    Events    ┌─────────────────┐
/// │  Songbird       │◄────────────►│   BearDog       │
/// │  Network Layer  │              │  Security Layer │
/// │                 │              │                 │
/// │ • Routing       │              │ • Threat Detection │
/// │ • Performance   │              │ • Access Control   │
/// │ • Optimization  │              │ • Compliance       │
/// └─────────────────┘              └─────────────────┘
/// ```
/// ## Module Structure
/// * **types**: Core event types and enumerations
/// * **network**: Network-related event definitions
/// * **security**: Security-related event definitions
/// * **threat**: Threat detection and response events
/// * **compliance**: Compliance and policy events
/// * **capabilities**: Peer capability and trust structures

// Re-export all event types and structures
pub use capabilities::*;
pub use compliance::*;
pub use network::*;
pub use security::*;
pub use threat::*;
pub use types::*;
// Module declarations
pub mod capabilities;
pub mod compliance;
pub mod network;
pub mod security;
pub mod threat;
pub mod types;
