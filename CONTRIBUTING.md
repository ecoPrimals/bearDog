<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- Creative content: CC-BY-SA 4.0 (scyBorg provenance trio) -->

# Contributing to BearDog

BearDog is the cryptographic service provider for the ecoPrimals ecosystem,
licensed under **AGPL-3.0-or-later** (code) with **CC-BY-SA 4.0** for creative
content (docs, specs) per the scyBorg provenance trio.

---

## Standards

All contributions must comply with the wateringHole standards:

- **Edition 2024** — Rust 2024, MSRV 1.93.0 (pinned in `rust-toolchain.toml`)
- **Pure Rust** — Zero C dependencies in default build (ecoBin v3.0; `serde_yaml` eliminated Wave 56)
- **`forbid(unsafe_code)`** — Workspace-wide; no exceptions without wateringHole approval
- **Clippy pedantic + nursery** — Zero warnings (`cargo clippy --workspace --all-features`)
- **All public items documented** — `#![warn(missing_docs)]` on all library crates
- **No TODO/FIXME/HACK** — Resolve before committing; track in ROADMAP.md instead
- **< 800 lines per file** — Smart refactoring into domain-driven modules, not arbitrary splits
- **Result-based error handling** — Zero `.unwrap()` in production; `.expect("invariant")` only
- **Zero hardcoding** — Capability-based discovery; `from_env()` at boundaries
- **Self-knowledge only** — Primals discover peers at runtime, never hardcode other primal names
- **Fully concurrent tests** — 35 `#[serial]` in `beardog-production` (shared `AtomicBool` config state), zero sleeps in non-chaos tests; all others concurrent

## Workflow

```bash
cargo fmt --all -- --check         # Format check
cargo clippy --workspace --all-features  # Lint (must be 0 warnings)
cargo test --workspace             # All tests pass
cargo doc --workspace --no-deps    # Docs build clean
cargo deny check                   # Advisories, bans, licenses, sources
```

## Commit Guidelines

- Atomic commits with clear, descriptive messages
- Reference relevant specs or wateringHole standards when applicable
- SPDX license headers (`// SPDX-License-Identifier: AGPL-3.0-or-later`) on all `.rs` files

## Architecture Principles

1. **Tower Atomic Pattern** — BearDog provides crypto atoms via JSON-RPC; other primals delegate
2. **Dependency Injection** — Pure `Default` (no I/O), `from_env()` at startup boundaries
3. **Semantic method naming** — `{domain}.{operation}[.{variant}]` per wateringHole standard
4. **JSON-RPC 2.0 over NDJSON** — Single protocol per `PRIMAL_IPC_PROTOCOL.md` v3.0
5. **Sovereignty** — Primal boundaries at the wire (JSON-RPC); no embedded cross-primal code

## Testing

- Target: **90% line coverage** (llvm-cov)
- Run coverage: `cargo llvm-cov --workspace --summary-only`
- E2E, chaos, and fault injection tests live in `tests/`
- Mocks isolated behind `#[cfg(test)]` / `test-utils` feature gates

## Documentation

| Document | Purpose |
|----------|---------|
| `START_HERE.md` | Quick onboarding |
| `ARCHITECTURE.md` | System design |
| `STATUS.md` | Current metrics |
| `ROADMAP.md` | Priorities and future work |
| `specs/` | Detailed specifications |

---

**License**: AGPL-3.0-or-later (code) | CC-BY-SA 4.0 (creative content)
