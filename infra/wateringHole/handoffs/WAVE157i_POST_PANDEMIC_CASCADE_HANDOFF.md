<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 157i — Post-Pandemic Cascade + G72 Tier 2 url Excision

**Date**: August 11, 2026
**Primal**: bearDog
**Gate**: eastGate
**Wave**: 157i
**From**: bearDog code team (eastGate)

---

## Summary

Four items resolved, bearDog gossip now LIVE on eastGate:

1. **Gossip protocol alignment** — bearDog was sending `gossip.spread` (peer-to-peer replication API) instead of `gossip.inject` (local origination API). Fixed to use `gossip.inject` with correct swarmVine entry structure: `topic` domain mapping, `key` for deduplication, structured `payload`. Live validated: `"result":"Accepted"` on eastGate swarmVine.
2. **Gossip socket resolution** — `default_swarmvine_socket()` was constructing a relative path (`ecosystem/swarmvine.sock`) instead of the full path (`/run/user/1000/biomeos/swarmvine.sock`). Fixed to read `BIOMEOS_SOCKET_DIR` env var, falling back to `biomeos_ipc_socket_dir_from_env()`.
3. **Darwin ios.rs import fix** — graftGate reported `ios.rs` missing `use beardog_config::env_keys` when compiling for `aarch64-apple-darwin`. Fixed and upstreamed.
4. **G72 Tier 2: `url` crate excision** — Discovered during binary growth investigation that `url` v2.5 pulled the entire ICU4X Unicode data chain (~32 transitive crates) for a single `From<url::ParseError>` impl. Excised `url` from `beardog-discovery` and `beardog-acme`.
5. **Binary growth investigation** — +2.9MB is from real code additions (tarpc 7→30, genetics, gossip), not phantom dependencies.

---

## Changes

### 1. Gossip Protocol Alignment — `gossip.spread` → `gossip.inject`

**File**: `crates/beardog-ipc/src/gossip.rs`

**Root cause**: bearDog's gossip client was calling `gossip.spread`, which is swarmVine's peer-to-peer replication API (expects fully-formed `GossipEntry` with nonce, TTL, version, etc.). The correct API for primals injecting local events is `gossip.inject`, which accepts:
- `topic`: gossip domain (`"tower"` / `"data"` / `"compute"`)
- `key`: deduplication key (e.g. `trust.bond.created:beardog`)
- `payload`: arbitrary JSON metadata

swarmVine automatically assigns nonce, TTL (8), version, and expiry (600s) for injected entries.

**Domain mapping**: All bearDog trust/crypto/HSM events map to `"tower"` domain. The `topic_to_domain()` function routes `compute.*` → `"compute"`, `data.*`/`cas.*` → `"data"`, everything else → `"tower"`.

### 2. Gossip Socket Resolution Fix

**File**: `crates/beardog-ipc/src/gossip.rs`

**Root cause**: `default_swarmvine_socket()` called `resolve_biomeos_ipc_subdir_from_optional(None)` which returns just the namespace string (`"ecosystem"`), then constructed `PathBuf::from("ecosystem").join("swarmvine.sock")` — a relative path. The running deployment has `BIOMEOS_SOCKET_DIR=/run/user/1000/biomeos`, so the correct socket path is `/run/user/1000/biomeos/swarmvine.sock`.

**Fix**: Resolution now checks `SWARMVINE_SOCKET` env (explicit override) → `BIOMEOS_SOCKET_DIR` env (deployment standard) → `biomeos_ipc_socket_dir_from_env()` (XDG/temp fallback).

### 3. Darwin ios.rs Fix (graftGate Finding)

**File**: `crates/beardog-tunnel/src/platform/ios.rs`

- Added `#[cfg(target_os = "macos")] use beardog_config::env_keys;`
- The `create_endpoint()` macOS code path uses `env_keys::ENV_BIOMEOS_SOCKET_DIR` but the import was missing
- On Linux this compiles because the `#[cfg(target_os = "macos")]` block is excluded
- On `aarch64-apple-darwin` (graftGate M4 Mac Mini) it failed
- Import is cfg-gated to `target_os = "macos"` since only the macOS path uses it

### 2. `url` Crate Excision (G72 Tier 2)

**Files modified**:
- `Cargo.toml` (workspace) — removed `url = "2.5"` from workspace deps
- `crates/beardog-discovery/Cargo.toml` — removed `url = { workspace = true }`
- `crates/beardog-discovery/src/error.rs` — removed `From<url::ParseError>` impl, updated test
- `crates/beardog-acme/Cargo.toml` — removed `url = { workspace = true }` (never imported)

**Crate count**: 382 → 350 (**-32 transitive crates**)

**Crates removed** (entire ICU4X chain):
- `url`, `idna`, `idna_adapter`
- `icu_normalizer`, `icu_normalizer_data`
- `icu_properties`, `icu_properties_data`
- `icu_provider`, `icu_collections`, `icu_locale_core`
- `zerotrie`, `zerovec`, `zerovec-derive`
- `yoke`, `yoke-derive`, `zerofrom`, `zerofrom-derive`
- `litemap`, `tinystr`, `writeable`, `potential_utf`, `utf8_iter`
- `displaydoc`, `synstructure`
- Plus additional ICU support crates

**Why `url` was there**: Single `From<url::ParseError> for DiscoveryError` impl in `error.rs` + one test. bearDog uses string socket paths (`unix:///path/to/socket`, plain filesystem paths), never `url::Url::parse()`. The `url` crate was vestigial from an earlier HTTP discovery phase.

**Why it grew**: `url` v2.5 switched its IDNA backend from the lightweight `unicode-bidi`/`unicode-normalization` crates to the full ICU4X stack (`icu_normalizer_data` + `icu_properties_data` — multi-megabyte Unicode data tables). The `cargo update` in Wave 157g picked up this new chain. While the linker dead-stripped the data tables (no binary size impact), the 32 crates were compiled and linked every build.

### 3. Binary Growth Analysis

**Finding**: The +2.9MB growth (pre-G72 vs post-G72 on darwin) is from real code additions, not dependency inflation.

Top `.text` contributors (cargo-bloat):
| Crate | Size | % of .text | Notes |
|-------|------|-----------|-------|
| beardog_tunnel | 1.6 MiB | 24.5% | HSM backends, 30 tarpc methods, crypto handlers, FIDO2 |
| std | 1.1 MiB | 17.6% | Standard library |
| beardog_cli | 374.6 KiB | 5.7% | CLI + handler registration |
| clap_builder | 240.9 KiB | 3.7% | Argument parser |
| tokio | 223.2 KiB | 3.4% | Async runtime |
| serde_json | 204.2 KiB | 3.1% | JSON serialization |
| regex_syntax + regex_automata | 260 KiB | 4.0% | tracing-subscriber env-filter |
| beardog_genetics | 74.3 KiB | 1.1% | New crate (Wave 156l) |

Growth attributable to:
- **tarpc 7→30 methods** (Wave 156h-156i): massive monomorphized generic codegen
- **beardog-genetics** crate: new in Wave 156l
- **gossip scaffolding**: new in Wave 157e
- **tracing-subscriber regex**: 260K from env-filter feature

Binary size is **healthy** for a trust primal with this feature set. No action needed.

---

## Verification

```
cargo check:   PASS (0 errors, 1 expected Android StrongBox note)
cargo clippy:  PASS (0 warnings)
cargo test:    1,153 passed, 0 failed, 6 ignored
```

---

## Remaining bearDog Items from Wave 157i Blurb

| Item | Status | Notes |
|------|--------|-------|
| Darwin ios.rs fix | **DONE** | This handoff |
| Binary growth investigation | **DONE** | Explained — real code, not leak |
| G72 Tier 2: url excision | **DONE** | -32 crates |
| G72 Tier 2: axum 0.7→0.8 | Not applicable | bearDog does not depend on axum |
| G72 Tier 2: HTTP→songBird/capability.call | Not applicable | bearDog has no HTTP client in default build (reqwest gated behind `tls-gateway`) |
| Gossip injection → LIVE | **DONE** | Protocol aligned (`gossip.inject`), socket resolution fixed, live validated on eastGate — `"result":"Accepted"` |
| `beardog-node-registry` absorption | DEFERRED | Very thin crate but still has consumers; not urgent |
| `beardog-workflows` absorption | DEFERRED | Similarly thin; review in next deep-debt wave |

---

## Cascade

Push to golgiBody/Forgejo (`git.primals.eco`) after this handoff.
