# 🤖 BearDog Android Setup Guide

**Target:** Pixel 8a (GrapheneOS with StrongBox HSM)  
**Phase:** Phase 2 Preparation  
**Status:** Ready for implementation  
**Updated:** October 29, 2025

---

## 🎯 **Overview**

This guide covers setting up BearDog for Android development, specifically targeting the Pixel 8a with GrapheneOS and its StrongBox hardware-backed keystore (Titan M2 chip).

---

## 📋 **Prerequisites**

### **Development Machine (Eastgate):**
- ✅ Linux (Ubuntu/Debian)
- ✅ Rust toolchain (already installed)
- ✅ BearDog workspace (already set up)
- ⏳ Android NDK (to be installed)
- ⏳ Android SDK tools (to be installed)

### **Target Device:**
- **Device:** Google Pixel 8a
- **OS:** GrapheneOS (Android-based)
- **Security:** StrongBox HSM (Titan M2)
- **API Level:** 33+ (Android 13+)
- **Features:** Hardware-backed keystore, attestation

---

## 🔧 **Step 1: Install Android NDK**

### **1.1: Download Android SDK Command Line Tools**

```bash
cd ~/Downloads

# Download latest command line tools
wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip

# Create SDK directory
mkdir -p ~/Android/Sdk/cmdline-tools
cd ~/Android/Sdk/cmdline-tools

# Extract tools
unzip ~/Downloads/commandlinetools-linux-9477386_latest.zip
mv cmdline-tools latest

# Add to PATH
echo 'export ANDROID_HOME=$HOME/Android/Sdk' >> ~/.bashrc
echo 'export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin' >> ~/.bashrc
echo 'export PATH=$PATH:$ANDROID_HOME/platform-tools' >> ~/.bashrc
source ~/.bashrc
```

### **1.2: Install NDK and Build Tools**

```bash
# Accept licenses
sdkmanager --licenses

# Install NDK (version 25+)
sdkmanager "ndk;25.2.9519653"

# Install platform tools
sdkmanager "platform-tools"

# Install build tools
sdkmanager "build-tools;34.0.0"

# Verify installation
sdkmanager --list_installed
```

### **1.3: Set NDK Path**

```bash
echo 'export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653' >> ~/.bashrc
source ~/.bashrc

# Verify
echo $NDK_HOME
ls $NDK_HOME
```

---

## 🦀 **Step 2: Setup Rust for Android**

### **2.1: Add Android Targets**

```bash
# Add ARM64 target (primary for Pixel 8a)
rustup target add aarch64-linux-android

# Add other targets (optional, for testing)
rustup target add armv7-linux-androideabi   # 32-bit ARM
rustup target add x86_64-linux-android      # Emulator
rustup target add i686-linux-android        # 32-bit emulator
```

### **2.2: Configure Cargo for Cross-Compilation**

Create `~/.cargo/config.toml` (or update existing):

```toml
# Android ARM64 (Pixel 8a)
[target.aarch64-linux-android]
ar = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"

# Android ARM (32-bit, optional)
[target.armv7-linux-androideabi]
ar = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi33-clang"

# Android x86_64 (emulator)
[target.x86_64-linux-android]
ar = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/eastgate/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android33-clang"
```

**Note:** Adjust paths if your NDK version differs.

### **2.3: Install cargo-ndk**

```bash
# Install cargo-ndk for easier Android builds
cargo install cargo-ndk

# Verify
cargo ndk --version
```

---

## 📱 **Step 3: Setup Android Project Structure**

### **3.1: Current Structure**

The BearDog Android structure already exists:

```
beardog/
└── android/
    ├── app/
    │   └── src/
    │       └── main/
    │           └── AndroidManifest.xml
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### **3.2: Update Android Cargo.toml**

Edit `android/Cargo.toml`:

```toml
[package]
name = "beardog-android"
version = "3.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]
name = "beardog_android"

[dependencies]
beardog-tunnel = { path = "../crates/beardog-tunnel", features = ["strongbox_hardware"] }
beardog-types = { path = "../crates/beardog-types" }
beardog-errors = { path = "../crates/beardog-errors" }
jni = "0.21"
android_logger = "0.13"
log = "0.4"

[target.'cfg(target_os = "android")'.dependencies]
# Android-specific dependencies

[profile.release]
opt-level = "z"  # Optimize for size
lto = true
codegen-units = 1
strip = true
```

### **3.3: Create JNI Bindings**

Update `android/src/lib.rs`:

```rust
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[no_mangle]
pub extern "C" fn Java_com_beardog_BearDog_initialize(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info),
    );

    log::info!("BearDog Android library initialized");

    let output = env.new_string("BearDog initialized")
        .expect("Couldn't create java string!");
    
    output.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_beardog_BearDog_getEntropyFromStrongBox(
    env: JNIEnv,
    _class: JClass,
    size: i32,
) -> jstring {
    log::info!("Collecting {} bytes from StrongBox", size);
    
    // TODO: Implement real StrongBox integration
    let result = format!("Collected {} bytes (stub)", size);
    
    let output = env.new_string(result)
        .expect("Couldn't create java string!");
    
    output.into_raw()
}
```

---

## 🔨 **Step 4: Build for Android**

### **4.1: Build with cargo-ndk**

```bash
cd /home/eastgate/Development/ecoPrimals/beardog/android

# Build for ARM64 (Pixel 8a)
cargo ndk -t arm64-v8a build --release

# Build for all Android targets
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 build --release

# Output location:
# target/aarch64-linux-android/release/libbeardog_android.so
```

### **4.2: Manual Build (alternative)**

```bash
cd /home/eastgate/Development/ecoPrimals/beardog/android

# Build for ARM64
cargo build --target aarch64-linux-android --release

# Output:
# target/aarch64-linux-android/release/libbeardog_android.so
```

### **4.3: Verify Build**

```bash
# Check architecture
file target/aarch64-linux-android/release/libbeardog_android.so

# Should show:
# ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked
```

---

## 📲 **Step 5: Setup Pixel 8a Device**

### **5.1: Enable Developer Options**

On your Pixel 8a:
1. Settings → About phone
2. Tap "Build number" 7 times
3. Developer options enabled!

### **5.2: Enable USB Debugging**

1. Settings → System → Developer options
2. Enable "USB debugging"
3. Enable "USB debugging (Security settings)"

### **5.3: Connect Device**

```bash
# Connect Pixel 8a via USB

# Verify connection
adb devices

# Should show:
# List of devices attached
# <serial>    device

# Authorize on device if prompted
```

### **5.4: Verify StrongBox**

```bash
# Check StrongBox availability
adb shell getprop ro.hardware.keystore

# Should show: strongbox or trusty

# Check security level
adb shell cmd keystore2 list-security-levels

# Should include: STRONGBOX
```

---

## 🔐 **Step 6: StrongBox Integration**

### **6.1: Understanding StrongBox**

**StrongBox (Titan M2) on Pixel 8a:**
- Hardware-isolated security chip
- Tamper-resistant
- Key storage in secure element
- Hardware-backed attestation
- Android Keystore API

### **6.2: Required Permissions**

Add to `android/app/src/main/AndroidManifest.xml`:

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.beardog">

    <!-- StrongBox/Keystore permissions -->
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-feature 
        android:name="android.hardware.strongbox_keystore"
        android:required="false" />
    
    <application
        android:allowBackup="false"
        android:label="BearDog">
    </application>
</manifest>
```

### **6.3: Java/Kotlin Bridge (TODO)**

Create Java/Kotlin code to interface with Android Keystore:

```kotlin
// BearDog.kt
package com.beardog

import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import java.security.KeyStore
import javax.crypto.KeyGenerator

object BearDog {
    init {
        System.loadLibrary("beardog_android")
    }

    external fun initialize(): String
    external fun getEntropyFromStrongBox(size: Int): String

    fun generateKeyInStrongBox(): Boolean {
        val keyGenerator = KeyGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_AES,
            "AndroidKeyStore"
        )

        val spec = KeyGenParameterSpec.Builder(
            "beardog_key",
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setKeySize(256)
            .setIsStrongBoxBacked(true)  // Use StrongBox!
            .build()

        keyGenerator.init(spec)
        keyGenerator.generateKey()
        
        return true
    }
}
```

---

## 🧪 **Step 7: Testing on Device**

### **7.1: Create Test APK**

This requires Android Studio or Gradle setup (beyond current scope).

**Quick test with adb:**

```bash
# Push library to device
adb push target/aarch64-linux-android/release/libbeardog_android.so /data/local/tmp/

# Test loading
adb shell "ls -lh /data/local/tmp/libbeardog_android.so"
```

### **7.2: Run Instrumented Tests**

```bash
# Once APK is built:
adb install -r app/build/outputs/apk/debug/app-debug.apk

# Run tests
adb shell am instrument -w com.beardog.test/androidx.test.runner.AndroidJUnitRunner
```

---

## 📊 **Current Status**

### **What's Ready:**
- ✅ Android directory structure exists
- ✅ Basic Cargo.toml configuration
- ✅ lib.rs stub implementation

### **What's Needed (Phase 2):**
- ⏳ Install Android NDK
- ⏳ Configure cargo for Android
- ⏳ Update JNI bindings
- ⏳ Implement StrongBox integration
- ⏳ Create Kotlin/Java bridge
- ⏳ Build test APK
- ⏳ Test on real Pixel 8a

### **What's Blocked:**
- ❌ Real StrongBox integration (needs device)
- ❌ Hardware testing (needs device)
- ❌ APK creation (needs Android Studio/Gradle)

---

## 🎯 **Phase 2 Roadmap**

### **Week 1: Setup & Build**
1. Install Android NDK
2. Configure Rust toolchain
3. Update Android crate
4. Successfully build for ARM64
5. Test library loads on device

### **Week 2: StrongBox Integration**
1. Study Android Keystore API
2. Create Kotlin bridge code
3. Implement StrongBox provider
4. Test key generation
5. Test entropy collection

### **Week 3: Integration & Testing**
1. Build complete APK
2. Test on Pixel 8a
3. Validate StrongBox usage
4. Compare with SoloKeys
5. Document findings

---

## 🔗 **Resources**

### **Official Documentation:**
- [Android NDK](https://developer.android.com/ndk)
- [Android Keystore](https://developer.android.com/training/articles/keystore)
- [StrongBox](https://developer.android.com/training/articles/keystore#HardwareSecurityModule)
- [Rust on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)

### **Tools:**
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [jni-rs](https://github.com/jni-rs/jni-rs)
- [android_logger](https://docs.rs/android_logger/)

### **BearDog Docs:**
- `HARDWARE_SETUP.md` - SoloKeys setup (reference)
- `PHASE1_COMPLETE_SUCCESS.md` - What's working now
- `PRE_PRODUCTION_MVP_PLAN.md` - Full roadmap

---

## 🐛 **Common Issues**

### **NDK not found**
```bash
# Set NDK_HOME
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653

# Verify
echo $NDK_HOME
```

### **Linker errors**
```bash
# Check cargo config paths
cat ~/.cargo/config.toml

# Verify NDK version matches
ls $ANDROID_HOME/ndk/
```

### **ADB not found**
```bash
# Add platform-tools to PATH
export PATH=$PATH:$ANDROID_HOME/platform-tools

# Or install separately
sudo apt install adb
```

### **Device not authorized**
```bash
# Revoke and re-authorize
adb kill-server
adb start-server
adb devices

# Accept prompt on device
```

---

## ✅ **Verification Checklist**

Before starting Phase 2 implementation:

- [ ] Android NDK installed
- [ ] Rust Android targets added
- [ ] cargo-ndk installed
- [ ] Cargo config updated
- [ ] Pixel 8a connected via ADB
- [ ] Developer mode enabled
- [ ] USB debugging enabled
- [ ] StrongBox available (verified)
- [ ] Can build for aarch64-linux-android
- [ ] Library loads on device

---

## 🚀 **Quick Start Commands**

```bash
# Install everything
./scripts/setup-android.sh  # To be created

# Build for Android
cd android
cargo ndk -t arm64-v8a build --release

# Test on device
adb push target/aarch64-linux-android/release/libbeardog_android.so /data/local/tmp/
adb shell ls -lh /data/local/tmp/libbeardog_android.so
```

---

## 💡 **Next Steps**

When ready to begin Phase 2:

1. **Install NDK** (Step 1)
2. **Configure Rust** (Step 2)
3. **Test build** (Step 4)
4. **Connect device** (Step 5)
5. **Start integration** (Step 6)

See `PRE_PRODUCTION_MVP_PLAN.md` for complete Phase 2 details.

---

**Status:** Ready for Phase 2 implementation  
**Requirements:** Android NDK + Pixel 8a device  
**Timeline:** 2-3 weeks  
**Complexity:** Medium

---

*This guide will be updated as Phase 2 progresses.*

