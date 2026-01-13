# 🐦 Songbird Evolution for LiveSpore Support
**Cross-Primal Architecture Evolution Plan**

**Date**: January 13, 2026  
**For**: Songbird Team  
**From**: BearDog Team (Post-LiveSpore Architecture Discovery)  
**Status**: 🎯 **EVOLUTION ROADMAP**

---

## 🎯 **Executive Summary**

**Current State**: Songbird v3.22.1 (Grade A, 92/100) is production-ready with:
- ✅ Zero hardcoding
- ✅ Genetic lineage integration
- ✅ BirdSong encrypted discovery (v2)
- ✅ Capability-based discovery
- ✅ Federation support

**Gap**: BirdSong protocol needs evolution to support **multi-callsign tag system** for LiveSpore.

**Recommendation**: Parallel evolution of BirdSong v3.0 while maintaining v2.0 compatibility.

---

## 🔥 **Critical Realizations from BearDog's LiveSpore Work**

### **1. Multi-Callsign Tag System is Actually Simple**

The "multi-callsign" system isn't a complex new protocol - it's just **thoughtful use** of existing BirdSong:

```rust
// Current BirdSong v2.0 (already supports this!)
{
  "version": 2,
  "family_id": "MSU",  // ← Public callsign (visible to all)
  "encrypted_payload": {
    "ciphertext": "<encrypted>",  // ← Only genetic family can decrypt
    "nonce": "...",
    "algorithm": "ChaCha20-Poly1305"
  }
}

// Inside encrypted payload (only family sees):
{
  "primal_id": "basement-hpc-node-42",
  "endpoint": "192.168.1.100:8080",  // ← Private routing!
  "identity_attestations": {
    "family_id": "MSU",  // ← Confirms family
    "genetic_lineage": "<hash>",  // ← Cryptographic proof
    ...
  }
}
```

**Insight**: You already have 80% of what's needed! Just need:
1. Multiple `family_id` tags per node (user-configurable)
2. Tag management interface
3. Routing info in encrypted payload

### **2. The MSU Use Case (Brilliant!)**

Users want to use **institutional NAT** (MSU, university, etc.) instead of cloud servers:

```
Public Tag: "MSU" (visible, legitimate use)
↓
Encrypted Routing: 192.168.1.100:8080 (only family decrypts)
↓
Result: Use MSU network → Route to basement HPC → Zero cloud costs
```

**Songbird's Role**: Broadcast `family_id: "MSU"` while encrypting private routing for genetic family only.

### **3. Current BirdSong Issues (From wateringHole/birdsong/BIRDSONG_PROTOCOL.md)**

Already documented:

1. ❌ **No key rotation** - Same family seed forever
2. ❌ **No replay protection** - Old packets can be resent
3. ❌ **No rate limiting** - Beacon spam possible

**Plus new requirements**:

4. ❌ **No multi-tag support** - Node can only have one `family_id`
5. ❌ **No tag management** - Can't configure/update tags at runtime
6. ❌ **No routing metadata** - Encrypted payload structure is ad-hoc

---

## 🏗️ **Recommended Evolution Path**

### **Phase 1: Concurrent Test Evolution (Weeks 1-2)**

**Status**: Songbird has 254 `sleep` calls and `Arc<Mutex>` patterns (BearDog had similar issues)

**Goal**: Evolve to modern concurrent Rust like BearDog did

**Impact**: 
- ✅ Faster tests (BearDog: ~5x faster)
- ✅ More reliable (event-driven vs timing-based)
- ✅ Better production patterns (test concurrency = production concurrency)

**Action Items**:

1. **Create `songbird-test-utils/src/concurrent_helpers.rs`** (2-3 hours)
   ```rust
   // Copy from BearDog's pattern:
   pub struct ReadinessSignal { ... }
   pub struct CompletionWaiter { ... }
   pub struct AsyncBarrier { ... }
   pub fn unique_unix_socket() -> PathBuf { ... }
   pub struct RetryPolicy { ... }
   ```

2. **Refactor `tests/common/sync_helpers.rs`** (2-3 hours)
   - Replace `sleep` with event-driven synchronization
   - Use `ReadinessSignal` for service startup
   - Use `RetryPolicy` for network polling

3. **Remove `Arc<Mutex>` anti-patterns** (3-4 hours)
   - Replace with `tokio::sync::RwLock` (async-aware)
   - Use `parking_lot::RwLock` for sync contexts only
   - Prefer lock-free patterns (channels, atomics)

**Effort**: ~10 hours  
**Benefit**: Production-grade concurrent patterns  
**Reference**: `beardog/tests/support/concurrent_helpers.rs`

---

### **Phase 2: BirdSong v3.0 Protocol Evolution (Weeks 2-3)**

**Goal**: Support multi-callsign tags while maintaining v2.0 compatibility

#### **2.1: Multi-Tag Support (4-6 hours)**

**Problem**: Node can only broadcast one `family_id`

**Solution**: Support multiple tags per node

```rust
// Current (v2.0):
pub struct BirdSongPacket {
    version: u8,
    family_id: String,  // ← Single tag only
    encrypted_payload: EncryptedPayload,
    timestamp: u64,
    ttl: u32,
}

// Proposed (v3.0):
pub struct BirdSongPacket {
    version: u8,
    
    // v2 compatibility (single tag)
    family_id: Option<String>,
    
    // v3 evolution (multi-tag)
    tags: Option<Vec<CallsignTag>>,
    
    encrypted_payload: EncryptedPayload,
    timestamp: u64,
    ttl: u32,
}

pub struct CallsignTag {
    /// Public tag (visible to all)
    pub tag: String,  // "MSU", "Personal", "Federation", etc.
    
    /// Tag purpose (hints for routing)
    pub purpose: TagPurpose,
    
    /// Priority (for multi-tag resolution)
    pub priority: u8,  // 0-255, higher = prefer
}

pub enum TagPurpose {
    Institutional,  // MSU, university, etc.
    Personal,       // Direct personal access
    Federation,     // Federated network
    Public,         // Public service
    Custom(String), // User-defined
}
```

**Migration Strategy**:

1. **Week 1**: Add `tags` field (optional), keep `family_id` (required)
2. **Week 2**: Both fields supported (v2 and v3 compatibility)
3. **Week 3**: Deprecate `family_id`, prefer `tags`

**Testing**:
- v2 clients can talk to v3 servers ✅
- v3 clients can talk to v2 servers ✅
- Graceful degradation ✅

#### **2.2: Routing Metadata Schema (2-3 hours)**

**Problem**: Encrypted payload structure is ad-hoc

**Solution**: Formalize routing metadata

```rust
// New type for encrypted payload
pub struct RoutingMetadata {
    /// Primary endpoint (always present)
    pub primary_endpoint: String,  // "192.168.1.100:8080"
    
    /// Fallback endpoints (optional)
    pub fallback_endpoints: Option<Vec<String>>,
    
    /// NAT traversal hints
    pub nat_config: Option<NatConfig>,
    
    /// Geographic hints (for latency optimization)
    pub geo_hints: Option<GeoHints>,
    
    /// Primal capabilities
    pub capabilities: Vec<String>,
    
    /// Identity attestations (genetic lineage)
    pub identity_attestations: IdentityAttestations,
}

pub struct NatConfig {
    pub traversal_method: NatTraversalMethod,
    pub public_endpoint: Option<String>,
    pub stun_servers: Option<Vec<String>>,
}

pub enum NatTraversalMethod {
    Direct,           // No NAT
    PortForwarding,   // Manual port forwarding
    UPnP,            // Automatic UPnP
    STUN,            // STUN-based
    Institutional,   // Use institutional NAT (MSU, etc.)
}
```

**Benefits**:
- ✅ Structured routing info
- ✅ Supports complex NAT scenarios
- ✅ Enables intelligent routing decisions
- ✅ Backward compatible (nested in `encrypted_payload`)

#### **2.3: Tag Management API (3-4 hours)**

**Problem**: No runtime tag configuration

**Solution**: Add tag management endpoints

```rust
// New IPC endpoints for tag management

// Add a tag
POST /api/v1/birdsong/tags/add
{
  "tag": "MSU",
  "purpose": "Institutional",
  "priority": 100,
  "routing": {
    "primary_endpoint": "192.168.1.100:8080",
    "nat_config": {
      "traversal_method": "Institutional"
    }
  }
}

// Remove a tag
DELETE /api/v1/birdsong/tags/remove
{
  "tag": "MSU"
}

// List current tags
GET /api/v1/birdsong/tags

// Update tag routing
PATCH /api/v1/birdsong/tags/{tag}
{
  "routing": { ... }
}
```

**Implementation**:
- Add to `songbird-orchestrator/src/ipc/handlers/p2p_discovery.rs`
- Persist tags in node configuration
- Hot-reload on tag changes (no restart)

---

### **Phase 3: BirdSong Security Hardening (Weeks 3-4)**

**Goal**: Fix known BirdSong v2.0 issues

#### **3.1: Key Rotation (8-10 hours)**

**Problem**: Same family seed forever (high security impact)

**Solution**: Implement key rotation protocol

```rust
pub struct KeyRotationConfig {
    /// Rotation interval (e.g., 30 days)
    pub rotation_interval: Duration,
    
    /// Overlap period (old + new keys both valid)
    pub overlap_period: Duration,
    
    /// Key derivation (from genetic lineage)
    pub derivation: KeyDerivationMethod,
}

pub enum KeyDerivationMethod {
    /// Derive from genetic lineage + epoch
    GeneticWithEpoch { epoch: u64 },
    
    /// HKDF with time-based salt
    HKDFTimeBased { salt: Vec<u8> },
}

// BirdSong packet includes key epoch
pub struct BirdSongPacket {
    // ... existing fields ...
    
    /// Key epoch (for rotation support)
    pub key_epoch: Option<u64>,
}
```

**Migration**:
1. Add `key_epoch` field (optional, v3.0)
2. Derive keys from `genetic_lineage + epoch`
3. Support 2 keys during overlap period
4. Retire old key after overlap

**Integration with BearDog**:
- BearDog provides key derivation (`/api/v1/lineage/derive-key`)
- Songbird manages rotation schedule
- Automatic re-encryption on rotation

#### **3.2: Replay Protection (3-4 hours)**

**Problem**: Old packets can be resent

**Solution**: Sequence numbers + timestamp validation

```rust
pub struct BirdSongPacket {
    // ... existing fields ...
    
    /// Sequence number (per sender)
    pub sequence: u64,
    
    /// Sender ID (for sequence tracking)
    pub sender_id: String,
}

// Per-sender sequence tracking
pub struct SequenceTracker {
    last_seen: HashMap<String, u64>,
    max_age: Duration,
}

impl SequenceTracker {
    pub fn is_valid(&mut self, sender: &str, seq: u64, ts: u64) -> bool {
        // Check timestamp freshness
        if Utc::now().timestamp() as u64 - ts > self.max_age.as_secs() {
            return false;
        }
        
        // Check sequence number
        let last = self.last_seen.entry(sender.to_string()).or_insert(0);
        if seq <= *last {
            return false;  // Replay!
        }
        *last = seq;
        true
    }
}
```

**Impact**:
- ✅ Prevents replay attacks
- ✅ Minimal overhead (~8 bytes per packet)
- ✅ Automatic cleanup (old entries expire)

#### **3.3: Rate Limiting (2-3 hours)**

**Problem**: Beacon spam possible

**Solution**: Adaptive beaconing + rate limits

```rust
pub struct BeaconScheduler {
    /// Base beacon interval (e.g., 30s)
    base_interval: Duration,
    
    /// Current interval (adapts based on network)
    current_interval: Duration,
    
    /// Rate limiter (per sender)
    rate_limiter: RateLimiter,
}

impl BeaconScheduler {
    pub fn adapt(&mut self, network_state: NetworkState) {
        match network_state {
            NetworkState::Stable => {
                // Increase interval (reduce frequency)
                self.current_interval = (self.current_interval * 120 / 100)
                    .min(Duration::from_secs(300));  // Max 5min
            }
            NetworkState::Changing => {
                // Decrease interval (increase frequency)
                self.current_interval = (self.current_interval * 80 / 100)
                    .max(self.base_interval);  // Min 30s
            }
        }
    }
}

pub struct RateLimiter {
    max_beacons_per_minute: usize,
    sender_counts: HashMap<String, usize>,
    last_reset: Instant,
}
```

**Benefits**:
- ✅ Prevents spam
- ✅ Adapts to network conditions
- ✅ Fair resource usage

---

### **Phase 4: BiomeOS Integration Preparation (Weeks 4-5)**

**Goal**: Prepare for BiomeOS LiveSpore integration

#### **4.1: First-Boot Tag Configuration UI (4-6 hours)**

**Problem**: LiveSpore needs SoloKey personalization flow

**Solution**: Interactive tag configuration during genesis

```rust
// New module: songbird-cli/src/cli/commands/genesis.rs

pub async fn genesis_ceremony(solokey: &SoloKeyDevice) -> Result<GenesisConfig> {
    println!("🔑 SoloKey Detected: {}", solokey.device_id());
    println!("\nPress the button on your SoloKey to begin...\n");
    
    solokey.wait_for_button_press().await?;
    
    println!("✅ Hardware witness received!");
    println!("🔐 Collecting hardware entropy...");
    
    let hw_entropy = solokey.generate_entropy(32).await?;
    println!("✅ {} bytes collected (Quality: {}%)", 
        hw_entropy.len(), 
        calculate_entropy_quality(&hw_entropy)
    );
    
    println!("\n👤 Now collecting human entropy...");
    println!("   Type random keys and move your mouse for 30 seconds.\n");
    
    let human_entropy = collect_human_entropy(30).await?;
    println!("✅ {} interactions captured", human_entropy.len());
    
    println!("\n🧬 Generating genetic lineage...");
    let lineage = generate_genetic_lineage(
        solokey.witness(),
        &hw_entropy,
        &human_entropy,
    ).await?;
    
    println!("✅ Genetic Lineage: {}", lineage.family_id());
    
    println!("\n🏷️  Configure public tags (optional):");
    println!("   [ ] MSU (Michigan State University)");
    println!("   [ ] Personal");
    println!("   [ ] Federation");
    println!("   [✓] Default (use Family ID as tag)");
    
    let tags = prompt_tag_configuration().await?;
    
    Ok(GenesisConfig {
        lineage,
        tags,
        solokey_witness: solokey.witness(),
    })
}
```

**Integration Points**:
- Calls BearDog for genetic lineage generation
- Configures Songbird tags
- Stores witness for future verification

#### **4.2: NUCLEUS Discovery Enhancement (3-4 hours)**

**Problem**: BiomeOS NUCLEUS needs enhanced discovery metadata

**Solution**: Add NUCLEUS-specific fields to BirdSong

```rust
pub struct RoutingMetadata {
    // ... existing fields ...
    
    /// NUCLEUS discovery metadata
    pub nucleus_metadata: Option<NucleusMetadata>,
}

pub struct NucleusMetadata {
    /// BiomeOS version
    pub biomeos_version: String,
    
    /// Available primals
    pub primals: Vec<PrimalInfo>,
    
    /// Atomic type (Tower, Node, Nest, NUCLEUS)
    pub atomic_type: AtomicType,
    
    /// Trust level (for graduated disclosure)
    pub trust_level: TrustLevel,
}

pub struct PrimalInfo {
    pub name: String,           // "beardog", "songbird", etc.
    pub version: String,        // "1.0.0"
    pub capabilities: Vec<String>,
    pub endpoint: Option<String>,  // If different from primary
}

pub enum AtomicType {
    Tower,      // BearDog + Songbird
    Node,       // Tower + Toadstool
    Nest,       // Tower + NestGate
    NUCLEUS,    // Tower + Node + Nest
}
```

**Benefits**:
- ✅ BiomeOS can discover complete ecosystems
- ✅ Graduated information disclosure (trust-based)
- ✅ Automatic primal coordination

---

## 📊 **Technical Debt to Address in Parallel**

### **Current Songbird Issues (From Audit)**

1. **306 TODO/FIXME comments** (71 informational, 235 actionable)
   - Priority: Clean up during refactoring
   - Effort: ~2-3 hours per week over 4 weeks

2. **254 `sleep` calls** (timing-based tests)
   - Priority: HIGH (Phase 1)
   - Effort: ~10 hours
   - Impact: 5x faster tests, better reliability

3. **70 files with `Arc<Mutex>`** (blocking locks)
   - Priority: MEDIUM
   - Effort: ~8 hours
   - Impact: Better async performance

4. **1 file over 1000 lines** (`connection_manager.rs`)
   - Priority: MEDIUM
   - Effort: 2-3 hours
   - Impact: Better maintainability

5. **Test coverage ~20%** (target 90%)
   - Priority: HIGH (Phase 2)
   - Effort: ~20 hours over 4 weeks
   - Impact: Production confidence

---

## 🎯 **Recommended 6-Week Roadmap**

### **Week 1: Concurrent Evolution** (10 hours)
- [ ] Create `concurrent_helpers.rs` (BearDog pattern)
- [ ] Refactor `sync_helpers.rs` (remove sleep)
- [ ] Replace `Arc<Mutex>` in critical paths
- **Deliverable**: Modern concurrent test patterns

### **Week 2: BirdSong v3.0 Foundation** (12 hours)
- [ ] Multi-tag support (`CallsignTag` type)
- [ ] Routing metadata schema
- [ ] Tag management API
- [ ] v2/v3 compatibility tests
- **Deliverable**: BirdSong v3.0 alpha

### **Week 3: Security Hardening** (15 hours)
- [ ] Key rotation protocol
- [ ] Replay protection (sequence numbers)
- [ ] Rate limiting (adaptive beaconing)
- [ ] Security audit
- **Deliverable**: BirdSong v3.0 beta

### **Week 4: BiomeOS Integration** (10 hours)
- [ ] Genesis ceremony CLI
- [ ] NUCLEUS metadata support
- [ ] Integration tests with BearDog
- [ ] Documentation
- **Deliverable**: LiveSpore-ready Songbird

### **Week 5: Test Coverage** (15 hours)
- [ ] Expand E2E tests (multi-tag scenarios)
- [ ] Chaos tests (tag rotation, NAT failure)
- [ ] Fault tests (replay attacks, beacon spam)
- [ ] Coverage analysis (`llvm-cov`)
- **Deliverable**: 90% test coverage

### **Week 6: Production Hardening** (8 hours)
- [ ] Performance benchmarks
- [ ] Production deployment guide
- [ ] Migration guide (v2 → v3)
- [ ] Final audit
- **Deliverable**: BirdSong v3.0 production release

**Total Effort**: ~70 hours (1.75 weeks full-time, or 6 weeks part-time)

---

## 🤝 **Coordination with BearDog Team**

### **BearDog's Responsibilities**

1. **Key Derivation API** (Week 2)
   ```rust
   POST /api/v1/lineage/derive-key
   {
     "genetic_lineage": "<hash>",
     "epoch": 12345,
     "purpose": "birdsong-encryption"
   }
   Response: { "key": "<32-byte-key>" }
   ```

2. **Genesis Ceremony Integration** (Week 4)
   - SoloKey witness verification
   - Hardware entropy collection
   - Genetic lineage generation

3. **Lineage Verification** (Week 4)
   - Already implemented: `POST /api/v1/lineage/verify`
   - May need enhancement for multi-tag scenarios

### **Joint Testing** (Week 5)

- Multi-tag discovery scenarios
- NAT traversal with genetic verification
- LiveSpore boot simulation
- Cross-primal integration tests

---

## 📈 **Expected Outcomes**

### **Quality Metrics**

| Metric | Current | After Evolution | Improvement |
|--------|---------|-----------------|-------------|
| Grade | A (92/100) | A+ (98/100) | +6 points |
| Test Coverage | ~20% | 90% | +70% |
| Test Speed | Baseline | 5x faster | 400% |
| Concurrency | Mixed | Modern | 100% |
| BirdSong Version | v2.0 | v3.0 | Major upgrade |
| Multi-Tag Support | No | Yes | New capability |
| Key Rotation | No | Yes | Security |
| Replay Protection | No | Yes | Security |

### **Capability Expansion**

✅ **Multi-Callsign Tags** - Support MSU, Personal, Federation tags  
✅ **Institutional NAT** - Route via university networks (zero cloud costs)  
✅ **LiveSpore Ready** - BiomeOS first-boot integration  
✅ **Key Rotation** - Automatic security enhancement  
✅ **Replay Protection** - Production-grade security  
✅ **90% Coverage** - Production confidence  

### **Production Impact**

1. **Cost Reduction**: Users can leverage institutional NAT instead of cloud
2. **Security Enhancement**: Key rotation + replay protection
3. **Better UX**: Multi-tag discovery (one node, many identities)
4. **LiveSpore Support**: First-boot personalization ready
5. **Production Confidence**: 90% test coverage

---

## 🚀 **Getting Started**

### **Immediate Next Steps** (This Week)

1. **Review this document** with Songbird team (30 min)
2. **Prioritize roadmap** - adjust based on team capacity (30 min)
3. **Copy BearDog's `concurrent_helpers.rs`** to Songbird (1 hour)
4. **Start Week 1 concurrent evolution** (8 hours remaining)

### **Resources**

**From BearDog** (ready to share):
- `beardog/tests/support/concurrent_helpers.rs` - Production concurrent test utilities
- `beardog/specs/current/security/LIVESPORE_FINAL_ARCHITECTURE.md` - Complete LiveSpore architecture
- `beardog/specs/current/security/HOT_PLUG_HSM_UPGRADE_SPECIFICATION.md` - HSM hierarchy

**From wateringHole**:
- `wateringHole/birdsong/BIRDSONG_PROTOCOL.md` - Current v2.0 spec
- Update needed: BirdSong v3.0 spec (create during Week 2)

### **Communication Channels**

- **Slack**: #songbird-evolution
- **Weekly Sync**: Songbird + BearDog teams (1 hour)
- **Shared Docs**: `wateringHole/birdsong/`
- **Issues**: Track progress in `songbird` and `beardog` repos

---

## 💡 **Key Insights for Songbird Team**

### **1. You're 80% There Already**

BirdSong v2.0 already supports most of what LiveSpore needs:
- ✅ Encrypted discovery (ChaCha20-Poly1305)
- ✅ Genetic lineage integration
- ✅ Public/private separation (`family_id` + encrypted payload)

**Just need**: Multi-tag support + routing metadata formalization

### **2. This is Parallel Evolution, Not Blocking**

You can evolve BirdSong v3.0 **while** BearDog continues on LiveSpore integration:
- Week 1-2: You work on concurrent evolution
- Week 2-3: You add multi-tag support
- Week 4: We integrate together

**No blockers** - we both ship when ready.

### **3. The Concurrent Evolution Pays Dividends**

BearDog's experience:
- 5x faster tests
- More reliable (event-driven vs timing)
- Better production patterns
- Easier debugging

**Investment**: 10 hours  
**Payoff**: Permanent improvement to development velocity

### **4. Test Coverage = Production Confidence**

Current ~20% coverage is risky for production:
- Untested code paths
- Unknown failure modes
- Hard to refactor safely

**Target**: 90% coverage  
**Benefit**: Confidence to ship LiveSpore-enabled Songbird to production

---

**Status**: 🎯 **READY TO EXECUTE**  
**Timeline**: 6 weeks (part-time) or 1.75 weeks (full-time)  
**Risk**: LOW (parallel evolution, backward compatible)  
**Impact**: HIGH (LiveSpore support, security, production quality)

🐦🌱 **Let's Build LiveSpore Together!**

---

## 📚 **Appendix: Example Multi-Tag Scenario**

### **Scenario: Graduate Student with Multiple Identities**

**User**: Alice (PhD student at MSU, also runs personal HPC at home)

**Tags Configured**:

1. **"MSU" Tag** (Institutional)
   - Public: Visible to all
   - Purpose: Use MSU network routing
   - Routing: Encrypted for family → `192.168.1.50:8080` (home HPC)
   - Priority: 100

2. **"AliceResearch" Tag** (Personal)
   - Public: Visible to all
   - Purpose: Personal research identity
   - Routing: Encrypted for family → `192.168.1.50:8080` (same HPC)
   - Priority: 90

3. **"ML-Federation" Tag** (Federation)
   - Public: Visible to all
   - Purpose: Distributed ML network
   - Routing: Encrypted for federation family → `192.168.1.50:8181` (ML service)
   - Priority: 80

### **BirdSong Broadcast** (v3.0):

```json
{
  "version": 3,
  "tags": [
    {
      "tag": "MSU",
      "purpose": "Institutional",
      "priority": 100
    },
    {
      "tag": "AliceResearch",
      "purpose": "Personal",
      "priority": 90
    },
    {
      "tag": "ML-Federation",
      "purpose": "Federation",
      "priority": 80
    }
  ],
  "encrypted_payload": {
    "ciphertext": "<encrypted-routing-metadata>",
    "nonce": "...",
    "algorithm": "ChaCha20-Poly1305",
    "key_epoch": 42
  },
  "sequence": 12345,
  "sender_id": "alice-node-001",
  "timestamp": 1735000000,
  "ttl": 300
}
```

### **Peer Discovery Scenarios**:

**Scenario A: Alice's Sibling (Same Genetic Family)**

```
1. Receives BirdSong packet
2. Sees tags: ["MSU", "AliceResearch", "ML-Federation"]
3. Uses genetic key to decrypt payload ✅
4. Reads routing: 192.168.1.50:8080
5. Connects via MSU network (tag priority 100)
6. Auto-trusted (same genetic family)
```

**Scenario B: Random MSU Student**

```
1. Receives BirdSong packet
2. Sees tags: ["MSU", "AliceResearch", "ML-Federation"]
3. Tries to decrypt with their genetic key ❌
4. Decryption fails (different genetic lineage)
5. Ignores packet
```

**Scenario C: ML Federation Member (Different Genetic Family, Same Federation)**

```
1. Receives BirdSong packet
2. Sees tag: "ML-Federation"
3. Uses federation key to decrypt payload ✅
4. Reads routing: 192.168.1.50:8181 (ML service, not personal HPC!)
5. Connects to ML service only
6. Graduated trust (federation member, not genetic family)
```

**Result**:
- ✅ One node, multiple identities
- ✅ Different routing per identity
- ✅ Genetic family gets full access
- ✅ Federation gets limited access (ML only)
- ✅ Random students get no access
- ✅ All via public tags (legitimate network use)

---

**End of Evolution Plan**

🐦🔑🌱 **Songbird + BearDog + BiomeOS = LiveSpore-Ready Ecosystem**

