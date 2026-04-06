# BearDog Security

**Last Updated**: April 2, 2026
**Status**: Production Ready

---

## Security Posture

| Metric | Status |
|--------|--------|
| **Pure Rust** | ✅ 100% — No C dependencies |
| **Unsafe Code** | ✅ 0 blocks in production |
| **Panic Paths** | ✅ 0 — All `Result<T, E>` |
| **Constant-Time** | ✅ `subtle` crate for secrets |
| **Audit Logging** | ✅ All security operations |
| **HSM Support** | ✅ Software, hardware, mobile |

---

## Security Advisory

### RSA Timing Sidechannel (RUSTSEC-2023-0071)

- **Status**: ACKNOWLEDGED — Monitoring for stable fix
- **Severity**: Medium (5.9 CVSS)
- **Description**: Potential key recovery through timing sidechannels (Marvin Attack)
- **Mitigation**:
  - RSA operations use random blinding to mask timing variability
  - Network-based attacks require significant resources and positioning
  - Monitoring RustCrypto/RSA#390 for permanent fix
- **Action**: Will update to patched version when stable release available

---

## Implemented Protections

### Code Safety

- **Zero unsafe code** — `forbid(unsafe_code)` workspace-wide
- **Zero panic paths** — No `unwrap()` in production; `#[expect(clippy::expect_used, reason = "...")]` on justified invariants; `unwrap_used`/`expect_used` warn at workspace level
- **Result-based errors** — All fallible operations return `Result<T, E>`
- **Constant-time comparisons** — Uses `subtle::ConstantTimeEq` for secrets
- **Zeroize secrets** — Sensitive memory zeroized on drop

### Cryptographic Safety

- **Pure Rust crypto** — RustCrypto suite, no C bindings
- **HSM abstraction** — Software, PKCS#11, Android StrongBox backends
- **Key derivation** — HKDF-SHA256, family-scoped keys
- **AEAD encryption** — ChaCha20-Poly1305, AES-256-GCM
- **Post-quantum ready** — Kyber, Dilithium, SPHINCS+ modules

### Operational Safety

- **No hardcoded secrets** — Environment-first configuration
- **Family isolation** — Per-family key material, no cross-family leakage
- **Comprehensive logging** — All security operations audited
- **Capability discovery** — Runtime service discovery, no hardcoded endpoints

---

## Dependency Auditing

```bash
# Run security audit
cargo audit

# Check for advisories
cargo deny check advisories
```

BearDog runs regular dependency audits. All known advisories are documented and mitigated.

---

## Reporting Security Issues

Report security vulnerabilities through secure channels to the ecoPrimals development team.

**Do not** open public issues for security vulnerabilities.

---

## License

AGPL-3.0-or-later — Ensures full transparency of security implementations.
