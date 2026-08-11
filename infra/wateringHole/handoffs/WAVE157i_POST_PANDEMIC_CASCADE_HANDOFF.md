<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 157i — Post-Pandemic Cascade + G72 Tier 2 url Excision

**Date**: August 11, 2026
**Primal**: bearDog
**Gate**: eastGate
**Wave**: 157i
**From**: bearDog code team (eastGate)

---

## Summary

Two items from the Wave 157i ecosystem blurb resolved:

1. **Darwin ios.rs import fix** — graftGate reported `ios.rs` missing `use beardog_config::env_keys` when compiling for `aarch64-apple-darwin`. Fixed and upstreamed.
2. **Binary growth investigation** — +2.9MB growth despite 41-dep removal was investigated using `cargo-bloat`. Root cause: real code additions (tarpc 7→30 methods, `beardog-genetics` crate, gossip scaffolding), not phantom dependencies. The linker was already dead-stripping unused dependency code.
3. **G72 Tier 2: `url` crate excision** — Discovered during investigation that `url` v2.5 pulled the entire ICU4X Unicode data chain (~32 transitive crates) for a single `From<url::ParseError>` impl. Excised `url` from `beardog-discovery` and `beardog-acme`.

---

## Changes

### 1. Darwin ios.rs Fix (graftGate Finding)

**File**: `crates/beardog-tunnel/src/platform/ios.rs`

- Added `#[cfg(target_os = "macos")] use beardog_config::env_keys;`
- The `create_endpoint()` macOS code path uses `env_keys::ENV_BIOMEOS_SOCKET_DIR` but the import was missing
- On Linux this compiles because the `#[cfg(target_os = "macos")]` block is excluded
- On `aarch64-apple-darwin` (graftGate M4 Mac Mini) it failed
- Import is cfg-gated to `target_os = "macos"` since only the macOS path uses it

### 2. `url` Crate Excision (G72 Tier 2)

**Files modified**:
- `Cargo.toml` (workspace) — removed `url = "2.5"` from workspace deps
- `crates/beardog-discovery/Cargo.toml` — removed `url = { workspace = true }`
- `crates/beardog-discovery/src/error.rs` — removed `From<url::ParseError>` impl, updated test
- `crates/beardog-acme/Cargo.toml` — removed `url = { workspace = true }` (never imported)

**Crate count**: 382 → 350 (**-32 transitive crates**)

**Crates removed** (entire ICU4X chain):
- `url`, `idna`, `idna_adapter`
- `icu_normalizer`, `icu_normalizer_data`
- `icu_properties`, `icu_properties_data`
- `icu_provider`, `icu_collections`, `icu_locale_core`
- `zerotrie`, `zerovec`, `zerovec-derive`
- `yoke`, `yoke-derive`, `zerofrom`, `zerofrom-derive`
- `litemap`, `tinystr`, `writeable`, `potential_utf`, `utf8_iter`
- `displaydoc`, `synstructure`
- Plus additional ICU support crates

**Why `url` was there**: Single `From<url::ParseError> for DiscoveryError` impl in `error.rs` + one test. bearDog uses string socket paths (`unix:///path/to/socket`, plain filesystem paths), never `url::Url::parse()`. The `url` crate was vestigial from an earlier HTTP discovery phase.

**Why it grew**: `url` v2.5 switched its IDNA backend from the lightweight `unicode-bidi`/`unicode-normalization` crates to the full ICU4X stack (`icu_normalizer_data` + `icu_properties_data` — multi-megabyte Unicode data tables). The `cargo update` in Wave 157g picked up this new chain. While the linker dead-stripped the data tables (no binary size impact), the 32 crates were compiled and linked every build.

### 3. Binary Growth Analysis

**Finding**: The +2.9MB growth (pre-G72 vs post-G72 on darwin) is from real code additions, not dependency inflation.

Top `.text` contributors (cargo-bloat):
| Crate | Size | % of .text | Notes |
|-------|------|-----------|-------|
| beardog_tunnel | 1.6 MiB | 24.5% | HSM backends, 30 tarpc methods, crypto handlers, FIDO2 |
| std | 1.1 MiB | 17.6% | Standard library |
| beardog_cli | 374.6 KiB | 5.7% | CLI + handler registration |
| clap_builder | 240.9 KiB | 3.7% | Argument parser |
| tokio | 223.2 KiB | 3.4% | Async runtime |
| serde_json | 204.2 KiB | 3.1% | JSON serialization |
| regex_syntax + regex_automata | 260 KiB | 4.0% | tracing-subscriber env-filter |
| beardog_genetics | 74.3 KiB | 1.1% | New crate (Wave 156l) |

Growth attributable to:
- **tarpc 7→30 methods** (Wave 156h-156i): massive monomorphized generic codegen
- **beardog-genetics** crate: new in Wave 156l
- **gossip scaffolding**: new in Wave 157e
- **tracing-subscriber regex**: 260K from env-filter feature

Binary size is **healthy** for a trust primal with this feature set. No action needed.

---

## Verification

```
cargo check:   PASS (0 errors, 1 expected Android StrongBox note)
cargo clippy:  PASS (0 warnings)
cargo test:    1,153 passed, 0 failed, 6 ignored
```

---

## Remaining bearDog Items from Wave 157i Blurb

| Item | Status | Notes |
|------|--------|-------|
| Darwin ios.rs fix | **DONE** | This handoff |
| Binary growth investigation | **DONE** | Explained — real code, not leak |
| G72 Tier 2: url excision | **DONE** | -32 crates |
| G72 Tier 2: axum 0.7→0.8 | Not applicable | bearDog does not depend on axum |
| G72 Tier 2: HTTP→songBird/capability.call | Not applicable | bearDog has no HTTP client in default build (reqwest gated behind `tls-gateway`) |
| Gossip injection → LIVE | PENDING | Scaffolding wired (4 injection points), but bearDog not counted in 7/16 live primals — likely needs depot rebuild + swarmVine socket availability on eastGate |
| `beardog-node-registry` absorption | DEFERRED | Very thin crate but still has consumers; not urgent |
| `beardog-workflows` absorption | DEFERRED | Similarly thin; review in next deep-debt wave |

---

## Cascade

Push to golgiBody/Forgejo (`git.primals.eco`) after this handoff.
