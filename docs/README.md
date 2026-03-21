# BearDog Documentation

**Last Updated**: March 21, 2026
**Status**: Production Ready

---

## Start Here

1. **[START_HERE.md](../START_HERE.md)** — Quick orientation
2. **[README.md](../README.md)** — Project overview
3. **[STATUS.md](../STATUS.md)** — Current build status and metrics
4. **[CHANGELOG.md](../CHANGELOG.md)** — Version history

---

## Core Documentation

### Architecture & Design
- **[ARCHITECTURE.md](../ARCHITECTURE.md)** — System architecture overview
- **[architecture/](architecture/)** — Provider system, HSM abstraction, discovery patterns

### Development
- **[DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md)** — Contributing guide
- **[IDIOMATIC_RUST_GUIDE.md](IDIOMATIC_RUST_GUIDE.md)** — Rust best practices
- **[DEPENDENCY_RATIONALE.md](DEPENDENCY_RATIONALE.md)** — Why each dependency exists

### Operations & Deployment
- **[PRODUCTION_DEPLOYMENT_GUIDE.md](PRODUCTION_DEPLOYMENT_GUIDE.md)** — Deploy to production
- **[GETTING_STARTED.md](GETTING_STARTED.md)** — Setup guide
- **[PERFORMANCE_GUIDE.md](PERFORMANCE_GUIDE.md)** — Optimization tips

### Security
- **[SECURITY.md](../SECURITY.md)** — Security policies
- **[ENTROPY_SECURITY_ENFORCEMENT_GUIDE.md](ENTROPY_SECURITY_ENFORCEMENT_GUIDE.md)** — Entropy best practices
- **[security/](security/)** — Detailed security documentation

---

## API & Integration

- **[API_DOCUMENTATION.md](API_DOCUMENTATION.md)** — API reference
- **[BEARDOG_RPC_API.md](BEARDOG_RPC_API.md)** — JSON-RPC method catalog
- **[BEARDOG_RPC_RESPONSE_FORMATS.md](BEARDOG_RPC_RESPONSE_FORMATS.md)** — Response format guide
- **[BTSP_UNIFIED_API.md](BTSP_UNIFIED_API.md)** — BTSP tunnel API
- **[TLS_CRYPTO_API.md](TLS_CRYPTO_API.md)** — TLS crypto operations
- **[ECOSYSTEM_INTEGRATION_GUIDE.md](ECOSYSTEM_INTEGRATION_GUIDE.md)** — Integration patterns
- **[BIRDSONG_INTEGRATION_GUIDE_FOR_SONGBIRD.md](BIRDSONG_INTEGRATION_GUIDE_FOR_SONGBIRD.md)** — Songbird integration
- **[GENETIC_CRYPTO_INTEGRATION.md](GENETIC_CRYPTO_INTEGRATION.md)** — Genetic crypto integration

### Device & Hardware
- **[hardware/](hardware/)** — HSM setup
- **[devices/](devices/)** — Android StrongBox, FIDO2 tokens
- **[mobile/](mobile/)** — Mobile platform guides

---

## Guides

- **[guides/](guides/)** — How-to guides (testing, handoff, sovereignty compliance)
- **[setup/](setup/)** — Installation and configuration
- **[testing-guides/](testing-guides/)** — Test strategies and E2E testing
- **[references/](references/)** — Quick references, env vars, patterns

---

## Directory Structure

```
docs/
├── README.md               # This file
├── api/                    # API documentation
├── architecture/           # Architecture deep-dives
├── devices/                # Device-specific guides
├── examples/               # Example configurations
├── genetics/               # Genetic crypto guides
├── guides/                 # How-to guides
├── hardware/               # Hardware integration
├── mobile/                 # Mobile platform guides
├── performance/            # Performance analysis
├── references/             # Quick references and patterns
├── security/               # Security documentation
├── setup/                  # Installation and config
├── templates/              # Code templates
└── testing-guides/         # Testing documentation
```

---

**BearDog**: 100% Pure Rust Cryptographic Service Provider for the ecoPrimals Ecosystem
