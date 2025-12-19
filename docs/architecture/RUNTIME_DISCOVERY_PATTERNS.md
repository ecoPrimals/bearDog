# 🔍 Runtime Discovery Patterns

**Status**: ✅ Production-Ready  
**Last Updated**: December 17, 2025  
**Architecture**: Zero-Hardcoding, Capability-Based Discovery

---

## Core Philosophy

> **Primals only know themselves. They discover others through capability-based queries at runtime.**

BearDog implements a sophisticated runtime discovery system that eliminates hardcoded endpoints, IP addresses, and service locations. All primal-to-primal communication is discovered dynamically based on capabilities.

---

## Discovery Architecture

### 1. Environment-First Discovery

**Priority**: Highest  
**Source**: Environment variables

```rust
// Primary discovery endpoint
std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
    .unwrap_or_else(|_| "https://discovery.ecosystem.internal:PORT")

// Fallback discovery endpoint
std::env::var("FALLBACK_DISCOVERY_ENDPOINT")
    .unwrap_or_else(|_| "http://discovery.ecosystem.internal:PORT")
```

**Why Environment-First?**
- Deployment flexibility (dev, staging, production)
- Zero code changes between environments
- Sovereignty: users control their discovery endpoints
- No hardcoded production values

### 2. Service Mesh Discovery

**Priority**: High  
**Source**: Service mesh APIs (Istio, Linkerd, Consul)

```rust
// Service mesh endpoints discovered via:
// - Consul: http://consul.service.consul:8500/v1/catalog/service/{service}
// - Kubernetes: DNS-based discovery via service.namespace.svc.cluster.local
// - Istio: Envoy sidecar discovery
```

### 3. mDNS/DNS-SD Discovery

**Priority**: Medium  
**Source**: Bonjour/Avahi (local network)

```rust
// Service type: _primal-{capability}._tcp.local.
// Example: _primal-crypto._tcp.local.
```

**Use Cases**:
- Local development
- Edge deployments
- Air-gapped networks
- Zero-config setups

### 4. Well-Known Discovery Domains

**Priority**: Fallback  
**Source**: DNS-resolvable domains (never IPs)

```rust
// Well-known service discovery domains (DNS-resolvable, not IPs)
"http://discovery.ecosystem.internal/api/v1/capabilities"
"http://capability-registry.local/api/v1/capabilities"
```

**Why Domains, Not IPs?**
- DNS provides natural load balancing
- IPs can change, domains are stable
- Supports geo-routing and failover
- Kubernetes/cloud-native compatibility

---

## Discovery Strategies by Component

### Primal Runtime Discovery
**File**: `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs`

**Flow**:
1. Check cache for recent discoveries (performance)
2. Try mDNS for local network (zero-config)
3. Query capability registry endpoints (environment-based)
4. Cache successful discoveries (efficiency)

**Key Methods**:
- `discover_by_capability()` - Main entry point
- `discover_via_mdns()` - Local network discovery
- `discover_via_capability_query()` - Registry-based discovery
- `get_registry_endpoints()` - Environment-based endpoint resolution

### Universal Capability Discovery
**File**: `crates/beardog-adapters/src/universal/capability_discovery/`

**Strategies**:
1. **EnvironmentDiscoveryStrategy** - ENV variables (highest priority)
2. **ServiceMeshDiscoveryStrategy** - Istio, Consul, K8s
3. **CloudVendorDiscoveryStrategy** - AWS, GCP, Azure service discovery
4. **ContainerDiscoveryStrategy** - Docker, Kubernetes
5. **NetworkDiscoveryStrategy** - mDNS, DNS-SD

**Discovery Process**:
```rust
pub async fn discover_capabilities(
    &self,
    capability_type: &CapabilityType,
) -> Result<Vec<UniversalCapability>> {
    // Try all strategies in priority order
    for strategy in &self.discovery_strategies {
        if let Ok(capabilities) = strategy.discover(capability_type).await {
            if !capabilities.is_empty() {
                return Ok(capabilities);
            }
        }
    }
    // Fallback to cached or default
}
```

---

## BearDog HTTP API for Primal Discovery

### Capability Advertisement

**Endpoint**: `GET /api/v1/capabilities`

**Response**:
```json
{
  "capabilities": [
    {
      "id": "crypto-aes-gcm",
      "name": "AES-256-GCM Encryption",
      "category": "encryption",
      "endpoint": "/api/v1/crypto/aes-gcm/encrypt"
    },
    {
      "id": "crypto-ed25519",
      "name": "Ed25519 Signing",
      "category": "signing",
      "endpoint": "/api/v1/crypto/ed25519/sign"
    }
  ]
}
```

**Usage by Other Primals**:
```rust
// Songbird discovers BearDog's crypto capabilities
let response = http_client
    .get("http://beardog.service.local/api/v1/capabilities")
    .send()
    .await?;

let capabilities: CapabilitiesResponse = response.json().await?;

// Use discovered endpoints, never hardcoded
for cap in capabilities.capabilities {
    if cap.category == "encryption" {
        // Use cap.endpoint for operations
    }
}
```

---

## Network Configuration

### Address Configuration
**File**: `crates/beardog-config/src/domains/network_addresses.rs`

All network addresses are configurable via environment:

```rust
pub struct NetworkAddressesConfig {
    /// API server hostname (BEARDOG_API_HOST)
    pub api_host: String,
    
    /// Server bind address (BEARDOG_BIND_ADDRESS)
    pub bind_address: String,
    
    /// External/public hostname (BEARDOG_EXTERNAL_HOST)
    pub external_host: String,
    
    /// Multicast address (BEARDOG_MULTICAST_ADDRESS)
    pub multicast_address: String,
}
```

**Defaults**:
- Development: `127.0.0.1` (localhost only, secure by default)
- Production: Set via environment variables
- No hardcoded production IPs anywhere

### Service Discovery Configuration
**File**: `crates/beardog-config/src/domains/network.rs`

```rust
pub struct ServiceDiscoveryConfig {
    /// Discovery port (BEARDOG_DISCOVERY_PORT)
    pub port: u16,
    
    /// Discovery backends (dns-sd, static, consul, etc.)
    pub backends: Vec<String>,
    
    /// Multicast address for mDNS (BEARDOG_MULTICAST_ADDRESS)
    pub multicast_address: String,
    
    /// Discovery interval in seconds (BEARDOG_DISCOVERY_INTERVAL_SECS)
    pub interval_secs: u64,
}
```

---

## Songbird Integration Example

### How Songbird Discovers BearDog

**Step 1: Environment-Based Discovery**
```bash
# Songbird looks for BearDog via environment
export BEARDOG_DISCOVERY_ENDPOINT="http://beardog.namespace.svc.cluster.local:8080"
```

**Step 2: Capability Query**
```rust
// Songbird queries BearDog's capabilities
let capabilities = discover_service_capabilities("crypto").await?;
```

**Step 3: Dynamic Endpoint Usage**
```rust
// Songbird uses discovered encrypt endpoint
let encrypt_endpoint = capabilities
    .find(|c| c.id == "crypto-generic-encrypt")
    .endpoint;

// POST to discovered endpoint (never hardcoded)
let encrypted = http_client
    .post(encrypt_endpoint)
    .json(&encrypt_request)
    .send()
    .await?;
```

---

## Benefits of Runtime Discovery

### 1. **Zero Hardcoding**
- No IP addresses in source code
- No ports in production code
- No service URLs hardcoded

### 2. **Deployment Flexibility**
- Same binary for dev, staging, production
- Environment-specific configuration
- Cloud-agnostic deployment

### 3. **Sovereignty & Privacy**
- Users control discovery endpoints
- No "phone home" to hardcoded servers
- Air-gap friendly

### 4. **Resilience**
- Automatic failover to fallback endpoints
- Service mesh integration
- Load balancing via DNS

### 5. **Zero-Configuration**
- mDNS for local development
- Auto-discovery in container environments
- Kubernetes-native service discovery

---

## Testing Discovery

### Unit Tests
```rust
#[test]
fn test_environment_discovery_priority() {
    std::env::set_var("BEARDOG_DISCOVERY_ENDPOINT", "http://test.local");
    let endpoints = get_registry_endpoints();
    assert_eq!(endpoints[0], "http://test.local/api/v1/capabilities");
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_capability_discovery() {
    let discovery = RuntimePrimalDiscovery::new(5000, 60000);
    let services = discovery
        .discover_by_capability(vec![UniversalCapabilityType::Crypto])
        .await?;
    assert!(!services.is_empty());
}
```

---

## Migration Guide: Hardcoded → Runtime Discovery

### Before (Hardcoded)
```rust
// ❌ BAD: Hardcoded endpoint
let response = http_client
    .post("http://localhost:8080/api/encrypt")
    .send()
    .await?;
```

### After (Runtime Discovery)
```rust
// ✅ GOOD: Environment-based discovery
let discovery_endpoint = std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
    .unwrap_or_else(|_| "http://beardog.service.local".to_string());

let capabilities = query_capabilities(&discovery_endpoint).await?;
let encrypt_endpoint = capabilities
    .find_capability("encrypt")
    .endpoint;

let response = http_client
    .post(encrypt_endpoint)
    .send()
    .await?;
```

---

## Configuration Examples

### Development
```bash
# Local development (mDNS auto-discovery)
# No configuration needed!
cargo run
```

### Kubernetes
```yaml
# ConfigMap for BearDog discovery
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-discovery
data:
  BEARDOG_DISCOVERY_ENDPOINT: "http://beardog-api.beardog.svc.cluster.local:8080"
  FALLBACK_DISCOVERY_ENDPOINT: "http://beardog-api-fallback.beardog.svc.cluster.local:8080"
```

### Production (Cloud)
```bash
# AWS ECS/EKS
export BEARDOG_DISCOVERY_ENDPOINT="https://beardog.production.internal"
export BEARDOG_API_HOST="0.0.0.0"  # Bind to all interfaces
export BEARDOG_EXTERNAL_HOST="beardog.production.example.com"
```

---

## Future Enhancements

### Planned
- [ ] Distributed hash table (DHT) for peer discovery
- [ ] Blockchain-based service registry (for sovereignty)
- [ ] WebRTC for direct peer-to-peer discovery
- [ ] IPFS/libp2p integration for decentralized discovery

### Under Consideration
- [ ] Zero-knowledge proofs for capability verification
- [ ] Tor hidden service discovery
- [ ] I2P eepsite discovery

---

## Summary

BearDog's runtime discovery system embodies the core principle: **Primals only know themselves**.

✅ **Zero hardcoded endpoints**  
✅ **Environment-first configuration**  
✅ **Multiple discovery strategies**  
✅ **Sovereignty-preserving**  
✅ **Cloud-agnostic**  
✅ **Zero-config capable**  

All service discovery is dynamic, capability-based, and respects user sovereignty. No primal ever needs to know the address of another primal at compile time.

