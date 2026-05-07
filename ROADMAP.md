<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Roadmap

**Updated**: May 7, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.93.0, `rust-toolchain.toml` pinned)
- 100% Pure Rust (zero C dependencies, RustCrypto suite)
- 100+ JSON-RPC methods (semantic naming; ionic bond lifecycle, consent gate, contract signing, lineage queries)
- 0 clippy warnings (pedantic + nursery + all cast lints warn + `doc_markdown` warn + `missing_errors_doc` warn + unwrap/expect warn, workspace-centralized)
- 0 missing documentation warnings (all public items documented, all `# Errors` sections present)
- 0 unsafe code blocks (`forbid(unsafe_code)` workspace-wide)
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 800 lines of code (production)
- 12,610 tests passing (concurrent; 35 `#[serial]` in `beardog-production`)
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
- **Stadial parity gate (Wave 53)** — Complete: all `#[async_trait]` removed in favor of native `async fn` in traits; `async-trait` dependency removed from every `Cargo.toml`; finite-implementor `dyn Trait` sites replaced with enum dispatch for monomorphized async routing; Clippy and rustdoc `-D warnings` clean

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

### Stale Alias Cleanup, Dep Pruning & Feature Gating — DONE (Wave 85)

4 confusing/dead re-export aliases removed (`UnifiedBearDogConfig` root alias, `PrimaryUnifiedBearDogConfig`, `WorkingUnifiedConfig`, `capability_router`). Unused `rsa` dep removed from `beardog-security`. `x509-parser` feature-gated behind `tls-x509` (default-on) in `beardog-tunnel`.

### O(1) Handler Dispatch, getrandom Alignment & Workspace Hygiene — DONE (Wave 84)

`HandlerRegistry` builds a `HashMap<&str, usize>` at init for O(1) method→handler dispatch (was O(n)). `getrandom` 0.2→0.3 aligned with `rand 0.9`/`rand_core 0.9`. 5 internal crates migrated to `{ workspace = true }`, 3 zero-consumer workspace dep entries removed.

### Bond Persistence Default Upgraded to Capability Discovery — DONE (Wave 83)

`HandlerRegistry::new()` now creates `CapabilityDiscoveryBondPersistence` instead of `InMemoryBondPersistence`. Sealed bonds auto-persist to NestGate/loamSpine `bonding.ledger` RPCs when available, in-memory fallback otherwise. Phase 58 primalSpring audit: 4/4 gaps resolved.

### Deep Debt: Orphan Code Removal, Stale Lints, Workspace Hygiene — DONE (Wave 82)

716-line orphan `discovery.rs` deleted (never in module tree). 6 deprecated zero-caller type aliases removed. 2 stale `#[expect(dead_code)]` fixed. 8 `#[allow]` given `reason`. `base64-url` centralized. HSM docs clarified. Net -783 LOC.

### BTSP Phase 3: Encrypted Frame I/O Transition — DONE (Wave 81)

Phase 3 interop gap fixed. After `btsp.negotiate` selects a non-null cipher, BearDog now transitions the connection to encrypted frame I/O (`Phase3Session` with random 12-byte nonces, ChaCha20-Poly1305 AEAD). Both NDJSON loops (`handle_jsonrpc_universal`, `handle_jsonrpc_ndjson_loop`) detect the negotiate and upgrade. BearDog is the Tower reference — this fix pattern cascades to all other primals.

### Deep Debt: Dead Code Removal, Deprecated Symbols, Flaky Test Fix — DONE (Wave 80)

6 dead production code items removed (orphaned `HsmSource`, `DeployConfig`, `probe_service_endpoint`, 3 `SovereigntyManager` fields). Deprecated `BearDogResult` and `PrimalTypeMigrationHelper` removed (zero callers). Flaky `key_export_roundtrip` test fixed — HOME mutex scope extended. Zero test failures across full workspace.

### Deep Debt: Workspace Dependency Drift, Build Script Cleanup — DONE (Wave 79b)

16 workspace deps normalized. Stale `android_native` cfg emission removed from build.rs. `tempfile` reclassified to dev-dep in beardog-security. Full audit: 0 unsafe, 0 `todo!()`, 0 `async_trait`, 0 production mocks outside test gates.

### BTSP Phase 3: `btsp.negotiate` Server-Side Implementation — DONE (Wave 79)

`btsp.negotiate` JSON-RPC method handler implemented. ChaCha20-Poly1305 cipher negotiation with NULL cipher fallback. Phase 3 session key derivation via HKDF-SHA256(handshake_key, client_nonce || server_nonce) with directional info strings matching primalSpring reference implementation. 12 new tests. BTSP methods 35 → 36. primalSpring `#[ignore]` integration tests will auto-validate on next plasmidBin harvest.

### Deep Debt: Production Mock Isolation, Workspace Drift & Dead Feature — DONE (Wave 78b)

Production mocks (`IpcTestHandler`, `IpcFailing*`, `IpcHandlerBackend`, `IpcServer`) gated behind `#[cfg(test)]`. 10 workspace deps centralized. Dead `android_native` feature removed. Deprecation form normalized. Audit: 0 production mocks outside test gates, 0 `unreachable!()` in release binaries.

### primalSpring Phase 56c Audit Response — DONE (Wave 78)

Both audit items confirmed stale/resolved: `async-trait` 49→0 (Wave 53-55), `crypto.sign_contract` wired since Wave 42, `syn v1` eliminated. Zero code changes.

### Deep Debt Pass: `#[allow]` Reason Hygiene & Workspace Dependency Drift — DONE (Wave 77b)

19 bare `#[allow]` given `reason` metadata. 4 internal crates added to `[workspace.dependencies]`, 11 path pins normalized. `mockito` unified to workspace. Deep audit confirmed zero remaining unsafe, production mocks, `#[async_trait]`, `Box<dyn Error>`, or `todo!()`.

### `crypto.derive_public_key` for biomeOS Coordination Keys — DONE (Wave 77)

New RPC method derives Ed25519 public key for a named purpose from `FAMILY_SEED`. Enables biomeOS Neural API auto-derivation of coordination keys. Methods 101→102 (CryptoHandler 97, IonicBondHandler 8). 7 new tests. BTSP Phase 3 deferred.

### primalSpring Phase 56 Audit: GAP-23 Reclassified, IONIC-RUNTIME Confirmed — DONE (Wave 76/76b)

Exhaustive UDS accept-path audit confirmed zero path-dependent behavior in BearDog's socket handling — GAP-23 reclassified to primalSpring (same class as GAP-22). IONIC-RUNTIME confirmed resolved since Wave 42 (`crypto.sign_contract` fully wired). Error messages improved with parameter format guidance.

### Deep Debt: Purpose-Key Module Extraction, Dependency Drift & Stale Feature Cleanup — DONE (Wave 75)

Refactored `aliases_and_beardog.rs` (927→452 LOC) by extracting NUCLEUS purpose-key ops into `purpose_key.rs`. Normalized 2 workspace dep drifts (`tokio-test`, `wiremock`). Removed dead `dns-sd` feature gate. File-size threshold tightened from 1000 to 800 LOC — zero files exceed it.

### primalSpring Phase 55b — Lazy Purpose-Key Derivation & Purpose-Based Encrypt/Decrypt — DONE (Wave 74)

Added lazy purpose-key derivation in `secrets.retrieve` (auto-derives NUCLEUS purpose keys from `FAMILY_SEED` on first access) and `purpose` parameter in `crypto.encrypt`/`crypto.decrypt` for NUCLEUS envelope operations. Resolves Phase 55b audit: lights up end-to-end encryption for NestGate and Squirrel.

### Deep Debt: Orphan Cleanup, Benchmark Normalization, Lint Hygiene & Stale Comment Purge — DONE (Waves 73/73b)

Deleted 6 orphan test files (2,277 LOC), normalized 9 benchmark deps to workspace, added missing `[lints] workspace = true`, added `reason` fields to all bare `#[allow(deprecated)]` and `#[expect()]`. Deleted tarpc debris (`tests/tarpc_e2e_tests.rs`, `docs/references/QUICK_REFERENCE_TARPC.md`). Purged stale `REMOVED:` comment blocks from 7 `Cargo.toml` files.

### primalSpring Phase 55 Audit — NUCLEUS Purpose Key Derivation & Signed Registrations — DONE (Wave 72)

Added `crypto.derive_purpose_key` (HMAC-SHA256 purpose-v1 convention) and `crypto.sign_registration` (canonical Ed25519 signing of ipc.register payloads). Methods 99→101. Resolves primalSpring gaps: single-family self-derivation and unsigned service registrations.

### Workspace Hygiene, Debris Cleanup & Broken Links — DONE (Wave 71)

Normalized 5 workspace dependencies, deleted 2,685 LOC of orphan code (beardog-errors `unified_error_system/` + unreferenced tests), renamed 2 date-stamped test files, fixed 15+ broken cross-references across showcase/docs/specs. Full audit confirmed 70+ deprecated items are intentional v0.10.0 migration targets, zero `todo!()`/`unimplemented!()`, all feature gates valid.

### Deep Debt Audit: Clean Bill of Health — DONE (Wave 70b)

Comprehensive 8-dimension audit: zero async-trait, zero unsafe (28 crates `#![forbid(unsafe_code)]`), zero production `Box<dyn Error>`, zero ring, zero ungated mocks, zero TODO/FIXME, zero hardcoded peer primal names. All >800L files are test-only. Renamed stale date-stamped test file. Enforced `reason` on all remaining bare `#[allow()]`.

### ludoSpring Audit: Identity Fallback Alignment — DONE (Wave 70)

Aligned `get_node_id_with()` and `get_family_id_with()` fallbacks to match `PrimalIdentity::from_env()` and `SocketConfig` (ephemeral `standalone-{uuid}` and `"standalone"` instead of `"unknown"`). Clarified in README that only ONE of `NODE_ID`/`BEARDOG_NODE_ID` is needed. Resolves ludoSpring launcher audit.

### Deep Debt: Self-Knowledge, Allow-Reasons, Blake3 Pure — DONE (Wave 69b)

Removed `skunkBat` from capabilities wire data. Added structured `reason` to all `#[allow()]`. Aligned 5 showcase `blake3` to `features=["pure"]`. Full audit: 0 unsafe, 0 TODO, 0 other-primal refs in production, 0 ungated mocks, 0 ring/openssl. 101 CryptoHandler + 8 IonicBondHandler methods. 15,000+ tests.

### Lineage Semantic IPC Methods — DONE (Wave 69)

`lineage.list`, `lineage.verify`, `lineage.get` semantic IPC methods for verifiable lineage queries. `crypto.sign_contract` confirmed fully wired via `IonicBondHandler`. Enables downstream thymic selection and multi-gate federation.

### primalSpring Launcher Audit: NODE_ID Env Var Documentation — DONE (Wave 68)

Documented `NODE_ID`/`BEARDOG_NODE_ID` env var requirements. Standardized precedence: `BEARDOG_*` first across `primal_identity.rs`, `handlers/utils.rs`, `socket_config.rs`.

### Deep Debt: Hardcoded Primal Name Cleanup, Full Audit — DONE (Wave 67)

2 hardcoded `biomeOS` references removed. Full audit: 0 unsafe, 0 TODO/FIXME, 0 async-trait, 0 production files >800 LOC, all mocks gated, all ports config-driven, `Box<dyn Error>` tests/docs only.

### primalSpring Audit: crypto.public_key, Sign→Verify Roundtrip — DONE (Wave 66)

New `crypto.public_key` method for standalone public key retrieval. Router tests updated to use sign response `public_key`. Resolves primalSpring `crypto:ed25519_verify` SKIP. 99 crypto methods, 14,928+ tests.

### Deep Debt: Workspace Dep Normalization, server.rs Smart Refactor — DONE (Wave 65)

6 dep pins normalized to workspace references (`mdns-sd` ×4, `validator`, `tokio-serde`). `server.rs` smart-refactored 834 → 619 LOC with 5 handlers extracted to `connection_handlers.rs` (231 LOC). Mock isolation verified.

### primalSpring Phase 45b: BTSP JSON-Line Wire-Format on UDS — DONE (Wave 64)

BTSP ClientHello detection on UDS: first-line JSON inspection for `"protocol":"btsp"` routes to new `continue_server_handshake_jsonline` (NDJSON steps 2–4). Post-handshake: null cipher → NDJSON loop, encrypted → frame handler. 3 new tests. Resolves primalSpring `btsp:Tower:security` FAIL.

### Deep Debt: deny.toml Cleanup, Final Workspace Dep Normalization — DONE (Wave 63)

`cargo deny` passes 4/4 clean after removing 3 stale skips and unused wrappers. 8 explicit dep pins in `crates/beardog/Cargo.toml` normalized to workspace. Comprehensive survey confirms: 0 production files >800 LOC, 0 unsafe, 0 TODO, 0 C deps compiled, all mocks gated.

### primalSpring Phase 45 Audit: Sign→Verify Roundtrip, Base64 Standardization — DONE (Wave 62)

`crypto.sign` now returns `public_key` (standard base64), enabling sign→verify roundtrip via IPC. All Ed25519 output standardized from hex to standard base64 across capability announcements, ionic bonds, and contract signing. Backward-compatible verification (accepts hex or base64). Resolves primalSpring guidestone BD-PG-01 and BD-PG-02.

### Deep Debt: Workspace Dep Normalization, Cross-Arch Fix — DONE (Wave 61)

40+ explicit dep pins across 9 crates normalized to `{ workspace = true }`. Cross-arch compilation fixed (macOS/iOS/Windows/WASM `PlatformSocket::bind` return type, platform HSM `vec![]` initialization). Deep debt survey: zero production files >800 LOC, zero unsafe, zero TODOs. 14,928+ tests, 0 failures.

### Documentation Cleanup, Clippy Fixes — DONE (Wave 60)

All 7 root docs updated with Wave 58/59 entries. Enum dispatch count corrected to 20. 56 CLI receipt artifacts cleaned. 2 clippy fixes.

### Deep Debt: Enum Dispatch, Workspace Deps, Test Refactoring — DONE (Wave 59)

`Box<dyn ProtocolHandler>` → `ProtocolHandlerBackend` enum; `Box<dyn AsyncStream>` → `IpcStream` enum (20 dispatch types total). 21 explicit dep pins normalized to workspace. 7 `#[allow()]` → `#[expect()]`. 2 test files >800 LOC smart-refactored. 14,928+ tests, 0 failures.

### primalSpring Audit: BTSP Documentation, Cleartext Bypass — DONE (Wave 58)

`BEARDOG_FAMILY_SEED` documented in README with security modes table. Cleartext JSON-RPC bypass documented and advertised in `capabilities.list` (`cleartext_methods` array). Resolves spring audit findings.

### serde_yaml Elimination + Deep Debt — DONE (Wave 56)

`serde_yaml` (deprecated, uses `unsafe-libyaml`) removed from all crates and workspace. YAML config paths return deprecation errors; TOML/JSON remain. Sole production `Box<dyn Error>` converted to typed `BearDogError`. `#[allow()]` migrated to `#[expect()]`. 3 test files >900 LOC smart-refactored into domain modules. 14,928+ tests, 0 failures.

### async-trait Lockfile Elimination — DONE (Wave 55)

`async-trait` fully eliminated from `Cargo.lock` by removing unused `hickory-resolver` from `beardog-core` and never-enabled `tarpc` optional dep from `beardog-ipc`. Banned in `deny.toml`. BearDog clears stadial gate as 13th/13 primal.

### Deep Debt Pass — DONE (Wave 54)

Smart-refactored 2 production files over 800 LOC by domain concern. Dependency cleanup: workspace pins unified, `gethostname` consolidated, `syn v1` eliminated. All mocks verified behind `#[cfg(test)]`. Zero unsafe, zero TODOs, zero production mocks. 14,928+ tests, 0 failures.

### Stadial Parity Gate — DONE (Wave 53)

Native async traits (RPITIT) across the workspace, zero `async-trait` in manifests, enum-backed dispatch for closed backend sets, quality gates unchanged (14,928+ tests; Clippy and rustdoc `-D warnings`).

### Test Coverage to 90% — DONE (Wave 20)

Coverage reached **90.51% line** (14,928+ tests passing).

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

Dead CI workflows (7 files), `docker-compose.yml`, stale profiling scripts, and deployment scripts removed. Moved to `ecoPrimals/infra/wateringHole/fossilRecord/beardog/`. Retained and fixed: `ci.yml` (pinned 1.93.0, correct workspace commands) and `Dockerfile` (correct binary name, no phantom features).

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

**Last Updated**: May 7, 2026
