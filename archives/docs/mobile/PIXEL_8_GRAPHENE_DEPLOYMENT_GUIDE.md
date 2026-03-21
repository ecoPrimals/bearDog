# BearDog Pixel 8 GrapheneOS Deployment Guide

**Version**: 1.0  
**Date**: January 2025  
**Status**: ✅ **IMPLEMENTATION READY** - Hardware HSM integration prepared  

## 🎯 **Overview**

This guide walks you through deploying BearDog's hardware security module (HSM) on your Pixel 8 device running GrapheneOS. BearDog leverages the Titan M security chip and Android StrongBox to provide hardware-backed cryptographic operations.

## 📱 **Hardware Requirements**

### **✅ Validated Hardware**
- **Device**: Google Pixel 8 or Pixel 8 Pro
- **Security Chip**: Titan M (hardware security module)
- **Verified Boot**: GREEN state (locked bootloader)
- **StrongBox**: Android StrongBox HSM implementation

### **✅ Software Requirements**
- **OS**: GrapheneOS (latest stable)
- **Android API Level**: 28+ (Android 9+)
- **Build Tools**: Android NDK, Rust Android targets
- **Development**: ADB access for deployment

## 🔧 **Pre-Installation Setup**

### **Step 1: Verify GrapheneOS Installation**

Ensure your Pixel 8 is properly running GrapheneOS:

```bash
# Check device via ADB
adb shell getprop ro.build.fingerprint
# Should show GrapheneOS build signature

# Verify verified boot state  
adb shell getprop ro.boot.verifiedbootstate
# Should return "green"

# Check security patch level
adb shell getprop ro.build.version.security_patch
# Should show recent date
```

### **Step 2: Enable Developer Options**

1. **Settings** → **About phone** 
2. Tap **Build number** 7 times
3. **Settings** → **System** → **Developer options**
4. Enable **USB debugging**
5. Enable **Wireless debugging** (optional)

### **Step 3: Verify StrongBox Availability**

```bash
# Check for StrongBox support
adb shell pm list features | grep strongbox
# Should show: feature:android.hardware.strongbox_keystore

# Check Titan M availability  
adb shell getprop ro.hardware.keystore
# Should show hardware backing
```

## 🚀 **BearDog Installation**

### **Option 1: Quick Setup (Recommended)**

The simplest way to get BearDog running on your Pixel 8:

```rust
use beardog::tunnel::hsm::android_strongbox::setup_pixel8_beardog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This handles everything automatically
    let (hsm, anchor_key) = setup_pixel8_beardog().await?;
    
    println!("🎉 BearDog HSM initialized!");
    println!("Anchor key: {}", anchor_key.id);
    
    Ok(())
}
```

### **Option 2: Custom Configuration**

For fine-tuned security requirements:

```rust
use beardog::tunnel::hsm::android_strongbox::{
    Pixel8GrapheneOSConfig, Pixel8GrapheneOSSetup, Pixel8PerformanceMode
};
use beardog::tunnel::hsm::types::SecurityLevel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure for your security requirements
    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,                              // Require Titan M
        require_green_boot: true,                           // Require verified boot
        enable_attestation: true,                           // Enable attestation
        security_level: SecurityLevel::Maximum,            // Maximum security
        performance_mode: Pixel8PerformanceMode::MaxSecurity, // All operations in hardware
    };
    
    // Initialize BearDog with custom config
    let setup = Pixel8GrapheneOSSetup::new(config).await?;
    let hsm = setup.initialize_hsm().await?;
    let anchor_key = setup.create_anchor_key(&hsm).await?;
    
    // Generate device identity for ecosystem
    let identity = setup.generate_ecosystem_identity(&hsm).await?;
    
    println!("🌐 Device ID: {}", identity.device_id);
    println!("🔐 Anchor Key: {}", anchor_key.id);
    
    Ok(())
}
```

## 🔐 **Security Configuration**

### **Maximum Security Mode (Recommended)**

For highest security with your Pixel 8:

```rust
Pixel8GrapheneOSConfig {
    require_titan_m: true,        // ✅ Require Titan M chip
    require_green_boot: true,     // ✅ Require GREEN verified boot
    enable_attestation: true,     // ✅ Enable key attestation
    security_level: SecurityLevel::Maximum,
    performance_mode: Pixel8PerformanceMode::MaxSecurity,
}
```

**Security Guarantees:**
- ✅ All keys hardware-backed in Titan M
- ✅ Keys cannot be extracted from device
- ✅ Operations verified in secure hardware
- ✅ Attestation proves hardware backing
- ✅ User presence required for sensitive operations

### **Balanced Mode**

For good security with better performance:

```rust
Pixel8GrapheneOSConfig {
    require_titan_m: true,
    require_green_boot: true,
    enable_attestation: true,
    security_level: SecurityLevel::High,
    performance_mode: Pixel8PerformanceMode::Balanced,
}
```

## 🧪 **Testing Your Installation**

### **Basic Functionality Test**

Run the included demo to verify everything works:

```bash
# Build for Android (if cross-compiling)
cargo build --target aarch64-linux-android --example pixel8_grapheneos_demo

# Or run directly on device
cargo run --example pixel8_grapheneos_demo
```

### **Expected Output**

```
🚀 BearDog Pixel 8 GrapheneOS Demo Starting
==================================================

📱 Demo 1: Quick Setup
---------------------
🔐 Initializing Pixel 8 GrapheneOS HSM setup
📱 Detected Pixel device - excellent StrongBox support
✅ Verified boot state: GREEN - optimal security
✅ Android StrongBox HSM initialized successfully
🎉 Android StrongBox HSM operational on Pixel 8!
✅ Quick setup complete!

🧪 Testing basic operations with anchor key...
✅ Data signed successfully (64 byte signature)
✅ Signature verification passed
📋 HSM Info:
   Vendor: Google
   Model: Pixel 8
   Capabilities: [KeyGeneration, Signing, Encryption, KeyAttestation]
```

### **Verification Commands**

Verify your BearDog installation:

```bash
# Check if keys are in StrongBox
adb shell keystore_cli_v2 list-keys
# Should show beardog-prefixed keys

# Verify attestation (requires root on device)
adb shell su -c "cat /proc/crypto"
# Should show hardware crypto modules
```

## 🔧 **Advanced Configuration**

### **Biometric Authentication**

Enable fingerprint/face unlock for sensitive operations:

```rust
KeyUsagePolicy {
    requires_biometric: Some(true),
    requires_user_presence: Some(true),
    // ... other settings
}
```

### **Custom Key Types**

Generate different types of keys for different purposes:

```rust
// Signing key (ECC P-256 - optimal for mobile)
GenerateKeyRequest {
    key_type: KeyType::EccP256,
    usage_policy: KeyUsagePolicy {
        can_sign: true,
        can_verify: true,
        exportable: false,
        // ...
    },
    // ...
}

// Encryption key (AES-256)
GenerateKeyRequest {
    key_type: KeyType::Aes256,
    usage_policy: KeyUsagePolicy {
        can_encrypt: true,
        can_decrypt: true,
        exportable: false,
        // ...
    },
    // ...
}
```

### **Ecosystem Integration**

Configure BearDog as security anchor for ecosystem:

```rust
// Generate ecosystem identity
let identity = setup.generate_ecosystem_identity(&hsm).await?;

println!("🌐 Device capabilities: {:?}", identity.capabilities);
// Shows: ["StrongBox", "TitanM", "HardwareAttestation", "BiometricAuth", "VerifiedBoot"]
```

## ⚠️ **Security Considerations**

### **✅ Strengths**

- **Hardware-backed keys**: Keys stored in Titan M, cannot be extracted
- **Verified boot**: GrapheneOS provides GREEN verified boot state
- **Attestation**: Cryptographic proof of hardware backing
- **Tamper resistance**: Titan M provides hardware tamper detection
- **Privacy**: GrapheneOS provides enhanced privacy controls

### **⚠️ Important Notes**

- **No key backup**: StrongBox keys cannot be backed up or restored
- **Device binding**: Keys are permanently bound to this specific device
- **User presence**: Some operations may require user interaction
- **Performance**: Hardware operations slower than software alternatives
- **Root access**: Some advanced features require root access

### **🔒 Best Practices**

1. **Keep GREEN boot state** - Never unlock bootloader in production
2. **Regular updates** - Keep GrapheneOS updated for security patches
3. **Backup strategy** - Plan for device replacement (keys cannot migrate)
4. **Test thoroughly** - Verify all operations work before production use
5. **Monitor attestation** - Regularly verify key attestation certificates

## 🚨 **Troubleshooting**

### **Common Issues**

**Issue**: `StrongBox not available`
```bash
# Solution: Verify device supports StrongBox
adb shell pm list features | grep strongbox
```

**Issue**: `Verified boot not GREEN`
```bash
# Solution: Check bootloader status
adb shell getprop ro.boot.verifiedbootstate
# If not "green", device may have unlocked bootloader
```

**Issue**: `Permission denied for keystore`
```bash
# Solution: Check app permissions and developer options
# Ensure USB debugging is enabled and device is authorized
```

**Issue**: `Titan M not detected`
```bash
# Solution: Verify hardware
adb shell getprop ro.hardware.keystore
# Should show "trusty" or similar hardware backing
```

### **Debug Mode**

Enable verbose logging for troubleshooting:

```rust
// Add to your main function
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

## 📊 **Performance Characteristics**

### **Typical Operation Times on Pixel 8**

| Operation | Typical Time | Notes |
|-----------|--------------|--------|
| **Key Generation** | 50-200ms | ECC faster than RSA |
| **ECDSA Signing** | 5-20ms | Hardware-optimized |
| **Signature Verification** | 3-15ms | Very fast |
| **AES Encryption** | 1-10ms | Excellent performance |
| **Key Attestation** | 100-500ms | Includes certificate chain |

### **Recommended Usage Patterns**

- **Generate keys once** - Key generation is slow, cache key references
- **Batch operations** - Group multiple signing operations together  
- **Use appropriate algorithms** - ECC P-256 optimal for mobile
- **Cache attestations** - Attestation generation is expensive

## 🎯 **Next Steps**

Once BearDog is running on your Pixel 8:

1. **🔗 Ecosystem Integration** - Connect to other BearDog nodes
2. **🔑 Key Management** - Set up key rotation and lifecycle policies
3. **📊 Monitoring** - Implement health checks and alerting
4. **🔧 Automation** - Create deployment scripts for your environment
5. **🧪 Testing** - Set up continuous integration with hardware testing

## 📞 **Support**

If you encounter issues:

1. **Check logs** - Enable DEBUG logging for detailed information
2. **Verify hardware** - Ensure Pixel 8 has required security features
3. **Test step-by-step** - Use the provided demo to isolate issues
4. **Community support** - Check BearDog documentation and forums

---

**🎉 Congratulations!** You now have BearDog running with hardware security on your Pixel 8 GrapheneOS device! Your cryptographic operations are backed by the Titan M security chip, providing enterprise-grade security in a mobile form factor. 