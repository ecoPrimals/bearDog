# BearDog Roadmap

**Updated**: March 20, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.85.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.85.0)
- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 91+ JSON-RPC crypto methods (semantic naming)
- 0 clippy warnings (pedantic + nursery, workspace-centralized)
- 0 missing documentation warnings (all public items documented)
- 0 unsafe code blocks (`forbid(unsafe_code)` per-crate; 1 justified exception in `process_env.rs`)
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 1000 lines of code
- Multi-family socket support (`--family-id` flag)
- Encrypted secret storage (family-scoped ChaCha20-Poly1305)
- `discover_capabilities` introspection method
- Tor v3 onion address derivation + ntor handshake + cell crypto
- Dark Forest beacon (zero metadata leakage discovery)
- Universal IPC (Unix sockets, abstract sockets, TCP)
- Android StrongBox integration (complete)
- HSM abstraction (software, PKCS#11, StrongBox)
- All production `unwrap()`/`expect()` eliminated (zero panic paths)
- SPDX license headers on all .rs files
- ecoBin C-dependency compliance (sysinfo removed, blake3 pure, pprof optional)
- Smart refactoring of oversized files into submodule directories
- All mocks isolated behind `cfg(test)` / `test-utils` feature
- Hardcoding eliminated — capability-based discovery throughout
- `deny.toml` hardened — C deps banned, duplicate versions denied

### Platform Coverage

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux (x86_64, ARM64) | Unix sockets | Validated |
| macOS (Intel, M-series) | Unix sockets | Validated |
| Android (ARM64) | Abstract sockets + TCP | Ready |
| Windows (x86_64, ARM64) | Named pipes + TCP | Ready |
| iOS (ARM64) | TCP | Ready |

---

## In Progress

### Test Coverage to 90%

Coverage measured via `cargo-llvm-cov`. Top crates at or above target; lower crates need integration tests.

| Crate | Line Coverage | Status |
|-------|-------------|--------|
| beardog-utils | 92.3% | Above target |
| beardog-genetics | 89.9% | At target |
| beardog-ipc | 86.0% | Above target |
| beardog-auth | 84.0% | In progress |
| beardog-types | 80.8% | In progress |
| beardog-errors | 77.0% | In progress |
| beardog-security | 73.7% | Platform-gated code |
| beardog-tunnel | 71.0% | In progress |
| beardog-core | 62.2% | In progress |

### primalSpring Capability Audit Fixes

Three quick fixes from the primalSpring capability audit:
1. Register `health.liveness` and `health.readiness` method aliases
2. Register `capabilities.list` method alias
3. Register bare crypto method aliases for Songbird TLS 1.3 compatibility

---

## Future Work

These items are enhancements — nothing is blocking production use.

### Zero-Copy Hot Path Evolution

Deferred pending profiling to identify actual hot paths. `bytes::Bytes` and `Arc<str>` ready for adoption where measurements justify it.

### Fault Injection Tests

Stub framework exists. Needs fleshing out with chaos engineering scenarios for crypto operations under adverse conditions.

### Secret Storage Evolution (when NestGate available)

Current in-memory storage backend evolves to persistent NestGate-backed storage via capability discovery. BearDog discovers NestGate's `storage.store` / `storage.retrieve` at runtime. No code changes needed — the discovery pattern is already implemented.

### Graph Security Phase 2-3 (optional)

- Phase 2: Public key infrastructure (storage/retrieval, trust_db integration)
- Phase 3: Signature verification (Ed25519 helpers, chain of custody validation)

### Semantic Method Naming Phase 3 (ecosystem coordination)

Fully generic methods: `crypto.encrypt` + `{"algorithm": "aes-256-gcm"}` instead of algorithm-specific method names. Requires coordination across Songbird, Squirrel, NestGate.

---

## Design Principles

These guide all BearDog evolution:

1. **Pure Rust** — No C dependencies, ever
2. **Smart Refactoring** — Domain-driven module boundaries, not arbitrary splits
3. **Safe Code** — Zero `unsafe`, zero production panics
4. **Agnostic Config** — Environment variables and capability discovery, no hardcoding
5. **Runtime Discovery** — Primals discover each other at runtime, never hardcode names
6. **Honest Code** — No production mocks, clear capability boundaries

---

**Last Updated**: March 20, 2026
