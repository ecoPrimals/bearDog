# 🏗️ Smart Refactoring Plan: discovery_unified.rs

## 📊 Current State
- **File**: `crates/beardog-types/src/canonical/config/domains/discovery_unified.rs`
- **Size**: 992 lines (under 1000 limit, but can be improved)
- **Complexity**: 12 structs, 2 enums, 10 impl blocks
- **Issue**: Monolithic file mixing multiple concerns

## 🎯 Refactoring Strategy: Domain-Driven Design

### Principle: **Cohesion over Coupling**
Split by **domain responsibility**, not arbitrary line counts. Each module should represent a **single concern** with high cohesion.

---

## 📁 New Module Structure

```
crates/beardog-types/src/canonical/config/domains/discovery/
├── mod.rs                    # Main orchestrator (UnifiedDiscoveryConfig + DiscoveryProtocol)
├── registry.rs               # Service Registry domain
├── network.rs                # Network Discovery domain
├── quantum.rs                # Quantum Discovery domain (experimental)
├── cache.rs                  # Discovery Cache domain
├── security.rs               # Discovery Security domain
├── load_balancing.rs         # Load Balancing & Circuit Breaker domain
└── tests.rs                  # Integration tests (optional)
```

---

## 📦 Module Breakdown

### 1. `mod.rs` (Orchestrator) - ~200 lines
**Responsibility**: Main configuration struct that composes all domains

**Contents**:
- `UnifiedDiscoveryConfig` struct (main config)
- `DiscoveryProtocol` enum
- `impl UnifiedDiscoveryConfig` (builder methods)
- `impl Default for UnifiedDiscoveryConfig`
- `impl BearDogConfig for UnifiedDiscoveryConfig`
- Re-exports from submodules

**Why**: Central point of coordination, minimal logic

---

### 2. `registry.rs` (Service Registry) - ~150 lines
**Responsibility**: Service registration and tracking

**Contents**:
- `ServiceRegistryConfig` struct
- `EtcdAuth` struct
- `impl ServiceRegistryConfig`
- `impl Default for ServiceRegistryConfig`

**Domain Concepts**:
- Backend selection (etcd, consul, zookeeper, redis)
- Service TTL and health checks
- Cleanup intervals
- Versioning

**Why**: Clear domain boundary - all about service lifecycle management

---

### 3. `network.rs` (Network Discovery) - ~120 lines
**Responsibility**: Protocol-based service discovery

**Contents**:
- `NetworkDiscoveryConfig` struct
- `impl NetworkDiscoveryConfig`
- `impl Default for NetworkDiscoveryConfig`

**Domain Concepts**:
- Protocol selection (mDNS, DNS-SD, UPnP, SSDP, HTTP, gRPC)
- Port scanning
- Timeout configuration
- Retry strategies

**Why**: Network-specific discovery mechanisms

---

### 4. `quantum.rs` (Quantum Discovery) - ~100 lines
**Responsibility**: Experimental quantum-enhanced discovery

**Contents**:
- `QuantumDiscoveryConfig` struct
- `impl QuantumDiscoveryConfig`
- `impl Default for QuantumDiscoveryConfig`

**Domain Concepts**:
- Quantum coherence
- Entanglement-based discovery
- Superposition states
- Experimental features

**Why**: Isolated experimental feature, easy to feature-gate

---

### 5. `cache.rs` (Discovery Cache) - ~100 lines
**Responsibility**: Caching discovered services

**Contents**:
- `DiscoveryCacheConfig` struct
- `impl DiscoveryCacheConfig`
- `impl Default for DiscoveryCacheConfig`

**Domain Concepts**:
- Cache size limits
- TTL management
- Eviction policies
- Cache invalidation

**Why**: Performance optimization concern, separate from discovery logic

---

### 6. `security.rs` (Discovery Security) - ~120 lines
**Responsibility**: Security and authentication for discovery

**Contents**:
- `DiscoverySecurityConfig` struct
- `impl DiscoverySecurityConfig`
- `impl Default for DiscoverySecurityConfig`

**Domain Concepts**:
- Authentication requirements
- Encryption settings
- TLS configuration
- Access control

**Why**: Security is a cross-cutting concern that should be isolated

---

### 7. `load_balancing.rs` (Load Balancing) - ~200 lines
**Responsibility**: Load balancing and resilience patterns

**Contents**:
- `LoadBalancingConfig` struct
- `LoadBalancingAlgorithm` enum
- `CircuitBreakerConfig` struct
- `StickySessionsConfig` struct
- All related impl blocks and defaults

**Domain Concepts**:
- Load balancing algorithms (round-robin, least-connections, random, weighted)
- Circuit breaker patterns
- Sticky sessions
- Health checks

**Why**: Resilience patterns form a cohesive domain

---

## 🔄 Migration Strategy

### Phase 1: Create New Modules (No Breaking Changes)
1. Create `discovery/` directory
2. Extract each domain to its own file
3. Keep original file as re-export wrapper
4. Add deprecation notice

### Phase 2: Update Imports (Gradual)
1. Update internal imports to use new modules
2. External users still use `discovery_unified`
3. No breaking changes yet

### Phase 3: Deprecate Old File (After 1-2 releases)
1. Mark `discovery_unified.rs` as deprecated
2. Provide migration guide
3. Eventually remove old file

---

## ✅ Benefits

### 1. **Maintainability**
- Each file has a single, clear responsibility
- Easier to find and modify specific functionality
- Reduced cognitive load

### 2. **Testability**
- Domain-specific tests in each module
- Easier to mock dependencies
- Better test organization

### 3. **Extensibility**
- Add new discovery methods without touching others
- Feature flags per domain
- Independent versioning

### 4. **Performance**
- Faster compilation (smaller modules)
- Better IDE performance
- Easier to optimize individual domains

### 5. **Team Collaboration**
- Multiple developers can work on different domains
- Reduced merge conflicts
- Clear ownership boundaries

---

## 📏 Size Comparison

| Module | Lines | Complexity | Responsibility |
|--------|-------|------------|----------------|
| **Before** | 992 | High | Everything |
| **After (Total)** | ~990 | Low | Distributed |
| mod.rs | ~200 | Low | Orchestration |
| registry.rs | ~150 | Low | Service Registry |
| network.rs | ~120 | Low | Network Discovery |
| quantum.rs | ~100 | Low | Quantum (Experimental) |
| cache.rs | ~100 | Low | Caching |
| security.rs | ~120 | Low | Security |
| load_balancing.rs | ~200 | Medium | Resilience |

---

## 🎯 Success Criteria

- ✅ Each module under 250 lines
- ✅ Clear domain boundaries
- ✅ No circular dependencies
- ✅ All tests passing
- ✅ Zero breaking changes (Phase 1)
- ✅ Improved compilation time
- ✅ Better IDE performance

---

## 🚀 Implementation Steps

1. **Create directory structure**
2. **Extract `registry.rs`** (Service Registry domain)
3. **Extract `network.rs`** (Network Discovery domain)
4. **Extract `quantum.rs`** (Quantum Discovery domain)
5. **Extract `cache.rs`** (Cache domain)
6. **Extract `security.rs`** (Security domain)
7. **Extract `load_balancing.rs`** (Resilience domain)
8. **Create `mod.rs`** (Orchestrator with re-exports)
9. **Update `discovery_unified.rs`** (Wrapper with deprecation)
10. **Run tests** (Verify no regressions)
11. **Update documentation**

---

## 📝 Notes

- This refactoring follows **Domain-Driven Design** principles
- Each module represents a **Bounded Context**
- The orchestrator (`mod.rs`) is the **Aggregate Root**
- Submodules are **Entities** within the aggregate
- This is **smart refactoring**, not arbitrary splitting

---

**Status**: Ready to implement  
**Estimated Time**: 2-3 hours  
**Risk**: Low (no breaking changes in Phase 1)  
**Value**: High (long-term maintainability)

🐻🏗️ **BearDog: Building for the Future!**

