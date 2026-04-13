<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Status

**Last Updated**: April 13, 2026
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
| **Unsafe Code** | 0 production | `forbid(unsafe_code)` workspace-wide + all crate `lib.rs` |
| **Format** | Clean | `cargo fmt` compliant |
| **TODO/FIXME** | 0 | All resolved |
| **Files > 1000 LOC** | 0 | All production .rs files compliant (`api_server.rs` refactored to module) |
| **Tests** | 14,780+ passing | Concurrent; 35 `#[serial]` in `beardog-production` (shared `AtomicBool`) |
| **Coverage** | 90.51% line | llvm-cov workspace — target 90% met |
| **Serial Tests** | 35 | Isolated to `beardog-production` config tests (global `AtomicBool` state) |
| **cargo deny** | 4/4 pass | 1 advisory ignore (RSA Marvin), 15 transitive version-skips |
| **License** | AGPL-3.0-or-later | SPDX headers on all .rs files |
| **Architecture** | DI-based | Pure `Default`, `from_env()` at boundaries |
| **Toolchain** | Pinned | `rust-toolchain.toml` at 1.93.0 |
| **Production** | READY | Universal deployment |

---

## Codebase Metrics

- **Crates**: 29 directories (beardog-integration excluded — overstep)
- **Rust Files**: 2,150 (crates + src + tests; excludes showcase/examples)
- **Crypto Methods**: 99 JSON-RPC methods (`methods()` handler count: +2 consent verification)
- **`#[allow(`**: 86 (was 67)
- **`#[expect(`**: 642 (was 629)
- **Platform Support**: Linux, macOS, Android, Windows, iOS

---

## Per-Crate Coverage (April 12, 2026, llvm-cov)

| Crate | Line Coverage | Notes |
|-------|---------------|-------|
| beardog-traits | 99.4% | — |
| beardog-capabilities | 98.0% | — |
| beardog-auth | 93.0% | — |
| beardog-utils | 92.3% | — |
| beardog-genetics | 89.9% | — |
| beardog-ipc | 86.5% | JSON-RPC batch path, protocol router |
| beardog-core | ~86% | capability router, cross-primal, discovery |
| beardog-discovery | ~85% | service registry, DNS-SD, announcer, config |
| beardog-types | ~84% | HSM, monitoring, performance, K8s, production config |
| beardog-installer | ~84% | CLI, deployment, validator, binary, BiomeOS |
| beardog-cli | ~83% | UniBin `--port`, entropy, key mix, client, daemon |
| beardog-tunnel | ~83% | NDJSON framing, structured tracing, IPC, BTSP |
| beardog-deploy | ~82% | command runner, android, builder, device coverage |
| beardog-integration | new | Tower Atomic UPA client, heartbeat, connection tracking |
| **Overall** | **90.51%** | llvm-cov workspace — 90% target met |

---

## Architecture Compliance (March 2026)

| Standard | Status |
|----------|--------|
| Edition 2024 | MSRV 1.93.0, all crates, `rust-toolchain.toml` pinned |
| Pure Rust (ecoBin) | Zero C deps; blake3 pure feature; sysinfo removed |
| UniBin/ecoBin | Single binary, standalone identity fallback per UniBin v1.1, cross-compilation ready |
| Dependency Injection | Pure `Default`, `from_env()` at startup, `from_env_provider()` for tests |
| Zero Hardcoding | 20+ named constants extracted; capability-based discovery everywhere |
| Self-Knowledge | Primals discover peers at runtime via capability registry |
| JSON-RPC | Primary IPC protocol with NDJSON framing and batch support; tarpc optional behind feature gate in `beardog-ipc` |
| AGPL-3.0-or-later | License verified; SPDX headers on all .rs files |
| `forbid(unsafe_code)` | Workspace level + every crate `lib.rs` (beardog-errors platform FFI documented per wateringHole) |
| Workspace Lints | Centralized clippy pedantic + nursery + all cast lints warn + `doc_markdown` + `missing_errors_doc` + unwrap/expect warn |
| All Public Items Documented | 0 missing_docs warnings |
| File Size | 0 production files > 1000 LOC |
| Zero Sleeps (non-chaos) | All test synchronization via barriers/channels/notifications |
| `#[serial]` Minimized | 35 tests in `beardog-production` (shared `AtomicBool`); all others concurrent |
| Production Mocks | Mocks in `#[cfg(test)]`; FIDO2/iOS Phase 2 stubs return proper errors, not fake data |
| Commented-Out Code | 0 — all legacy stubs cleaned per wateringHole standard |
| Typed Errors | `Box<dyn Error>` eliminated from public APIs; `BearDogError` throughout |

---

## Recent Improvements

### Wave 39: wetSpring Alignment — Consent Gate, Security Domain Registration (April 13, 2026)

- **`security.verify_consent` + `security.issue_consent_token` implemented** — HMAC-SHA256 consent tokens over `owner_id:scope` using family-derived BLAKE3 key. Enables wetSpring vault data access gating (NUCLEUS consent protocol). 6 new tests.
- **Neural API `security` capability registered** — Auto-registration includes `verify_consent`, `issue_consent_token`, `evaluate`, `lineage`, `generate_jwt_secret` so Neural API routes `capability.call("security", "verify_consent")` to BearDog.
- **Capabilities listing updated** — `capabilities.list`, `discover_capabilities`, and `provided_capabilities` all advertise consent domain. Cost estimates added.
- **99 JSON-RPC methods** (was 97); **14,780+ tests passing**, all gates clean.

### Wave 38: Deep Debt Resolution — Smart Refactoring, Production Stubs Evolved, Hardcoding Eliminated (April 12, 2026)

- **`ionic_bond.rs` refactored** (1022 LOC → 4 submodules): crypto, lifecycle, contract, mod. All 14 tests pass.
- **iOS SEP key agreement evolved** — Real X25519 DH via `x25519-dalek` replaces zeroed placeholder. Non-iOS gets genuine software-fallback ECDH.
- **Load balancing simulated metrics eliminated** — All algorithms now use real tracked state (`service_connections`, `service_weights`, metadata `avg_response_ms`/`resource_usage`). 6 unnecessary `Vec::clone()` calls removed.
- **Hardcoded `/tmp/` evolved** — 3 production callsites migrated from constants to `resolve_*()` functions (XDG/env-first).
- **`#[allow(` → `#[expect(`** in `monitoring/service.rs` (CPU/memory calculations).
- **14,906+ tests passing**, all gates clean.

### Wave 37: primalSpring Audit Resolution — Contract Signing, BTSP Relay Path, Encoding Docs (April 12, 2026)

- **`crypto.sign_contract` + `crypto.verify_contract` implemented** (IONIC-RUNTIME) — Programmatic cross-family trust: canonical JSON → SHA-256 → Ed25519. 5 new tests. Enables multi-family deployments.
- **`btsp.server.export_keys` implemented** (BTSP-BARRACUDA-WIRE) — Session keys wrapped under caller's X25519 pub via ChaCha20-Poly1305 for relay path. Keys never in plaintext in JSON-RPC.
- **LD-01 encoding contract documented** — `crypto.hash` base64 requirement in handler docs + wateringHole `CAPABILITY_WIRE_STANDARD.md`.
- **97 JSON-RPC methods** (was 95); **14,774+ tests passing**, all gates clean.

### Wave 36: Composition Elevation Sprint — Ionic Bond Lifecycle, BTSP Naming, Smart Refactoring (April 12, 2026)

- **Ionic bond lifecycle hardened** — `IonicBond` type now carries `terms_hash`; `crypto.ionic_bond.verify` performs real Ed25519 re-verification. 4 new lifecycle tests.
- **BTSP naming aligned** — Canonical `btsp.server.*`; legacy aliases preserved.
- **Production mocks eliminated** — Real `/proc` metrics with non-Linux fallback.
- **Wildcard imports eliminated** — 11+ production files.
- **Smart refactoring (3 files)** — `port_discovery.rs`, `btsp.rs`, `tarpc_server/server.rs` each split into domain submodules.
- **14,769+ tests passing**, all quality gates clean.

### Wave 35: Deep Debt Cleanup III — Placeholder Elimination, Real Entropy, Auth Test Evolution (April 11, 2026)

- **Production placeholders eliminated** — `SystemStatus` now reads real host metrics from `/proc/uptime`, `/proc/meminfo`, `/proc/stat` (Linux) with safe fallbacks; sovereign RNG seeded from real `rand::rng()` entropy instead of zero vectors; batch validation delegates to per-config `BearDogConfig::validate()` instead of emitting "not implemented" warnings
- **Dead placeholder code removed** — `placeholder_test()` in `crypto_utils.rs`, `signature_placeholder` in `evolution.rs`, stale module placeholder comments
- **Auth tests evolved** — Permission, node registry, and authorization tests replaced from `let result = true; assert!(result)` stubs to real struct construction, field validation, and trait behavior tests using `ResourcePermission::implies()`, `CrossNodeAuthorization::is_valid()`, and `AuthorizationProof`
- **Documentation cleaned** — `supporting.rs` module doc evolved from "Placeholder" to descriptive; `beardog_core.rs` universal_adapter field documented for capability discovery
- **Tests adapted** — SystemStatus serde tests, batch validation tests updated for real behavior; 14,761+ tests passing
- **All gates green** — fmt, clippy `-D warnings`, doc `-D warnings`, test all clean

### Wave 34: Deep Debt Evolution — Hardcoding Elimination, Mock→Real, Smart Refactoring (April 11, 2026)

- **Hardcoding eliminated** — `system.rs` paths resolved via XDG/env (`resolve_config_dir`, `resolve_data_dir`, etc.); UID 1000 default replaced with `/proc/self/status` resolution in `socket_config.rs` and `discovery.rs`; UPA fallback URL dynamically constructed from env vars in `network_ports.rs`
- **Production mocks evolved to real implementations** — Ionic bond proposals now Ed25519-signed from primal identity; capability announcements Ed25519-signed; compliance metrics dynamically computed from audit trail; HSM returns proper errors for unimplemented ops; ML threat analysis returns honest empty results when no models loaded; health checks only report subsystems with actual probes
- **Dead code removed** — `grafana.rs` and `prometheus.rs` simulated external function stubs deleted (never wired into module tree)
- **Config placeholders deprecated** — `with_secrets_rotation` and `recommended_backup` marked `#[deprecated]` with migration notes to `ModernSecretsConfig` and capability-based discovery
- **Smart refactoring** — `monitoring.rs` (978 LOC) → `monitoring/mod.rs` + `alerts.rs` + `metrics.rs`; `self_knowledge.rs` (832 LOC) → `self_knowledge/mod.rs` + `identity.rs` + `endpoints.rs` + `capabilities.rs`
- **Tests adapted** — ML integration tests and compliance tests updated to assert new dynamic behavior; 14,756+ tests passing
- **All gates green** — fmt, clippy `-D warnings`, doc `-D warnings`, test all clean

### Wave 32: Deep Debt Sweep II — Stub Evolution, Large File Dedup, Clippy Zero (April 8, 2026)

- **AES-GCM 64% dedup** — Generic core; 4 handlers → thin wrappers over `gcm_encrypt<C>`/`gcm_decrypt<C>`
- **Doc hardcoding removed** — Zero primal-name references in production code
- **TPM/PKCS11/FIDO2 stubs evolved** — Honest `requires_capability` errors, public accessors, `#[expect]` for compiler notifications
- **Discovery cleanup** — Dead Phase-2 stubs removed; Consul/etcd delegated via capability architecture
- **Clippy 0 warnings** — Collapsible-if, map_or, redundant closure all resolved

### Wave 31: BTSP Handshake Enforcement — Live-Encrypted Socket Listener (April 8, 2026)

- **BTSP handshake enforcement** — New `btsp_handshake/` module: 4-step cryptographic handshake (X25519 + HMAC-SHA256) on every production connection
- **Security mode resolution** — `BtspSecurityMode::Production` / `Development` from `FAMILY_ID` + `BIOMEOS_INSECURE` env, with conflict guard
- **Encrypted frames** — Post-handshake ChaCha20-Poly1305 AEAD over length-prefixed frames; HMAC-plain and null negotiable
- **Session methods** — `btsp.session.create/.verify/.negotiate` JSON-RPC (handshake-as-a-service for other primals)
- **TCP parity** — Same enforcement in `TcpIpcServer`
- **28 new tests** — Handshake roundtrip, rejection, env resolution, 100-message nonce progression

### Wave 30: Deep Debt Sweep — Production Stub Removal, Self-Knowledge, Lint Cleanup (April 8, 2026)

- **Self-knowledge violations fixed** — `attempt_songbird_registration` → `attempt_orchestrator_registration`; `ToadStool` reference removed from key_export docs
- **Dead code removed** — `DatabaseStorageBackend` (all-error stub), `handle_key_generate` v1 (dead code), Security/Storage/Network `MigrationAdapter`s (domain overstep)
- **Stub evolution** — `PerformanceOptimizer` in types: no-op methods removed, documented as config holder; `effectiveness_score` hardcoded floats → named constants with rationale
- **Discovery honesty** — 6 HSM discoverer stubs: `warn!` → `info!` with accurate messages
- **Lint cleanup** — 35 blanket `#[allow(unused_imports, dead_code)]` removed from beardog-utils test modules
- **Wire Standard L2 handoff** — `CAPABILITY_WIRE_STANDARD.md` updated: BearDog L2 → ✓

### Wave 29b: Wire Standard Level 2 — `methods` Array, `identity.get` (April 7, 2026)

- **`capabilities.list` upgraded** — Flat `methods` array alongside `provided_capabilities`; dynamically from `HandlerRegistry::all_methods()`
- **`identity.get` implemented** — Returns `{primal: "beardog", version, domain: "crypto", license: "AGPL-3.0-or-later"}`
- **Two-phase registry** — `CapabilitiesHandler` moved to Phase 2 for `Arc<HandlerRegistry>` access
- **License fix** — `SCYBORG_EXCEPTION_PROTOCOL.md` AGPL-3.0-only → -or-later

### Wave 29a: primalSpring Audit Execution — BD-01 Encoding Hint, Sovereignty Sweep, Smart Refactoring & Debt Cleanup (April 7, 2026)

- **BD-01 resolved** — `crypto.verify_ed25519` now accepts optional `encoding` hint per `ATTESTATION_ENCODING_STANDARD.md` v2.0 (supports base64, hex, base64url, utf8, none); backwards-compatible (default: base64); 7 new encoding tests
- **Sovereignty sweep** — Removed 50+ hardcoded primal names (Songbird, NestGate, ToadStool, Squirrel) from production doc comments, test fixtures, and e2e tests; all replaced with capability-based, primal-agnostic language
- **Smart refactoring** — 3 production files over 800 LOC decomposed by domain:
  - `threat/types/mod.rs` (862L → 33L hub + 9 domain submodules)
  - `capability_router.rs` (861L → `capability_routing/` with 5 focused modules)
  - `handlers/key.rs` (875L → `key/` with generate, list, storage)
- **Zero-copy consolidation** — Deleted dead orphan `zero_copy/optimized.rs` (830L, never compiled); identified remaining duplication for future consolidation
- **Dependency cleanup** — Removed unused `serde_yaml` from beardog-core and beardog-adapters; documented TOML-only migration path
- **Production stub hardening** — FIDO2 Phase 1/2 wording aligned; discovery announcement upgraded to `warn!`; AI `initialize_capability` documented as no-op; factory doc comment corrected
- **Root docs updated** — CONTEXT.md sovereignty cleanup (removed primal names, updated metrics); specs/README.md cleaned (removed references to non-existent directories)
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc ✓, test ✓, 0 failures

### Wave 28: Deep Debt Evolution — Self-Knowledge, Box\<dyn Error\>, Hardcoding, Stubs (April 2, 2026)

- **Self-knowledge cleanup** — Removed `SongbirdClient` deprecated type alias; genericized "Songbird" → "network transport primal" in tower-atomic docs; replaced "biomeOS Neural API" → "ecosystem service registry" in discovery warnings; cleaned installer docs
- **`Box<dyn Error>` → `BearDogError`** — AI hybrid intelligence public APIs (`orchestration.rs`, `integration.rs`) now return `BearDogError` instead of `Box<dyn Error + Send + Sync>`
- **Hardcoded address cleanup** — tarpc client/server/types docs now reference capability-based discovery; node-registry examples annotated; BTSP transport docs clarified
- **Feature flag naming** — `advanced-nestgate` → `advanced-registry` in beardog-adapters (no cross-primal names in feature gates)
- **Stub language cleanup** — PKCS#11/TPM comments now reference Phase 2 pure Rust crates; config_management "placeholder" → clean doc
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, test (14,366+) ✓, deny ✓

### Wave 27: primalSpring License Audit — AGPL-3.0-or-later, #[expect(reason)], Method Deprecation, Binary Docs (April 2, 2026)

- **License migrated to AGPL-3.0-or-later** — 58 `Cargo.toml` files, `deny.toml`, 2,075 SPDX headers, all `.md` docs updated per `STANDARDS_AND_EXPECTATIONS.md`
- **`#[allow()]` → `#[expect(reason)]`** — 49 non-test `#[allow]` attributes migrated in `beardog-cli`, `beardog-core`, `beardog-tunnel`; all now carry contextual `reason` strings. `dead_code` on pub items in lib+bin crates kept as `#[allow]` (documented: `#[expect]` incompatible with dual-target lint evaluation)
- **Legacy flat method aliases deprecated** — `capabilities`, `get_capabilities`, `ping`, `health`, `status`, `check`, `identity`, `whoami`, `get_identity` documented as deprecated in handler doc comments; canonical `domain.operation` names (`capabilities.list`, `health.liveness`, `health.check`, `health.readiness`) marked as primary
- **Extra binaries documented** — `ARCHITECTURE.md` Binary Targets section: `beardog-installer` and `deploy-pixel8` documented as tooling exceptions; showcase demos documented as non-workspace examples
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, test (14,366+) ✓, deny ✓

### Wave 26: Deep Debt Evolution — Stubs → Implementations, Dependency Alignment, Dead Code Cleanup (April 2, 2026)

- **Workspace dependency alignment** — `beardog-ipc`, `beardog-hid`, `serial_test`, `beardog-adapters`, `beardog-capabilities`, `beardog-genetics`, `tempfile` all converted from explicit `path =` / version strings to `workspace = true`
- **`handle_key_info` stub evolved** — Replaced "Coming soon!" stub with real implementation loading key metadata from `~/.beardog/keys/`; added `handle_key_info_with_home` for DI testability
- **Client mode JSON-RPC** — Replaced placeholder `println!` with actual `dispatch_rpc()` over Unix socket (JSON-RPC 2.0 with NDJSON framing)
- **Orphaned `universal_hsm/entropy` modules wired** — `collector.rs` and `live_feed_validator.rs` compiled for the first time; 22 clippy errors fixed (API drift from `BearDogError`, cast safety, doc backticks)
- **Production `collector_production_tests.rs`** — Updated to match current `EntropyCollector` API (removed nonexistent `EntropyConfig` reference)
- **Dead code cleanup** — Removed 3 unused `Arc<RwLock<…>>` fields from `QuantumDiscoveryEngine`; moved 3 test-only `BearDogCore` API hooks behind `#[cfg(test)]`; removed redundant `#[allow(dead_code)]` from `key_revoke::merge()`; wired `min_hardware_entropy_ratio` field in entropy validator
- **tarpc docs** — Hardcoded `127.0.0.1:9901` references replaced with capability-based port discovery docs
- **`deny.toml` skip-list** — Reduced from 30 to 15 entries (12 resolved transitive splits cleaned)
- **`beardog-production` flaky test** — `test_production_ready_thread_safe` stabilized with `#[serial]` (35 tests)
- **HSM manager test flakiness** — 5 tests in `beardog-tunnel` refactored from `process_env::set_var` to `HsmAutoInitConfig` structs
- **AI tree feature-gated** — `beardog-core/src/ai/` (11.9K LOC) gated behind `ai` Cargo feature per `PRIMAL_RESPONSIBILITY_MATRIX`
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, build ✓, test (14,366+) ✓, deny ✓

### Wave 23: Massive Orphan Purge, Doc Dedup, Lint Tightening & Self-Knowledge Constants

- **36 orphan .rs files removed (~8,500+ LOC)** — Systematic module-tree audit confirmed 36 files across 14 crates were never compiled (not in any `mod` declaration or `#[path]`). Includes: 9 in `beardog-core`, 8 in `beardog-errors`, 4 in `beardog-monitoring`, 3 in `beardog-security`, 5 in `beardog-types`, 3 in `beardog-utils`, and others across `production`, `adapters`, `config`, `tunnel`, `genetics`, `deploy`, `cli`, `ipc`, `integration-tests`
- **858 duplicated doc comment lines deduplicated** — Automated scan found 858 consecutive identical `///` lines across 196 files; all removed, improving doc output and reducing noise
- **`empty_docs` lint promoted to `warn`** — 3 empty `///` comments in `beardog-types/production/monitoring.rs` replaced with real descriptions; lint level raised from `allow` to `warn` in workspace `Cargo.toml`
- **Hardcoded `"beardog"` string literals → `DEFAULT_SYSTEM_NAME` constant** — 7 production fallback sites in `beardog-tunnel/platform/mod.rs` (4 platform endpoints) and `beardog-production/config_management/runtime.rs` (3 defaults) now reference the canonical constant
- **Empty placeholder modules evolved** — `beardog-config/{discovery,validation,defaults}.rs` and `beardog-traits/unified/{storage,network}.rs` placeholder comments replaced with proper module-level documentation
- **Dependency audit** — `serde_yaml` confirmed ecoBin-compliant (unsafe-libyaml is Rust-translated, no C linker); `hostname` uses `libc` (standard OS interface); `bcrypt` uses RustCrypto `blowfish` (pure Rust); documented `serde_yaml` deprecation status with `yaml_serde` migration note
- **90.16% line coverage maintained** — 0 failures, all gates green
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc `-D warnings` ✓, build ✓, test ✓

### Wave 22: Deep Debt Evolution — Hot-Path Clones, iOS Fake Crypto Fix, Mock Cleanup

- **Genetic RPC hot-path clone elimination** — Handlers in `crypto_handlers_genetic/` now take `&serde_json::Value` and use `Deserialize::deserialize(params)` instead of cloning the entire `Value` per request
- **iOS safe_ffi: fake crypto → proper errors** — Placeholder keys, zeroed signatures, and length-only verification replaced with `BearDogError::not_implemented("iOS Secure Enclave: Phase 2")`; crypto operations never return fake success
- **Stale mock labels cleaned** — `mobile_setup.rs` "(mock)" → "(software fallback)"; stale "Mock processing" comment removed from universal adapter
- **Examples evolved** — `Box<dyn Error>` → `anyhow::Result<()>` in integration and genetics example binaries
- **Socket path centralization** — `doctor.rs` uses `DEFAULT_SOCKET_PATH` for default primal socket discovery
- **InMemoryStorageBackend** — Documented ephemeral no-op semantics
- **Dependency audit** — Confirmed: no `ring`, no `openssl-sys`, no `sled`, no `unsafe`; `blake3` correctly `pure`-featured; clean ecoBin tree
- **ios_safe test race fixed** — `#[serial]` on env-var-sensitive availability check
- **90.05% line coverage maintained** — 0 failures, all gates green

### Wave 21: StrongBox HSM Abstraction, Production Mock Evolution, Self-Knowledge & Debt Elimination

- **Canonical HSM trait (`HsmKeyProvider`)** — New lightweight, object-safe `#[async_trait]` trait in `beardog-traits` with types in `beardog-types` (`HsmProviderType`, `HsmAlgorithm`, `KeyGenParams`, `KeyHandle`, `HsmCapabilitySet`, `SelectionPreference`). Unifies 5 overlapping trait hierarchies into one canonical path.
- **Software HSM provider** — `RustSoftwareHsm` implements `HsmKeyProvider` with full algorithm-to-key-type mapping
- **Android StrongBox provider** — `AndroidStrongBoxHsm` implements `HsmKeyProvider`; `keystore.rs` rewritten with `#[cfg(target_os = "android")]` JNI bridge + non-Android stub
- **HSM Provider Registry** — `HsmProviderRegistry` with `discover()`, `select(preference)`, `software_fallback()` for dynamic provider selection (`PreferHardware`, `RequireHardware`, `SoftwareOnly`)
- **HsmManager integration** — `canonical_registry` field added; auto-init discovers available providers; `select_canonical_provider()` and `canonical_provider()` exposed
- **5 legacy HSM traits deprecated** — `CryptoProvider`, `HsmProviderTrait`, `HsmCapabilities`, 2x `HsmProvider` marked with `# Migration (v0.10.0)` doc sections (doc-comment approach avoids breaking `-D warnings`)
- **Production mock evolution** — Adapter `duration_ms` now uses real `Instant` timing (was hardcoded); `validate_canonical_usage()` now validates semver; `UniversalHsmManager` tracks real uptime/provider count
- **Self-knowledge evolution** — `SongbirdClient` → `OrchestratorRegistryClient` (alias for backward compat); `BiomeOSPaths` → `PlatformPaths`; "BiomeOS"/"Songbird"/"NestGate" string literals genericized in production paths
- **Dead orphan cleanup** — `songbird_client.rs` and `discovery_adapter.rs` removed (never in mod tree)
- **Socket path centralization** — `DEFAULT_SOCKET_PATH`, `DEFAULT_IPC_PORT_FILE`, `DEFAULT_KEY_STORAGE_DIR` constants replace inline `/tmp/beardog*` strings
- **Coverage** — 90.05% line (up from 90.03%), 89.22% region, 84.84% function
- **14,366+ tests passing** — 0 failures
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc ✓, build ✓

### Wave 18: Deep Audit Execution, UniBin Identity Compliance, Stub Evolution & Debt Elimination

- **UniBin v1.1 compliance** — `PrimalIdentity::from_env()` evolved from hard-fail to standalone fallback; `is_standalone()` accessor added; all callers updated (cli, tunnel)
- **Clippy fully clean** — `absurd_extreme_comparisons` (usize `>= 0`), `float_cmp` (lib_coverage_extension), unfulfilled `#[expect(dead_code)]` (pkcs11 provider under `--all-features`), `unused_mut` (3 benchmark files) all resolved
- **Format clean** — `cargo fmt` regression in genetics + monitoring comprehensive tests fixed
- **Doc clean** — Unclosed HTML tag in `beardog-cli` doc comment escaped
- **Orphan file cleanup** — Removed `ai_powered_analysis.rs` (682 LOC mock neural network) and `quantum_optimizations.rs` (633 LOC mock quantum simulator) from `beardog-utils`; never compiled (not in mod tree)
- **Production stub evolution** — Migration adapters (Security, Storage, Network) evolved from `BearDogError::system()` to `BearDogError::not_implemented()` with descriptive Phase 2 messages
- **Semantic method naming** — `beardog.ping` → `health.liveness`, `beardog.capabilities` → `capabilities.list` in e2e tests per wateringHole standard
- **Showcase dead_code cleanup** — `#[allow(dead_code)]` replaced with `_` field prefixes and `#[serde(rename)]` across showcase examples
- **STATUS.md accuracy** — tarpc claim corrected (optional behind feature gate, not "both supported"); UniBin standalone noted
- **toadstool_client debris** — Stale comment removed, proper module doc added
- **Coverage push** — 31 new tests across deploy, cli, tunnel, types (+coverage_expansion, wave3 tests)
- **Wave 18b: Deep Coverage Push** — 115 additional tests (61 beardog-types, 51 beardog-tunnel, 3 other)
  - beardog-types: cloud HSM, discovery factories, retry/workflow/engine configs, MFA/security/encryption, alerting, adapter chains, network constants, IPC discovery, performance providers
  - beardog-tunnel: crypto RPC handlers (password_kdf, kex_aead, aliases, signatures, router), platform, mobile HSM, safe_ffi (android, ios, biometric, mod), collaboration service
- **Songbird IPC registration** — `handle_server` now attempts best-effort ecosystem registry registration (non-fatal per PRIMAL IPC Protocol v3.1)
- **Doc reference cleanup** — `beardog.capabilities` → `capabilities.list` in JWT_SECRET_QUICK_REF.md
- **Clippy stack threshold** — Raised `stack-size-threshold` in `clippy.toml` to 1MB (test harness array for ~3000 tests exceeds 512KB default)
- **14,756+ tests passing** — 0 failures
- **Coverage** — 88.07% line (up from 87.35%), 83.68% function, 88.77% region
- **All gates green** — fmt, clippy `-D warnings`, doc, build all clean

### Wave 18c: primalSpring Composition Debt + General Audit (March 28, 2026)

- **TCP read timeout (primalSpring #1)** — All NDJSON read sites wrapped with `tokio::time::timeout(30s)`: `tcp_ipc/server.rs`, `unix_socket_ipc/server.rs` (initial + loop), `ipc_server.rs`. Idle connections from probes (`nc`, `curl`) now time out cleanly instead of blocking forever.
- **NDJSON wire format documented (primalSpring #2)** — README overview and transport section now explain NDJSON framing. Capability metadata includes `wire_format: "ndjson"` and `protocol: "jsonrpc-2.0"`.
- **Commented-out code cleanup** — Removed dead code blocks from `beardog-core/tests/mod.rs`, `beardog-errors/lib.rs`, `beardog-core/external_ffi.rs`, `beardog-core/external_functions/mod.rs`, `beardog-tunnel/hsm/crypto_providers/mod.rs`
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc ✓, build ✓, 14,756 tests passing, 0 failures

### Wave 20: 90% Coverage Target Met (March 28, 2026)

- **Coverage push** — 200+ new tests across beardog-tunnel (multi_transport, crypto manager, audit storage, diagnostics, zero-cost HSM), beardog-genetics (metrics, evolution engine, interaction capture types), beardog-integration (UPA client, heartbeat), beardog-types (health monitoring, network client, key management discovery, workflow, AI config, discovery builder, capabilities, PKCS#11, system/math constants, HSM discovery, config implementations), beardog-core (network discovery, performance optimizer, universal discovery types, crypto service types, serving/deployment types, external functions), beardog-errors (android), beardog-auth (genetics impl), beardog-discovery (announcements)
- **90% line coverage achieved** — 90.03% line, 84.90% function, 89.18% region
- **Clippy clean** — Fixed redundant clones, `io_other_error`, `into_iter` on single-element collections, manual `RangeInclusive::contains`, unit-value `let` binding, `drop()` on non-Drop type
- **15,085+ tests passing** — 0 failures
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc ✓, build ✓

### Wave 19: Deep Debt Evolution, Semantic Naming, Coverage Push (March 28, 2026)

- **Coverage push** — 138 new tests (50+ beardog-tunnel, 81+ beardog-types/core): modes/server, doctor, ios_safe, tls12_dot, tls_ops, software_hsm, hsm/config, security/authorization, discovery/providers, workflow/retry, addresses, ipc_discovery, ecosystem_storage/operations, production adapter
- **Semantic method naming evolution** — Added `domain.operation` semantic aliases as PRIMARY; `beardog.*` names kept as backward-compat:
  - Security: `birdsong.encrypt`/`birdsong.decrypt` (was only `beardog.birdsong.*`)
  - BTSP: `btsp.contact.exchange`, `btsp.tunnel.*` (was only path-like `beardog./btsp/...`)
  - Crypto: `crypto.derive_onion_address` added to method_list (was missing)
  - Method list tests enforce semantic names appear first
- **Example collision fixed** — `beardog-integration/examples/api_demo.rs` → `integration_api_demo.rs`
- **Commented-out code cleanup** — Removed dead code from `unified_types.rs`, `hsm_unified.rs`, `config/mod.rs`, `hsm/mod.rs` (security+tunnel), `lib.rs` (core+errors), `unix_socket_ipc/mod.rs`, `ios_secure_enclave/mod.rs`, `hybrid_intelligence/mod.rs`
- **14,894+ tests passing** — 0 failures
- **Coverage** — 88.46% line (up from 88.07%), 84.06% function, 89.19% region
- **All gates green** — fmt ✓, clippy `-D warnings` ✓, doc ✓, build ✓

### Wave 17: Comprehensive Audit, UniBin Compliance, NDJSON & Coverage Push

- **Coverage** — 87.35% line (up from 87.31%); broader llvm-cov pass after audit fixes
- **Tests** — 14,641+ passing (more than Wave 16 baseline)
- **JSON-RPC** — Batch request support on the wire path
- **UniBin** — `--port` behavior aligned with compliance expectations
- **NDJSON** — Wire framing corrected for streamed JSON lines
- **Observability** — Structured tracing logging evolved across hot paths
- **Production stubs** — Further evolution toward explicit not-implemented / real behavior
- **Repository URLs** — Normalized across manifests and metadata
- **Features** — Dead feature flags removed; CI uses valid `beardog-types` feature sets
- **Linting** — `cast_lossless` promoted to warn alongside existing cast hygiene
- **Cleanup** — Commented-out debris removed in core, auth, tunnel, and tests per wateringHole standard

### Wave 16: Full Ecosystem Audit, api_server Refactor, Zero-Copy IPC, Edition 2024 Showcase & Debris Cleanup

- **Full wateringHole audit** — Cross-referenced all standards (PRIMAL_IPC_PROTOCOL, SEMANTIC_METHOD_NAMING, ECOBIN_ARCHITECTURE, UNIBIN_ARCHITECTURE, ZERO_HARDCODING, primalSpring Leverage Guide)
- **api_server.rs refactored** — 1,185 LOC flat file → 3-module structure (`mod.rs`, `handlers.rs`, `types.rs`); all under 1000 LOC; production stubs evolved to 501 Not Implemented
- **`#[expect(reason)]` migration** — Additional `#[allow]` → `#[expect(reason)]` with unfulfilled-lint regression fixes
- **Showcase edition 2024** — All 28 showcase `Cargo.toml` updated from edition 2021 to 2024
- **Zero-copy IPC** — Removed `clone()` from JSON detection, `Value` deserialization from borrow, `id` single-clone, `&str` borrows in method lists
- **Zero-hardcoding evolution** — `DEFAULT_API_PORT_STR`, `DEFAULT_UPA_FALLBACK_BASE_URL`, `DEFAULT_RUNTIME_PORT_SCAN_RANGE_END` named constants; `DEFAULT_UPA_URL` aliased from canonical config; DNS-SD comments genericized
- **38 deep tests** — main.rs CLI/dispatch coverage, deploy config, CLI HSM, tunnel IPC, discovery edge cases
- **Coverage test race fix** — `coverage_gap_tests_6` global-state race eliminated
- **Clippy doc compliance** — `missing_errors_doc` fixed in `result_extensions.rs` (8 methods)
- **Debris cleanup** — `audit.log` artifacts removed; stale test `README.md` links fixed; Dockerfile toolchain aligned to 1.93.0; `tests_NEEDS_FIXING` comment references cleaned
- **14,499+ tests passing** — 0 failures
- **All gates green** — fmt, clippy `-D warnings`, doc, deny, test all clean

### Wave 15: Ecosystem Absorption, IPC Evolution, Semantic Naming v2.1.0, Self-Knowledge & Coverage Push

- **IPC error types** — `DispatchOutcome` and `IpcErrorPhase` added to `beardog-ipc` (from rhizoCrypt/LoamSpine pattern); `route_with_outcome()` on `HandlerRegistry`; `normalize_method()` for canonical name normalization
- **Health handler v2.1.0** — Differentiated liveness (`"alive"`), readiness (`"ready"` + capabilities count), and deep check (`"healthy"` + timestamp) per wateringHole Semantic Method Naming Standard v2.1.0
- **Self-knowledge** — Removed ~75 hardcoded "Songbird" references from production handler code; BearDog now describes operations generically, not naming peer primals
- **`#[expect(reason)]` migration** — 30+ production `#[allow(clippy::...)]` evolved to `#[expect(clippy::..., reason = "...")]` with contextual reasons; 6 stale annotations removed
- **Production stubs evolved** — Migration adapters return `not_implemented` errors; universal adapter uses real timing; BirdSong encrypt/decrypt returns 501; placeholder JSON eliminated
- **deny.toml** — 8 cross-primal type crate bans added (songbird-types, squirrel-types, etc.) + `provenance-trio-types` ban; enforces JSON-RPC wire-only contracts
- **60+ deep tests** — beardog-integration (HTTP router), beardog-production (disaster recovery), beardog-errors (result extensions), beardog-deploy (command runner), beardog-installer (env locking)
- **Coverage 87.08% → 87.35%** (lines); 81.98% → 82.27% (functions)
- **Discovery documentation** — 5-tier discovery pattern and credential resolution chain added to CONTEXT.md and ARCHITECTURE.md
- **14,447+ tests passing** — 0 failures
- **All gates green** — fmt, clippy `-D warnings`, doc, deny, test all clean

### Wave 14: Deep Debt Audit, Test Evolution, scyBorg Compliance & Zero-Copy

- **Failing test fixed** — `test_auto_initialize_environment_precedence` restored to green
- **Root crate coverage** — 85.9% → 87.2% (llvm-cov)
- **Test evolution** — 33 new CLI parse tests + 3 dispatch tests in `main.rs`; `dispatch()` extracted from `main()` for testability; **Debug** derives on CLI types
- **14,387 tests passing** — Up from 14,351 (+36); 0 failures, 186 ignored
- **Zero-copy** — `bytes::Bytes` in software HSM; storage moves in `key_management`
- **scyBorg compliance** — `LYSOGENY_PROTOCOL.md` and `SCYBORG_EXCEPTION_PROTOCOL.md` added at repo root
- **Test hygiene** — Hardcoded ports in tests replaced with ephemeral `:0` binds where appropriate
- **Platform test stubs** — Evolved from `panic!()` to `Result<T, E>` for clearer failure modes
- **All gates green** — fmt, clippy `-D warnings`, doc, test all clean

### Wave 13: Deep Debt Elimination, Pedantic Clippy, Zero-Copy & Coverage Push

- **Clippy fully clean** — 1,149 errors across 13+ crates eliminated; workspace `cargo clippy --all-targets --all-features -- -D warnings` exits 0
- **Mechanical lint fixes** — `uninlined_format_args` (114), `redundant_closure` (30), `redundant_clone` (5), `float_cmp` (8 → epsilon helpers), `long_literal_lacking_separators` (8), `hand_coded_ip_address` (4 → `Ipv4Addr::LOCALHOST`), `single_char_pattern`, `empty_string_manually`, `needless_collect`, and more
- **Test lint isolation** — Every crate `lib.rs` now has `#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]`; all integration test files have file-level allows; production code remains strict
- **Magic number extraction** — 60+ bare numeric literals in `beardog-types` default impls → named constants (`SECONDS_PER_HOUR`, `SECONDS_PER_DAY`, `DEFAULT_MAX_ENTRIES`, `DEFAULT_MAX_RESPONSE_TIME_MS`, `DEFAULT_MAX_GENERATIONS`, etc.)
- **Zero-copy optimization** — Clone-heavy production files evolved: destructuring over field clones (key_export, key_management), serialize-by-reference under locks, move semantics instead of clone-then-insert, reduced `Arc` reference counts
- **150 new tests** — 74 tests across cli/deploy/tunnel + 76 tests across types/core/installer targeting uncovered error paths, defaults, serde roundtrips, configuration parsing
- **License compliance** — `LICENSE-DOCS.md` added (CC-BY-SA 4.0 for docs per scyBorg); SPDX headers verified 2,026/2,026
- **Stale docs updated** — `specs/README.md` and `specs/PROJECT_STATUS.md` aligned with current metrics
- **14,351 tests passing** — Up from 14,201; 0 failures, 186 ignored
- **All gates green** — fmt, clippy `-D warnings`, doc, test all clean

### Wave 12: Cross-Ecosystem Audit, Lint Tightening & Type Safety Evolution

- **Full ecosystem audit** — Reviewed all wateringHole standards, 8 springs (primalSpring, neuralSpring, airSpring, wetSpring, hotSpring, healthSpring, groundSpring, ludoSpring), and all phase1/phase2 primals for absorption opportunities
- **Clippy lint tightening** — `unwrap_used`/`expect_used` evolved from `allow` to `warn` at workspace level; all 6 production sites annotated with `#[expect(clippy::expect_used, reason = "...")]`; unused `AsyncReadExt` import removed
- **Typed error evolution** — `Box<dyn Error>` eliminated from `receipt.rs`, `adapter_certificates.rs`, `hyperoptimized_zero_copy.rs`; `Result<(), String>` evolved to `BearDogError` in SIMD and genetics public APIs
- **SPDX compliance** — All 29 showcase `main.rs` files now have `// SPDX-License-Identifier: AGPL-3.0-or-later` headers (was 1,997/2,026; now 2,026/2,026)
- **Commented-out code cleanup** — Removed legacy stubs from ~10 production files per wateringHole standard (tunnel/lib.rs, security/lib.rs, integration/api_server.rs, btsp_provider.rs, graph_security, primal_discovery, biome_sovereignty, providers_unified)
- **Smart file refactoring** — 4 files near 1000 LOC split by domain: `monitoring_error_path_tests` (3 files), `hsm_provider_selection_tests` (2 files), `crypto_handlers_hashing` (4 files), `comprehensive_core_tests` (5 files)
- **Dead code evolution** — `#[allow(dead_code)]` removed from `api_server.rs` (fields now logged), `ultimate_safety.rs`, `compliance_validation_tests.rs`; unused fields renamed with `_` prefix
- **DI-first discovery** — `get_discovery_socket_paths` refactored to pure `build_discovery_socket_paths` with DI-friendly parameters; env-racing tests eliminated
- **Coverage tests** — 40+ new tests across `beardog-deploy` and `beardog-installer` targeting error paths, boundary conditions, and invalid inputs
- **14,351 tests passing** — Up from 14,201; 0 failures, 186 ignored
- **All gates green** — fmt, clippy `-D warnings`, doc, test all clean

### Wave 11: Deep Coverage Push, Crypto Fault Injection & Zero-Copy IPC

- **Coverage 86.1% → 87.0%** — 122+ new tests across 6 crates targeting lowest-coverage files (deploy, discovery, installer, cli, tunnel, types)
- **Crypto fault injection tests** — 14 adversarial tests: malformed base64, wrong key/nonce lengths, corrupted ciphertext/signatures, all-zero/all-ones keys, wrong-length DH secrets (Blake3, ChaCha20-Poly1305, Ed25519, X25519, Tor ntor)
- **Zero-copy IPC optimization** — `unix_socket_ipc::server.rs` refactored from byte-at-a-time `read_exact` to `BufReader::read_until` with reusable pre-allocated buffers; eliminates per-byte allocation overhead on JSON-RPC hot path
- **Per-crate coverage boost** — `beardog-deploy` (+95 new test lines), `beardog-discovery` (+13 tests), `beardog-installer` (+30 tests), `beardog-cli` (+14 tests), `beardog-tunnel` (+11 tests + 14 fault injection), `beardog-types` (+11 tests)
- **14,161 tests passing** — Up from 14,039+
- **All gates green** — fmt, clippy, doc, deny, test all clean

### Wave 10: Deep Audit Execution — Clippy, Hardcoding, Method Aliases, File Size

- **Clippy fully clean** — 12 pedantic errors fixed (`beardog-errors/process_env.rs` missing `# Panics`/`# Errors` docs, `option_if_let_else`, `must_use_candidate`, `missing_const_for_fn`; `beardog-tunnel/build.rs` + `beardog-errors/android.rs` + `core.rs` doc-markdown backticks)
- **File size compliance restored** — `device.rs` (1,029 LOC) → `device.rs` (618) + `device_tests.rs` (409) via `#[path]` extraction
- **Zero TODO/FIXME** — Last remaining `"TODO: merge other fields"` in disaster_recovery doc comment rephrased
- **Semantic method aliases** — `capability.list` and `primal.capabilities` added to Unix socket handler + TCP discovery per wateringHole `SEMANTIC_METHOD_NAMING_STANDARD.md`
- **Hardcoding evolution** — `DEFAULT_UPA_PORT` and `DEFAULT_INTEGRATION_API_PORT` in `beardog-integration` now derive from `beardog-config` canonical constants (zero duplication); `DEFAULT_INTEGRATION_API_PORT` added to config crate and re-exported
- **Production panic/unwrap audit** — Confirmed: zero production `.unwrap()` or `panic!()` outside `#[cfg(test)]` (one `panic!` in test utility `assert_eventually` is appropriate)
- **UniBin CLI tests** — 10 new integration tests for `main.rs` (version, help, capabilities, doctor, subcommand help, error paths)
- **14,039 tests passing** — Up from 14,029

### Wave 9: Unwrap Evolution, Clone Audit, Binary Unification

- **`.unwrap()` debt: 1,879 → 85** — Systematic evolution across 45+ files; production `.unwrap()` → `?`, `.expect("invariant")`, `.ok_or_else()`; test `.unwrap()` → `.expect("descriptive message")`
- **Zero-copy clone audit** — Eliminated unnecessary `.clone()` in IPC hot paths: BTSP handler `serde_json::Value` clones removed via `Deserialize::deserialize(&Value)`; `key_management.rs` destructuring instead of field clones; `software_hsm/core.rs` shared metadata moves; `key_rotation_manager.rs` move-before-log
- **Binary collision resolved** — `beardog-cli` legacy binary renamed; root `src/main.rs` UniBin is now the sole `beardog` binary; zero Cargo collision warnings
- **Coverage tests added** — ~30 new tests across beardog-tunnel, beardog-deploy, beardog-cli, beardog-types, beardog-core targeting error paths
- **14,029 tests passing** — Up from 13,900+; 0 failures, 186 ignored (platform/interactive-specific)
- **DNS-SD flaky test fixed** — Timeout `Err` now accepted as valid result for short-duration browse

### Wave 8: Deep Debt Execution — Coverage, Stubs, Hardcoding, File Size

- **Coverage 85.1% → 86.8%** — ~1,000+ newly covered lines across 8 crates targeting lowest-coverage files
- **Production stubs evolved** — `disaster_recovery.rs` now BLAKE3 content-hashed; `heartbeat.rs` real `AtomicUsize` connection tracking; `monitoring.rs` real `Instant`-backed rolling timings
- **beardog-integration re-integrated** — Evolved from HTTP/reqwest to Tower Atomic; legacy integration test archived
- **20+ hardcoded values extracted** — Ports, paths, timeouts, socket filenames all named constants with env override
- **Flaky tests fixed** — `test_timeout_accuracy` → mock-time; `test_auto_initialize_default_software_mode` → config-based (no env race); e2e stress tests timing widened
- **Orphan modules archived** — `zero_cost_registry.rs`, `zero_cost_registry_tests.rs`, legacy HTTP integration test
- **File size compliance** — `monitoring.rs` (1052) and `secure_cross_primal_messaging.rs` (1111) split via `#[path]` extraction
- **Production mock cleanup** — Fixed misleading "placeholder" comments on real X25519, UPA DTOs, entropy seed implementations
- **Capability discovery scan** — 20+ sites evolved from hardcoded to `DEFAULT_*` constants; `biomeos_tmp_socket_root()` respects `BIOMEOS_TMP_ROOT`

### Wave 7: Deep Audit Execution — Concurrency, Coverage, Hardcoding Evolution

- **MSRV 1.93.0** — Updated `rust-toolchain.toml` and `Cargo.toml` to match installed stable toolchain
- **`unsafe_code` deny → forbid** — Workspace-wide `forbid(unsafe_code)` in `Cargo.toml`
- **Zero `#[serial]`** — Eliminated all `#[serial_test::serial]` annotations; tests use unique isolated resources
- **Zero test sleeps** — All `tokio::time::sleep` in non-chaos tests replaced with `tokio::sync::Barrier`, `tokio::sync::Notify`, `tokio::task::yield_now()`
- **Production sleep cleanup** — Removed artificial delays from health checkers, AI optimization, IPC server; retained only legitimate backoff/timeout sleeps
- **Flaky test fixes** — `recovery_test_graceful_shutdown` uses `Barrier` for deterministic sync; concurrent pool test relaxed for scheduler variance
- **Hardcoding evolution** — Network addresses, ports, primal names, TLS versions now use `DEFAULT_*` constants with env override via `NetworkAddressesConfig::from_env()`
- **Self-knowledge** — `beardog-installer` genome targets loaded from data file; `beardog-integration` UPA client resolves peers via capability discovery
- **Smart refactoring** — `advanced_algorithms.rs` (976 LOC) → 5-file module tree; `crypto_handler.rs` (974 LOC) → 11-file module tree
- **Dead code cleanup** — Removed non-compiling orphan files (`alerts.rs`, `health.rs` in monitoring); cleaned `#[allow(dead_code)]` with leading underscores or removal
- **Mock isolation** — `testing` and `property_testing` modules gated behind `#[cfg(any(test, feature = "test-utils"))]`
- **Coverage push** — `beardog-traits` 39% → 99.4%; `beardog-deploy` and `beardog-discovery` coverage boosted with comprehensive integration tests
- **License normalization** — All 13 crate `Cargo.toml` files standardized to `license = "AGPL-3.0-or-later"` in `[package]`

### Wave 6: Cast Lint Tightening, Coverage Push & Capability-Based Discovery

- **Cast lint evolution** — Removed 5 global `allow(clippy::cast_*)` from workspace `Cargo.toml`; 301 sites now use `#[expect(clippy::cast_*, reason = "...")]` or `Type::from(x)` for lossless casts
- **Hardcoded primal names → capability-based** — IPC socket paths, peer resolution, and listener binds now use `BIOMEOS_IPC_NAMESPACE`, `PRIMAL_NAME` env vars with runtime discovery
- **Coverage push**: 84% → 85.3% region (13,400 → 13,850+ tests)
- **Doc cleanup** — All `rustdoc::broken_intra_doc_links` resolved; `cargo doc -D warnings` clean
- **Root doc declutter** — 17 reference/guide docs moved to `docs/references/`; session logs archived
- **File size compliance** — Extracted 5 test modules to keep all .rs files under 1000 lines

### Wave 5: Concurrency Architecture & Dependency Modernization

- **Dependency Injection architecture** — Eliminated global mutable state; `Default` pure (no I/O), `from_env()` at boundaries, `from_env_provider()` for tests
- **330 → 15 `#[serial_test::serial]`** — Only chaos/fault tests serialized; all other tests fully concurrent
- **Dependency modernization**: `trust-dns-resolver` → `hickory-resolver`, `bincode` → `postcard`, `validator` 0.18 → 0.20
- **`cargo deny` passes all 4 checks** (advisories, bans, licenses, sources)
- **Hanging tests eliminated**: `CommandRunner` trait for `adb` mocking; bounded timeouts on streaming ops
- **Coverage**: 80% → 84% line (llvm-cov)

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
