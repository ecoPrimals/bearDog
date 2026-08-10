# Wave 157g — G72 Dependency Pandemic Tier 1

**Date**: August 10, 2026
**Primal**: bearDog v0.9.0
**Gate**: eastGate

---

## Summary

bearDog sheds 41 dead dependencies across 9 crates and eliminates all `tokio = ["full"]` usage. This is Tier 1 of the G72 Dependency Pandemic excision.

## Excision Results

### Per-Crate Dead Dependency Removal

| Crate | Removed | Deps Excised |
|-------|---------|-------------|
| **beardog-crypto** | 6 | `x25519-dalek`, `sha3`, `base64`, `hex`, `zeroize`, `tokio` (dev) |
| **beardog-integration** | 7 | `futures`, `thiserror`, `anyhow`, `tracing-subscriber`, `chrono`, `tokio-util`, `uuid` |
| **beardog-production** | 1 | `thiserror` |
| **beardog-adapters** | 3 | `semver`, `sha2`, `mdns-sd` |
| **beardog-monitoring** | 2 | `thiserror`, `sha2` |
| **beardog-compliance** | 1 | `thiserror` |
| **beardog-workflows** | 8 | `parking_lot`, `hex`, `hmac`, `sha2`, `base64`, `thiserror`, `futures`, `rand` |
| **beardog-node-registry** | 9 | `tokio`, `chrono`, `tracing`, `uuid`, `thiserror`, `hex`, `rand`, `parking_lot`, `sha2` |
| **beardog-deploy** | 5 | `thiserror`, `walkdir`, `path-absolutize`, `indicatif`, `chrono` |
| **TOTAL** | **41** | |

### tokio Feature Trim

| Crate | Before | After |
|-------|--------|-------|
| `beardog-integration` | `["full"]` | `["rt", "rt-multi-thread", "macros", "net", "sync", "time", "signal"]` |
| `beardog-crypto` | `["full"]` (dev) | **REMOVED** (zero tokio usage) |

### Version Alignment

- `cargo update` aligned 94 transitive dependencies to latest compatible versions

## Patterns Found

### Common Anti-Pattern: `thiserror` Declared but `BearDogError` Used

6 crates declared `thiserror` but exclusively used `BearDogError` for error handling.
This is a stadial artifact — early crates were templated with `thiserror` before `beardog-errors` matured.

### `beardog-node-registry` — Ghost Crate

9 of its declared dependencies were unused. The crate's source only imports `serde` and `beardog-errors`.
This crate may be a candidate for archival or absorption into `beardog-discovery`.

### `beardog-workflows` — Vestigial Crypto Deps

8 crypto-adjacent deps (hmac, sha2, blake3-via-hex, etc.) were declared but never used.
The workflow engine uses `beardog-types` for crypto dispatch — it doesn't do crypto directly.

## Verification

- Build: clean (0 errors)
- Clippy: 0 warnings
- Tests: 1,153 pass, 0 failures
- No behavioral regressions

## Upstream Signals

### overwatch
- bearDog is **G72 Tier 1 COMPLIANT**: zero `tokio = ["full"]`, 41 dead deps excised, all versions aligned
- Ghost crate candidates for Tier 2 review: `beardog-node-registry`, `beardog-workflows`
- bearDog workspace dep count: ~395 unique (transitive). Further reduction requires Tier 2 work (HTTP→capability.call, shared crate excision)

### All Primals
- **Pattern**: grep for `thiserror` in crates that use `YourPrimalError` — high probability of dead dep
- **Pattern**: grep for `tokio = ["full"]` — almost always overkill
- **Pattern**: `beardog-node-registry` style ghost crates — deps declared during scaffolding, never used
