# 🔧 BearDog Capability Architecture

**Date**: January 4, 2026  
**Status**: Implemented - Capability-Based Integration  
**Purpose**: Enable N primal ecosystem without N^2 connections

---

## 🎯 Core Principle: Self-Knowledge Only

**BearDog knows**:
- ✅ I provide encryption
- ✅ I provide trust evaluation
- ✅ I provide key management
- ✅ I expose Unix socket at `/tmp/beardog-{family}.sock`
- ✅ I expose HTTP API at `:9000` (optional)

**BearDog does NOT know**:
- ❌ Songbird exists
- ❌ ToadStool exists
- ❌ How many primals exist
- ❌ Which primals need me

**biomeOS handles**:
- ✅ Routing capability requests
- ✅ Discovering which primals provide what
- ✅ Connecting primals based on capabilities

---

## 📦 Implementation

### 1. Capability Manifest

**File**: `crates/beardog-core/src/capabilities.rs`

```rust
pub struct BearDogCapabilities {
    pub primal_id: String,              // "beardog"
    pub family_id: Option<String>,       // "nat0", etc.
    pub node_id: String,                 // "tower1_abc123"
    pub provides: Vec<Capability>,       // What I can do
    pub requires: Vec<Capability>,       // What I optionally use
    pub endpoints: Vec<IpcEndpoint>,     // How to reach me
    pub metadata: HashMap<String, String>, // Version, etc.
}

pub enum Capability {
    Encryption {
        algorithms: Vec<String>,  // ["ChaCha20Poly1305", "AES-256-GCM"]
        key_types: Vec<String>,   // ["X25519", "Ed25519"]
    },
    TrustEvaluation {
        trust_models: Vec<String>, // ["family_based", "progressive_trust"]
    },
    KeyManagement {
        hsm_types: Vec<String>,    // ["software", "hardware", "strongbox"]
    },
    Signatures {
        algorithms: Vec<String>,   // ["Ed25519", "ECDSA-P256"]
    },
    // Other primals can define their own:
    Discovery {
        protocols: Vec<String>,    // Songbird would provide this
    },
    Storage {
        storage_types: Vec<String>, // ToadStool would provide this
    },
    Compute {
        compute_types: Vec<String>, // Gorilla would provide this
    },
}
```

###2. Generic IPC Server

**File**: `crates/beardog-tunnel/src/ipc_server.rs`

**Key Design**:
- ✅ Primal-agnostic Unix socket server
- ✅ Handles any `IpcHandler` implementation
- ✅ No knowledge of Songbird, ToadStool, etc.
- ✅ JSON-RPC style messages

```rust
pub trait IpcHandler: Send + Sync {
    async fn handle_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError>;
    
    async fn handle_register(
        &self,
        primal_id: String,
        capabilities: Vec<String>,
    ) -> Result<(), BearDogError>;
    
    async fn handle_event(
        &self,
        event_type: String,
        data: serde_json::Value,
    ) -> Result<(), BearDogError>;
}

pub struct IpcServer {
    socket_path: PathBuf,                    // /tmp/beardog-{family}.sock
    handler: Arc<dyn IpcHandler>,            // BearDog-specific handler
    active_connections: Arc<RwLock<Vec<String>>>,
}
```

### 3. BearDog IPC Handler

**File**: `crates/beardog-tunnel/src/ipc_handler.rs`

**Implements**: `IpcHandler` trait

**Responsibilities**:
- Handle encryption requests (from ANY primal)
- Handle trust evaluation requests (from ANY primal)
- No primal-specific logic

```rust
impl IpcHandler for BearDogIpcHandler {
    async fn handle_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        // 1. Check if we provide this capability
        if !self.capabilities.provides_capability(&request.capability) {
            return Ok(CapabilityResponse {
                status: ResponseStatus::NotAvailable,
                error: Some("Capability not provided"),
            });
        }
        
        // 2. Route to appropriate handler
        match &request.capability {
            Capability::Encryption { .. } => {
                self.handle_encryption_request(&request.params).await
            }
            Capability::TrustEvaluation { .. } => {
                self.handle_trust_evaluation(&request.params).await
            }
            _ => Err(BearDogError::system("Unsupported capability")),
        }
    }
}
```

---

## 🔄 How It Works

### Startup

```
1. biomeOS reads tower.toml
   ├── Discovers: I need encryption, discovery, storage
   └── Starts primals in dependency order

2. BearDog starts
   ├── Creates capability manifest
   ├── Binds Unix socket: /tmp/beardog-nat0.sock
   ├── Registers with biomeOS: "I provide encryption, trust"
   └── Waits for requests

3. Songbird starts (separately)
   ├── Creates capability manifest
   ├── Binds Unix socket: /tmp/songbird-nat0.sock
   ├── Binds UDP multicast: 224.0.0.251:5353
   ├── Registers with biomeOS: "I provide discovery"
   └── Waits for requests

4. ToadStool starts (future)
   ├── Creates capability manifest
   ├── Binds Unix socket: /tmp/toadstool-nat0.sock
   ├── Registers with biomeOS: "I provide storage"
   └── Waits for requests
```

### Runtime Example: ToadStool Needs Encryption

```
ToadStool:
  "I need to encrypt data before storing"
  
ToadStool → biomeOS:
  {
    "from_primal": "toadstool",
    "capability": {"type": "encryption"},
    "params": {"plaintext": "secret data"}
  }

biomeOS:
  1. Lookup: Who provides encryption?
  2. Find: BearDog at /tmp/beardog-nat0.sock
  3. Route request to BearDog

biomeOS → BearDog:
  Forward capability request

BearDog:
  1. Receive request
  2. Validate: Do I provide encryption? YES
  3. Encrypt data using BirdSong
  4. Return: {"ciphertext": "...", "family_id": "nat0"}

BearDog → biomeOS → ToadStool:
  Return encrypted data

ToadStool:
  Store encrypted data
```

### Key Insight: No Direct Connection

- ToadStool doesn't know BearDog exists ✅
- BearDog doesn't know ToadStool exists ✅
- biomeOS knows both and routes between them ✅
- **Adding Gorilla doesn't require changes to BearDog or ToadStool** ✅

---

## 🚀 Benefits

### 1. No N^2 Problem

**Traditional approach** (N^2 connections):
```
3 primals = 3×2 = 6 direct connections
10 primals = 10×9 = 90 direct connections
100 primals = 100×99 = 9,900 direct connections
```

**Capability approach** (N connections):
```
3 primals → biomeOS = 3 connections
10 primals → biomeOS = 10 connections
100 primals → biomeOS = 100 connections
```

### 2. Primal Independence

- BearDog can evolve without knowing about Songbird
- Songbird can evolve without knowing about BearDog
- New primals (Gorilla, etc.) don't require changes to existing primals

### 3. Multiple Providers

If we have TWO encryption providers:
```
biomeOS registry:
  Encryption:
    - beardog (primary, family-based)
    - gorilla-encrypt (backup, hardware-accelerated)

Request routing:
  1. Try beardog first
  2. If unavailable, try gorilla-encrypt
  3. If both fail, error
```

### 4. Zero Hardcoding

No primal knows:
- Which other primals exist
- What ports they use
- What protocols they speak

biomeOS discovers everything at runtime from capability manifests.

---

## 📊 Capability Registry

**biomeOS maintains**:

```json
{
  "primals": {
    "beardog": {
      "family_id": "nat0",
      "node_id": "tower1_abc123",
      "provides": [
        {"type": "encryption", "algorithms": ["ChaCha20Poly1305"]},
        {"type": "trust_evaluation", "trust_models": ["family_based"]},
        {"type": "key_management", "hsm_types": ["software", "hardware"]},
        {"type": "signatures", "algorithms": ["Ed25519"]}
      ],
      "endpoints": [
        {"type": "unix_socket", "path": "/tmp/beardog-nat0.sock"},
        {"type": "http", "bind_addr": "127.0.0.1:9000"}
      ]
    },
    "songbird": {
      "family_id": "nat0",
      "node_id": "tower1_xyz789",
      "provides": [
        {"type": "discovery", "protocols": ["udp_multicast", "mdns"]},
        {"type": "messaging", "protocols": ["json_rpc"]}
      ],
      "endpoints": [
        {"type": "unix_socket", "path": "/tmp/songbird-nat0.sock"},
        {"type": "udp_multicast", "addr": "224.0.0.251:5353"}
      ]
    }
  }
}
```

---

## 🔧 Integration Guide

### For New Primals

**1. Create capability manifest**:
```rust
use beardog_core::capabilities::{Capability, IpcEndpoint};

let my_capabilities = vec![
    Capability::Custom {
        name: "my_special_thing".to_string(),
        version: "1.0.0".to_string(),
        properties: HashMap::new(),
    },
];
```

**2. Implement IPC handler**:
```rust
use beardog_tunnel::ipc_server::IpcHandler;

struct MyPrimalHandler;

#[async_trait::async_trait]
impl IpcHandler for MyPrimalHandler {
    async fn handle_capability_request(...) { ... }
    async fn handle_register(...) { ... }
    async fn handle_event(...) { ... }
}
```

**3. Start IPC server**:
```rust
let handler = Arc::new(MyPrimalHandler);
let server = IpcServer::new(
    PathBuf::from("/tmp/my_primal-nat0.sock"),
    handler,
);
server.serve().await?;
```

**4. Register with biomeOS**:
```rust
// biomeOS will discover via capability manifest
// No manual registration needed
```

### For Requesting Capabilities

**From any primal**:
```rust
// Send to biomeOS (via its Unix socket)
let request = CapabilityRequest {
    from_primal: "my_primal".to_string(),
    capability: Capability::Encryption {
        algorithms: vec!["any".to_string()],
        key_types: vec!["any".to_string()],
    },
    params: HashMap::from([
        ("plaintext".to_string(), json!("hello world")),
        ("family_id".to_string(), json!("nat0")),
    ]),
    request_id: "req_123".to_string(),
};

// biomeOS routes to BearDog, returns response
let response = send_to_biomeos(request).await?;
```

---

## ✅ Current Status

| Component | Status | Location |
|-----------|--------|----------|
| Capability manifest | ✅ Implemented | `beardog-core/src/capabilities.rs` |
| Generic IPC server | ✅ Implemented | `beardog-tunnel/src/ipc_server.rs` |
| BearDog IPC handler | ✅ Implemented | `beardog-tunnel/src/ipc_handler.rs` |
| Capability routing | ⏭️ biomeOS | `phase2/biomeOS/` |
| Primal registry | ⏭️ biomeOS | `phase2/biomeOS/` |

---

## 🎯 Next Steps

### For BearDog (Phase 1)

1. ✅ Capability manifest defined
2. ✅ IPC server implemented
3. ✅ IPC handler implemented
4. ⏭️ Expose capabilities via `/capabilities` HTTP endpoint
5. ⏭️ Update `beardog-server` to start IPC server

### For biomeOS (Phase 2)

1. ⏭️ Implement capability registry
2. ⏭️ Implement capability routing
3. ⏭️ Implement primal discovery
4. ⏭️ Implement health monitoring
5. ⏭️ Implement dependency resolution

### For Ecosystem

1. ⏭️ Songbird adopts capability manifest
2. ⏭️ ToadStool adopts capability manifest
3. ⏭️ Gorilla adopts capability manifest
4. ⏭️ All primals communicate via capabilities only

---

## 📝 Key Takeaways

1. **BearDog has self-knowledge only**
   - Knows what it provides
   - Doesn't know who needs it

2. **Capability-based routing avoids N^2 problem**
   - 100 primals = 100 connections to biomeOS
   - Not 9,900 direct connections

3. **biomeOS is the orchestrator**
   - Discovers capabilities from all primals
   - Routes requests to appropriate providers
   - No primal-specific logic

4. **Zero hardcoding**
   - No ports
   - No IP addresses
   - No primal names in other primals

5. **Extensible**
   - New primals don't require changes to existing ones
   - Multiple providers for same capability
   - Custom capabilities supported

---

**Architecture Status**: ✅ Ready for ecosystem evolution without N^2 coupling

