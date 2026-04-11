// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability introspection and wire-format mapping.

use serde::{Deserialize, Serialize};
use tracing::debug;

/// Simplified capability enumeration for the self-knowledge pattern.
///
/// The full capability system is in `beardog-capabilities`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleCapability {
    /// Secure tunneling capability.
    SecureTunneling,
    /// Genetic lineage generation/verification.
    GeneticLineage,
    /// General cryptographic operations.
    Cryptography,
    /// HSM integration.
    HsmIntegration,
    /// Discovery services.
    Discovery,
}

/// Discover this primal's capabilities from feature flags.
pub fn discover_capabilities() -> Vec<SimpleCapability> {
    let capabilities: Vec<SimpleCapability> = [
        Some(SimpleCapability::SecureTunneling),
        Some(SimpleCapability::GeneticLineage),
        Some(SimpleCapability::Cryptography),
        cfg!(feature = "hsm-integration").then(|| SimpleCapability::HsmIntegration),
        cfg!(feature = "mdns").then(|| SimpleCapability::Discovery),
    ]
    .into_iter()
    .flatten()
    .collect();

    debug!("Discovered capabilities: {:?}", capabilities);
    capabilities
}

/// Same capability list as `discover_capabilities`, exposed for IPC
/// helpers (e.g. wateringHole v3.1 capability-domain symlinks).
#[must_use]
pub fn discovered_simple_capabilities() -> Vec<SimpleCapability> {
    discover_capabilities()
}

/// Wire-format capability tags for JSON-RPC `ipc.register`.
///
/// Derived from [`SimpleCapability`] only — no fixed primal-specific lists.
#[must_use]
pub fn ipc_registry_capability_strings(caps: &[SimpleCapability]) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut set: BTreeSet<String> = BTreeSet::new();
    for cap in caps {
        match cap {
            SimpleCapability::SecureTunneling => {
                set.insert("btsp".to_string());
            }
            SimpleCapability::GeneticLineage => {
                set.insert("genetics".to_string());
            }
            SimpleCapability::Cryptography => {
                set.insert("crypto".to_string());
                set.insert("ed25519".to_string());
                set.insert("x25519".to_string());
                set.insert("chacha20poly1305".to_string());
                set.insert("aesgcm".to_string());
            }
            SimpleCapability::HsmIntegration => {
                set.insert("hsm".to_string());
            }
            SimpleCapability::Discovery => {
                set.insert("discovery".to_string());
            }
        }
    }
    set.into_iter().collect()
}
