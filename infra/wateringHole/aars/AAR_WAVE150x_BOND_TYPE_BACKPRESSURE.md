# AAR: Wave 150x — Bond-Type Cipher Awareness + Backpressure Signaling

**Date**: July 25, 2026
**Wave**: 150x
**Team**: bearDog (flockGate)
**Duration**: ~30 min
**Commits**: `988730d`

---

## Mission

Ship the final two bearDog P2 items from the Wave 150x blurb:
1. Bond-type cipher awareness in BTSP negotiation (covalent > ionic floor differentiation)
2. UDS backpressure signaling (explicit error instead of silent hang)

## What We Did

### 1. Bond-Type Cipher Awareness (P2 #1)

**Problem**: BTSP cipher floor was flat — one global `BEARDOG_BTSP_CIPHER_FLOOR`
for all connection types. Same-family (covalent) connections and cross-family
(ionic) contracts were enforced identically.

**Implementation**:
- Added `BtspBondType` enum (`Covalent` / `Ionic`) to `beardog-types/src/btsp/rpc.rs`
- Added per-type env keys: `BEARDOG_BTSP_CIPHER_FLOOR_COVALENT`, `BEARDOG_BTSP_CIPHER_FLOOR_IONIC`
- `load_cipher_floor_for_bond()` — precedence: bond-type env → global env → default (`chacha20-poly1305`)
- `select_best_cipher_for_bond()` — replaces flat `select_best_cipher()` in production paths
- Wired into both `btsp.negotiate` (Phase 3) and `btsp.server.negotiate`
- `SessionNegotiateParams` gains `bond_type` field (default `covalent`, backward compatible)
- 6 new tests: covalent default floor, ionic lowered floor, ionic fallback to global,
  covalent enforces strong, bond type default/serde roundtrip

### 2. UDS Backpressure Signaling (P2 #2)

**Problem**: When all UDS connection slots were occupied, callers connected at
the OS level but hung indefinitely waiting for the handler to start (semaphore
blocked after accept).

**Implementation**:
- Changed `sem.acquire_owned().await` → `tokio::time::timeout(100ms, sem.acquire_owned())`
- On timeout: `send_saturation_error()` writes JSON-RPC error code `-32003`
  ("Server saturated") with `retry_after_ms: 500` and `max_connections` hint,
  then closes the connection
- Mirrors the existing BTSP rejection pattern (structured error → close)
- 100ms grace period allows brief traffic bursts to self-resolve

## Files Changed

| File | Change |
|------|--------|
| `crates/beardog-types/src/btsp/rpc.rs` | `BtspBondType` enum; `bond_type` on `SessionNegotiateParams` |
| `crates/beardog-types/src/btsp/mod.rs` | Re-export `BtspBondType` |
| `crates/beardog-config/src/env_keys/security.rs` | `ENV_BTSP_CIPHER_FLOOR_COVALENT`, `ENV_BTSP_CIPHER_FLOOR_IONIC` |
| `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/negotiation.rs` | Bond-type-aware floor logic + 6 tests |
| `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` | Backpressure timeout + `send_saturation_error()` |
| `specs/current/integration/CRYPTO_JSONRPC_HANDOFF_WAVE149b.md` | Wire contract updated |
| `CHANGELOG.md`, `STATUS.md`, `ROADMAP.md` | Updated |

## What Went Well

- Both items shipped cleanly in a single commit — no intermediate breakage
- All 13,973 tests pass (6 new from bond-type, rest unchanged)
- Clippy clean (workspace-wide, all targets)
- Backward compatible — absent `bond_type` defaults to `covalent`; absent
  ionic floor falls back to global floor
- Backpressure pattern follows existing BTSP rejection precedent

## What We Learned

- The ionic bond system already had `EncryptionTier` and `BondTrustModel`
  but they were metadata-only (not enforced in cipher selection). The new
  `BtspBondType` bridges the gap between bond semantics and negotiation enforcement.
- `OnceLock`-based cipher floor caching means bond-type-specific floors read
  from env on each call (not cached), which is intentional — allows runtime
  rotation without restart. The global floor remains cached for performance.

## Risks

- **Ionic floor misconfiguration**: setting `BEARDOG_BTSP_CIPHER_FLOOR_IONIC=null`
  allows plaintext cross-family traffic. Mitigated by: (a) default is covalent
  floor (strongest), (b) production deployments should audit env vars.
- **Backpressure thundering herd**: if many clients are rejected simultaneously,
  retry storms could amplify load. The 500ms `retry_after_ms` hint mitigates this
  but callers need to respect it with jitter.

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests | 13,967 | 13,973 |
| Bond-type env keys | 0 | 2 |
| Cipher floor paths | 1 (flat) | 3 (global, covalent, ionic) |
| UDS saturation signal | none (hang) | JSON-RPC -32003 |

## Status

All bearDog P1 and P2 items from Wave 150x are **SHIPPED**. Remaining items:
- P2: Android Keystore + grapheneGate — code complete, awaiting hardware validation
  (handoff issued to primalSpring/eastGate team)
- P2: Publication readiness — pending pen test + validation pass
