# 🔐 Threshold Key Shares Demo

**Phase**: 4 (Advanced Features)  
**Demo**: 2/10  
**Priority**: 🔥🔥 HIGH  
**Status**: 🚧 IN PROGRESS

---

## 🎯 Overview

This demo demonstrates **threshold cryptography** using **Shamir's Secret Sharing** to split a genetic key into **N shares**, requiring **M-of-N shares** to reconstruct the key. This enables distributed trust, multi-party authorization, and disaster recovery scenarios.

### **What You'll Learn**
- Shamir's Secret Sharing for genetic keys
- M-of-N threshold signatures (e.g., 3-of-5, 2-of-3)
- Multi-party key ceremonies
- Distributed trust models
- Share distribution and secure storage
- Key reconstruction from threshold shares

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    THRESHOLD KEY CEREMONY                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. Generate Master Key                                       │
│     └─→ genetic-key-corporate-treasury                        │
│                                                               │
│  2. Split into N Shares (Shamir's Secret Sharing)            │
│     ├─→ Share 1 (CEO)                                         │
│     ├─→ Share 2 (CFO)                                         │
│     ├─→ Share 3 (CTO)                                         │
│     ├─→ Share 4 (COO)                                         │
│     └─→ Share 5 (Board Member)                                │
│                                                               │
│  3. Distribute Shares Securely                                │
│     └─→ Each shareholder receives encrypted share             │
│                                                               │
│  4. Threshold Operation (M-of-N required)                     │
│     └─→ Collect M shares (e.g., 3-of-5)                       │
│                                                               │
│  5. Reconstruct Key                                           │
│     └─→ Verify shares and reconstruct master key              │
│                                                               │
│  6. Perform Operation                                         │
│     └─→ Sign transaction, decrypt data, etc.                  │
│                                                               │
│  7. Audit Trail                                               │
│     └─→ Record which shares were used                         │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔐 Cryptographic Foundation

### **Shamir's Secret Sharing (SSS)**
- **Polynomial Construction**: Secret `S` as constant term of polynomial
- **Share Generation**: Evaluate polynomial at N distinct points
- **Threshold Reconstruction**: M shares → Lagrange interpolation → Secret
- **Information-Theoretic Security**: M-1 shares reveal ZERO information

### **Key Properties**
1. **Perfect Security**: Fewer than M shares are cryptographically useless
2. **Flexibility**: Any M-of-N combination works
3. **No Trusted Dealer** (in advanced variants): Multi-party computation
4. **Lineage Tracking**: Each share usage recorded in audit trail

---

## 📋 Use Cases

### 1. Corporate Key Management (3-of-5)
**Scenario**: $10M treasury transaction authorization  
**Threshold**: 3-of-5 executives (CEO, CFO, CTO, COO, Board)  
**Goal**: Prevent single-party misuse

### 2. Disaster Recovery (2-of-3)
**Scenario**: Backup key split across 3 geographic locations  
**Threshold**: 2-of-3 locations (NY, London, Tokyo)  
**Goal**: Survive single-site disaster

### 3. Secure Enclave (5-of-7)
**Scenario**: National security key ceremony  
**Threshold**: 5-of-7 senior officials  
**Goal**: Maximum security, no single point of compromise

---

## 🚀 Quick Start

### **Run the Demo**
```bash
cd showcase/04-advanced-features/02-threshold-key-shares
./run-demo.sh
```

### **With Custom Scenario**
```bash
./run-demo.sh scenarios/corporate_treasury.json configs/demo.toml
```

---

## 📊 Expected Performance

| Metric | Target | Typical |
|--------|--------|---------|
| **Share Generation** | < 50ms | ~10ms |
| **Key Reconstruction** | < 100ms | ~50ms |
| **Threshold Signature** | < 200ms | ~150ms |
| **Audit Overhead** | < 10ms | ~5ms |
| **Total Ceremony** | < 500ms | ~250ms |

---

## ✅ Validation Criteria

The demo validates:
1. ✅ **Share Generation**: N shares created from master key
2. ✅ **Threshold Enforcement**: M-1 shares fail, M shares succeed
3. ✅ **Key Reconstruction**: Original key perfectly reconstructed
4. ✅ **Audit Trail**: All share usage recorded
5. ✅ **Performance**: Sub-500ms total ceremony time

---

## 🔍 Security Properties

### **Guaranteed**
- ✅ **Perfect Secrecy**: M-1 shares reveal ZERO information
- ✅ **Tamper Detection**: Invalid shares rejected
- ✅ **Audit Trail**: Every reconstruction logged
- ✅ **Lineage Tracking**: Share provenance maintained

### **Threats Mitigated**
- ❌ Single-party compromise (requires M-of-N)
- ❌ Insider threat (no single person can act alone)
- ❌ Physical theft (partial shares are useless)
- ❌ Coercion (M-1 parties can resist)

---

## 📚 Technical Details

### **Implementation**
- **Library**: `sharks` (Rust SSS implementation)
- **Field**: GF(256) for byte-oriented secrets
- **Polynomial Degree**: M-1
- **Share Size**: Same as secret size
- **Reconstruction**: Lagrange interpolation

### **Genetic Key Integration**
- Master key split into shares
- Each share inherits genetic constraints
- Lineage tracked per-share
- Reconstruction validates constraints

---

## 🎓 Learning Objectives

After completing this demo, you will understand:
1. How Shamir's Secret Sharing works mathematically
2. How to implement threshold cryptography in production
3. How to design multi-party authorization workflows
4. How to audit and track threshold operations
5. How to integrate SSS with genetic key lineage

---

## 📖 Related Demos

- **Demo 01**: Multi-Primal Workflow (lineage tracking)
- **Demo 03**: Hardware Attestation Chain (secure share storage)
- **Demo 08**: Constraint Composition (advanced policies)

---

## 🔧 Configuration

See `configs/demo.toml` for:
- Threshold parameters (M, N)
- Share distribution strategy
- Audit logging settings
- Performance tuning

---

**Created**: December 26, 2025  
**Status**: 🚧 Implementation in progress

🐻 **BearDog: Distributed Trust Through Threshold Cryptography!** 🔐

