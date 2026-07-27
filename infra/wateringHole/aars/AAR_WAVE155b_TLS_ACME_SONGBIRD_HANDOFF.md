<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# AAR: Wave 155b — TLS/ACME Excision + songBird Transport Handoff

**Date**: Jul 27, 2026
**From**: bearDog code team (eastGate overwatch)
**To**: songBird code team, upstream overwatch
**Wave**: 155b
**Status**: TLS DEPRECATED in bearDog — songBird handoff required

---

## Summary

bearDog contained ~2,800 lines of TLS termination and ACME certificate management
code that violates primal responsibility boundaries. TLS is network transport —
**songBird's domain**. This code has been feature-gated off (`tls-gateway`, default
disabled) and is marked for excision once songBird absorbs the capability.

The `rustls-rustcrypto` git dependency — the only blocker for G6 (crates.io public
flip) — is now eliminated from bearDog's default build.

---

## What Was Found

### Three components that don't belong in bearDog

| Component | Location | Lines | What It Does |
|-----------|----------|-------|--------------|
| `beardog-acme` | `crates/beardog-acme/` | 2,574 | Full RFC 8555 ACME client (Let's Encrypt), HTTP-01 challenge solver, cert storage, renewal daemon, hot-reload acceptor |
| TLS server | `crates/beardog-tunnel/src/tcp_ipc/tls.rs` | 244 | rustls TLS termination, X.509 cert loading, SNI validation |
| HTTPS gateway | `crates/beardog-cli/src/handlers/server/gateway.rs` | 136 | Reverse proxy on :443 using ACME certs, upstream forwarding |

### Why they don't belong

1. **Tower Atomic principle**: bearDog provides crypto primitives over IPC (BTSP).
   Other primals delegate crypto to bearDog. bearDog does not own network transport.
2. **songBird owns transport**: songBird's `universal-ipc` module handles UDS, TCP,
   named pipes, abstract sockets. songBird's CONTEXT.md explicitly claims
   "Pure Rust TLS 1.3 client for sovereign HTTPS". TLS termination is songBird's job.
3. **BTSP already handles encrypted IPC**: bearDog has its own ChaCha20-Poly1305
   encrypted framing (BTSP Phase 3) with HKDF session keys. BTSP runs over Unix
   sockets and TCP without needing TLS at all.
4. **The git dependency**: `rustls-rustcrypto` is only needed for these three
   components. It's a git dep (not on crates.io in a usable form), blocking G6.

### What bearDog owns vs songBird owns

| Capability | Owner | Implementation |
|-----------|-------|----------------|
| Crypto primitives (Ed25519, AEAD, hashing, HSM, FIDO2) | **bearDog** | `beardog-crypto`, JSON-RPC, `libtower.so` |
| Encrypted IPC framing | **bearDog** | BTSP Phase 3 (ChaCha20-Poly1305 + HKDF) |
| TLS termination (X.509, cert loading, SNI) | **songBird** | `universal-ipc`, rustls |
| ACME cert lifecycle (Let's Encrypt, HTTP-01, renewal) | **songBird** | Should absorb `beardog-acme` |
| HTTPS reverse proxy | **songBird** | Network routing |
| WireGuard replacement / mesh transport | **songBird** | Core mission |

---

## What Was Done (Wave 155b)

1. **`tls-server` removed from default features** in `beardog-tunnel/Cargo.toml`
2. **`beardog-acme` made optional** in `beardog-cli/Cargo.toml` (behind `tls-gateway` feature)
3. **`rustls-rustcrypto` made optional** in `beardog-cli/Cargo.toml` (behind `tls-gateway` feature)
4. **Gatehouse mode block** in server handler feature-gated behind `#[cfg(feature = "tls-gateway")]`
5. **ACME/gateway modules** feature-gated behind `#[cfg(feature = "tls-gateway")]`
6. **rustls provider install** in `main.rs` feature-gated behind `#[cfg(feature = "tls-gateway")]`

### Result

| Metric | Before | After |
|--------|--------|-------|
| `rustls` in default dep tree | Yes | **No** |
| `rustls-rustcrypto` in default build | Yes (git dep) | **No** |
| `reqwest` in default build | Yes (via acme) | **No** |
| G6 blocker (git dep) | Blocked | **Unblocked** (default build) |
| `cargo deny check` | 4/4 pass | 4/4 pass |
| Default build warnings | 0 | 0 |
| Legacy TLS available | — | `--features tls-gateway` |

---

## Deprecation Plan

### Phase 1 — DONE (Wave 155b)
- Feature-gate TLS/ACME off by default
- bearDog default build is TLS-free

### Phase 2 — songBird absorbs (songBird team)
- songBird should absorb ACME cert lifecycle from `beardog-acme`
- songBird already has `rustls-rustcrypto` in its own dep tree
- songBird's `universal-ipc` is the right home for TLS termination
- bearDog's BTSP handles encrypted IPC without TLS

### Phase 3 — Excise from bearDog
- Once songBird ships TLS/ACME, remove:
  - `crates/beardog-acme/` entirely
  - `crates/beardog-tunnel/src/tcp_ipc/tls.rs`
  - `crates/beardog-cli/src/handlers/server/gateway.rs`
  - `crates/beardog-cli/src/handlers/server/acme.rs`
  - `tls-gateway` feature from all `Cargo.toml` files
  - `rustls`, `tokio-rustls`, `rustls-pki-types`, `rustls-rustcrypto` from workspace deps
  - `reqwest` from `beardog-acme` (crate deleted)
- Archive to ecoPrimals fossil record

### What songBird needs from bearDog for TLS
- **Nothing new**. songBird already delegates crypto to bearDog via JSON-RPC.
- JWS signing for ACME account keys → `crypto.sign_ed25519` or `crypto.sign_ecdsa_p256`
- CSR generation → bearDog already has `p256` + `x509-cert`
- HKDF/HMAC for session keys → `beardog-crypto` or JSON-RPC `crypto.hkdf`

---

## Impact on Glacial Goals

| Goal | Impact |
|------|--------|
| **G5 (Chimera)** | No impact — `libtower.so` doesn't include TLS |
| **G6 (crates.io)** | **Unblocked** — git dep gone from default build; `cargo publish --dry-run` can proceed for all Tier 1-2 crates |
| **G1 (Tower on Windows)** | No impact — Windows uses named pipes + TCP, not TLS |
| **G2 (Tower on Android)** | No impact — Android uses abstract sockets + TCP |

---

## For songBird Team

The `beardog-acme` crate is self-contained and well-factored (2,574 LOC, 4 modules:
account, challenge, client, storage). It uses `reqwest` for ACME directory HTTP and
`p256` + `ed25519-dalek` for JWS account key signing. Those crypto ops should be
delegated to bearDog via Tower Atomic (`crypto.sign_ecdsa_p256`) when songBird
absorbs it.

The TLS termination code in `tcp_ipc/tls.rs` is a thin rustls `ServerConfig` wrapper
(244 LOC). songBird's `universal-ipc` already has the transport abstraction for this.

**No rush** — bearDog's BTSP provides encrypted IPC today. The TLS path was for H2-10
(direct HTTPS without Cloudflare), which is songBird's sovereign transport mission.

---

*Wave 155b. bearDog = crypto. songBird = transport. Clean cut.*
