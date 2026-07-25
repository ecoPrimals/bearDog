<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Status

**Last Updated**: July 24, 2026 (Wave 150x — enrollment seed rotation)
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
| **Files > 800 LOC** | 0 | All production .rs files compliant; 2 monoliths refactored Wave 119 (server.rs→7 files, orchestrator.rs→10 files) |
| **Tests** | 13,948+ passing | Concurrent; 35 `#[serial]` in `beardog-production` (shared `AtomicBool`) |
| **Coverage** | 90.51% line | llvm-cov workspace — target 90% met |
| **Serial Tests** | 35 | Isolated to `beardog-production` config tests (global `AtomicBool` state) |
| **cargo deny** | all 4 pass | 2 advisory ignores (RSA Marvin, `paste`); `ring` + `aws-lc-rs` + `rcgen` + 16 C-crypto crates banned; TLS backend is Pure Rust `rustls-rustcrypto` |
| **License** | AGPL-3.0-or-later | SPDX headers on all .rs files |
| **Architecture** | DI-based | Pure `Default`, `from_env()` at boundaries |
| **Toolchain** | Pinned | `rust-toolchain.toml` at 1.93.0 |
| **Production** | READY | Universal deployment |

---

## Codebase Metrics

- **Crates**: 25 workspace members
- **Rust Files**: 1,947 (crates + src + tests; excludes showcase/examples)
- **JSON-RPC Methods**: 231 dispatchable (219 registry + 12 pre-dispatch gate) — see `docs/PRIMAL_CONTRACTS.md` v4.2.0 for category breakdown
- **`#[allow(`**: 81 (was 86; all carry `reason`)
- **`#[expect(`**: 644 (was 646; 2 stale removed)
- **Platform Support**: Linux, macOS, Android, Windows, iOS

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
| Pure Rust (ecoBin) | Zero C deps; `ring` + `aws-lc-rs` + `rcgen` + 16 C-crypto crates banned in `deny.toml`; TLS backend is Pure Rust `rustls-rustcrypto`; CSR via `p256` + `x509-cert`; blake3 pure feature; sysinfo removed |
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
