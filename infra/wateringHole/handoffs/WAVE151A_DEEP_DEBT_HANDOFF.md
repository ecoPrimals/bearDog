# Wave 151a Deep Debt Sweep — Handoff

**Date**: July 26, 2026
**Author**: eastGate (local overwatch)
**Scope**: bearDog primal — deep debt, Silicon Atheism, pure Rust evolution, idiomatic modernization

---

## Summary

Comprehensive deep debt sweep across the bearDog workspace. 475 files changed, +3,119 / -4,969 lines. All 13,990 tests pass with zero failures. Workspace compiles clean with zero errors.

---

## Delivered

### Silicon Atheism (Platform Unification)

| Item | Status | Files |
|------|--------|-------|
| `HsmKeyProviderBackend` — remove `#[cfg]` from enum variants | DONE | `hsm_key_provider_backend.rs` |
| `HsmProviderRegistry::discover()` — runtime `is_available()` | DONE | `providers/registry.rs` |
| `hsm/mod.rs` — unconditional module exports | DONE | `hsm/mod.rs` |
| `KeystoreTransportBackend` — unified enum dispatch | DONE | `types/android_transports.rs` |
| `AttestationTransportBackend` — unified | DONE | `types/android_transports.rs` |
| `HealthMetricsTransportBackend` — unified | DONE | `types/android_transports.rs` |
| Android transport constructors — runtime `cfg!()` | DONE | `types/android.rs` |

### Pure Rust Evolution

| Item | Status | Impact |
|------|--------|--------|
| `directories`/`dirs-sys` → `etcetera` | DONE | Eliminated last C-FFI dep on Linux desktop |
| Dependency audit (deny.toml) | DONE | Linux: PASS. Mobile deps documented as platform-FFI tier |

### Clippy Pedantic Sweep

| Lint | Count Fixed |
|------|------------|
| `unused_async` | 290 functions made sync |
| `unnecessary_wraps` | 83 functions unwrapped |
| `must_use_candidate` | 589 annotations added |
| `missing_const_for_fn` | 119 functions promoted to `const` |

### Hardcoding Elimination

| Fix | Files |
|-----|-------|
| `/tmp/` and `/data/local/tmp/` → `std::env::temp_dir()` | `android_keystore.rs` |
| Compile-time pricing URL → runtime `BEARDOG_LICENSE_PRICING_URL` | `verification.rs` |
| Magic `Duration::from_secs(30)` → named constants | `server.rs`, `framing.rs`, `session_store.rs` |
| IPC namespace `"biomeos"` → env-driven `"ecosystem"` | `ipc_discovery.rs`, `socket_config.rs` |

### Unsafe Code

| Fix | Files |
|-----|-------|
| RAII `DpapiBlob` guard for Windows DPAPI | `windows_dpapi/mod.rs` |
| `// SAFETY:` docs on all unsafe blocks | `windows_dpapi/mod.rs` |
| Module-level `#[allow(unsafe_code)]` documented | `windows_dpapi/mod.rs` |

### Silent Failure Fixes

| Fix | Files |
|-----|-------|
| HID `discover()` → `Err(unsupported_platform)` on non-Linux | `beardog-hid/src/lib.rs` |
| `query_provider()` → structured `tracing::warn!` | `service_registry.rs` |

### Test Decomposition

| Original | Lines | Split Into |
|----------|-------|------------|
| `audit_comprehensive_tests.rs` | 875 | 5 files (severity, events, engine, filtering, compliance) |
| `method_gate_tests.rs` | 872 | 4 files + helpers (classification, enforcement, auth, dispatch) |

---

## Upstream Attention Required

### For overwatch audit

1. **Deprecated APIs** — 12 `#[deprecated]` sites remain as migration shims. Recommend removal wave before 1.0. See `STATUS.md` Architecture Compliance table.
2. **IPC namespace rename** — `BIOMEOS_RUNTIME_SOCKET_SUBDIR` is deprecated; new function is `default_ecosystem_ipc_namespace()` returning `"ecosystem"`. Socket paths changed from `biomeos/beardog.sock` to `ecosystem/beardog.sock`. Upstream primals referencing the old constant will get a deprecation warning.
3. **Service discovery backends** — Consul, etcd, DNS-SRV providers return `BackendUnavailable`. These are Phase 2 and owned by upstream ecosystem primals, not bearDog.
4. **REST API bridge** — 9/13 crypto endpoints in `beardog-integration` return HTTP 501. Module is excluded from workspace. Owned by upstream HTTP transport.

### For upstream primal teams

1. **songBird**: `handle_trust_seed()` is now synchronous (was async). If songBird calls this via IPC dispatch, no change needed (JSON-RPC is request/response). If calling Rust directly, remove `.await`.
2. **biomeOS**: IPC socket namespace changed from `"biomeos"` to `"ecosystem"` (env-overridable via `BIOMEOS_IPC_NAMESPACE`). Set the env var to `"biomeos"` for backward compatibility, or update socket discovery.
3. **squirrel**: `CredentialStoreBackend` enum is unchanged (already Silicon Atheism compliant). No action needed.

### Known gaps (not bearDog-owned)

- PQC simulation (`beardog-security/quantum_crypto`) — sign/encapsulate produce random bytes, verify/decapsulate fail-closed. Awaiting upstream PQC library maturity.
- Mobile HSM JNI — Android StrongBox and iOS Secure Enclave operations return `not_yet_available`. Requires AIDL binder marshalling (Android) or native FFI (iOS).
- Graph collaboration service — 5 methods return `not_yet_available`. Blocked on upstream graph security spec.

---

## Metrics

| Metric | Value |
|--------|-------|
| Files changed | 475 |
| Insertions | +3,119 |
| Deletions | -4,969 |
| Tests passing | 13,990 |
| Tests failing | 0 |
| Tests ignored | 131 |
| Compilation errors | 0 |
| Production TODO/FIXME/HACK | 0 |
| Production files > 800 LOC | 0 |
| C-FFI deps (Linux desktop) | 0 |

---

## Fossil Record

This handoff should be archived to `ecoPrimals/fossilRecord/bearDog/wave151a/` after upstream audit.

The following docs are candidates for archive:
- `infra/wateringHole/aars/AAR_WAVE150x_BOND_TYPE_BACKPRESSURE.md`
- `infra/wateringHole/aars/AAR_WAVE150x_SECURITY_HARDENING.md`
- `sporeprint/frago-wave113.md`
- `sporeprint/frago-wave114.md`
- `specs/current/integration/BEARDOG_BTSP_IMPLEMENTATION_HANDOFF.md` (superseded)
- `specs/JUPYTERHUB_DUAL_AUTH_INTEGRATION.md` (superseded)
- `specs/current/architecture/IDIOMATIC_ERROR_HANDLING_MIGRATION.md` (complete)
- `specs/current/architecture/WORKFLOW_TRAITS_MIGRATION_GUIDE.md` (excluded crate)
