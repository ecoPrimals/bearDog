<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Roadmap

**Updated**: Aug 6, 2026
**Status**: Production Ready
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Current State

BearDog is production-ready with TRUE ecoBin v2.0 compliance achieved. Edition 2024, zero clippy warnings, zero missing docs, zero unsafe code.

### Completed

- Rust edition 2024 (MSRV 1.93.0, `rust-toolchain.toml` pinned)
- 100% Pure Rust crypto achieved — `aws-lc-rs` replaced by `rustls-rustcrypto` (Pure Rust CryptoProvider); `rcgen` replaced by `p256` + `x509-cert`; 19 C-crypto crates banned in `deny.toml`
- 236 JSON-RPC methods (semantic naming; ionic bond lifecycle, consent gate, contract signing, lineage queries, auth gate, ionic tokens, cross-gate trust exchange, FIDO2/CTAP2, seed fingerprint, universal discovery, threat analysis, ecosystem integration, enrollment verification, cipher floor enforcement, ACME Phase 2 crypto delegation)
- 0 clippy warnings (pedantic + nursery + all cast lints warn + `doc_markdown` warn + `missing_errors_doc` warn + unwrap/expect warn, workspace-centralized)
- 0 missing documentation warnings (all public items documented, all `# Errors` sections present)
- 0 unsafe production (Linux); `deny(unsafe_code)` workspace-wide with targeted `#[allow]` on Windows DPAPI FFI + libtower C ABI
- 0 TODO/FIXME/HACK in codebase
- 0 files exceeding 800 lines of code (production) — `enrollment.rs` (1061L) decomposed into 7 focused modules; `lineage_proof.rs` (877L) tests extracted
- Hardcoded `eth0` → `BEARDOG_MDNS_INTERFACE` env var; `/tmp/` fallbacks → `std::env::temp_dir()`
- 14,026 tests passing (concurrent; 35 `#[serial]` in `beardog-production`)
- 90.51% line coverage (llvm-cov workspace) — target 90% met
- Dependency Injection architecture — pure `Default`, `from_env()` at boundaries
- `#[serial]` minimized — 35 tests in `beardog-production` (shared `AtomicBool`); all others concurrent
- `cargo deny` passes all 4 checks
- `trust-dns-resolver` → `hickory-resolver`, `bincode` → `postcard`, `validator` 0.20
- Multi-family socket support (`--family-id` flag)
- Encrypted secret storage (family-scoped ChaCha20-Poly1305) + `CredentialStore` trait with in-memory and file-vault backends
- `discover_capabilities` introspection method
- Tor v3 onion address derivation + ntor handshake + cell crypto
- Dark Forest beacon (zero metadata leakage discovery)
- Universal IPC (Unix sockets, abstract sockets, TCP)
- Android StrongBox integration (complete)
- HSM abstraction (software, PKCS#11, StrongBox, Windows DPAPI, Linux SecretService, iOS Secure Enclave, FIDO2/CTAP2)
- Enrollment seed rotation via genetic HKDF hierarchy (generation-based, grace-period dual verification)
- Two-layer genetic enrollment: mitochondrial gate (HMAC/family seed) + nuclear lineage distance (tree proximity → trust tier)
- Bond-type cipher awareness: `BtspBondType` (Covalent/Ionic) drives per-type cipher floors in BTSP negotiation
- UDS backpressure signaling: explicit JSON-RPC error `-32003` on connection saturation (retry-with-backoff instead of hang)
- BTSP on local UDS (defense-in-depth): `BEARDOG_UDS_REQUIRE_BTSP=1` disables the `{` bypass, all connections must BTSP handshake
- All production `unwrap()`/`expect()` eliminated (zero panic paths)
- SPDX license headers on all .rs files (100%)
- ecoBin C-dependency compliance (sysinfo removed, blake3 pure, pprof optional)
- Smart refactoring of oversized files into submodule directories (Wave 119: server.rs→7, orchestrator.rs→10)
- All mocks isolated behind `cfg(test)` / `test-utils` feature (Wave 119: StubVerificationHandler gated)
- Hardcoding eliminated — capability-based discovery throughout (Wave 119: PRIMAL_NAME self-knowledge wired)
- BTSP trust bootstrap over WAN mesh — client handshake, trust DB seeding, env-aware UPA registry (Wave 119)
- Blocking `std::fs` → `tokio::fs` in all async handlers (Waves 119–120)
- `Arc<Mutex>` → `Arc<RwLock>` for read-heavy data structures (Wave 119)
- 12 manual error Display types migrated to `thiserror::Error` derive (Waves 119–120)
- Dependency consolidation: hostname→whoami, dirs→directories, removed unused jni/aes/ctr (Waves 119–120)
- UID helper centralized from 3 crates into beardog-utils::platform (Wave 119)
- Mutual BTSP authentication — server proves family membership to client via `server_proof` HMAC (Wave 120)
- TCP BTSP parity — `btsp_family_verified` + `peer_id` binding on TCP path matching UDS (Wave 120)
- `PeerTrustRecord.family_id` persistence — trust seeding preserves family lineage (Wave 120)
- Primal identity unification — 10 hardcoded `"beardog"` sites → `resolve_primal_name()` (Wave 120)
- Graph permissions fail-closed — collaboration unavailable → error, not silent `Viewer` downgrade (Wave 120)
- 25 pre-existing test failures fixed (adapter dispatch, platform assertions, error codes) (Wave 120)
- `auth.trust_issuer` auth gate — requires BTSP/ionic/UDS; previously unauthenticated (Wave 123)
- `BEARDOG_TRUSTED_ISSUERS` env bootstrap — cross-gate issuer keys seeded at server init (Wave 123)
- `auth.exchange_trust` returns `node_id` as `local_gate_id` (was `primal_name`) (Wave 123)
- `try_verify_bearer` pre-dispatch — ionic-token auth path wired for gate-handled methods (Wave 123)
- `deny.toml` hardened — C deps banned, duplicate versions denied
- `CommandRunner` trait for mocking external commands (`adb`) in tests
- **Stadial parity gate (Wave 53)** — Complete: all `#[async_trait]` removed in favor of native `async fn` in traits; `async-trait` dependency removed from every `Cargo.toml`; finite-implementor `dyn Trait` sites replaced with enum dispatch for monomorphized async routing; Clippy and rustdoc `-D warnings` clean

### Platform Coverage

| Platform | Transport | Status |
|----------|-----------|--------|
| Linux (x86_64, ARM64) | Unix sockets | Validated |
| macOS (Intel, M-series) | Unix sockets | Validated |
| Android (ARM64) | Abstract sockets + TCP | **Validated on grapheneGate (Pixel 8a)** |
| Windows (x86_64, ARM64) | Named pipes + TCP | Ready |
| iOS (ARM64) | Unix sockets (sandbox) | Ready (Secure Enclave registered) |

---

## Recently Completed

### Wave 156j — grapheneGate Validation + Vendor-Agnostic Mobile HSM — DONE

Cross-compiled ARM64 binary (6.6M) deployed and validated on grapheneGate (Pixel 8a) with 13-check matrix: all crypto, auth, secrets, HSM discovery pass. Build infra fixed (portable NDK linker path). `IosSecureEnclaveProvider` registered in `HsmKeyProviderBackend` (Silicon Atheism). `MobileHsmCapability` + `MasterKeySealer` traits created in `beardog-traits`. Validation automation script shipped.

### Wave 156i — tarpc Convergence + Deep Debt Sweep — DONE

tarpc expanded 7→30 methods (full crypto + auth domains). `tarpc_service.rs` monolith (1165L) split into `types.rs`/`server.rs`/`mod.rs`. 10 Clippy warnings fixed, 14 bare `#[allow()]` given `reason`. All production mocks confirmed `cfg(test)` gated. All 17 `.expect()` calls confirmed annotated.

### Wave 156h — G64 Cephalization: tarpc Dual-Protocol — DONE

tarpc 0.37 added as feature-gated side-channel (`tarpc-rpc`). Initial 7 RPC methods. `.tarpc.sock` sibling socket listener with bincode binary framing. `async-trait` ban preserved (tarpc 0.37 independent).

### Wave 156e — Neural API Routing Stub: E1 Debt Fix — DONE

`capability.register` domains expanded 4→8. Crypto domain operations updated to 36 canonical dotted names. `primal.announce` TCP path fixed for Neural API routing table.

### Wave 155m — Deep Debt Sweep: Orphan Purge + Hardcoding Fix — DONE

94 orphan `.rs` files deleted (~19K lines of dead code). Hardcoded primal names fixed in riboCipher probe and `PrimalSelfKnowledge` fallbacks. Dead `ed448-goldilocks` dependency removed. `#[allow(dead_code)]` hygiene completed.

### Wave 155l — P2 Divergence Fixes — DONE

Fixed dual-socket footgun: `--family-id` CLI flag now propagates to env vars, capability symlinks use family-scoped suffix. `FAMILY_SEED` load precedence normalized (`BEARDOG_FAMILY_SEED` before `FAMILY_SEED` at all 6 call sites).

### Wave 155k — Windows Platform Gating — DONE

All `UnixStream`/`UnixListener` usage gated with `#[cfg(unix)]` + `#[cfg(not(unix))]` fallbacks. Windows DPAPI FFI evolved to zero external C dependencies (direct `extern "system"` declarations). `unsafe_code` lint evolved from `forbid` to `deny` to allow targeted FFI.

### Wave 155j — crypto.sign_ed25519 Direct Key Signing — DONE

`crypto.sign_ed25519` enhanced to accept direct 32-byte `secret_key` parameter. Unblocked Provenance Trio 7/7 for loamSpine/sweetGrass.

### Wave 155i — ACME Phase 2 Crypto Delegation — DONE

5 new ACME crypto methods for songBird: `crypto.ecdsa_p256_generate_signing_keypair`, `crypto.sign_jws_es256`, `crypto.jwk_thumbprint`, `x509.build_csr`, `x509.parse_certificate`. TLS/ACME code feature-gated behind `tls-gateway` (default off).

### Wave 155b — TLS/ACME Handoff to songBird — DONE

TLS/ACME responsibility excised from bearDog to songBird. `rustls-rustcrypto` + `beardog-acme` made optional behind `tls-gateway` feature. BearDog handles crypto primitives; songBird handles transport/TLS.

### Wave 155 — Chimera Phase 0 + crates.io Audit — DONE

`beardog-crypto` crate extracted. `libtower` shared library (C ABI) for hot-path crypto. G6 crates.io audit: 21 crates publish-ready, metadata fixed, `publish = false` on internal crates.

### Wave 154 — HSM Agnostic Evolution + Transport Abstraction — DONE

Full HSM layer audit across 10+ providers on 5 platforms. iOS Safe FFI stubs eliminated (wired to real Security.framework P-256/ECDSA). iOS Secure Enclave module fixed and properly integrated into module tree. Health monitor stub removed. IPC transport finding: Unix domain sockets are the universal standard (Linux, macOS, Android, iOS) — XPC is unnecessary for single-process daemon model.

### Wave 153c — iosGate Deployment + Secure Enclave — DONE

iPhone XS provisioned as iosGate (iOS 18.5, paired, cross-compilation verified). Five iOS build errors fixed. `SafeSecureEnclave` wired to real Security.framework. iOS IPC evolved from XPC stubs to Unix domain sockets in sandbox. IPA build pipeline automated with `zsign`. Deploy blocked on Apple Developer certificate (in progress).

### Wave 153 — Deep Debt Sweep + Production Mock Evolution — DONE

Workspace-wide clippy reduced from 296+ warnings to 0. `solo_v2/provider.rs` refactored 936→736 LOC. FIDO2 VID/PID registry consolidated. 47 hardcoded latency constants replaced. Dead code removed. Android stub tests fixed.

### Wave 152 — SoloKey FIDO2 Hardware Integration + iosGate Prep — DONE

End-to-end FIDO2/CTAP2 hardware path on eastGate with SoloKey v2. udev rules shipped. HID interface selection fixed via usage page filtering. EAGAIN retry/polling for reliable CTAP2 transport. ClientPIN protocol (ECDH, AES-256-CBC, HMAC) implemented. `hmac-secret` CTAP2 extension for hardware-backed entropy. Live discovery via real CTAP2 GetInfo. Gate registration for iosGate, eastGate, grapheneGate.

### Cross-Gate Trust Hardening — DONE (Wave 123)

`auth.trust_issuer` auth gate (BTSP/ionic/UDS required; was unauthenticated). `BEARDOG_TRUSTED_ISSUERS` env-based issuer bootstrap at `MethodGate::new()`. `auth.exchange_trust` returns `node_id` as `local_gate_id`. `try_verify_bearer` pre-dispatch wires ionic-token auth for gate-handled methods.

### WAN Periplasm Hardening + Deep Debt Sweep II — DONE (Wave 120)

Mutual BTSP authentication (server `server_proof` HMAC in `HandshakeComplete`). TCP BTSP parity (`btsp_family_verified` + `peer_id`). `PeerTrustRecord.family_id` persistence. Primal identity unification: 10 hardcoded sites → `resolve_primal_name()`, `PrimalIdentity.primal_name` field for encryption tag binding. Graph permissions fail-closed. `streaming.rs` async I/O migration. `SocketConfigError` → thiserror. Unused `aes`/`ctr` deps removed. 25 pre-existing test failures fixed.

### Deep Debt Sweep — DONE (Wave 119)

BTSP WAN mesh, structural evolution, modern idiomatic Rust. 92 files changed. Smart refactoring of `server.rs` (7 files) and `orchestrator.rs` (10 files). 11 error types → thiserror. Blocking I/O → async. `Arc<Mutex>` → `Arc<RwLock>`. Dependency consolidation.

### ACME Smart Refactor + Stub Evolution — DONE (Wave 134)

`client.rs` (860 lines) refactored into 4 lifecycle-phase modules. Production stubs evolved with `warn!` logging. Dead `hostname` deps removed.

### Deep Debt Audit + Env Migration Wave 5+ — DONE (Wave 133)

Full 2,134-file audit. ~35 env literals centralized. Windows `.expect()` removed. iOS XPC hardcoding fixed. ring confirmed absent from all targets.

### AI Type Migration + Mobile Feature Gate — DONE (Wave 132)

1,104 lines of AI types moved from `beardog-core` to `beardog-types`. `feature = "mobile"` gates Android/iOS HSM paths.

### auth.verify_ionic Scopes Fix — DONE (Wave 131)

Top-level `scopes` array always present in verify response. Unblocks primalSpring SecurityVerifier Enforced mode.

### grapheneGate Keystore Architecture — DONE (Wave 129)

`KeystoreTransport` trait with Stub/AndroidJni/AndroidKeymaster backends. Pixel 8a device detection. `BEARDOG_KEYSTORE_BACKEND` env switch.

### ring Elimination — DONE (Wave 125), Pure Rust Completion (Wave 145)

`ring` → `aws-lc-rs` → Pure Rust `rustls-rustcrypto` complete. `deny.toml` bans ring, aws-lc-rs, rcgen, and 16 C-crypto crates. Zero C-crypto in dependency graph. CSR generation via `p256` + `x509-cert`. Crypto wrapper stacks consolidated; `BearDogCrypto` established as canonical primitive layer.

### Env Migration Complete — DONE (Waves 121–128)

803+ environment variable constants centralized in `env_keys.rs`. Zero inline `BEARDOG_*` strings in production code.

### Deep Debt Cleanup: Refactors, Bug Fixes & Dead Code Removal — DONE (Wave 95)

`method_gate.rs` smart-refactored 811→425 LOC. Lineage proof signature verification bug fixed (sign/verify message mismatch). Commented-out dead code cleaned in 4 production files. All files under 800-line threshold.

### JH-1: Primal-Native Identity and Ionic Token Infrastructure — DONE (Wave 94)

Ed25519-signed ionic capability tokens (`ionic_token.rs`). 3 new methods: `identity.create`, `auth.issue_ionic`, `auth.verify_ionic`. Full cryptographic token verification in `MethodGate` replacing `bearer_token.is_some()`. Methods 114→117.

### JH-0: MethodGate Pre-Dispatch Authorization — DONE (Wave 93)

Ecosystem-standard method gate adopted. Every JSON-RPC method classified as Public or Protected. `CallerContext` threading through UDS and TCP. 3 auth introspection methods. Methods 111→114.

### Contract Signing IPC Confirmation & Documentation — DONE (Wave 92)

`crypto.sign_contract`/`crypto.verify_contract` confirmed IPC-routable since Wave 42. Documentation gap closed.

### Typed Errors: BondPersistence & SslKeylog — DONE (Wave 91)

`BondPersistenceError` and `SslKeylogError` enums. Zero `Result<_, String>` remaining in persistence or TLS keylog modules.

### Doc/Port Sweep, Debris Cleanup — DONE (Wave 90)

Docs, metrics port consistency (9190 sweep), stale count cleanup.

### `crypto.sign` Contract Fix & `did:key` Derivation — DONE (Wave 89)

`PRIMAL_CONTRACTS.md` Ed25519 section rewritten. `crypto.did_from_key` added. Methods 102→103.

### Doc Cleanup, Port Alignment — DONE (Wave 88)

Root doc refresh and port/debris cleanup.

### Dependency Evolution & Typed Config Errors — DONE (Wave 87)

`crossterm` 0.27→0.29 (eliminates `mio` duplication). 5 `validate()` methods migrated to `ConfigResult<()>`.

### TCP IPC Port Alignment & Discovery Hierarchy Documentation — DONE (Wave 86)

TCP IPC default port aligned with ecosystem. Discovery/schema hierarchy documented.

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

Removed `skunkBat` from capabilities wire data. Added structured `reason` to all `#[allow()]`. Aligned showcase `blake3` to `features=["pure"]` (showcase fossilized Wave 49). Full audit: 0 unsafe, 0 TODO, 0 other-primal refs in production, 0 ungated mocks, 0 ring/openssl.

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

### Secret Storage Evolution — DONE (Wave 150t)

`CredentialStore` trait defines `store`/`retrieve`/`list`/`delete` with `SecretMetadata`. Two backends shipped: `InMemoryCredentialStore` (volatile, dev/test) and `FileVaultCredentialStore` (persistent, ChaCha20-Poly1305 + HKDF-SHA256). `SecretsHandler` rewired to `CredentialStoreBackend` enum dispatch. Trait surface aligned with `squirrel` `SecurityProvider` delegation for cross-primal credential storage.

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

**Last Updated**: Aug 6, 2026 (Wave 156j)
