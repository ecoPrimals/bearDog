# BearDog Roadmap

**Updated**: March 24, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.93.0, `rust-toolchain.toml` pinned)
- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 91+ JSON-RPC crypto methods (semantic naming)
- 0 clippy warnings (pedantic + nursery + cast + unwrap/expect warn, workspace-centralized)
- 0 missing documentation warnings (all public items documented)
- 0 unsafe code blocks (`forbid(unsafe_code)` workspace-wide)
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 1000 lines of code (production)
- 14,447+ tests passing (fully concurrent, zero sleeps in non-chaos)
- 87.35% line coverage (llvm-cov workspace)
- Dependency Injection architecture — pure `Default`, `from_env()` at boundaries
- Zero `#[serial_test::serial]` — all tests concurrent via unique isolated resources
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
- SPDX license headers on all 2,026 .rs files (100%)
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

## In Progress

### Test Coverage to 90%

Coverage at 87.35% overall. Top crates above target; remaining crates approaching via targeted test waves.

| Crate | Line Coverage | Status |
|-------|-------------|--------|
| beardog-traits | 99.4% | Above target |
| beardog-capabilities | 98.0% | Above target |
| beardog-auth | 93.0% | Above target |
| beardog-utils | 92.3% | Above target |
| beardog-genetics | 89.9% | At target |
| beardog-ipc | 86.0% | Approaching |
| beardog-core | ~85% | Approaching |
| beardog-discovery | ~85% | Boosted (Wave 11) |
| beardog-types | ~84% | Boosted (Wave 11) |
| beardog-installer | ~84% | Boosted (Wave 11) |
| beardog-cli | ~83% | Boosted (Wave 11) |
| beardog-tunnel | ~83% | Boosted (Wave 11 + fault injection) |
| beardog-deploy | ~82% | Boosted (Wave 11) |

### primalSpring Capability Audit Fixes

~~Three quick fixes from the primalSpring capability audit~~ — **DONE (Wave 10)**:
1. ~~Register `health.liveness` and `health.readiness` method aliases~~ — already registered
2. ~~Register `capabilities.list` method alias~~ — already registered; added `capability.list` + `primal.capabilities`
3. ~~Register bare crypto method aliases for TLS 1.3 compatibility~~ — already bridged in `HandlerRegistry::route`

---

## Future Work

These items are enhancements — nothing is blocking production use.

### Zero-Copy Hot Path Evolution

~~Deferred pending profiling~~ — **DONE (Wave 11)**: IPC hot paths audited and optimized. `unix_socket_ipc::server.rs` refactored from byte-at-a-time reads to `BufReader::read_until` with reusable buffers. `bytes::Bytes` and `Arc<str>` adoption deferred as JSON-RPC message sizes don't justify the complexity; current buffer reuse eliminates the primary allocation overhead.

### Fault Injection Tests

~~Stub framework exists~~ — **DONE (Wave 11)**: 14 crypto fault injection tests covering adversarial inputs (malformed base64, wrong key/nonce lengths, corrupted ciphertext/signatures, all-zero/all-ones keys, wrong-length DH secrets) for Blake3, ChaCha20-Poly1305, Ed25519, X25519, and Tor ntor handlers.

### Secret Storage Evolution (when NestGate available)

Current in-memory storage backend evolves to persistent NestGate-backed storage via capability discovery. BearDog discovers NestGate's `storage.store` / `storage.retrieve` at runtime. No code changes needed — the discovery pattern is already implemented.

### Graph Security Phase 2-3 (optional)

- Phase 2: Public key infrastructure (storage/retrieval, trust_db integration)
- Phase 3: Signature verification (Ed25519 helpers, chain of custody validation)

### Semantic Method Naming Phase 3 (ecosystem coordination)

Fully generic methods: `crypto.encrypt` + `{"algorithm": "aes-256-gcm"}` instead of algorithm-specific method names. Requires coordination across ecosystem primals via wateringHole standards.

### CI & Docker Scaffold Cleanup

The `.github/workflows/` files and `docker-compose.yml` reference scripts and services from early development that no longer exist (`scripts/run_benchmarks.sh`, `scripts/deploy-staging.sh`, `scripts/migrate-config.sh`, `postgres`/`redis` services in docker-compose). These are aspirational scaffolds that should be reconciled with the actual BearDog architecture (Unix socket IPC, no database dependencies) when CI is activated.

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

**Last Updated**: March 23, 2026
