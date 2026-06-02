# BearDog Tunnel Integration Tests

This directory contains integration tests for the `beardog-tunnel` crate.

## Test Types

### 1. **Unit Tests** (in `src/`)
Standard tests within the source files — run automatically with `cargo test`.

### 2. **Integration Tests** (in `tests/`)

| Test file | Focus |
|-----------|-------|
| `tunnel_simple_comprehensive_tests.rs` | `SessionManager` lifecycle and session handling |
| `btsp_contact_exchange_tests.rs` | BTSP contact exchange over Unix socket IPC |
| `isomorphic_ipc_integration.rs` | Isomorphic IPC round-trips |
| `phase6_crypto_comprehensive_tests.rs` | Phase 6 crypto primitives |
| `rfc8448_validation_test.rs` | RFC 8448 TLS 1.3 test vectors |
| `aes_gcm_rfc5116_validation.rs` | AES-GCM per RFC 5116 |
| `sha384_cipher_suite_test.rs` | SHA-384 cipher suite validation |
| `property_crypto_roundtrips.rs` | Property-based crypto round-trip tests |
| `chaos_network_tests.rs` | Network chaos / fault injection |

## Running Tests

**All tunnel integration tests:**
```bash
cargo test -p beardog-tunnel --tests
```

**Single test file:**
```bash
cargo test -p beardog-tunnel --test tunnel_simple_comprehensive_tests
cargo test -p beardog-tunnel --test btsp_contact_exchange_tests
```

**With output:**
```bash
cargo test -p beardog-tunnel --tests -- --nocapture
```

Most tests run without hardware. BTSP contact exchange tests set `BEARDOG_HSM_MODE=software` internally.

## CI/CD Integration

```yaml
# .github/workflows/test.yml
jobs:
  test-tunnel:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tunnel integration tests
        run: cargo test -p beardog-tunnel --tests
```

## Contributing

When adding new integration tests:
1. Place the file in this `tests/` directory
2. Add an entry to the table above
3. Prefer software HSM mode for CI compatibility (`BEARDOG_HSM_MODE=software`)
4. Mark hardware-dependent tests with `#[ignore]` and document prerequisites

## Documentation

- **Project Overview:** `../../START_HERE.md`
- **Architecture:** `../../ARCHITECTURE.md`

---

**Last Updated:** Jun 2, 2026
