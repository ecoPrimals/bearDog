// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tunnel event system
//!
//! Event types, capabilities, and compliance tracking for BearDog tunnels.

/// Tunnel capability advertisement and negotiation
pub mod capabilities;
/// Compliance event tracking
pub mod compliance;
/// Network event handling
pub mod network;
/// Security event handling
pub mod security;
/// Threat detection events
pub mod threat;
/// Common event type definitions
pub mod types;
