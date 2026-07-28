# Handoff — Wave 155d: G6 Public Flip Audit

**Date**: Jul 28, 2026
**Wave**: 155d
**From**: bearDog code team (eastGate)
**To**: upstream overwatch, cellMembrane (CI), songBird (TLS excision)

---

## Summary

G6 audit complete. 21 library crates are publish-ready for crates.io. Metadata
validated, supply chain clean, zero Clippy warnings, 13,996 tests passing.

## Publish Order

Sequential publish required — topological order:

```
beardog-errors → beardog-config → beardog-discovery → beardog-types →
beardog-traits → beardog-hid → beardog-utils → beardog-capabilities →
beardog-threat → beardog-crypto → beardog-security → beardog-auth →
beardog-adapters → beardog-genetics → beardog-monitoring →
beardog-compliance → beardog-installer → beardog-core → beardog-ipc →
beardog-tower-atomic → beardog-tunnel
```

## What bearDog Owns vs What Others Own

| Item | Owner | Status |
|------|-------|--------|
| 21 library crates metadata + versions | bearDog | READY |
| `cargo publish` CI step | cellMembrane | NEEDED |
| crates.io API token | overwatch | NEEDED |
| TLS/ACME excision (Phase 3) | songBird | PENDING — absorb TLS first |
| `rustls-rustcrypto` stable release | RustCrypto upstream | EXTERNAL — monitor |

## Changes This Wave

| File | Change |
|------|--------|
| `Cargo.toml` (root) | Added `version = "0.0.2-alpha"` to `rustls-rustcrypto` git dep; added `publish = false` to root binary |
| `crates/beardog-cli/Cargo.toml` | Added `publish = false` |
| `deny.toml` | Removed 41 unnecessary RustCrypto skip entries (resolved by TLS feature-gating) |
| `crates/beardog-types/.../utils/tests.rs` | Fixed flaky shared-state test assertion |

## Not Publishable (by design)

- `beardog` / `beardog-cli`: binaries → genomeBin
- `beardog-acme`: songBird domain → deprecated
- `libtower`: C ABI → genomeBin
- `benchmarks` / `beardog-integration-tests`: internal

## Next Steps

1. **Overwatch**: Reserve `beardog-*` crate names on crates.io
2. **cellMembrane**: Wire `cargo publish` into release CI
3. **songBird**: Continue TLS absorption → beardog excises `beardog-acme` (Phase 3)
4. **bearDog**: G5 Chimera Phase 1 (next after G6 ships)
