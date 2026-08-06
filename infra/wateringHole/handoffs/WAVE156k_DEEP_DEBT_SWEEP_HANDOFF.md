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

## For Overwatch

bearDog is **debt-clean**:
- 0 Clippy warnings
- 0 TODOs/FIXMEs
- 0 production mocks
- 0 production unwrap() calls
- 0 bare `#[allow()]` without reason
- 100% pure Rust (0 C deps)
- All `unsafe` confined to C ABI exports in `libtower`
