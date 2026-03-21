# 🔑 Solo 2 Security Key Testing Guide
## FIDO2/U2F Hardware Token Testing

**Devices**: 2x Solo 2 Security Keys  
**USB ID**: 1209:beee  
**Protocol**: FIDO2/CTAP2, U2F  
**Status**: ✅ Detected

---

## 🎯 BearDog Universal Approach

### Hardware-Agnostic FIDO2 Integration

**BearDog Code** (Hardware-Agnostic):
```rust
// Works with SoloKey, YubiKey, any FIDO2 token
let fido_hsm = HsmManager::discover_fido2().await?;
let credential = fido_hsm.create_credential(params).await?;
let assertion = fido_hsm.get_assertion(challenge).await?;
```

**The code never knows which specific token it's using!** ✅

---

## 🔧 Setup Requirements

### 1. USB Detection ✅
```
Bus 001 Device 007: ID 1209:beee Generic Solo 2 Security Key
Bus 001 Device 005: ID 1209:beee Generic Solo 2 Security Key
HID Devices: /dev/hidraw5, /dev/hidraw6
```

### 2. Required Libraries
```bash
# Install FIDO2 libraries
sudo apt-get install -y libfido2-dev libfido2-1 fido2-tools

# Test token detection
fido2-token -L

# Get token info
fido2-token -I /dev/hidraw5
```

### 3. Permissions
```bash
# Add udev rules for non-root access
sudo tee /etc/udev/rules.d/70-solo2.rules > /dev/null << 'UDEV'
# Solo 2 Security Keys
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="beee", MODE="0660", TAG+="uaccess"
UDEV

sudo udevadm control --reload-rules
sudo udevadm trigger
```

---

## 🧪 Test Strategy

### Phase 1: Token Discovery
```rust
#[tokio::test]
async fn discover_fido2_tokens() {
    // BearDog discovers all connected FIDO2 tokens
    let tokens = HsmManager::discover_all_fido2().await?;
    assert!(!tokens.is_empty(), "Should find Solo 2 keys");
}
```

### Phase 2: Credential Management
```rust
#[tokio::test]
async fn test_credential_lifecycle() {
    let token = discover_any_fido2_token().await?;
    
    // Create credential (requires user presence - touch button)
    let cred = token.make_credential(MakeCredentialParams {
        rp: "beardog.test",
        user: "test@example.com",
        alg: Algorithm::Ed25519,
    }).await?;
    
    // Get assertion
    let assertion = token.get_assertion(GetAssertionParams {
        rp: "beardog.test",
        challenge: &random_challenge(),
    }).await?;
    
    assert!(assertion.verify());
}
```

### Phase 3: Multi-Token Testing
```rust
#[tokio::test]
async fn test_multiple_tokens() {
    // Discover both Solo 2 keys
    let tokens = discover_all_fido2_tokens().await?;
    assert_eq!(tokens.len(), 2, "Should find 2 Solo 2 keys");
    
    // BearDog can use either token interchangeably
    for (i, token) in tokens.iter().enumerate() {
        println!("Testing on token {}", i+1);
        run_universal_tests(token).await?;
    }
}
```

---

## 📋 Solo 2 Capabilities

### What Solo 2 Supports
- ✅ FIDO2/WebAuthn
- ✅ U2F (legacy)
- ✅ CTAP2 protocol
- ✅ Ed25519 signatures
- ✅ ECDSA P-256 signatures
- ✅ Resident keys
- ✅ User presence detection
- ✅ PIN protection

### What's Unique About Solo 2
- Open source firmware
- No vendor lock-in
- Community-driven development
- Secure element chip
- Physical button confirmation

---

## 🎯 Testing Commands

### Manual Token Testing
```bash
# List available tokens
fido2-token -L

# Get token info
fido2-token -I /dev/hidraw5
fido2-token -I /dev/hidraw6

# Test credential creation (requires button press)
fido2-cred -M -i /dev/hidraw5
```

### BearDog Universal Testing
```bash
# BearDog discovers and tests on Solo 2 automatically
export BEARDOG_ENABLE_HARDWARE_TESTS=true
cargo test --test hardware_agnostic_suite -- --include-ignored

# Or specifically test FIDO2 tokens
export BEARDOG_PREFER_FIDO2=true
./scripts/test-on-hardware.sh
```

---

## ⚠️ User Interaction Required

**Important**: FIDO2 tokens require **physical button press** for operations:
- Creating credentials
- Signing operations
- PIN entry (if configured)

**Test Strategy**:
```rust
// Tests should inform user when button press needed
println!("👆 Please touch the Solo 2 button to confirm...");
tokio::time::sleep(Duration::from_secs(5)).await; // Give user time
```

---

## 🔒 Security Benefits

### Why Test on Real Hardware
1. **Real Security**: Actual secure element, not simulation
2. **User Experience**: Test physical button interaction
3. **Compatibility**: Verify BearDog works with real FIDO2
4. **Performance**: Real-world latency measurements
5. **Confidence**: Production validation

### Hardware-Agnostic Benefit
- Same BearDog code works with YubiKey, Titan, etc.
- No recompilation needed for different tokens
- User can swap hardware without code changes

---

## ✅ Key Principle

**BearDog Code**: Never hardcoded to Solo 2 ✅  
**Universal Discovery**: Finds ANY FIDO2 token ✅  
**Testing**: Validates on real Solo 2 hardware ✅  
**Result**: Works with Solo 2, YubiKey, Titan, etc. ✅  

---

🐻 **BearDog + Solo 2 = Universal FIDO2 Security!** ✨

**Remember**: BearDog stays universal, we just validate it works! 🔑

