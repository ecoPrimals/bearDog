# BearDog Client Library

Tower Atomic client library for integrating with BearDog's genetic lineage and crypto API via **JSON-RPC 2.0 over Unix sockets**.

## Features

- Pure Rust IPC (zero C dependencies)
- Full async/await support with Tokio
- Type-safe request/response handling via JSON-RPC 2.0
- Comprehensive error handling (`BearDogClientError`)
- Ready for Songbird and other primal integration

## Installation

```toml
[dependencies]
beardog-client = { workspace = true }
```

## Quick Start

```rust
use beardog_client::{BearDogClient, BearDogClientError};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), BearDogClientError> {
    // Connect to BearDog via Unix socket (Tower Atomic pattern)
    let mut client = BearDogClient::connect().await?;

    // Create genesis lineage
    let genesis = client.create_lineage("tower", None).await?;
    println!("Created lineage: {}", genesis["lineage_id"]);

    Ok(())
}
```

## Usage for Songbird Integration

### Auto-Accept Same Lineage

```rust
use beardog_client::{BearDogClient, BearDogClientError};

struct SongbirdPeerHandler {
    beardog_client: BearDogClient,
}

impl SongbirdPeerHandler {
    async fn handle_peer_discovery(&self, packet: DiscoveryPacket) -> Decision {
        let Some(proof) = packet.lineage_proof else {
            return Decision::PromptUser(packet);
        };

        match self.beardog_client.verify_lineage(&proof).await {
            Ok(result) if result.valid && result.same_genesis => {
                Decision::AutoAccept
            }
            Ok(result) if result.valid => {
                Decision::PromptUser(packet)
            }
            _ => {
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
Check if BearDog is reachable via the Unix socket.

## License

AGPL-3.0-or-later
