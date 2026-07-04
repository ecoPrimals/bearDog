# 🔐 BearDog Physical Genesis Bootstrap - Implementation Plan

> **Note (Wave 131):** Some internal references in this document point to archived paths from the HTTP-era API. Core concepts remain valid.

**Date**: December 22, 2025  
**Status**: Planning Complete → Implementation Starting  
**Timeline**: 4-5 weeks  
**Integration**: Songbird + BearDog Multi-Primal Genesis

---

## 🎯 Vision

**"Never let a bird be alone in the dark forest"**

Every new node receives cryptographic identity at birth via physical witness ceremony, not vulnerable internet bootstrap.

### Traditional vs. Physical Genesis

**Traditional** ❌:
```
New Node → Internet → Find server → Hope it's safe
```

**Physical Genesis** ✅:
```
New Node → SoloKey tap → Witnessed genesis → Multi-primal lineage
         ✅ Protected from first moment
         ✅ Born with cryptographic identity
         ✅ Never alone, never vulnerable
```

---

## 🏗️ Architecture Integration

### Existing Foundation (Already Built)

✅ **BirdSong Lineage** (`beardog-genetics/src/birdsong/`):
- `lineage_chain.rs` - Parent-child lineage chains
- `lineage_proof.rs` - Merkle-based lineage proofs
- `key_derivation.rs` - HKDF key derivation from lineage
- `manager.rs` - Orchestrates all lineage operations

✅ **Capability Framework** (`beardog-capabilities/`):
- Generic primal-agnostic trait system
- Discovery and registration
- Metadata and advertisements

✅ **Security Infrastructure** (`beardog-security/`):
- HSM abstraction
- Entropy hierarchy
- TPM integration

### New Modules (To Build)

🔴 **Genesis Lineage** (`beardog-genetics/src/birdsong/genesis.rs`):
- Establish genetic lineage for new nodes
- Integrate with existing `LineageChainManager`
- Create genesis-specific lineage from witness

🔴 **Witness Verification** (`beardog-security/src/genesis/witness.rs`):
- Verify witness signatures using genetic cryptography
- HSM integration for trusted witness store
- Ed25519 signature verification

🟡 **Physical Proof** (`beardog-security/src/genesis/physical_proof.rs`):
- Verify physical channel type (HardwareKey, QR, Bluetooth, NFC)
- Assign trust levels based on channel
- Hardware attestation verification

🟢 **Genesis Tunnel** (`beardog-tunnel/src/genesis_tunnel.rs`, OPTIONAL):
- Special BTSP tunnel for genesis ceremony
- Ephemeral genesis credentials exchange
- Upgrade to standard tunnel with lineage

---

## 📋 Core Types

### GenesisWitness

```rust
/// A device that witnesses the birth of a new node
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
#[zeroize(drop)]
pub struct GenesisWitness {
    /// Witness device identifier
    pub device_id: String,
    
    /// Witness public key (Ed25519)
    pub public_key: Vec<u8>,
    
    /// Physical channel used
    pub physical_channel: PhysicalChannelType,
    
    /// Timestamp of genesis ceremony (Unix timestamp)
    pub timestamp: u64,
    
    /// Signature over new node's identity
    #[zeroize(skip)]
    pub signature: Vec<u8>,
}
```

### PhysicalChannelType

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhysicalChannelType {
    /// Hardware security key (SoloKey, YubiKey) - ⭐⭐⭐⭐⭐
    HardwareKey,
    
    /// QR code + out-of-band verification - ⭐⭐⭐⭐
    QrCodeWithOob,
    
    /// Bluetooth pairing - ⭐⭐⭐
    Bluetooth,
    
    /// NFC tap - ⭐⭐⭐⭐
    Nfc,
}
```

### GeneticLineage

```rust
/// Genetic cryptographic lineage for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticLineage {
    /// Node's genetic identity
    pub genetic_id: Vec<u8>,
    
    /// Lineage chain (from ancestor to this node)
    pub lineage_chain: Vec<LineageProof>,
    
    /// Genesis witness who created this lineage
    pub genesis_witness: GenesisWitness,
    
    /// Birth timestamp (Unix timestamp)
    pub birth_timestamp: u64,
}
```

### TrustLevel

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// ⭐ - Untrusted
    Low = 1,
    /// ⭐⭐⭐ - Medium trust (Bluetooth)
    Medium = 3,
    /// ⭐⭐⭐⭐ - High trust (QR+OOB, NFC)
    High = 4,
    /// ⭐⭐⭐⭐⭐ - Maximum trust (Hardware key)
    Maximum = 5,
}
```

---

## 🔄 Integration Flow

### Multi-Primal Genesis Ceremony

```
1. User taps SoloKey on new Pixel 8a
   ↓
2. Songbird initiates genesis ceremony
   ↓
3. Songbird → BearDog: POST /genesis/lineage
   {
     "new_node_id": "pixel-8a-abc123",
     "witness": { ... }
   }
   ↓
4. BearDog verifies:
   - Witness signature ✓
   - Physical channel proof ✓
   - Witness authority ✓
   ↓
5. BearDog creates genetic lineage for new node
   - Generates genetic ID
   - Creates lineage chain from witness
   - Signs with BearDog's genesis authority
   ↓
6. BearDog → Songbird: Returns signed lineage
   {
     "genetic_lineage": { ... },
     "status": "established"
   }
   ↓
7. Songbird combines lineages from all primals
   ↓
8. New node receives unified genesis certificate
   ↓
9. New node is born with full identity! ✅
```

---

## 📊 Implementation Timeline

### Week 1: Foundation & API Design (In Progress)
- ✅ Review handoff document
- ✅ Create implementation plan
- 🔄 Define shared types
- 🔄 Create module structure
- 🔄 Design REST API endpoints
- 🔄 Update root documentation

**Deliverables**:
- Module structure created
- Type definitions complete
- API contract agreed with Songbird
- Mock implementations for testing

### Week 2-3: Core Implementation
**Week 2: Genesis Lineage**
- Implement `GenesisLineageProvider`
- Integrate with existing `LineageChainManager`
- Genetic ID generation
- Lineage chain creation
- Storage layer

**Week 3: Witness + Physical Verification**
- Implement `GenesisWitnessVerifier`
- Ed25519 signature verification
- HSM integration for trusted witnesses
- Implement `PhysicalProximityVerifier`
- Trust level assignment by channel

**Deliverables**:
- `beardog-genetics/src/birdsong/genesis.rs` - Complete
- `beardog-security/src/genesis/witness.rs` - Complete
- `beardog-security/src/genesis/physical_proof.rs` - Complete
- Unit tests passing

### Week 4: Integration & Testing
- API server endpoints for genesis
- Integration with capability registry
- Showcase examples
- Mock-based integration tests with Songbird
- Performance benchmarks

**Deliverables**:
- Genesis API endpoints live
- Showcase demo scripts working
- Integration tests passing

### Week 5: Real Crypto & Polish
- Switch from mocks to real genetic cryptography
- End-to-end genesis ceremony with Songbird
- Optional: Genesis tunnel support
- Documentation finalization
- Production readiness review

**Deliverables**:
- Real genesis ceremony working
- Documentation complete
- Production-ready code

---

## 🧪 Testing Strategy

### Local Testing First
```bash
# Terminal 1: Start BearDog with genesis support
cargo run --release --features genesis-api --example genesis_server

# Terminal 2: Test genesis lineage endpoint
curl -X POST http://localhost:9000/genesis/lineage \
  -H "Content-Type: application/json" \
  -d '{
    "new_node_id": "test-node-001",
    "witness": {
      "device_id": "solokey-abc123",
      "public_key": "...",
      "physical_channel": "HardwareKey",
      "timestamp": 1735000000,
      "signature": "..."
    }
  }'
```

### Test Progression
1. **Week 1-2**: Unit tests with mock data
2. **Week 3**: Mock integration tests (BearDog ↔ Songbird)
3. **Week 4**: Real crypto with test keys
4. **Week 5**: End-to-end with real hardware (SoloKey)

### Test Coverage Goals
- Unit tests: 90%+ for new genesis modules
- Integration tests: All API endpoints
- E2E tests: Multi-primal genesis ceremony
- Security tests: Invalid witness, expired signatures, wrong channel

---

## 🎯 Success Criteria

### Week 5 Goal

```bash
# User taps SoloKey on new device
# → Songbird + BearDog coordinate genesis
# → New node receives:

✅ Genetic cryptographic identity (from BearDog)
✅ Federation membership (from Songbird)
✅ Unified genesis certificate
✅ Full lineage from birth
✅ Never vulnerable to network attacks
✅ Protected from first moment of existence
```

### Technical Criteria

**Functional**:
- ✅ Genesis lineage established via witness
- ✅ Witness signatures verified (Ed25519)
- ✅ Physical channel proof validated
- ✅ Trust levels correctly assigned
- ✅ Lineage stored and retrievable
- ✅ API endpoints functional
- ✅ Integration with Songbird working

**Security**:
- ✅ Cannot forge witness signatures
- ✅ Cannot replay old genesis certificates
- ✅ Physical channel attestation validated
- ✅ HSM-backed witness authority
- ✅ Secrets properly zeroized
- ✅ No lineage can be backdated

**Performance**:
- ✅ Genesis ceremony < 500ms (local)
- ✅ Witness verification < 50ms
- ✅ Physical proof verification < 10ms
- ✅ Lineage generation < 100ms

**Quality**:
- ✅ 90%+ test coverage on new modules
- ✅ All clippy warnings resolved
- ✅ Full API documentation
- ✅ Showcase examples working
- ✅ Integration guide for Songbird

---

## 🤝 Coordination with Songbird

### Week 1: API Design
- **Monday**: Share type definitions
- **Wednesday**: Agree on REST API contract
- **Friday**: Document error cases and edge cases

### Week 2-3: Parallel Development
- **Weekly sync**: Progress updates, blockers
- **Async**: Slack/Discord for quick questions
- **Shared**: Mock data for testing

### Week 4: Integration
- **Integration sprint**: BearDog + Songbird working together
- **Joint testing**: Multi-primal genesis ceremony
- **Showcase demos**: End-to-end with SoloKey

### Week 5: Polish & Deploy
- **Performance testing**: Measure latencies
- **Security review**: Audit new code
- **Documentation**: Finalize integration guide
- **Deploy prep**: Production readiness checklist

---

## 📁 File Structure

```
beardog/
├── crates/
│   ├── beardog-genetics/
│   │   └── src/
│   │       └── birdsong/
│   │           ├── genesis.rs         ← NEW (Genesis lineage)
│   │           ├── lineage_chain.rs   ← EXISTS (reuse)
│   │           ├── lineage_proof.rs   ← EXISTS (reuse)
│   │           └── manager.rs         ← ENHANCE (add genesis)
│   │
│   ├── beardog-security/
│   │   └── src/
│   │       └── genesis/               ← NEW MODULE
│   │           ├── mod.rs
│   │           ├── witness.rs         ← NEW (Witness verification)
│   │           └── physical_proof.rs  ← NEW (Physical channel)
│   │
│   └── beardog-tunnel/
│       └── src/
│           └── genesis_tunnel.rs      ← NEW (Optional)
│
├── examples/
│   └── genesis_server.rs              ← NEW (Run genesis API)
│
└── showcase/                               ← Fossilized (Wave 49)
            ├── README.md
            ├── 00-START_GENESIS_SERVER.sh
            ├── 01-establish-genesis-lineage.sh
            ├── 02-verify-witness.sh
            └── 03-multi-primal-ceremony.sh
```

---

## 🚀 Next Steps (Week 1)

### Immediate Actions (Next 2-3 hours)

1. ✅ **Create module structure**
2. ✅ **Define types in beardog-genetics**
3. ✅ **Sketch genesis lineage API**
4. ✅ **Update root docs with genesis roadmap**
5. 🔄 **Coordinate with Songbird on API contract**

### This Week

- [ ] Implement `GenesisLineageProvider` (mock first)
- [ ] Implement `GenesisWitnessVerifier` (mock first)
- [ ] Create genesis API endpoints
- [x] ~~Write showcase examples~~ (showcase fossilized Wave 49)
- [ ] Integration tests with mocks

---

## 🔗 Related Documents

**Architecture**:
- `../../specs/current/architecture/CAPABILITY_BASED_PRIMAL_INTERACTION.md` - Primal interaction patterns

**Implementation**:
- See wateringHole handoffs for phase evolution plans.

**Specs**:
- `specs/LINEAGE_GATED_RELAY_PROTOCOL.md` - Lineage concepts
- `specs/BIRDSONG_PROTOCOL.md` - Privacy-preserving discovery

---

## 💡 Key Insights

### Why This Matters

**Traditional P2P bootstrap**:
- ❌ Trust infrastructure (DNS, servers)
- ❌ Vulnerable during discovery
- ❌ Weak initial identity
- ❌ Alone and exposed

**Physical genesis**:
- ✅ Trust physics + cryptography
- ✅ Protected from first moment
- ✅ Strong genetic lineage from birth
- ✅ Multi-primal witness coordination
- ✅ Never alone, never vulnerable

### The BearDog Contribution

BearDog provides the **genetic cryptographic foundation** for genesis:
- Genetic ID generation
- Lineage chain establishment
- Witness signature verification
- Physical channel attestation
- Trust level assignment

This transforms Songbird's genesis from "network bootstrap" to "witnessed birth ceremony."

---

## 🎉 The Vision

**"Every node is born witnessed, protected, and sovereign"**

No node faces the dark forest alone.  
No node trusts the internet at birth.  
Every node has cryptographic lineage from its first moment.

**Physical Genesis: The Right Way to Birth a Node!** ✨

---

**Status**: Implementation Starting (Week 1)  
**Next**: Create module structure and type definitions  
**Coordination**: Songbird team weekly sync  
**Timeline**: 4-5 weeks to full deployment

🔐🐻🎵 **Genesis + Lineage + Federation = Sovereign Birth!**

