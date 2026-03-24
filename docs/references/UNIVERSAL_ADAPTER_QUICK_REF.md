# 🧒 Universal Adapter - Quick Reference

**Module**: `beardog_core::universal_adapter`  
**Pattern**: Infant Discovery (zero initial knowledge)  
**Purpose**: Single interface for all primal-to-primal communication

---

## 🚀 Quick Start (30 seconds)

```rust
use beardog_core::universal_adapter::UniversalAdapter;
use beardog_core::self_knowledge::SimpleCapability;

// Create adapter (zero initial knowledge)
let adapter = UniversalAdapter::new().await?;

// Find ANY primal with AI capability
let ai_primal = adapter
    .find_primal_by_capability(SimpleCapability::Discovery)
    .await?;

println!("Using: {}", ai_primal.name);
// Prints: "Squirrel" (or whoever provides AI)
```

---

## 📖 Common Patterns

### 1. Discover All Providers
```rust
// Find ALL primals providing a capability
let crypto_providers = adapter
    .discover_capability(SimpleCapability::Cryptography)
    .await?;

for primal in crypto_providers {
    println!("{} at {:?}", primal.name, primal.endpoints);
}
```

### 2. Get Best Provider
```rust
// Get BEST provider (by trust, load, or latency)
let best_storage = adapter
    .find_primal_by_capability(SimpleCapability::SecureTunneling)
    .await?;
```

### 3. Check Who I Am
```rust
// Access self-knowledge
let me = adapter.self_knowledge();
println!("I am: {} v{}", 
    me.my_name(), 
    me.my_version().version
);
```

### 4. Check Cached Capabilities
```rust
// Check what's in cache (non-blocking)
if adapter.has_capability(&SimpleCapability::Discovery) {
    let count = adapter.count_capability_providers(&SimpleCapability::Discovery);
    println!("Know about {} AI providers", count);
}
```

### 5. Clear Cache
```rust
// Force re-discovery
adapter.clear_cache();

// Or clear specific capability
adapter.clear_capability_cache(&SimpleCapability::Discovery);
```

---

## 🔧 Environment Variables

### Required
```bash
export PRIMAL_NAME=BearDog              # Who am I?
export BEARDOG_LISTEN_ADDR=127.0.0.1:8900  # Where do I listen?
export BEARDOG_CAPABILITIES=SecureTunneling,Cryptography  # What do I provide?
```

### Optional: Discovery Method
```bash
export PRIMAL_DISCOVERY_METHOD=env      # env, upa, mdns, dns-sd, multi (default)
```

### Optional: Other Primals (for env discovery)
```bash
export PRIMAL_SONGBIRD_ADDR=127.0.0.1:9100
export PRIMAL_SQUIRREL_ADDR=127.0.0.1:9300
export PRIMAL_TOADSTOOL_ADDR=127.0.0.1:9400
```

### Optional: Cache Settings
```bash
export UNIVERSAL_ADAPTER_CACHE_TTL_SECS=300  # Default: 5 minutes
```

---

## 🎯 Migration: Before → After

### Old Way (Hardcoded)
```rust
// ❌ Hardcoded primal names and addresses
let songbird = SongbirdClient::connect("songbird.local:9100").await?;
let services = songbird.discover_service("database").await?;

let squirrel = SquirrelClient::connect("squirrel.local:9300").await?;
let result = squirrel.analyze_sentiment(text).await?;
```

### New Way (Infant Discovery)
```rust
// ✅ Zero hardcoded knowledge
let adapter = UniversalAdapter::new().await?;

// Find discovery provider (don't know/care who)
let discovery_primal = adapter
    .find_primal_by_capability(SimpleCapability::Discovery)
    .await?;

// Find AI provider (don't know/care who)
let ai_primal = adapter
    .find_primal_by_capability(SimpleCapability::Discovery)
    .await?;

// BearDog discovered Songbird and Squirrel automatically!
```

---

## 💡 Real-World Examples

### Example 1: Service Mesh Discovery
```rust
// Need to find a database, but don't know who provides service discovery
let adapter = UniversalAdapter::new().await?;

let mesh = adapter
    .find_primal_by_capability(SimpleCapability::ServiceMesh)
    .await?;

// mesh could be Songbird, or any future service mesh primal
```

### Example 2: AI Analysis Pipeline
```rust
// Complex workflow: storage → compute → AI
let adapter = UniversalAdapter::new().await?;

// 1. Get data from storage (whoever provides it)
let storage = adapter.find_primal_by_capability(SimpleCapability::Storage).await?;

// 2. Run on compute (whoever has capacity)
let compute = adapter.find_primal_by_capability(SimpleCapability::Compute).await?;

// 3. Analyze with AI (whoever provides best model)
let ai = adapter.find_primal_by_capability(SimpleCapability::AI).await?;

// All discovered dynamically - no hardcoded names!
```

### Example 3: Multi-Provider Failover
```rust
// Get ALL crypto providers for redundancy
let crypto_providers = adapter
    .discover_capability(SimpleCapability::Cryptography)
    .await?;

for provider in crypto_providers {
    match provider.sign(data).await {
        Ok(signature) => return Ok(signature),
        Err(e) => {
            warn!("Provider {} failed: {}", provider.name, e);
            continue; // Try next provider
        }
    }
}
```

---

## 🎓 API Reference

### UniversalAdapter

#### Constructor
```rust
pub async fn new() -> Result<Self, BearDogError>
```
Create adapter with zero initial knowledge.

#### Discovery Methods
```rust
// Find ALL primals providing capability
pub async fn discover_capability(
    &self,
    capability: SimpleCapability,
) -> Result<Vec<DiscoveredPrimal>, BearDogError>

// Find BEST primal providing capability
pub async fn find_primal_by_capability(
    &self,
    capability: SimpleCapability,
) -> Result<DiscoveredPrimal, BearDogError>
```

#### Cache Methods
```rust
// Check cache (non-blocking)
pub fn has_capability(&self, capability: &SimpleCapability) -> bool

// Count providers in cache
pub fn count_capability_providers(&self, capability: &SimpleCapability) -> usize

// Get all cached capabilities
pub fn cached_capabilities(&self) -> Vec<SimpleCapability>

// Clear cache
pub fn clear_cache(&self)
pub fn clear_capability_cache(&self, capability: &SimpleCapability)
```

#### Self-Knowledge
```rust
// Access self-knowledge
pub fn self_knowledge(&self) -> &PrimalSelfKnowledge
```

---

## 🐛 Troubleshooting

### "Failed to discover self-knowledge"
```bash
# Missing required environment variable
export PRIMAL_NAME=BearDog
```

### "No primals found providing capability"
```bash
# No primals discovered - check discovery method
export PRIMAL_DISCOVERY_METHOD=env
export PRIMAL_SONGBIRD_ADDR=127.0.0.1:9100
```

### "Cache is stale"
```rust
// Force re-discovery
adapter.clear_capability_cache(&SimpleCapability::Discovery);
let fresh = adapter.discover_capability(SimpleCapability::Discovery).await?;
```

---

## 📊 Capabilities Reference

```rust
pub enum SimpleCapability {
    SecureTunneling,     // BTSP, VPN, encrypted channels
    GeneticLineage,      // Identity, family trees
    Cryptography,        // Signing, encryption, KMS
    HsmIntegration,      // Hardware security modules
    Discovery,           // Service discovery, mDNS
}
```

**Extend as needed** - add your own capabilities!

---

## ✅ Best Practices

### DO ✅
- **Use capabilities** instead of primal names
- **Cache wisely** - balance freshness vs performance
- **Handle failures** - primals may go offline
- **Log discoveries** - helps debugging
- **Test with mocks** - use test fixtures

### DON'T ❌
- **Hardcode primal names** - defeats the purpose
- **Assume specific primals** - use capabilities
- **Ignore errors** - discovery can fail
- **Cache forever** - primals change
- **Skip environment setup** - required for self-knowledge

---

## 🚀 Quick Test

```bash
# Terminal 1: Start BearDog
export PRIMAL_NAME=BearDog
export BEARDOG_LISTEN_ADDR=127.0.0.1:8900
export BEARDOG_CAPABILITIES=SecureTunneling,Cryptography
cargo run --bin beardog -- server

# Terminal 2: Test discovery
export PRIMAL_NAME=TestClient
export PRIMAL_DISCOVERY_METHOD=env
export PRIMAL_BEARDOG_ADDR=127.0.0.1:8900
cargo test -p beardog-core universal_adapter
```

---

## 📚 Related Documentation

- `INFANT_DISCOVERY_COMPLETE.md` - Complete architecture guide
- `INFANT_DISCOVERY_EVOLUTION_PLAN.md` - Migration strategy
- `QUICK_START_ZERO_HARDCODING.md` - Zero-hardcoding guide
- `crates/beardog-core/src/universal_adapter.rs` - Source code & examples

---

**Quick Reference Version**: 1.0  
**Last Updated**: January 13, 2026  
**Status**: Production-ready

**Need help?** See full documentation in `INFANT_DISCOVERY_COMPLETE.md`

