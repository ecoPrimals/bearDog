# **For BearDog Team: Adapter Locking & Self-Enforcing Keys**

**TL;DR:** Detection works. Classification works. But **enforcement doesn't exist yet**. Here's what we need.

---

## **🎯 The Two Missing Pieces**

### **1. Self-Enforcing Keys (CRITICAL - Build This First)**

**What We Have:**
- ✅ `BearDogGenetics` - stores capabilities, generation, fitness
- ✅ `GeneticSpawningEngine` - creates keys with inheritance
- ✅ Key lineage tracking

**What's Missing:**
- ❌ `KeyConstraints` - scope, lifetime, data access rules
- ❌ Cryptographic constraint embedding (signed into key, cannot be removed)
- ❌ `verify_operation()` - checks constraints before allowing operation
- ❌ Constraint evolution (keys adapt constraints based on behavior)

**Why Critical:**
This is the foundation. Without self-enforcing constraints, keys are just identity. With it, keys become **architectural law**.

**Code Sketch:**
```rust
pub struct KeyConstraints {
    pub scope: ScopeConstraint,        // "Climate modeling only"
    pub lifetime: LifetimeConstraint,  // Expires, triggers evolution
    pub data_access: DataAccessConstraint, // Cannot delete raw_data/*
    pub co_signers: Vec<String>,       // Requires other keys to co-sign
    pub behavior: BehavioralConstraint, // Biometric required, etc.
}

impl BearDogGenetics {
    pub fn generate_with_constraints(
        entropy: &[u8],
        constraints: KeyConstraints,
        parents: Vec<&Self>,
    ) -> Result<Self> {
        // 1. Generate key pair
        // 2. Sign constraints with private key
        // 3. Embed signed constraints in key (cannot be separated)
    }
    
    pub fn verify_operation(&self, op: &KeyOperation) -> Result<()> {
        // 1. Verify constraint signature (detect tampering)
        // 2. Check: Is operation allowed by scope?
        // 3. Check: Is key expired?
        // 4. Check: Does this violate data access rules?
        // 5. Check: Are required co-signers present?
        // 6. Check: Is biometric verification required?
    }
}
```

**Collaboration Key Story Test:**
```rust
// Can we do this?
let collab_key = BearDogGenetics::mix_keys(
    vec![martinez_key, kowalski_key],
    KeyConstraints {
        scope: "Climate modeling project",
        lifetime: 18 months,
        data_access: DataAccessConstraint {
            cannot_delete: vec!["raw_data/*"],
            must_encrypt_to: vec![martinez_key.id(), kowalski_key.id()],
        },
        ..Default::default()
    },
)?;

// This should be CRYPTOGRAPHICALLY BLOCKED:
let result = collab_key.verify_operation(&KeyOperation::Delete {
    path: "raw_data/2020-temperature.nc"
});
assert!(result.is_err()); // ✅ Blocked by constraint
```

---

### **2. Adapter Certificate System**

**What We Have:**
- ✅ `CommercialExtractionDetector` - classifies Human/Commercial
- ✅ License check at entry point (Prometheus, Grafana)
- ✅ Classification confidence scoring

**What's Missing:**
- ❌ `AdapterUnlockCertificate` - cryptographically signed unlock
- ❌ Certificate issuance by BearDog daemon
- ❌ Certificate verification in adapters
- ❌ Short-lived certificate renewal

**Code Sketch:**
```rust
pub struct AdapterUnlockCertificate {
    pub adapter_id: String, // "prometheus", "consul", etc.
    pub classification: CommercialClassification,
    pub expires_at: DateTime<Utc>, // 15 min for commercial, 24hr for individual
    pub signature: Vec<u8>, // Ed25519 signature from BearDog root key
    pub constraints: Option<KeyConstraints>, // Self-enforcing
}

impl PrometheusAdapter {
    pub fn initialize(&mut self, cert: AdapterUnlockCertificate) -> Result<()> {
        // 1. Verify certificate signature
        beardog_crypto::verify_certificate(&cert)?;
        
        // 2. Check classification allows this adapter
        match cert.classification {
            Human { .. } => { /* Always allow */ },
            Commercial { risk: High, .. } if cert.license.is_none() => {
                return Err("🔒 Corporate usage detected. License required.");
            },
            _ => {}
        }
        
        self.certificate = Some(cert);
        Ok(())
    }
    
    pub fn execute(&self, op: &str) -> Result<Response> {
        let cert = self.certificate.as_ref().ok_or("Adapter not unlocked")?;
        
        // Check expiry
        if cert.expires_at < Utc::now() {
            return Err("Certificate expired");
        }
        
        self.do_operation(op)
    }
}
```

**Adapter Locking Story Test:**
```rust
// TechOps (commercial, pure automation)
let request = create_request_with_high_commercial_score();
let adapter = PrometheusAdapter::new();
let result = adapter.initialize_with_request(&request);

assert!(result.is_err()); // ✅ Blocked without license

// Individual developer
let request = create_request_with_human_classification();
let adapter = PrometheusAdapter::new();
let result = adapter.initialize_with_request(&request);

assert!(result.is_ok()); // ✅ Automatic unlock
```

---

## **🏗️ What Other Primals Need**

### **Songbird (Service Mesh)**

**Every external adapter needs:**

```rust
// IN: songbird-universal/src/adapters/

pub trait ExternalAdapter {
    /// Initialize with BearDog certificate
    fn initialize(&mut self, cert: AdapterUnlockCertificate) -> Result<()>;
    
    /// Check certificate on every call
    fn execute(&self, op: &str, params: &[u8]) -> Result<Vec<u8>>;
}

// Implement for:
// - ConsulAdapter
// - KubernetesAdapter  
// - VaultAdapter
// - EtcdAdapter
// - AWS/Azure/GCP adapters
```

### **NestGate (Storage)**

**Data access constraint enforcement:**

```rust
// IN: nestgate-core/src/access_control.rs

impl AccessController {
    pub fn authorize_delete(&self, path: &Path, key: &BearDogGenetics) -> Result<()> {
        // Use key.verify_operation()
        key.verify_operation(&KeyOperation::Delete { path })?;
        Ok(())
    }
}
```

### **ToadStool (Compute)**

**Compute quota enforcement:**

```rust
// IN: toadstool-runtime/src/authorization.rs

impl ResourceAuthorizer {
    pub fn authorize_compute(&self, request: &ComputeRequest, key: &BearDogGenetics) -> Result<()> {
        // Check key constraints for compute quota
        if let Some(constraints) = &key.constraints {
            if let Some(quota) = &constraints.compute_quota {
                // Enforce
            }
        }
        Ok(())
    }
}
```

### **SweetGrass (Provenance)**

**Record key genetics in provenance:**

```rust
// IN: sweetgrass-core/src/braid.rs

impl BraidBuilder {
    pub fn record_key_usage(&mut self, op: &Operation, key: &BearDogGenetics) {
        self.add_provenance(ProvenanceNode {
            agent: AgentIdentity::CryptographicKey {
                key_id: key.id(),
                generation: key.generation,
                constraints_hash: key.constraint_hash(), // Immutable
                parent_keys: key.parent_lineage,
            },
            // ...
        });
    }
}
```

---

## **🔨 Binary-Level Enforcement (Phase 3)**

**Problem:** Someone could strip license checks from source, recompile.

**Solution:** Binary attestation

```rust
// In build.rs:
fn main() {
    let binary_hash = hash_output_binary();
    let attestation = beardog_sdk::sign_binary(binary_hash);
    println!("cargo:rustc-env=ATTESTATION={}", attestation);
}

// At runtime:
impl PrometheusAdapter {
    pub fn initialize() -> Result<Self> {
        let attestation = env!("ATTESTATION");
        let binary = std::fs::read(std::env::current_exe()?)?;
        
        // Verify with BearDog daemon
        beardog_daemon::verify_attestation(attestation, &binary)?;
        
        Ok(Self { .. })
    }
}
```

**Why This Works:**
- Tampering with binary invalidates attestation
- BearDog daemon refuses unattested adapters
- Requires BearDog SDK signing key to create valid attestation
- Even with source code, can't bypass without forking BearDog (AGPL)

---

## **📋 Implementation Priority**

### **Phase 1: Self-Enforcing Keys (2-4 weeks)**
**CRITICAL - DO THIS FIRST**

1. Add `KeyConstraints` type system
2. Embed constraints in `BearDogGenetics` with signature
3. Implement `verify_operation()`
4. Add constraint evolution to `GeneticSpawningEngine`
5. Test: Collaboration key story works end-to-end

### **Phase 2: Adapter Certificates (2-3 weeks)**
**Depends on Phase 1**

1. Add `AdapterUnlockCertificate` type
2. Implement certificate signing in BearDog daemon
3. Update all external adapters (Prometheus, Grafana, etc.)
4. Add short-lived certificate renewal
5. Test: Adapter locking story works

### **Phase 3: Binary Attestation (2-3 weeks)**
**Can run parallel to Phase 2**

1. Add attestation generation to `build.rs`
2. Implement runtime verification
3. Update BearDog daemon to check attestations
4. Test: Modified binary refuses to run

### **Phase 4: Primal Integration (4-6 weeks)**
**Parallel work across teams**

1. Songbird: Adapter trait + key enforcement
2. NestGate: Data access constraint enforcement
3. ToadStool: Compute quota enforcement
4. SweetGrass: Key provenance tracking

---

## **✅ Success Criteria**

**Phase 1 Complete When:**
- [ ] Can generate collaboration key with "cannot delete" constraint
- [ ] Deletion attempt is cryptographically blocked
- [ ] Key evolution adapts constraints based on usage
- [ ] Constraint tampering is detected and rejected

**Phase 2 Complete When:**
- [ ] Individual developer gets automatic adapter unlock
- [ ] Commercial use without license is blocked
- [ ] Certificate expiry forces re-unlock
- [ ] All external adapters support certificates

**Phase 3 Complete When:**
- [ ] Modified adapter binary refuses to run
- [ ] Stripping license checks breaks attestation
- [ ] Only BearDog-SDK-built adapters work

**Phase 4 Complete When:**
- [ ] Collaboration key works across all Primals
- [ ] NestGate blocks deletions per key constraints
- [ ] ToadStool enforces compute quotas
- [ ] SweetGrass records full key lineage

---

## **❓ Questions**

1. **Certificate lifetime:** 15 min for commercial, 24hr for individual? Or different?

2. **Binary attestation:** Build-time signing or runtime daemon dependency? Or both?

3. **Offline keys:** Should keys work offline, or always require BearDog daemon?

4. **Mobile enforcement:** How do constraints work on Android (Pixel 8a)?

5. **Key revocation:** Need revocation list / OCSP-like system?

---

## **📁 Full Details**

See `ADAPTER_LOCKING_AND_SELF_ENFORCING_KEYS_GAP_ANALYSIS.md` for:
- Complete code examples
- Testing strategy
- Documentation needs
- Integration specifications for each Primal

---

**Bottom Line:**

The **architecture is sound**. The **detection works**. We just need **enforcement**:

1. Keys that cryptographically enforce their own constraints
2. Adapters that cryptographically verify unlock certificates
3. Binaries that cryptographically attest their integrity

**Start with Phase 1. Everything else builds on it.**

🐻🐕🔒

