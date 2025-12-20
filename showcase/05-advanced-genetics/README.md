# 🧬 Phase 5: Advanced Genetics

**Focus:** Threshold Cryptography, Hierarchical Keys, Advanced Constraints  
**Status:** 🚧 Building  
**Target:** Verify advanced genetic cryptography claims

---

## 🎯 Overview

This phase demonstrates BearDog's most advanced genetic cryptography features:
- **Threshold Cryptography** - N-of-M key schemes (3-of-5, 7-of-11, etc.)
- **Complex Hierarchies** - Multi-level key derivation with inheritance
- **Advanced Constraints** - Time, resource, context-aware key limits
- **Automated Rotation** - Self-rotating keys with lineage tracking
- **Visual Lineage** - Interactive key family tree visualization

---

## 📋 Planned Demonstrations

### **Demo 1: N-of-M Threshold Cryptography** (~20 min)
**Script:** `demos/01-threshold-nofm.sh`  
**Status:** 🚧 IN PROGRESS

**What It Proves:**
- Create 3-of-5 threshold scheme (requires 3 of 5 keys)
- Create 7-of-11 scheme (requires 7 of 11 keys)
- Any M keys can reconstruct secret
- Fewer than M keys reveals nothing
- Real-world multi-signature scenarios

**Use Cases:**
- Corporate board approval (3-of-5 executives)
- Cryptocurrency wallets (7-of-11 signers)
- Nuclear launch codes (2-of-3 generals)
- Emergency access (5-of-9 administrators)

---

### **Demo 2: Complex Hierarchical Keys** (~20 min)
**Script:** `demos/02-hierarchical-keys.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- Multi-level key hierarchies (5+ generations)
- Root → Branch → Leaf inheritance
- Constraints propagate down tree
- Revoke entire branch with one key
- Department → Team → Individual structure

**Use Cases:**
- Enterprise org structure
- Delegated authority chains
- Multi-tenant systems
- Hierarchical access control

---

### **Demo 3: Advanced Key Constraints** (~20 min)
**Script:** `demos/03-advanced-constraints.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- Time constraints (valid 9AM-5PM only)
- Resource constraints (max 100 MB/s)
- Geographic constraints (US-only)
- Context constraints (mobile device only)
- Composite constraints (ALL must be met)

**Use Cases:**
- Business hours only keys
- Rate-limited API keys
- Compliance-bound keys
- Device-specific keys

---

### **Demo 4: Automated Key Rotation** (~15 min)
**Script:** `demos/04-key-rotation.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- Automatic key rotation (every 24h)
- Zero-downtime rotation
- Old keys remain valid for grace period
- Full lineage tracking
- Seamless client updates

**Use Cases:**
- Security best practices
- Compliance requirements
- Breach mitigation
- Long-running services

---

### **Demo 5: Key Lineage Visualization** (~15 min)
**Script:** `demos/05-lineage-visualization.sh`  
**Status:** 📋 PLANNED

**What It Proves:**
- Interactive family tree of keys
- Show inheritance paths
- Highlight constraints per key
- Trace revocation impact
- Export lineage graph

**Use Cases:**
- Security audits
- Compliance reporting
- Forensic analysis
- Understanding key relationships

---

## 🏗️ Architecture Demonstrated

### **Threshold Cryptography:**
```
Secret S split into 5 shares: [S1, S2, S3, S4, S5]
Threshold M = 3

Any 3 shares reconstruct S:
  S1 + S2 + S3 = S ✅
  S2 + S4 + S5 = S ✅
  S1 + S3 + S5 = S ✅

Any 2 shares reveal nothing:
  S1 + S2 = ??? ❌
  S3 + S5 = ??? ❌

Security: Need M shares minimum
Flexibility: Total shares can be N > M
```

### **Hierarchical Keys:**
```
Root Key (Gen 0)
    ├─ Dept A (Gen 1)
    │   ├─ Team 1 (Gen 2)
    │   │   ├─ Alice (Gen 3)
    │   │   └─ Bob (Gen 3)
    │   └─ Team 2 (Gen 2)
    │       └─ Charlie (Gen 3)
    └─ Dept B (Gen 1)
        └─ Team 3 (Gen 2)
            └─ Diana (Gen 3)

Revoke Dept A → All children revoked
Constraint on Root → Inherited by all
```

### **Advanced Constraints:**
```
Composite Constraint:
  AND (
    Time: 9AM-5PM EST,
    Resource: < 100 requests/min,
    Geography: US-only,
    Device: Mobile only
  )

ALL conditions must be met for key to work
Any violation → Operation denied
```

---

## 🎯 Claims to Verify

From `specs/current/genetics/` and related:

| Claim | Demo | Priority |
|-------|------|----------|
| N-of-M threshold schemes | Demo 1 | HIGH |
| Shamir's Secret Sharing | Demo 1 | HIGH |
| Multi-level hierarchies | Demo 2 | HIGH |
| Constraint inheritance | Demo 2 | HIGH |
| Branch revocation | Demo 2 | HIGH |
| Time-bound constraints | Demo 3 | MEDIUM |
| Resource constraints | Demo 3 | MEDIUM |
| Context-aware constraints | Demo 3 | MEDIUM |
| Automated rotation | Demo 4 | MEDIUM |
| Zero-downtime rotation | Demo 4 | HIGH |
| Lineage visualization | Demo 5 | LOW |
| Family tree export | Demo 5 | LOW |

---

## 🚀 Running the Demos

### **Prerequisites:**
```bash
cd /home/eastgate/Development/ecoPrimals/beardog
cargo build --workspace
```

### **Run Demo 1 (Threshold):**
```bash
cd showcase/05-advanced-genetics
./demos/01-threshold-nofm.sh          # Interactive
./demos/01-threshold-nofm.sh --auto   # Automatic
```

### **Run All Genetics Demos:**
```bash
./run-all-genetics-demos.sh
```

---

## 💡 Why This Matters

### **Threshold Cryptography:**
- **No Single Point of Failure** - Need M of N keys
- **Flexible Security** - Choose M and N per use case
- **Real Multi-Sig** - Not just multiple signatures, actual secret sharing
- **Quantum Resistant** - Can increase N without changing M

### **Hierarchical Keys:**
- **Organizational Structure** - Mirrors real org charts
- **Delegated Authority** - Safely delegate without losing control
- **Easy Revocation** - Revoke entire branches instantly
- **Audit Trail** - Full lineage for compliance

### **Advanced Constraints:**
- **Fine-Grained Control** - Precise limits on key usage
- **Compliance** - Meet regulatory requirements
- **Risk Mitigation** - Limit blast radius of compromise
- **Context-Aware** - Keys adapt to usage context

---

## 📊 Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Threshold schemes (N-of-M) | ≥2 | 🚧 |
| Hierarchy depth | ≥5 levels | 📋 |
| Constraint types | ≥8 | 📋 |
| Rotation scenarios | ≥3 | 📋 |
| Lineage visualization | Yes | 📋 |
| Auto-mode support | 100% | 🎯 |

---

## 🔗 Related Specifications

- `specs/current/genetics/GENETIC_KEY_EXCHANGE.md`
- `specs/current/genetics/HIERARCHICAL_KEYS.md`
- `specs/current/genetics/THRESHOLD_CRYPTOGRAPHY.md`
- `specs/current/genetics/KEY_CONSTRAINTS.md`

---

**🧬 Advanced Genetics: Building the Future of Key Management**

*Threshold. Hierarchical. Constrained. Rotating. Sovereign.*

