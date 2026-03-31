// SPDX-License-Identifier: AGPL-3.0-only

//! # Ecosystem Discovery Adapter
//!
//! Bridges between `EcosystemListener` and `PrimalDiscoveryService` trait.
//! Enables the CLI to use real ecosystem discovery infrastructure.
//!
//! ## Modern Rust Patterns
//! - Uses `Arc<T>` for shared ownership (idiomatic async)
//! - Leverages `tokio::sync::RwLock` for async-friendly concurrency
//! - Employs `?` operator for error propagation (no unwraps)
//! - Zero-copy where possible with borrowed slices

use beardog_core::ecosystem::primal_types::DiscoveredPrimal;
use beardog_core::zero_knowledge_bootstrap::ecosystem_listener::EcosystemListener;
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod adapter;
mod discovery;
mod endpoint;
mod listener;
mod rpc;

#[cfg(test)]
mod tests;

/// Adapter that implements `PrimalDiscoveryService` using real ecosystem discovery
///
/// This adapter provides a simplified interface for CLI usage, wrapping the
/// `EcosystemListener` infrastructure for primal discovery.
///
/// ## Zero-Copy Design
/// Where possible, this adapter uses `Cow<'_, str>` and borrowed slices to avoid
/// unnecessary allocations in hot paths.
#[derive(Debug)]
pub struct EcosystemDiscoveryAdapter {
    /// Shared reference to discovered primals from `EcosystemListener`
    pub(crate) discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    /// Shared reference to discovered capabilities
    pub(crate) discovered_capabilities:
        Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
    /// Optional ecosystem listener (lazily initialized)
    pub(crate) listener: Arc<RwLock<Option<EcosystemListener>>>,
    /// Bootstrap configuration
    pub(crate) config: UnifiedBootstrapConfig,
}
