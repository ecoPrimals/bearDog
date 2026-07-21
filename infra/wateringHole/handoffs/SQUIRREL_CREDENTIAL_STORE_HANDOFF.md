# Handoff: squirrel CredentialStore Integration

**From**: bearDog (flockGate)
**To**: squirrel team
**Date**: July 21, 2026
**Wave**: 150t
**Priority**: P1 (cross-primal credential storage)

---

## Context

bearDog has shipped the `CredentialStore` trait and two backends as of Wave 150t.
This enables squirrel's `SecurityProvider` to delegate secret storage to bearDog
over the existing `secrets.*` JSON-RPC interface.

### What bearDog Provides

| Component | Location | Purpose |
|-----------|----------|---------|
| `CredentialStore` trait | `beardog-traits::unified::storage` | `store`, `retrieve`, `list`, `delete` async API with `SecretMetadata` |
| `InMemoryCredentialStore` | `beardog-tunnel::credential_store::in_memory` | Volatile backend (dev/test) |
| `FileVaultCredentialStore` | `beardog-tunnel::credential_store::file_vault` | Persistent encrypted backend (ChaCha20-Poly1305 + HKDF-SHA256) |
| `CredentialStoreBackend` | `beardog-tunnel::credential_store::backend` | Enum dispatch (Silicon Atheism pattern) |
| `secrets.*` JSON-RPC | IPC handlers | Wire-level API for cross-primal access |

### Wire Contract (JSON-RPC)

```json
// secrets.store
{"jsonrpc":"2.0","method":"secrets.store","params":{"name":"api_key","value":"sk-..."},"id":1}
// → {"jsonrpc":"2.0","result":{"stored":true},"id":1}

// secrets.retrieve
{"jsonrpc":"2.0","method":"secrets.retrieve","params":{"name":"api_key"},"id":2}
// → {"jsonrpc":"2.0","result":{"name":"api_key","value":"sk-..."},"id":2}

// secrets.list
{"jsonrpc":"2.0","method":"secrets.list","params":{},"id":3}
// → {"jsonrpc":"2.0","result":{"secrets":["api_key","db_password"]},"id":3}

// secrets.delete
{"jsonrpc":"2.0","method":"secrets.delete","params":{"name":"api_key"},"id":4}
// → {"jsonrpc":"2.0","result":{"deleted":true},"id":4}
```

---

## Integration Path for squirrel

### 1. Wire Contract Alignment

bearDog uses `name`/`value` parameters. If squirrel's `SecurityProvider` uses
`purpose`/`key` internally, map at the IPC boundary:

```
squirrel purpose → bearDog name
squirrel key     → bearDog value
```

### 2. AI Provider API Keys

Route AI provider API keys through `secrets.retrieve` instead of raw env vars:

```
// Instead of:
let key = std::env::var("OPENAI_API_KEY")?;

// Use:
let resp = ipc_call("secrets.retrieve", {"name": "openai_api_key"}).await?;
let key = resp["value"].as_str();
```

### 3. SecurityProvider Delegation

squirrel's `CredentialStorage::SecurityProvider` variant should delegate to
bearDog's `secrets.*` methods over UDS/TCP IPC. The connection path follows
standard primal IPC discovery (socket auto-detect or `BEARDOG_TCP_IPC_PORT`).

---

## Verification Checklist

| # | Check | Method |
|---|-------|--------|
| 1 | Store a secret via bearDog | `secrets.store` → confirm `stored: true` |
| 2 | Retrieve it from squirrel | squirrel `SecurityProvider` → `secrets.retrieve` |
| 3 | List secrets | `secrets.list` → confirm name present |
| 4 | Delete + confirm gone | `secrets.delete` → `secrets.retrieve` returns error |
| 5 | Persistence across restart | Store → restart bearDog → retrieve (FileVault backend) |

---

## Code Pointers

| File | What |
|------|------|
| `crates/beardog-traits/src/unified/storage.rs` | `CredentialStore` trait + `SecretMetadata` |
| `crates/beardog-tunnel/src/credential_store/mod.rs` | Module root |
| `crates/beardog-tunnel/src/credential_store/in_memory.rs` | Volatile backend |
| `crates/beardog-tunnel/src/credential_store/file_vault.rs` | Persistent encrypted backend |
| `crates/beardog-tunnel/src/credential_store/backend.rs` | `CredentialStoreBackend` enum dispatch |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/secrets.rs` | `secrets.*` JSON-RPC handlers |
| `specs/current/integration/CRYPTO_JSONRPC_HANDOFF_WAVE149b.md` | Full JSON-RPC method reference |

---

## Report Back

When integration is tested, update this handoff with results and push via
cascade. The bearDog agent on flockGate will pick up in the next wave.
