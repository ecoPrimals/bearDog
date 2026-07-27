# Wave 155c — Deep Debt Sweep Handoff

**Date**: July 27, 2026
**From**: bearDog code team on eastGate

---

## Summary

Deep debt sweep achieving Clippy zero (31→0 warnings), wiring 2 unwired CLI handlers,
deleting orphan code, and hardening 6 production unwrap sites with proper error handling.

## Changes

| File | Change |
|------|--------|
| `beardog-crypto/src/symmetric.rs` | Added `# Errors` doc sections to 5 functions |
| `beardog-crypto/src/hashing.rs` | Removed redundant `#[must_use]`, added `# Errors` to argon2 |
| `beardog-crypto/src/lib.rs` | `BearDog` → `` `BearDog` `` in doc |
| `libtower/src/lib.rs` | `const fn`, `let else`, `i32::from(!valid)`, backtick fixes |
| `beardog-tunnel/.../safe_secure_enclave.rs` | Backtick fixes in docs |
| `beardog-tunnel/.../beacon.rs` | Added `# Errors` to proximity handlers |
| `beardog-tunnel/.../attest_enrollment.rs` | Collapsible if, backtick fix |
| `beardog-tunnel/.../asymmetric.rs` | Empty line after doc comment fix |
| `beardog-tunnel/.../symmetric.rs` | Empty line after doc comment fix |
| `beardog-tunnel/.../secrets.rs` | doc_lazy_continuation fix |
| `beardog-tunnel/.../key_derivation_helpers.rs` | `type_complexity` module-level allow (deprecated TLS code) |
| `beardog-config/.../config.rs` | Struct initializer instead of field reassignment |
| `beardog-types/.../compliance.rs` | Struct initializer instead of field reassignment |
| `beardog-types/.../hsm/config.rs` | doc_lazy_continuation fix |
| `beardog-types/.../metrics.rs` | doc_lazy_continuation fix |
| `beardog-cli/src/main.rs` | Added ExportRevocations/ImportRevocations commands + dispatch |
| `beardog-cli/.../key_revoke.rs` | Removed dead_code markers on now-wired export/import |
| `beardog-cli/.../entropy/helpers.rs` | save/load entropy → `#[cfg(test)]` |
| `beardog-cli/.../entropy/mod.rs` | Conditional re-exports for test-only functions |
| `beardog-cli/.../hsm_agnostic.rs` | `select_hsm` → `#[cfg(test)]`, path field allow |
| `beardog-cli/src/commands/mod.rs` | Deleted (orphan module) |
| `beardog-security/.../client_pin.rs` | 3 expect → BearDogError with ? |
| `beardog-security/.../hmac_secret.rs` | 3 expect → BearDogError with ? |
| `beardog-acme/Cargo.toml` | Added `publish = false` |
| `Cargo.toml` | Root workspace unchanged (beardog-acme stays in members) |
| `STATUS.md` | Updated with Wave 155b + 155c entries |

## Test Results

- Build: clean (0 errors, 0 warnings)
- Clippy: 0 warnings
- Tests: ~14,000 passing, 0 failures

## Remaining Work

- G6 crates.io publish (metadata ready, dry-run passes)
- Test module relocation for production metrics clarity
- G5 Phase 1: expand libtower exports
