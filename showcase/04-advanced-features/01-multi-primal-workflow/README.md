# 🌐 Multi-Primal Workflow Demo

**Status**: 🚧 IN PROGRESS  
**Priority**: 🔥🔥🔥 CRITICAL  
**Time**: 6-8 hours

---

## 🎯 What This Demo Proves

### Core Achievement
**Complete end-to-end workflow** across ALL ecosystem primals using **capability-based discovery** with **zero mocks** and **comprehensive audit trails**.

### Validates
- ✅ Capability-based discovery in production
- ✅ Cross-primal integration (5 services)
- ✅ Distributed lineage tracking
- ✅ Zero-knowledge operations
- ✅ Performance at scale
- ✅ Comprehensive auditing

---

## 🌊 Workflow Overview

### The Journey of a User Request

```
1. User Request
   "Analyze this sensitive document and store the encrypted result"
   
   ↓

2. BearDog (Security)
   - Generate genetic key with purpose constraints
   - Enforce privacy policy
   - Create audit log entry
   
   ↓

3. Squirrel (AI Routing)
   - Sanitize PII from request
   - Route to AI service (privacy-preserving)
   - Return anonymized analysis
   
   ↓

4. NestGate (Encrypted Storage)
   - Encrypt analysis with BearDog key
   - Store with content-addressing
   - Return storage receipt
   
   ↓

5. Toadstool (Encrypted Compute)
   - Perform computation on encrypted data
   - Zero-knowledge processing
   - Return encrypted results
   
   ↓

6. Songbird (Orchestration)
   - Coordinate multi-node operation
   - Manage service discovery
   - Handle fault tolerance
   
   ↓

7. BearDog (Verification & Audit)
   - Verify lineage across all operations
   - Generate comprehensive audit receipt
   - Validate policy compliance
   - Report performance metrics
```

---

## 🎓 What You'll Learn

### 1. Capability-Based Discovery at Scale
- How to discover multiple services by capability
- Fallback strategies when services unavailable
- Health checking and service selection
- Load balancing across multiple providers

### 2. Cross-Primal Integration
- Coordinating operations across 5+ services
- Maintaining lineage through distributed operations
- Error handling in distributed systems
- Transaction-like semantics without distributed transactions

### 3. Zero-Knowledge Operations
- Processing sensitive data without exposure
- Privacy-preserving AI analysis
- Encrypted compute (process without decryption)
- Audit trails without revealing data

### 4. Production-Grade Patterns
- Service mesh integration
- Circuit breaker patterns
- Graceful degradation
- Comprehensive monitoring

---

## 🏗️ Architecture

### Services Involved

```
┌─────────────┐
│   User      │
└──────┬──────┘
       │
       ↓
┌─────────────────────────────────────────┐
│         BearDog (Security)              │
│  - Key Generation                       │
│  - Policy Enforcement                   │
│  - Audit Logging                        │
└──────┬──────────────────────────────────┘
       │
       ├──────────────────────┬──────────────────────┬──────────────────────┐
       ↓                      ↓                      ↓                      ↓
┌──────────────┐      ┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│  Squirrel    │      │  NestGate    │      │  Toadstool   │      │  Songbird    │
│  (AI)        │      │  (Storage)   │      │  (Compute)   │      │  (Orchestr.) │
│              │      │              │      │              │      │              │
│ Capability:  │      │ Capability:  │      │ Capability:  │      │ Capability:  │
│ "ai"         │      │ "storage"    │      │ "compute"    │      │ "orchestr."  │
└──────────────┘      └──────────────┘      └──────────────┘      └──────────────┘
       │                      │                      │                      │
       └──────────────────────┴──────────────────────┴──────────────────────┘
                                      │
                                      ↓
                              ┌──────────────┐
                              │  BearDog     │
                              │  (Audit)     │
                              └──────────────┘
```

### Discovery Flow

```rust
// BearDog discovers ALL services by capability
let orchestrator = discovery.find_by_capability("orchestration").await?;
let ai_service = discovery.find_by_capability("ai").await?;
let storage_service = discovery.find_by_capability("storage").await?;
let compute_service = discovery.find_by_capability("compute").await?;

// Works with ANY service providing these capabilities!
// No hardcoded "Songbird", "Squirrel", "NestGate", "Toadstool"
```

---

## 📋 Scenario Example

### Input
```json
{
  "user_id": "alice@example.com",
  "operation": "analyze_and_store",
  "data": {
    "content": "Sensitive medical records...",
    "metadata": {
      "patient_id": "12345",
      "date": "2025-12-26"
    }
  },
  "policies": {
    "purpose": "medical_research",
    "retention": "90_days",
    "allowed_operations": ["analyze", "store"],
    "prohibited_operations": ["export", "print"]
  }
}
```

### Expected Output
```json
{
  "status": "success",
  "operation_id": "multi-primal-op-abc123",
  "steps": [
    {
      "service": "beardog",
      "operation": "key_generation",
      "duration_ms": 5.2,
      "result": "genetic_key_xyz789"
    },
    {
      "service": "squirrel (discovered)",
      "operation": "ai_analysis",
      "duration_ms": 145.7,
      "result": "analysis_id_def456",
      "privacy": "pii_sanitized"
    },
    {
      "service": "nestgate (discovered)",
      "operation": "encrypted_storage",
      "duration_ms": 8.3,
      "result": "storage_id_ghi789"
    },
    {
      "service": "toadstool (discovered)",
      "operation": "encrypted_compute",
      "duration_ms": 234.6,
      "result": "compute_id_jkl012"
    },
    {
      "service": "songbird (discovered)",
      "operation": "multi_node_coordination",
      "duration_ms": 12.1,
      "nodes_coordinated": 3
    }
  ],
  "lineage": {
    "master_key": "genetic_key_xyz789",
    "derived_keys": ["storage_key_aaa", "compute_key_bbb"],
    "operations_tracked": 5,
    "audit_trail_hash": "blake3_ccc..."
  },
  "audit_receipt": {
    "operation_hash": "blake3_ddd...",
    "timestamp": "2025-12-26T10:30:45Z",
    "verified": true,
    "compliance": ["HIPAA", "GDPR"]
  },
  "performance": {
    "total_duration_ms": 406.9,
    "services_contacted": 5,
    "discovery_overhead_ms": 23.5,
    "lineage_verification_ms": 3.2
  }
}
```

---

## 🚀 How to Run

### Prerequisites
```bash
# Ensure all ecosystem services are available
# Option 1: Real services (recommended)
# - Songbird running at discovered endpoint
# - NestGate, Toadstool, Squirrel available
# 
# Option 2: Environment-based discovery
export PRIMAL_SONGBIRD_ENDPOINT="http://localhost:9090"
export PRIMAL_SONGBIRD_CAPABILITIES="orchestration,federation"
export PRIMAL_NESTGATE_ENDPOINT="http://localhost:8080"
export PRIMAL_NESTGATE_CAPABILITIES="storage,encryption"
export PRIMAL_TOADSTOOL_ENDPOINT="http://localhost:7070"
export PRIMAL_TOADSTOOL_CAPABILITIES="compute,gpu"
export PRIMAL_SQUIRREL_ENDPOINT="http://localhost:6060"
export PRIMAL_SQUIRREL_CAPABILITIES="ai,analytics,routing"
```

### Run Demo
```bash
# Build
cargo build --release

# Run with example scenario
./target/release/multi-primal-workflow \
  --scenario scenarios/medical_analysis.json \
  --config configs/demo.toml \
  --verbose

# Or use the convenience script
./run-demo.sh
```

---

## 📊 Performance Targets

| Metric | Target | Stretch Goal |
|--------|--------|--------------|
| Total Duration | <500ms | <300ms |
| Discovery Overhead | <30ms | <15ms |
| Services Contacted | 5+ | 5+ |
| Lineage Verification | <5ms | <2ms |
| Audit Receipt Generation | <10ms | <5ms |

---

## 🎯 Validates

### Spec Claims
- [ ] `UNIVERSAL_ADAPTER_SPECIFICATION.md` - Cross-primal operations
- [ ] `ZERO_HARDCODING_SPECIFICATION.md` - No hardcoded services
- [ ] `CAPABILITY_DISCOVERY_SPECIFICATION.md` - Runtime discovery
- [ ] `GENETIC_KEYS_SPECIFICATION.md` - Lineage tracking
- [ ] `AUDIT_SPECIFICATION.md` - Comprehensive audit trails

### Architectural Patterns
- [ ] Capability-based discovery (production)
- [ ] Service mesh integration
- [ ] Distributed lineage tracking
- [ ] Zero-knowledge operations
- [ ] Graceful degradation
- [ ] Circuit breaker patterns
- [ ] Health checking
- [ ] Load balancing

---

## 📋 Files

```
01-multi-primal-workflow/
├── README.md                      - This file
├── Cargo.toml                     - Dependencies
├── src/
│   └── main.rs                    - Main workflow implementation
├── configs/
│   └── demo.toml                  - Configuration
├── scenarios/
│   ├── medical_analysis.json      - Healthcare scenario
│   ├── financial_audit.json       - Finance scenario
│   └── research_workflow.json     - Research scenario
├── run-demo.sh                    - Convenience script
└── PERFORMANCE_RESULTS.md         - Benchmark results (generated)
```

---

**Status**: 🚧 Under Construction  
**Started**: December 26, 2025  
**Expected Completion**: December 27, 2025

🐻 **BearDog: Multi-Primal Workflows with Capability Discovery!** 🌐

