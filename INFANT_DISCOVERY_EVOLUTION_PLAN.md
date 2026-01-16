# 🧒 Infant Discovery Evolution Plan

**Date**: January 13, 2026  
**Status**: 🔄 In Progress  
**Goal**: Complete Zero-Knowledge Bootstrap (Infant Discovery Pattern)

---

## 🎯 Vision: Infant Discovery

**Core Principle**: "Like an infant, primals start with zero knowledge and discover everything"

### Current State
- ✅ Self-knowledge from environment
- ✅ Primal discovery by capability
- ✅ Some vendor-agnostic patterns (service discovery, KMS)

### Target State
- 🎯 **Zero hardcoded primal names** - Discover all primals by capability
- 🎯 **Zero hardcoded vendor names** - Adapt to any infrastructure (K8s, Consul, bare metal)
- 🎯 **Zero hardcoded constants** - All ports, timeouts, addresses from environment/discovery
- 🎯 **Universal Adapter Pattern** - Single interface for all inter-primal communication

---

## 📊 Hardcoding Audit Results

### 1. Vendor Hardcoding ⚠️ (Partially Evolved)

**Found in 30+ files**: `kubernetes`, `k8s`, `consul`, `etcd`, `docker`, `prometheus`

**Status**: 🟡 **Good Foundation** - Already vendor-agnostic in key areas!

**Existing Good Patterns**:
- ✅ `ServiceDiscoveryCapability` trait (vendor-agnostic)
- ✅ Auto-detection pattern (`create_service_discovery()`)
- ✅ Fallback chains (K8s → Consul → DNS)
- ✅ Runtime capability detection

**Needs Evolution**:
- 🔄 Some hardcoded domain names (`cluster.local`, `service.consul`)
- 🔄 Vendor-specific config keys
- 🔄 Documentation examples with specific vendors

### 2. Primal Hardcoding ⚠️ (Needs Evolution)

**Found in 30+ files**: `Songbird`, `BiomeOS`, `Toadstool`, `Squirrel`, `Nestgate`

**Patterns Found**:
```rust
// ❌ BAD: Hardcoded primal names
if primal_name == "Songbird" { ... }
let songbird_url = "http://songbird.local:9100";

// ✅ GOOD: Capability-based
let discovery_primals = discover_by_capability(Capability::ServiceDiscovery);
let ai_primals = discover_by_capability(Capability::AI);
```

**Files Needing Evolution**:
- Test files (acceptable for testing)
- Example/documentation code (needs capability examples)
- Some API handlers (need capability-based routing)

### 3. Numeric Hardcoding ⚠️ (Well Managed)

**Status**: 🟢 **Excellent** - Already environment-driven!

**Good Patterns Found**:
- ✅ Port configuration in `beardog-config`
- ✅ Environment variables for all defaults
- ✅ Documented constants with fallbacks
- ✅ No magic numbers in code

**Remaining**:
- 🔄 Some test fixtures have hardcoded ports (acceptable)
- 🔄 DNS search domains hardcoded (`cluster.local`, `service.consul`)

---

## 🏗️ Architecture: Universal Adapter Pattern

### Concept

```
┌─────────────────────────────────────────────────────────────┐
│                    Universal Adapter                        │
│  "Single interface for all primal-to-primal communication" │
└─────────────────────────────────────────────────────────────┘
                         ↓
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
   Capability       Discovery        Routing
   Registry         Engine            Engine
        ↓                ↓                ↓
   [Security]       [Songbird]      [Choose by]
   [Storage]        [BiomeOS]       [capability]
   [AI]             [Toadstool]     [not name]
   [...]            [...]           [...]
```

### Flow Example

**Before** (Hardcoded):
```rust
// ❌ Hardcoded to Songbird
let songbird = connect_to("songbird.local:9100");
let result = songbird.discover_service("toadstool").await?;
```

**After** (Universal Adapter):
```rust
// ✅ Capability-based, primal-agnostic
let adapter = UniversalAdapter::new().await?;

// Discover ANY primal providing ServiceDiscovery
let discovery_primal = adapter
    .find_primal_by_capability(Capability::ServiceDiscovery)
    .await?;

// Use it (don't care if it's Songbird, BiomeOS, or future primal)
let result = discovery_primal
    .discover_service_by_capability(Capability::Compute)
    .await?;
```

---

## 📝 Evolution Phases

### Phase 1: Universal Adapter Foundation ✅ (Already Started!)

**Status**: 🟢 Partially Complete

**Existing**:
- ✅ `ServiceDiscoveryCapability` trait (vendor-agnostic)
- ✅ `KeyManagementCapability` trait (vendor-agnostic)
- ✅ Capability-based types (`ServiceCapabilityType`)
- ✅ Auto-detection patterns

**Next**:
- 🔄 Consolidate into single `UniversalAdapter`
- 🔄 Add primal-to-primal capability routing

### Phase 2: Primal Name Elimination 🔄 (This Phase)

**Target**: Remove all hardcoded primal names from production code

**Strategy**:
1. ✅ Scan for hardcoded names (complete)
2. 🔄 Categorize by usage:
   - Tests → Mark with `// TEST FIXTURE: Songbird`
   - Examples → Convert to generic `CapabilityProvider`
   - Production → Evolve to capability-based discovery
3. 🔄 Create migration guide
4. 🔄 Update documentation

**Files to Evolve** (Priority):
- `crates/beardog-tunnel/src/api/birdsong.rs` → Generic discovery API
- `crates/beardog-tunnel/src/api/lineage.rs` → Capability-based lineage
- `crates/beardog-core/src/ecosystem_integration/secure_cross_primal_messaging.rs` → Universal messaging

### Phase 3: Vendor Name Abstraction 🔄 (This Phase)

**Target**: No vendor assumptions in runtime code

**Strategy**:
1. ✅ Scan for vendor names (complete)
2. 🔄 Verify abstraction layers:
   - Service discovery → `ServiceDiscoveryCapability` ✅
   - KMS → `KeyManagementCapability` ✅
   - Monitoring → `MonitoringCapability` 🔄
3. 🔄 Remove hardcoded search domains
4. 🔄 Environment-driven vendor detection

**Domain Names to Evolve**:
```rust
// ❌ Before: Hardcoded search domains
vec!["cluster.local", "service.consul"]

// ✅ After: Environment-driven
env::var("DISCOVERY_SEARCH_DOMAINS")
    .unwrap_or_else(|| detect_search_domains())
```

### Phase 4: Complete Infant Discovery 🎯 (Target)

**Capabilities**:
1. **Zero Initial Knowledge**
   - Primal starts knowing only itself
   - No assumptions about infrastructure
   - No assumptions about other primals

2. **Progressive Discovery**
   - Detect local environment (K8s, bare metal, etc.)
   - Discover available capabilities
   - Build capability map

3. **Adaptive Behavior**
   - Use best available infrastructure
   - Graceful degradation
   - Runtime optimization

---

## 🔨 Implementation: Universal Adapter

### Core Interface

```rust
/// Universal adapter for primal-to-primal communication
///
/// **Philosophy**: Primals only know themselves, discover others by capability
pub struct UniversalAdapter {
    /// Self-knowledge (who am I?)
    self_knowledge: PrimalSelfKnowledge,
    
    /// Discovery engine (who provides what?)
    discovery: PrimalDiscovery,
    
    /// Capability router (how to reach them?)
    router: CapabilityRouter,
    
    /// Cached capability map
    capability_map: RwLock<HashMap<Capability, Vec<DiscoveredPrimal>>>,
}

impl UniversalAdapter {
    /// Create adapter with zero initial knowledge
    pub async fn new() -> Result<Self, BearDogError> {
        // Step 1: Discover self
        let self_knowledge = PrimalSelfKnowledge::discover()?;
        
        // Step 2: Initialize discovery (environment-driven)
        let discovery = PrimalDiscovery::from_env()?;
        
        // Step 3: Initialize routing (capability-based)
        let router = CapabilityRouter::new().await?;
        
        Ok(Self {
            self_knowledge,
            discovery,
            router,
            capability_map: RwLock::new(HashMap::new()),
        })
    }
    
    /// Find ANY primal providing a capability
    pub async fn find_primal_by_capability(
        &self,
        capability: Capability,
    ) -> Result<PrimalConnection, BearDogError> {
        // 1. Check cache
        if let Some(cached) = self.get_cached_primal(&capability).await {
            return Ok(cached);
        }
        
        // 2. Discover primals with this capability
        let primals = self.discovery
            .discover(DiscoveryQuery::by_capability(capability.clone()))
            .await?;
        
        // 3. Route to best primal
        let decision = self.router
            .route(capability.clone(), RequestContext::default())
            .await?;
        
        // 4. Connect
        let connection = self.connect_to_primal(&decision.primal).await?;
        
        // 5. Cache for future use
        self.cache_primal(capability, decision.primal).await;
        
        Ok(connection)
    }
    
    /// Send request to ANY primal providing capability (don't care who)
    pub async fn send_to_capability<T, R>(
        &self,
        capability: Capability,
        request: T,
    ) -> Result<R, BearDogError>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let primal = self.find_primal_by_capability(capability).await?;
        primal.send(request).await
    }
}
```

---

## 📋 Migration Checklist

### Production Code

- [ ] Create `UniversalAdapter` in `beardog-core/src/`
- [ ] Migrate `birdsong.rs` API to capability-based
- [ ] Migrate `lineage.rs` API to capability-based
- [ ] Update cross-primal messaging to use adapter
- [ ] Remove hardcoded search domains
- [ ] Add environment detection for domains

### Test Code

- [ ] Mark test fixtures with `// TEST FIXTURE: <PrimalName>`
- [ ] Create capability-based test helpers
- [ ] Document test primal setup

### Documentation

- [ ] Update examples to use capabilities
- [ ] Create Universal Adapter guide
- [ ] Update architecture diagrams
- [ ] Add infant discovery explanation

### Configuration

- [ ] Add `DISCOVERY_SEARCH_DOMAINS` env var
- [ ] Add `INFRASTRUCTURE_TYPE` env var (optional hint)
- [ ] Update config templates

---

## 🎯 Success Criteria

| Criterion | Status |
|-----------|--------|
| Zero hardcoded primal names in production | 🔄 |
| Zero hardcoded vendor names in production | 🟢 |
| Zero hardcoded search domains | 🔄 |
| Universal adapter implemented | 🔄 |
| All tests use test fixtures | 🔄 |
| Documentation updated | 🔄 |
| Infant discovery demonstrated | 🔄 |

---

## 🧪 Demonstration: Infant Discovery

**Scenario**: BearDog needs AI analysis but doesn't know who provides it

**Before** (Hardcoded):
```rust
// ❌ Hardcoded to Squirrel AI primal
let squirrel = connect_to("squirrel.local:9300");
let analysis = squirrel.analyze(data).await?;
```

**After** (Infant Discovery):
```rust
// ✅ Discover ANY primal with AI capability
let adapter = UniversalAdapter::new().await?;

// Don't know who provides AI, don't care!
// Could be Squirrel, could be future AI primal, doesn't matter
let result: AnalysisResult = adapter
    .send_to_capability(
        Capability::AI,
        AnalysisRequest { data, model: "sentiment" }
    )
    .await?;

// BearDog discovered Squirrel (or whoever provides AI) automatically!
```

**Benefits**:
- 🎯 No hardcoded primal names
- 🎯 Works with future AI primals
- 🎯 Automatic failover if primary offline
- 🎯 Load balancing across multiple AI providers
- 🎯 True zero-knowledge bootstrap

---

## 🚀 Next Steps

### Immediate (This Session)
1. Complete vendor hardcoding scan
2. Complete primal name scan
3. Create Universal Adapter skeleton
4. Update 3-5 key files to use adapter

### Short Term (Next Session)
1. Complete Universal Adapter implementation
2. Migrate all production code
3. Update all documentation
4. Create demonstration examples

### Long Term
1. Extend to all ecoPrimals
2. Create ecosystem-wide adapter protocol
3. Enable dynamic primal mesh networking
4. Implement infant learning patterns

---

**Status**: 🔄 Phase 2 in progress  
**Target**: Complete infant discovery pattern  
**Impact**: True zero-knowledge, capability-based ecosystem

