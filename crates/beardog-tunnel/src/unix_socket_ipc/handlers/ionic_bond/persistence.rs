// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bond persistence trait and implementations.
//!
//! Abstracts durable bond storage so the `IonicBondHandler` can persist sealed
//! bonds beyond process lifetime. Three backends:
//!
//! - [`InMemoryBondPersistence`] — default; bonds lost on restart.
//! - [`CapabilityDiscoveryBondPersistence`] — discovers a `bonding.ledger`
//!   capable provider (NestGate/loamSpine) at runtime and delegates via JSON-RPC.
//! - [`BondPersistenceBackend`] — enum dispatch over the built-in implementations.

use std::future::Future;

use beardog_ipc::OrchestratorRegistryClient;
use beardog_types::ionic_bond::IonicBond;
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Typed error for bond persistence operations.
#[derive(Debug)]
pub enum BondPersistenceError {
    /// JSON serialization/deserialization failure.
    Serialization {
        /// What operation was being attempted.
        context: &'static str,
        /// Underlying serde error.
        source: serde_json::Error,
    },
    /// I/O error communicating with a ledger provider.
    Io {
        /// What I/O operation failed.
        context: &'static str,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// Ledger RPC timed out.
    RpcTimeout,
    /// Ledger returned a JSON-RPC error object.
    RpcError(String),
    /// Ledger response was structurally invalid (e.g. missing `result` field).
    InvalidResponse(&'static str),
}

impl std::fmt::Display for BondPersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization { context, source } => write!(f, "{context}: {source}"),
            Self::Io { context, source } => write!(f, "{context}: {source}"),
            Self::RpcTimeout => write!(f, "ledger RPC timed out"),
            Self::RpcError(msg) => write!(f, "ledger RPC error: {msg}"),
            Self::InvalidResponse(msg) => write!(f, "ledger RPC: {msg}"),
        }
    }
}

impl std::error::Error for BondPersistenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization { source, .. } => Some(source),
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Convenience alias for bond persistence results.
pub type BondPersistenceResult<T> = Result<T, BondPersistenceError>;

/// Durable bond storage abstraction.
///
/// `IonicBondHandler` calls these methods on seal/revoke/list to persist
/// bond state. The in-memory implementation is the default; production
/// NUCLEUS deployments should discover a `bonding.ledger.*` provider.
pub trait BondPersistence: Send + Sync {
    /// Persist or overwrite a bond record (e.g. after seal).
    fn store(&self, bond: &IonicBond) -> impl Future<Output = BondPersistenceResult<()>> + Send;
    /// Load a single bond by id, if present.
    fn retrieve(
        &self,
        bond_id: &str,
    ) -> impl Future<Output = BondPersistenceResult<Option<IonicBond>>> + Send;
    /// Enumerate all stored bonds.
    fn list(&self) -> impl Future<Output = BondPersistenceResult<Vec<IonicBond>>> + Send;
    /// Delete a bond record (e.g. after revoke).
    fn remove(&self, bond_id: &str) -> impl Future<Output = BondPersistenceResult<bool>> + Send;
}

/// Enum dispatch for [`BondPersistence`].
pub enum BondPersistenceBackend {
    /// Ephemeral in-process store (lost on restart).
    InMemory(InMemoryBondPersistence),
    /// Delegates to a `bonding.ledger` provider discovered at runtime.
    CapabilityDiscovery(CapabilityDiscoveryBondPersistence),
}

async fn bond_backend_store(
    backend: &BondPersistenceBackend,
    bond: &IonicBond,
) -> BondPersistenceResult<()> {
    match backend {
        BondPersistenceBackend::InMemory(p) => p.store(bond).await,
        BondPersistenceBackend::CapabilityDiscovery(p) => p.store(bond).await,
    }
}

async fn bond_backend_retrieve(
    backend: &BondPersistenceBackend,
    bond_id: &str,
) -> BondPersistenceResult<Option<IonicBond>> {
    match backend {
        BondPersistenceBackend::InMemory(p) => p.retrieve(bond_id).await,
        BondPersistenceBackend::CapabilityDiscovery(p) => p.retrieve(bond_id).await,
    }
}

async fn bond_backend_list(
    backend: &BondPersistenceBackend,
) -> BondPersistenceResult<Vec<IonicBond>> {
    match backend {
        BondPersistenceBackend::InMemory(p) => p.list().await,
        BondPersistenceBackend::CapabilityDiscovery(p) => p.list().await,
    }
}

async fn bond_backend_remove(
    backend: &BondPersistenceBackend,
    bond_id: &str,
) -> BondPersistenceResult<bool> {
    match backend {
        BondPersistenceBackend::InMemory(p) => p.remove(bond_id).await,
        BondPersistenceBackend::CapabilityDiscovery(p) => p.remove(bond_id).await,
    }
}

impl BondPersistence for BondPersistenceBackend {
    fn store(&self, bond: &IonicBond) -> impl Future<Output = BondPersistenceResult<()>> + Send {
        bond_backend_store(self, bond)
    }

    fn retrieve(
        &self,
        bond_id: &str,
    ) -> impl Future<Output = BondPersistenceResult<Option<IonicBond>>> + Send {
        bond_backend_retrieve(self, bond_id)
    }

    fn list(&self) -> impl Future<Output = BondPersistenceResult<Vec<IonicBond>>> + Send {
        bond_backend_list(self)
    }

    fn remove(&self, bond_id: &str) -> impl Future<Output = BondPersistenceResult<bool>> + Send {
        bond_backend_remove(self, bond_id)
    }
}

/// In-memory bond persistence (process-scoped, non-durable).
#[derive(Default)]
pub struct InMemoryBondPersistence {
    bonds: RwLock<HashMap<String, IonicBond>>,
}

async fn inmemory_store(
    this: &InMemoryBondPersistence,
    bond: &IonicBond,
) -> BondPersistenceResult<()> {
    this.bonds
        .write()
        .await
        .insert(bond.bond_id.clone(), bond.clone());
    Ok(())
}

async fn inmemory_retrieve(
    this: &InMemoryBondPersistence,
    bond_id: &str,
) -> BondPersistenceResult<Option<IonicBond>> {
    Ok(this.bonds.read().await.get(bond_id).cloned())
}

async fn inmemory_list(this: &InMemoryBondPersistence) -> BondPersistenceResult<Vec<IonicBond>> {
    Ok(this.bonds.read().await.values().cloned().collect())
}

async fn inmemory_remove(
    this: &InMemoryBondPersistence,
    bond_id: &str,
) -> BondPersistenceResult<bool> {
    Ok(this.bonds.write().await.remove(bond_id).is_some())
}

impl BondPersistence for InMemoryBondPersistence {
    fn store(&self, bond: &IonicBond) -> impl Future<Output = BondPersistenceResult<()>> + Send {
        inmemory_store(self, bond)
    }

    fn retrieve(
        &self,
        bond_id: &str,
    ) -> impl Future<Output = BondPersistenceResult<Option<IonicBond>>> + Send {
        inmemory_retrieve(self, bond_id)
    }

    fn list(&self) -> impl Future<Output = BondPersistenceResult<Vec<IonicBond>>> + Send {
        inmemory_list(self)
    }

    fn remove(&self, bond_id: &str) -> impl Future<Output = BondPersistenceResult<bool>> + Send {
        inmemory_remove(self, bond_id)
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
    ) -> BondPersistenceResult<serde_json::Value> {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });

        let request_bytes =
            serde_json::to_vec(&request).map_err(|e| BondPersistenceError::Serialization {
                context: "serialize request",
                source: e,
            })?;

        let mut stream = tokio::net::UnixStream::connect(endpoint)
            .await
            .map_err(|e| BondPersistenceError::Io {
                context: "connect to ledger",
                source: e,
            })?;

        stream
            .write_all(&request_bytes)
            .await
            .map_err(|e| BondPersistenceError::Io {
                context: "write request",
                source: e,
            })?;
        stream
            .write_all(b"\n")
            .await
            .map_err(|e| BondPersistenceError::Io {
                context: "write newline",
                source: e,
            })?;

        let mut buf = vec![0u8; 16384];
        let n = tokio::time::timeout(std::time::Duration::from_secs(10), stream.read(&mut buf))
            .await
            .map_err(|_| BondPersistenceError::RpcTimeout)?
            .map_err(|e| BondPersistenceError::Io {
                context: "read response",
                source: e,
            })?;

        let resp: serde_json::Value =
            serde_json::from_slice(&buf[..n]).map_err(|e| BondPersistenceError::Serialization {
                context: "deserialize response",
                source: e,
            })?;

        if let Some(error) = resp.get("error") {
            return Err(BondPersistenceError::RpcError(error.to_string()));
        }

        resp.get("result")
            .cloned()
            .ok_or(BondPersistenceError::InvalidResponse("missing result"))
    }
}

async fn capability_store(
    this: &CapabilityDiscoveryBondPersistence,
    bond: &IonicBond,
) -> BondPersistenceResult<()> {
    let Some(endpoint) = this.resolve_endpoint().await else {
        return this.fallback.store(bond).await;
    };

    let bond_json =
        serde_json::to_value(bond).map_err(|e| BondPersistenceError::Serialization {
            context: "serialize bond",
            source: e,
        })?;

    match this
        .rpc_call(
            &endpoint,
            "bonding.ledger.store",
            json!({ "bond": bond_json }),
        )
        .await
    {
        Ok(_) => {
            this.fallback.store(bond).await?;
            Ok(())
        }
        Err(e) => {
            warn!(error = %e, "Ledger store failed — persisting in-memory only");
            this.fallback.store(bond).await
        }
    }
}

async fn capability_retrieve(
    this: &CapabilityDiscoveryBondPersistence,
    bond_id: &str,
) -> BondPersistenceResult<Option<IonicBond>> {
    let Some(endpoint) = this.resolve_endpoint().await else {
        return this.fallback.retrieve(bond_id).await;
    };

    match this
        .rpc_call(
            &endpoint,
            "bonding.ledger.retrieve",
            json!({ "bond_id": bond_id }),
        )
        .await
    {
        Ok(result) => {
            if result.is_null() {
                return this.fallback.retrieve(bond_id).await;
            }
            let bond: IonicBond = serde_json::from_value(result).map_err(|e| {
                BondPersistenceError::Serialization {
                    context: "deserialize bond",
                    source: e,
                }
            })?;
            Ok(Some(bond))
        }
        Err(e) => {
            warn!(error = %e, "Ledger retrieve failed — checking in-memory");
            this.fallback.retrieve(bond_id).await
        }
    }
}

async fn capability_list(
    this: &CapabilityDiscoveryBondPersistence,
) -> BondPersistenceResult<Vec<IonicBond>> {
    let Some(endpoint) = this.resolve_endpoint().await else {
        return this.fallback.list().await;
    };

    match this
        .rpc_call(&endpoint, "bonding.ledger.list", json!({}))
        .await
    {
        Ok(result) => {
            let remote_bonds: Vec<IonicBond> = serde_json::from_value(result).unwrap_or_default();
            let local_bonds = this.fallback.list().await?;

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
            this.fallback.list().await
        }
    }
}

async fn capability_remove(
    this: &CapabilityDiscoveryBondPersistence,
    bond_id: &str,
) -> BondPersistenceResult<bool> {
    let local_removed = this.fallback.remove(bond_id).await?;

    let Some(endpoint) = this.resolve_endpoint().await else {
        return Ok(local_removed);
    };

    match this
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

impl BondPersistence for CapabilityDiscoveryBondPersistence {
    fn store(&self, bond: &IonicBond) -> impl Future<Output = BondPersistenceResult<()>> + Send {
        capability_store(self, bond)
    }

    fn retrieve(
        &self,
        bond_id: &str,
    ) -> impl Future<Output = BondPersistenceResult<Option<IonicBond>>> + Send {
        capability_retrieve(self, bond_id)
    }

    fn list(&self) -> impl Future<Output = BondPersistenceResult<Vec<IonicBond>>> + Send {
        capability_list(self)
    }

    fn remove(&self, bond_id: &str) -> impl Future<Output = BondPersistenceResult<bool>> + Send {
        capability_remove(self, bond_id)
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
