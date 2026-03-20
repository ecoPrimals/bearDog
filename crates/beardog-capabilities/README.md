# beardog-capabilities

**Capability-based primal interaction framework for BearDog**

## Overview

This crate provides the foundation for sovereign, capability-based interactions between primals in the ecoPrimals ecosystem. It enables dynamic service discovery, capability negotiation, and runtime adaptation without tight coupling.

## Features

- **Capability Registry**: Type-safe registration and discovery of primal capabilities
- **Runtime Discovery**: Dynamic capability querying and matching
- **Zero Coupling**: True primal sovereignty with no compile-time dependencies
- **mDNS Support**: Optional mDNS-based service discovery (feature: `mdns`)
- **Async-first**: Built on Tokio for modern concurrent patterns

## Core Concepts

### Capability Metadata
Describes what a service provides:
```rust
let metadata = CapabilityMetadata::new("secure_tunnel", "1.0")
    .with_interface("SecureTunnelProvider")
    // Endpoint comes from discovery at runtime; below is illustrative only
    .with_endpoint("https://discovered-host/capabilities/secure_tunnel");
```

### Capability Registry
Type-erased storage for capability providers. HTTP/mDNS defaults resolve from `BEARDOG_*` env vars (see crate docs); use `with_http_base` when pinning a base URL.

```rust
let registry = CapabilityRegistry::new(
    "550e8400-e29b-41d4-a716-446655440000",
    "cryptographic_services",
);

registry.register("secure_tunnel", provider, metadata);
```

### Capability Querying
Find services by capability:
```rust
let capabilities = registry.query_capabilities()
    .filter(|c| c.id == "secure_tunnel")
    .collect();
```

## Architecture

This crate implements the **Capability-Based Primal Interaction** pattern from:
- `specs/current/architecture/CAPABILITY_BASED_PRIMAL_INTERACTION.md`
- `CAPABILITY_PHASE2_COMPLETE_DEC_21_2025.md`

Key principles:
1. **Zero Coupling**: No compile-time dependencies between primals
2. **Runtime Discovery**: Services discover each other at runtime
3. **Type Safety**: Trait-based abstractions with type erasure
4. **Sovereignty**: Each primal maintains full autonomy

## Usage Example

```rust
use beardog_capabilities::{CapabilityMetadata, CapabilityRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = CapabilityRegistry::new(
        "550e8400-e29b-41d4-a716-446655440000",
        "security_provider",
    );

    let metadata = CapabilityMetadata::new("encryption", "2.0")
        .with_interface("EncryptionProvider")
        .with_endpoint("resolved-from-discovery");

    registry.register("encryption", my_provider, metadata);

    registry.advertise().await?;

    Ok(())
}
```

## Features

- `default`: Enables `mdns` feature
- `mdns`: mDNS-based service discovery (requires `mdns-sd`)

## License

AGPL-3.0

## Related Crates

- `beardog-tunnel`: BTSP provider (uses capabilities)
- `beardog-genetics`: BirdSong and lineage (uses capabilities)
- `beardog-core`: Core security primitives

