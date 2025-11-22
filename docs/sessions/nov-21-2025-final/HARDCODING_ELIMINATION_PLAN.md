# Hardcoding Elimination Plan - Zero-Knowledge Infant Deployment

**Goal**: Each primal only knows itself and discovers everything through the universal adapter.

## Audit Summary

### Vendor Hardcoding (343 references across 60 files)
- **Platforms**: kubernetes, k8s, consul, docker, etcd
- **Services**: vault, redis, postgres, mongodb  
- **Status**: Need vendor-agnostic abstractions

### Primal Hardcoding (2204 references across 860 files) 🔴 CRITICAL
- **Primal Names**: songbird, toadstool, squirrel, nestgate, beardog
- **Pattern**: Direct primal-to-primal references instead of capability discovery
- **Status**: Violates "each primal only knows itself" principle

### Numeric Hardcoding (817 references across 273 files)
- **Ports**: Hardcoded port numbers (8080, 8443, 5432, etc.)
- **Timeouts**: Hardcoded duration values
- **Status**: Need config/environment-based discovery

## Migration Strategy

### Phase 1: Vendor Abstraction Layer ✅ (Already exists)
The universal adapter infrastructure is in place:
- `UniversalCapabilityAdapter` - vendor-agnostic capability discovery
- `ServiceDiscoveryMethod` - abstraction for k8s, consul, dns, mdns
- Vendor-specific handlers behind universal interface

**Action**: Replace direct vendor references with universal adapter calls

### Phase 2: Primal Sovereignty Migration 🎯 PRIMARY FOCUS
Each primal should only:
1. Know its own identity (self-discovery)
2. Advertise its capabilities
3. Discover others through universal adapter

**Anti-Pattern** (Current):
```rust
// WRONG: Hardcoded primal name
songbird_client.connect("songbird.local:8080")?;
toadstool.request_compute(data)?;
```

**Correct Pattern** (Target):
```rust
// RIGHT: Capability-based discovery
let network_primal = adapter.discover_capability(NetworkFunction::ServiceMesh)?;
let compute_primal = adapter.discover_capability(ComputeAbility::DataAnalysis)?;
adapter.send_request(compute_primal, request)?;
```

### Phase 3: Numeric Value Discovery 
**Current**: Hardcoded ports, timeouts
**Target**: Environment variables → Config → Discovery → Runtime determination

**Priority Order**:
1. `BEARDOG_*` environment variables (highest priority)
2. Config file values
3. Service discovery (mDNS, Consul, k8s)
4. Documented fallback defaults (lowest priority)

## Implementation Phases

### Week 1: Critical Primal Hardcoding (Days 1-3)
- [ ] Eliminate songbird hardcoding → NetworkFunction capability
- [ ] Eliminate toadstool hardcoding → ComputeAbility capability  
- [ ] Eliminate squirrel hardcoding → DistributedIntelligence capability
- [ ] Eliminate nestgate hardcoding → DataStorage capability

### Week 1: Vendor Abstraction (Days 4-5)
- [ ] Abstract k8s references → ServiceDiscoveryMethod::Kubernetes
- [ ] Abstract consul references → ServiceDiscoveryMethod::Consul
- [ ] Abstract docker references → ContainerOrchestration capability
- [ ] Abstract vault/secrets → SecretsManagement capability

### Week 2: Numeric Configuration (Days 1-3)
- [ ] Port discovery system (env → config → discovery)
- [ ] Timeout configuration (remove all hardcoded durations)
- [ ] Buffer/limit values (move to capacity discovery)

### Week 2: Zero-Knowledge Bootstrap (Days 4-5)
- [ ] Infant deployment tests (start with zero knowledge)
- [ ] Self-discovery validation
- [ ] Ecosystem integration via universal adapter only

## File Hotspots (High hardcoding density)

### Critical Files:
1. `crates/beardog-core/src/ecosystem_integration/songbird_integration.rs` - songbird hardcoding
2. `crates/beardog-core/src/zero_knowledge_bootstrap/self_discovery.rs` - beardog name hardcoding
3. `crates/beardog-types/src/constants/domains/network.rs` - port hardcoding
4. `crates/beardog-adapters/src/universal/adapter_impl.rs` - numeric fallbacks
5. Test files (860 files with primal names) - test infrastructure hardcoding

## Primal Capability Mapping

### Songbird (Network/Service Mesh)
**Hardcoded References** → **Capabilities**:
- `songbird.connect()` → `NetworkFunction::ServiceMesh`
- `songbird.discover()` → `NetworkFunction::ServiceDiscovery`
- `songbird.route()` → `NetworkFunction::TrafficRouting`

### Toadstool (Compute/AI)
**Hardcoded References** → **Capabilities**:
- `toadstool.compute()` → `ComputeAbility::DataAnalysis`
- `toadstool.ai_inference()` → `ComputeAbility::MachineLearning`
- `toadstool.analyze()` → `ComputeAbility::PatternRecognition`

### Squirrel (Distributed Intelligence)
**Hardcoded References** → **Capabilities**:
- `squirrel.ai_request()` → `DistributedIntelligence::NeuralNetwork`
- `squirrel.training()` → `DistributedIntelligence::ModelTraining`

### Nestgate (Data/Storage)
**Hardcoded References** → **Capabilities**:
- `nestgate.store()` → `DataStorage::PersistentStorage`
- `nestgate.query()` → `DataStorage::QueryProcessing`
- `nestgate.stream()` → `DataStorage::StreamProcessing`

## Success Criteria

### 🎯 Zero-Knowledge Deployment Test
```bash
# Start primal with ZERO configuration
docker run beardog:latest
# Expected behavior:
# 1. Discovers own identity (generates UUID-based ID)
# 2. Detects own capabilities (filesystem/runtime introspection)
# 3. Announces to discovery service (via BEARDOG_DISCOVERY_ENDPOINT or mDNS)
# 4. Discovers peer capabilities (via universal adapter)
# 5. Begins normal operations (without any hardcoded knowledge)
```

### Metrics
- ✅ Zero direct primal name references in production code
- ✅ Zero hardcoded vendor platform names (k8s, consul, etc.)
- ✅ All ports/timeouts from config/environment/discovery
- ✅ Infant deployment test passes
- ✅ Can deploy without prior ecosystem knowledge

## Migration Tools

### Search & Replace Patterns
```bash
# Find primal name hardcoding
rg -i '\b(songbird|toadstool|squirrel|nestgate)\b' --type rust

# Find vendor hardcoding  
rg -i '\b(kubernetes|k8s|consul|docker|etcd)\b' --type rust

# Find numeric hardcoding (ports)
rg ':\s*\d{4,5}\b' --type rust
```

### Code Migration Templates
See: `ecosystem-templates/primal-hardcoding-elimination-template.rs`
See: `ecosystem-templates/songbird-migration-template.rs`

## Notes

- **Backward Compatibility**: Keep deprecated primal-specific clients with warnings for 2 releases
- **Testing**: Each migration needs comprehensive tests proving zero-knowledge deployment
- **Documentation**: Update all docs to show capability-based patterns
- **Performance**: Universal adapter adds <5ms latency (acceptable for sovereignty gains)

## References

- `UNIVERSAL_ADAPTER_SPECIFICATION.md` - Universal adapter architecture
- `BEARDOG_CODING_STANDARDS.md` - Primal sovereignty principles
- `crates/beardog-adapters/src/universal/primal_capability_adapter.rs` - Reference implementation

