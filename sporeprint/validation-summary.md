+++
title = "bearDog Validation Summary"
description = "Zero-trust sovereign cryptographic orchestrator — 14,987+ tests, 30 crates, 223 IPC methods, ACME auto-cert, FIDO2/CTAP2, BTSP P3 AEAD"
date = 2026-06-02

[taxonomies]
primals = ["beardog"]
springs = []
+++

## Status

- **14,987+ tests** passing (workspace), 0 failures
- **30 crates** in workspace (`beardog-integration` excluded — overstep)
- **223 JSON-RPC methods** — 215 registry + 8 pre-dispatch gate (see `docs/PRIMAL_CONTRACTS.md` v4.0.0)
- **v0.9.0** — edition 2024, MSRV 1.93.0
- **Pure Rust** — 100% (zero C dependencies), `forbid(unsafe_code)` workspace-wide
- **Clippy** — pedantic + nursery + cast lints + `doc_markdown` + `missing_errors_doc` + unwrap/expect warn
- **Coverage** — 90.51% line (llvm-cov)
- **License** — AGPL-3.0-or-later, SPDX headers on all `.rs` files

## Key Capabilities

| Capability | Description |
|------------|-------------|
| **TLS termination** | `rustls` X.509 with per-IP sliding-window rate limiter (H2-10 sovereignty) |
| **ACME auto-cert** | `beardog-acme` — RFC 8555 HTTP-01 challenge, cert storage, hot-reload via `Arc` swap |
| **BTSP P3 AEAD** | ChaCha20-Poly1305 encrypted tunnel on all 13 primals |
| **Ionic tokens** | Ed25519-signed, scoped, TTL-aware (issue/verify/revoke) |
| **Cross-family contracts** | `crypto.contract.propose/countersign/verify` — multi-party signing |
| **FIDO2/CTAP2** | `beardog.fido2.discover/register/authenticate` — hardware-attested auth |
| **Seed fingerprint** | `BLAKE3(HMAC-SHA256)` for Tower atomic validation (GAP-16) |
| **HKDF + HMAC** | Key derivation + message authentication for crypto delegation |
| **Shadow metrics** | Sovereign vs commercial parity tracking (p50/p95/p99, cutover criteria) |

## Architecture

- **DI-based** — pure `Default`, `from_env()` at boundaries
- **Self-knowledge only** — discovers other primals at runtime via capability resolution
- **Multi-transport** — UDS (primary), TCP (fallback), WebSocket (tunnel)
- **Signal handling** — SIGINT + SIGTERM with explicit socket cleanup
- **Stale socket prevention** — `unlink-before-bind` at 3 layers + explicit `stop()` on shutdown

## Shadow Runs (Wave 24)

| Shadow | Status | Metric |
|--------|--------|--------|
| S1: TLS termination | LIVE — 10ms sovereign vs 120ms Cloudflare | ACME Phase 2 shipped |
| S4: Auth / JupyterHub | Design complete | `specs/JUPYTERHUB_DUAL_AUTH_INTEGRATION.md` |

## Platform Support

Linux, macOS, Android, Windows, iOS

## See Also

- `STATUS.md` — full metrics and per-crate coverage
- `CHANGELOG.md` — wave-by-wave evolution history
- `specs/ACME_TLS_INTEGRATION_PATH.md` — ACME design
- `specs/JUPYTERHUB_DUAL_AUTH_INTEGRATION.md` — JupyterHub auth design
