# 🔌 Capability Architecture Evolution - Execution Plan

**"From Hardcoded Primals to Runtime Discovery"**

**Date**: December 21, 2025  
**Status**: Ready to Execute  
**Effort**: 3-4 weeks (28-32 hours)  
**Priority**: CRITICAL (Sovereignty Violation Fix)

---

## 🎯 **What We're Fixing**

### **Current Violations** ❌
```rust
// File: btsp_provider.rs
//! Implementation of Songbird's BTSP interface  <-- Hardcoded "Songbird"

pub trait BtspProvider {  <-- "BTSP" encodes Songbird knowledge
    async fn establish_tunnel(&self, peer: PeerInfo) -> Result<TunnelHandle>;
    // ...
}

// BearDog "knows" it's serving Songbird
impl BtspProvider for BeardogBtspProvider { ... }
```

### **Target Architecture** ✅
```rust
// BearDog knows only itself, advertises capabilities
pub trait SecureTunnelProvider {  <-- Generic, no primal names
    async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle>;
    // ...
}

// Any primal can discover and use
let provider = discovery.find_capability("secure_tunnel").await?;
let tunnel = provider.establish_tunnel(peer).await?;
```

---

## 📋 **Phase 1: Create Generic Capability Framework** (8 hours)

### **Tasks**

#### **1.1 Create New Crate** (1 hour)
```bash
# Create capability-focused crate
cd /home/eastgate/Development/ecoPrimals/beardog
cargo new --lib crates/beardog-capabilities
```

**Files to create**:
- `crates/beardog-capabilities/src/lib.rs` - Main exports
- `crates/beardog-capabilities/src/traits.rs` - Generic capability traits
- `crates/beardog-capabilities/src/registry.rs` - Capability registry
- `crates/beardog-capabilities/src/discovery.rs` - Discovery protocol
- `crates/beardog-capabilities/src/metadata.rs` - Capability metadata
- `crates/beardog-capabilities/Cargo.toml` - Dependencies

#### **1.2 Define Generic Traits** (3 hours)
Create `crates/beardog-capabilities/src/traits.rs`:

```rust
/// Generic secure tunnel capability (replaces BtspProvider)
#[async_trait]
pub trait SecureTunnelProvider: Send + Sync {
    async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle>;
    async fn tunnel_encrypt(&self, handle: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>>;
    async fn tunnel_decrypt(&self, handle: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>>;
    async fn close_tunnel(&self, handle: &TunnelHandle) -> Result<()>;
    async fn tunnel_status(&self, handle: &TunnelHandle) -> Result<TunnelStatus>;
}

/// Generic lineage signing capability
#[async_trait]
pub trait LineageSigningProvider: Send + Sync {
    async fn generate_lineage(&self, node_id: &str, parent_id: Option<&str>) -> Result<LineageChain>;
    async fn sign_relationship(&self, parent: &str, child: &str) -> Result<Signature>;
    async fn verify_lineage(&self, proof: &LineageProof) -> Result<bool>;
    async fn get_descendants(&self, root: &str) -> Result<Vec<String>>;
}

/// Generic broadcast encryption capability
#[async_trait]
pub trait BroadcastEncryptionProvider: Send + Sync {
    async fn encrypt_for_lineage(&self, payload: &[u8], hint: LineageHint) -> Result<EncryptedBroadcast>;
    async fn decrypt_broadcast(&self, broadcast: &EncryptedBroadcast, proof: LineageProof) -> Result<Vec<u8>>;
    async fn request_key(&self, hint: &LineageHint, proof: LineageProof) -> Result<BroadcastKey>;
    async fn rotate_keys(&self, lineage: &str) -> Result<()>;
}

/// Generic key derivation capability
#[async_trait]
pub trait KeyDerivationProvider: Send + Sync {
    async fn derive_key(&self, context: &[u8], info: &[u8], length: usize) -> Result<Zeroizing<Vec<u8>>>;
    async fn derive_key_hierarchy(&self, root: &[u8], paths: &[DerivationPath]) -> Result<Vec<Zeroizing<Vec<u8>>>>;
}
```

#### **1.3 Capability Registry** (2 hours)
Create `crates/beardog-capabilities/src/registry.rs`:

```rust
/// Capability metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMetadata {
    pub id: String,
    pub version: String,
    pub interface: String,
    pub endpoint: String,
    pub protocols: Vec<String>,
    pub rate_limit: Option<u64>,
    pub requires_auth: bool,
}

/// Capability registry
pub struct CapabilityRegistry {
    capabilities: RwLock<HashMap<String, (Box<dyn Any + Send + Sync>, CapabilityMetadata)>>,
    advertisement: RwLock<CapabilityAdvertisement>,
}

impl CapabilityRegistry {
    pub fn new(primal_id: String) -> Self { /* ... */ }
    
    pub fn register<T: Send + Sync + 'static>(
        &self,
        capability_id: &str,
        provider: T,
        metadata: CapabilityMetadata
    ) { /* ... */ }
    
    pub async fn advertise(&self) -> Result<()> { /* ... */ }
    
    pub fn get_advertisement(&self) -> CapabilityAdvertisement { /* ... */ }
}
```

#### **1.4 Discovery Protocol** (2 hours)
Create `crates/beardog-capabilities/src/discovery.rs`:

```rust
/// Capability discovery
pub struct CapabilityDiscovery {
    discovered: RwLock<HashMap<String, CapabilityEndpoint>>,
}

impl CapabilityDiscovery {
    pub async fn discover_via_mdns(&mut self) -> Result<Vec<CapabilityMetadata>> { /* ... */ }
    
    pub async fn discover_via_http(&mut self, url: &str) -> Result<Vec<CapabilityMetadata>> { /* ... */ }
    
    pub async fn find_capability(&self, capability_id: &str) -> Result<CapabilityEndpoint> { /* ... */ }
    
    pub async fn find_all_capabilities(&self, capability_id: &str) -> Result<Vec<CapabilityEndpoint>> { /* ... */ }
}
```

---

## 📋 **Phase 2: Migrate BTSP to Generic Capabilities** (8 hours)

### **Tasks**

#### **2.1 Rename BTSP Types** (2 hours)
Update `crates/beardog-tunnel/src/`:

```rust
// OLD (btsp_provider.rs)
pub struct PeerInfo { ... }
pub struct TunnelHandle { ... }
pub trait BtspProvider { ... }

// NEW (secure_tunnel_provider.rs)
pub use beardog_capabilities::traits::SecureTunnelProvider;
pub struct PeerEndpoint { ... }  // More generic name
pub struct TunnelHandle { ... }  // Keep same

// Implement generic trait
impl SecureTunnelProvider for BeardogSecureTunnelProvider {
    // Same implementation, new trait
}
```

#### **2.2 Update API Server** (2 hours)
Update `crates/beardog-tunnel/src/api_server.rs`:

```rust
// OLD
pub struct BtspApiServer {
    provider: Arc<dyn BtspProvider>,
    // ...
}

// NEW
pub struct CapabilityApiServer {
    secure_tunnel: Arc<dyn SecureTunnelProvider>,
    registry: Arc<CapabilityRegistry>,
    // ...
}

// Add capability advertisement endpoint
async fn get_capabilities() -> Json<CapabilityAdvertisement> {
    // Return all advertised capabilities
}
```

#### **2.3 Migrate BirdSong Capabilities** (3 hours)
Update `crates/beardog-genetics/src/birdsong/manager.rs`:

```rust
// Implement generic capability traits
impl LineageSigningProvider for BirdSongManager {
    async fn generate_lineage(&self, node_id: &str, parent_id: Option<&str>) -> Result<LineageChain> {
        // Existing implementation
    }
    // ...
}

impl BroadcastEncryptionProvider for BirdSongManager {
    async fn encrypt_for_lineage(&self, payload: &[u8], hint: LineageHint) -> Result<EncryptedBroadcast> {
        // Existing implementation
    }
    // ...
}

impl KeyDerivationProvider for BirdSongManager {
    async fn derive_key(&self, context: &[u8], info: &[u8], length: usize) -> Result<Zeroizing<Vec<u8>>> {
        // Existing implementation
    }
    // ...
}
```

#### **2.4 Update Tests** (1 hour)
Update all tests to use generic traits:

```rust
// OLD
let provider: Arc<dyn BtspProvider> = ...;

// NEW
let provider: Arc<dyn SecureTunnelProvider> = ...;
```

---

## 📋 **Phase 3: Implement Discovery** (8 hours)

### **Tasks**

#### **3.1 mDNS Advertisement** (3 hours)
Implement mDNS broadcasting:

```rust
// crates/beardog-capabilities/src/mdns.rs
use mdns_sd::{ServiceDaemon, ServiceInfo};

pub async fn advertise_capabilities(
    registry: &CapabilityRegistry
) -> Result<()> {
    let mdns = ServiceDaemon::new()?;
    
    for cap in registry.get_capabilities() {
        let service_type = format!("_{}._tcp.local.", cap.id);
        let instance_name = format!("beardog-{}", uuid::Uuid::new_v4());
        
        let service = ServiceInfo::new(
            &service_type,
            &instance_name,
            "localhost",
            "",
            cap.endpoint.port,
            Some(cap.to_txt_records()),
        )?;
        
        mdns.register(service)?;
    }
    
    Ok(())
}

pub async fn discover_capabilities() -> Result<Vec<CapabilityMetadata>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse("_capabilities._tcp.local.")?;
    
    let mut capabilities = Vec::new();
    while let Ok(event) = receiver.recv_timeout(Duration::from_secs(5)) {
        if let ServiceEvent::ServiceResolved(info) = event {
            capabilities.push(parse_service_info(info));
        }
    }
    
    Ok(capabilities)
}
```

#### **3.2 HTTP Advertisement** (2 hours)
Implement HTTP endpoint for capability discovery:

```rust
// Add to API server
async fn get_capabilities(
    State(registry): State<Arc<CapabilityRegistry>>
) -> Json<CapabilityAdvertisement> {
    Json(registry.get_advertisement())
}

// Register route
let app = Router::new()
    .route("/capabilities", get(get_capabilities))
    .route("/capabilities/:id", get(get_capability_details))
    // ... existing routes
```

#### **3.3 Capability Client/Proxy** (3 hours)
Implement client-side proxy for discovered capabilities:

```rust
// crates/beardog-capabilities/src/client.rs
pub struct CapabilityClient<T: ?Sized> {
    endpoint: CapabilityEndpoint,
    http_client: reqwest::Client,
    _marker: PhantomData<T>,
}

#[async_trait]
impl SecureTunnelProvider for CapabilityClient<dyn SecureTunnelProvider> {
    async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle> {
        // HTTP request to capability endpoint
        let response = self.http_client
            .post(&format!("{}/establish", self.endpoint.url))
            .json(&peer)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    // ... implement other methods via HTTP
}
```

---

## 📋 **Phase 4: Update Documentation & Examples** (4 hours)

### **Tasks**

#### **4.1 Remove Primal Names** (2 hours)
Update all documentation:

```bash
# Find and replace
find . -type f -name "*.rs" -o -name "*.md" | xargs sed -i 's/Songbird/other primals/g'
find . -type f -name "*.rs" -o -name "*.md" | xargs sed -i 's/BTSP/secure tunnel capability/g'
find . -type f -name "*.rs" -o -name "*.md" | xargs sed -i 's/BtspProvider/SecureTunnelProvider/g'
```

Files to update:
- `README.md` - Remove "Songbird" references
- `specs/current/integration/BEARDOG_BTSP_IMPLEMENTATION_HANDOFF.md` - Generalize
- `showcase/BIRDSONG_INTEGRATION_ROADMAP_DEC_21_2025.md` - Use capability language
- All `*.rs` file comments

#### **4.2 Create Discovery Examples** (2 hours)
Create `examples/capability_discovery.rs`:

```rust
//! Demonstrates capability-based discovery
//! 
//! This example shows how any primal can discover and use
//! BearDog's capabilities without knowing about BearDog.

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Discover capabilities
    let mut discovery = CapabilityDiscovery::new();
    discovery.discover_via_mdns().await?;
    
    // 2. Find secure tunnel capability
    let tunnel_provider = discovery
        .find_capability::<dyn SecureTunnelProvider>("secure_tunnel")
        .await?;
    
    println!("Found secure tunnel provider!");
    
    // 3. Use capability (provider identity unknown)
    let peer = PeerEndpoint {
        endpoint: "127.0.0.1:9000".to_string(),
    };
    
    let tunnel = tunnel_provider.establish_tunnel(peer).await?;
    println!("Tunnel established: {:?}", tunnel.id);
    
    Ok(())
}
```

---

## 📋 **Phase 5: Backward Compatibility** (4 hours)

### **Tasks**

#### **5.1 Create Compatibility Layer** (2 hours)
Create `crates/beardog-tunnel/src/compat.rs`:

```rust
//! Backward compatibility for BTSP (deprecated)
//! 
//! This module provides compatibility with the old BTSP interface.
//! **DEPRECATED**: Use `SecureTunnelProvider` instead.
//! Removal planned for: June 2026

#[deprecated(since = "0.10.0", note = "Use SecureTunnelProvider instead")]
pub type BtspProvider = dyn SecureTunnelProvider;

#[deprecated(since = "0.10.0", note = "Use PeerEndpoint instead")]
pub type PeerInfo = PeerEndpoint;

// Type alias for backward compatibility
pub type BeardogBtspProvider = BeardogSecureTunnelProvider;
```

#### **5.2 Update Integration Guide** (2 hours)
Create migration guide for Songbird team:

```markdown
# Migration Guide: BTSP → Capability-Based Discovery

## Old Approach (Deprecated)
```rust
use beardog_tunnel::BtspProvider;
let provider = BeardogBtspProvider::new();
```

## New Approach
```rust
use beardog_capabilities::{CapabilityDiscovery, SecureTunnelProvider};

let discovery = CapabilityDiscovery::new();
let provider = discovery.find_capability("secure_tunnel").await?;
```

## Timeline
- **Now**: Both approaches work
- **March 2026**: Deprecation warnings
- **June 2026**: Old approach removed
```

---

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_capability_registration() {
        let registry = CapabilityRegistry::new("test-primal");
        let provider = MockSecureTunnelProvider::new();
        registry.register("secure_tunnel", provider, metadata);
        assert!(registry.has_capability("secure_tunnel"));
    }
    
    #[tokio::test]
    async fn test_capability_discovery() {
        // Start advertising
        let registry = setup_registry();
        registry.advertise().await.unwrap();
        
        // Discover
        let mut discovery = CapabilityDiscovery::new();
        discovery.discover_via_mdns().await.unwrap();
        
        let cap = discovery.find_capability("secure_tunnel").await.unwrap();
        assert_eq!(cap.id, "secure_tunnel");
    }
}
```

### **Integration Tests**
```rust
#[tokio::test]
async fn test_end_to_end_discovery_and_use() {
    // Provider side (BearDog)
    let registry = CapabilityRegistry::new("beardog-test");
    let provider = BeardogSecureTunnelProvider::new(hsm, genetics).await.unwrap();
    registry.register("secure_tunnel", provider, metadata);
    registry.advertise().await.unwrap();
    
    // Consumer side (any primal)
    let mut discovery = CapabilityDiscovery::new();
    discovery.discover_via_mdns().await.unwrap();
    
    let tunnel_provider = discovery
        .find_capability::<dyn SecureTunnelProvider>("secure_tunnel")
        .await
        .unwrap();
    
    // Use capability
    let tunnel = tunnel_provider.establish_tunnel(peer).await.unwrap();
    assert!(!tunnel.id.is_empty());
}
```

---

## 📊 **Timeline Summary**

| Phase | Tasks | Duration | Status |
|-------|-------|----------|--------|
| 1. Generic Framework | Create capability crate & traits | 8 hours | 🔄 Ready |
| 2. Migrate BTSP | Rename & refactor existing code | 8 hours | 🔄 Ready |
| 3. Discovery | Implement mDNS & HTTP discovery | 8 hours | 🔄 Ready |
| 4. Documentation | Update docs & examples | 4 hours | 🔄 Ready |
| 5. Compatibility | Backward compat layer | 4 hours | 🔄 Ready |
| **TOTAL** | | **32 hours** | **~4 weeks** |

---

## ✅ **Success Criteria**

- [ ] Zero primal names in trait definitions
- [ ] `BtspProvider` → `SecureTunnelProvider` (generic)
- [ ] mDNS discovery functional
- [ ] HTTP `/capabilities` endpoint working
- [ ] Backward compatibility layer complete
- [ ] All tests passing (26 existing + 10 new)
- [ ] Documentation updated (no primal names)
- [ ] Examples demonstrate discovery
- [ ] Migration guide for Songbird team

---

## 🚀 **Next Steps**

1. **Review this plan** with team
2. **Approve architecture evolution**
3. **Execute Phase 1** (8 hours)
4. **Validate Phase 1** before proceeding
5. **Continue through phases**
6. **Final validation** (all criteria met)

---

**Ready to proceed? Let's evolve to true primal sovereignty!** 🐻🔌✨

**Document Version**: 1.0  
**Date**: December 21, 2025  
**Status**: Ready to Execute  
**Estimated**: 4 weeks (32 hours)

