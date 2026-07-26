<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 153 — Deep Debt Sweep + Production Mock Evolution Handoff

**Date**: July 26, 2026
**From**: bearDog code team @ eastGate
**To**: Upstream overwatch / primal teams

---

## What Changed

Comprehensive clippy zero-warning sweep, smart file refactoring, hardcoded value elimination, dead code cleanup, and test fixes across the entire workspace.

### Metrics

| Metric | Before | After |
|--------|--------|-------|
| Clippy warnings | 296+ | 0 |
| Build warnings | 158 | 1 (expected) |
| Largest file | 936 LOC | 736 LOC |
| Hardcoded values | 50+ inline | Named constants |
| Test failures | 3 pre-existing | 0 |
| Tests passing | ~14,000 | 14,065 |
| Files > 800 LOC | 1 | 0 |
| TODO/FIXME/HACK | 0 | 0 |

### Key Changes

1. **Clippy zero-warning workspace** — Auto-fixed + manual resolution of all warnings. Modern patterns: `let...else`, `map_or_else`, cast `#[expect]` with reasons.

2. **`solo_v2/provider.rs` refactored** — 936→736 LOC via extraction of `ceremony.rs` and `client_pin.rs`.

3. **FIDO2 VID/PID registry** consolidated into `beardog_hid::fido2_names` module.

4. **Named constants** for CTAPHID settle delay, FIDO2 provider config, capability latency tiers (47 inline values).

5. **Android stub tests fixed** — Assertions now match non-Android platform reality.

### Files Modified

```
crates/beardog-hid/src/{lib,types,linux,types_tests,linux_tests}.rs
crates/beardog-security/src/hsm/{entropy_orchestrator,fido2}/**/*.rs
crates/beardog-tunnel/src/tunnel/hsm/{android_strongbox,solo_v2,types,windows_dpapi,universal_discovery}/**/*.rs
crates/beardog-tunnel/src/unix_socket_ipc/handlers/{capabilities,fido2}/**/*.rs
examples/{entropy_hardware_comparison,test_ctap2_getinfo}.rs
```

### New Files

```
crates/beardog-tunnel/src/tunnel/hsm/solo_v2/ceremony.rs   — extracted tap timing analysis
crates/beardog-tunnel/src/tunnel/hsm/solo_v2/client_pin.rs  — extracted PIN protocol helpers
crates/beardog-security/src/hsm/fido2/ctap2/hmac_secret.rs  — hmac-secret extension (Wave 152)
infra/gates/{eastGate,grapheneGate,iosGate}.toml             — device registrations
infra/udev/70-fido2.rules                                    — FIDO2 security key udev rules
```

## Testing

```bash
cargo build --features fido2         # 0 errors, 1 expected warning
cargo clippy --features fido2 --workspace  # 0 warnings
cargo test --features fido2 --workspace    # 14,065 passing, 0 failures
```

## What Remains

### Path Deduplication (Low Priority)
- `/etc/beardog`, `/var/lib/beardog` defined in 4 modules independently
- PKCS#11 search paths duplicated between `beardog-config` and `discovery_engine`

### Timeout Consolidation (Low Priority)
- Some modules use inline `5000` ms instead of `constants/domains/timeouts.rs`

### FIDO2 Phase 2 (Future Wave)
- `ctap2_make_credential`, `ctap2_get_assertion` still return `requires_capability` errors
- `ctap2_enumerate_credentials`, `ctap2_delete_credential` not yet wired

### iOS (Wave 153+)
- `aarch64-apple-ios` target installed, `beardog-security` cross-compiles
- Needs macOS host for codesigning
