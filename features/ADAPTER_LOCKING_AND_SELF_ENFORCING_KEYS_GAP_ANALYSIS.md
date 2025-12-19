# **Adapter Locking & Self-Enforcing Keys - Gap Analysis**
**For BearDog Team**  
**Date:** December 11, 2025  
**Purpose:** Identify missing implementation for architectural enforcement at git and binary level

---

## **🎯 Executive Summary**

**Current State:** Detection and classification work. Licensing checks exist at entry points.  
**Missing:** Cryptographic enforcement, self-evolving constraints, and binary-level adapter locks.  
**Impact:** Cannot yet enforce "open gates for humans, locked tight for commercial extraction" at the architectural level.

---

## **1. ADAPTER LOCKING**

### **✅ What Exists**

**License Check at Entry Point** (`external_functions/prometheus.rs`):
```rust
fn handle(&self, license_manager: &LicenseManager, _operation: &str, payload: serde_json::Value) 
    -> Result<serde_json::Value, BearDogError> 
{
    if !license_manager.is_function_available(self.function_name())? {
        return Err(BearDogError::configuration(
            "🔒 External adapter locked\n\
             👤 Individuals: Automatically granted access\n\
             👥 Small teams: Automatically granted access\n\
             🏢 Corporate usage: External adapters locked - acquire unlock certificate"
        ));
    }
    // ... proceed with function
}
```

**Classification System** (`commercial_extraction/`):
- `CommercialExtractionDetector` - analyzes requests
- `CommercialClassification` - Human/Commercial/Uncertain
- Privacy-preserving identity hashing

### **❌ What's Missing**

#### **1.1 - Cryptographic Adapter Certificates**

**Need:** Adapters should require cryptographically signed "unlock certificates" from BearDog keys.

**Implementation Gap:**
```rust
// MISSING: Certificate verification in adapter initialization

pub struct AdapterUnlockCertificate {
    /// BearDog key that issued this certificate
    pub issuing_key_id: String,
    
    /// What this certificate unlocks
    pub adapter_id: String, // "prometheus", "consul", "kubernetes", etc.
    
    /// Classification that earned this unlock
    pub classification: CommercialClassification,
    
    /// Cryptographic proof (Ed25519 signature)
    pub signature: Vec<u8>,
    
    /// Expiration
    pub expires_at: DateTime<Utc>,
    
    /// Self-enforcing constraints (MISSING - see section 2)
    pub constraints: Option<KeyConstraints>,
}

impl PrometheusAdapter {
    pub fn initialize(&mut self, certificate: AdapterUnlockCertificate) -> Result<(), BearDogError> {
        // 1. Verify certificate signature against BearDog root key
        self.verify_certificate_signature(&certificate)?;
        
        // 2. Check classification allows this adapter
        match certificate.classification {
            Human { .. } => { /* Always allow */ },
            Commercial { risk: High, .. } if certificate.license.is_none() => {
                return Err("Commercial usage requires license");
            },
            _ => {}
        }
        
        // 3. Store certificate for per-call validation
        self.unlock_certificate = Some(certificate);
        
        Ok(())
    }
    
    pub fn execute(&self, operation: &str) -> Result<Response, BearDogError> {
        // Per-call certificate check
        let cert = self.unlock_certificate.as_ref()
            .ok_or("Adapter not unlocked")?;
        
        if cert.expires_at < Utc::now() {
            return Err("Certificate expired");
        }
        
        // Execute operation
        self.do_operation(operation)
    }
}
```

#### **1.2 - Binary-Level Enforcement**

**Need:** Prevent stripped binaries from bypassing checks.

**Git-Level Enforcement** (exists - code is there):
- ✅ License checks in source code
- ✅ AGPL-3.0 requires derivative works stay open

**Binary-Level Enforcement** (missing):
```rust
// MISSING: Compile-time embedding of enforcement logic

// Option A: Proc macro that injects checks at compile time
#[beardog_locked_adapter]
impl PrometheusAdapter {
    pub fn connect(&mut self) -> Result<()> {
        // Macro automatically injects:
        // - Certificate verification
        // - Runtime licensing check
        // - Telltale binary markers (see below)
        
        self.do_connect() // User code
    }
}

// Option B: Link-time verification
// At binary link, inject a verification routine that:
// 1. Checks for BearDog runtime in process
// 2. Validates adapter unlock via IPC with BearDog daemon
// 3. Refuses to operate without verified unlock

// Option C: Runtime attestation (strongest)
// Adapter binary includes:
static ADAPTER_ATTESTATION: &[u8] = include_bytes!("adapter.attestation");

impl PrometheusAdapter {
    pub fn initialize(&mut self) -> Result<()> {
        // Call BearDog daemon for attestation
        let attestation_valid = beardog_runtime::attest_adapter(
            ADAPTER_ATTESTATION,
            std::env::current_exe()?, // Binary path
        )?;
        
        if !attestation_valid {
            return Err("Adapter binary attestation failed - recompile with BearDog SDK");
        }
        
        Ok(())
    }
}
```

**Why This Matters:**
- Someone could strip out `if !license_manager.is_function_available()` and recompile
- AGPL-3.0 requires they publish modified source, but doesn't *prevent* the modification
- Binary attestation makes it **cryptographically expensive** to bypass

#### **1.3 - Per-Adapter Pricing Integration**

**Need:** Adapters report usage for billing.

**Implementation Gap:**
```rust
// MISSING: Usage metering hook in adapters

pub trait MeteredAdapter {
    fn record_usage(&self, operation: &str, data_volume: u64, compute_units: u64);
}

impl PrometheusAdapter {
    pub fn execute(&self, operation: &str) -> Result<Response, BearDogError> {
        let start = Instant::now();
        
        // Execute
        let response = self.do_operation(operation)?;
        
        // Record metered usage
        let cert = self.unlock_certificate.as_ref().unwrap();
        if let Commercial { .. } = cert.classification {
            let elapsed = start.elapsed();
            beardog_runtime::record_adapter_usage(AdapterUsageEvent {
                adapter_id: "prometheus",
                operation,
                duration_ms: elapsed.as_millis(),
                data_volume: response.len() as u64,
                pricing: cert.pricing_tier.per_call_cost,
            });
        }
        
        Ok(response)
    }
}
```

---

## **2. SELF-ENFORCING KEYS**

### **✅ What Exists**

**Genetic Key Framework** (`beardog-genetics/`):
- `GeneticSpawningEngine` - creates keys with parent lineage
- `SpawnRequest` / `SpawnResult` - capability-based spawning
- `BearDogGenetics` - stores capabilities, fitness score, generation
- Inheritance from parent keys
- Fitness scoring

**Key Components:**
```rust
pub struct BearDogGenetics {
    pub id: String,
    pub capabilities: Vec<NodeCapability>,
    pub security_clearance: SecurityClearance,
    pub fitness_score: f64,
    pub generation: u32,
    // ...
}
```

### **❌ What's Missing**

#### **2.1 - Cryptographic Constraint Embedding**

**Need:** Keys must cryptographically enforce their own constraints.

**The Collaboration Key Story Requires:**
```rust
// MISSING: Constraint system in genetic keys

pub struct KeyConstraints {
    /// Cryptographically enforced scope (cannot be removed)
    pub scope: ScopeConstraint,
    
    /// Time-based self-destruction
    pub lifetime: LifetimeConstraint,
    
    /// Data access restrictions
    pub data_access: DataAccessConstraint,
    
    /// Mandatory co-signing requirements
    pub co_signers: Vec<String>, // Other key IDs required
    
    /// Behavioral constraints
    pub behavior: BehavioralConstraint,
}

pub enum ScopeConstraint {
    /// Key can only be used for specific project
    Project { 
        name: String,
        /// Cryptographic hash of project definition
        project_hash: [u8; 32],
    },
    
    /// Key can only access specific resources
    Resources {
        allow_read: Vec<String>,  // Glob patterns
        allow_write: Vec<String>,
        deny_delete: Vec<String>, // Cannot be overridden
    },
    
    /// Key scoped to specific operations
    Operations {
        allowed_operations: Vec<String>,
    },
}

pub struct DataAccessConstraint {
    /// Cannot delete these paths (cryptographically enforced)
    pub immutable_paths: Vec<String>,
    
    /// Must encrypt to these keys
    pub mandatory_encryption: Vec<String>, // Key IDs
    
    /// Audit logging required
    pub audit_required: bool,
}

pub struct LifetimeConstraint {
    /// Hard expiration (key self-destructs)
    pub expires_at: DateTime<Utc>,
    
    /// Soft expiration (triggers evolution)
    pub evolution_trigger: DateTime<Utc>,
    
    /// Can renewal be requested?
    pub renewable: bool,
    
    /// Who can approve renewal?
    pub renewal_approvers: Vec<String>, // Key IDs
}

pub struct BehavioralConstraint {
    /// Biometric verification required?
    pub biometric_required: bool,
    
    /// Expected usage patterns (detects hijacking)
    pub expected_patterns: Vec<UsagePattern>,
    
    /// Challenge-response on anomaly
    pub challenge_on_anomaly: bool,
}
```

**Cryptographic Enforcement:**
```rust
impl BearDogGenetics {
    /// Generate key with embedded constraints
    pub fn generate_with_constraints(
        entropy: &[u8],
        constraints: KeyConstraints,
        parent_keys: Vec<&BearDogGenetics>,
    ) -> Result<Self, BearDogError> {
        // 1. Generate key material
        let (private_key, public_key) = ed25519_dalek::Keypair::generate(entropy);
        
        // 2. Serialize constraints
        let constraint_bytes = bincode::serialize(&constraints)?;
        
        // 3. Create self-signed constraint certificate
        // This binds the constraints to the key cryptographically
        let constraint_signature = private_key.sign(&constraint_bytes);
        
        // 4. Embed in key metadata (cannot be separated from key)
        let genetics = BearDogGenetics {
            id: uuid::Uuid::new_v4().to_string(),
            private_key: private_key.to_bytes().to_vec(),
            public_key: public_key.to_bytes().to_vec(),
            constraints: Some(constraints),
            constraint_signature: Some(constraint_signature.to_bytes().to_vec()),
            // ... other fields
        };
        
        Ok(genetics)
    }
    
    /// Verify operation against constraints
    pub fn verify_operation(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        let constraints = self.constraints.as_ref()
            .ok_or("Key has no constraints")?;
        
        // 1. Verify constraint signature (detect tampering)
        self.verify_constraint_integrity()?;
        
        // 2. Check lifetime
        if Utc::now() > constraints.lifetime.expires_at {
            return Err(BearDogError::unauthorized("Key expired (lifetime constraint)"));
        }
        
        // 3. Check scope
        match &constraints.scope {
            ScopeConstraint::Project { name, .. } => {
                if operation.project != *name {
                    return Err(BearDogError::unauthorized(format!(
                        "Key scoped to project '{}', cannot access '{}'",
                        name, operation.project
                    )));
                }
            }
            ScopeConstraint::Resources { allow_read, allow_write, deny_delete } => {
                if operation.is_delete() && deny_delete.iter().any(|p| operation.path.starts_with(p)) {
                    return Err(BearDogError::unauthorized(
                        "Key cannot delete protected resources (cryptographically enforced)"
                    ));
                }
                // ... other checks
            }
            _ => {}
        }
        
        // 4. Check co-signer requirements
        if !constraints.co_signers.is_empty() && operation.co_signatures.is_empty() {
            return Err(BearDogError::unauthorized(
                "Operation requires co-signature from other keys"
            ));
        }
        
        // 5. Behavioral checks
        if constraints.behavior.biometric_required && operation.biometric_proof.is_none() {
            return Err(BearDogError::unauthorized(
                "Key requires biometric verification for this operation"
            ));
        }
        
        Ok(())
    }
    
    fn verify_constraint_integrity(&self) -> Result<(), BearDogError> {
        let constraints = self.constraints.as_ref().unwrap();
        let signature = self.constraint_signature.as_ref().unwrap();
        
        let constraint_bytes = bincode::serialize(constraints)?;
        let public_key = ed25519_dalek::PublicKey::from_bytes(&self.public_key)?;
        let signature = ed25519_dalek::Signature::from_bytes(signature)?;
        
        public_key.verify(&constraint_bytes, &signature)
            .map_err(|_| BearDogError::security("Key constraints have been tampered with"))?;
        
        Ok(())
    }
}
```

#### **2.2 - Genetic Evolution with Constraint Adaptation**

**Need:** Keys evolve, constraints adapt.

**Implementation Gap:**
```rust
// MISSING: Constraint evolution in spawning

impl GeneticSpawningEngine {
    pub fn evolve_key(&self, parent: &BearDogGenetics, context: &EvolutionContext) 
        -> Result<BearDogGenetics, BearDogError> 
    {
        let parent_constraints = parent.constraints.as_ref()
            .ok_or("Parent has no constraints to evolve")?;
        
        // Analyze usage history
        let usage_analysis = self.analyze_key_usage(parent, context)?;
        
        // Evolve constraints based on behavior
        let evolved_constraints = match usage_analysis {
            UsageAnalysis::TrustworthyBehavior => {
                // Good behavior: Relax constraints
                KeyConstraints {
                    lifetime: LifetimeConstraint {
                        expires_at: parent_constraints.lifetime.expires_at + Duration::days(30),
                        ..parent_constraints.lifetime.clone()
                    },
                    // Maybe add more resources
                    ..parent_constraints.clone()
                }
            }
            UsageAnalysis::SuspiciousBehavior => {
                // Suspicious: Tighten constraints
                KeyConstraints {
                    behavior: BehavioralConstraint {
                        biometric_required: true, // Force biometric
                        challenge_on_anomaly: true,
                        ..parent_constraints.behavior.clone()
                    },
                    ..parent_constraints.clone()
                }
            }
            UsageAnalysis::UnauthorizedAccessAttempt => {
                // Revoke: Self-destruct immediately
                return Err(BearDogError::security("Key revoked due to unauthorized access"));
            }
        };
        
        // Generate evolved key
        BearDogGenetics::generate_with_constraints(
            context.entropy,
            evolved_constraints,
            vec![parent],
        )
    }
}
```

#### **2.3 - Runtime Enforcement Integration**

**Need:** Keys must check constraints at operation time, not trust time.

**Implementation Gap:**
```rust
// MISSING: Integration point in all operations

pub trait BearDogOperation {
    fn execute_with_key(&self, key: &BearDogGenetics) -> Result<Self::Output, BearDogError> {
        // 1. Verify key constraints allow this operation
        key.verify_operation(&self.to_key_operation())?;
        
        // 2. If co-signers required, collect signatures
        if let Some(constraints) = &key.constraints {
            for co_signer_id in &constraints.co_signers {
                let co_signature = self.request_co_signature(co_signer_id)?;
                self.add_co_signature(co_signature);
            }
        }
        
        // 3. Execute (now cryptographically authorized)
        self.execute_internal()
    }
}

// Every NestGate read/write, ToadStool launch, Songbird RPC would use this
```

---

## **3. WHAT OTHER PRIMALS NEED**

### **3.1 - Songbird (Service Mesh)**

**Adapter Integration Points:**

```rust
// IN: songbird-universal/src/adapters/

pub trait ExternalAdapter {
    /// Initialize with BearDog certificate
    fn initialize(&mut self, certificate: AdapterUnlockCertificate) -> Result<(), Error>;
    
    /// Every call checks certificate validity
    fn execute(&self, operation: &str, params: &[u8]) -> Result<Vec<u8>, Error>;
    
    /// Report usage for metering
    fn report_usage(&self, event: UsageEvent);
}

// Implementations needed for:
// - ConsulAdapter (service discovery)
// - KubernetesAdapter (container orchestration)
// - PrometheusAdapter (already started in BearDog)
// - GrafanaAdapter (already started in BearDog)
// - VaultAdapter (HashiCorp secrets)
// - EtcdAdapter (distributed config)
// - (...all external integrations)
```

**Key Constraint Enforcement:**

```rust
// IN: songbird-execution-agent/src/security_beardog.rs

impl SecurityLayer {
    pub fn authorize_rpc(&self, rpc: &RpcRequest, key: &BearDogGenetics) -> Result<(), Error> {
        // Use BearDog's key constraint verification
        key.verify_operation(&KeyOperation::RpcCall {
            target_service: rpc.service,
            method: rpc.method,
            // ...
        })?;
        
        Ok(())
    }
}
```

### **3.2 - NestGate (Storage)**

**Data Access Constraints:**

```rust
// IN: nestgate-core/src/access_control.rs

impl AccessController {
    pub fn authorize_read(&self, path: &Path, key: &BearDogGenetics) -> Result<(), Error> {
        // Check key constraints
        if let Some(constraints) = &key.constraints {
            match &constraints.data_access {
                DataAccessConstraint { allow_read, .. } => {
                    if !allow_read.iter().any(|pattern| path_matches(path, pattern)) {
                        return Err("Key not authorized to read this path");
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn authorize_delete(&self, path: &Path, key: &BearDogGenetics) -> Result<(), Error> {
        // Check immutable paths
        if let Some(constraints) = &key.constraints {
            if constraints.data_access.immutable_paths.iter()
                .any(|protected| path.starts_with(protected)) 
            {
                return Err("Key cannot delete protected data (cryptographically enforced)");
            }
        }
        
        Ok(())
    }
}
```

**Mandatory Encryption:**

```rust
// IN: nestgate-core/src/encryption.rs

impl StorageEngine {
    pub fn write(&mut self, path: &Path, data: &[u8], key: &BearDogGenetics) -> Result<(), Error> {
        // Check if this key requires mandatory encryption
        if let Some(constraints) = &key.constraints {
            if !constraints.data_access.mandatory_encryption.is_empty() {
                // Must encrypt to specified keys
                let encrypted = self.encrypt_multi_key(
                    data,
                    &constraints.data_access.mandatory_encryption,
                )?;
                return self.write_encrypted(path, &encrypted);
            }
        }
        
        // Otherwise, normal write
        self.write_internal(path, data)
    }
}
```

### **3.3 - ToadStool (Compute Orchestration)**

**Compute Quota Enforcement:**

```rust
// IN: toadstool-runtime/src/authorization.rs

impl ResourceAuthorizer {
    pub fn authorize_compute(&self, request: &ComputeRequest, key: &BearDogGenetics) 
        -> Result<(), Error> 
    {
        // Check if key has compute constraints
        if let Some(constraints) = &key.constraints {
            if let Some(compute_quota) = &constraints.compute_quota {
                let used = self.get_compute_usage(key.id())?;
                if used + request.estimated_hours > compute_quota.max_hours {
                    return Err("Compute quota exceeded (key constraint)");
                }
            }
        }
        
        Ok(())
    }
}
```

**Biometric Verification for Sensitive Operations:**

```rust
// IN: toadstool-runtime/src/launcher.rs

impl RuntimeLauncher {
    pub fn launch_privileged(&self, container: &Container, key: &BearDogGenetics) 
        -> Result<(), Error> 
    {
        // Check if key requires biometric verification
        if let Some(constraints) = &key.constraints {
            if constraints.behavior.biometric_required {
                let biometric = self.prompt_biometric_verification()?;
                if !key.verify_biometric(&biometric)? {
                    return Err("Biometric verification failed");
                }
            }
        }
        
        self.launch_internal(container)
    }
}
```

### **3.4 - SweetGrass (Provenance)**

**Key Provenance Tracking:**

```rust
// IN: sweetgrass-core/src/braid.rs

impl BraidBuilder {
    pub fn record_key_usage(&mut self, operation: &Operation, key: &BearDogGenetics) {
        self.add_provenance(ProvenanceNode {
            agent: AgentIdentity::CryptographicKey {
                key_id: key.id().clone(),
                key_generation: key.generation,
                parent_keys: key.parent_lineage.clone(),
                constraints_hash: key.constraint_hash(), // Immutable record
            },
            activity: operation.to_activity(),
            timestamp: Utc::now(),
            // ...
        });
    }
}
```

This creates an immutable record that:
- Dr. Martinez and Dr. Kowalski used collaboration_key (Generation 2)
- The key had specific constraints (hashed, cannot be disputed)
- All operations are traceable to key genetics

---

## **4. BINARY vs GIT ENFORCEMENT**

### **4.1 - Git Level (Source Code)**

**✅ Already Effective:**
- AGPL-3.0 licensing requires derivative works stay open
- License checks embedded in source
- Anyone modifying must publish modifications

**Limitation:**
- Can't *prevent* someone from stripping checks and not publishing
- Relies on legal enforcement, not technical enforcement

### **4.2 - Binary Level (Runtime)**

**❌ Needs Implementation:**

#### **Option A: Cryptographic Binary Attestation**

```rust
// Build-time: Generate attestation
// In build.rs:

fn main() {
    let binary_hash = hash_output_binary();
    let attestation = beardog_sdk::generate_attestation(
        binary_hash,
        env!("BEARDOG_SDK_KEY"),
    );
    
    // Embed in binary
    println!("cargo:rustc-env=BINARY_ATTESTATION={}", hex::encode(attestation));
}

// Runtime: Verify attestation
impl PrometheusAdapter {
    pub fn initialize() -> Result<Self, Error> {
        let attestation = env!("BINARY_ATTESTATION");
        let binary_path = std::env::current_exe()?;
        
        // Call BearDog daemon to verify
        let valid = beardog_runtime::verify_adapter_attestation(
            attestation,
            &std::fs::read(binary_path)?,
        )?;
        
        if !valid {
            return Err("Binary attestation failed - this adapter was modified after build");
        }
        
        Ok(Self { /* ... */ })
    }
}
```

**How This Works:**
1. At build time, BearDog SDK signs the binary hash
2. Signature embedded in binary as static data
3. At runtime, binary contacts BearDog daemon
4. Daemon verifies signature matches expected hash
5. If tampered with, signature won't match → adapter refuses to run

**Bypassing This Requires:**
- Recompiling from source (already requires AGPL compliance)
- Generating valid BearDog attestation (requires BearDog SDK signing key - not public)
- Running own BearDog daemon that doesn't check (requires forking BearDog - AGPL)

**Result:** Technical barrier + legal barrier (AGPL) + social barrier (reputation)

#### **Option B: Runtime Daemon Dependency**

```rust
// Adapter MUST communicate with BearDog daemon

impl PrometheusAdapter {
    pub fn connect(&mut self) -> Result<()> {
        // 1. Find BearDog daemon via Unix socket / IPC
        let daemon = BearDogDaemon::connect("/var/run/beardog.sock")?;
        
        // 2. Request adapter unlock
        let certificate = daemon.request_adapter_unlock(AdapterUnlockRequest {
            adapter_id: "prometheus",
            pid: std::process::id(),
            binary_path: std::env::current_exe()?,
        }).await?;
        
        // 3. Daemon performs:
        //    - Binary attestation
        //    - Classification check (human vs commercial)
        //    - License validation
        //    - Issues time-limited certificate
        
        // 4. Store certificate, use for all operations
        self.certificate = Some(certificate);
        
        Ok(())
    }
    
    pub fn execute(&self, op: &str) -> Result<Response> {
        // Check certificate still valid
        let cert = self.certificate.as_ref().ok_or("Not initialized")?;
        if cert.expires_at < Utc::now() {
            return Err("Certificate expired - reinitialize adapter");
        }
        
        self.do_operation(op)
    }
}
```

**How This Works:**
- Adapter is *inert* without BearDog daemon running
- Daemon performs all verification
- Issues short-lived certificates (15 minute expiry)
- Adapter must re-request periodically

**Bypassing This Requires:**
- Mocking the daemon socket (requires root access + custom daemon)
- Reimplementing certificate logic (requires forking BearDog - AGPL)

#### **Option C: Encrypted Adapter Logic**

**Most Aggressive (Also Most Controversial):**

```rust
// Critical adapter logic is encrypted at build time
// Decryption key only available from BearDog daemon

impl PrometheusAdapter {
    const ENCRYPTED_LOGIC: &[u8] = include_bytes!("prometheus_logic.encrypted");
    
    pub fn initialize() -> Result<Self> {
        let daemon = BearDogDaemon::connect()?;
        
        // Daemon performs classification
        // Returns decryption key ONLY if authorized
        let decryption_key = daemon.authorize_adapter("prometheus").await?;
        
        // Decrypt core logic
        let logic_module = decrypt_and_load(Self::ENCRYPTED_LOGIC, &decryption_key)?;
        
        Ok(Self {
            logic: logic_module,
        })
    }
}
```

**Controversial Because:**
- Obscurity is not security
- May violate AGPL "preferred form for modification" clause
- Could be seen as DRM (anti-freedom)

**But Effective Because:**
- Even with source, can't run without BearDog authorization
- Makes stripping checks functionally useless

**Recommendation:** Don't use Option C. Stick with A + B.

---

## **5. PRIORITY IMPLEMENTATION ORDER**

### **Phase 1: Self-Enforcing Keys (Foundation)**
**Duration:** 2-4 weeks  
**Impact:** High - Enables collaboration key story

**Deliverables:**
1. `KeyConstraints` type system (scope, lifetime, data access, behavior)
2. Cryptographic constraint embedding in `BearDogGenetics`
3. `verify_operation()` enforcement at operation time
4. Constraint evolution in `GeneticSpawningEngine`
5. Integration hooks in BearDog auth layer

**Success Criteria:**
- Can generate collaboration key with "cannot delete raw data" constraint
- Deletion attempt cryptographically blocked
- Key evolution adapts constraints based on usage

---

### **Phase 2: Adapter Certificates**
**Duration:** 2-3 weeks  
**Impact:** High - Enables adapter locking story

**Deliverables:**
1. `AdapterUnlockCertificate` type
2. Certificate signing in BearDog daemon
3. Certificate verification in adapters
4. Short-lived certificate renewal
5. Classification-based certificate issuance

**Success Criteria:**
- Individual gets automatic certificate
- Commercial use without license blocked
- Certificate expiry forces re-classification

---

### **Phase 3: Binary Attestation**
**Duration:** 2-3 weeks  
**Impact:** Medium - Hardens against bypass

**Deliverables:**
1. Build-time attestation generation (`build.rs`)
2. Runtime attestation verification
3. BearDog daemon attestation checking
4. Binary hash verification

**Success Criteria:**
- Modified adapter binary refuses to run
- Stripping license checks breaks attestation
- Only BearDog-SDK-built adapters work

---

### **Phase 4: Primal Integration**
**Duration:** 4-6 weeks (parallel across teams)  
**Impact:** High - Makes system-wide enforcement real

**Deliverables:**
1. Songbird adapter trait implementations
2. NestGate data access constraint enforcement
3. ToadStool compute quota enforcement
4. SweetGrass key provenance tracking
5. Cross-primal key verification

**Success Criteria:**
- Collaboration key works across all Primals
- Constraints enforced in Songbird RPC
- NestGate blocks deletions per key constraints
- SweetGrass records full key lineage
- ToadStool enforces compute quotas

---

### **Phase 5: Usage Metering & Pricing**
**Duration:** 3-4 weeks  
**Impact:** Medium - Enables commercial model

**Deliverables:**
1. Per-adapter usage tracking
2. Usage aggregation in BearDog daemon
3. Pricing calculation based on classification + usage
4. Invoice generation
5. Integration with sunCloud (future)

**Success Criteria:**
- Commercial user gets per-adapter usage report
- Automation tax correctly calculated
- Individual usage untracked (privacy preserved)

---

## **6. TESTING STRATEGY**

### **6.1 - Self-Enforcing Keys Tests**

```rust
#[test]
fn test_collaboration_key_cannot_delete_protected_data() {
    let martinez_key = generate_key("martinez");
    let kowalski_key = generate_key("kowalski");
    
    let collab_key = BearDogGenetics::mix_keys(
        vec![&martinez_key, &kowalski_key],
        KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
    ).unwrap();
    
    // Try to delete protected path
    let delete_op = KeyOperation::Delete {
        path: "raw_data/2020-temperature.nc".to_string(),
    };
    
    let result = collab_key.verify_operation(&delete_op);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cryptographically enforced"));
}

#[test]
fn test_key_expires_after_lifetime() {
    let key = BearDogGenetics::generate_with_constraints(
        &random_entropy(),
        KeyConstraints {
            lifetime: LifetimeConstraint {
                expires_at: Utc::now() + Duration::seconds(1),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    ).unwrap();
    
    // Works immediately
    assert!(key.verify_operation(&KeyOperation::Read { .. }).is_ok());
    
    // Wait for expiry
    std::thread::sleep(Duration::seconds(2));
    
    // Now fails
    assert!(key.verify_operation(&KeyOperation::Read { .. }).is_err());
}

#[test]
fn test_constraint_tampering_detected() {
    let mut key = BearDogGenetics::generate_with_constraints(...).unwrap();
    
    // Try to modify constraints
    key.constraints.as_mut().unwrap().lifetime.expires_at = Utc::now() + Duration::days(999);
    
    // Verification detects tampering
    let result = key.verify_constraint_integrity();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("tampered"));
}
```

### **6.2 - Adapter Locking Tests**

```rust
#[test]
fn test_individual_gets_automatic_adapter_access() {
    let detector = CommercialExtractionDetector::new();
    
    let request = create_request_with_classification(CommercialClassification::Human {
        confidence: 0.9,
    });
    
    let adapter = PrometheusAdapter::new();
    let result = adapter.initialize_with_request(&request);
    
    assert!(result.is_ok());
    assert!(adapter.is_unlocked());
}

#[test]
fn test_commercial_without_license_blocked() {
    let request = create_request_with_classification(CommercialClassification::Commercial {
        confidence: 0.8,
        risk_level: ExtractionRisk::High,
    });
    
    let adapter = PrometheusAdapter::new();
    let result = adapter.initialize_with_request(&request);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("license required"));
}

#[test]
fn test_certificate_expiry_blocks_operation() {
    let adapter = PrometheusAdapter::new();
    
    // Initialize with short-lived certificate
    let cert = AdapterUnlockCertificate {
        expires_at: Utc::now() + Duration::seconds(1),
        ..Default::default()
    };
    adapter.initialize_with_certificate(cert).unwrap();
    
    // Works immediately
    assert!(adapter.execute("query", params).is_ok());
    
    // Wait for expiry
    std::thread::sleep(Duration::seconds(2));
    
    // Now blocked
    assert!(adapter.execute("query", params).is_err());
}
```

### **6.3 - Binary Attestation Tests**

```rust
#[test]
fn test_unattested_binary_refuses_to_run() {
    // Simulate binary without valid attestation
    env::remove_var("BINARY_ATTESTATION");
    
    let result = PrometheusAdapter::initialize();
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("attestation"));
}

#[test]
fn test_tampered_binary_detected() {
    // Build adapter with attestation
    let adapter_binary = build_attested_adapter("prometheus");
    
    // Modify binary (simulate stripping checks)
    let mut modified = std::fs::read(&adapter_binary).unwrap();
    modified[1000] ^= 0xFF; // Flip some bits
    std::fs::write(&adapter_binary, modified).unwrap();
    
    // Try to run
    let result = run_binary(&adapter_binary);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("attestation failed"));
}
```

---

## **7. DOCUMENTATION NEEDS**

### **For BearDog Developers:**
1. **Key Constraints Guide** - How to add new constraint types
2. **Adapter Integration Guide** - How to lock new external adapters
3. **Attestation Build Guide** - How to enable attestation in `build.rs`
4. **Testing Guide** - How to test enforcement (without real licenses)

### **For Primal Teams:**
1. **Songbird Integration** - How to enforce key constraints in RPC
2. **NestGate Integration** - How to enforce data access constraints
3. **ToadStool Integration** - How to enforce compute quotas
4. **SweetGrass Integration** - How to record key provenance

### **For End Users:**
1. **Collaboration Key Tutorial** - How to create constrained collaboration keys
2. **Adapter Unlock Guide** - What to do if adapter locked
3. **Key Evolution Guide** - How keys automatically improve
4. **Troubleshooting** - Common issues and solutions

---

## **8. OPEN QUESTIONS FOR BEARDOG TEAM**

1. **Binary attestation approach:** Option A (attestation) or Option B (daemon dependency) or both?

2. **Certificate lifetime:** How long should adapter unlock certificates last? (Recommend: 15 minutes for commercial, 24 hours for individual)

3. **Constraint extensibility:** Should third-party code be able to define custom constraint types?

4. **Cross-primal constraint format:** JSON, CBOR, or custom binary format for constraint serialization?

5. **Key revocation:** How to revoke a key if it's been compromised? (Need revocation list / OCSP-like system?)

6. **Offline operation:** Can keys work offline, or must they always contact BearDog daemon?

7. **Mobile support:** How do constraints work on Android (Pixel 8a)? Different enforcement model needed?

8. **Performance:** What's acceptable latency for `verify_operation()` check? (Recommend: < 1ms for hot path)

---

## **9. CONCLUSION**

**What's Solid:**
- Detection and classification foundation is strong
- Genetic key system has good architecture
- License checks exist at entry points

**What's Missing:**
- **Cryptographic self-enforcement** of constraints
- **Adapter certificate** issuance and verification
- **Binary attestation** to prevent bypass
- **Cross-primal integration** hooks

**Estimated Total Effort:** 13-18 weeks across Phase 1-5

**Blocking Order:**
1. Phase 1 (self-enforcing keys) must come first - foundation for everything
2. Phase 2 (adapter certificates) depends on Phase 1
3. Phase 3 (attestation) can run parallel to Phase 2
4. Phase 4 (primal integration) depends on Phase 1 + 2
5. Phase 5 (metering) depends on Phase 2

**Recommendation:** Start Phase 1 immediately. It's the cornerstone. Once constraint system exists, everything else falls into place.

---

**Questions? Concerns? Better ideas?**  
→ This is a living document. Update as implementation progresses.

🐻🐕🔒🧬

