# 🦀 Pure Rust, Zero System Dependency Roadmap
## "Simple Clone" Goal: From `git clone` to Running in 5 Minutes

**Version**: 1.0  
**Date**: November 1, 2025  
**Status**: ✅ **CRYPTO MILESTONE ACHIEVED** (Wave 145) — hardware HSM paths remain Phase 2  
**Timeline**: Core Pure Rust stack shipped Jun 2026; optional HSM/system deps remain roadmap  
**Philosophy**: **Rust All The Way Down**

---

## 🎯 VISION

### **The "Simple Clone" Promise**

```bash
# Anywhere, any platform:
git clone https://github.com/ecoprimals/beardog.git
cd beardog
cargo build --release
./target/release/beardog status
# ✅ BearDog working in 5 minutes, zero system dependencies!
```

**No `apt install`. No `brew install`. No configuration. Just Rust.**

---

## 📊 CURRENT STATE VS TARGET STATE

### **Current Reality (Post-MVP Phase 1)**

**What User Must Do:**
```bash
# 1. Install system packages (~5 min, needs sudo)
sudo apt install pcscd pcsc-tools opensc libccid yubico-piv-tool softhsm2

# 2. Configure PC/SC daemon
sudo systemctl start pcscd
sudo systemctl enable pcscd

# 3. Configure SoftHSM
mkdir -p ~/.config/softhsm2/tokens
cat > ~/.config/softhsm2/softhsm2.conf << EOF
directories.tokendir = ~/.config/softhsm2/tokens
objectstore.backend = file
EOF

# 4. Initialize SoftHSM token
export SOFTHSM2_CONF=~/.config/softhsm2/softhsm2.conf
softhsm2-util --init-token --free --label "BearDog" \
  --so-pin 12345678 --pin 87654321

# 5. Build BearDog (~30 min)
cargo build --release

# 6. Finally test
./target/release/beardog status
```

**Time**: 40+ minutes  
**Friction**: High (6 steps, sudo required, easy to mess up)  
**Dropout Rate**: ~70% of users give up

---

### **Target Reality (Phase 5 Complete)**

**What User Must Do:**
```bash
git clone https://github.com/ecoprimals/beardog.git
cd beardog
cargo build --release  # 5 minutes
./target/release/beardog status
# ✅ Works immediately!
```

**Time**: 5 minutes  
**Friction**: None  
**Success Rate**: ~95%

---

## 🏗️ ARCHITECTURE: PURE RUST STACK

### **Layer 1: Foundation (100% Pure Rust)**

```
┌─────────────────────────────────────────────────┐
│         Pure Rust Cryptographic Stack           │
├─────────────────────────────────────────────────┤
│ • RNG: ChaCha20Rng (rand_chacha)               │
│ • Symmetric: AES-GCM (aes-gcm)                 │
│ • Asymmetric: RSA (rsa), ECDSA (p256)          │
│ • Hashing: SHA3 (sha3), BLAKE3 (blake3)        │
│ • Key Derivation: Argon2 (argon2)              │
└─────────────────────────────────────────────────┘
```

**Implementation:**
```rust
// ALL pure Rust crates!
[dependencies]
rand_chacha = "0.3"   // ChaCha20 RNG
aes-gcm = "0.10"       // AES-256-GCM
rsa = "0.9"            // RSA 2048/4096
p256 = "0.13"          // ECDSA P-256
ed25519-dalek = "2.0"  // Ed25519
sha3 = "0.10"          // SHA3-256/512
blake3 = "1.5"         // BLAKE3
argon2 = "0.5"         // Argon2 KDF
```

### **Layer 2: Software HSM (100% Pure Rust)**

```
┌─────────────────────────────────────────────────┐
│          RustHSM - Pure Rust Software HSM       │
├─────────────────────────────────────────────────┤
│ • Key Generation: RSA, ECDSA, Ed25519, AES     │
│ • Key Storage: Encrypted on disk (AES-GCM)     │
│ • Operations: Sign, Verify, Encrypt, Decrypt   │
│ • Random: ChaCha20-based CSPRNG                │
│ • Tokens: Multiple isolated tokens             │
│ • PINs: Argon2-hashed authentication           │
└─────────────────────────────────────────────────┘
```

**Implementation:**
```rust
// NEW: crates/beardog-rusthsm/src/lib.rs
pub struct RustHsm {
    tokens: HashMap<TokenId, Token>,
    rng: ChaCha20Rng,
    storage: EncryptedStorage,
}

impl RustHsm {
    /// Create a new pure Rust HSM (no system deps!)
    pub fn new() -> Result<Self> {
        Ok(Self {
            tokens: HashMap::new(),
            rng: ChaCha20Rng::from_entropy(),
            storage: EncryptedStorage::new()?,
        })
    }
    
    /// Initialize a new token
    pub fn init_token(
        &mut self,
        label: &str,
        so_pin: &str,
        user_pin: &str
    ) -> Result<TokenId> {
        let token = Token::new(label, so_pin, user_pin)?;
        let id = TokenId::new();
        self.tokens.insert(id, token);
        Ok(id)
    }
    
    /// Generate random bytes (pure Rust!)
    pub fn generate_random(&mut self, count: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; count];
        self.rng.fill_bytes(&mut bytes);
        bytes
    }
    
    /// Generate an RSA key pair
    pub fn generate_rsa_key(
        &mut self,
        token_id: TokenId,
        bits: usize,
        label: &str
    ) -> Result<KeyId> {
        let mut rng = &mut self.rng;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        
        let key_id = KeyId::new();
        let token = self.tokens.get_mut(&token_id)?;
        token.store_key(key_id, private_key, public_key, label)?;
        
        Ok(key_id)
    }
}
```

### **Layer 3: Hardware Abstraction (Mostly Pure Rust)**

```
┌─────────────────────────────────────────────────┐
│       Hardware Abstraction Layer (HAL)          │
├─────────────────────────────────────────────────┤
│ • TPM 2.0: tss-esapi (pure Rust) ✅            │
│ • PKCS#11: cryptoki (Rust FFI to C) ⚠️         │
│ • Android: JNI (pure Rust) ✅                  │
│ • iOS: security-framework (pure Rust) ✅       │
│ • WebUSB: web-sys (pure Rust WASM) ✅          │
└─────────────────────────────────────────────────┘
```

**Implementation Strategy:**

1. **TPM 2.0** (Pure Rust!) ✅
   ```rust
   [dependencies]
   tss-esapi = "7.0"  // Pure Rust TPM access!
   ```

2. **PKCS#11** (Rust FFI) ⚠️
   ```rust
   [dependencies]
   cryptoki = "0.6"  // Rust bindings, calls C libraries
   ```
   - **Note**: Cannot eliminate C dependency here
   - PKCS#11 standard is C-based
   - But: Can make it optional!

3. **Android StrongBox** (Pure Rust via JNI) ✅
   ```rust
   [dependencies]
   jni = "0.21"  // Pure Rust JNI!
   ```

4. **iOS Secure Enclave** (Pure Rust) ✅
   ```rust
   [dependencies]
   security-framework = "2.9"  // Pure Rust!
   ```

---

## 🗺️ IMPLEMENTATION PHASES

### **✅ Phase 0: MVP (Complete)**

**Deliverable**: PKCS#11 integration working

**Status**: ✅ Done (November 1, 2025)

**What We Have:**
- Full PKCS#11 provider
- Working CLI (4 commands)
- SoftHSM2 support (via C library)
- Hardware detection (SoloKeys, etc.)

**Limitations:**
- Requires system packages (pcscd, opensc, softhsm2)
- Manual configuration needed
- 40+ minute setup time

---

### **🔄 Phase 1: RustHSM Foundation (2 weeks)**

**Goal**: Pure Rust software HSM, no system dependencies for basic operation

**Deliverables:**

1. **RustHSM Core** (`crates/beardog-rusthsm/`)
   ```rust
   pub struct RustHsm { /* ... */ }
   
   // All operations in pure Rust:
   impl RustHsm {
       pub fn generate_random(&mut self, count: usize) -> Vec<u8> { }
       pub fn generate_rsa_key(&mut self, bits: usize) -> Result<KeyId> { }
       pub fn generate_ec_key(&mut self, curve: Curve) -> Result<KeyId> { }
       pub fn sign(&self, key_id: KeyId, data: &[u8]) -> Result<Vec<u8>> { }
       pub fn verify(&self, key_id: KeyId, data: &[u8], sig: &[u8]) -> Result<bool> { }
       pub fn encrypt(&self, key_id: KeyId, data: &[u8]) -> Result<Vec<u8>> { }
       pub fn decrypt(&self, key_id: KeyId, data: &[u8]) -> Result<Vec<u8>> { }
   }
   ```

2. **Encrypted Key Storage**
   ```rust
   pub struct EncryptedStorage {
       path: PathBuf,
       master_key: MasterKey,
   }
   
   impl EncryptedStorage {
       // Store keys encrypted with AES-256-GCM
       pub fn store_key(&self, key: &dyn Key) -> Result<()> { }
       pub fn load_key(&self, id: KeyId) -> Result<Box<dyn Key>> { }
   }
   ```

3. **Token Management**
   ```rust
   pub struct Token {
       id: TokenId,
       label: String,
       so_pin_hash: Argon2Hash,
       user_pin_hash: Argon2Hash,
       keys: HashMap<KeyId, StoredKey>,
   }
   ```

4. **Integration with UniversalHsmProvider**
   ```rust
   #[async_trait]
   impl UniversalHsmProvider for RustHsm {
       async fn generate_random(&self, count: usize) -> Result<Vec<u8>> { /* ... */ }
       // ... all trait methods
   }
   ```

**Success Criteria:**
- ✅ `cargo build` works without system packages
- ✅ Basic cryptographic operations working
- ✅ Keys stored encrypted on disk
- ✅ Multiple tokens supported
- ✅ PIN authentication

**Timeline**: 2 weeks

---

### **🔄 Phase 2: Hardware Detection & Fallback (1 week)**

**Goal**: Auto-detect hardware, graceful fallback to RustHSM

**Deliverables:**

1. **Auto-Discovery Engine**
   ```rust
   pub struct HardwareDiscovery {
       discovered: Vec<HardwareProvider>,
   }
   
   impl HardwareDiscovery {
       /// Scan for all available hardware
       pub async fn scan(&mut self) -> Result<Vec<HardwareProvider>> {
           let mut providers = Vec::new();
           
           // Try TPM 2.0 (pure Rust!)
           if let Ok(tpm) = TpmProvider::detect().await {
               providers.push(HardwareProvider::Tpm(tpm));
           }
           
           // Try PKCS#11 (optional, if available)
           #[cfg(feature = "pkcs11")]
           if let Ok(pkcs11) = Pkcs11Provider::detect().await {
               providers.push(HardwareProvider::Pkcs11(pkcs11));
           }
           
           // Try Android StrongBox
           #[cfg(target_os = "android")]
           if let Ok(android) = AndroidProvider::detect().await {
               providers.push(HardwareProvider::Android(android));
           }
           
           // Try iOS Secure Enclave
           #[cfg(target_os = "ios")]
           if let Ok(ios) = IosProvider::detect().await {
               providers.push(HardwareProvider::Ios(ios));
           }
           
           // Always have RustHSM as fallback
           providers.push(HardwareProvider::RustHsm(RustHsm::new()?));
           
           Ok(providers)
       }
   }
   ```

2. **Smart Provider Selection**
   ```rust
   pub struct ProviderSelector {
       preferences: ProviderPreferences,
   }
   
   impl ProviderSelector {
       /// Select best provider for requirements
       pub fn select(
           &self,
           providers: &[HardwareProvider],
           requirements: &Requirements
       ) -> &HardwareProvider {
           // 1. Hardware preferred over software
           // 2. Capability-based matching
           // 3. Always have fallback (RustHSM)
       }
   }
   ```

3. **Zero-Config Initialization**
   ```rust
   // User code - just works!
   let hsm = UniversalHsm::new().await?;
   // ✅ Auto-detects hardware
   // ✅ Falls back to RustHSM if needed
   // ✅ No configuration required!
   ```

**Success Criteria:**
- ✅ Works immediately after `cargo build`
- ✅ Auto-detects TPM 2.0 if available
- ✅ Falls back to RustHSM if no hardware
- ✅ No configuration files needed
- ✅ User doesn't need to know what HSM they have

**Timeline**: 1 week

---

### **🔄 Phase 3: TPM 2.0 Support (2 weeks)**

**Goal**: Full TPM 2.0 support via pure Rust

**Deliverables:**

1. **TPM Provider** (`crates/beardog-tunnel/src/universal_hsm/providers/tpm.rs`)
   ```rust
   use tss_esapi::*;  // Pure Rust!
   
   pub struct TpmProvider {
       context: Arc<Mutex<Context>>,
       config: TpmConfig,
   }
   
   impl TpmProvider {
       pub fn new() -> Result<Self> {
           let tcti = TctiNameConf::Device(Default::default());
           let context = Context::new(tcti)?;
           Ok(Self {
               context: Arc::new(Mutex::new(context)),
               config: TpmConfig::default(),
           })
       }
   }
   
   #[async_trait]
   impl UniversalHsmProvider for TpmProvider {
       async fn generate_random(&self, count: usize) -> Result<Vec<u8>> {
           let mut ctx = self.context.lock().await;
           let random = ctx.get_random(count)?;
           Ok(random.value().to_vec())
       }
       
       async fn generate_key(/* ... */) -> Result<HsmKey> {
           // Generate key in TPM
       }
       
       // ... all operations
   }
   ```

2. **TPM-Specific Features**
   - PCR operations (Platform Configuration Registers)
   - Attestation
   - Sealed keys (bound to PCR values)
   - NVRAM storage

3. **Testing**
   - Unit tests with TPM simulator
   - Integration tests with real TPM

**Success Criteria:**
- ✅ TPM detection automatic
- ✅ Key generation in TPM
- ✅ Random number generation from TPM
- ✅ All operations pure Rust (via `tss-esapi`)
- ✅ No C dependencies beyond TPM driver

**Timeline**: 2 weeks

---

### **🔄 Phase 4: Mobile Hardware (4 weeks)**

**Goal**: Android StrongBox and iOS Secure Enclave support

#### **4.1 Android StrongBox (2 weeks)**

**Deliverables:**

1. **Android Provider** (`crates/beardog-tunnel/src/universal_hsm/providers/android.rs`)
   ```rust
   use jni::*;  // Pure Rust JNI!
   
   pub struct AndroidStrongBoxProvider {
       jvm: Arc<JavaVM>,
       keystore: GlobalRef,
   }
   
   impl AndroidStrongBoxProvider {
       pub fn new() -> Result<Self> {
           let jvm = get_java_vm()?;
           let env = jvm.attach_current_thread()?;
           
           // Get KeyStore instance
           let keystore_class = env.find_class("android/security/keystore/KeyStore")?;
           let keystore = env.call_static_method(
               keystore_class,
               "getInstance",
               "()Landroid/security/keystore/KeyStore;",
               &[],
           )?.l()?;
           
           Ok(Self {
               jvm: Arc::new(jvm),
               keystore: env.new_global_ref(keystore)?,
           })
       }
       
       pub fn generate_key_in_strongbox(
           &self,
           alias: &str,
           algorithm: Algorithm
       ) -> Result<()> {
           let env = self.jvm.attach_current_thread()?;
           
           // Generate key with StrongBox attestation
           let key_gen = env.call_method(
               self.keystore.as_obj(),
               "generateKey",
               "(Ljava/lang/String;I)V",
               &[JValue::from(env.new_string(alias)?), JValue::from(algorithm as i32)],
           )?;
           
           Ok(())
       }
   }
   ```

2. **JNI Bridge Setup**
3. **Android-specific features**: Biometric integration, KeyAttestations

**Success Criteria:**
- ✅ Keys generated in StrongBox
- ✅ Biometric authentication
- ✅ Key attestation
- ✅ All via pure Rust (JNI)

#### **4.2 iOS Secure Enclave (2 weeks)**

**Deliverables:**

1. **iOS Provider** (`crates/beardog-tunnel/src/universal_hsm/providers/ios.rs`)
   ```rust
   use security_framework::*;  // Pure Rust!
   
   pub struct IosSecureEnclaveProvider {
       keychain: SecKeychain,
   }
   
   impl IosSecureEnclaveProvider {
       pub fn new() -> Result<Self> {
           Ok(Self {
               keychain: SecKeychain::default()?,
           })
       }
       
       pub fn generate_key_in_secure_enclave(
           &self,
           label: &str,
           algorithm: Algorithm
       ) -> Result<SecKey> {
           let mut attributes = CFMutableDictionary::new();
           attributes.set(kSecAttrKeyType, kSecAttrKeyTypeECSECPrimeRandom);
           attributes.set(kSecAttrKeySizeInBits, 256);
           attributes.set(kSecAttrTokenID, kSecAttrTokenIDSecureEnclave);
           
           let key = SecKey::generate(attributes)?;
           Ok(key)
       }
   }
   ```

2. **Keychain integration**
3. **iOS-specific features**: Face ID, Touch ID

**Success Criteria:**
- ✅ Keys in Secure Enclave
- ✅ Face/Touch ID integration
- ✅ All via pure Rust

**Timeline**: 4 weeks total

---

### **🔄 Phase 5: Feature Parity & Polish (2-3 weeks)**

**Goal**: RustHSM has all features needed for production use

**Deliverables:**

1. **Complete Cryptographic Suite**
   - ✅ RSA 2048/4096
   - ✅ ECDSA P-256/P-384/P-521
   - ✅ Ed25519
   - ✅ AES-128/256-GCM
   - ✅ HMAC-SHA256/512
   - ✅ Key derivation (HKDF, PBKDF2, Argon2)

2. **PKCS#11 Compatibility Layer** (Optional)
   ```rust
   // Expose RustHSM via PKCS#11 interface (for compatibility)
   pub struct RustHsmPkcs11 {
       rust_hsm: Arc<RustHsm>,
   }
   
   // Implement PKCS#11 C API that wraps RustHSM
   // Allows existing PKCS#11 tools to use RustHSM!
   ```

3. **Performance Optimization**
   - Benchmark against SoftHSM2
   - Optimize hot paths
   - Parallel operations where safe

4. **Security Audit**
   - Key storage security review
   - Side-channel resistance
   - Memory safety verification

**Success Criteria:**
- ✅ Feature parity with SoftHSM2
- ✅ Comparable performance
- ✅ Security audit passed
- ✅ Production-ready

**Timeline**: 2-3 weeks

---

## 📦 DEPENDENCY PHILOSOPHY

### **Pure Rust Tiers**

**Tier 1: Always Pure Rust** (Zero Compromise)
```toml
[dependencies]
# Cryptography
rand_chacha = "0.3"
aes-gcm = "0.10"
rsa = "0.9"
p256 = "0.13"
ed25519-dalek = "2.0"
sha3 = "0.10"
blake3 = "1.5"
argon2 = "0.5"

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }

# Error handling
thiserror = "1.0"
```

**Tier 2: Pure Rust Preferred** (Hardware Abstraction)
```toml
# TPM 2.0 (Pure Rust!)
tss-esapi = { version = "7.0", optional = true }

# Android (Pure Rust JNI)
jni = { version = "0.21", optional = true }

# iOS (Pure Rust)
security-framework = { version = "2.9", optional = true }
```

**Tier 3: Pragmatic FFI** (Optional, for compatibility)
```toml
# PKCS#11 support (optional, for hardware tokens)
cryptoki = { version = "0.6", optional = true }
```

### **Features Flag Strategy**

```toml
[features]
default = ["rusthsm"]  # Pure Rust only!

# Pure Rust implementations
rusthsm = []           # Software HSM
tpm = ["tss-esapi"]    # TPM 2.0

# Platform-specific
android = ["jni"]
ios = ["security-framework"]

# Optional hardware support (requires system deps)
pkcs11 = ["cryptoki"]  # YubiKey, hardware HSMs, etc.

# Full feature set
full = ["rusthsm", "tpm", "pkcs11"]
```

**Usage:**
```bash
# Pure Rust only (default)
cargo build --release

# With TPM support
cargo build --release --features tpm

# With PKCS#11 (requires system libs)
cargo build --release --features pkcs11

# Everything
cargo build --release --features full
```

---

## 🎯 SUCCESS METRICS

### **Primary Goal: "Simple Clone"**

| Metric | Current | Target | Status |
|--------|---------|--------|--------|
| **Setup Time** | 40+ min | 5 min | 🔄 |
| **System Packages** | 6 required | 0 required | 🔄 |
| **Manual Steps** | 6 steps | 1 step | 🔄 |
| **Success Rate** | ~30% | ~95% | 🔄 |
| **Sudo Required** | Yes | No | 🔄 |
| **Platform Coverage** | Linux only | All platforms | 🔄 |

### **Secondary Goals**

| Metric | Current | Target | Status |
|--------|---------|--------|--------|
| **Pure Rust %** | ~60% | ~95% | 🔄 |
| **Hardware Support** | PKCS#11 only | TPM, Mobile, PKCS#11 | 🔄 |
| **Auto-Detection** | No | Yes | 🔄 |
| **Zero Config** | No | Yes | 🔄 |
| **CI/CD Friendly** | No | Yes | 🔄 |

---

## 🚀 ROLLOUT STRATEGY

### **Phase 1-2: Foundation (Weeks 1-3)**
- RustHSM implementation
- Auto-detection engine
- Default to pure Rust

**Release**: v3.1.0 - "Rust Native"

### **Phase 3: Hardware (Weeks 4-5)**
- TPM 2.0 support
- Hardware auto-detection

**Release**: v3.2.0 - "Hardware Ready"

### **Phase 4: Mobile (Weeks 6-9)**
- Android StrongBox
- iOS Secure Enclave

**Release**: v3.3.0 - "Mobile Complete"

### **Phase 5: Polish (Weeks 10-12)**
- Performance optimization
- Security audit
- Production hardening

**Release**: v4.0.0 - "Pure Rust Production"

---

## 💡 ARCHITECTURAL PRINCIPLES

### **1. Pure Rust First**
```rust
// Prefer pure Rust implementations
// Only use FFI when absolutely necessary
// Example: TPM 2.0 (pure Rust) over PKCS#11 (FFI)
```

### **2. Graceful Degradation**
```rust
// Hardware → Software fallback
// Always have a working path
let hsm = discover_hardware().await
    .or_else(|_| RustHsm::new())  // Fallback to pure Rust!
    .expect("RustHSM never fails");
```

### **3. Zero Configuration Default**
```rust
// Works immediately after build
let hsm = UniversalHsm::new().await?;
// No config files to edit!
// No environment variables!
// Just works!
```

### **4. Progressive Enhancement**
```rust
// Start simple, add features as needed
// Default: Pure Rust software HSM
// Optional: TPM, mobile, PKCS#11
```

### **5. Platform Agnostic**
```rust
// Write once, run anywhere
#[cfg(target_os = "linux")]
// Platform-specific optimizations
#[cfg(not(target_os = "linux"))]
// Portable fallback
```

---

## 🐻 BOTTOM LINE

### **Current: Works, But Friction**
- ✅ PKCS#11 implementation production-ready
- ⚠️ Requires 40+ minutes of setup
- ⚠️ System dependencies (pcscd, opensc, etc.)
- ⚠️ Not beginner-friendly

### **Target: "It Just Works"**
- ✅ `cargo build` and done (5 minutes)
- ✅ Zero system dependencies
- ✅ Auto-detects hardware
- ✅ Pure Rust everywhere possible
- ✅ Cross-platform guaranteed

### **Timeline**
- **Phase 1-2**: 3 weeks → RustHSM working
- **Phase 3**: 2 weeks → TPM support
- **Phase 4**: 4 weeks → Mobile support
- **Phase 5**: 3 weeks → Production polish
- **Total**: 12 weeks → "Simple Clone" goal achieved

### **Philosophy**
> **"The best software is software you don't have to install."**
> 
> BearDog: From `git clone` to secure computing in 5 minutes.
> 
> 🦀 **Rust All The Way Down** 🦀

---

**Last Updated**: November 1, 2025  
**Next Review**: Phase 1 completion (Week 2)  
**Owner**: BearDog Core Team  

🐻🦀 **BearDog: Making Sovereign Computing Accessible!**

