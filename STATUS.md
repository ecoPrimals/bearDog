# BearDog Status

**Last Updated**: March 21, 2026
**Version**: 0.9.0
**Edition**: 2024 | **MSRV**: 1.85.0

---

## Quick Status

| Metric | Status | Details |
|--------|--------|---------|
| **Build** | Clean | Zero errors, edition 2024 |
| **Clippy** | 0 warnings | Pedantic + nursery + cast lints, workspace-centralized |
| **Missing Docs** | 0 warnings | All public items documented |
| **Pure Rust** | 100% | Zero C dependencies (ecoBin) |
| **Unsafe Code** | 0 production | `forbid(unsafe_code)` per-crate, `deny` workspace |
| **Format** | Clean | `cargo fmt` compliant |
| **TODO/FIXME** | 0 | All resolved |
| **Files > 1000 LOC** | 0 | All .rs files compliant |
| **Tests** | 13,850+ | Fully concurrent (8 threads) |
| **Coverage** | 85.3% region / 85.7% line | llvm-cov, workspace-wide |
| **Serial Tests** | 15 | Chaos/fault injection only |
| **cargo deny** | 4/4 pass | Advisories, bans, licenses, sources |
| **License** | AGPL-3.0-only | SPDX headers on all .rs files |
| **Architecture** | DI-based | Pure `Default`, `from_env()` at boundaries |
| **Cast Lints** | Per-site | Global allows removed; `#[expect(clippy::cast_*)]` per-site |
| **Production** | READY | Universal deployment |

---

## Codebase Metrics

- **Crates**: 29 in workspace
- **Rust Files**: 1,773+
- **Crypto Methods**: 91+ JSON-RPC methods
- **Platform Support**: Linux, macOS, Android, Windows, iOS

---

## Per-Crate Coverage (March 21, 2026, llvm-cov)

| Crate | Region Coverage | Line Coverage |
|-------|----------------|---------------|
| beardog-utils | 90.1% | 92.3% |
| beardog-genetics | 87.5% | 89.9% |
| beardog-ipc | 86.0% | 86.0% |
| beardog-auth | 88.0% | 93.0% |
| beardog-types | 82.5% | 84.0% |
| beardog-errors | 77.0% | 77.0% |
| beardog-tunnel | 76.0% | 80.0% |
| beardog-security | 73.7% | 73.7% |
| beardog-core | 72.0% | 75.0% |
| beardog-capabilities | 94.0% | 98.0% |
| beardog-cli | 85.0% | 85.0% |

---

## Architecture Compliance (March 2026)

| Standard | Status |
|----------|--------|
| Edition 2024 | MSRV 1.85.0, all crates |
| Pure Rust (ecoBin) | Zero C deps; blake3 pure feature; sysinfo removed |
| UniBin/ecoBin | Single binary, cross-compilation ready |
| Dependency Injection | Pure `Default`, `from_env()` at startup, `from_env_provider()` for tests |
| Zero Hardcoding | `PRIMAL_NAME` env var, capability-based discovery |
| Self-Knowledge | Primals discover peers at runtime |
| JSON-RPC + tarpc | Both protocols supported |
| AGPL-3.0-only | License verified; SPDX headers on all .rs files |
| `forbid(unsafe_code)` | Per-crate `lib.rs`; platform FFI documented per wateringHole |
| Workspace Lints | Centralized clippy pedantic + nursery |
| All Public Items Documented | 0 missing_docs warnings |
| File Size | 0 files > 1000 LOC |

---

## Recent Improvements (March 21, 2026)

### Wave 6: Cast Lint Tightening, Coverage Push & Capability-Based Discovery

- **Cast lint evolution** — Removed 5 global `allow(clippy::cast_*)` from workspace `Cargo.toml`; 301 sites now use `#[expect(clippy::cast_*, reason = "...")]` or `Type::from(x)` for lossless casts
- **Hardcoded primal names → capability-based** — IPC socket paths, peer resolution, and listener binds now use `BIOMEOS_IPC_NAMESPACE`, `PRIMAL_NAME` env vars with runtime discovery
- **Coverage push**: 84% → 85.3% region (13,400 → 13,850+ tests)
- **Test stability** — `serial_test::serial` on all env-dependent tests in beardog-capabilities; eliminated test pollution
- **Doc cleanup** — All `rustdoc::broken_intra_doc_links` resolved; `cargo doc -D warnings` clean
- **Root doc declutter** — 17 reference/guide docs moved to `docs/references/`; session logs archived
- **File size compliance** — Extracted 5 test modules to keep all .rs files under 1000 lines

### Wave 5: Concurrency Architecture & Dependency Modernization

- **Dependency Injection architecture** — Eliminated global mutable state; `Default` pure (no I/O), `from_env()` at boundaries, `from_env_provider()` for tests
- **330 → 15 `#[serial_test::serial]`** — Only chaos/fault tests serialized; all other tests fully concurrent
- **Dependency modernization**: `trust-dns-resolver` → `hickory-resolver`, `bincode` → `postcard`, `validator` 0.18 → 0.20
- **`cargo deny` passes all 4 checks** (advisories, bans, licenses, sources)
- **Hanging tests eliminated**: `CommandRunner` trait for `adb` mocking; bounded timeouts on streaming ops
- **13,400+ tests passing** at `--test-threads=8` with zero races
- **Coverage**: 80% → 84% line (llvm-cov)

### Wave 4: Deep Debt Execution & Stub Evolution

- Eliminated all 49 remaining library clippy warnings — **0 source warnings**
- Evolved 5 production stubs to real implementations (threat lifecycle, auth genetics/ecosystem/consensus, safe memory)
- Swept `#[allow]` → `#[expect(reason)]` across core crates
- beardog-ipc coverage 72% → 86%, beardog-core 59% → 62%
- Fixed failing doctests, extracted oversized test module
- `#![forbid(unsafe_code)]` on all crate `lib.rs` files (except beardog-errors)
- Dockerfile and `.pedantic_clippy.toml` updated to MSRV 1.85

### Wave 3: Edition 2024 + Total Documentation

- Upgraded to **Rust edition 2024** (MSRV 1.85.0)
- Documented **all** public items — 1,710 missing_docs warnings resolved to 0
- Fixed all 13 non-doc clippy warnings — 0 total warnings
- Removed redundant local lint attrs from 16 crate lib.rs files
- Centralized all lints in workspace Cargo.toml
- Evolved all unsafe code in beardog-utils to safe Rust
- Refactored 3 files that grew past 1000 lines from doc additions
- Edition 2024 `std::env::set_var` safety — resolved via DI architecture (Wave 5)
- Updated `gen` keyword usage (now reserved in edition 2024)

### Wave 2: Deep Compliance (prior session)

- Removed ~130 production `#[allow(dead_code)]`
- Added SPDX `AGPL-3.0-only` to 1,634 .rs files
- Removed `sysinfo` C dependency, blake3 `pure` feature on all showcase crates
- Optimized IPC hot path: `take()` instead of triple `clone()`
- Added ~250 new tests across 30+ files

### Wave 1: Audit Remediation (prior session)

- Fixed 46 clippy errors across beardog-core
- Workspace lint inheritance on all 29 crates
- Evolved hardcoded primal names to env var discovery
- Smart-refactored 3 oversize files into submodule directories
- Replaced 15 production TODOs with implementations

---

## Verification

```bash
cargo fmt --all -- --check                    # Format — clean
cargo clippy --workspace --all-features       # Lint — 0 warnings
cargo check --workspace --all-features        # Compile — clean
cargo test --workspace --lib                  # Tests
cargo doc --workspace --no-deps               # Docs — clean
```

---

**Status**: PRODUCTION READY
