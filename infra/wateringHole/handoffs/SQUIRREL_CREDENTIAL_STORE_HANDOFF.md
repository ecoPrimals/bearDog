# Handoff: squirrel CredentialStore Integration

**From**: bearDog (flockGate)
**To**: squirrel team
**Date**: July 25, 2026
**Wave**: 150u (integration shipped) / 150x (handoff created)
**Status**: ✅ INTEGRATED (item 5 pending co-located host)
**Priority**: P1 (cross-primal credential storage)

---

## Context

bearDog has shipped the `CredentialStore` trait and three backends as of Wave 150u.
This enables squirrel's `SecurityProvider` to delegate secret storage to bearDog
over the existing `secrets.*` JSON-RPC interface. The Android Keystore backend
(Wave 150u) extends coverage to grapheneGate hardware.

### What bearDog Provides

| Component | Location | Purpose |
|-----------|----------|---------|
| `CredentialStore` trait | `beardog-traits::unified::storage` | `store`, `retrieve`, `list`, `delete` async API with `SecretMetadata` |
| `InMemoryCredentialStore` | `beardog-tunnel::credential_store::in_memory` | Volatile backend (dev/test) |
| `FileVaultCredentialStore` | `beardog-tunnel::credential_store::file_vault` | Persistent encrypted backend (ChaCha20-Poly1305 + HKDF-SHA256) |
| `AndroidKeystoreCredentialStore` | `beardog-tunnel::credential_store::android_keystore` | Android TEE/StrongBox master key (Wave 150u) |
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

### Integration Results — Wave 150u (July 22, 2026)

**Status**: ✅ **INTEGRATED** — squirrel's `SecurityProvider` now delegates to
bearDog's `secrets.*` JSON-RPC over IPC.

#### What was implemented

| Deliverable | Status | Details |
|-------------|--------|---------|
| `CredentialStorage::SecurityProvider` delegates to `secrets.*` over IPC | ✅ | `SecurityProviderSecretStore` implements `SecretStore` via JSON-RPC |
| AI key retrieval via `secrets.retrieve` (not `std::env::var`) | ✅ | `api_key_resolver` module, `discover_http_providers` wired |
| Graceful fallback to env vars when bearDog is not running | ✅ | `resolve_secret_or_env` tries store first, falls back to env |
| 5-point verification checklist passing | ⏳ | Unit tests pass; integration test blocked until bearDog is running on same host |
| Report back on handoff doc | ✅ | This section |

#### Code delivered

| File | Purpose |
|------|---------|
| `crates/core/mcp/src/security/security_provider_secret_store.rs` | IPC-backed `SecretStore` — `secrets.store/retrieve/list/delete` JSON-RPC |
| `crates/core/mcp/src/security/secret_store.rs` | `SecretStoreBackend::SecurityProvider` variant + `from_config` wiring |
| `crates/main/src/api/ai/api_key_resolver.rs` | `resolve_api_key`, `is_api_key_available`, `filter_providers_with_keys` |
| `crates/main/src/api/ai/router_discovery.rs` | HTTP provider discovery uses bearDog store before env fallback |

#### Endpoint discovery

Tiered resolution (same pattern as all primal IPC):
1. `SECURITY_ENDPOINT` env var (full URL: `unix:///path` or `tcp://host:port`)
2. `BEARDOG_ENDPOINT` env var (fallback)
3. `resolve_capability_unix_socket("SECURITY_SOCKET", "beardog")` (socket auto-detect)

#### Test results

- **14 new tests** (8 security provider, 6 api key resolver)
- **7,122 total tests passing** across 16 workspace crates
- 0 failures, 0 warnings, Clippy clean

#### Remaining for full 5-point checklist

Items 1–4 of the verification checklist are covered by unit tests against
`InMemorySecretStore`. Item 5 (persistence across restart) requires bearDog
running on the same host with `FileVaultCredentialStore`. This will be tested
when bearDog and squirrel co-locate on eastGate/sporeGate.
