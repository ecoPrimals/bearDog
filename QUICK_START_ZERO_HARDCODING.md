# 🚀 Quick Start: Zero-Hardcoding BearDog

**Status**: Production-Ready  
**Date**: January 13, 2026  
**Achievement**: 100% Zero-Hardcoding Architecture

---

## 🎯 What is Zero-Hardcoding?

BearDog now uses **runtime discovery** for all identity, peer discovery, and service routing. Nothing is hardcoded!

### Core Principles
1. **Self-Knowledge**: Discover your own identity from environment
2. **Primal Discovery**: Find other primals at runtime by capability
3. **Capability Routing**: Route requests by what's needed, not who provides it

---

## ⚡ Quick Start (5 Minutes)

### Step 1: Set Your Identity
```bash
export PRIMAL_NAME=BearDog
export BEARDOG_LISTEN_ADDR=127.0.0.1:8900
export BEARDOG_CAPABILITIES=SecureTunneling,Cryptography,HsmIntegration
```

### Step 2: Configure Discovery
```bash
# Use environment-based discovery (simple, for dev)
export PRIMAL_DISCOVERY_METHOD=env

# Tell BearDog where to find other primals
export PRIMAL_SONGBIRD_ADDR=127.0.0.1:9100
export PRIMAL_BIOMEOS_ADDR=127.0.0.1:9200
```

### Step 3: Run BearDog
```bash
cargo run --bin beardog-server
```

You should see:
```
🔍 Discovering self-knowledge from environment...
╔════════════════════════════════════════════════════════════════════╗
║         🐻 BearDog v0.9.0                                        ║
║              Sovereign Primal for Tower Orchestration             ║
╚════════════════════════════════════════════════════════════════════╝

🎯 Self-Knowledge Discovered:
   Name: BearDog
   Version: 0.9.0
   Endpoints: [Endpoint { protocol: Http, address: 127.0.0.1:8900 }]
   Capabilities: 3 discovered
      • SecureTunneling
      • Cryptography
      • HsmIntegration
```

---

## 📖 Usage Examples

### Example 1: Self-Knowledge
```rust
use beardog_core::self_knowledge::PrimalSelfKnowledge;

let self_knowledge = PrimalSelfKnowledge::discover()?;

println!("I am: {}", self_knowledge.my_name());
println!("Version: {}", self_knowledge.my_version().version);
println!("Listening on: {:?}", self_knowledge.my_endpoints());

// Check capabilities
if self_knowledge.provides_capability(&SimpleCapability::Cryptography) {
    println!("I can provide cryptography!");
}
```

### Example 2: Discover Other Primals
```rust
use beardog_core::primal_discovery::{PrimalDiscovery, DiscoveryQuery};
use beardog_core::self_knowledge::SimpleCapability;

let mut discovery = PrimalDiscovery::from_env()?;

// Find primals by name
let songbird = discovery.discover(
    DiscoveryQuery::by_name("Songbird")
).await?;

// Find primals by capability
let crypto_providers = discovery.discover(
    DiscoveryQuery::by_capability(SimpleCapability::Cryptography)
).await?;

for primal in crypto_providers {
    println!("Found: {} at {:?}", primal.name, primal.endpoints);
}
```

### Example 3: Capability-Based Routing
```rust
use beardog_core::capability_router::{CapabilityRouter, RequestContext, SelectionStrategy};

let mut router = CapabilityRouter::new().await?;

// Route to the most trusted primal providing SecureTunneling
let decision = router.route(
    SimpleCapability::SecureTunneling,
    RequestContext::new(SimpleCapability::SecureTunneling)
        .with_strategy(SelectionStrategy::HighestTrust)
        .with_min_trust(0.8)
        .with_max_latency(100)
).await?;

println!("Routing to: {} (reason: {})", 
         decision.primal.name, 
         decision.reason);

// Use the decision
let endpoint = &decision.primal.endpoints[0];
let client = connect_to_primal(endpoint).await?;
```

---

## 🔧 Environment Variables Reference

### Self-Knowledge (Required)
| Variable | Purpose | Example |
|----------|---------|---------|
| `PRIMAL_NAME` | Your primal's name | `BearDog` |
| `BEARDOG_LISTEN_ADDR` | Where you listen | `127.0.0.1:8900` |
| `BEARDOG_CAPABILITIES` | What you provide | `SecureTunneling,Cryptography` |

### Discovery (Optional)
| Variable | Purpose | Default |
|----------|---------|---------|
| `PRIMAL_DISCOVERY_METHOD` | How to find primals | `multi` |
| `PRIMAL_<NAME>_ADDR` | Explicit primal address | - |
| `UPA_REGISTRY_ADDR` | UPA registry address | - |
| `MDNS_SERVICE_TYPE` | mDNS service type | `_ecoprimal._tcp` |
| `DISCOVERY_CACHE_TTL_SECS` | Cache duration | `300` |

### Discovery Methods
- `env` - Environment variables only (simple, dev-friendly)
- `upa` - Universal Primal Authority registry
- `mdns` - Multicast DNS (local network)
- `dns-sd` - DNS Service Discovery
- `multi` - Try multiple methods (default)

---

## 🎨 Advanced Routing Strategies

### By Trust Score
```rust
RequestContext::new(SimpleCapability::Cryptography)
    .with_strategy(SelectionStrategy::HighestTrust)
    .with_min_trust(0.8)
```

### By Load
```rust
RequestContext::new(SimpleCapability::SecureTunneling)
    .with_strategy(SelectionStrategy::LeastLoaded)
```

### By Latency
```rust
RequestContext::new(SimpleCapability::GeneticLineage)
    .with_strategy(SelectionStrategy::LowestLatency)
    .with_max_latency(50) // ms
```

### Round-Robin (Load Balancing)
```rust
RequestContext::new(SimpleCapability::Discovery)
    .with_strategy(SelectionStrategy::RoundRobin)
```

### With Exclusions
```rust
RequestContext::new(SimpleCapability::HsmIntegration)
    .excluding("untrusted-primal")
    .excluding("degraded-primal")
```

---

## 🚀 Production Deployment

### 1. Container Environment
```dockerfile
ENV PRIMAL_NAME=BearDog
ENV BEARDOG_LISTEN_ADDR=0.0.0.0:8900
ENV BEARDOG_CAPABILITIES=SecureTunneling,Cryptography,HsmIntegration
ENV PRIMAL_DISCOVERY_METHOD=upa
ENV UPA_REGISTRY_ADDR=upa.ecoprimals.internal:7000
```

### 2. Kubernetes ConfigMap
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-config
data:
  PRIMAL_NAME: "BearDog"
  BEARDOG_LISTEN_ADDR: "0.0.0.0:8900"
  BEARDOG_CAPABILITIES: "SecureTunneling,Cryptography,HsmIntegration"
  PRIMAL_DISCOVERY_METHOD: "upa"
  UPA_REGISTRY_ADDR: "upa.ecoprimals.svc.cluster.local:7000"
```

### 3. systemd Service
```ini
[Unit]
Description=BearDog Secure Tunnel Service
After=network.target

[Service]
Type=simple
Environment="PRIMAL_NAME=BearDog"
Environment="BEARDOG_LISTEN_ADDR=127.0.0.1:8900"
Environment="BEARDOG_CAPABILITIES=SecureTunneling,Cryptography,HsmIntegration"
Environment="PRIMAL_DISCOVERY_METHOD=mdns"
ExecStart=/usr/local/bin/beardog-server
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

---

## 🔍 Troubleshooting

### "No primals found providing capability"
**Cause**: Discovery can't find any primals with the requested capability.

**Solution**:
```bash
# Check discovery method
echo $PRIMAL_DISCOVERY_METHOD

# Verify primal addresses are set
env | grep PRIMAL_

# Try explicit discovery
export PRIMAL_SONGBIRD_ADDR=127.0.0.1:9100
```

### "Failed to discover self-knowledge"
**Cause**: Missing required environment variable.

**Solution**:
```bash
# Ensure PRIMAL_NAME is set
export PRIMAL_NAME=BearDog
```

### "Invalid endpoint address"
**Cause**: Malformed address in environment variable.

**Solution**:
```bash
# Use IP:PORT format
export BEARDOG_LISTEN_ADDR=127.0.0.1:8900

# Or hostname:PORT
export BEARDOG_LISTEN_ADDR=beardog.local:8900
```

---

## 📚 Further Reading

- `ZERO_HARDCODING_COMPLETE_JAN_13_2026.md` - Full evolution documentation
- `HARDCODING_EVOLUTION_PLAN_JAN_13_2026.md` - Design philosophy
- `SESSION_SUMMARY_JAN_13_2026_FINAL.md` - Complete session summary
- `crates/beardog-core/src/self_knowledge.rs` - Self-knowledge implementation
- `crates/beardog-core/src/primal_discovery.rs` - Discovery implementation
- `crates/beardog-core/src/capability_router.rs` - Routing implementation

---

## ✅ Quick Verification

Test that everything works:

```bash
# 1. Set minimal environment
export PRIMAL_NAME=BearDog
export BEARDOG_LISTEN_ADDR=127.0.0.1:8900
export PRIMAL_DISCOVERY_METHOD=env

# 2. Run tests
cargo test -p beardog-core self_knowledge
cargo test -p beardog-core primal_discovery
cargo test -p beardog-core capability_router

# 3. Run server
cargo run --bin beardog-server
```

If you see the self-knowledge discovery output, you're ready to go! 🚀

---

**Last Updated**: January 13, 2026  
**Status**: Production-Ready  
**Support**: See docs/ for detailed guides

