# beardog-ipc - Songbird IPC Client
**Version**: 0.1.0  
**Purpose**: Primal IPC Protocol Implementation (JSON-RPC over Unix Sockets)  
**Standard**: `/wateringHole/PRIMAL_IPC_PROTOCOL.md`

## Overview

This crate provides BearDog's implementation of the ecoPrimals Primal IPC Protocol, enabling:
- Registration with Songbird service registry
- Capability-based service discovery
- JSON-RPC 2.0 communication over Unix sockets
- Runtime primal discovery (zero hardcoded knowledge)

## Usage

```rust
use beardog_ipc::{SongbirdClient, Capability};

// Register with Songbird on startup
let client = SongbirdClient::connect().await?;
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
- ✅ Songbird registration protocol
- ✅ Capability-based discovery
- ✅ Heartbeat mechanism
- ✅ Unix socket transport (tokio)

## Architecture

```
BearDog Startup
    ↓
SongbirdClient::register()
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

- [Primal IPC Protocol](../../wateringHole/PRIMAL_IPC_PROTOCOL.md)
- [Inter-Primal Interactions](../../wateringHole/INTER_PRIMAL_INTERACTIONS.md)
- [UniBin Standard](../../wateringHole/UNIBIN_ARCHITECTURE_STANDARD.md)
