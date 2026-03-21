# 📱 Android StrongBox Testing Guide
## Pixel 8a with GrapheneOS - Hardware-Agnostic Testing

**Device**: Pixel 8a  
**OS**: GrapheneOS (Android 16)  
**StrongBox**: Level 300 ✅  
**Security Patch**: 2025-07-05

---

## 🎯 BearDog Universal Approach

### Principle: Code Stays Agnostic, Tests Validate Reality

**BearDog Code** (100% Hardware-Agnostic):
```rust
// This code works with SoftHSM, StrongBox, TPM, YubiKey, etc.
let hsm = HsmManager::discover().await?;
let key = hsm.generate_key(KeyParams::Ed25519).await?;
```

**Test Validation** (Hardware-Specific):
```rust
#[tokio::test]
#[ignore = "Requires Android device"]
async fn validate_strongbox_implementation() {
    // Verify BearDog works correctly on StrongBox
    let hsm = discover_any_hsm().await?;
    assert!(hsm.is_hardware_backed());
    run_universal_tests(&hsm).await?;
}
```

---

## 🔧 Setup Requirements

### 1. ADB Connection ✅
```bash
# Already connected!
Device: 44251JEKB04957
Model: Pixel 8a
Android: 16
```

### 2. StrongBox Verification
```bash
# Verify StrongBox features
adb shell pm list features | grep strongbox
# Output: android.hardware.strongbox_keystore=300 ✅
```

### 3. Test App Deployment (If Needed)
```bash
# BearDog can test via:
# Option A: ADB shell commands (keystore operations)
# Option B: Deploy minimal test app
# Option C: Use existing GrapheneOS security features
```

---

## 🧪 Test Strategy

### Phase 1: Capability Detection
```rust
#[tokio::test]
async fn detect_strongbox_capabilities() {
    // BearDog discovers StrongBox automatically
    let hsm = HsmManager::discover().await?;
    
    let caps = hsm.get_capabilities().await?;
    // Should include: KeyGeneration, Signing, HardwareBacked
    assert!(caps.is_hardware_backed);
}
```

### Phase 2: Key Operations
```rust
#[tokio::test]
async fn test_key_lifecycle_on_strongbox() {
    let hsm = discover_any_hsm().await?;
    
    // Generate key (StrongBox specific)
    let key = hsm.generate_key(KeyParams::Ed25519).await?;
    assert!(key.is_hardware_backed);
    
    // Sign data
    let signature = hsm.sign(&key.id, b"test data").await?;
    
    // Verify
    let verified = hsm.verify(&key.id, b"test data", &signature).await?;
    assert!(verified);
    
    // Delete
    hsm.delete_key(&key.id).await?;
}
```

### Phase 3: Performance Comparison
```rust
#[tokio::test]
async fn compare_strongbox_vs_software() {
    // Test same operations on both
    // Compare latency, throughput
    // Document hardware acceleration benefits
}
```

---

## 📊 Expected Capabilities (Pixel 8a)

### StrongBox Features (Level 300)
- ✅ Hardware-backed key generation
- ✅ Ed25519 signatures
- ✅ ECDSA P-256 signatures
- ✅ AES encryption
- ✅ Secure key storage
- ✅ Attestation support

### What Makes StrongBox Special
- Hardware-isolated secure element
- Side-channel attack resistant
- Physical tampering detection
- Verified boot chain
- Higher security than software HSM

---

## 🎯 Testing Commands

### Manual Testing
```bash
# Test StrongBox key operations via ADB
adb shell "keystore_cli_v2 list-keys"
adb shell "keystore_cli_v2 generate-key test-key"
```

### BearDog Universal Testing
```bash
# BearDog discovers and tests StrongBox automatically
export BEARDOG_ENABLE_HARDWARE_TESTS=true
cargo test --test hardware_agnostic_suite -- --include-ignored

# Or specifically prefer mobile HSM
export BEARDOG_PREFER_MOBILE_HSM=true
cargo test android_strongbox
```

---

## ✅ Key Principle

**BearDog Code**: Never knows it's talking to StrongBox  
**Universal API**: Same code works on SoftHSM, StrongBox, TPM, YubiKey  
**Tests**: Validate BearDog works correctly on each platform  

**Result**: Write once, run anywhere, test everywhere! ✅

---

🐻 **BearDog + Pixel 8a StrongBox = Real Hardware Security!** ✨

