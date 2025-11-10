# SoloKey Advanced Experiments - November 9, 2025

## 🔬 **Research Questions**

1. **Can we imprint a crypto key on one and copy it to the other?**
2. **Can we have multiple genetic samples (credentials) on the same key for different roles?**

---

## 🔑 **Question 1: Key Copying Between Devices**

### TL;DR: **Not directly, but we have alternatives!**

### Why HSMs Don't Allow Key Extraction:

**Security by Design** 🔒
```
┌─────────────────────────────────────┐
│      Hardware Security Module       │
│                                     │
│  ┌───────────────────────────┐     │
│  │   Private Key Storage     │     │
│  │   (Secure Element)        │     │
│  │                           │     │
│  │   Keys NEVER leave here!  │ ← 🔐 This is the point!
│  │                           │     │
│  └───────────────────────────┘     │
│            ↑                        │
│            │ Sign/Decrypt only      │
│            │ (never export)         │
└─────────────────────────────────────┘
```

**The Fundamental Rule**: 
> HSM keys are **non-exportable** by design. If you could copy a key out, it wouldn't be a "hardware" security module!

---

### ✅ **Alternative Approaches**

#### Option 1: Deterministic Key Derivation (Best Practice)

**Concept**: Generate the same key on both devices from the same seed.

```rust
// NOT copying keys, but deriving the same key on both devices
async fn derive_shared_key_on_device(
    device: &Fido2Device,
    master_seed: &[u8],
    role: &str,
) -> Result<KeyHandle, BearDogError> {
    // Use hmac-secret to derive device-specific key
    // from shared master seed + device-specific salt
    
    let salt = format!("beardog:{}:v1", role);
    
    // Each device independently derives the same key
    let derived_key = device.hmac_secret(
        master_seed,
        salt.as_bytes(),
    ).await?;
    
    // Create a resident key from the derived material
    device.make_credential(
        rp_id: "beardog.dev",
        key_material: &derived_key,
        role: role,
    ).await
}

// Usage:
let master_seed = generate_master_seed(); // Shared secret
let key1 = derive_shared_key_on_device(&device1, &master_seed, "admin").await?;
let key2 = derive_shared_key_on_device(&device2, &master_seed, "admin").await?;

// Both devices can now sign with "the same" key
// But the private key never left either device!
```

**Security Properties**:
- ✅ Master seed can be backed up securely
- ✅ Keys never leave hardware
- ✅ Both devices can independently derive same key
- ✅ Deterministic and reproducible

---

#### Option 2: Key Wrapping/Migration (Advanced)

**Concept**: Wrap a key for transport between HSMs.

```rust
// Device 1: Export wrapped key
async fn export_wrapped_key(
    device1: &Fido2Device,
    key_handle: &KeyHandle,
    wrapping_key: &PublicKey, // Device 2's public key
) -> Result<WrappedKey, BearDogError> {
    // Some HSMs support exporting keys in encrypted form
    // The key is encrypted with device2's public key
    
    device1.wrap_key(
        key_handle,
        wrapping_key, // Only device2 can unwrap
    ).await
}

// Device 2: Import wrapped key
async fn import_wrapped_key(
    device2: &Fido2Device,
    wrapped_key: &WrappedKey,
) -> Result<KeyHandle, BearDogError> {
    // Device 2 unwraps using its private key
    device2.unwrap_key(wrapped_key).await
}
```

**Status**: ⚠️ **Not commonly supported in FIDO2 keys**
- YubiKey 5 Series: Limited support
- SoloKeys: Check capability (likely no)
- TPM 2.0: Full support
- Enterprise HSMs: Full support

---

#### Option 3: Multi-Signature (Recommended for High Security)

**Concept**: Don't copy keys - use both devices together!

```rust
// Create a 2-of-2 multisig setup
struct MultiDeviceKey {
    device1_key: KeyHandle,
    device2_key: KeyHandle,
}

impl MultiDeviceKey {
    async fn sign(&self, message: &[u8]) -> Result<Signature, BearDogError> {
        // Require BOTH devices to sign
        let sig1 = self.device1.sign(message, &self.device1_key).await?;
        let sig2 = self.device2.sign(message, &self.device2_key).await?;
        
        // Combine signatures (threshold cryptography)
        combine_signatures(&sig1, &sig2)
    }
}
```

**Benefits**:
- ✅ Even stronger security (need both devices)
- ✅ No key copying required
- ✅ Redundancy if one device fails
- ✅ Distributed trust

---

## 🎭 **Question 2: Multiple Credentials per Key (Roles/Permissions)**

### TL;DR: **YES! This is a core FIDO2 feature!**

### FIDO2 Resident Keys (Discoverable Credentials)

**Each SoloKey can store multiple credentials** for different:
- 🏢 Organizations
- 👤 Users/identities
- 🎭 Roles (admin, user, viewer)
- 🔐 Permission levels
- 🌐 Services/relying parties

---

### 📋 **Storage Capacity**

**SoloKey Storage**:
```
Solo 2 Specification:
- Resident Keys: Up to 50+ credentials
- Non-Resident: Unlimited (stored on relying party)
```

---

### 💡 **Implementation: Multi-Role Credentials**

#### Example: BearDog Role-Based Access

```rust
/// Role-based credential system
#[derive(Debug, Clone)]
pub enum BearDogRole {
    SystemAdmin,
    SecurityOfficer,
    Operator,
    Auditor,
    ReadOnly,
}

/// Credential with embedded role
#[derive(Debug)]
pub struct RoleBasedCredential {
    pub credential_id: Vec<u8>,
    pub role: BearDogRole,
    pub permissions: PermissionSet,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Create multiple credentials on one device
async fn setup_multi_role_device(
    device: &Fido2Device,
    user_id: &str,
) -> Result<Vec<RoleBasedCredential>, BearDogError> {
    let mut credentials = Vec::new();
    
    // Admin credential (full permissions)
    let admin_cred = device.make_credential(
        rp_id: "beardog.dev",
        user_id: format!("{}:admin", user_id),
        user_name: "System Administrator",
        display_name: format!("{} (Admin)", user_id),
        algorithms: vec![Algorithm::Ed25519],
        resident_key: true,
    ).await?;
    
    credentials.push(RoleBasedCredential {
        credential_id: admin_cred.id,
        role: BearDogRole::SystemAdmin,
        permissions: PermissionSet::all(),
        user_id: user_id.to_string(),
        created_at: Utc::now(),
        expires_at: None,
    });
    
    // Security Officer credential (security operations only)
    let security_cred = device.make_credential(
        rp_id: "beardog.dev",
        user_id: format!("{}:security", user_id),
        user_name: "Security Officer",
        display_name: format!("{} (Security)", user_id),
        algorithms: vec![Algorithm::Ed25519],
        resident_key: true,
    ).await?;
    
    credentials.push(RoleBasedCredential {
        credential_id: security_cred.id,
        role: BearDogRole::SecurityOfficer,
        permissions: PermissionSet::security(),
        user_id: user_id.to_string(),
        created_at: Utc::now(),
        expires_at: None,
    });
    
    // Operator credential (day-to-day operations)
    let operator_cred = device.make_credential(
        rp_id: "beardog.dev",
        user_id: format!("{}:operator", user_id),
        user_name: "Operator",
        display_name: format!("{} (Operator)", user_id),
        algorithms: vec![Algorithm::Ed25519],
        resident_key: true,
    ).await?;
    
    credentials.push(RoleBasedCredential {
        credential_id: operator_cred.id,
        role: BearDogRole::Operator,
        permissions: PermissionSet::operator(),
        user_id: user_id.to_string(),
        created_at: Utc::now(),
        expires_at: Some(Utc::now() + Duration::days(90)), // Expires
    });
    
    // Read-only auditor credential
    let auditor_cred = device.make_credential(
        rp_id: "beardog.dev",
        user_id: format!("{}:auditor", user_id),
        user_name: "Auditor",
        display_name: format!("{} (Auditor)", user_id),
        algorithms: vec![Algorithm::Ed25519],
        resident_key: true,
    ).await?;
    
    credentials.push(RoleBasedCredential {
        credential_id: auditor_cred.id,
        role: BearDogRole::Auditor,
        permissions: PermissionSet::read_only_with_audit(),
        user_id: user_id.to_string(),
        created_at: Utc::now(),
        expires_at: None,
    });
    
    Ok(credentials)
}
```

---

### 🎯 **Usage Example**

```rust
// Setup a single device with multiple roles
let credentials = setup_multi_role_device(&solokey, "alice").await?;

println!("Created {} credentials on device:", credentials.len());
for cred in &credentials {
    println!("  - {:?}: {:?}", cred.role, cred.permissions);
}

// Later: Authenticate with specific role
async fn authenticate_with_role(
    device: &Fido2Device,
    role: BearDogRole,
) -> Result<AuthenticationToken, BearDogError> {
    // User selects which credential to use
    let challenge = generate_challenge();
    
    // Get assertion with specific credential
    let assertion = device.get_assertion(
        rp_id: "beardog.dev",
        challenge: &challenge,
        // User picks role via button/PIN
        user_presence: true,
    ).await?;
    
    // Verify and extract role from credential
    let cred = lookup_credential(&assertion.credential_id)?;
    
    // Generate token with role-specific permissions
    AuthenticationToken {
        user_id: cred.user_id,
        role: cred.role,
        permissions: cred.permissions,
        valid_until: Utc::now() + Duration::hours(8),
    }
}

// Use in BearDog operations
async fn perform_admin_operation(token: &AuthenticationToken) -> Result<(), BearDogError> {
    if !token.permissions.can_admin() {
        return Err(BearDogError::forbidden(
            "Admin role required for this operation"
        ).with_code(BearDogErrorCode::SEC_1002_FORBIDDEN));
    }
    
    // Proceed with admin operation
    Ok(())
}
```

---

## 🔬 **Experimental Implementation**

### Genetic Key Derivation (BearDog Specific)

**Concept**: Use "genetic" metadata to create role hierarchies

```rust
/// Genetic sample - a unique identity pattern
#[derive(Debug, Clone)]
pub struct GeneticSample {
    pub dna_hash: [u8; 32],        // Unique identifier
    pub generation: u32,            // Derivation depth
    pub traits: GeneticTraits,      // Capabilities
    pub lineage: Vec<[u8; 32]>,    // Parent samples
}

/// Genetic traits define capabilities
#[derive(Debug, Clone)]
pub struct GeneticTraits {
    pub can_spawn: bool,            // Can create child samples
    pub can_admin: bool,            // Admin capabilities
    pub can_sign: bool,             // Signing capability
    pub can_encrypt: bool,          // Encryption capability
    pub sovereignty_level: u8,      // 0-100
}

impl GeneticSample {
    /// Create a root genetic sample on device
    async fn create_root(
        device: &Fido2Device,
        master_seed: &[u8],
    ) -> Result<Self, BearDogError> {
        let dna_hash = hash_seed(master_seed);
        
        let credential = device.make_credential(
            rp_id: "beardog.genetic",
            user_id: hex::encode(&dna_hash),
            user_name: "Root Genetic Sample",
            display_name: "Generation 0 (Root)",
            algorithms: vec![Algorithm::Ed25519],
            resident_key: true,
        ).await?;
        
        Ok(Self {
            dna_hash,
            generation: 0,
            traits: GeneticTraits {
                can_spawn: true,
                can_admin: true,
                can_sign: true,
                can_encrypt: true,
                sovereignty_level: 100,
            },
            lineage: vec![],
        })
    }
    
    /// Spawn a child sample with reduced permissions
    async fn spawn_child(
        &self,
        device: &Fido2Device,
        role: &str,
        traits: GeneticTraits,
    ) -> Result<Self, BearDogError> {
        if !self.traits.can_spawn {
            return Err(BearDogError::forbidden("Cannot spawn children"));
        }
        
        // Derive child DNA from parent
        let child_seed = derive_child_seed(&self.dna_hash, role);
        let child_dna = hash_seed(&child_seed);
        
        let credential = device.make_credential(
            rp_id: "beardog.genetic",
            user_id: hex::encode(&child_dna),
            user_name: format!("Gen {} - {}", self.generation + 1, role),
            display_name: format!("Generation {} ({})", self.generation + 1, role),
            algorithms: vec![Algorithm::Ed25519],
            resident_key: true,
        ).await?;
        
        let mut lineage = self.lineage.clone();
        lineage.push(self.dna_hash);
        
        Ok(Self {
            dna_hash: child_dna,
            generation: self.generation + 1,
            traits,
            lineage,
        })
    }
}
```

### Usage: Genetic Key Hierarchy

```rust
// Create root sample on Device 1
let root = GeneticSample::create_root(&device1, &master_seed).await?;

// Spawn admin child
let admin_sample = root.spawn_child(
    &device1,
    "admin",
    GeneticTraits {
        can_spawn: true,
        can_admin: true,
        can_sign: true,
        can_encrypt: true,
        sovereignty_level: 90,
    },
).await?;

// Spawn operator child (limited permissions)
let operator_sample = admin_sample.spawn_child(
    &device1,
    "operator",
    GeneticTraits {
        can_spawn: false,      // Can't create more keys
        can_admin: false,       // No admin access
        can_sign: true,         // Can sign
        can_encrypt: true,      // Can encrypt
        sovereignty_level: 50,  // Reduced sovereignty
    },
).await?;

// Spawn read-only child
let readonly_sample = operator_sample.spawn_child(
    &device1,
    "readonly",
    GeneticTraits {
        can_spawn: false,
        can_admin: false,
        can_sign: false,        // Can't sign
        can_encrypt: false,     // Can't encrypt
        sovereignty_level: 10,  // Minimal sovereignty
    },
).await?;

// Each sample is a real credential on the device!
println!("Created {} genetic samples on device", 4);
```

---

## 🎭 **Multi-Device Genetic Replication**

### Replicate Genetic Hierarchy Across Devices

```rust
/// Replicate the same genetic hierarchy on both devices
async fn replicate_genetic_hierarchy(
    device1: &Fido2Device,
    device2: &Fido2Device,
    master_seed: &[u8],
    roles: Vec<&str>,
) -> Result<(), BearDogError> {
    // Create identical hierarchies on both devices
    for device in &[device1, device2] {
        let root = GeneticSample::create_root(device, master_seed).await?;
        
        for role in &roles {
            let traits = get_traits_for_role(role);
            let sample = root.spawn_child(device, role, traits).await?;
            
            println!("Device {}: Created {} sample", 
                device.serial_number()?,
                role
            );
        }
    }
    
    Ok(())
}

// Usage:
replicate_genetic_hierarchy(
    &solokey1,
    &solokey2,
    &master_seed,
    vec!["admin", "operator", "auditor"],
).await?;

// Result: Both devices have identical genetic hierarchies
// But private keys never left either device!
```

---

## 📊 **Summary & Recommendations**

### Question 1: Copying Keys ❌➡️✅

**Direct Copying**: ❌ Not possible (by design)

**Alternatives**: ✅
1. **Deterministic Derivation** ⭐ (Best for your use case)
2. **Key Wrapping** (If supported)
3. **Multi-Signature** (Highest security)

**Recommendation**: Use deterministic derivation with `hmac-secret`

---

### Question 2: Multiple Credentials ✅

**YES! Fully supported!**

**Capacity**: 50+ credentials per SoloKey

**Use Cases**:
- ✅ Multiple roles (admin, operator, auditor)
- ✅ Multiple users on same device
- ✅ Multiple services/applications
- ✅ Hierarchical permissions
- ✅ Time-limited credentials
- ✅ **Genetic hierarchies** (BearDog-specific)

---

## 🚀 **Next Experiments**

1. **Implement CTAP2 MakeCredential** - Create first resident key
2. **Test Multi-Credential Storage** - Create 5-10 different roles
3. **Implement Genetic Derivation** - Build key hierarchy
4. **Test Cross-Device Replication** - Same hierarchy on both keys
5. **Implement GetAssertion** - Authenticate with specific credential

---

**Status**: Ready to implement! 🎯

