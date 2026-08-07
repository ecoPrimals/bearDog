<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Status

**Last Updated**: August 7, 2026 (Wave 157a — G68 Platform Substrate Convergence)
**Version**: 0.9.0
**Edition**: 2024 | **MSRV**: 1.93.0

---

## Quick Status

| Metric | Status | Details |
|--------|--------|---------|
| **Build** | Clean | Zero errors, edition 2024 |
| **Clippy** | 0 warnings | Pedantic + nursery + all cast lints warn + `doc_markdown` warn + `missing_errors_doc` warn + unwrap/expect warn, workspace-centralized |
| **Missing Docs** | 0 warnings | All public items documented |
| **Pure Rust** | 100% | Zero C dependencies (ecoBin) |
| **Unsafe Code** | 0 production (Linux) | `deny(unsafe_code)` workspace-wide; targeted `#[allow]` on Windows DPAPI FFI + libtower C ABI |
| **Format** | Clean | `cargo fmt` compliant |
| **TODO/FIXME** | 0 | All resolved |
| **Files > 800 LOC** | 0 | All production .rs files compliant; 2 monoliths refactored Wave 119 (server.rs→7 files, orchestrator.rs→10 files) |
| **Tests** | 11,565+ passing | Concurrent; 35 `#[serial]` in `beardog-production` (shared `AtomicBool`) |
| **Coverage** | 90.51% line | llvm-cov workspace — target 90% met |
| **Serial Tests** | 35 | Isolated to `beardog-production` config tests (global `AtomicBool` state) |
| **cargo deny** | all 4 pass | 1 advisory ignore (RSA Marvin); 41 RustCrypto skip entries removed (TLS feature-gated); `ring` + `aws-lc-rs` + `rcgen` + 16 C-crypto crates banned |
| **crates.io** | G6 READY | 21 library crates publish-ready; `beardog-errors` full dry-run PASS; 6 internal crates `publish = false` |
| **License** | AGPL-3.0-or-later | SPDX headers on all .rs files |
| **Architecture** | DI-based | Pure `Default`, `from_env()` at boundaries |
| **Toolchain** | Pinned | `rust-toolchain.toml` at 1.93.0 |
| **Production** | READY | Universal deployment |

---

## Codebase Metrics

- **Crates**: 27 workspace members (including `beardog-crypto` + `libtower` + `beardog-acme` [publish=false])
- **Rust Files**: 1,856 (crates + src + tests; excludes showcase/examples)
- **JSON-RPC Methods**: 236 dispatchable (224 registry + 12 pre-dispatch gate) — see `docs/PRIMAL_CONTRACTS.md` v4.2.0 for category breakdown
- **`#[allow(`**: 99 (all carry `reason`)
- **`#[expect(`**: 446
- **Platform Support**: Linux, macOS, Android (validated on grapheneGate Pixel 8a), Windows, iOS

---

## Per-Crate Coverage (Jun 13, 2026, llvm-cov; excludes listed separately)

| Crate | Line Coverage | Notes |
|-------|---------------|-------|
| beardog-traits | 99.4% | — |
| beardog-capabilities | 98.0% | — |
| beardog-auth | 93.0% | — |
| beardog-utils | 92.3% | — |
| beardog-genetics | 89.9% | — |
| beardog-ipc | 86.5% | JSON-RPC batch path, protocol router |
| beardog-core | ~86% | capability router, cross-primal, discovery |
| beardog-discovery | ~85% | service registry, mDNS, announcer, config |
| beardog-types | ~84% | HSM, monitoring, performance, K8s, production config |
| beardog-installer | ~84% | CLI, deployment, validator, binary, BiomeOS |
| beardog-cli | ~83% | UniBin `--port`, entropy, key mix, client, daemon |
| beardog-tunnel | ~83% | NDJSON framing, structured tracing, IPC, BTSP |
| beardog-deploy | excluded | Android deploy — biomeOS owns deployment |
| beardog-integration | excluded | HTTP REST — songbird owns transport |
| **Overall** | **90.51%** | llvm-cov workspace — 90% target met |

---

## Architecture Compliance (April 2026)

| Standard | Status |
|----------|--------|
| Edition 2024 | MSRV 1.93.0, all crates, `rust-toolchain.toml` pinned |
| Pure Rust (ecoBin) | Zero C deps; `ring` + `aws-lc-rs` + `rcgen` + 16 C-crypto crates banned in `deny.toml`; TLS backend is Pure Rust `rustls-rustcrypto`; CSR via `p256` + `x509-cert`; blake3 pure feature; sysinfo removed; `directories`/`dirs-sys` replaced with `etcetera` (pure Rust) Wave 151a |
| UniBin/ecoBin | Single binary, standalone identity fallback per UniBin v1.1, cross-compilation ready |
| Dependency Injection | Pure `Default`, `from_env()` at startup, `from_env_provider()` for tests |
| Zero Hardcoding | 850+ env_keys constants; capability-based discovery everywhere |
| Self-Knowledge | Primals discover peers at runtime via capability registry |
| JSON-RPC | Primary IPC protocol with NDJSON framing and batch support |
| AGPL-3.0-or-later | License verified; SPDX headers on all .rs files |
| `forbid(unsafe_code)` | Workspace level + every crate `lib.rs` (beardog-errors platform FFI documented per wateringHole) |
| Workspace Lints | Centralized clippy pedantic + nursery + all cast lints warn + `doc_markdown` + `missing_errors_doc` + unwrap/expect warn |
| All Public Items Documented | 0 missing_docs warnings |
| File Size | 0 production files > 800 LOC (threshold lowered Wave 75) |
| Zero Sleeps (non-chaos) | All test synchronization via barriers/channels/notifications |
| `#[serial]` Minimized | 35 tests in `beardog-production` (shared `AtomicBool`); all others concurrent |
| Production Mocks | Mocks in `#[cfg(test)]`; FIDO2/iOS Phase 2 stubs return proper errors, not fake data |
| Commented-Out Code | 0 — all legacy stubs cleaned per wateringHole standard |
| Typed Errors | `Box<dyn Error>` eliminated from public APIs; `BearDogError` throughout; `BondPersistenceError`/`SslKeylogError` replace last `Result<_, String>` |

---

## Recent Improvements

### Wave 157a — G68 Platform Substrate Convergence (Aug 7, 2026)

- **`PlatformAccess` abstraction**: New `beardog_utils::PlatformAccess` struct wraps `PermissionsExt` behind cross-platform API (`read_mode`, `set_owner_only`, `set_owner_only_async`, `set_executable_async`)
- **6 production L2 sites migrated**: `beardog-tunnel/doctor.rs` (2 sites), `beardog-acme/account.rs`, `beardog-acme/storage.rs`, `beardog-installer/validator.rs`, `beardog-installer/installer.rs` — all now use `PlatformAccess`
- **2 beardog-types sites G68-documented**: Circular dep prevents using `PlatformAccess`; already cross-platform gated with `#[cfg(unix)]` + `#[cfg(not(unix))]` fallbacks; annotated with G68 rationale
- **9 L2 violations → 0**: bearDog moves from "Heavy" to **G68 COMPLIANT**
- 5 new tests in `platform_access`, 0 Clippy warnings, all existing tests pass

### Wave 156m — G65 Protocol Negotiation (Aug 6, 2026)

- **G65 Phase 3 protocol negotiation**: Single-socket `PROTOCOLS:` greeting replaces C2 dual-socket pattern
- `Protocol::Negotiation` variant in both `beardog-ipc` and `beardog-tunnel` protocol enums
- `ProtocolNegotiator` in `beardog-ipc/protocol_router.rs`: parse client greeting, select best mutual protocol, format response
- `handle_protocol_negotiation()` in `connection_handlers.rs`: routes to tarpc or JSON-RPC based on negotiation result
- `serve_tarpc_on_stream()`: serves tarpc binary RPC on already-negotiated stream via `Transport::from`
- **Backward compatible**: no-negotiation connections still work (first-byte `{` → JSON-RPC, `.tarpc.sock` → tarpc)
- 37 new tests (31 protocol_router + 6 g65_tests), 0 Clippy warnings

### Wave 156l — Entropy Evolution + Temporal Pattern Cleanup (Aug 6, 2026)

- **Entropy orchestrator async path wired**: `generate_entropy_async()` routes through FIDO2 hardware entropy; `generate_human_entropy()` now async
- **SHA3→BLAKE3 unification**: `mix_with_human_input()` migrated from SHA3 to BLAKE3; `sha3` dependency removed from `beardog-security`
- **EntropyProvenance metadata**: `MixEntropyResponse` now includes `provenance` struct tracking which entropy sources contributed
- **parking_lot unification**: `std::sync::Mutex/RwLock` → `parking_lot` in `secrets_backend`, threat `incident` handlers, iOS orchestrator placeholder; 12 poison-handling boilerplate sites removed
- **Temporal pattern audit**: Confirmed no `lazy_static`, `once_cell`, `extern crate`, `try!`, `impl ToString`, clone-then-borrow in production
- **0 Clippy warnings, 11,565+ tests passing**

### Wave 156k — Deep Debt Sweep: Clippy + allow hygiene + full audit (Aug 6, 2026)

- **Clippy 0**: Fixed 2 remaining warnings (async fn simplification in `IosSecureEnclaveProvider::encrypt`/`decrypt`)
- **allow→expect hygiene**: Upgraded `#[allow]` → `#[expect]` where lint reliably fires (`unreachable_patterns`, `unsafe_code`, `wildcard_imports`); all 73 bare `#[allow()]` verified to carry `reason=`
- **Large file audit**: No production file exceeds 800L (largest is `tarpc_service/server.rs` at 507L production + 295L test)
- **Mock audit**: Zero production mocks — all mock references are in `#[cfg(test)]`, doc comments, or properly documented placeholders (quantum crypto)
- **Dependency audit**: 42 external deps, 100% pure Rust, zero C deps. Only crates.io blocker: `rustls-rustcrypto` git dep (feature-gated behind `tls-gateway`, agreed for songBird excision)
- **Dead code audit**: All `dead_code` allows justified and documented (struct fields reserved for future wiring, pub API surface not called from bin target)
- **TODO/FIXME**: 0 in production code
- **`unsafe`**: Only in `libtower` (C ABI exports) — justified and annotated with `reason`
- **14,026 tests**, 0 failures, 0 Clippy warnings

### Wave 156j — grapheneGate Validation + Vendor-Agnostic Mobile Abstraction (Aug 6, 2026)

- **grapheneGate validated (13/13 checks)**: Cross-compiled ARM64 binary (6.6M) deployed to Pixel 8a via ADB. All crypto roundtrips pass: Ed25519, ChaCha20-Poly1305, AES-256-GCM, BLAKE3, HKDF, ionic token lifecycle, secrets store/retrieve. StrongBox HSM discovered (Hardware tier). Full 200+ method capability surface advertised
- **Build infra fixed**: Hardcoded NDK linker path removed from `.cargo/config.toml`; portable `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` env var approach documented. `aarch64-linux-android` target added to `rust-toolchain.toml`
- **Validation automation**: `infra/validation/grapheneGate-validate.sh` — 13-check matrix script with colored pass/fail output, automatic server lifecycle, cleanup trap
- **`IosSecureEnclaveProvider` registered**: New `HsmKeyProviderBackend::IosSecureEnclave` variant with full `HsmKeyProvider` impl. Silicon Atheism: compiles everywhere, `is_available()` false off-iOS. All 10 dispatch arms wired (provider_id, type, available, capabilities, generate, delete, exists, encrypt, decrypt, sign, verify)
- **`MobileHsmCapability` trait**: Vendor-agnostic mobile HSM trait in `beardog-traits::mobile_hsm` — `hardware_attestation_available()`, `biometric_gate_available()`, `secure_element_type()`, `supported_algorithms()`. `SecureElementType` enum: AndroidStrongBox / IosSecureEnclave / SoftwareFallback / Unknown
- **`MasterKeySealer` trait**: Platform-agnostic master key sealing surface — `seal()` / `unseal()` for hardware-wrapped key storage (Android Keystore, iOS Keychain, software HKDF)
- **Finding**: Abstract socket IPC blocked by SELinux under `adb shell` on GrapheneOS (expected for non-app processes). `--bind-mode filesystem` incorrectly routes through abstract socket logic — documented for future fix
- **14,026 tests**, 0 failures, 0 Clippy warnings

### Wave 156i — Deep Debt Sweep + tarpc Convergence (Aug 6, 2026)

- **tarpc 7→30 methods**: Full crypto domain coverage + auth domain. bearDog moves from "tarpc-wired" to **near-converged**
- **Crypto domain complete**: hash (BLAKE3, SHA-256/384/512), MAC (HMAC-SHA256, BLAKE3 keyed), KDF (HKDF-SHA256, BLAKE3 derive, Argon2id), signing (Ed25519), key exchange (X25519), AEAD (ChaCha20-Poly1305, AES-256-GCM, AES-128-GCM encrypt+decrypt), generic `derive_key`
- **Auth domain wired**: `auth_issue_ionic`, `auth_verify_ionic`, `auth_public_key`, `auth_issue_session`, `identity_create` — full ionic token lifecycle over binary RPC
- **tarpc_service.rs split** (1165L monolith → 3 files): `types.rs` (251L), `server.rs` (802L incl tests), `mod.rs` (117L). Zero files over 800 lines of production code
- **Clippy 0 warnings**: Fixed 10 lib-level lints (redundant closure, collapsible if, items-after-statements, doc backticks, format inline vars, Iterator::last, borrowed-impl)
- **`#[allow()]` hygiene**: 14 bare `#[allow()]` attributes given `reason =` across 10 production files
- **Mock audit**: All production mocks (`Placeholder*`, `Stub*`, `Mock*`, `hsm_provider_mocks.rs`) confirmed `cfg(test)` gated — zero mock leakage in release builds
- **`.expect()` audit**: All 17 production `.expect()` calls already have `#[expect(clippy::expect_used, reason)]` — no changes needed
- **E2E tests**: 3 tarpc roundtrip tests over real UDS (health, crypto, auth) — all pass
- **Capabilities**: `tarpc_methods: 30` advertised in `capabilities.list`
- **14,019 tests**, 0 failures, 0 Clippy warnings

### Wave 156h — G64 Cephalization: tarpc Dual-Protocol (Aug 5, 2026)

- **tarpc 0.37 added**: Feature-gated (`tarpc-rpc`) — zero impact on default build. `async-trait` ban preserved (tarpc 0.37 no longer depends on it). `cargo deny check` all 4 pass.
- **`BearDogRpc` service trait**: Initial 7 RPC methods — `health_check`, `blake3_hash`, `sign_ed25519`, `verify_ed25519`, `sha256`, `hmac_sha256`, `version`. Delegates directly to `beardog-crypto` (no JSON serde overhead).
- **`.tarpc.sock` listener**: Sibling socket alongside `.sock` (ecosystem convention from `biomeos-primal-sdk`). Bincode binary framing. Spawned at server startup when `tarpc-rpc` feature + Unix platform.
- **Capabilities updated**: `capabilities.list` advertises `["json-rpc", "tarpc"]` when feature is active.
- **Stale references cleaned**: server.rs module doc, `BinaryFrame` doc, `beardog-ipc/Cargo.toml` comment all updated to reflect tarpc re-addition.
- **bearDog status**: "tarpc-absent" → "tarpc-wired" (Phase 1 ready for intra-gate binary RPC)
- **14,019 tests**, 0 failures, 0 Clippy errors

### Wave 156e — Neural API Routing Stub: E1 Debt Fix (Aug 5, 2026)

- **Legacy `capability.register` expanded**: 4→8 domains — added `auth`, `btsp`, `bonding`, `secrets`, `relay`, `consent`; stale `tls_crypto` (songBird's responsibility) and `genetic_lineage` (renamed) replaced with actual operation names matching runtime handler surface
- **Crypto domain operations updated**: 12 generic names replaced with 36 dotted canonical operations (signing, AEAD, hash, KDF, key-exchange, ionic bonds, contracts, semantic aliases)
- **`primal.announce` TCP path fix**: CLI server now passes `registration_addr` (TCP when available, UDS otherwise) to `send_primal_announce`, matching legacy `capability.register` behavior — TCP-only deployments now appear correctly in Neural API routing table
- **E1 debt item from overwatch blurb resolved**: bearDog already implemented both `capability.register` and `primal.announce` paths; the stale domain list and TCP socket mismatch caused routing table gaps
- **14,019 tests**, 0 failures, 0 Clippy errors

### Wave 155m — Deep Debt Sweep: Orphan Purge + Hardcoding Fix (Jul 30, 2026)

- **94 orphan `.rs` files deleted**: Never `mod`-included, compile-unreachable dead code across 8 crates (`beardog-types`, `beardog-security`, `beardog-tunnel`, `beardog-core`, `beardog-config`, `beardog-auth`, `beardog-discovery`)
- **Hardcoded primal names fixed**: riboCipher probe response and `PrimalSelfKnowledge` fallbacks now use `resolve_primal_name()` instead of literal strings
- **Dead dependency removed**: `ed448-goldilocks` workspace dep (handlers deleted, Phase 3 deferred)
- **`#[allow(dead_code)]` hygiene**: Placeholder variant now carries `reason`
- **`universal_hsm` orphan tree cleaned**: 17 unwired files deleted, stale doc comment removed
- **`timeouts_new` splits cleaned**: 5 orphaned domain files deleted
- **Zero regressions**: 14,019 tests, 0 Clippy warnings, clean `cargo clean` + rebuild

### Wave 155l — P2 Divergence Fixes (Jul 30, 2026)

- **Dual-socket footgun resolved**: Health socket doc updated to match always-on behavior; `--family-id` CLI flag now propagates to `FAMILY_ID` + `BEARDOG_FAMILY_ID` env vars so BTSP mode, `PrimalIdentity`, and socket naming all align
- **Capability symlink suffix fixed**: `MultiTransportServer` now uses family-scoped symlink suffix (`crypto-{family}.sock`) instead of hardcoded `.sock`, matching `SocketConfig::ipc_symlink_filename_suffix()` convention
- **`FAMILY_SEED` precedence normalized**: All 4 load sites now check `BEARDOG_FAMILY_SEED` (primal-scoped) before `FAMILY_SEED` (unprefixed fallback); previously 2 of 4 sites had the order reversed
- **P2 divergences from Wave 155k blurb addressed**: Both bearDog-owned items (dual-socket + FAMILY_SEED) closed

### Wave 155k — Windows Platform Gating (Jul 30, 2026)

- **P1 unblock for Windows depot `beardog.exe`**: All `UnixStream`/`UnixListener` usage gated behind `#[cfg(unix)]` with `#[cfg(not(unix))]` fallbacks
- **10 files fixed across 5 crates**: `beardog-ipc` (isomorphic IPC), `beardog-tunnel` (platform, modes, protocol, DPAPI HSM), `beardog-cli` (health socket, ecosystem discovery), `beardog-tower-atomic` (already gated — caller sites fixed)
- **`IpcStream::Unix` variant**: Conditional compilation with TCP-only on Windows; `IpcEndpoint::UnixSocket` still available on Unix
- **Windows DPAPI FFI evolved**: Removed `windows-sys` dependency; direct `extern "system"` declarations with manual `DataBlob` struct (zero-dep, zero-overhead)
- **`unsafe_code` lint**: Workspace level evolved from `forbid` to `deny` to allow targeted `#[allow(unsafe_code)]` on Windows DPAPI FFI and libtower C ABI
- **`modes/client.rs` evolved**: `ClientStream` enum replaces `Box<dyn Read + Write>` for correct trait-object dispatch (TCP + Unix)
- **Cross-compile verified**: `cargo check --target x86_64-pc-windows-gnu` passes with 0 errors
- **Linux tests**: 14,019 passed, 0 failures, 0 Clippy warnings — zero regressions

### Wave 155j — crypto.sign_ed25519 Direct Key Signing (Jul 29, 2026)

- **P1 unblock for Provenance Trio 7/7**: `crypto.sign_ed25519` now accepts `secret_key` param directly (base64-encoded 32-byte Ed25519 seed) in addition to `key_id` derivation
- **Dual-mode signing**: Mode 1 (direct key) for callers with their own keypair; Mode 2 (derived key) for primal-identity signing via BLAKE3 KDF
- **Full E2E roundtrip**: `crypto.ed25519_generate_keypair` → `crypto.sign_ed25519(secret_key=...)` → `crypto.verify_ed25519` validated through handler, router, and `capability.call` dispatch
- **6 new tests**: Direct key signing, generate→sign→verify roundtrip, error cases (wrong length, invalid base64), precedence verification
- **14,019 total tests**, 0 failures, 0 Clippy warnings

### Wave 155i — ACME Phase 2 Crypto Delegation (Jul 29, 2026)

- **5 new JSON-RPC methods**: `crypto.ecdsa_p256_generate_signing_keypair`, `crypto.sign_jws_es256`, `crypto.jwk_thumbprint`, `x509.build_csr`, `x509.parse_certificate`
- **songBird delegation surface**: ACME account key gen, JWS ES256 signing (raw `r||s`), PKCS#10 CSR with SAN, JWK thumbprint (RFC 7638), cert metadata extraction
- **17 new tests**: All handlers + routing tested; 14,013 total workspace tests
- **Crypto method count**: 109 → 114
- **Phase 2 complete**: bearDog exposes all crypto ops songBird needs for ACME absorption

### Wave 155d — G6 Public Flip Audit (Jul 28, 2026)

- **G6 READY**: 21 library crates validated for crates.io publishing; `beardog-errors` full dry-run PASS
- **Publish order documented**: topological 21-crate sequence from `beardog-errors` (leaf) to `beardog-tunnel` (root library)
- **Binary crates classified**: `beardog`/`beardog-cli` marked `publish = false` (genomeBin distribution); `libtower`/`benchmarks`/`beardog-integration-tests`/`beardog-acme` already `publish = false`
- **`rustls-rustcrypto` version specifier**: added `version = "0.0.2-alpha"` to git dep for cargo publish compatibility
- **`deny.toml` cleaned**: 41 unnecessary RustCrypto skip entries removed (resolved by TLS feature-gating in Wave 155b)
- **Flaky test fixed**: `test_clear_shared_configs_and_stats` shared-state race resolved (relative assertion)
- **Supply chain verified**: `cargo deny check` all 4 checks pass; zero git deps in default build; zero C crypto

### Wave 155c — Deep Debt Sweep + Clippy Zero (Jul 27, 2026)

- **Clippy 31→0**: Fixed doc lint (`# Errors`), backtick markup, `if let` style, `const fn`, `bool_to_int_with_if`, `field_reassign_with_default`, `doc_lazy_continuation`, `empty_line_after_doc_comments`, `type_complexity` (suppressed on deprecated TLS code) across 10 files
- **CLI wired**: Revocation export/import handlers connected to `KeyCommands::ExportRevocations` / `ImportRevocations`; previously implemented but unwired
- **Dead code cleaned**: Orphan `commands/mod.rs` deleted; `save_entropy_file`/`load_entropy_file` moved to `#[cfg(test)]`; `select_hsm` (duplicate, inconsistent priority) moved to `#[cfg(test)]`
- **Production unwrap evolved**: 6 `expect()` sites in FIDO2 CTAP2 client PIN and hmac-secret converted to `BearDogError::hsm()` with `?`; 9 remaining have documented `#[expect]` (provably safe mutex/HKDF/HMAC)
- **TLS/ACME feature-gated**: `beardog-acme` marked `publish = false`; `rustls-rustcrypto` removed from default dep tree
- **beardog-types/beardog-config lints fixed**: `field_reassign_with_default` resolved via struct initializer syntax

### Wave 155b — TLS/ACME Handoff to songBird (Jul 27, 2026)

- **TLS/ACME feature-gated**: `tls-server` removed from default features; `beardog-acme` + `rustls-rustcrypto` gated behind `tls-gateway` (default off) in `beardog-cli`
- **`rustls`, `tokio-rustls`, `rustls-pki-types`, `reqwest` removed from default dep tree**: bearDog default build is TLS-free
- **AAR shipped**: `AAR_WAVE155b_TLS_ACME_SONGBIRD_HANDOFF.md` with 3-phase deprecation plan (feature-gate → songBird absorb → bearDog excise)

### Wave 155 — Chimera Phase 0 + crates.io Audit (Jul 27, 2026)

- **`beardog-crypto` crate extracted**: Core crypto algorithms (Ed25519, X25519, ChaCha20-Poly1305, AES-GCM, BLAKE3, SHA-2/3, HMAC, HKDF, Argon2) extracted from `beardog-core` into standalone lean crate (deps: only `beardog-errors` + `beardog-types` + RustCrypto)
- **`libtower` cdylib shipped**: 565K `.so` (Linux x86_64), 446K (Android ARM64) with 8 C ABI exports (`tower_hash_blake3`, `tower_sign_ed25519`, `tower_verify_ed25519`, `tower_encrypt_chacha20`, `tower_hmac_sha256`, `tower_capabilities`, `tower_version`, `tower_last_error`). Validated on grapheneGate via ADB.
- **crates.io metadata audit**: All 27 crate `Cargo.toml` files fixed — invalid categories replaced, `readme` wired, `homepage` unified, `publish = false` on 8 internal/test crates, `version + path` on all workspace deps
- **`rustls-rustcrypto` git dep investigated**: crates.io `0.0.2-alpha` pins vulnerable `rustls-webpki 0.102.x` — git dep required until upstream publishes stable release (only remaining hard blocker for G6 public flip)
- **21 orphaned files deleted**: -3,329 lines of corrupted/unwired debris across monitoring, threat, tunnel crates
- **`beardog-errors` dry-run passes**: First crate validated for crates.io publishing

### Wave 154 — HSM Agnostic Evolution + Transport Abstraction (Jul 26, 2026)

- **Full HSM layer audit**: Mapped all 10+ providers across 5 platforms — identified dual provider stacks, production stubs, and abstraction gaps
- **iOS Safe FFI stubs eliminated**: `not_yet_available` stubs in `ios_safe.rs` now delegate to real `SafeSecureEnclave` with Security.framework P-256/ECDSA
- **iOS Secure Enclave module tree fixed**: Was orphaned from module tree; now properly declared, legacy prototypes excluded
- **Health monitor evolved**: Removed misleading "provider probe not yet wired" stub
- **IPC transport finding**: Unix domain sockets work on Linux, macOS, Android, and iOS — XPC is unnecessary for bearDog's single-process daemon model
- **Zero production mocks**: All remaining stubs are fail-closed by design (return errors, never simulated success)

### Wave 153c — iosGate Deployment + Secure Enclave (Jul 26, 2026)

- **iPhone XS provisioned as iosGate**: Paired over USB, UDID and hardware model registered in `infra/gates/iosGate.toml`
- **iOS cross-compilation verified**: Five build errors fixed (`aarch64-apple-ios` target compiles clean alongside host)
- **Secure Enclave wired to Security.framework**: `SafeSecureEnclave` evolved from software Ed25519 fallback to real P-256 keygen + ECDSA-SHA256 signing via `security-framework` crate; private keys never leave hardware
- **iOS IPC evolved from XPC stubs to Unix domain sockets**: `create_endpoint` and `bind` now produce working `Filesystem` endpoints within the iOS app sandbox
- **IPA build pipeline automated**: `ios/build-ipa.sh` script handles cross-compile, bundle assembly, and env-var-driven signing with `zsign`
- **Deploy blocked on Apple Developer certificate**: Ad-hoc signed IPA verified but iOS 18 requires Apple-issued cert; enrollment in progress

### Wave 153 — Deep Debt Sweep + Production Mock Evolution (Jul 26, 2026)

- **Clippy clean**: Workspace-wide clippy reduced from 296+ warnings to 0 (1 expected build-script notice). Applied auto-fixes, `let...else`, collapsible `if`, `map_or_else`, cast suppressions with reasons, moved `use` items before statements
- **`solo_v2/provider.rs` refactored**: 936 LOC down to 736 LOC — extracted `ceremony.rs` (tap timing analysis) and `client_pin.rs` (PIN protocol helpers) as clean modules
- **FIDO2 VID/PID registry consolidated**: Inline vendor/product name maps in `discovery.rs` replaced with shared `beardog_hid::types::fido2_names` module and lookup functions
- **Hardcoded constants eliminated**: CTAPHID settle delay, FIDO2 provider config defaults, capability latency tiers all use named constants; 47 inline latency values replaced
- **Dead code removed**: Unused `POLL_INTERVAL_MS`, orphaned imports, unreachable code, unfulfilled lint expectations
- **Android stub tests fixed**: 3 pre-existing test failures corrected — assertions now match non-Android platform reality (`strongbox_available: false`, `operations_per_second: 0.0`)
- **All 14,065 tests pass**: Zero failures across full workspace with `--features fido2`

### Wave 152 — SoloKey FIDO2 Hardware Integration + iosGate Prep (Jul 26, 2026)

- **FIDO2 end-to-end wired**: SoloKey v2 → HidCtap2Transport → SoloV2Provider → JSON-RPC IPC — all 5 methods (discover, register, authenticate, entropy, ceremony) validated
- **udev rules shipped**: `infra/udev/70-fido2.rules` covering Solo 2, YubiKey, Titan, Feitian, generic CTAPHID usage page
- **hidraw interface selection fixed**: HID report descriptor parsing filters by usage page (0xF1D0), preventing CTAP2 on wrong U2F interface
- **PIN auth fixed**: `SoloV2Provider` now uses full ClientPIN protocol 1 (P-256 ECDH + AES-256-CBC + HMAC-SHA-256) instead of raw PIN bytes
- **Live discovery**: `discovery.rs` calls real `ctap2_get_info()` for AAGUID, versions, extensions, algorithms — no more hardcoded defaults
- **Fido2HsmProvider wired**: HSM trait hierarchy delegates to real CTAP2 transport for entropy generation
- **Entropy orchestrator wired**: Constructs real `Fido2MultiCredentialProvider` instances from discovered FIDO2 devices; async hardware entropy path
- **hmac-secret extension implemented**: Full CTAP2 hmac-secret encoding (ECDH, salt encryption, extension parsing from authData and response map)
- **Transport dedup assessed**: Two transports retained by design — lightweight (security crate) and production-grade (tunnel crate with CANCEL, timing, ceremony)
- **iosGate prepared**: `aarch64-apple-ios` target installed, `beardog-security` cross-compiles for iOS, gate registrations created
- **All tests pass**: 1,161+ tests across workspace with `--features fido2`, zero failures

### Wave 151b — Android HSM Hardware Integration (Jul 26, 2026)

- **`Keystore2CliTransport` implemented** — real hardware transport that shells out to `/system/bin/keystore_cli_v2` for StrongBox/TEE operations (generate, sign/verify, encrypt/decrypt, list, delete)
- **Hardware HSM discovered on grapheneGate** — `beardog hsm discover` now finds "Android StrongBox HSM (Hardware tier)" via real Titan M2 probe
- **Availability probes fixed** — `check_android_keystore_strongbox()` and `probe_android_keystore()` replaced hardcoded `true`/`false` with real `keystore_cli_v2` probing; device info via `getprop` instead of stubs
- **`KeystoreTransportBackend::Keystore2Cli` variant** wired into all 8 dispatch functions (generate_key, sign, verify, encrypt, decrypt, list_aliases, delete_key, import_key)
- **Platform transport factory** (`with_platform_keystore_transport`) now prefers CLI transport on Android when `/system/bin/keystore_cli_v2` is available
- **`CredentialStoreBackend::platform_default()`** auto-selects Android Keystore on device, in-memory fallback elsewhere
- **Server startup wired** — `SecretsHandler` now uses `platform_default()` instead of `in_memory()` — confirmed `"backend":"android-keystore"` in production logs on grapheneGate
- **`MobileHsmDiscoverer::discover()`** replaced stub with real runtime probing — returns StrongBox (Hardware tier) or TEE (SecureEnclave tier)
- **Knox detection** evolved from `#[cfg]` to runtime `cfg!()` (Silicon Atheism)
- **Vault path fix** — Android credential vault resolves to writable `std::env::temp_dir()` instead of root-only `/data/beardog/`
- **Android `AndroidDeviceCapabilities::detect_capabilities()`** — runtime detection replaces hardcoded `strongbox_available: true`
- **Deployed + validated on grapheneGate (Pixel 8a)** — all JSON-RPC crypto ops confirmed: Ed25519 keygen, sign/verify, ChaCha20-Poly1305 encrypt/decrypt, secrets.store/retrieve with `android-keystore` backend

### Wave 151c — Deep Debt Sweep + Production Mock Evolution (Jul 26, 2026)

- **StrongBox detection evolved**: `check_strongbox_with_safe_api()` and `check_tee_with_safe_api()` replaced env-var mock (`STRONGBOX_MOCK_AVAILABLE`) with real `Keystore2CliTransport::probe_strongbox()` / `is_available()` hardware probes
- **Memory protection evolved**: `DefaultMemoryProtector::protect()` / `unprotect()` upgraded from no-op copy to real ChaCha20-Poly1305 encrypt-at-rest with ephemeral key (defends against cold-boot / memory-scan attacks)
- **Stub health metrics fixed**: `StubHealthMetricsTransport` no longer emits fabricated performance numbers (42.0 ops/s, 99.5% success) — zero-valued fail-closed metrics
- **Hardcoded primal name fixed**: `beardog-installer/platform.rs` replaced hardcoded `"nucleus"` with `BEARDOG_PRIMAL_NAME` env-aware resolution
- **Hardcoded URLs fixed**: `issuance.rs` pricing URL → `BEARDOG_LICENSE_PRICING_URL` env-aware
- **CLI transport temp paths**: `Keystore2CliTransport` sign/encrypt/decrypt temp files evolved from hardcoded `/data/local/tmp/` to `std::env::temp_dir()`
- **Magic number centralization**: IPC read buffers (3 files) → `beardog_types::constants::domains::buffers::UDP_PACKET_SIZE`; protocol-peek timeouts → named `PROTOCOL_PEEK_TIMEOUT` constants
- **`android_transports.rs` refactored**: 929→670 lines; `Keystore2CliTransport` extracted to standalone `keystore2_cli_transport.rs` (273 lines)
- **Unawaited futures fixed**: `SafeAndroidStrongBoxWrapper` metrics recording evolved from leaked async futures to sync `parking_lot::RwLock` (was `tokio::sync::RwLock`)
- **Items-after-statements fixed**: 11 `use`/`const` items moved before statements across 6 files
- **Deprecated constant migrated**: `beardog-installer` fully migrated from `BIOMEOS_RUNTIME_SOCKET_SUBDIR` to `default_ecosystem_ipc_namespace()`; Silicon Atheism `cfg!()` runtime check replaces `#[cfg(target_os = "android")]`
- **Clippy auto-fix applied**: `redundant_clone`, `format!` variable capture, and other auto-fixable warnings resolved workspace-wide
- **72 test suites**, 0 failures, all passing

### Wave 151a — Deep Debt Sweep + Silicon Atheism + Pure Rust Evolution (Jul 26, 2026)

- **475 files changed**, +3,119 / -4,969 lines — comprehensive deep debt resolution
- **Silicon Atheism V1+V2**: `HsmKeyProviderBackend` and `KeystoreTransportBackend` enums unified — all variants compile on all platforms with runtime `is_available()` dispatch; zero `#[cfg]` on enum variants
- **Clippy pedantic sweep**: 589 `#[must_use]` annotations + 119 `const fn` promotions + 290 `unused_async` + 83 `unnecessary_wraps` resolved
- **Pure Rust evolution**: `directories`/`dirs-sys` (C-FFI) replaced with `etcetera` (pure Rust) across 3 crates
- **Unsafe code tightened**: DPAPI module gets RAII `DpapiBlob` guard; unsafe blocks 6→4 with `// SAFETY:` docs
- **Hardcoding eliminated**: `/tmp/` paths → `std::env::temp_dir()`; pricing URL → runtime env var; magic `Duration::from_secs(30)` → named constants; IPC namespace `"biomeos"` → env-driven `"ecosystem"`
- **Silent failures fixed**: HID `discover()` returns `unsupported_platform` error on non-Linux (was empty vec); `service_registry::query_provider` logs structured warning when delegation is unwired
- **Test monoliths decomposed**: `audit_comprehensive_tests.rs` (875L) → 5 files; `method_gate_tests.rs` (872L) → 4 files + helpers
- **13,990 tests passing**, 0 failures, 131 ignored

### Wave 150x — Two-Layer Genetic Enrollment (Jul 25, 2026)

- **Two-layer genetic model**: `enrollment.verify` now mirrors biological dual-DNA verification:
  - **Mitochondrial gate** (HMAC/family seed): shared family identity — "can hear the birdsong"
  - **Nuclear lineage distance** (optional `lineage_proof`): tree proximity → trust tier
- **`genetic_distance()`**: computes lineage tree distance via common ancestor depth
- **`GeneticEnrollmentTier`**: `Identity` / `Kin` / `Sibling` / `Extended` / `Distant` with `auto_enroll()` predicate
- Wire: `enrollment_tier` + `genetic_distance` in response (backward compatible — omitted without proof)

### Wave 150x — Security Hardening: Pen Test Response (Jul 24, 2026)

- **Enrollment timestamp window** — rejects proofs with >±300s drift (configurable `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW`)
- **Enrollment replay tracking** — BLAKE3-digested proof cache with bounded capacity + self-pruning
- **UDS connection cap** — semaphore-based backpressure (default 512, configurable `BEARDOG_UDS_MAX_CONNECTIONS`)
- Responds to `s_tower_pen_enrollment_replay` and `s_tower_stress_btsp_storm` pen test findings

### Wave 150u — Deep Evolution + Tower Atomic Alignment (Jul 22, 2026)

- **`AndroidKeystoreCredentialStore`** — `CredentialStore` trait impl with TEE/StrongBox master key, `#[cfg(target_os = "android")]` gated
- **`CredentialStoreBackend::AndroidKeystore`** variant in Silicon Atheism enum dispatch
- **`enrollment.verify`** JSON-RPC endpoint (method 230) — HMAC-SHA256 proof verification for mesh enrollment (Tower Atomic parity P1)
- **Deep evolution pass**: Removed hardcoded port 7780 from gateway.rs (capability-based discovery); evolved `PerformanceAnalyzer` from no-op to real statistical analysis with anomaly detection; clone reduction in `key_management/store.rs`; updated yanked deps (`spin`, `crypto-bigint`); removed dead `ed448-goldilocks` dependency; documented DPAPI unsafe FFI safety invariants
- **Crypto throughput benchmark** — ChaCha20-Poly1305, AES-256-GCM, Ed25519, X25519, HKDF, HMAC at 64B–1MB payloads (Tower Atomic parity P2)

### Wave 150t — CredentialStore Trait + Cleanup (Jul 21, 2026)

- **`CredentialStore` trait** in `beardog-traits::unified::storage` — `store`, `retrieve`, `list`, `delete` async methods with `SecretMetadata`
- **`InMemoryCredentialStore`** + **`FileVaultCredentialStore`** backends (ChaCha20-Poly1305 + HKDF-SHA256, atomic writes)
- **`CredentialStoreBackend`** enum dispatch (Silicon Atheism pattern)
- **`SecretsHandler` rewired** to use `CredentialStoreBackend` for storage; retains family-scoped HKDF encryption
- **STATUS.md trimmed** — 1,076→185 lines; Waves 5–127 moved to CHANGELOG fossil record
- Cross-primal: trait surface designed for `squirrel` `SecurityProvider` delegation

### Wave 148 — Dead Code Elimination, Workspace Hygiene (Jun 14, 2026)

- **~19,700 lines of dead code removed** — beardog-utils gutted from 18K+ to 887 lines (only SafeOps + SafePinnedBuffer remain, zero external consumers for everything else). performance_optimizer module (1,192 lines) also removed.
- **Workspace deps trimmed** — beardog-utils reduced from 20+ deps to 5. Dead `beardog-compliance` dep removed from beardog-core and beardog-security.
- **4 crates excluded from workspace** — beardog-node-registry, beardog-client, beardog-workflows, beardog-production (zero consumers, dormant).
- **Wave 113 bearDog P1 complete** — plaintext health socket (`beardog-default.sock`) auto-spawns alongside main socket; accepts plain JSON-RPC or riboCipher-prefixed probes for cellMembrane monitoring.

### Wave 145 — Pure Rust Crypto, Crypto Dedup, Debris Cleanup (Jun 9, 2026)

- **100% Pure Rust crypto achieved** — `aws-lc-rs` replaced by `rustls-rustcrypto` (Pure Rust `CryptoProvider`); `rcgen` replaced by `p256` + `x509-cert` for CSR generation. Zero C-crypto in dependency graph.
- **`deny.toml` hardened** — 19 C-crypto crates banned (aws-lc-rs, openssl, ring, boring, native-tls, etc.); `cc` allowed only as `blake3` pure-mode wrapper.
- **Crypto wrapper dedup** — 5 parallel crypto stacks audited; `BearDogCrypto` established as canonical primitive layer. ChaCha20-Poly1305 consolidated into `BearDogCrypto`. `EncryptionService` and `lib.rs` free functions now delegate to `BearDogCrypto`. `SoftwareHsmCryptoProvider` renamed to resolve naming collision.
- **Dead code removed** — `unified.rs` (20KB orphan), `service.rs`, `config.rs` (superseded), `crypto_edge_cases_tests.rs` (never compiled); 15 additional orphaned/corrupted `.rs` files cleaned.
- **Root docs updated** — STATUS, SECURITY, ROADMAP, ACME spec corrected from `aws-lc-rs`/`rcgen` to Pure Rust stack.

### Wave 134 — ACME Smart Refactor, Stub Evolution, Dead Dep Removal (Jun 3, 2026)

- **ACME client smart refactor** — `client.rs` (860 lines) split into 4 cohesive modules by lifecycle phase (`config`, `mod`, `renewal`, `issuance`). All 36 tests preserved.
- **Production stub evolution** — Silent no-ops now emit `warn!` when registry URL configured but client unimplemented.
- **Dead dependency removal** — `hostname` removed from `beardog-tunnel` and root binary (zero usage).

### Wave 133 — Deep Debt Audit, Env Migration Wave 5+, Safety Fixes (Jun 3, 2026)

- **Full codebase audit** — All 2,134 .rs files audited for debt. Zero `unsafe`, `todo!()`, `unimplemented!()`, production `.unwrap()`.
- **Env migration Wave 5+** — ~35 more env literals centralized across 10 files. 850+ total env_keys constants.
- **Windows .expect() removed** — `platform/mod.rs` uses graceful fallback instead of panicking.
- **iOS XPC hardcoding fixed** — Derives from `ENV_PRIMAL_NAME` at runtime (self-knowledge pattern).
- **ring confirmed absent** — `cargo tree -i ring --target all` empty. deny.toml ban effective.

### Wave 131 Eco — BTSP Trust Issuer Exchange (Jul 4, 2026)

- **mesh_join orchestrator** — `mesh_join()` async function in `beardog-tunnel/src/mesh_join.rs`; wraps BTSP connection → `auth.exchange_trust` → local registry population; returns `MeshJoinResult` with bidirectional trust confirmation.
- **Registry persistence** — `save_to_file()` / `load_from_file()` with JSON v1 format; `save_path()` resolves XDG data dir; DID/key binding validated on load.
- **exchange_trust enhanced** — returns `local_family_id`; emits `KeyExchangeCompleted` event for downstream mesh listeners.
- **E2E test** — `mesh_join_e2e_over_tcp`: real TCP + BTSP handshake + trust dispatch + bidirectional registry assertion.
- **Test count**: 2,331 beardog-tunnel lib tests (↑1 from mesh_join E2E).

### Wave 132c Eco — Tower HTTP Gateway: ACME TLS Front (Jul 4, 2026)

- **HTTPS gateway on :443** — TLS-terminating reverse proxy in `beardog-cli/src/handlers/server/gateway.rs`; uses `HotReloadAcceptor` for atomic cert swap; bidirectional TCP forwarding to configurable upstream.
- **HTTP-01 solver spawned** — `Http01Solver::serve()` as background task; unblocks Let's Encrypt domain validation.
- **Full ACME supervisor** — `start_acme_gateway()` orchestrates solver + initial issuance + hot-reload bootstrap + renewal daemon in single startup.
- **`BEARDOG_GATEWAY_UPSTREAM`** — env key for upstream target (`host:port` or `unix:/path`); no default — explicit configuration required (primal isolation).
- **Env**: `BEARDOG_TLS_MODE=acme` activates full gateway; `BEARDOG_HTTPS_PORT` configures bind port (default 443).

### Wave 132 — AI Type Migration, Mobile Feature Gate (Jun 3, 2026)

- **AI type redesign** — Moved 1,104 lines from `beardog-core` to `beardog-types` (core_learning, core_neural_networks). Deprecated modules now thin re-exports.
- **Mobile feature flag** — `feature = "mobile"` gates Android/iOS HSM paths in `beardog-security`, mirroring FIDO2 pattern.

### Wave 131 — auth.verify_ionic Scopes Fix, Gap Analysis (Jun 3, 2026)

- **auth.verify_ionic fix** — Top-level `scopes` array always present in response (empty `[]` on error, actual scopes on success). Unblocks primalSpring SecurityVerifier.
- **health.liveness confirmed** — Already implemented (stale -32601 report).
- **Android type stack confirmed** — Already cfg-gated (false positive).

### Wave 129–130 — grapheneGate Keystore, Quantum Gate, Deprecated Purge (Jun 2–3, 2026)

- **grapheneGate keystore architecture** — `KeystoreTransport` trait with Stub/AndroidJni/AndroidKeymaster backends. Pixel 8a device detection.
- **quantum_crypto feature-gated** — Prevents false PQC claims on public API.
- **3 deprecated modules deleted** — -369 lines, zero external callers.

### Wave 128 — Evolution Sweep: PQC Gating, Async I/O, Identity Fix, Lock-Free (Jul 4, 2026)

- **PQC simulation gated** — `quantum_crypto` behind `#[cfg(any(test, feature = "pqc-simulation"))]`; random-byte crypto no longer ships in production.
- **Identity resolution fixed** — `resolve_primal_name()` now checks `BEARDOG_PRIMAL_NAME` → `PRIMAL_NAME` → default; `IdentityHints` and fallback corrected.
- **Async I/O migration** — `beardog-acme` cert storage and `beardog-monitoring` `/proc` reads converted from blocking `std::fs` to `tokio::fs`.
- **Lock evolution** — `std::sync::Mutex` → `parking_lot::Mutex` (monitoring), `Arc<Mutex<SystemTime>>` → `AtomicU64` (BTSP tunnel), mDNS stats `Mutex` → `RwLock`.
- **Trust test coverage** — 11 new tests for `seed_from_env`, `try_verify_bearer`, auth gate; orphan files deleted; DID helpers consolidated.
- **SslKeylogError** → `thiserror` derive.

### Waves 5–127 (Historical)

Full wave-by-wave history is in [CHANGELOG.md](CHANGELOG.md) (fossil record).


---

## Verification

```bash
cargo fmt --all -- --check                    # Format — clean
cargo clippy --workspace --all-features       # Lint — 0 warnings
cargo check --workspace --all-features        # Compile — clean
cargo test --workspace                        # Tests — 0 failures
cargo doc --workspace --no-deps               # Docs — clean
cargo deny check                              # Advisories, bans, licenses, sources
cargo llvm-cov --workspace --summary-only     # Coverage — 90.51%
```

---

**Status**: PRODUCTION READY
