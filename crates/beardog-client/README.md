# BearDog Client Library

HTTP client library for integrating with BearDog's genetic lineage API.

## Features

- ✅ Simple, idiomatic Rust API
- ✅ Full async/await support with Tokio
- ✅ Type-safe request/response handling
- ✅ Comprehensive error handling
- ✅ Ready for Songbird and other primal integration

## Installation

```toml
[dependencies]
beardog-client = { version = "0.9.0", path = "../beardog/crates/beardog-client" }
```

## Quick Start

```rust
use beardog_client::BearDogClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = BearDogClient::new("http://localhost:9000");
    
    // Create genesis lineage
    let genesis = client.create_lineage("tower", None).await?;
    println!("Created lineage: {}", genesis.lineage_id);
    
    // Verify a lineage proof
    let verification = client.verify_lineage(&proof).await?;
    
    if verification.valid && verification.same_genesis {
        println!("✅ Same family - auto-accept!");
    }
    
    Ok(())
}
```

## Usage for Songbird Integration

### Auto-Accept Same Lineage

```rust
use beardog_client::{BearDogClient, VerificationResult};

struct SongbirdPeerHandler {
    beardog_client: BearDogClient,
}

impl SongbirdPeerHandler {
    async fn handle_peer_discovery(&self, packet: DiscoveryPacket) -> Decision {
        // Extract lineage proof
        let Some(proof) = packet.lineage_proof else {
            return Decision::PromptUser(packet);
        };

        // Verify via BearDog
        match self.beardog_client.verify_lineage(&proof).await {
            Ok(result) if result.valid && result.same_genesis => {
                // Same genetic family - auto-accept!
                Decision::AutoAccept
            }
            Ok(result) if result.valid => {
                // Different family - prompt user (sovereignty)
                Decision::PromptUser(packet)
            }
            _ => {
                // Invalid proof
                Decision::PromptUser(packet)
            }
        }
    }
}
```

## API Methods

### `create_lineage(service_type, metadata) -> CreateLineageResponse`

Create a new genesis lineage.

### `spawn_lineage(parent, service_type, metadata) -> SpawnLineageResponse`

Spawn a child lineage from a parent.

### `verify_lineage(proof) -> VerificationResult`

Verify a lineage proof (returns `valid` and `same_genesis` fields).

### `same_family(lineage_a, lineage_b) -> SameFamilyResponse`

Check if two lineages share the same genesis.

### `get_current_lineage() -> CurrentLineageResponse`

Get current node's lineage state.

### `health_check() -> bool`

Check if BearDog API is reachable.

## License

AGPL-3.0

