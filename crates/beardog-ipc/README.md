# beardog-ipc — Orchestrator registry IPC client
**Version**: 0.1.0  
**Purpose**: Primal IPC Protocol Implementation (JSON-RPC over Unix Sockets)  
**Standard**: `/wateringHole/PRIMAL_IPC_PROTOCOL.md`

## Overview

This crate provides BearDog's implementation of the ecoPrimals Primal IPC Protocol, enabling:
- Registration with orchestrator service registry
- Capability-based service discovery
- JSON-RPC 2.0 communication over Unix sockets
- Runtime primal discovery (zero hardcoded knowledge)

The primary client type is **`OrchestratorRegistryClient`**. The name **`SongbirdClient`** remains as a deprecated type alias for the same type.

## Usage

```rust
use beardog_ipc::{OrchestratorRegistryClient, Capability};
// `SongbirdClient` is a deprecated alias for `OrchestratorRegistryClient`.

// Register with orchestrator on startup
let client = OrchestratorRegistryClient::connect().await?;
client.register(
    "beardog",
    vec![
        Capability::Crypto,
        Capability::BTSP,
        Capability::Ed25519,
        Capability::X25519,
    ]
).await?;

// Start heartbeat
let heartbeat = client.start_heartbeat(Duration::from_secs(30));

// Discover services by capability
let crypto_service = client.find_capability("crypto").await?;
println!("Found crypto at: {}", crypto_service.endpoint);

// Make RPC call
let response = client.call(&crypto_service.endpoint, "crypto.sign", params).await?;
```

## Features

- `tarpc` - Type-safe RPC via tarpc (recommended)
- `json-rpc` - JSON-RPC 2.0 (always enabled)

## Standards Compliance

This crate implements:
- ✅ `/primal/*` namespace convention
- ✅ JSON-RPC 2.0 message format
- ✅ Orchestrator registry registration protocol
- ✅ Capability-based discovery
- ✅ Heartbeat mechanism
- ✅ Unix socket transport (tokio)

## Architecture

```
BearDog Startup
    ↓
OrchestratorRegistryClient::register()
    ↓ JSON-RPC over /primal/songbird
Songbird Registry
    ↓
Periodic Heartbeat (30s)
    ↓
Service Discovery
    ↓ find_capability("crypto")
Direct P2P Connection
    ↓ /primal/{discovered-service}
JSON-RPC Call
```

## Standards References

Standards live in `ecoPrimals/infra/wateringHole/`:
- `PRIMAL_IPC_PROTOCOL.md` — IPC protocol specification
- `INTER_PRIMAL_INTERACTIONS.md` — Cross-primal communication patterns
- `UNIBIN_ARCHITECTURE_STANDARD.md` — Universal binary architecture
