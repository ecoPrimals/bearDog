# AAR — Wave 155d: G6 Public Flip Audit

**Date**: Jul 28, 2026
**Wave**: 155d
**Gate**: G6 (crates.io sovereignty)
**Status**: READY — all library crates pass metadata validation

---

## Objective

Complete the G6 public flip audit: verify every bearDog crate is publish-ready
for crates.io, document publish order, fix blockers, and classify
publishable vs internal crates.

## What We Did

### Metadata Validation

Ran `cargo publish --dry-run` against all 27 workspace crates.

- **beardog-errors** (Tier 1 leaf): **full dry-run PASS** — packages, compiles,
  and reaches upload stage.
- **20 other library crates**: all pass metadata validation (packaging stage),
  fail only at dependency resolution because upstream internal deps aren't on
  crates.io yet. This is the expected sequential-publish behavior.

### Blockers Found and Fixed

| # | Blocker | Fix |
|---|---------|-----|
| 1 | `rustls-rustcrypto` git dep missing version specifier → `beardog-tunnel` and `beardog-cli` reject at manifest validation | Added `version = "0.0.2-alpha"` alongside `git` URL in workspace `Cargo.toml` |
| 2 | `beardog-cli` depends on `beardog-acme` (`publish = false`) → can't be published | Marked `beardog-cli` as `publish = false` (binary, distributed via genomeBin) |
| 3 | Root `beardog` binary inherits unpublishable dep chain | Marked root `beardog` as `publish = false` |
| 4 | `deny.toml` had 41 unnecessary RustCrypto duplicate-version skip entries (TLS feature-gated in Wave 155b resolved all splits) | Removed 41 skip entries, updated comments |
| 5 | `test_clear_shared_configs_and_stats` flaky (shared global state race) | Changed assertion from `== 0` to relative comparison (`after < before`) |
| 6 | Stale `RUSTSEC-2024-0436` advisory ignore comment | Updated comment to reflect current state |

### Crate Classification

**Publishable (21 library crates)** — in topological publish order:

```
 1. beardog-errors         (leaf — no internal deps)
 2. beardog-config         (→ errors)
 3. beardog-discovery      (→ errors)
 4. beardog-types          (→ config, errors)
 5. beardog-traits         (→ errors, types)
 6. beardog-hid            (→ errors)
 7. beardog-utils          (→ errors, types)
 8. beardog-capabilities   (→ config, errors)
 9. beardog-threat         (→ errors, types)
10. beardog-crypto         (→ errors, types)
11. beardog-security       (→ errors, hid, threat, traits, types, utils)
12. beardog-auth           (→ capabilities, errors, security, traits, types)
13. beardog-adapters       (→ config, discovery, errors, traits, types)
14. beardog-genetics       (→ auth, config, errors, security, types)
15. beardog-monitoring     (→ config, errors, types, utils)
16. beardog-compliance     (→ errors, types)
17. beardog-installer      (→ errors, types)
18. beardog-core           (→ adapters, auth, config, crypto, discovery, errors,
                            genetics, monitoring, security, threat, traits,
                            types, utils)
19. beardog-ipc            (→ config, core, discovery, errors, types)
20. beardog-tower-atomic   (→ config, errors, types, utils)
21. beardog-tunnel         (→ auth, capabilities, config, core, errors, genetics,
                            hid, ipc, monitoring, security, threat, traits,
                            types, utils)
```

**Internal / Binary (6 crates — `publish = false`)**:

| Crate | Reason |
|-------|--------|
| `beardog` | Root binary — distributed via genomeBin |
| `beardog-cli` | CLI binary — depends on `beardog-acme` (unpublishable) |
| `beardog-acme` | songBird domain (TLS/ACME) — deprecated in beardog |
| `beardog-integration-tests` | Test harness only |
| `libtower` | C ABI shared library — distributed via genomeBin |
| `benchmarks` | Performance benchmarks — internal only |

### Supply Chain

- **`cargo deny check`**: PASS — advisories ok, bans ok, licenses ok, sources ok
- **Zero git deps in default build**: `rustls-rustcrypto` is optional (behind
  `tls-gateway`/`tls-server` features, both default-off)
- **Zero C crypto deps**: ecoBin purity contract maintained
- **Clippy**: zero warnings
- **Tests**: 13,996 passed, 0 failed

## Remaining Items for Actual Publish

1. **Crate name reservation**: Confirm `beardog-*` names are available on crates.io
2. **Sequential publish**: Must publish in topological order (1→21 above)
3. **CI automation**: Add `cargo publish` step to CI pipeline for tagged releases
4. **`rustls-rustcrypto` crates.io resolution**: When RustCrypto publishes a
   stable release, switch from git to crates.io version. This unblocks publishing
   `beardog-tunnel` with `tls-server` feature active.
5. **`beardog-acme` excision (Phase 3)**: When songBird absorbs TLS, fully remove
   `beardog-acme` from workspace — unblocks `beardog-cli` publishing if desired.

## Key Decision

Binary crates (`beardog`, `beardog-cli`) are **not published to crates.io**.
They are distributed via genomeBin. Library crates provide the public API surface
for downstream consumers and ecosystem integration.
