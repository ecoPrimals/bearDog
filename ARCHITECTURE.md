# BearDog Architecture

**Last Updated**: March 28, 2026
**Status**: Production Ready
**Crates**: 30 | **Tests**: 15,100+ | **Coverage**: 90.16% | **MSRV**: 1.93.0

---

## Core Pattern: Tower Atomic

BearDog provides **crypto atoms** to the ecoPrimals ecosystem. Every primal delegates cryptographic operations to BearDog via JSON-RPC, maintaining a single auditable crypto codebase.

```
┌─────────────┐                    ┌─────────────┐
│  Any Primal │ ←─ JSON-RPC ────→ │  BearDog    │
│ (Protocol)  │    Unix Socket     │  (Crypto)   │
└─────────────┘                    └─────────────┘
     Zero crypto code                 91+ crypto methods
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
| `beardog-deploy` | Deployment orchestration |
| `beardog-production` | Production configuration |
| `beardog-installer` | Installation helpers |
| `beardog-compliance` | Compliance validation |
| `beardog-node-registry` | Node registration |
| `beardog-tower-atomic` | Tower Atomic IPC pattern |
| `beardog-integration-tests` | Cross-crate integration tests |

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
├── [future] Pkcs11Provider
└── [future] TpmProvider

HsmProviderRegistry → discover() → select(PreferHardware | RequireHardware | SoftwareOnly)
```

The canonical `HsmKeyProvider` trait is object-safe and async. It supersedes 5 legacy trait hierarchies (`CryptoProvider`, `HsmProviderTrait`, `HsmCapabilities`, unified `HsmProvider`, canonical `HsmProvider`) which carry migration doc sections and will be removed in v0.10.0.

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
| Zero `#[serial]` | All tests concurrent via unique isolated resources |
| Zero test sleeps | Barriers, channels, notifications — no timing dependencies |
| Toolchain pinned | `rust-toolchain.toml` at 1.93.0 with cross-compile targets |

---

**Last Updated**: March 28, 2026
