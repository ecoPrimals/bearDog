# AAR — Wave 155i: bearDog Status Report for Overwatch + songBird

**Date**: Jul 29, 2026
**Wave**: 155i
**From**: bearDog code team (eastGate)
**To**: eastGate overwatch, songBird team
**Status**: ALL ASSIGNED ITEMS COMPLETE. No P0/P1 blockers. Awaiting upstream.

---

## Executive Summary

bearDog has completed both assigned glacial goals ahead of the 155i wave:

| Assignment | Status | Commit | When |
|------------|--------|--------|------|
| **ACME Phase 2 client** (songBird needs it) | **SHIPPED** | `cd509ac81` | Wave 155i |
| **G6 public flip audit** | **SHIPPED** | `df9591d8e` | Wave 155d |

bearDog has **zero P0/P1 items**, zero clippy warnings, zero TODOs, and
14,013 passing tests. The codebase is production-ready and idle — waiting
on upstream dependencies to unblock the next glacial goals.

---

## What bearDog Shipped (Waves 155b–155i)

### Wave 155i — ACME Phase 2 Crypto Delegation (`cd509ac81`)

5 new JSON-RPC methods enabling songBird to delegate all ACME certificate
lifecycle crypto to bearDog via Tower Atomic IPC:

| Method | RFC | Purpose |
|--------|-----|---------|
| `crypto.ecdsa_p256_generate_signing_keypair` | — | Persistent ECDSA P-256 key with JWK output |
| `crypto.sign_jws_es256` | 7515 | JWS ES256 signing (raw 64-byte `r\|\|s`) |
| `crypto.jwk_thumbprint` | 7638 | SHA-256 JWK thumbprint for HTTP-01 |
| `x509.build_csr` | 2986 | PKCS#10 CSR with SubjectAltName |
| `x509.parse_certificate` | 5280 | X.509 metadata: expiry, SANs, issuer, is_ca |

17 new tests. 114 total crypto methods. Full integration guide in
`WAVE155i_ACME_PHASE2_HANDOFF.md`.

### Wave 155d — G6 Public Flip Audit (`df9591d8e`)

- 21 library crates validated for crates.io publish
- `beardog-errors` full `cargo publish --dry-run` PASS
- Topological publish order documented (21-crate sequence)
- 6 binary/internal crates classified `publish = false`
- 41 stale `deny.toml` skip entries removed (TLS feature-gating resolved splits)
- Flaky shared-state test fixed

### Wave 155c — Deep Debt Sweep (`f5c81d243`)

- Clippy 31→0 warnings
- CLI wired: revocation export/import handlers connected
- Dead code cleaned: orphan modules deleted, test-only functions gated
- 6 production `expect()` sites hardened to `BearDogError` propagation

### Wave 155b — TLS/ACME Handoff (`645908f47`)

- TLS/ACME feature-gated behind `tls-gateway` (default off)
- `rustls`, `tokio-rustls`, `rustls-rustcrypto` removed from default dep tree
- 3-phase deprecation plan documented

### Wave 155 — Chimera Phase 0 (`4f24600aa`)

- `beardog-crypto` crate extracted from `beardog-core`
- `libtower.so` C ABI shared library (8 exports, 565K x86_64, 446K aarch64)
- Cross-compiled and validated on grapheneGate (Android ARM64)

---

## Codebase Health

| Metric | Value |
|--------|-------|
| Version | 0.9.0 |
| Crates | 27 workspace members |
| Tests | **14,013 passing**, 0 failing |
| Coverage | 90.51% line (llvm-cov) |
| Clippy | **0 warnings** (pedantic + nursery + cast lints) |
| TODO/FIXME | **0** |
| Files >800 LOC | **0** production (3 test files at 860–866L) |
| cargo deny | **all 4 pass** |
| Pure Rust | **100%** (zero C crypto deps, ecoBin compliant) |
| Unsafe code | **0 production** (`forbid(unsafe_code)` workspace-wide) |
| JSON-RPC methods | **236** dispatchable (114 crypto + 122 other) |
| Platform support | Linux, macOS, Android, Windows, iOS |

---

## For songBird — ACME Phase 2 Integration Guide

### What bearDog Provides (via JSON-RPC IPC)

```
ACME Account Registration:
  1. crypto.ecdsa_p256_generate_signing_keypair  →  account key + JWK
  2. crypto.jwk_thumbprint                       →  HTTP-01 keyAuthorization
  3. crypto.sign_jws_es256                       →  signed ACME requests

Certificate Issuance:
  4. x509.build_csr                              →  PKCS#10 CSR (domains + SAN)

Renewal Monitoring:
  5. x509.parse_certificate                      →  expiry, SANs, issuer
```

### What songBird Owns (port from beardog-acme)

| Component | Source | Lines |
|-----------|--------|-------|
| ACME directory/orders/finalize | `beardog-acme/src/client/mod.rs` | ~370 |
| HTTP-01 challenge TCP server | `beardog-acme/src/challenge.rs` | ~230 |
| Renewal daemon + scheduling | `beardog-acme/src/client/renewal.rs` | ~200 |
| PEM cert filesystem storage | `beardog-acme/src/storage.rs` | ~200 |
| ACME config + reqwest client | `beardog-acme/src/client/config.rs` | ~120 |
| Hot-reload ServerConfig | `beardog-acme/src/hot_reload.rs` | ~165 |

**Critical note**: The live `beardog-acme` code uses **ECDSA P-256 (ES256)**
for all JWS signing, not Ed25519. The spec doc (`ACME_TLS_INTEGRATION_PATH.md`)
is stale on this point — follow the code.

### Deprecation Plan Progress

| Phase | Status | Action |
|-------|--------|--------|
| Phase 1: Feature-gate | **DONE** (Wave 155b) | TLS/ACME off default, `tls-gateway` feature |
| Phase 2: Expose crypto IPC | **DONE** (Wave 155i) | 5 methods, handoff issued |
| Phase 3: songBird absorbs | **NEXT** — songBird team | Port `beardog-acme` protocol logic |
| Phase 4: bearDog excises | After songBird ships | Delete `beardog-acme`, all TLS deps/features |

---

## For Overwatch — What bearDog Needs

### To Actually Publish (G6)

1. **crates.io API token** — overwatch decision
2. **Crate name reservation** — confirm `beardog-*` names available
3. **CI publish step** — cellMembrane to wire into release pipeline

### Glacial Goal Dependencies

| Goal | bearDog Status | Blocked On |
|------|---------------|------------|
| G6 (public flip) | **READY** — audit complete, 21 crates validated | crates.io token + CI |
| G5 (Chimera Phase 1) | Phase 0 shipped (`libtower.so`) | G1 (Tower on Windows proof) |
| G2 (Tower on Android) | Validated on grapheneGate | G1 |
| ACME excision | Phase 2 complete | songBird Phase 3 |

### bearDog Can Pick Up

If overwatch has additional work for bearDog, we're available. Current
codebase is clean and idle. Potential deep debt targets:

- Promote `doc_lazy_continuation` from allow to warn (low-priority doc polish)
- Profile and optimize hot-path crypto (libtower benchmarks)
- Expand `libtower` C ABI exports for Chimera Phase 1 prep (AES-GCM, X25519, HKDF)
- Integration test suite for ACME Phase 2 methods (mock songBird caller)

---

*bearDog v0.9.0. All Wave 155 assignments complete. 14,013 tests, 0 warnings,
0 TODOs, 90.5% coverage, 236 JSON-RPC methods, 100% pure Rust. Awaiting
upstream: songBird ACME absorption, G1 proof, crates.io token.*
