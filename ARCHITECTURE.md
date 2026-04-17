<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# BearDog Architecture

**Last Updated**: April 16, 2026
**Status**: Production Ready
**Crates**: 29 | **Tests**: 14,786+ | **Coverage**: 90.51% | **MSRV**: 1.93.0

---

## Core Pattern: Tower Atomic

BearDog provides **crypto atoms** to the ecoPrimals ecosystem. Every primal delegates cryptographic operations to BearDog via JSON-RPC, maintaining a single auditable crypto codebase.

```
┌─────────────┐                    ┌─────────────┐
│  Any Primal │ ←─ JSON-RPC ────→ │  BearDog    │
│ (Protocol)  │    Unix Socket     │  (Crypto)   │
└─────────────┘                    └─────────────┘
     Zero crypto code                 100 JSON-RPC methods
```

**Principles**:
- Separation of concerns: protocol logic in primals, crypto in BearDog
- Single audit surface for crypto correctness
- HSM abstraction: software (RustCrypto), hardware (PKCS#11, TPM), mobile (StrongBox, Secure Enclave) via `HsmKeyProvider` trait + `HsmProviderRegistry`
- Family isolation: per-family key derivation

---

## Crate Organization

### Core Runtime

| Crate | Purpose |
|-------|---------|
| `beardog` | Binary entry point, CLI, server |
| `beardog-core` | Core crypto handlers, JSON-RPC dispatch |
| `beardog-tunnel` | BTSP secure tunnel protocol, TLS crypto |
| `beardog-ipc` | Unix socket / TCP / named pipe transport |
| `beardog-cli` | Command-line interface |
| `beardog-client` | Client library for other primals |

### Type System & Configuration

| Crate | Purpose |
|-------|---------|
| `beardog-types` | Canonical types, config structs, HSM abstractions |
| `beardog-config` | Configuration loading, hierarchy, validation |
| `beardog-errors` | Error types and propagation |
| `beardog-traits` | Shared trait definitions |

### Security & Crypto

| Crate | Purpose |
|-------|---------|
| `beardog-security` | Trust evaluation, lineage verification |
| `beardog-genetics` | Genetic entropy, lineage key derivation |
| `beardog-hid` | Hardware device discovery (FIDO2, HID) |
| `beardog-auth` | Authentication and authorization |
| `beardog-threat` | Threat detection and ML-based analysis |

### Infrastructure

| Crate | Purpose |
|-------|---------|
| `beardog-monitoring` | Health checks, metrics, tracing |
| `beardog-workflows` | Workflow engine and processing |
| `beardog-adapters` | Certificate verification, adapter patterns |
| `beardog-capabilities` | Capability registry and discovery |
| `beardog-discovery` | Service discovery (mDNS, env, runtime) |
| `beardog-utils` | Shared utilities, AI optimization, buffer pools |

### Deployment & Integration

| Crate | Purpose |
|-------|---------|
| `beardog-deploy` | Deployment orchestration (excluded) |
| `beardog-integration` | Tower Atomic UPA client, heartbeat (excluded) |
| `beardog-production` | Production configuration |
| `beardog-installer` | Installation helpers |
| `beardog-compliance` | Compliance validation |
| `beardog-node-registry` | Node registration |
| `beardog-tower-atomic` | Tower Atomic IPC pattern |
| `beardog-integration-tests` | Cross-crate integration tests |

---

## Binary Targets

### Primary UniBin

| Binary | Source | Purpose |
|--------|--------|---------|
| `beardog` | `src/main.rs` | Primary UniBin — server, client, key ops, doctor, capabilities |

All user-facing functionality ships through the single `beardog` binary via
subcommands, per the ecoPrimals UniBin architecture.

### Tooling Exceptions (not part of the UniBin)

| Binary | Crate | Purpose | Justification |
|--------|-------|---------|---------------|
| `beardog-installer` | `beardog-installer` | Deployment tooling: installs, validates, and manages BearDog on target devices | Runs on the host, not on the deployed device; different compilation target and dependency surface |
| `deploy-pixel8` | `beardog-deploy` | Android Pixel 8 adb-based deployment script | Android-specific cross-compilation helper; not shipped to end users |

### Showcase Demos (not part of the workspace)

The `showcase/` directory contains 29 standalone demo binaries illustrating
BearDog features. They are **not workspace members** and are excluded from
`cargo build`/`cargo test`. They exist as working examples and reference
implementations, not as shipped artifacts.

---

## Trait Hierarchy

### Unified Provider Architecture

```
ConsolidatedProvider (base)
├── SecurityProvider
│   ├── CryptoProvider
│   └── HsmProvider
├── MonitoringProvider
├── StorageProvider
├── NetworkProvider
├── GeneticsProvider
├── AdapterProvider
└── WorkflowProvider
```

```rust
/// Base trait for all providers
pub trait ConsolidatedProvider: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    type Config: Send + Sync + Clone;
    type Data: Send + Sync + Clone;

    fn provider_info(&self) -> ProviderInfo;
    async fn health_check(&self) -> Result<ProviderHealth, Self::Error>;
    async fn metrics(&self) -> Result<ProviderMetrics, Self::Error>;
    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}
```

### Canonical HSM Trait (v0.10.0+)

```
HsmKeyProvider (beardog-traits::hsm)
├── RustSoftwareHsm         (RustCrypto, always available)
├── AndroidStrongBoxHsm     (JNI bridge, cfg(target_os = "android"))
├── [future] IosSecureEnclave
├── Pkcs11Provider
└── [future] TpmProvider

HsmProviderRegistry → discover() → select(PreferHardware | RequireHardware | SoftwareOnly)
```

The canonical `HsmKeyProvider` trait uses native `async fn` (RPITIT). Runtime selection for finite backend sets uses **enum dispatch** (`HsmKeyProviderBackend` and related enums) instead of `dyn Trait`, preserving monomorphization and avoiding async-trait-style indirection. It supersedes 5 legacy trait hierarchies (`CryptoProvider`, `HsmProviderTrait`, `HsmCapabilities`, unified `HsmProvider`, canonical `HsmProvider`) which carry migration doc sections and will be removed in v0.10.0.

### Domain-Specific Traits

- **`UniversalHsmProvider`**: HSM-specific operations (crypto, keys, signing)
- **`ServiceDiscovery`**: Service registry backends (Consul, etcd, Kubernetes)
- **`UniversalServiceDiscovery`**: Advanced capability-based discovery

### Trait Selection

```
What are you implementing?
├─→ General adapter/monitoring/storage → ConsolidatedProvider + specialized trait
├─→ HSM/crypto operations → UniversalHsmProvider
├─→ Service registry backend → ServiceDiscovery
└─→ Discovery manager → UniversalServiceDiscovery
```

---

## Key Architectural Patterns

### Enum dispatch (finite implementors)

Traits with a small, closed set of implementations (handlers, transports, crypto/HSM backends) dispatch through **enum wrapper types** rather than `Box<dyn Trait>`. Each variant holds a concrete type; `async` methods use native `async fn` in traits without the `async-trait` crate. This keeps call sites monomorphized and aligns with the stadial parity gate (Wave 53–55: no `#[async_trait]` in source or lockfile).

### 1. Zero-Knowledge Bootstrap

Dynamic service discovery without hardcoded endpoints:

```rust
// Dynamic discovery (no hardcoded services)
let mut bootstrap = ZeroKnowledgeBootstrap::new().await?;
let identity = bootstrap.discover_self_identity().await?;
bootstrap.start_ecosystem_listening().await?;
```

### 2. Universal Adapter Pattern

Capability-based provider abstraction:

```rust
let request = CapabilityDiscoveryRequest::new()
    .with_capability(CapabilityType::KeyManagement)
    .with_security_level(SecurityLevel::High);

let providers = universal_adapter.discover_capability(request).await?;
```

### 3. Health-Based Routing

Dynamic routing based on service health:

```rust
let healthy_providers = service_discovery
    .get_healthy_providers(CapabilityType::Storage)
    .await?;

let best_provider = healthy_providers
    .into_iter()
    .min_by_key(|p| p.response_time_ms)
    .ok_or_else(|| BearDogError::no_healthy_providers())?;
```

### 4. Multi-Family Isolation

```bash
./beardog server --family-id alpha   # beardog-alpha.sock, own key material
./beardog server --family-id bravo   # beardog-bravo.sock, fully isolated
```

Key material derived from family seed. Family A never shares keys with Family B.

---

## Quality Standards

| Standard | Enforcement |
|----------|-------------|
| Pure Rust | Zero C dependencies, RustCrypto suite only |
| Zero unsafe | `forbid(unsafe_code)` workspace-wide |
| Zero panics | No `unwrap()` in production; `#[expect]` with reason on justified invariants; `unwrap_used`/`expect_used` warn at workspace |
| Zero hardcoding | Environment variables and capability discovery |
| File size | < 1000 LOC per file (exceptions justified) |
| std preferred | `std::sync::LazyLock` over `once_cell`, etc. |
| Mock isolation | All mocks behind `#[cfg(test)]` or `test-utils` feature |
| `#[serial]` minimized | 35 tests in `beardog-production` (shared `AtomicBool`); all others concurrent |
| Zero test sleeps | Barriers, channels, notifications — no timing dependencies |
| Toolchain pinned | `rust-toolchain.toml` at 1.93.0 with cross-compile targets |

---

**Last Updated**: April 16, 2026
