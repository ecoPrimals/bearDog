# After Action Report: Wave 151a/151b/151c/151d

**Date**: July 25, 2026
**Author**: flockGate
**Commit range**: `f7e2f50..HEAD`

---

## Scope

Four back-to-back waves delivering defense-in-depth UDS enforcement (151a),
deep debt sweep with smart file decomposition (151b), spec audit + env wiring
+ DNS bug fix (151c), and publication readiness pen test + security hardening (151d).

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

## Wave 151c — Spec Audit + Env Wiring + DNS Bug Fix

### What was delivered

| Item | Status |
|------|--------|
| 7 stale spec status headers corrected | SHIPPED |
| `dns_timeout_seconds` bug (mapped to `DEFAULT_POOL_SIZE`) | FIXED |
| `HumanEntropyConfig.collection_timeout_ms` env wired | SHIPPED |
| `OperationRouterConfig.from_env()` added | SHIPPED |
| `UniversalHsmConfig.from_env()` added | SHIPPED |
| New env key: `BEARDOG_ENTROPY_COLLECTION_TIMEOUT_MS` | SHIPPED |

### Files changed

| File | Change |
|------|--------|
| `specs/current/ZERO_HARDCODING_SPECIFICATION.md` | `ACTIVE MANDATE` → `ACHIEVED` |
| `specs/current/production/PRODUCTION_READINESS_SPECIFICATION.md` | 231 methods, 13,973+ tests |
| `specs/current/architecture/BEARDOG_NODE_REGISTRY_MODERNIZATION_ROADMAP.md` | `CRITICAL` → `DEFERRED/DORMANT` |
| `specs/current/architecture/BEARDOG_SCOPE_AND_BOUNDARIES.md` | Updated metrics |
| `specs/current/PURE_RUST_ZERO_DEPENDENCY_ROADMAP.md` | `ROADMAP ACTIVE` → `ACHIEVED` |
| `specs/current/security/UNIVERSAL_HSM_ENTROPY_ORCHESTRATION.md` | `IN PROGRESS` → `PHASE 1 SHIPPED` |
| `specs/KEY_ROTATION_AND_LIFECYCLE.md` | `IMPLEMENTATION SPEC` → `PARTIAL` |
| `crates/beardog-types/.../connection.rs` | DNS timeout bug fix + env wiring |
| `crates/beardog-config/src/env_keys/identity.rs` | Added `ENV_ENTROPY_COLLECTION_TIMEOUT_MS` |
| `crates/beardog-genetics/.../legacy.rs` | Wired `collection_timeout_ms` from env |
| `crates/beardog-tunnel/.../operation_router.rs` | Added `from_env()` |
| `crates/beardog-core/.../universal_hsm_provider.rs` | Added `from_env()` |

---

## Wave 151d — Publication Readiness Pen Test + Security Hardening

### Pen test findings (3-surface audit: JSON-RPC, HTTP API, BTSP TCP)

| Severity | Count | Status |
|----------|-------|--------|
| CRITICAL | 3 | **ALL RESOLVED** |
| HIGH | 4 | Documented, deployment-hardened |
| MEDIUM | 6 | Justified or scoped |
| LOW | 3 | By design |

### Critical fixes shipped

| Finding | Fix |
|---------|-----|
| `BearDogError` text with filesystem paths forwarded to IPC clients | `sanitize_error_message()` strips paths + OS errors; full errors logged server-side |
| `capability.call` in PUBLIC_METHODS — auth bypass on re-dispatch to crypto | Moved to Protected — requires valid token |
| `auth.issue_ionic` mints wildcard `["*"]` scope to unauthenticated callers | Default scope → `[]` (empty); explicit scope required |
| BTSP initial handshake accepts `null` cipher (floor only on re-negotiation) | Cipher floor enforced at Phase 2 via `BtspCipher::rank()` |
| TCP BTSP server forwards parse/business errors verbatim | Sanitized via `into_json_rpc_error()` |

### Android target compile fixes (eastGate readiness)

| File | Fix |
|------|-----|
| `credential_store/android_keystore.rs` | Removed `whoami` dep; added `tracing::warn!` |
| `android_strongbox/core/unified.rs` | `HashMap` → `BTreeMap` for unified provider traits |

### Remaining HIGH items (deployment hardening, not code)

- `BEARDOG_AUTH_MODE` defaults Permissive (set `=enforced` in production)
- `BEARDOG_UDS_REQUIRE_BTSP` defaults OFF (set `=1` in production)
- No per-method crypto rate limiting (connection-level cap exists at 512)
- Same-UID UDS bypass (defense-in-depth; acceptable for single-user gates)

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
- `DEFAULT_POOL_SIZE` was being used as a generic "10" constant across 30+ call sites, including DNS timeout — semantic constants prevent category errors
- Specs drift even faster than root docs — 7 specs had incorrect status labels that would mislead upstream auditors
- `from_env()` should be added at struct creation time, not bolted on later; 3 config structs had Default but no env wiring
- Security pen tests are most effective post-integration: songBird crypto delegation 6/6 meant the full API surface was exercised for the first time
- Error sanitization must happen at the boundary, not at each error site — one `sanitize_error_message()` covers all 36+ `BearDogError` propagation paths
- `PUBLIC_METHODS` list is a critical security surface — any method there bypasses auth entirely; should be reviewed on every handler addition
- Android target compilation diverges from host — `HashMap`/`BTreeMap` mismatches only surface in `#[cfg(target_os = "android")]` blocks
