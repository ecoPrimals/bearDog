// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bond persistence trait and implementations.
//!
//! Abstracts durable bond storage so the `IonicBondHandler` can persist sealed
//! bonds beyond process lifetime. Two backends:
//!
//! - [`InMemoryBondPersistence`] — default; bonds lost on restart (current behavior).
//! - Future: `CapabilityDiscoveryBondPersistence` — discovers a `bonding.ledger.store`
//!   capable provider (NestGate/loamSpine) at runtime and persists via JSON-RPC.

use async_trait::async_trait;
use beardog_types::ionic_bond::IonicBond;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Durable bond storage abstraction.
///
/// `IonicBondHandler` calls these methods on seal/revoke/list to persist
/// bond state. The in-memory implementation is the default; production
/// NUCLEUS deployments should discover a `bonding.ledger.*` provider.
#[async_trait]
pub trait BondPersistence: Send + Sync {
    /// Persist or overwrite a bond record (e.g. after seal).
    async fn store(&self, bond: &IonicBond) -> Result<(), String>;
    /// Load a single bond by id, if present.
    async fn retrieve(&self, bond_id: &str) -> Result<Option<IonicBond>, String>;
    /// Enumerate all stored bonds.
    async fn list(&self) -> Result<Vec<IonicBond>, String>;
    /// Delete a bond record (e.g. after revoke).
    async fn remove(&self, bond_id: &str) -> Result<bool, String>;
}

/// In-memory bond persistence (process-scoped, non-durable).
#[derive(Default)]
pub struct InMemoryBondPersistence {
    bonds: RwLock<HashMap<String, IonicBond>>,
}

#[async_trait]
impl BondPersistence for InMemoryBondPersistence {
    async fn store(&self, bond: &IonicBond) -> Result<(), String> {
        self.bonds
            .write()
            .await
            .insert(bond.bond_id.clone(), bond.clone());
        Ok(())
    }

    async fn retrieve(&self, bond_id: &str) -> Result<Option<IonicBond>, String> {
        Ok(self.bonds.read().await.get(bond_id).cloned())
    }

    async fn list(&self) -> Result<Vec<IonicBond>, String> {
        Ok(self.bonds.read().await.values().cloned().collect())
    }

    async fn remove(&self, bond_id: &str) -> Result<bool, String> {
        Ok(self.bonds.write().await.remove(bond_id).is_some())
    }
}
