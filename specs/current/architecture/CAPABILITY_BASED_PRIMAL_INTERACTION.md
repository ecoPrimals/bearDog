# 🔌 Capability-Based Primal Interaction Specification

**"Primals know only themselves. Capabilities are discovered at runtime."**

## Document Metadata
- **Version**: 1.0.0
- **Status**: SPECIFICATION (Implementation Required)
- **Date**: December 21, 2025
- **Priority**: CRITICAL (Sovereignty Violation Fix)
- **Related**: PRIMAL_SOVEREIGNTY_ARCHITECTURE.md, ZERO_HARDCODING_SPECIFICATION.md

---

## 🚨 **The Problem: Sovereignty Violation**

### **What We Built (WRONG)** ❌
```rust
// ❌ VIOLATION: BearDog knows about "Songbird"
pub trait BtspProvider {  // "BTSP" = BearDog-to-Songbird Protocol
    // ...
}

// ❌ VIOLATION: Comments hardcode primal names
//! Implementation of Songbird's BTSP interface

// ❌ VIOLATION: Architecture diagrams show specific primals
//!                  ┌───────────────┐
//!                  │   Songbird    │  <-- Hardcoded!
//!                  │ Orchestration │
//!                  └───────────────┘
```

### **Why This Violates Sovereignty**
1. **Primal Self-Knowledge Only**: BearDog should only know about itself
2. **Dev Knowledge vs Primal Knowledge**: That "Songbird" and "BearDog" interact is dev/deployment knowledge
3. **New Primals**: What happens when Toadstool, NestGate, or new primals need the same capability?
4. **Evolution**: Primals evolve independently; capabilities shouldn't be coupled to specific primal names

---

## ✅ **The Solution: Capability-Based Discovery**

### **Core Principle**
> "BearDog advertises capabilities. Other primals discover and use them. BearDog doesn't know who."

### **Architecture**

```text
┌──────────────────────────────────────────────────────────────────┐
│                    Capability Advertisement                      │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  BearDog (Self-Knowledge):                                      │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ "I provide these capabilities:"                        │    │
│  │   • secure_tunnel (v1.0)                              │    │
│  │   • lineage_signing (v1.0)                            │    │
│  │   • broadcast_encryption (v1.0)                       │    │
│  │   • key_derivation (v1.0)                             │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  Other Primals (Discovery):                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ Songbird:   "I need: secure_tunnel, broadcast_encryption" │
│  │ Toadstool:  "I need: secure_tunnel"                   │    │
│  │ NestGate:   "I need: secure_tunnel, key_derivation"   │    │
│  │ NewPrimal:  "I need: lineage_signing"                 │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  Runtime Discovery (mDNS, etc.):                                │
│  ┌────────────────────────────────────────────────────────┐    │
│  │  1. BearDog broadcasts capabilities                    │    │
│  │  2. Primals query for needed capabilities              │    │
│  │  3. Connection established (no names known)            │    │
│  │  4. Capability used through generic interface          │    │
│  └────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 📐 **Capability Taxonomy**

### **1. Secure Tunnel Capability**
```rust
/// Generic secure tunnel capability (no primal names)
pub trait SecureTunnelProvider: Send + Sync {
    /// Establish a secure tunnel to a peer
    async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle>;
    
    /// Encrypt data for tunnel
    async fn tunnel_encrypt(&self, handle: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>>;
    
    /// Decrypt data from tunnel
    async fn tunnel_decrypt(&self, handle: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>>;
    
    /// Close tunnel
    async fn close_tunnel(&self, handle: &TunnelHandle) -> Result<()>;
    
    /// Get tunnel status
    async fn tunnel_status(&self, handle: &TunnelHandle) -> Result<TunnelStatus>;
}
```

**Metadata**:
- Capability ID: `"secure_tunnel"`
- Version: `1.0`
- Provider: BearDog (but consumers don't need to know)
- Discovery: Via mDNS `_secure_tunnel._tcp.local`

### **2. Lineage Signing Capability**
```rust
/// Generic lineage/trust chain capability
pub trait LineageSigningProvider: Send + Sync {
    /// Generate a lineage chain
    async fn generate_lineage(&self, node_id: &str, parent_id: Option<&str>) -> Result<LineageChain>;
    
    /// Sign a parent-child relationship
    async fn sign_relationship(&self, parent: &str, child: &str) -> Result<Signature>;
    
    /// Verify lineage proof
    async fn verify_lineage(&self, proof: &LineageProof) -> Result<bool>;
    
    /// Get descendants
    async fn get_descendants(&self, root: &str) -> Result<Vec<String>>;
}
```

**Metadata**:
- Capability ID: `"lineage_signing"`
- Version: `1.0`
- Provider: BearDog
- Discovery: Via mDNS `_lineage_signing._tcp.local`

### **3. Broadcast Encryption Capability**
```rust
/// Generic broadcast encryption (family-only decryption)
pub trait BroadcastEncryptionProvider: Send + Sync {
    /// Encrypt for specific lineage
    async fn encrypt_for_lineage(
        &self, 
        payload: &[u8], 
        lineage_hint: LineageHint
    ) -> Result<EncryptedBroadcast>;
    
    /// Decrypt broadcast (if authorized)
    async fn decrypt_broadcast(
        &self, 
        broadcast: &EncryptedBroadcast,
        lineage_proof: LineageProof
    ) -> Result<Vec<u8>>;
    
    /// Request decryption key
    async fn request_key(
        &self, 
        lineage_hint: &LineageHint, 
        proof: LineageProof
    ) -> Result<BroadcastKey>;
    
    /// Rotate keys for lineage
    async fn rotate_keys(&self, lineage: &str) -> Result<()>;
}
```

**Metadata**:
- Capability ID: `"broadcast_encryption"`
- Version: `1.0`
- Provider: BearDog
- Discovery: Via mDNS `_broadcast_encryption._tcp.local`

### **4. Key Derivation Capability**
```rust
/// Generic key derivation from context
pub trait KeyDerivationProvider: Send + Sync {
    /// Derive key from context
    async fn derive_key(
        &self, 
        context: &[u8], 
        info: &[u8],
        length: usize
    ) -> Result<Zeroizing<Vec<u8>>>;
    
    /// Derive multiple keys
    async fn derive_key_hierarchy(
        &self,
        root_context: &[u8],
        derivation_paths: &[DerivationPath]
    ) -> Result<Vec<Zeroizing<Vec<u8>>>>;
}
```

**Metadata**:
- Capability ID: `"key_derivation"`
- Version: `1.0`
- Provider: BearDog
- Discovery: Via mDNS `_key_derivation._tcp.local`

---

## 🔍 **Capability Discovery Protocol**

### **1. Advertisement (BearDog)**
```json
{
  "primal": {
    "id": "beardog-<uuid>",
    "type": "cryptographic_services",
    "self_description": "Sovereign cryptographic platform"
  },
  "capabilities": [
    {
      "id": "secure_tunnel",
      "version": "1.0",
      "interface": "SecureTunnelProvider",
      "endpoint": "http://localhost:8080/capabilities/secure_tunnel",
      "protocol": ["http", "grpc", "native"],
      "constraints": {
        "rate_limit": 1000,
        "requires_auth": false
      }
    },
    {
      "id": "lineage_signing",
      "version": "1.0",
      "interface": "LineageSigningProvider",
      "endpoint": "http://localhost:8080/capabilities/lineage_signing",
      "protocol": ["http", "grpc", "native"],
      "constraints": {
        "rate_limit": 500,
        "requires_auth": false
      }
    },
    {
      "id": "broadcast_encryption",
      "version": "1.0",
      "interface": "BroadcastEncryptionProvider",
      "endpoint": "http://localhost:8080/capabilities/broadcast_encryption",
      "protocol": ["http", "grpc", "native"],
      "constraints": {
        "rate_limit": 1000,
        "requires_auth": false
      }
    },
    {
      "id": "key_derivation",
      "version": "1.0",
      "interface": "KeyDerivationProvider",
      "endpoint": "http://localhost:8080/capabilities/key_derivation",
      "protocol": ["http", "grpc", "native"],
      "constraints": {
        "rate_limit": 5000,
        "requires_auth": false
      }
    }
  ],
  "discovery": {
    "mdns": "_beardog_capabilities._tcp.local",
    "http": "http://localhost:8080/capabilities",
    "ttl": 300
  }
}
```

### **2. Discovery (Any Primal)**
```rust
// Generic capability discovery (no primal names)
let discovery = CapabilityDiscovery::new();

// Query for needed capabilities
let tunnel_provider = discovery
    .find_capability::<dyn SecureTunnelProvider>("secure_tunnel")
    .await?;

let encryption_provider = discovery
    .find_capability::<dyn BroadcastEncryptionProvider>("broadcast_encryption")
    .await?;

// Use capabilities (provider identity unknown and irrelevant)
let tunnel = tunnel_provider.establish_tunnel(peer).await?;
let encrypted = encryption_provider.encrypt_for_lineage(data, hint).await?;
```

### **3. Multi-Provider Support**
```rust
// Multiple providers can offer the same capability
let all_tunnel_providers = discovery
    .find_all_capabilities::<dyn SecureTunnelProvider>("secure_tunnel")
    .await?;

// Choose based on criteria (latency, trust, etc.)
let best_provider = all_tunnel_providers
    .iter()
    .min_by_key(|p| p.metadata().latency)
    .unwrap();
```

---

## 🏗️ **Implementation Architecture**

### **BearDog Side (Provider)**

```rust
// crates/beardog-capabilities/src/lib.rs

/// Capability registry for BearDog
pub struct CapabilityRegistry {
    providers: HashMap<String, Box<dyn CapabilityProvider>>,
}

impl CapabilityRegistry {
    /// Register a capability provider
    pub fn register<T: CapabilityProvider>(&mut self, capability_id: &str, provider: T) {
        self.providers.insert(capability_id.to_string(), Box::new(provider));
    }
    
    /// Advertise capabilities via mDNS
    pub async fn advertise(&self) -> Result<()> {
        let advertisement = self.build_advertisement();
        mdns::advertise(&advertisement).await?;
        http::serve_advertisement(&advertisement).await?;
        Ok(())
    }
}

// Example registration
let mut registry = CapabilityRegistry::new();
registry.register("secure_tunnel", SecureTunnelProviderImpl::new(hsm, genetics));
registry.register("lineage_signing", LineageSigningProviderImpl::new(genetics));
registry.register("broadcast_encryption", BroadcastEncryptionProviderImpl::new(genetics));
registry.advertise().await?;
```

### **Consumer Side (Any Primal)**

```rust
// In any primal (Songbird, Toadstool, NestGate, etc.)

/// Generic capability consumer
pub struct CapabilityConsumer {
    discovered: HashMap<String, CapabilityEndpoint>,
}

impl CapabilityConsumer {
    /// Discover capabilities via mDNS
    pub async fn discover(&mut self) -> Result<()> {
        let capabilities = mdns::discover_capabilities().await?;
        for cap in capabilities {
            self.discovered.insert(cap.id, cap.endpoint);
        }
        Ok(())
    }
    
    /// Get a specific capability
    pub async fn get<T: CapabilityTrait>(&self, capability_id: &str) -> Result<Arc<T>> {
        let endpoint = self.discovered.get(capability_id)
            .ok_or_else(|| Error::CapabilityNotFound)?;
        
        // Create client proxy (HTTP, gRPC, or native)
        let client = CapabilityClient::<T>::connect(endpoint).await?;
        Ok(Arc::new(client))
    }
}
```

---

## 📋 **Migration Path**

### **Phase 1: Create Generic Capabilities** (Week 1, ~8 hours)
- [ ] Create `crates/beardog-capabilities/` crate
- [ ] Define generic capability traits (no primal names)
- [ ] Move `BtspProvider` → `SecureTunnelProvider` (rename)
- [ ] Move BirdSong traits → generic capability traits
- [ ] Define capability metadata & versioning
- [ ] Create capability registry

### **Phase 2: Implement Discovery** (Week 1, ~8 hours)
- [ ] Implement mDNS advertisement
- [ ] Implement HTTP capability endpoint (`/capabilities`)
- [ ] Create capability client/proxy
- [ ] Add capability versioning & negotiation
- [ ] Test multi-provider discovery

### **Phase 3: Update Implementations** (Week 2, ~8 hours)
- [ ] Update `BeardogBtspProvider` → `BeardogSecureTunnelProvider`
- [ ] Update API server to use generic capabilities
- [ ] Remove all "Songbird", "BTSP" hardcoded names
- [ ] Update documentation to be primal-agnostic
- [x] ~~Update showcase demos~~ (showcase fossilized Wave 49)

### **Phase 4: Backward Compatibility** (Week 2, ~4 hours)
- [ ] Create compatibility layer (BTSP → SecureTunnel alias)
- [ ] Add deprecation warnings
- [ ] Update integration guide for Songbird
- [ ] Document migration for other primals
- [ ] Create migration timeline (6 months deprecation)

---

## 🎯 **Success Criteria**

### **Code**
- [ ] Zero primal names in trait definitions
- [ ] Zero hardcoded primal knowledge in implementations
- [ ] Generic capability traits for all features
- [ ] mDNS discovery functional
- [ ] HTTP capability endpoint working

### **Discovery**
- [ ] BearDog advertises capabilities automatically
- [ ] Any primal can discover capabilities via mDNS
- [ ] Multiple providers can coexist
- [ ] Version negotiation works

### **Sovereignty**
- [ ] BearDog only knows itself
- [ ] No knowledge of "Songbird", "Toadstool", etc.
- [ ] New primals can use capabilities without code changes
- [ ] Primals evolve independently

### **Documentation**
- [ ] All docs use generic capability names
- [ ] Clear migration guide
- [ ] Examples show discovery pattern
- [ ] No primal names in capability specs

---

## 💡 **Key Benefits**

### **1. True Primal Sovereignty**
- BearDog knows only itself
- No coupling to other primals
- Evolution independent

### **2. Extensibility**
- New primals can use capabilities immediately
- No code changes needed
- Dynamic discovery

### **3. Multi-Provider**
- Multiple primals can provide same capability
- Consumers choose best provider
- Redundancy & failover

### **4. Versioning**
- Capability versions explicit
- Backward compatibility clear
- Gradual migration supported

---

## 🔄 **Evolution Example**

### **Today (WRONG)** ❌
```rust
// Songbird team needs to modify BearDog code
impl BtspProvider for BeardogProvider { ... }

// Toadstool team also needs to modify BearDog code
impl ToadstoolProvider for BeardogProvider { ... }

// NestGate team... more modifications
impl NestgateProvider for BeardogProvider { ... }
```

### **Tomorrow (RIGHT)** ✅
```rust
// BearDog registers capabilities once
registry.register("secure_tunnel", SecureTunnelProviderImpl::new());

// Songbird discovers and uses
let tunnel = discovery.get::<SecureTunnelProvider>("secure_tunnel").await?;

// Toadstool discovers and uses (same code!)
let tunnel = discovery.get::<SecureTunnelProvider>("secure_tunnel").await?;

// NestGate discovers and uses (same code!)
let tunnel = discovery.get::<SecureTunnelProvider>("secure_tunnel").await?;

// New primal X discovers and uses (no changes needed!)
let tunnel = discovery.get::<SecureTunnelProvider>("secure_tunnel").await?;
```

---

## 📚 **References**

- **PRIMAL_SOVEREIGNTY_ARCHITECTURE.md** - Sovereignty principles
- **ZERO_HARDCODING_SPECIFICATION.md** - No hardcoded values
- **mDNS RFC 6763** - Service discovery protocol
- **Capability-Based Security** - Object-capability model

---

## 🎉 **The Vision**

**Today**: BearDog "knows" it's talking to Songbird (WRONG)  
**Tomorrow**: BearDog advertises capabilities, any primal can use them (RIGHT)

**Primal Sovereignty**: Each primal knows only itself  
**Capability Discovery**: Runtime, dynamic, extensible  
**Zero Hardcoding**: No primal names in code  
**Evolution Ready**: New primals, no code changes

---

**Document Version**: 1.0  
**Date**: December 21, 2025  
**Status**: Specification (Implementation Next)  
**Priority**: CRITICAL (Sovereignty Fix)  
**Estimated Effort**: 3-4 weeks (28-32 hours)

**Let's evolve the architecture to true primal sovereignty!** 🐻🔌✨

