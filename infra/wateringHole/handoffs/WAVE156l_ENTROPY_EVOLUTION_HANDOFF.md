# Wave 156l — Entropy Evolution + Temporal Pattern Cleanup

**Date**: August 6, 2026
**Author**: bearDog local gate
**Status**: SHIPPED

---

## Summary

Deep evolution pass targeting entropy system unification, temporal pattern
cleanup, and dependency streamlining.

## Changes Shipped

### 1. Entropy Orchestrator Evolution

- **`generate_entropy()` → async path wired**: `generate_entropy_async()` now
  uses the FIDO2 hardware entropy path (`generate_from_hsm_async`) instead of
  always falling back to OS RNG. `generate_human_entropy()` calls the async
  path.
- **SHA3 → BLAKE3 unification**: `mix_with_human_input()` migrated from
  `sha3::Sha3_256` to `blake3::Hasher` with a domain-separated KDF context
  (`beardog-hsm-entropy-human-mix-v2`). Consistent with the rest of bearDog.
- **`sha3` dependency removed** from `beardog-security` (still used by 6 other
  crates).
- **FIDO2 quality score corrected**: Hardcoded `0.85` → canonical `0.75` (Tier
  2) matching `calculate_quality_score()`.

### 2. Entropy Provenance Metadata

- **`EntropyProvenance` struct** added to `MixEntropyResponse`: tracks which
  sources contributed (`has_human`, `has_supervised`, `has_machine`,
  `machine_explicit`).
- Callers can now distinguish auto-generated machine entropy from explicitly
  provided entropy.
- Tests added for provenance in both all-tier and auto-machine scenarios.

### 3. parking_lot Unification

Migrated `std::sync::Mutex`/`RwLock` → `parking_lot` in production code:

| File | Old | New | Benefit |
|------|-----|-----|---------|
| `beardog-production/secrets_backend.rs` | `std::sync::Mutex` + 4× poison `.map_err()` | `parking_lot::Mutex` | No poison, cleaner API |
| `beardog-threat/incident.rs` | `std::sync::RwLock` + 2× poison `.map_err()` | `parking_lot::RwLock` | No poison, 8 call sites simplified |
| `beardog-security/discovery.rs` | `std::sync::RwLock<()>` iOS placeholder | `parking_lot::RwLock<()>` | Consistency |

Test-only `std::sync::Mutex` (CLI env locks, installer, deploy) left as-is
(acceptable in test code, no poisoning concerns).

### 4. Temporal Pattern Audit (Clean)

Verified codebase is free of old patterns:
- No `lazy_static` or `once_cell` (uses `std::sync::LazyLock`/`OnceLock`)
- No `extern crate`
- No `try!` macro
- No `impl ToString` (all use `Display`)
- No `clone()-then-borrow` antipatterns
- No `Box<dyn Error>` in production (all `BearDogError`)
- All `#[allow()]` have `reason =` (64 attrs verified)
- All production `unwrap()` confined to test code

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Clippy warnings | 0 | 0 |
| Tests passing | 11,565 | 11,565 |
| `sha3` in beardog-security | yes | **removed** |
| Poison-handling boilerplate | 6 sites | **0** |
| Entropy provenance tracking | none | **full** |

## For Upstream

### songBird
- TLS/ACME excision still pending on your end. bearDog crypto surface is ready.

### overwatch
- bearDog entropy systems now have provenance tracking — useful for attribution
  auditing when sunCloud radiant attribution goes live.
- Quality scoring is consistent between orchestrator and genetic mixer.

### Near-Term Evolution Targets
- Wire `HsmEntropyOrchestrator` quality scores into tarpc `entropy.*` methods
- Provenance Trio braiding (Human + Hardware + External witnesses)
- Ledger anchoring for entropy receipts (gas-only, external membrane)
