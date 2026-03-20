# Changelog

All notable changes to the BearDog security platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### March 20, 2026 -- Concurrency Architecture & Dependency Modernization

- **Dependency Injection architecture** — Eliminated global mutable state (process_env overlay)
  across ALL crates. `Default` impls are pure (no I/O). `from_env()` uses `std::env::var`
  (read-only, thread-safe). `from_env_provider()` accepts closures for injectable testing.
- **330 → 15 `#[serial_test::serial]` annotations** — Only chaos/fault tests remain serialized.
  All other tests run fully concurrent at `--test-threads=8`.
- **Dependency modernization**:
  - `trust-dns-resolver` → `hickory-resolver` 0.25 (trust-dns was unmaintained)
  - `bincode` → `postcard` 1.0 (bincode was unmaintained)
  - `validator` 0.18 → 0.20 (drops unmaintained `proc-macro-error`)
  - `cargo update` applied across all transitive deps
- **`cargo deny` passes all 4 checks** (advisories, bans, licenses, sources):
  - RSA Marvin Attack documented as accepted risk (no upstream fix)
  - 27 transitive duplicate crates skipped (RustCrypto ecosystem version split)
  - `deny.toml` simplified: AGPL-3.0-only in global allow, per-crate exceptions removed
- **Hanging tests eliminated**: `beardog-deploy` refactored with `CommandRunner` trait +
  `MockAdbCommandRunner`. `show_logs` follow mode has bounded timeout. No more zombie
  `adb` processes.
- **Zero clippy warnings** (pedantic + nursery)
- **Coverage**: 80% → 84% (13,400+ tests passing)
- **Config types refactored**: `SocketPathInputs`, `IdentityInputs`, `SelfKnowledgeInputs`,
  `EcosystemListenerEnvInputs`, `ConstraintEnforcementPolicy`, `HsmAutoInitConfig`,
  `DiscoverSocketEnv`, `UnixListenHints`, `IdentityHints`, `ResourceLimits`, and many more
  now flow through the call chain instead of reading globals.

### March 19, 2026 -- Deep Debt Execution & Stub Evolution

- **Zero source clippy warnings**: Fixed all 49 remaining library warnings (redundant borrows,
  `clone_from`, `unwrap_or_else`, `let...else`, redundant closures, format appends, etc.)
- **58 unfulfilled `#[expect]` attributes** fixed (over-converted `#[allow]` reverted or removed)
- **Production stubs evolved to real implementations**:
  - Threat incident handler: typed lifecycle engine with `ManagedIncident`, `BTreeMap` storage, `tracing`
  - Auth genetics: BLAKE3-based nuclear + mitochondrial lineage verification
  - Auth ecosystem: dynamic capability discovery from node registry + security policy
  - Auth consensus: in-memory `BTreeMap<String, ConsensusNodeRecord>` with health tracking
  - Safe memory: real `zeroize::Zeroize` + `ZeroizeOnDrop` for `SafePinnedBuffer`, `SensitiveByteBuf`
- **`#[allow]` → `#[expect(reason)]` sweep** across beardog-core, beardog-tunnel, beardog-utils, beardog-security
- **Test coverage improved**: beardog-ipc 72% → 86%, beardog-core 59% → 62%
- **Failing doctests fixed** in beardog-utils `mock_time.rs` (missing trait import)
- **`primal_discovery.rs`** tests extracted to separate file (1026 → 756 lines)
- **Dockerfile** updated: MSRV 1.85, removed C dependencies, fixed license label
- **`.pedantic_clippy.toml`** MSRV updated to 1.85.0
- 1,072 tests passing, 0 failures, 0 files over 1000 lines

### March 19, 2026 -- Deep Compliance & Coverage Wave 2

- SPDX license headers (`AGPL-3.0-only`) added to 1,634 .rs files
- Removed ~130 production `#[allow(dead_code)]` — idiomatic underscore prefixes
- Removed redundant `#[allow]` from beardog-types (workspace config handles them)
- Removed `sysinfo` C dependency (ecoBin violation) — was unused in beardog-deploy
- blake3 `pure` feature enforced across all 13 showcase crates
- `pprof` made optional behind `profiling` feature flag in benchmarks
- Platform FFI (Android NDK, iOS Security.framework) documented as out-of-ecoBin-scope
- IPC hot path optimized: `take()` replaces triple `clone()` on request IDs
- Added ~250 new tests: beardog-core 60%→74%, beardog-ipc 30%→41%
- Prefixed unused struct fields with `_` instead of `#[allow(dead_code)]`
- Migration plan comments added to all `#[allow(deprecated)]` usage sites
- 8,542+ tests passing across 29 crates (0 failures)

### March 19, 2026 -- Deep Compliance & Coverage Wave 1

- Fixed 46 clippy errors in beardog-core (struct_excessive_bools, items_after_statements, etc.)
- Standardized AGPL-3.0-only license across all 29 Cargo.toml files
- Workspace lint inheritance (`[lints] workspace = true`) applied to all crates
- Refactored `ProductionConfig` (5 bools → enum) and `SecurityConfig` (4 bools → Option sub-configs)
- Removed `async` from 14 functions that had no `.await`
- Hardcoded primal names evolved to `PRIMAL_NAME` env var + `CARGO_PKG_NAME` fallback
- Zero-key HSM stub replaced with HKDF-SHA256 derived from `BEARDOG_HSM_MASTER_KEY`
- Smart-refactored 3 oversize files (canonical_examples, enforcement, genesis) into submodules
- Replaced 15 production TODOs with doc comments and tracing warnings
- Platform stubs deprecated with `#[deprecated]` guidance

### February 11, 2026 -- Deep Debt Evolution Session

- Smart refactored `quantum_crypto.rs` (1000+ LOC) into modular structure:
  - `quantum_crypto/mod.rs` — module orchestration
  - `quantum_crypto/types.rs` — core types and algorithm enums
  - `quantum_crypto/kem.rs` — Kyber Key Encapsulation Mechanism
  - `quantum_crypto/signatures.rs` — Dilithium and SPHINCS+ signatures
  - `quantum_crypto/engine.rs` — quantum crypto engine orchestration
  - `quantum_crypto/tests.rs` — isolated test module
- Fixed all Clippy warnings (pedantic level, 0 errors)
- Removed corrupted `audit_logging.rs` dead code from `beardog-security`
- Fixed flaky tests with `#[serial_test::serial]` for env var tests
- Improved error handling in `IntegrationEngine` (removed panic-prone `unwrap()`)
- Verified constant-time comparisons for all secret operations
- Root documentation cleaned and metrics updated (12,751+ tests)

### February 11, 2026 -- Relay Authorization & Documentation

- `relay.authorize` JSON-RPC method: lineage-gated relay authorization for coordinated punch
- BearDog verifies family membership before allowing Songbird relay sessions
- Blake3-based lineage proof verification for cross-family relay trust
- `relay` capability type added to `discover_capabilities` response
- 12 new relay handler tests (family match, denial, proof validation, isolation)
- Specs directory rewritten with current metrics (was stale since Oct 2025)
- Primal contracts updated with relay, beacon, and secrets API documentation

### February 11, 2026 -- Test Coverage Push

- Expanded test coverage from ~62% baseline to 78.6% overall
- 9 of 11 measured crates now above 90% coverage target
- Added 3,500+ new tests across all crates
- Root documentation cleaned and consolidated

### February 9, 2026 -- Deep Debt Evolution & Feature Completion

- Multi-family socket support: `--family-id` flag creates per-family sockets
- `discover_capabilities` JSON-RPC method for ecosystem consistency
- Secret storage: `secrets.store`, `secrets.retrieve`, `secrets.list`, `secrets.delete`
- Hardcoded primal names eliminated (all handlers use `get_primal_name()`)
- Smart refactoring: `crypto_handlers_genetic` (2193 lines split into 3 focused modules)
- Production mock audit: dead `stub_types` removed, all mocks isolated to `#[cfg(test)]`
- Cross-primal hardcoding eliminated: `upa_client.rs` uses `discover_upa_provider()`
- All production `expect()`/`unwrap()` eliminated (zero panic paths)
- `once_cell` migrated to `std::sync::LazyLock` across 3 crates
- `base64` 0.21 upgraded to 0.22 (dedup)
- Pre-existing test flakes fixed (env var race conditions)

### February 6, 2026 -- Tor Phase 2 Evolution

- Tor v3 onion address derivation + ntor handshake + cell crypto (8 RPC methods)
- Ed25519, X25519, SHA3-256, ChaCha20, HMAC-SHA256, HKDF-SHA256 primitives

### January 31, 2026 -- ecoBin v2.0 Roadmap

- Q1 2026 ecoBin v2.0 roadmap published
- README biomeOS socket documentation

### January 27-30, 2026 -- Deep Debt Sessions

- Deep debt rounds 12-20 completed (LEGENDARY milestone)
- Panic safety: `debug_assert!` guards, unwrap fixes
- XDG compliance: configurable paths, env vars
- Async traits: `PrimalDiscoveryClient` async
- Feature flags: missing features added
- Workspace versions: 11 crates migrated

### January 24, 2026 -- Smart File Refactoring

- **v0.23.0**: TLS module refactored (2174 lines to 4 focused modules, 34% code reduction)
- **v0.22.0**: 7-phase architectural evolution (unsafe audit, hardcoding removal, mock isolation)
- **v0.21.0**: Crypto handlers refactored (2499 lines to 7 semantic domain modules)

### January 23, 2026 -- RFC 8448 Validation

- **v0.18.0**: Enhanced debug logging for TLS key derivation
- **v0.15.1**: RFC 8448 validation complete (BearDog TLS 1.3 proven compliant)

### January 22, 2026 -- HTTPS & Crypto Expansion

- **v0.15.0**: `tls.derive_handshake_secrets` (RFC 8446 Section 7.1)
- **v0.14.0**: 100% test pass rate achieved (17 failures fixed)
- **v0.13.1**: RFC 8446 full compliance with transcript hash support
- **v0.13.0**: Phase 8 comprehensive testing (20 tests: unit, E2E, chaos, fault)
- **v0.13.0**: `tls.derive_application_secrets` -- Pure Rust HTTPS complete
- Phase 7: Legacy compatibility (bcrypt, scrypt, SHA-1, SHA3-256, HMAC variants)
- Phase 6: Production gaps closed (SHA-256/384/512, ECDH P-256/P-384, AES-GCM, Argon2id)
- Phase 5: Genetic crypto integration (lineage keys, entropy mixing, verification)

### January 21, 2026 -- Handler Registry & BTSP

- **v0.14.0**: Handler registry 100% complete (legacy router deleted, -1514 lines)
- BTSP unified evolution complete (type system, handler extensions)
- Handler registry pattern: 7 modular handlers, 47 RPC methods
- Dead code cleanup: 315 unreachable lines removed

### January 19, 2026 -- UniBin & Tower Atomic

- UniBin commands: server, daemon, doctor, client (100% complete)
- 151 comprehensive tests (unit, E2E, chaos, fault)
- `beardog-tower-atomic` crate: Pure Rust IPC (Unix socket + JSON-RPC)
- Capability-based discovery (zero vendor hardcoding)

### January 18, 2026 -- Crypto API

- 8 Pure Rust crypto operations for Songbird TLS integration
- Ed25519 sign/verify, X25519 key exchange, ChaCha20-Poly1305, Blake3, HMAC-SHA256
- 52 comprehensive crypto tests
- JSON-RPC crypto API over Unix sockets

---

## [0.9.0] -- November-December 2025

### Added

- Universal HSM Discovery Engine with 8+ HSM type support
- BSTP (BearDog Secure Tunnel Protocol) for encrypted communication
- Genetic entropy system with self-evolving key management
- Protocol-agnostic design (HTTP REST, JSON-RPC 2.0, tarpc)
- Runtime primal discovery via mDNS and service registry
- Hardware security module integration (YubiKey, TPM 2.0, StrongBox, Secure Enclave)
- 8,174+ tests with 70+ chaos tests
- Comprehensive error handling system
- Zero-configuration capability discovery

### Changed

- Eliminated all hardcoded values from production code
- Evolved to modern idiomatic Rust patterns throughout
- Refactored large files to maintain <1000 lines per file discipline
- 23 crates with zero circular dependencies

---

For detailed session notes, see `archives/` organized by date and topic.
