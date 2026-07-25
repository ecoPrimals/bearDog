# After Action Report: Wave 151a/151b

**Date**: July 25, 2026
**Author**: flockGate
**Commit range**: `f7e2f50..f3797b0`

---

## Scope

Two back-to-back waves delivering defense-in-depth UDS enforcement (151a) and a
deep debt sweep with smart file decomposition (151b).

---

## Wave 151a — BTSP on Local UDS (Defense-in-Depth)

### What was delivered

| Item | Status |
|------|--------|
| `BEARDOG_UDS_REQUIRE_BTSP=1` env var | SHIPPED |
| Gate the first-byte `{` bypass on family socket | SHIPPED |
| Reject plain JSON-RPC with `-32600` + guidance | SHIPPED |
| Allow JSON-line BTSP `ClientHello` as the `{`-prefixed entry point | SHIPPED |
| Health socket (`beardog-default.sock`) unaffected | SHIPPED |
| `capabilities.list` advertises `btsp_strict`, `cleartext_available` | SHIPPED |

### Files changed

- `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` — `require_btsp` field, connection gating
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/capabilities.rs` — strict-mode advertisement
- `crates/beardog-config/src/env_keys/security.rs` — `ENV_UDS_REQUIRE_BTSP`
- `specs/current/integration/CRYPTO_JSONRPC_HANDOFF_WAVE149b.md` — wire contract

### Risks

- songBird must send JSON-line `ClientHello` when strict mode is active (shared P2)
- Health socket remains plaintext by design — monitoring tools rely on it

---

## Wave 151b — Deep Debt Sweep

### What was delivered

| Item | Status |
|------|--------|
| `enrollment.rs` (1061L) decomposed into 7 modules | SHIPPED |
| `lineage_proof.rs` (877L) tests extracted to `lineage_proof_tests.rs` | SHIPPED |
| Hardcoded `"eth0"` → `BEARDOG_MDNS_INTERFACE` env var | SHIPPED |
| Hardcoded `/tmp` fallbacks → `std::env::temp_dir()` | SHIPPED |
| `reject()` helper to reduce enrollment response boilerplate | SHIPPED |
| All `redundant_pub_crate` clippy warnings resolved | SHIPPED |

### Files changed

- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment.rs` — DELETED
- `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/enrollment/` — NEW (7 files)
  - `mod.rs`, `handler.rs`, `crypto.rs`, `config.rs`, `lineage.rs`, `replay_cache.rs`, `tests.rs`
- `crates/beardog-genetics/src/birdsong/lineage_proof.rs` — tests extracted
- `crates/beardog-genetics/src/birdsong/lineage_proof_tests.rs` — NEW
- `crates/beardog-config/src/env_keys/network.rs` — `ENV_MDNS_INTERFACE`
- `crates/beardog-core/src/universal_discovery/types.rs` — env-driven mDNS config
- `crates/beardog-tunnel/src/tunnel/hsm/linux_secret_service/mod.rs` — `temp_dir()`

### Audit findings (for upstream review)

Items found during audit but not actioned this wave (external deps or low priority):

| Finding | Category | Action |
|---------|----------|--------|
| 9× `ApiError::NotImplemented` in REST handlers | Stub in production | P3 — REST→UDS forwarding bridge; owned by songBird transport layer |
| Consul/etcd discovery backends return `BackendUnavailable` | Stub in production | P3 — Phase 2 when service mesh required |
| Android Keystore JNI not wired | Platform gap | Blocked on eastGate hardware (handoff open) |
| iOS Secure Enclave stubs | Platform gap | No iOS hardware in ecosystem |
| `quantum_crypto` module is simulation-only | Design intent | Clearly documented; not for production crypto |
| 69 duplicate crate versions in `Cargo.lock` | Dependency hygiene | RustCrypto RC convergence will resolve when stable |
| `ZERO_HARDCODING_SPECIFICATION.md` claims 307 remaining | Stale spec | Contradicts STATUS.md; spec needs refresh |
| `PRODUCTION_READINESS_SPECIFICATION.md` says 100 methods, 29 crates | Stale spec | Reality: 231 methods, 25 crates |

---

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests passing | 13,973+ | 13,973+ |
| Clippy warnings | 0 | 0 |
| Production files >800L | 0 | 0 |
| Largest enrollment file | 1,061L (monolith) | 202L (`handler.rs`) |
| Largest lineage_proof file | 877L (inline tests) | 399L (tests extracted) |

---

## What went well

- Four-dimensional parallel audit (large files, unsafe, hardcoding, deps) enabled efficient triage
- Smart decomposition by concern (not arbitrary splitting) preserved API boundaries
- Zero test regressions despite 1,500+ lines restructured
- Prior-wave items (unused_self, ed448-goldilocks, parking_lot migration) confirmed already clean

## What was learned

- Root docs drift faster than code — test counts, method counts, and dates accumulate inconsistencies across 9+ files
- The `#[path]` attribute for test extraction is the cleanest pattern for Rust test isolation
- `cargo clippy --fix` handles `redundant_pub_crate` automatically
