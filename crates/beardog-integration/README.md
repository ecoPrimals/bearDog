# beardog-integration

**Note:** This crate is excluded from the workspace. HTTP REST API is owned by songBird per PRIMAL_RESPONSIBILITY_MATRIX.

**Songbird UPA Integration for BearDog**

## Overview

This crate provides the integration layer between BearDog and Songbird's Universal Port Authority (UPA), enabling federated service discovery, health monitoring, and cross-primal capability orchestration.

## Features

- **UPA Registration**: Automatic service registration with Songbird
- **Heartbeat Service**: Continuous health reporting and status updates
- **Expanded API**: 17 REST endpoints (BTSP, BirdSong, Lineage)
- **Connection Pooling**: Efficient HTTP client with keep-alive
- **Graceful Degradation**: Continues operation if UPA unavailable
- **Modern Concurrency**: Built on Tokio with async/await patterns

## Architecture

Implements Phase 3 of the BirdSong Integration Roadmap:
- See wateringHole handoffs for phase evolution plans.
- `BEARDOG_PHASE3_HANDOFF_DEC_21_2025.md` - Songbird handoff document

### Components

1. **UPA Client** (`upa_client.rs`)
   - Registration with Songbird UPA
   - Service discovery queries
   - Health status updates

2. **Integration API Server** (`api_server.rs`)
   - 6 BTSP endpoints
   - 4 BirdSong endpoints
   - 3 Lineage endpoints
   - Health and metrics endpoints

3. **Heartbeat Service** (`heartbeat.rs`)
   - 30-second interval health checks
   - System metrics collection (CPU, memory)
   - Automatic reconnection on failure

4. **Integration Orchestrator** (`lib.rs`)
   - Coordinates all components
   - Manages lifecycle
   - Handles errors gracefully

## Usage

```rust
use beardog_integration::{IntegrationConfig, BearDogIntegration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure integration
    let config = IntegrationConfig {
        upa_url: "https://localhost:8080".to_string(),
        api_port: 9000,
        service_name: "beardog-security-provider".to_string(),
        capabilities: vec![
            "security".to_string(),
            "btsp".to_string(),
            "lineage".to_string(),
            "birdsong".to_string(),
        ],
        heartbeat_interval_secs: 30,
    };

    // Start integration
    let integration = BearDogIntegration::new(config).await?;
    integration.start().await?;

    Ok(())
}
```

## API Endpoints

### BTSP (6 endpoints)
- `POST /btsp/tunnel/establish` - Establish secure tunnel
- `POST /btsp/tunnel/{id}/encrypt` - Encrypt data through tunnel
- `POST /btsp/tunnel/{id}/decrypt` - Decrypt data from tunnel
- `GET /btsp/tunnel/{id}/status` - Get tunnel status
- `DELETE /btsp/tunnel/{id}` - Close tunnel
- `GET /health` - Health check

### BirdSong (4 endpoints)
- `POST /birdsong/encrypt` - Encrypt broadcast for lineage
- `POST /birdsong/decrypt` - Decrypt broadcast
- `GET /birdsong/lineage/{node_id}` - Get lineage information
- `POST /birdsong/lineage/verify` - Verify lineage proof

### Lineage (3 endpoints)
- `POST /lineage/generate` - Generate new lineage
- `POST /lineage/verify` - Verify lineage chain
- `GET /lineage/proof/{node_id}` - Get cryptographic proof

## Testing

```bash
# Run unit tests
cargo test

# Run with mock UPA (no live Songbird needed)
cargo test --features mock-upa

# Integration tests with live Songbird
cargo test --test integration -- --ignored
```

## Configuration

Environment variables:
```bash
# UPA endpoint
export BEARDOG_UPA_URL=https://localhost:8080

# API server port
export BEARDOG_API_PORT=9000

# Heartbeat interval (seconds)
export BEARDOG_HEARTBEAT_INTERVAL=30

# Service name
export BEARDOG_SERVICE_NAME=beardog-security-provider
```

## License

AGPL-3.0

## Related Crates

- `beardog-tunnel`: BTSP provider
- `beardog-genetics`: BirdSong and lineage
- `beardog-capabilities`: Capability framework

