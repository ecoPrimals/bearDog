# 🎯 Hardcoding Evolution Plan - January 13, 2026

**Philosophy**: Primal Self-Knowledge + Runtime Discovery  
**Goal**: Zero hardcoded primal names, ports, or topology assumptions  
**Status**: 🚀 **IN PROGRESS**

---

## 🎯 CORE PRINCIPLES

### 1. **Primal Self-Knowledge**
```rust
// ❌ Before: External knowledge
const SONGBIRD_PORT: u16 = 4200;
const BIRDSONG_URL: &str = "http://localhost:4200";

// ✅ After: Self-knowledge only
pub struct PrimalIdentity {
    my_name: String,              // From env/config
    my_capabilities: Vec<Capability>,  // What I can do
    my_endpoints: Vec<Endpoint>,  // Where I listen (OS-assigned)
}
```

### 2. **Runtime Discovery**
```rust
// ❌ Before: Hardcoded primal locations
let songbird = connect_to("localhost:4200");

// ✅ After: Capability-based discovery
let discovery = CapabilityDiscovery::new();
let discoverer = discovery
    .find_capability(Capability::Discovery)
    .await?;
```

### 3. **Environment-Driven Configuration**
```rust
// ❌ Before: Compile-time constants
const TIMEOUT_MS: u64 = 5000;

// ✅ After: Runtime configuration
let timeout = config
    .get_timeout()
    .or_env("BEARDOG_TIMEOUT_MS")
    .or_default(Duration::from_secs(5));
```

---

## 📊 CURRENT STATE

**Total Hardcoded Values**: ~500

### Breakdown by Category

| Category | Count | Location | Priority |
|----------|-------|----------|----------|
| **Network (IPs)** | 538 | Mostly tests | 🟡 MEDIUM |
| **Ports** | 298 | Config defaults | 🟡 MEDIUM |
| **Primal Names** | Minimal | Tests/docs | 🟢 LOW |
| **Timeouts** | ~50 | Constants | 🟢 LOW |
| **Test Fixtures** | ~400 | Tests | ✅ ACCEPTABLE |

### Key Insight
**80% of hardcoding is in tests** - This is ACCEPTABLE! ✅

**Production hardcoding**: ~100 values (config defaults, fallbacks)

---

## 🎯 EVOLUTION STRATEGY

### Phase 1: Primal Self-Knowledge Pattern (Week 1-2)

#### Step 1: Create Self-Knowledge Module

**File**: `crates/beardog-core/src/self_knowledge.rs`

```rust
//! Primal Self-Knowledge Module
//!
//! Implements the principle: "Primals only know themselves"

use std::net::SocketAddr;
use std::env;
use beardog_capabilities::Capability;

/// Primal's knowledge about itself
///
/// Discovered at runtime from:
/// - Environment variables
/// - Configuration files  
/// - OS introspection
/// - Capability registration
pub struct PrimalSelfKnowledge {
    /// My identity (from env or config, never hardcoded)
    identity: PrimalIdentity,
    
    /// My capabilities (what I can do)
    capabilities: Vec<Capability>,
    
    /// My endpoints (where I'm listening)
    endpoints: Vec<Endpoint>,
    
    /// My configuration
    config: RuntimeConfig,
}

impl PrimalSelfKnowledge {
    /// Discover self-knowledge at runtime
    pub fn discover() -> Result<Self, BearDogError> {
        Ok(Self {
            identity: PrimalIdentity::discover()?,
            capabilities: discover_my_capabilities(),
            endpoints: discover_my_endpoints()?,
            config: RuntimeConfig::load()?,
        })
    }
    
    /// Get my name (never assume it!)
    pub fn my_name(&self) -> &str {
        &self.identity.name
    }
    
    /// Get my capabilities
    pub fn my_capabilities(&self) -> &[Capability] {
        &self.capabilities
    }
    
    /// Get my listening endpoints
    pub fn my_endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }
}

struct PrimalIdentity {
    name: String,  // From PRIMAL_NAME env or config
    version: String,
}

impl PrimalIdentity {
    fn discover() -> Result<Self, BearDogError> {
        Ok(Self {
            name: env::var("PRIMAL_NAME")
                .or_else(|_| read_from_config())
                .unwrap_or_else(|_| "beardog".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

struct Endpoint {
    protocol: Protocol,
    address: SocketAddr,  // OS-assigned or from config
}

fn discover_my_endpoints() -> Result<Vec<Endpoint>, BearDogError> {
    // Read from config or let OS assign
    let config_endpoints = read_endpoints_from_config()?;
    
    if !config_endpoints.is_empty() {
        return Ok(config_endpoints);
    }
    
    // No config? Let OS assign port 0 (random available)
    Ok(vec![Endpoint {
        protocol: Protocol::Http,
        address: "127.0.0.1:0".parse()?,  // Port 0 = OS assigns
    }])
}

fn discover_my_capabilities() -> Vec<Capability> {
    // Introspect what capabilities we actually implement
    vec![
        Capability::SecureTunnel,
        Capability::GeneticLineage,
        Capability::CryptographicOperations,
    ]
}
```

#### Step 2: Update BearDog Server

**File**: `crates/beardog-tunnel/src/bin/beardog-server.rs`

```rust
// ❌ Before
const DEFAULT_PORT: u16 = 8080;
let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT));

// ✅ After
let self_knowledge = PrimalSelfKnowledge::discover()?;
let endpoints = self_knowledge.my_endpoints();
let addr = endpoints[0].address;  // OS-assigned or from config

info!("BearDog '{}' starting on {}", 
    self_knowledge.my_name(), 
    addr
);
```

---

### Phase 2: Runtime Primal Discovery (Week 2-3)

#### Step 1: Create Discovery Module

**File**: `crates/beardog-core/src/runtime_discovery.rs`

```rust
//! Runtime Primal Discovery
//!
//! Discovers other primals at runtime without hardcoding

use beardog_capabilities::Capability;
use std::time::Duration;

pub struct PrimalDiscovery {
    discovery_timeout: Duration,
}

impl PrimalDiscovery {
    pub fn new() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(5),
        }
    }
    
    /// Find any primal that provides a capability
    ///
    /// NO HARDCODED NAMES! Discovers at runtime via:
    /// - mDNS (local network)
    /// - BirdSong (encrypted UDP multicast)  
    /// - Configuration (if explicitly configured)
    pub async fn find_capability(
        &self,
        capability: Capability,
    ) -> Result<PrimalConnection, BearDogError> {
        // Try multiple discovery methods
        
        // 1. Check explicit configuration first
        if let Ok(conn) = self.find_in_config(capability).await {
            return Ok(conn);
        }
        
        // 2. Try mDNS discovery (local network)
        if let Ok(conn) = self.discover_via_mdns(capability).await {
            return Ok(conn);
        }
        
        // 3. Try BirdSong (encrypted multicast)
        if let Ok(conn) = self.discover_via_birdsong(capability).await {
            return Ok(conn);
        }
        
        Err(BearDogError::capability_not_found(capability))
    }
    
    async fn discover_via_mdns(&self, capability: Capability) -> Result<PrimalConnection, BearDogError> {
        // Use mDNS to find services advertising this capability
        let service_type = format!("_beardog-{:?}._tcp", capability);
        let discovered = mdns_discover(&service_type, self.discovery_timeout).await?;
        
        Ok(PrimalConnection {
            endpoint: discovered.endpoint,
            capabilities: discovered.capabilities,
            discovered_via: DiscoveryMethod::Mdns,
        })
    }
    
    async fn discover_via_birdsong(&self, capability: Capability) -> Result<PrimalConnection, BearDogError> {
        // Listen for BirdSong broadcasts advertising this capability
        let birdsong = BirdSongListener::new().await?;
        let announcement = birdsong
            .find_capability(capability, self.discovery_timeout)
            .await?;
            
        Ok(PrimalConnection {
            endpoint: announcement.endpoint,
            capabilities: announcement.capabilities,
            discovered_via: DiscoveryMethod::BirdSong,
        })
    }
}

pub struct PrimalConnection {
    endpoint: String,
    capabilities: Vec<Capability>,
    discovered_via: DiscoveryMethod,
}

enum DiscoveryMethod {
    Config,      // Explicitly configured
    Mdns,        // Local mDNS
    BirdSong,    // Encrypted multicast
}
```

#### Step 2: Update Client Code

```rust
// ❌ Before: Hardcoded locations
let songbird_url = "http://localhost:4200";
let songbird = SongbirdClient::new(songbird_url);

// ✅ After: Runtime discovery
let discovery = PrimalDiscovery::new();
let discoverer = discovery
    .find_capability(Capability::Discovery)
    .await?;
    
// Use the discovered endpoint (could be ANY primal with Discovery capability!)
let client = CapabilityClient::new(discoverer.endpoint);
```

---

### Phase 3: Configuration Evolution (Week 3-4)

#### Environment Variable Pattern

```rust
// Helper for environment-driven config
pub trait ConfigValue: Sized {
    fn from_env(key: &str) -> Option<Self>;
    fn default_value() -> Self;
}

pub struct ConfigBuilder<T> {
    env_key: Option<String>,
    config_value: Option<T>,
    default: T,
}

impl<T: ConfigValue> ConfigBuilder<T> {
    pub fn new(default: T) -> Self {
        Self {
            env_key: None,
            config_value: None,
            default,
        }
    }
    
    pub fn or_env(mut self, key: &str) -> Self {
        self.env_key = Some(key.to_string());
        self
    }
    
    pub fn or_config(mut self, value: Option<T>) -> Self {
        self.config_value = value;
        self
    }
    
    pub fn build(self) -> T {
        // Priority: env > config > default
        if let Some(key) = &self.env_key {
            if let Some(val) = T::from_env(key) {
                return val;
            }
        }
        
        self.config_value.unwrap_or(self.default)
    }
}

// Usage
let timeout = ConfigBuilder::new(Duration::from_secs(5))
    .or_env("BEARDOG_TIMEOUT_MS")
    .or_config(config.timeout)
    .build();
```

---

## 📋 MIGRATION CHECKLIST

### Week 1: Self-Knowledge Foundation
- [ ] Create `self_knowledge.rs` module
- [ ] Implement `PrimalSelfKnowledge::discover()`
- [ ] Update `beardog-server` to use self-knowledge
- [ ] Add environment variable documentation
- [ ] Test self-discovery on different environments

### Week 2: Runtime Discovery
- [ ] Create `runtime_discovery.rs` module
- [ ] Implement mDNS discovery
- [ ] Implement BirdSong discovery
- [ ] Add capability-based lookup
- [ ] Update client code to use discovery

### Week 3: Configuration Evolution
- [ ] Create `ConfigBuilder` pattern
- [ ] Migrate timeout values
- [ ] Migrate network addresses
- [ ] Add environment variable fallbacks
- [ ] Document configuration hierarchy

### Week 4: Test & Polish
- [ ] Add discovery tests
- [ ] Add self-knowledge tests
- [ ] Update documentation
- [ ] Create migration guide
- [ ] Verify zero hardcoded primals

---

## 🎯 SUCCESS CRITERIA

### Metrics

| Metric | Before | Target | Verification |
|--------|--------|--------|--------------|
| **Production Hardcoding** | ~100 | <10 | Grep search |
| **Primal Names in Code** | Several | 0 | Grep "songbird\|nestgate" |
| **Fixed Ports** | Many | 0 | Grep ":\d{4}" |
| **Discovery Methods** | 1 | 3+ | mDNS, BirdSong, Config |

### Tests

```rust
#[test]
fn test_no_hardcoded_primal_names() {
    // Verify no primal names in production code
    // (tests/docs/examples are OK)
}

#[tokio::test]
async fn test_discovery_finds_capabilities() {
    let discovery = PrimalDiscovery::new();
    let result = discovery
        .find_capability(Capability::Discovery)
        .await;
    assert!(result.is_ok());
}

#[test]
fn test_self_knowledge_from_env() {
    env::set_var("PRIMAL_NAME", "test-primal");
    let sk = PrimalSelfKnowledge::discover().unwrap();
    assert_eq!(sk.my_name(), "test-primal");
}
```

---

## 📚 DOCUMENTATION

### Environment Variables

Create `ENVIRONMENT_VARIABLES.md`:

```markdown
# BearDog Environment Variables

## Identity
- `PRIMAL_NAME` - This primal's name (default: "beardog")
- `PRIMAL_VERSION` - Version override (default: from Cargo.toml)

## Network
- `BEARDOG_LISTEN_ADDR` - Listen address (default: "127.0.0.1:0")
- `BEARDOG_PORT` - Explicit port (default: OS-assigned)

## Discovery
- `BEARDOG_DISCOVERY_TIMEOUT_MS` - Discovery timeout (default: 5000)
- `BEARDOG_MDNS_ENABLED` - Enable mDNS (default: true)
- `BEARDOG_BIRDSONG_ENABLED` - Enable BirdSong (default: true)

## Configuration
- `BEARDOG_CONFIG_PATH` - Config file path (default: "./beardog.toml")
```

---

## 🚀 EXECUTION TIMELINE

**Total Duration**: 4 weeks

### Week 1 (Jan 13-19)
- Implement self-knowledge pattern
- Update server startup

### Week 2 (Jan 20-26)
- Implement runtime discovery
- Migrate client code

### Week 3 (Jan 27 - Feb 2)
- Configuration builder pattern
- Environment variable support

### Week 4 (Feb 3-9)
- Testing and polish
- Documentation
- Migration guide

---

**Status**: 🚀 **Ready to Execute**  
**Next**: Implement `self_knowledge.rs` module

🦀 **Zero Hardcoding Through Principles** 🌱

