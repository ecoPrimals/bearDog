<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 156k — Deep Debt Sweep: Clippy + allow hygiene + full audit

**Date**: August 6, 2026
**Author**: bearDog local (eastGate)
**Tests**: 14,026 passing | 0 failures | 0 Clippy warnings

---

## Summary

Comprehensive deep debt sweep covering all dimensions: Clippy, lint hygiene,
large files, mocks, dependencies, dead code, TODOs, and unsafe code.

## Changes

### Clippy (2 warnings → 0)

- Fixed `async fn` simplification warnings in `IosSecureEnclaveProvider::encrypt`
  and `decrypt` — these methods don't capture `&self` state, so `async fn` syntax
  is cleaner than `-> impl Future<Output = ...>`

### allow → expect Hygiene

- Upgraded `#[allow]` → `#[expect]` in 5 files where the suppressed lint reliably
  fires (`unreachable_patterns`, `unsafe_code`, `wildcard_imports`)
- Verified all 73 bare `#[allow()]` attributes carry `reason = "..."`

### Audit Results (no changes required)

| Dimension | Finding |
|-----------|---------|
| **Large files (>800L)** | No production file exceeds 800L. Largest: `tarpc_service/server.rs` (507L prod + 295L test = 802L total) |
| **Mocks in production** | Zero. All mock references in `#[cfg(test)]`, doc comments, or documented placeholders (quantum crypto awaiting upstream PQC standards) |
| **External deps** | 42 deps, 100% pure Rust. Only crates.io blocker: `rustls-rustcrypto` git dep (feature-gated behind `tls-gateway`, agreed for songBird excision) |
| **Dead code** | All `dead_code` allows justified (struct fields reserved for future wiring, pub API not called from bin target) |
| **TODO/FIXME** | 0 in production code |
| **`unsafe`** | Only in `libtower` (C ABI FFI exports) — justified, annotated |
| **Production `unwrap()`** | All in `#[cfg(test)]` blocks or benchmarks — zero production unwrap calls |

## Remaining Work for Upstream

1. **`rustls-rustcrypto` excision** — songBird owns TLS; bearDog owns crypto. Feature-gated, non-blocking for default build
2. **`parking_lot`** — 10 crates use it; pure Rust, no urgency. Could migrate to `std::sync::Mutex` if desired (low priority)
3. **Quantum crypto placeholder** — awaiting upstream PQC standards (ML-KEM / ML-DSA)
4. **tarpc convergence 30→236** — incremental, not blocking

## Longtail Signals for Upstream

### songBird: TLS/ACME Excision Ready

bearDog has fully feature-gated the TLS/ACME surface behind `tls-gateway` (default off).
songBird can excise at any time. The blocking git dep (`rustls-rustcrypto v0.0.2-alpha`)
lives entirely behind that gate. Once songBird ships its own TLS surface, bearDog can
remove `beardog-acme` and all `rustls-*` deps from the workspace entirely, unblocking
the G6 crates.io public flip.

**Action for songBird**: Take ownership of `rustls-rustcrypto` and ACME client logic.
bearDog will continue to expose `crypto.sign_jws_es256`, `crypto.jwk_thumbprint`,
`x509.build_csr`, and `x509.parse_certificate` as crypto-atom delegation for songBird's
ACME Phase 2 workflow.

### Near-Term Evolution Opportunities (bearDog-local)

1. **SoloKey genetics**: FIDO2 `hmac-secret` extension is wired — can use hardware
   entropy for genetic material (lineage beacons, enrollment seeds). SoloKey v2
   available on eastGate.
2. **tarpc convergence 30→236**: Incremental wiring of remaining JSON-RPC methods
   into the tarpc binary surface. Not blocking, but increases intra-gate throughput.
3. **Quantum crypto readiness**: Placeholder module compiles and type-checks. When
   ML-KEM / ML-DSA crates reach stable, drop-in integration via `BearDogCrypto` layer.
4. **Android filesystem socket fix**: `AndroidSocket` rejects `Filesystem` endpoint
   variant — P2 bug, fix when Android app-context deployment is tested.
5. **`parking_lot` → `std::sync`**: 10 crates, low priority, no C deps involved.

### For Overwatch

bearDog is **debt-clean**:
- 0 Clippy warnings
- 0 TODOs/FIXMEs
- 0 production mocks
- 0 production unwrap() calls
- 0 bare `#[allow()]` without reason
- 100% pure Rust (0 C deps)
- All `unsafe` confined to C ABI exports in `libtower`
- 14,026 tests, 0 failures, 90.51% coverage
