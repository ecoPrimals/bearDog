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
    .with_endpoint("http://localhost:8080/btsp");
```

### Capability Registry
Type-erased storage for capability providers:
```rust
let registry = CapabilityRegistry::new(
    "beardog-node-1",
    "cryptographic_services",
    "http://localhost:8080"
);

registry.register(metadata, provider).await?;
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
use beardog_capabilities::{CapabilityRegistry, CapabilityMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create registry
    let registry = CapabilityRegistry::new(
        "my-primal",
        "security_provider",
        "http://localhost:8080"
    );

    // Register a capability
    let metadata = CapabilityMetadata::new("encryption", "2.0")
        .with_interface("EncryptionProvider")
        .with_endpoint("http://localhost:8080/encrypt");
    
    registry.register(metadata, my_provider).await?;

    // Advertise via mDNS (if enabled)
    #[cfg(feature = "mdns")]
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

