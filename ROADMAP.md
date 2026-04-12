<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Roadmap

**Updated**: April 11, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.93.0, `rust-toolchain.toml` pinned)
- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 95 JSON-RPC crypto methods (semantic naming; prior 96 was a count error)
- 0 clippy warnings (pedantic + nursery + all cast lints warn + `doc_markdown` warn + `missing_errors_doc` warn + unwrap/expect warn, workspace-centralized)
- 0 missing documentation warnings (all public items documented, all `# Errors` sections present)
- 0 unsafe code blocks (`forbid(unsafe_code)` workspace-wide)
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 1000 lines of code (production)
- 14,761+ tests passing (concurrent; 35 `#[serial]` in `beardog-production`)
- 90.51% line coverage (llvm-cov workspace) — target 90% met
- Dependency Injection architecture — pure `Default`, `from_env()` at boundaries
- `#[serial]` minimized — 35 tests in `beardog-production` (shared `AtomicBool`); all others concurrent
- `cargo deny` passes all 4 checks
- `trust-dns-resolver` → `hickory-resolver`, `bincode` → `postcard`, `validator` 0.20
- Multi-family socket support (`--family-id` flag)
- Encrypted secret storage (family-scoped ChaCha20-Poly1305)
- `discover_capabilities` introspection method
- Tor v3 onion address derivation + ntor handshake + cell crypto
- Dark Forest beacon (zero metadata leakage discovery)
- Universal IPC (Unix sockets, abstract sockets, TCP)
- Android StrongBox integration (complete)
- HSM abstraction (software, PKCS#11, StrongBox)
- All production `unwrap()`/`expect()` eliminated (zero panic paths)
- SPDX license headers on all .rs files (100%)
- ecoBin C-dependency compliance (sysinfo removed, blake3 pure, pprof optional)
- Smart refactoring of oversized files into submodule directories
- All mocks isolated behind `cfg(test)` / `test-utils` feature
- Hardcoding eliminated — capability-based discovery throughout
- `deny.toml` hardened — C deps banned, duplicate versions denied
- `CommandRunner` trait for mocking external commands (`adb`) in tests

### Platform Coverage

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux (x86_64, ARM64) | Unix sockets | Validated |
| macOS (Intel, M-series) | Unix sockets | Validated |
| Android (ARM64) | Abstract sockets + TCP | Ready |
| Windows (x86_64, ARM64) | Named pipes + TCP | Ready |
| iOS (ARM64) | TCP | Ready |

---

## Recently Completed

### Test Coverage to 90% — DONE (Wave 20)

Coverage reached **90.51% line** (14,761+ tests passing).

### primalSpring Composition Fixes — DONE (Wave 18c)

1. TCP read timeout — All NDJSON read sites wrapped with `tokio::time::timeout(30s)`
2. NDJSON wire format documented in README and capability metadata

### Semantic Method Naming — DONE (Wave 19)

Primary `domain.operation` names (`birdsong.encrypt`, `btsp.contact.exchange`, `crypto.derive_onion_address`) with `beardog.*` kept as backward-compatible aliases.

---

## Future Work

These items are enhancements — nothing is blocking production use.

### Zero-Copy Hot Path Evolution

~~Deferred pending profiling~~ — **DONE (Wave 11)**: IPC hot paths audited and optimized. `unix_socket_ipc::server.rs` refactored from byte-at-a-time reads to `BufReader::read_until` with reusable buffers. `bytes::Bytes` and `Arc<str>` adoption deferred as JSON-RPC message sizes don't justify the complexity; current buffer reuse eliminates the primary allocation overhead.

### Fault Injection Tests

~~Stub framework exists~~ — **DONE (Wave 11)**: 14 crypto fault injection tests covering adversarial inputs (malformed base64, wrong key/nonce lengths, corrupted ciphertext/signatures, all-zero/all-ones keys, wrong-length DH secrets) for Blake3, ChaCha20-Poly1305, Ed25519, X25519, and Tor ntor handlers.

### Secret Storage Evolution (when persistent storage primal available)

Current in-memory storage backend evolves to persistent storage via capability discovery. BearDog discovers any primal offering `storage.store` / `storage.retrieve` at runtime. No code changes needed — the discovery pattern is already implemented.

### Graph Security Phase 2-3 (optional)

- Phase 2: Public key infrastructure (storage/retrieval, trust_db integration)
- Phase 3: Signature verification (Ed25519 helpers, chain of custody validation)

### Semantic Method Naming Phase 3 (ecosystem coordination)

Fully generic methods: `crypto.encrypt` + `{"algorithm": "aes-256-gcm"}` instead of algorithm-specific method names. Requires coordination across ecosystem primals via wateringHole standards.

### CI & Docker Scaffold Cleanup — DONE (Wave 20)

Dead CI workflows (7 files), `docker-compose.yml`, stale profiling scripts, and deployment scripts removed. Moved to `ecoPrimals/infra/wateringHole/fossilRecord/beardog/`. Retained and fixed: `beardog-ci.yml` (pinned 1.93.0, correct workspace commands) and `Dockerfile` (correct binary name, no phantom features).

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

**Last Updated**: April 11, 2026
