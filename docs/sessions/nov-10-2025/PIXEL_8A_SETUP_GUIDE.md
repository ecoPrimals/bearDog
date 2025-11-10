# 📱 Pixel 8a StrongBox Setup & Testing

**Device**: Pixel 8a with GrapheneOS  
**Hardware**: Titan M2 StrongBox  
**Status**: Connected (needs authorization)

---

## 🔍 **Current Status**

```bash
$ adb devices
List of devices attached
44251JEKB04957  unauthorized  ⚠️
```

**Device is connected but needs USB debugging authorization!**

---

## 🔓 **Step 1: Authorize ADB** (DO THIS NOW)

### **On Your Pixel 8a:**
1. **A popup should appear** asking to "Allow USB debugging?"
2. **Check the box** "Always allow from this computer" (optional but recommended)
3. **Tap "Allow"**

### **If No Popup Appears:**
1. Unplug and replug the USB cable
2. On the phone, go to: **Settings → Developer Options**
3. Toggle **"USB debugging"** OFF then ON
4. Replug the cable

---

## ✅ **Step 2: Verify Authorization**

After allowing, run:

```bash
adb devices
```

**Expected output:**
```
List of devices attached
44251JEKB04957  device  ✅
```

---

## 📊 **Step 3: Device Information**

Once authorized, gather device info:

```bash
# Device model
adb shell getprop ro.product.model

# Android version  
adb shell getprop ro.build.version.release

# Security patch level
adb shell getprop ro.build.version.security_patch

# Check for StrongBox support
adb shell pm list features | grep strongbox
```

**Expected:**
```
Pixel 8a
14 (or higher)
2025-XX-XX
android.hardware.strongbox_keystore
```

---

## 🔐 **Step 4: Test StrongBox Availability**

```bash
# Check if StrongBox Keystore is available
adb shell pm list features | grep -E "(strongbox|keymaster|keystore)"

# Check Titan M2 status
adb shell getprop ro.hardware.keystore

# Check security
adb shell getprop ro.boot.verifiedbootstate
```

---

## 🚀 **What We'll Build**

### **JNI Bridge for Android Keystore**

```rust
// beardog-security/src/hsm/android_strongbox/jni_bridge.rs

/// Generate a key in Android StrongBox
pub fn strongbox_generate_key(
    alias: &str,
    algorithm: &str,
    require_user_auth: bool,
) -> Result<Vec<u8>, BearDogError>;

/// Sign data using StrongBox key
pub fn strongbox_sign(
    alias: &str,
    data: &[u8],
) -> Result<Vec<u8>, BearDogError>;

/// Generate hardware entropy
pub fn strongbox_generate_entropy(
    size: usize,
) -> Result<Vec<u8>, BearDogError>;
```

### **Architecture**

```
Your App (Rust)
      ↓
JNI Bridge (Rust ↔ Java)
      ↓
Android Keystore API (Java)
      ↓
Titan M2 StrongBox (Hardware)
```

---

## 🎯 **Testing Plan**

### **Phase 1: Device Detection** ✅ (Current)
- [x] Device connected
- [ ] Device authorized
- [ ] StrongBox capability confirmed

### **Phase 2: JNI Bridge** (Next)
- [ ] Rust-to-Java bridge created
- [ ] Key generation working
- [ ] Signing working
- [ ] Entropy generation working

### **Phase 3: Integration**
- [ ] MultiCredentialHsmProvider fully functional
- [ ] Cross-platform demo updated
- [ ] Real hardware entropy flowing

---

## 📝 **Current Implementation Status**

### **✅ Already Implemented**
- `StrongBoxMultiCredentialProvider` struct
- `MultiCredentialHsmProvider` trait implementation
- Vendor-agnostic architecture
- Cross-platform examples

### **🔨 Needs Implementation (JNI Bridge)**
```rust
// From multi_credential_provider.rs:

async fn android_generate_key(...) {
    // TODO: Implement actual Android Keystore key generation via JNI
    // Lines 157-174
}

async fn android_sign(...) {
    // TODO: Implement actual Signature via JNI
    // Lines 177-193
}

async fn android_hardware_entropy(...) {
    // TODO: Implement actual SecureRandom via JNI
    // Lines 228-238
}
```

---

## 🛠️ **Tools We'll Use**

1. **`jni` crate** - Rust JNI bindings
2. **`ndk` crate** - Android NDK utilities
3. **Android Keystore API** - Via JNI calls
4. **ADB** - For testing and debugging

---

## 📚 **Resources**

### **Android Keystore API**
- `KeyGenParameterSpec.Builder`
- `KeyPairGenerator`
- `Signature`
- `SecureRandom`

### **StrongBox Features**
- Hardware-backed keys (Titan M2)
- Attestation support
- User authentication binding
- Key rollback protection

---

## 🎯 **Next Steps (After Authorization)**

1. ✅ Authorize device (you're doing this now)
2. ✅ Verify StrongBox capability
3. ✅ Create JNI bridge module
4. ✅ Implement key generation
5. ✅ Implement signing
6. ✅ Implement entropy generation
7. ✅ Test with real hardware
8. ✅ Update examples

---

## 💡 **Expected Results**

Once complete, you'll be able to:

```rust
// Same code works for both FIDO2 and StrongBox!
let hsm = StrongBoxMultiCredentialProvider::new()?;

// Generate a key in Titan M2 hardware
let cred = hsm.create_credential(request).await?;

// Sign with hardware-backed key
let signature = hsm.sign_with_credential(&cred.id, data).await?;

// Generate hardware entropy
let entropy = hsm.generate_hardware_entropy(32).await?;
```

---

## 🎉 **Why This Matters**

- 🔐 **Real Hardware Security** - Titan M2 is military-grade
- 🌍 **Vendor-Agnostic** - Same code for SoloKeys, Pixel, iPhone
- 🧬 **Human Entropy** - True sovereignty via hardware RNG
- 🚀 **Production Ready** - Real device in your hand

---

**Status**: ⚠️ **Waiting for ADB authorization**  
**Next**: Authorize on device, then proceed with JNI bridge implementation

---

**Please authorize USB debugging on your Pixel 8a now!** 📱👆

