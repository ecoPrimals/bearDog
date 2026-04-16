// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bond persistence trait and implementations.
//!
//! Abstracts durable bond storage so the `IonicBondHandler` can persist sealed
//! bonds beyond process lifetime. Three backends:
//!
//! - [`InMemoryBondPersistence`] — default; bonds lost on restart.
//! - [`CapabilityDiscoveryBondPersistence`] — discovers a `bonding.ledger`
//!   capable provider (NestGate/loamSpine) at runtime and delegates via JSON-RPC.
//! - Any custom `BondPersistence` impl via [`super::IonicBondHandler::with_persistence`].

use async_trait::async_trait;
use beardog_ipc::OrchestratorRegistryClient;
use beardog_types::ionic_bond::IonicBond;
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, warn};

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

// ── Capability-discovery bond persistence ──────────────────────────────

/// Discovers a `bonding.ledger` provider via the IPC capability registry
/// and delegates bond storage via JSON-RPC to that provider's socket.
///
/// Falls back to in-memory storage when no provider is discovered or when
/// the remote call fails (log + degrade gracefully).
pub struct CapabilityDiscoveryBondPersistence {
    registry_client: OrchestratorRegistryClient,
    fallback: InMemoryBondPersistence,
    request_id: AtomicU64,
    /// Cached endpoint from last successful discovery.
    cached_endpoint: RwLock<Option<String>>,
}

impl CapabilityDiscoveryBondPersistence {
    /// Create with an existing registry client (shares the orchestrator connection).
    #[must_use]
    pub fn new(registry_client: OrchestratorRegistryClient) -> Self {
        Self {
            registry_client,
            fallback: InMemoryBondPersistence::default(),
            request_id: AtomicU64::new(1),
            cached_endpoint: RwLock::new(None),
        }
    }

    /// Discover (or return cached) the `bonding.ledger` provider endpoint.
    async fn resolve_endpoint(&self) -> Option<String> {
        {
            let cached = self.cached_endpoint.read().await;
            if cached.is_some() {
                return cached.clone();
            }
        }

        match self.registry_client.find_capability("bonding.ledger").await {
            Ok(services) if !services.is_empty() => {
                let endpoint = services[0].endpoint.clone();
                debug!(endpoint = %endpoint, "Discovered bonding.ledger provider");
                *self.cached_endpoint.write().await = Some(endpoint.clone());
                Some(endpoint)
            }
            Ok(_) => {
                debug!("No bonding.ledger provider registered — using in-memory fallback");
                None
            }
            Err(e) => {
                warn!(error = %e, "Failed to discover bonding.ledger provider — using fallback");
                None
            }
        }
    }

    /// Send a JSON-RPC request to the ledger provider and return the result.
    async fn rpc_call(
        &self,
        endpoint: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });

        let request_bytes = serde_json::to_vec(&request).map_err(|e| format!("serialize: {e}"))?;

        let mut stream = tokio::net::UnixStream::connect(endpoint)
            .await
            .map_err(|e| format!("connect to {endpoint}: {e}"))?;

        stream
            .write_all(&request_bytes)
            .await
            .map_err(|e| format!("write: {e}"))?;
        stream
            .write_all(b"\n")
            .await
            .map_err(|e| format!("write newline: {e}"))?;

        let mut buf = vec![0u8; 16384];
        let n = tokio::time::timeout(std::time::Duration::from_secs(10), stream.read(&mut buf))
            .await
            .map_err(|_| "ledger RPC timed out".to_string())?
            .map_err(|e| format!("read: {e}"))?;

        let resp: serde_json::Value =
            serde_json::from_slice(&buf[..n]).map_err(|e| format!("deserialize: {e}"))?;

        if let Some(error) = resp.get("error") {
            return Err(format!("ledger RPC error: {error}"));
        }

        resp.get("result")
            .cloned()
            .ok_or_else(|| "ledger RPC: missing result".to_string())
    }
}

#[async_trait]
impl BondPersistence for CapabilityDiscoveryBondPersistence {
    async fn store(&self, bond: &IonicBond) -> Result<(), String> {
        let Some(endpoint) = self.resolve_endpoint().await else {
            return self.fallback.store(bond).await;
        };

        let bond_json = serde_json::to_value(bond).map_err(|e| format!("serialize bond: {e}"))?;

        match self
            .rpc_call(
                &endpoint,
                "bonding.ledger.store",
                json!({ "bond": bond_json }),
            )
            .await
        {
            Ok(_) => {
                self.fallback.store(bond).await?;
                Ok(())
            }
            Err(e) => {
                warn!(error = %e, "Ledger store failed — persisting in-memory only");
                self.fallback.store(bond).await
            }
        }
    }

    async fn retrieve(&self, bond_id: &str) -> Result<Option<IonicBond>, String> {
        let Some(endpoint) = self.resolve_endpoint().await else {
            return self.fallback.retrieve(bond_id).await;
        };

        match self
            .rpc_call(
                &endpoint,
                "bonding.ledger.retrieve",
                json!({ "bond_id": bond_id }),
            )
            .await
        {
            Ok(result) => {
                if result.is_null() {
                    return self.fallback.retrieve(bond_id).await;
                }
                let bond: IonicBond =
                    serde_json::from_value(result).map_err(|e| format!("deserialize bond: {e}"))?;
                Ok(Some(bond))
            }
            Err(e) => {
                warn!(error = %e, "Ledger retrieve failed — checking in-memory");
                self.fallback.retrieve(bond_id).await
            }
        }
    }

    async fn list(&self) -> Result<Vec<IonicBond>, String> {
        let Some(endpoint) = self.resolve_endpoint().await else {
            return self.fallback.list().await;
        };

        match self
            .rpc_call(&endpoint, "bonding.ledger.list", json!({}))
            .await
        {
            Ok(result) => {
                let remote_bonds: Vec<IonicBond> =
                    serde_json::from_value(result).unwrap_or_default();
                let local_bonds = self.fallback.list().await?;

                let mut merged: HashMap<String, IonicBond> = remote_bonds
                    .into_iter()
                    .map(|b| (b.bond_id.clone(), b))
                    .collect();
                for b in local_bonds {
                    merged.entry(b.bond_id.clone()).or_insert(b);
                }
                Ok(merged.into_values().collect())
            }
            Err(e) => {
                warn!(error = %e, "Ledger list failed — returning in-memory only");
                self.fallback.list().await
            }
        }
    }

    async fn remove(&self, bond_id: &str) -> Result<bool, String> {
        let local_removed = self.fallback.remove(bond_id).await?;

        let Some(endpoint) = self.resolve_endpoint().await else {
            return Ok(local_removed);
        };

        match self
            .rpc_call(
                &endpoint,
                "bonding.ledger.remove",
                json!({ "bond_id": bond_id }),
            )
            .await
        {
            Ok(result) => {
                let remote_removed = result.as_bool().unwrap_or(false);
                Ok(local_removed || remote_removed)
            }
            Err(e) => {
                warn!(error = %e, "Ledger remove failed — local removal only");
                Ok(local_removed)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::ionic_bond::{BondState, BondTrustModel, EncryptionTier};

    fn test_bond(id: &str) -> IonicBond {
        IonicBond {
            bond_id: id.to_string(),
            proposal_id: format!("prop-{id}"),
            terms_hash: "abc123".to_string(),
            proposer: "alice".to_string(),
            acceptor: "bob".to_string(),
            trust_model: BondTrustModel::BtspEnforced,
            encryption_tier: EncryptionTier::Aead,
            state: BondState::Active,
            allowed_capabilities: vec!["crypto".to_string()],
            proposer_signature: None,
            proposer_public_key: None,
            acceptor_signature: None,
            acceptor_public_key: None,
            created_at: "2026-04-15T00:00:00Z".to_string(),
            expires_at: None,
        }
    }

    #[tokio::test]
    async fn in_memory_roundtrip() {
        let p = InMemoryBondPersistence::default();
        let bond = test_bond("test-1");
        p.store(&bond).await.unwrap();
        assert!(p.retrieve("test-1").await.unwrap().is_some());
        assert_eq!(p.list().await.unwrap().len(), 1);
        assert!(p.remove("test-1").await.unwrap());
        assert!(p.retrieve("test-1").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn capability_discovery_fallback_when_no_registry() {
        let client = OrchestratorRegistryClient::new();
        let persistence = CapabilityDiscoveryBondPersistence::new(client);

        let bond = test_bond("fallback-1");
        persistence.store(&bond).await.unwrap();
        assert!(persistence.retrieve("fallback-1").await.unwrap().is_some());
        assert_eq!(persistence.list().await.unwrap().len(), 1);
        assert!(persistence.remove("fallback-1").await.unwrap());
        assert!(persistence.retrieve("fallback-1").await.unwrap().is_none());
    }
}
