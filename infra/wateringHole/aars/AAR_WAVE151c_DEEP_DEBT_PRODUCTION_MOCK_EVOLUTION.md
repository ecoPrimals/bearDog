# After Action Report: Wave 151c — Deep Debt Sweep + Production Mock Evolution

**Date**: July 26, 2026
**Author**: eastGate
**Scope**: bearDog primal — production mock elimination, hardcoding evolution, file decomposition
**Status**: COMPLETE — 72 test suites, 0 failures

---

## What was delivered

### Security-critical production mock elimination

| Item | Before | After |
|------|--------|-------|
| StrongBox detection | Env-var mock (`STRONGBOX_MOCK_AVAILABLE`) | Real `Keystore2CliTransport::probe_strongbox()` hardware probe |
| TEE detection | Hardcoded `false` | Real `Keystore2CliTransport::is_available()` probe |
| Memory protection | No-op copy (protect = identity) | ChaCha20-Poly1305 encrypt-at-rest with ephemeral key |
| Health metrics stub | Fabricated numbers (42.0 ops/s, 99.5% success) | Zero-valued fail-closed metrics |
| Safe wrapper metrics | Async futures dropped (unawaited) | Sync `parking_lot::RwLock` — metrics actually recorded |

### Hardcoding elimination

| Target | Before | After |
|--------|--------|-------|
| Installer primal name | Hardcoded `"nucleus"` | `BEARDOG_PRIMAL_NAME` env-aware, default `"beardog"` |
| CLI transport temp paths | Hardcoded `/data/local/tmp/` | `std::env::temp_dir()` |
| Pricing URL in issuance | Hardcoded `beardog.dev/pricing` | `BEARDOG_LICENSE_PRICING_URL` env-aware |
| IPC read buffers (3 files) | Inline `8192` | `beardog_types::constants::domains::buffers::UDP_PACKET_SIZE` |
| Protocol peek timeouts (2 files) | Inline `Duration::from_secs(5)` | Named `PROTOCOL_PEEK_TIMEOUT` constant |
| Deprecated IPC constant (installer) | `BIOMEOS_RUNTIME_SOCKET_SUBDIR` (11 uses) | `default_ecosystem_ipc_namespace()` function |
| Android installer fallback | `#[cfg(target_os = "android")]` gate | `cfg!()` runtime check (Silicon Atheism) |

### Architecture improvements

| Action | Detail |
|--------|--------|
| `android_transports.rs` decomposed | 929→670 lines; `Keystore2CliTransport` extracted to standalone module (273 lines) |
| Clippy auto-fix | `redundant_clone`, `format!` variable capture resolved workspace-wide |
| Items-after-statements | 11 `use`/`const` items moved before statements in 6 files |
| Stale comments cleaned | 9 `"placeholder removed"` test comments deleted; `AttestationConfig` stale comment fixed |

---

## Metrics

| Metric | Value |
|--------|-------|
| Files changed | 29 |
| Lines added | ~500 |
| Lines removed | ~280 |
| Test suites | 72 |
| Test failures | 0 |
| Clippy warnings (before) | 306 |
| Clippy warnings (after) | 273 |
| Production `todo!()`/`unimplemented!()` | 0 |
| Production `TODO`/`FIXME`/`HACK` comments | 0 |

---

## Known remaining gaps (for upstream audit)

### Platform transports (not this primal's blocker — ecosystem-level)
- iOS XPC transport: documented, not implemented (no iOS target today)
- Windows named pipes: errors (TCP fallback works)
- WASM binding: documented, not implemented
- HID: Linux-only (Android HID via JNI pending)

### Android HSM (Phase 2 roadmap)
- JNI attestation transport: fail-closed placeholder
- Keystore2 Binder (native AIDL): documented evolution path from CLI transport
- `AndroidHsmConfig::attestation_config` field: still `String`, should evolve to typed `AttestationConfig`

### Crypto providers
- RustCrypto asymmetric encrypt/decrypt: returns `not_yet_available`
- GeneticCrypto: `supports_chacha20: false`, `supports_rsa: false`
- Cross-primal messaging: XOR encryption placeholder (Phase 1.2: ChaCha20-Poly1305)

### Discovery
- DNS-SD integration: `beardog-discovery` ready (45 tests), not wired into strategy layer
- K8s discovery: partial (returns empty), needs `kube-rs`
- KMS discovery: always falls back to software HSM

---

## Recommendation

Wave 151c completes the production mock elimination tier. Remaining gaps are honestly documented with `not_yet_available` error returns (fail-closed). Priority for next wave: wire `beardog-discovery` DNS-SD into the strategy layer (unblocks ecosystem peer resolution), and evolve `AndroidHsmConfig::attestation_config` to typed struct.
