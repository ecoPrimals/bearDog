# BearDog Showcase Index

**Last Updated:** April 7, 2026
**Status:** Active — 29 runnable demos, roadmap target 38
**Grade:** World-Class Quality (A+), showcase depth varies by directory

---

## What is BearDog?

**BearDog** is a sovereign cryptographic key management and security platform that provides:

- Hardware HSM Integration — YubiKey, TPM, Android StrongBox, iOS Secure Enclave
- Genetic Key Constraints — Self-enforcing, non-fungible keys with lineage
- Entropy Hierarchy — Mixed human + machine entropy (never simulated)
- BTSP Protocol — Encrypted Secure Tunnel with perfect forward secrecy
- Zero-Knowledge Bootstrap — No hardcoded config, self-discovering via capability registry
- Sovereignty First — Your keys, your control, zero vendor lock-in

**Security Grade**: **0 unsafe** — `#![forbid(unsafe_code)]` workspace-wide

---

## Available Showcases

### 00. **Local Primal — BearDog Basics** (`00-local-primal/`)
**Status:** **6/6** runnable (`01`–`06` each have `src/main.rs`)
**Purpose:** Understand what BearDog can do standalone

**Demos**:
1. `01-hello-beardog/` — Your first key generation
2. `02-hsm-discovery/` — Auto-detect hardware security modules
3. `03-key-constraints/` — Self-enforcing genetic keys
4. `04-entropy-mixing/` — Human + machine entropy
5. `05-key-lineage/` — Track key ancestry and evolution
6. `06-btsp-tunnel/` — Secure encrypted connections

**Time**: 45 minutes
**Level**: Beginner

---

### 02. **Ecosystem Integration** (`02-ecosystem-integration/`)
**Status:** **5** runnable demos (`01`–`05` with `main.rs`)
**Purpose:** BearDog working with peer primals via capability discovery

**Present (runnable)**:
1. `01-songbird-btsp/`
2. `02-nestgate-encryption/`
3. `03-toadstool-workloads/`
4. `04-squirrel-routing/`
5. `05-cross-primal-lineage/`

**Time**: 1.5 hours
**Level**: Advanced

---

### 03. **Production Features** (`03-production-features/`)
**Status:** **7** runnable demos (`01`–`07` with `main.rs`)
Examples: key rotation, policy enforcement, audit logging, monitoring, profiling, error recovery, dynamic configuration.

---

### 04. **Advanced Features** (`04-advanced-features/`)
**Status:** **10** runnable demos (`01`–`10` with `main.rs`)
**Purpose:** Advanced cryptographic operations

**Present (runnable)**:
1. `01-multi-primal-workflow/`
2. `02-threshold-key-shares/`
3. `03-hardware-attestation/`
4. `04-zero-knowledge-proofs/`
5. `05-post-quantum-readiness/`
6. `06-distributed-key-registry/`
7. `07-receipt-verification/`
8. `08-constraint-composition/`
9. `09-cross-tower-federation/`
10. `10-benchmarking-performance/`

**Time**: 1.5 hours
**Level**: Expert

---

### 05. Mixed Entropy (`05-mixed-entropy/`)
**Status:** **1** runnable demo
**Purpose:** Demonstrate entropy hierarchy and mixing

---

## Roadmap — Not Yet Implemented

The following showcase areas are planned but not yet built:

| Area | Status | Notes |
|------|--------|-------|
| `01-hardware-integration/` | Planned | Hardware HSM demos (YubiKey, TPM, StrongBox, Secure Enclave) |
| `03-network-federation/` | Planned | Distributed BearDog operations, threshold keys, cross-tower BTSP |
| `05-production-patterns/` | Planned | API key management, internal CA, secret rotation, disaster recovery |

---

## Showcase Maturity

**Runnable today:** **29** demos (`find showcase -name main.rs | wc -l`).
**Roadmap target:** **38** comprehensive demos.

| Area | Runnable `main.rs` count |
|------|-------------------------|
| `00-local-primal/` | 6 |
| `02-ecosystem-integration/` | 5 |
| `03-production-features/` | 7 |
| `04-advanced-features/` | 10 |
| `05-mixed-entropy/` | 1 |
| **Total** | **29** |

---

## Quick Start

```bash
# Build from repository root
cargo build --release

# Run a demo
cd showcase/00-local-primal/01-hello-beardog
cargo run --release
```

---

## Ecosystem Integration Points

BearDog integrates with peer primals via the Tower Atomic pattern — all discovery happens at runtime through the capability registry, never through hardcoded peer addresses.

- **Secure Tunnels** — BTSP encrypted connections with any transport-capable peer
- **Service Discovery** — mDNS + capability registration for zero-config peer finding
- **Key Federation** — Distributed key registry sync across tower boundaries
- **Encrypted Storage** — Encrypt-before-store delegation to storage-capable peers
- **Compute Attestation** — Verify remote compute environments via HSM attestation
- **Intelligent Routing** — Capability-based routing for key operations

---

## Demo Structure (Template)

```
XX-demo-name/
├── README.md           # What, why, how
├── Cargo.toml          # Dependencies
├── run.sh              # One-command execution
└── src/
    └── main.rs         # Implementation
```

---

## References

- [README.md](../README.md) — Project overview
- [START_HERE.md](../START_HERE.md) — Getting started
- [ARCHITECTURE.md](../ARCHITECTURE.md) — System design
- [specs/](../specs/) — Technical specifications

---

**Last Updated**: April 7, 2026
**Status**: Active expansion
