# BearDog Test Suite

**Last Updated**: July 22, 2026
**Total tests**: See **[STATUS.md](../STATUS.md)** for the current passing count (authoritative).
**Coverage**: See **[STATUS.md](../STATUS.md)** — measured via `cargo llvm-cov --workspace`

---

## Running Tests

```bash
cargo test --workspace                          # All tests (fully concurrent)
cargo test --workspace -- --ignored             # Run ignored / platform-specific
cargo test -p beardog-core                      # Single crate
cargo test --test chaos                         # Chaos suite
cargo test --test fault_injection               # Fault injection suite
cargo llvm-cov --workspace --summary-only       # Coverage summary
```

---

## Test Structure

```
tests/
├── e2e/                   # End-to-end workflows
├── chaos/                 # Chaos engineering (fault injection, resilience)
├── fault_injection/       # Crypto fault injection (adversarial inputs)
├── integration/           # Cross-crate integration
├── support/               # Shared test utilities
└── *.rs                   # Top-level integration tests

crates/*/src/**/tests.rs   # Inline unit tests (#[cfg(test)])
crates/*/src/tests/        # Crate-internal test modules
```

---

## Writing Tests

### Naming

```rust
// Pattern: test_[domain]_[component]_[action]_[condition]
#[test]
fn test_security_crypto_encrypt_with_valid_key() { /* ... */ }

#[tokio::test]
async fn test_hsm_android_initialize_full_workflow() { /* ... */ }
```

### Concurrency

All tests run **fully concurrently** — no `#[serial]`, no `tokio::time::sleep` (outside chaos).
Use unique resources (ephemeral `:0` binds, temp dirs, `Arc<Notify>`) for isolation.

### Error Handling

Production code uses `Result<T, BearDogError>` — tests should exercise error paths, not just happy paths.
Test `#[cfg(test)]` blocks may use `.expect("descriptive message")`.

---

## Test Guidelines

| Category | Scope | Target |
|----------|-------|--------|
| **Unit** | Single function/method, no I/O | <10ms |
| **Integration** | Multiple components, real impls | <1s |
| **E2E** | Full workflows | 1–10s |
| **Chaos** | Fault scenarios, resilience | Variable |

---

## Related

- [STATUS.md](../STATUS.md) — authoritative metrics and per-crate coverage
- [e2e/](e2e/) — end-to-end test modules
- [chaos/](chaos/) — chaos engineering tests
