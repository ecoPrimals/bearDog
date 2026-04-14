# BearDog Scope and Boundaries Specification

**Version:** 2.0  
**Date:** March 2026 (originally January 2025)  
**Status:** ECOSYSTEM ALIGNED — Responsibility matrix refreshed March 2026  
**Purpose:** Clear boundary definition between BearDog and other ecosystem primals

## 🎯 **Executive Summary**

BearDog operates as a **Security Provider** within the ecoPrimals ecosystem, NOT as a standalone server. This document clarifies BearDog's true scope versus dependencies on other ecosystem components to prevent scope creep and ensure proper responsibility allocation.

## 🔒 **BearDog's Core Responsibilities**

### **PRIMARY SCOPE: Security Provider**
BearDog is responsible for:

#### ✅ **Cryptographic Operations**
- Encryption/decryption services
- Digital signature generation and verification
- Key management and HSM integration
- Cryptographic proof generation

#### ✅ **Authentication & Authorization**
- User authentication (biometric, MFA, Ed25519)
- Authorization proof generation
- Cross-node permission validation
- Session security management

#### ✅ **Compliance & Audit**
- GDPR, HIPAA, SOX compliance monitoring
- Audit logging and reporting
- Regulatory compliance validation
- Security policy enforcement

#### ✅ **Threat Detection & Response**
- ML-enhanced security monitoring
- Anomaly detection and behavioral analysis
- Security incident response
- Real-time threat mitigation

#### ✅ **Genetic Security Evolution**
- Genetic spawning algorithms
- Security trait inheritance and evolution
- Adaptive cryptographic strategies
- Self-healing security mechanisms

## 🚫 **NOT BearDog's Responsibility**

### **Network Infrastructure (SongBird's Domain)**
- ❌ Service discovery and registration
- ❌ Load balancing and routing
- ❌ Network topology management
- ❌ NAT traversal and connection management
- ❌ Peer-to-peer networking protocols

### **Compute Orchestration (ToadStool's Domain)**
- ❌ Compute resource allocation
- ❌ Universal platform execution
- ❌ Cross-platform deployment
- ❌ Resource scheduling and optimization

### **Data Storage (NestGate's Domain)**
- ❌ ZFS storage management
- ❌ Data federation and backup
- ❌ Storage pool management
- ❌ Data replication strategies

### **AI/MCP Services (Squirrel's Domain)**
- ❌ MCP protocol implementation
- ❌ Plugin management systems
- ❌ AI model execution
- ❌ Context management workflows

### **Operating System (biomeOS Domain)**
- ❌ Container orchestration
- ❌ System service management
- ❌ Resource containerization
- ❌ OS-level process management

## 🔗 **Integration Boundaries**

### **BearDog ↔ SongBird Integration**
```yaml
beardog_provides:
  - Security services via standard traits
  - Authentication tokens and proofs
  - Authorization validation services

songbird_provides:
  - Service discovery and registration
  - Request routing to BearDog instances
  - Load balancing across BearDog nodes
  - Network health monitoring

integration_point:
  - BearDog implements SecurityProvider trait
  - SongBird handles all network communications
  - Clean API boundary with no direct networking
```

### **BearDog ↔ ToadStool Integration**
```yaml
beardog_provides:
  - Cryptographic compute authorization
  - Security genetics for hybrid spawning
  - Secure execution environment validation

toadstool_provides:
  - Compute resource orchestration
  - Universal platform execution
  - Resource allocation optimization

integration_point:
  - BearDog authorizes compute operations
  - ToadStool executes authorized operations
  - Genetic spawning creates BearDog+ToadStool hybrids
```

### **BearDog ↔ NestGate Integration**
```yaml
beardog_provides:
  - Data encryption/decryption services
  - Key management for storage operations
  - Access control and permissions

nestgate_provides:
  - ZFS storage backend services
  - Data federation and backup
  - Storage pool management

integration_point:
  - BearDog implements KeyManager trait
  - NestGate uses BearDog for all crypto operations
  - Clean separation: storage vs security
```

## 📋 **Responsibility Matrix**

### **✅ BearDog — Completed (as of March 2026)**
- ~~Ring crypto library integration~~ → Eliminated: 100% Pure Rust (RustCrypto suite, zero C deps)
- ~~Ed25519 signature verification~~ → Complete: full Ed25519 sign/verify via `crypto.sign_ed25519` / `crypto.verify_ed25519`
- ~~Advanced cryptographic algorithms~~ → Complete: 100 JSON-RPC methods (ChaCha20-Poly1305, X25519, BLAKE3, Argon2id, post-quantum ML-KEM/ML-DSA/SPHINCS+, Tor ntor)
- ~~Security test suite~~ → 14,784+ tests, 90.51% line coverage, 14 crypto fault injection tests
- ~~Key rotation automation~~ → Complete: `key_rotation_manager.rs` with policy-driven rotation
- ~~HSM configuration~~ → Complete: Software, PKCS#11, StrongBox backends via `HsmManager`
- ~~Genetic spawning cryptographic proof generation~~ → Complete: `genetic.*` methods, lineage key derivation, beacon seeds

### **🔮 BearDog — Future Work**
- HSM hot-reload (runtime backend switching without restart)
- ML-enhanced threat detection integration (deferred to skunkBat coordination)
- Biometric authentication (deferred to mobile platform maturity)
- Real-time compliance monitoring (deferred to ecosystem-wide observability)

### **❌ NOT BearDog's Responsibility (Other Primals)**
- Service discovery and mesh routing (Songbird)
- Compute orchestration (ToadStool)
- Storage federation (NestGate)
- MCP/AI protocol (Squirrel)
- Container orchestration (biomeOS)

## 🎯 **Scope Enforcement Guidelines**

### **When Someone Asks BearDog Team to Implement Network Features:**
**Response:** "That's SongBird's domain. BearDog provides security services that SongBird consumes via standard traits. Please coordinate with the SongBird team for network-level features."

### **When Someone Asks BearDog Team to Implement Storage Features:**
**Response:** "That's NestGate's domain. BearDog provides encryption and key management services that NestGate consumes. Please coordinate with the NestGate team for storage-level features."

### **When Someone Asks BearDog Team to Implement Compute Features:**
**Response:** "That's ToadStool's domain. BearDog provides security authorization for compute operations that ToadStool executes. Please coordinate with the ToadStool team for compute-level features."

## 🚀 **Network Effects Through Boundaries**

BearDog achieves exponential value through **clean integration boundaries**:

1. **Security + Compute = Hybrid Intelligence**
   - BearDog security genetics + ToadStool compute genetics
   - Creates capabilities impossible with either alone
   - Network effects through ecosystem diversity

2. **Security + Storage = Trusted Federation**
   - BearDog encryption + NestGate federation
   - Secure cross-node data sharing
   - Trust without central authorities

3. **Security + Discovery = Distributed Trust**
   - BearDog authorization + SongBird orchestration
   - Cryptographic proof-based networking
   - Zero-trust distributed architecture

## 🔒 **Sovereignty & Human Dignity**

BearDog maintains sovereignty by:
- **Self-aware cryptographic keys** (keys ARE the authority)
- **Decentralized trust** (no central validation)
- **Ecosystem participation** (not ecosystem dependency)
- **Clear boundaries** (security provider, not service consumer)

This ensures humans retain control over their security while benefiting from ecosystem network effects.

---

**Status**: All boundary-violating code refactored. BearDog operates strictly within its crypto domain. Integration with other primals occurs solely via JSON-RPC capability discovery at runtime.