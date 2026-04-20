<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Roadmap

**Updated**: April 20, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.93.0, `rust-toolchain.toml` pinned)
- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 100 JSON-RPC methods (semantic naming; ionic bond lifecycle, consent gate, contract signing)
- 0 clippy warnings (pedantic + nursery + all cast lints warn + `doc_markdown` warn + `missing_errors_doc` warn + unwrap/expect warn, workspace-centralized)
- 0 missing documentation warnings (all public items documented, all `# Errors` sections present)
- 0 unsafe code blocks (`forbid(unsafe_code)` workspace-wide)
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 1000 lines of code (production)
- 14,786+ tests passing (concurrent; 35 `#[serial]` in `beardog-production`)
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
- **Stadial parity gate (Wave 53)** — Complete: all `#[async_trait]` removed in favor of native `async fn` in traits; `async-trait` dependency removed from every `Cargo.toml`; finite-implementor `dyn Trait` sites replaced with enum dispatch for monomorphized async routing; 14,786+ tests, Clippy and rustdoc `-D warnings` clean

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

### Deep Debt: deny.toml Cleanup, Final Workspace Dep Normalization — DONE (Wave 63)

`cargo deny` passes 4/4 clean after removing 3 stale skips and unused wrappers. 8 explicit dep pins in `crates/beardog/Cargo.toml` normalized to workspace. Comprehensive survey confirms: 0 production files >800 LOC, 0 unsafe, 0 TODO, 0 C deps compiled, all mocks gated.

### primalSpring Phase 45 Audit: Sign→Verify Roundtrip, Base64 Standardization — DONE (Wave 62)

`crypto.sign` now returns `public_key` (standard base64), enabling sign→verify roundtrip via IPC. All Ed25519 output standardized from hex to standard base64 across capability announcements, ionic bonds, and contract signing. Backward-compatible verification (accepts hex or base64). Resolves primalSpring guidestone BD-PG-01 and BD-PG-02.

### Deep Debt: Workspace Dep Normalization, Cross-Arch Fix — DONE (Wave 61)

40+ explicit dep pins across 9 crates normalized to `{ workspace = true }`. Cross-arch compilation fixed (macOS/iOS/Windows/WASM `PlatformSocket::bind` return type, platform HSM `vec![]` initialization). Deep debt survey: zero production files >800 LOC, zero unsafe, zero TODOs. 14,786+ tests, 0 failures.

### Documentation Cleanup, Clippy Fixes — DONE (Wave 60)

All 7 root docs updated with Wave 58/59 entries. Enum dispatch count corrected to 20. 56 CLI receipt artifacts cleaned. 2 clippy fixes.

### Deep Debt: Enum Dispatch, Workspace Deps, Test Refactoring — DONE (Wave 59)

`Box<dyn ProtocolHandler>` → `ProtocolHandlerBackend` enum; `Box<dyn AsyncStream>` → `IpcStream` enum (20 dispatch types total). 21 explicit dep pins normalized to workspace. 7 `#[allow()]` → `#[expect()]`. 2 test files >800 LOC smart-refactored. 14,786+ tests, 0 failures.

### primalSpring Audit: BTSP Documentation, Cleartext Bypass — DONE (Wave 58)

`BEARDOG_FAMILY_SEED` documented in README with security modes table. Cleartext JSON-RPC bypass documented and advertised in `capabilities.list` (`cleartext_methods` array). Resolves spring audit findings.

### serde_yaml Elimination + Deep Debt — DONE (Wave 56)

`serde_yaml` (deprecated, uses `unsafe-libyaml`) removed from all crates and workspace. YAML config paths return deprecation errors; TOML/JSON remain. Sole production `Box<dyn Error>` converted to typed `BearDogError`. `#[allow()]` migrated to `#[expect()]`. 3 test files >900 LOC smart-refactored into domain modules. 14,786+ tests, 0 failures.

### async-trait Lockfile Elimination — DONE (Wave 55)

`async-trait` fully eliminated from `Cargo.lock` by removing unused `hickory-resolver` from `beardog-core` and never-enabled `tarpc` optional dep from `beardog-ipc`. Banned in `deny.toml`. BearDog clears stadial gate as 13th/13 primal.

### Deep Debt Pass — DONE (Wave 54)

Smart-refactored 2 production files over 800 LOC by domain concern. Dependency cleanup: workspace pins unified, `gethostname` consolidated, `syn v1` eliminated. All mocks verified behind `#[cfg(test)]`. Zero unsafe, zero TODOs, zero production mocks. 14,786+ tests, 0 failures.

### Stadial Parity Gate — DONE (Wave 53)

Native async traits (RPITIT) across the workspace, zero `async-trait` in manifests, enum-backed dispatch for closed backend sets, quality gates unchanged (14,786+ tests; Clippy and rustdoc `-D warnings`).

### Test Coverage to 90% — DONE (Wave 20)

Coverage reached **90.51% line** (14,786+ tests passing).

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

**Last Updated**: April 20, 2026
