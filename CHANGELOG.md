<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Changelog

All notable changes to the BearDog security platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### April 12, 2026 -- Wave 38: Deep Debt Resolution — Smart Refactoring, Production Stubs Evolved, Hardcoding Eliminated

- **`ionic_bond.rs` smart refactored** (1022 LOC → 4 submodules) — Extracted `crypto.rs` (signing helpers), `lifecycle.rs` (propose/accept/verify/revoke/list handlers), `contract.rs` (sign_contract/verify_contract), with `mod.rs` as dispatch. All 14 tests pass, all public APIs preserved.
- **iOS Secure Enclave key agreement evolved** — Placeholder zero-byte returns replaced with real X25519 Diffie-Hellman using `x25519-dalek`. Non-iOS platforms get genuine software-fallback ECDH; iOS path logs explicit "Secure Enclave unavailable" warning instead of silently returning zeroed secrets.
- **Load balancing simulated metrics eliminated** — `least_connections_balance` now reads real tracked connection counts from `LoadBalancerState`; `weighted_round_robin_balance` reads real tracked weights; `least_response_time_balance` reads `avg_response_ms` from service metadata; `resource_based_balance` reads `resource_usage` from metadata. Unnecessary `Vec::clone()` calls removed from dispatch (6 clones eliminated).
- **Hardcoded `/tmp/` paths evolved** — `SoftwareHsmConfig::default()` in `beardog-types` now uses `resolve_temp_dir()` (XDG/env-first); `SoftwareHsmConfig` in `beardog-tunnel` now uses `resolve_key_storage_dir()` (consolidated resolver); `doctor.rs` key-storage check uses `resolve_key_storage_dir()` instead of manual 4-tier fallback. Constants remain as documented Tier-5 last-resort fallbacks.
- **`#[allow(` → `#[expect(`** — Migrated `clippy::cast_precision_loss` in `monitoring/service.rs` (CPU/memory ratio calculations) from `#[allow]` to `#[expect]` with reason strings. Remaining `#[allow]` instances have legitimate reason strings (lib+bin crate incompatibility, deprecated re-exports, conditional compilation).
- **Zero TODO/FIXME** confirmed. Zero `todo!()`. Zero `unimplemented!()`. Zero wildcard imports in production code.
- **14,906+ tests passing**, all quality gates clean (fmt, clippy, doc, deny).

### April 12, 2026 -- Wave 37: primalSpring Audit Resolution — Contract Signing, BTSP Relay Path, Encoding Docs

- **`crypto.sign_contract` + `crypto.verify_contract` implemented** (IONIC-RUNTIME) — New JSON-RPC methods for programmatic cross-family trust: signs arbitrary contract terms (canonical JSON → SHA-256 → Ed25519), returns terms_hash + signature + public_key for independent verification. Enables multi-family deployments (CERN-level clouds, data federation, friend-hosted shards). 5 new tests: roundtrip, tamper rejection, deterministic hash ordering, method registration.
- **`btsp.server.export_keys` implemented** (BTSP-BARRACUDA-WIRE) — Completes the relay path: after `btsp.server.verify` succeeds, relay primals call `export_keys` to retrieve session keys wrapped under their X25519 ephemeral pub via ChaCha20-Poly1305 (keys never appear as plaintext in JSON-RPC). New types: `SessionExportKeysParams`, `SessionExportKeysResponse`.
- **LD-01 encoding contract documented** — `crypto.hash` base64 requirement documented in handler doc comments, module-level docs, and wateringHole `CAPABILITY_WIRE_STANDARD.md` §Parameter Encoding. Covers all crypto methods, encoding hints (BD-01), and the sign_contract hex convention.
- **Capability surface expanded** — `contract_signing` v1.0 capability added to `capabilities.list`; cost_estimates updated for `crypto.sign_contract` and `crypto.verify_contract`; ionic_bond capability bumped to v1.1.
- **97 JSON-RPC methods** (was 95: +`crypto.sign_contract`, +`crypto.verify_contract`, +`btsp.server.export_keys`, -0 = 98 total handler aliases, 97 logical methods).

### April 12, 2026 -- Wave 36: Composition Elevation Sprint — Ionic Bond Lifecycle, BTSP Naming, Smart Refactoring

- **Ionic bond lifecycle hardened** — `IonicBond` type now carries `terms_hash` (SHA-256 of canonical terms); `crypto.ionic_bond.verify` performs real Ed25519 re-verification of both proposer and acceptor signatures (was state-only check). 4 new lifecycle tests: `sealed_bond_stores_terms_hash`, `verify_detects_tampered_proposer_signature`, `verify_detects_tampered_acceptor_signature`, `full_lifecycle_propose_accept_list_revoke`.
- **BTSP naming alignment** — Doc comments in `beardog-types/src/btsp/rpc.rs` updated from legacy `btsp.session.*` to canonical `btsp.server.*` names (handler aliases preserved for backward compatibility). `crypto.ionic_bond.list` added to cost_estimates in capabilities.
- **Production mocks eliminated (2)** — `MonitoringService::collect_performance_metrics` reads real `/proc/stat` (CPU), `/proc/meminfo` (memory) with graceful non-Linux fallback; `PerformanceMonitor::collect_performance_metrics` returns actual history instead of `Default::default()`.
- **Wildcard imports eliminated** — 11+ production files in `beardog-tunnel/universal_hsm_discovery/`, `beardog-types/providers/`, `simplified_seed_tunnel.rs` switched from `use super::*` to explicit imports.
- **Smart refactoring (3 large files)** — `port_discovery.rs` (940 LOC) → 4 submodules (config, env, discoverer, hierarchical); `btsp.rs` (855 LOC) → 6 submodules (contact, session, negotiation, peer, tunnel, mod); `tarpc_server/server.rs` (811 LOC) → 9 submodules (keygen, signatures, key_exchange, aead, hashing, kdf, genetic, tls, introspection). All public APIs preserved via re-exports.
- **PKCS#11 discovery evolved** — `BEARDOG_PKCS11_SEARCH_PATHS` env var added for user-overridable library discovery before platform defaults.
- **14,769+ tests passing**, all quality gates clean (fmt, clippy, doc, deny).

### April 11, 2026 -- Wave 35: Deep Debt Cleanup III — Placeholder Elimination, Real Entropy, Auth Test Evolution

- **Production placeholders eliminated** — `SystemStatus` reads real `/proc` metrics; sovereign RNG uses real entropy; batch validation delegates per-config
- **Dead code removed** — `placeholder_test()`, `signature_placeholder`, stale module comments
- **Auth tests evolved** — Permission/authorization/node-registry stubs replaced with real struct validation tests
- **14,761+ tests passing**, all quality gates clean

### April 11, 2026 -- Wave 34: Deep Debt Evolution — Hardcoding Elimination, Mock→Real, Smart Refactoring

- **Hardcoding eliminated (4 sites)** — `system.rs` filesystem paths resolved via XDG Base Directory Specification (`resolve_config_dir`, `resolve_data_dir`, `resolve_cache_dir`, `resolve_runtime_dir`); UID 1000 default replaced with safe `/proc/self/status` resolution (no `unsafe`, no FFI) in `socket_config.rs` and `tower-atomic/discovery.rs`; UPA fallback URL dynamically constructed from `BEARDOG_UPA_URL`/`BEARDOG_EXTERNAL_HOST`/`BEARDOG_API_PORT` env vars in `network_ports.rs`.
- **Production mocks evolved to real implementations (8 items)** — Ionic bond proposals now Ed25519-signed (deterministic key from primal name + node ID via HKDF-SHA256); capability announcements Ed25519-signed with verifiable public key; compliance metrics dynamically computed from audit trail pass/fail ratio; HSM provider returns `success: false` with descriptive error for unimplemented operations; ML threat analysis returns honest empty results when no models loaded, dynamic events when models present; health checks only report subsystems with actual probes (no more blanket "unknown" for external services).
- **Dead code removed** — `grafana.rs` and `prometheus.rs` simulated external function stubs deleted (never wired into `external_functions/mod.rs`).
- **Config placeholders deprecated** — `with_secrets_rotation` and `recommended_backup` on `ProductionEnvironmentConfig` marked `#[deprecated]` with migration notes to `ModernSecretsConfig` and capability-based discovery.
- **Smart refactoring (2 large files)** — `monitoring.rs` (978 LOC) → `monitoring/mod.rs` + `alerts.rs` + `metrics.rs`; `self_knowledge.rs` (832 LOC) → `self_knowledge/mod.rs` + `identity.rs` + `endpoints.rs` + `capabilities.rs`. All re-exports preserved for API compatibility.
- **Test adaptation** — ML integration tests updated for dynamic event generation; compliance tests updated for audit-trail-based scoring; 14,756+ tests passing, 0 failures.
- **Metrics** — 95 crypto methods; 2,128 Rust files; 90.51% line coverage (llvm-cov).

### April 9, 2026 -- Wave 33: Deep Debt Cleanup & Evolution — Coverage 90.51%, allow→expect Migration, Standalone Startup, Capability Registration

- **`#[allow()]` → `#[expect(reason)]` migration** — 193 → 75 `#[allow(` remaining (62% reduction); 361 → 476 `#[expect(` with contextual `reason` strings. Where `expect` caused `unfulfilled_lint_expectations`, kept `allow` with documented `reason`.
- **Standalone startup (T10)** — `NODE_ID` / `BEARDOG_NODE_ID` no longer required; missing values use ephemeral `standalone-{uuid}` via `OnceLock`, logged once with `tracing::warn!`; PRIMAL IPC Protocol v3.1 degraded mode.
- **Dynamic `ipc.register` with Songbird (T4)** — Non-blocking registration at server startup: registry socket discovery, `ipc.register` with self-knowledge capability tags, heartbeats; exponential backoff on failure (standalone continues).
- **BD-01 per-field encoding hints** — `crypto.verify_ed25519` accepts `message_encoding`, `signature_encoding`, `public_key_encoding`; semantic aliases `crypto.ed25519.sign` / `crypto.ed25519.verify` registered.
- **Commented-out code cleaned (T1)** — 313-line orphan block in beardog-types + smaller blocks across 15+ files removed.
- **PII cleaned (T8)** — `/home/user/...` → `$HOME/...` in tests and builder.
- **Coverage** — 90.16% → 90.51% line; new tests across beardog-production, beardog-cli, beardog-compliance, beardog-config, beardog-core.
- **Smart refactoring** — `runtime.rs` (1244→360) split into `secrets.rs`, `defaults.rs`, `runtime_tests.rs`; `socket_config.rs` (1111→668) test extraction.
- **Metrics** — 95 crypto methods (`methods()` handler; prior 96 was a count error); 1,939 Rust files; 14,593+ tests passing, 0 failures.

### April 8, 2026 -- Wave 32: Deep Debt Sweep II — Stub Evolution, Large File Dedup, Clippy Zero

- **AES-GCM deduplication** — `crypto_handlers_aes_gcm.rs` production code reduced 64% (485 → 175 lines) via generic `gcm_encrypt<C>`/`gcm_decrypt<C>` core over `Aes128Gcm`/`Aes256Gcm`; extracted shared param helpers (`decode_b64_param`, `extract_key`, `extract_or_generate_nonce`)
- **BTSP handler DRY** — `resolve_tunnel_handle()` extracted from duplicated tunnel-id resolution in `handle_tunnel_status` and `handle_tunnel_close`
- **Doc hardcoding removed** — Primal name references ("Squirrel", "Songbird", "ToadStool") replaced with capability-agnostic phrasing in `universal_adapter.rs`, `capabilities.rs`, `genetics_impl.rs`
- **TPM/PKCS11 stubs evolved** — `#[allow(dead_code)]` removed from provider structs; new public accessors (`metadata()`, `library_path()`, `slot_id()`) make fields live
- **FIDO2 operations evolved** — `BearDogError::system(...)` → `BearDogError::requires_capability(...)` with honest feature-gate messaging; `#[allow(dead_code)]` → `#[expect(dead_code)]` for compiler notification when callers land
- **Service discovery cleanup** — Removed 3 deprecated dead detection stubs (`_detect_kubernetes`, `_detect_consul`, `_detect_etcd`) from factory; Consul/etcd properly delegated to capability-based discovery per architectural decision
- **Monitoring glob consolidation** — 10 per-line `#[allow(ambiguous_glob_reexports)]` consolidated into single inner module with documented reason
- **BTSP handshake docs** — Added missing field docs on `ClientHello`, `ServerHello`, `HandshakeError`, `SessionKeys`, `FamilySeed`, `BtspSecurityMode`; fixed clippy doc backtick warnings
- **Clippy zero** — Resolved all remaining warnings: collapsible-if (`let` chains), `map_or` → `is_some_and`, `format!`-in-iterator → `write!`, `match` → `if let`, redundant closure
- **All gates green** — fmt, clippy (0 warnings), 14,593+ tests passing, doc

### April 8, 2026 -- Wave 31: BTSP Handshake Enforcement — Live-Encrypted Socket Listener

- **BTSP handshake enforcement** — New `btsp_handshake/` module (6 files, ~750 LOC) implements Phase 2 of `BTSP_PROTOCOL_STANDARD.md`: when `FAMILY_ID` is set, every incoming connection must complete a 4-step cryptographic handshake (X25519 ephemeral + HMAC-SHA256 challenge-response) before any JSON-RPC is processed
- **`BtspSecurityMode` enum** — Resolved at startup from `FAMILY_ID` / `BIOMEOS_INSECURE` env vars; flows through `MultiTransportServer` to Unix and TCP server instances
- **`BIOMEOS_INSECURE` guard** — Server refuses to start when both `FAMILY_ID` and `BIOMEOS_INSECURE=1` are set (per BTSP standard)
- **Encrypted frame codec** — Post-handshake communication switches to length-prefixed (4-byte BE) frames with ChaCha20-Poly1305 AEAD encryption; supports HMAC-plain and null cipher negotiation
- **Session key derivation** — HKDF-SHA256 from X25519 shared secret with directional keys (server→client, client→server) and monotonic nonce counters
- **`btsp.session.create`/`.verify`/`.negotiate`** — 3 new JSON-RPC methods expose handshake-as-a-service for other primals to implement their own BTSP listeners using BearDog's crypto
- **TCP transport parity** — Same handshake enforcement and encrypted framing applied to `TcpIpcServer`
- **28 new tests** — Unit tests for crypto, framing, session encryption; integration tests for full handshake roundtrip, wrong-seed rejection, wrong-version rejection, env-based security mode resolution, BIOMEOS_INSECURE conflict, 100-message nonce progression
- **All gates green** — fmt, clippy, test, doc

### April 8, 2026 -- Wave 30: Deep Debt Sweep — Production Stub Removal, Self-Knowledge, Lint Cleanup

- **Self-knowledge violations fixed** — `attempt_songbird_registration` → `attempt_orchestrator_registration` (capability-based, never names another primal); `ToadStool` reference removed from key_export doc comments
- **Dead code removed** — `DatabaseStorageBackend` (all methods returned `unsupported_operation`), `handle_key_generate` v1 (superseded by v2, was `#[allow(dead_code)]`), `generate_aes_key_with_seed` gated `#[cfg(test)]`
- **Overstep cleanup** — Security/Storage/Network `MigrationAdapter`s removed from `ecosystem_integration.rs` (those domains belong to other primals per PRIMAL_RESPONSIBILITY_MATRIX)
- **Stub evolution** — `PerformanceOptimizer` in beardog-types: removed no-op `initialize_optimizations()` / `evaluate_scaling_needs()`, documented as config holder (real optimization in beardog-core); `effectiveness_score` hardcoded floats replaced with named constants + rationale
- **Discovery engine honesty** — 6 HSM discoverer stubs changed from misleading `warn!("not yet implemented")` to honest `info!` messages (Cloud: no credentials, USB: no devices, TPM: no hardware, etc.)
- **Lint cleanup** — Blanket `#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]` removed from 35 test modules in beardog-utils (clippy confirms they suppressed nothing)
- **Wire Standard handoff** — `CAPABILITY_WIRE_STANDARD.md` updated: BearDog L2 → ✓
- **All gates green** — fmt, clippy, test, doc

### April 7, 2026 -- Wave 29b: Wire Standard Level 2 — `methods` Array, `identity.get`

- **`capabilities.list` upgraded** — Flat `methods` array added alongside `provided_capabilities` (Wire Standard L2 Tier 1 migration); dynamically populated from `HandlerRegistry::all_methods()` filtered to `domain.operation` naming
- **`identity.get` implemented** — New JSON-RPC method returns `{primal: "beardog", version, domain: "crypto", license: "AGPL-3.0-or-later"}`
- **Two-phase registry construction** — `CapabilitiesHandler` moved to Phase 2 of `HandlerRegistry::new()` to receive `Arc<HandlerRegistry>` for dynamic method enumeration
- **`SCYBORG_EXCEPTION_PROTOCOL.md` license fix** — `AGPL-3.0-only` → `AGPL-3.0-or-later` for software (consistency fix from primalSpring audit)
- **`cargo fmt` regression fixed** — `edge_cases_nov_6_2025.rs` formatting corrected
- **All gates green** — fmt, clippy, test, doc

### April 7, 2026 -- Wave 29a: primalSpring Audit Execution — BD-01 Encoding, Sovereignty Sweep, Smart Refactoring

- **BD-01 resolved** — `crypto.verify_ed25519` now accepts optional `encoding` hint per `ATTESTATION_ENCODING_STANDARD.md` v2.0 (supports base64, hex, base64url, utf8, none); backwards-compatible default; 7 new encoding tests
- **Sovereignty sweep** — Removed 50+ hardcoded primal names (Songbird, NestGate, ToadStool, Squirrel) from production doc comments, test fixtures, and e2e tests; replaced with capability-based, primal-agnostic language
- **Smart refactoring** — 3 production files >800 LOC decomposed: `threat/types/mod.rs` (862L → 33L hub + 9 submodules), `capability_router.rs` (861L → 5-module directory), `handlers/key.rs` (875L → 3-module directory)
- **Zero-copy consolidation** — Deleted dead orphan `zero_copy/optimized.rs` (830L, never compiled)
- **Dependency cleanup** — Removed unused `serde_yaml` from `beardog-core` and `beardog-adapters`
- **Production stub hardening** — FIDO2 phase wording aligned; discovery announcement upgraded to `warn!`; AI `initialize_capability` documented as no-op; factory doc corrected
- **Root docs updated** — CONTEXT.md, START_HERE.md, specs/README.md sovereignty cleanup and metrics refresh; stale `otherTeams/` and `experiments/` phantom directory references removed
- **Stale references removed** — `specs/IMPLEMENTATION_GAPS_NOV_2025.md` pointer updated to `ROADMAP.md`
- **14,372+ tests passing** — 0 failures, all gates green

### April 2, 2026 -- Wave 28: Deep Debt Evolution — Self-Knowledge, Error Typing, Hardcoding

- **Self-knowledge** — Removed deprecated `SongbirdClient` type alias; genericized Songbird/biomeOS references in tower-atomic, discovery, installer docs
- **`Box<dyn Error>` → `BearDogError`** — AI hybrid intelligence public APIs return canonical typed errors
- **Hardcoded addresses** — tarpc/node-registry/BTSP docs annotated with capability-based discovery notes
- **Feature flag** — `advanced-nestgate` → `advanced-registry` (no cross-primal names)
- **Stub documentation** — PKCS#11/TPM Phase 2 comments reference target pure Rust crates
- **14,366+ tests passing**, all gates green

### April 2, 2026 -- Wave 27: primalSpring License Audit — AGPL-3.0-or-later, Lint Migration, Deprecation

- **License** — `AGPL-3.0-only` → `AGPL-3.0-or-later` across 58 `Cargo.toml`, `deny.toml`, 2,075 SPDX headers, all `.md` docs per `STANDARDS_AND_EXPECTATIONS.md`
- **`#[allow]` → `#[expect(reason)]`** — 49 non-test attributes migrated; `dead_code` on pub items in lib+bin crates kept as `#[allow]` (documented limitation)
- **Flat method aliases deprecated** — `capabilities`, `ping`, `health`, `status`, `identity`, etc. marked deprecated in favor of `domain.operation` names
- **Binary targets documented** — `ARCHITECTURE.md` now has Binary Targets section documenting UniBin, tooling exceptions, and showcase demos
- **14,366+ tests passing**, all gates green

### April 2, 2026 -- Wave 26: Deep Debt Evolution — Stubs → Implementations, Dependency Alignment, Dead Code Cleanup

- **Workspace dep alignment** — `beardog-ipc`, `beardog-hid`, `serial_test`, `beardog-adapters`, `beardog-capabilities`, `beardog-genetics`, `tempfile` → `workspace = true`
- **`handle_key_info` evolved** — Stub replaced with real key store load + `_with_home` DI variant
- **Client JSON-RPC dispatch** — Real Unix socket `dispatch_rpc()` replaces placeholder
- **Orphaned entropy modules compiled** — `collector.rs` / `live_feed_validator.rs` wired into `universal_hsm` module tree; 22 API-drift clippy errors fixed
- **Dead code cleanup** — 3 unused `QuantumDiscoveryEngine` fields removed; 3 test-only `BearDogCore` methods gated `#[cfg(test)]`; redundant `#[allow(dead_code)]` removed from `merge()`
- **`deny.toml` skip-list** — 30 → 15 entries (12 resolved transitive splits)
- **Flaky test stabilization** — `beardog-production` `#[serial]` (35 tests); `beardog-tunnel` HSM tests refactored to `HsmAutoInitConfig`
- **AI tree feature-gated** — `beardog-core/src/ai/` (11.9K LOC) behind `ai` feature per responsibility matrix
- **Root docs updated** — STATUS.md and CHANGELOG.md reflect accurate state
- **Debris cleaned** — `audit.log` removed; duplicate `env.example` consolidated; empty dirs removed
- **14,366+ tests passing** — 0 failures, all gates green

### March 30, 2026 -- Wave 25: Full Audit — Lint Promotion, Cast Safety, Doc Completeness, Smart Refactoring

- **Cast safety promoted to warn** — All 40 truncation/precision casts (`u128→u64`, `u64→u32`, `usize→f64`) fixed with `try_from`, const-assert, or per-site `#[expect]`; `cast_possible_truncation`, `cast_precision_loss`, `cast_sign_loss`, `cast_possible_wrap` promoted from allow to warn
- **`doc_markdown` promoted to warn** — 1,010 bare identifiers in doc comments backticked across all 29 crates; lint promoted from allow to warn
- **`missing_errors_doc` promoted to warn** — 780 public `Result`-returning functions now have `# Errors` sections; lint promoted from allow to warn
- **Smart file refactoring** — 4 of top 5 largest files decomposed into domain-driven submodules: `key_export/` (types/crypto/export/import/tests), `capabilities/` (type/discovery/compliance/infrastructure), `providers/base/` (configuration/performance/schema/trait/defaults), `entropy/` (collect/helpers/hsm_selection/info/types/tests)
- **Production mock evolution** — `quantum_discovery.rs` evolved from fake endpoints/providers to real `PrimalDiscovery` integration; Phase 2 stubs return `NotImplemented` instead of fabricated data
- **Formatting fixed** — 4 files that drifted from `cargo fmt` corrected
- **Licensing clarified** — `LICENSE-DOCS.md` updated with full scyBorg Provenance Trio table (AGPL-3.0-or-later / ORC / CC-BY-SA 4.0)
- **Coverage** — 90.16% line (workspace), 14,610+ tests passing, 0 failures
- **Stale docs cleaned** — `specs/PROJECT_STATUS.md` archived to fossilRecord, `scripts/` debris removed, root docs updated to current metrics

### March 29, 2026 -- Wave 23: Concurrency Deep Debt — ABBA Deadlock Fix, Sleep Elimination, Async Evolution

- **ABBA deadlock eliminated** — Root cause of ~30% intermittent test hangs found and fixed: `tests/e2e/hsm_operations.rs` had 3 global `OnceLock<Mutex<HashMap>>` statics with inconsistent lock ordering (`initialize_hsm` locked A→B, `check_hsm_health` locked B→A). Replaced with per-test `HsmTestContext` struct — zero global state, zero cross-test contamination
- **Global mutable state eliminated from all e2e tests** — `rate_limiting.rs` (3 global `OnceLock<AtomicUsize>` → `thread_local! Cell`), 4 disaster_recovery modules (`OnceLock<Mutex>` → per-invocation context structs)
- **`HybridIntelligenceSystem::initialize()` evolved to async** — Was sync with `blocking_write()` that panicked in tokio runtime context; now uses `.write().await`. Test un-ignored and passing
- **Ed25519 verify API corrected** — `GeneticCryptoProvider::verify()` restored to accept public key (not private seed); property tests updated to derive public key before verification
- **Sleep debt eliminated** — 1s `proof_verifier` (UUID uniqueness), 1100ms `key_expiration` (zero-duration interval), 100ms `idle_tracking`, 56ms circuit breaker (configurable cooldown), plus ~10 smaller test sleeps replaced with instant-past timestamps, `yield_now()`, or zero-duration intervals
- **Proof signature uniqueness** — `generate_proof()` now uses nanosecond timestamp + UUID instead of second-resolution timestamp
- **Dead code cleanup** — Deleted 8 orphan files from `android_strongbox/` (49.6 KB): `health.rs`, `attestation.rs`, `core_simple.rs`, `device_info.rs`, `entropy.rs`, `keystore.rs`, `pixel8_setup.rs`, `type_safe_wrapper.rs` — none referenced by `mod.rs`
- **Debris cleanup** — Removed `audit.log` artifacts from repo root and 2 crate directories
- **Method count** — 93 crypto methods (registered `derive_lineage_beacon_key`)
- **Tests** — 15,184 passed, 0 failed, 138 ignored; 10/10 stress tests clean on both e2e binaries under full parallelism

### March 28, 2026 -- Wave 22: Deep Debt Evolution — Hot-Path Clones, iOS Fake Crypto, Mock Cleanup

- **Hot-path clone elimination** — Genetic RPC handlers take `&Value` + `Deserialize::deserialize` instead of `Value::clone()` per request
- **iOS safe_ffi** — Placeholder keys/zeroed signatures replaced with `not_implemented` errors (fake crypto success is a security risk)
- **Mock label cleanup** — "(mock)" → "(software fallback)" in production logs; stale comment removed
- **Examples** — `Box<dyn Error>` → `anyhow::Result<()>`
- **Socket path centralization** — `doctor.rs` uses `DEFAULT_SOCKET_PATH`
- **Dependency audit clean** — No `ring`, no `openssl-sys`, no `sled`, no `unsafe`; `blake3` pure-featured
- **Coverage** — 90.05% line maintained, 0 failures

### March 28, 2026 -- Wave 21: StrongBox HSM Abstraction, Production Mock Evolution, Self-Knowledge

- **Canonical `HsmKeyProvider` trait** — Unified HSM provider trait in `beardog-traits` with supporting types in `beardog-types`; object-safe, `#[async_trait]`, supports software and hardware backends
- **HSM Provider Registry** — `HsmProviderRegistry` with `discover()`, `select()`, `software_fallback()` for automatic provider selection based on `PreferHardware`/`RequireHardware`/`SoftwareOnly` preference
- **Android StrongBox bridge** — `keystore.rs` rewritten with `#[cfg(target_os = "android")]` JNI bridge; non-Android stubs for development
- **5 legacy HSM traits deprecated** — Migration doc sections added to `CryptoProvider`, `HsmProviderTrait`, `HsmCapabilities`, 2x `HsmProvider`
- **Production mock evolution** — Real timing in adapter responses, semver validation in `validate_canonical_usage()`, live state in `UniversalHsmManager`
- **Self-knowledge** — `SongbirdClient` → `OrchestratorRegistryClient`, `BiomeOSPaths` → `PlatformPaths`, string literals genericized
- **Dead code cleanup** — Removed orphan `songbird_client.rs` and `discovery_adapter.rs`
- **Socket path centralization** — 3 new `DEFAULT_*` constants replacing inline `/tmp/beardog*` paths
- **Coverage** — 90.05% line, 14,610+ tests passing

### March 28, 2026 -- Wave 18-20: Deep Debt Evolution, 90% Coverage, Semantic Naming

- **90% coverage achieved** — 90.03% line, 89.18% region, 84.90% function (llvm-cov workspace)
- **15,085+ tests passing** — 0 failures (up from 14,600 in Wave 17)
- **UniBin v1.1 compliance** — `PrimalIdentity::from_env()` evolved to standalone fallback with `is_standalone()` accessor
- **TCP/Unix read timeout** — All NDJSON read sites wrapped with `tokio::time::timeout(30s)` per primalSpring audit
- **NDJSON wire format documented** — README transport section, capability metadata includes `wire_format: "ndjson"`
- **Semantic method naming** — `domain.operation` primary names (`birdsong.encrypt`, `btsp.contact.exchange`, `crypto.derive_onion_address`) with `beardog.*` as backward-compatible aliases
- **Songbird IPC registration** — Best-effort ecosystem registry registration at server startup
- **Commented-out code cleanup** — Removed dead code from 15+ files per wateringHole standard
- **Orphan file cleanup** — Removed `ai_powered_analysis.rs` (682 LOC) and `quantum_optimizations.rs` (633 LOC) from beardog-utils
- **Clippy clean** — Resolved `float_cmp`, `await_holding_lock`, `unfulfilled_lint_expectations`, `large_stack_frames`, `io_other_error`, `redundant_clone`, `into_iter` on singletons
- **Showcase cleanup** — `#[allow(dead_code)]` replaced with `_` prefixes and `#[serde(rename)]`
- **Production stub evolution** — Migration adapters return `BearDogError::not_implemented()` with Phase 2 messages
- **Debris cleanup** — `.pedantic_clippy.toml` and `.tarpaulin.toml` removed (superseded by workspace config and llvm-cov)
- **Archive migration** — `archives/` moved to ecoPrimals fossilRecord
- **Root docs updated** — All metrics current (tests, coverage, completion status)

### March 27, 2026 -- Wave 17: Comprehensive Audit, UniBin Compliance, NDJSON & Coverage Push

- **Coverage** — 87.31% line (up from 86.70%); llvm-cov workspace pass after audit fixes
- **Tests** — 14,600+ passing
- **JSON-RPC** — Batch request support on the wire path
- **UniBin** — `--port` compliance alignment
- **NDJSON** — Wire framing fixes for streamed JSON lines
- **Observability** — Structured tracing logging on hot paths
- **Production stubs** — Further evolution (explicit not-implemented / real behavior)
- **Manifests** — Repository URLs normalized; dead feature flags removed
- **Linting** — `cast_lossless` promoted to warn
- **CI** — `beardog-types` check uses `--all-features`; file-size scan excludes `./archives/*` (not typo `./archive/*`)
- **Cleanup** — Commented-out debris removed (core tests, `safety.rs`, tunnel ECDSA notes)
- **Tests** — `discover_socket_path_*` unit tests clear `BEARDOG_LOCAL_SOCKET_DIR` under the existing env lock (fixes parallel-test flake)

### March 24, 2026 -- Wave 16: Full Ecosystem Audit, api_server Refactor, Zero-Copy & Debris Cleanup

- **Full wateringHole audit** — Cross-referenced PRIMAL_IPC_PROTOCOL, SEMANTIC_METHOD_NAMING, ECOBIN, UNIBIN, ZERO_HARDCODING, primalSpring standards
- **api_server.rs refactored** — 1,185 LOC → 3-module structure (`mod.rs`, `handlers.rs`, `types.rs`); production stubs → 501 Not Implemented
- **`#[expect(reason)]` hardening** — Additional migrations with unfulfilled-lint regression fixes
- **Showcase edition 2024** — All 28 showcase `Cargo.toml` updated from 2021 to 2024
- **Zero-copy IPC** — clone-free JSON detection, `Value` deserialization from borrow, `&str` borrows in method lists
- **Zero-hardcoding evolution** — 3 new named constants; `DEFAULT_UPA_URL` aliased from canonical config; DNS-SD comments genericized
- **38 deep tests** — main.rs CLI/dispatch, deploy config, CLI HSM, tunnel IPC, discovery edge cases
- **Debris cleanup** — `audit.log` artifacts removed; stale `tests/README.md` links fixed; Dockerfile aligned to 1.93.0; `tests_NEEDS_FIXING` comment references cleaned
- **14,499+ tests passing** — 0 failures
- **All gates green** — fmt, clippy `-D warnings`, doc, deny, test all clean

### March 24, 2026 -- Wave 15: Ecosystem Absorption, IPC Evolution, Semantic Naming v2.1.0 & Self-Knowledge

- **IPC error types** — `DispatchOutcome` / `IpcErrorPhase` from rhizoCrypt/LoamSpine pattern; `route_with_outcome()` + `normalize_method()`
- **Health v2.1.0** — Differentiated liveness/readiness/deep-check per wateringHole standard
- **Self-knowledge** — ~75 Songbird references removed from production handlers
- **`#[expect(reason)]`** — 30+ `#[allow(clippy::)]` evolved to `#[expect]` with reasons; 6 stale removed
- **Production stubs evolved** — Migration adapters, universal adapter timing, BirdSong → `not_implemented`
- **deny.toml** — 8 cross-primal type crate bans; wire-only contract enforcement
- **60+ deep tests** — integration HTTP router, disaster recovery, result extensions, command runner
- **Coverage 87.08% → 87.35%** (lines)
- **Discovery docs** — 5-tier pattern and credential resolution in CONTEXT.md/ARCHITECTURE.md
- **14,447+ tests passing**

### March 24, 2026 -- Wave 14: Deep Debt Audit, Test Evolution, scyBorg Compliance & Zero-Copy

- **Failing test fixed** — `test_auto_initialize_environment_precedence`
- **main.rs** — `dispatch()` extracted from `main()` for testability; **36 new tests** (33 CLI parse + 3 dispatch)
- **scyBorg** — `LYSOGENY_PROTOCOL.md` and `SCYBORG_EXCEPTION_PROTOCOL.md` added (license/compliance artifacts)
- **Zero-copy** — `bytes::Bytes` in software HSM; storage moved in `key_management`
- **Tests** — Hardcoded ports replaced with ephemeral `:0` where appropriate
- **Platform stubs** — Test stubs evolved from `panic!()` to `Result<T, E>`

### March 24, 2026 -- Wave 13: Clippy Sweep, Constants, Zero-Copy, Tests & Docs

- **Clippy fully clean** — 1,149 errors eliminated workspace-wide
- **Magic number extraction** — 60+ bare literals → named constants
- **Zero-copy optimization** — clone-heavy production files evolved
- **150 new tests** (14,201 → 14,351)
- **License compliance** — `LICENSE-DOCS.md` added (CC-BY-SA 4.0)
- **Stale docs updated**

### March 23, 2026 -- Wave 12: Cross-Ecosystem Audit, Lint Tightening & Type Safety Evolution

- **Full ecosystem audit** — Reviewed all wateringHole standards, 8 springs, and all phase1/phase2 primals for absorption opportunities
- **Clippy lint tightening** — `unwrap_used`/`expect_used` evolved from `allow` to `warn` at workspace level; all 6 production sites annotated with `#[expect(clippy::expect_used, reason = "...")]`; unused `AsyncReadExt` import removed
- **Typed error evolution** — `Box<dyn Error>` eliminated from public APIs (`receipt.rs`, `adapter_certificates.rs`, `hyperoptimized_zero_copy.rs`); `Result<(), String>` evolved to `BearDogError` in SIMD and genetics APIs
- **SPDX compliance** — All 29 showcase `main.rs` files now have SPDX headers (2,026/2,026 = 100%)
- **Commented-out code cleanup** — Removed legacy stubs from ~10 production files per wateringHole standard
- **Smart file refactoring** — 4 files near 1000 LOC split by domain into 14 focused files
- **Dead code evolution** — `#[allow(dead_code)]` removed from production; fields wired into traces or renamed with `_` prefix
- **DI-first discovery** — `get_discovery_socket_paths` refactored to pure `build_discovery_socket_paths` with injectable parameters; env-racing tests eliminated
- **Coverage tests** — 40+ new tests across `beardog-deploy` and `beardog-installer`
- **14,201 tests passing** — Up from 14,161; 0 failures, 186 ignored
- **Root docs aligned** — README, ARCHITECTURE, SECURITY, START_HERE, STATUS, ROADMAP all reflect current state

### March 23, 2026 -- Wave 11: Deep Coverage Push, Crypto Fault Injection & Zero-Copy IPC

- **Coverage 86.1% → 87.0%** — 122+ new tests across 6 crates
- **Crypto fault injection tests** — 14 adversarial tests (Blake3, ChaCha20-Poly1305, Ed25519, X25519, Tor ntor)
- **Zero-copy IPC optimization** — `unix_socket_ipc::server.rs` refactored to `BufReader::read_until` with reusable buffers
- **14,161 tests passing**

### March 23, 2026 -- Wave 10: Deep Audit — Clippy, Hardcoding, Method Aliases, File Size

- **Clippy fully clean** — 12 pedantic errors fixed
- **File size compliance** — `device.rs` split via `#[path]` extraction
- **Zero TODO/FIXME** — All resolved
- **Semantic method aliases** — `capability.list` and `primal.capabilities` added per wateringHole standard
- **14,039 tests passing**

### March 23, 2026 -- Wave 9: Unwrap Evolution, Clone Audit, Binary Unification

- **`.unwrap()` debt: 1,879 → 85** — Systematic evolution across 45+ files
- **Zero-copy clone audit** — Eliminated unnecessary `.clone()` in IPC hot paths
- **Binary collision resolved** — root `src/main.rs` is the sole `beardog` binary
- **14,029 tests passing**

### March 22, 2026 -- Wave 8: Deep Debt Execution — Coverage, Stubs, Hardcoding, File Size

- **Coverage 85.1% → 86.8%** — ~1,000+ newly covered lines across beardog-core, beardog-types,
  beardog-tunnel, beardog-genetics, beardog-cli, beardog-threat, beardog-monitoring, beardog-integration
- **Production stubs evolved** — `disaster_recovery.rs` BLAKE3 content-hash; `heartbeat.rs` real
  `AtomicUsize` connection tracking with RAII guard; `monitoring.rs` `Instant`-backed rolling timings
- **beardog-integration re-integrated** — Evolved HTTP/reqwest to Tower Atomic (Unix sockets + JSON-RPC);
  legacy HTTP integration test archived; connection tracking via Axum middleware
- **20+ hardcoded values extracted** — Ports (`DEFAULT_INTEGRATION_API_PORT`, `DEFAULT_API_SERVER_LISTEN_PORT`),
  timeouts (`DEFAULT_TARPC_CONNECT_TIMEOUT`, `DEFAULT_TARPC_REQUEST_TIMEOUT`), paths
  (`FALLBACK_REGISTRY_UNIX_SOCKET_PATH`, `BEARDOG_TCP_DISCOVERY_FILENAME`), cache TTLs, socket dirs
- **Flaky test fixes** — `test_timeout_accuracy` uses mock-time (`start_paused`);
  `test_auto_initialize_default_software_mode` uses `HsmAutoInitConfig::default()` (no env race);
  e2e stress test timing widened; mass failure test uses guaranteed early-batch failures
- **Orphan modules archived** — `zero_cost_registry.rs`, `zero_cost_registry_tests.rs`,
  legacy HTTP `integration_test.rs` → `ecoPrimals/archive/` (fossil record)
- **File size compliance** — `monitoring.rs` (1052 LOC) → `monitoring.rs` (366) +
  `monitoring_tests.rs` (576); `secure_cross_primal_messaging.rs` (1111 LOC) → production (603) +
  `_tests.rs` (506) via `#[path]` extraction
- **Production mock cleanup** — Fixed "placeholder" comments on real X25519 key exchange, UPA DTOs,
  entropy seed ID generation; documented Phase 2 items (PKCS#11, TPM, compliance handlers)
- **Hardcoding scan** — `biomeos_tmp_socket_root()` respects `BIOMEOS_TMP_ROOT`;
  `InteractionCaptureConfig::default()` replaces magic numbers; network bind fallback uses config
- **Incident handler coverage** — Full lifecycle: create/classify/escalate/assign/resolve/close
- **Performance metrics coverage** — Trend analysis, threshold violations, throughput monitoring

### March 21, 2026 -- Deep Audit: Concurrency Evolution, Coverage Push & Hardcoding Elimination

- **MSRV 1.93.0** — `rust-toolchain.toml` created and pinned; `Cargo.toml` `rust-version` updated
- **`unsafe_code` deny → forbid** — Workspace-wide `forbid(unsafe_code)` in root `Cargo.toml`
- **Zero `#[serial]`** — Eliminated all `#[serial_test::serial]` annotations across the entire
  workspace. Tests now use unique isolated resources (temp dirs, unique socket paths, instance-based
  registries) for full concurrent execution.
- **Zero test sleeps (non-chaos)** — Replaced all `tokio::time::sleep` in non-chaos tests with
  `tokio::sync::Barrier`, `tokio::sync::Notify`, `tokio::task::yield_now()`. Chaos tests retain
  timing-based patterns where appropriate.
- **Production sleep cleanup** — Removed artificial delays from health checkers (`simulated_latency`),
  AI optimization engine (`measure_network_latency`), IPC server (`wait_ready`/`wait_ready_flag`).
  Multi-transport shutdown replaced fixed 100ms sleep with `join_servers_with_timeout`.
- **Flaky test fixes** — `recovery_test_graceful_shutdown` uses `tokio::sync::Barrier` for
  deterministic synchronization. Concurrent pool test assertion relaxed for scheduler variance.
- **Hardcoding evolution** — Network addresses, ports, primal names, TLS version defaults migrated
  to `DEFAULT_*` constants with `from_env()` override. `NetworkAddressesConfig`, `RuntimeNetworkConfig`,
  `SecurityConfig` defaults all use named constants.
- **Self-knowledge enforcement** — `beardog-installer` genome targets loaded from data file via
  `include_str!`. `beardog-integration` UPA client resolves peers via `UPA_UNIX_SOCKET`,
  `CAPABILITY_UPA_REGISTER_ENDPOINT`, or `UPA_PROVIDER` env vars (no hardcoded peer names).
- **Smart refactoring** — `advanced_algorithms.rs` (976 LOC) → `mod.rs` + `types.rs` + `metrics.rs`
  + `engines.rs` + `evolution_engine.rs`. `crypto_handler.rs` (974 LOC) → 11-file module tree.
- **Dead code cleanup** — Removed orphan `alerts.rs`/`health.rs` from monitoring; cleaned
  `#[allow(dead_code)]` with leading underscores or removal; documented remaining with `reason`.
- **Mock isolation** — `testing` and `property_testing` modules gated behind
  `#[cfg(any(test, feature = "test-utils"))]`.
- **Coverage push** — `beardog-traits` 39% → 99.4%, `beardog-deploy` and `beardog-discovery`
  boosted with comprehensive integration tests. Overall: 85.1% line (102,969/121,020).
- **License normalization** — All 13 crate `Cargo.toml` files standardized to
  `license = "AGPL-3.0-or-later"` in `[package]` section.
- **Clippy cleanup** — Fixed cast lints, `mul_add` ambiguity, redundant closures, `#[derive(Default)]`
  opportunities, format string interpolation, let-binding returns.

### March 21, 2026 -- Cast Lint Tightening, Coverage Push & Capability Discovery

- **Cast lint evolution** — Removed 5 global `allow(clippy::cast_*)` from workspace Cargo.toml.
  All 301 cast sites now use per-site `#[expect(clippy::cast_*, reason = "...")]` or
  `Type::from(x)` for lossless casts. New casts must annotate individually.
- **Hardcoded primal names → capability-based discovery** — IPC socket paths, peer resolution,
  and listener binds use `BIOMEOS_IPC_NAMESPACE`, `PRIMAL_NAME`/`BEARDOG_PRIMAL_NAME` env vars
  with runtime discovery fallbacks. Production code no longer hardcodes peer primal names.
- **Coverage push**: 84% → 85.3% region / 85.7% line (13,400 → 13,850+ tests). New tests across
  CLI handlers (daemon, key_derive, key_export, entropy, HSM, streaming, birdsong), tunnel crypto
  (TLS 1.2, Tor, AES-GCM, BTSP, beacon, genetic enrollment, KDF), core (key management,
  interaction capture), and library crates (capabilities, discovery, config, types, auth, threat).
- **Test stability** — Added `serial_test::serial` on all env-dependent tests in
  beardog-capabilities. Eliminated intermittent test-pollution failures.
- **Doc link cleanup** — Resolved all `rustdoc::broken_intra_doc_links`. `cargo doc -D warnings`
  passes clean across the full workspace.
- **Root doc declutter** — 17 reference/guide docs moved from repo root to `docs/references/`.
  Session logs (335 files) moved to `ecoPrimals/archive/` (fossil record).
- **File size compliance** — Extracted 5 test modules into separate `_tests.rs` files to keep all
  .rs files under 1000 lines.
- **Build artifact cleanup** — Removed tracked receipts from git; audit.log files already
  gitignored.

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
  - `deny.toml` simplified: AGPL-3.0-or-later in global allow, per-crate exceptions removed
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

- SPDX license headers (`AGPL-3.0-or-later`) added to 1,634 .rs files
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
- Standardized AGPL-3.0-or-later license across all 29 Cargo.toml files
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

For detailed session notes, see `ecoPrimals/archive/` (fossil record) organized by date and topic.
