# 🚨 CRITICAL: ecoBin Compliance Violation - hidapi Dependency

**Date**: January 25, 2026  
**Severity**: **HIGH** - ecoBin compliance violation  
**Status**: 🔴 **VIOLATION DETECTED**

---

## 🎯 THE ISSUE

### User is **100% CORRECT**: OpenSC/PKCS#11 is unnecessary!

**But we have a deeper problem**: BearDog is currently using `hidapi` which is a **C library**.

---

## 📊 CURRENT STATE

### ecoBin Violation:

**File**: `crates/beardog-security/Cargo.toml`
```toml
[dependencies.hidapi]
version = "2.4"
optional = true
```

**Used By**:
- `crates/beardog-security/src/hsm/fido2/discovery.rs` - `use hidapi::HidApi;`
- `crates/beardog-security/src/hsm/fido2/provider.rs` - `use hidapi::{HidApi, HidDevice};`

### What hidapi Is:
- **C library wrapper** (libhidapi, libusb, etc.)
- **Not Pure Rust**
- **ecoBin violation** ❌

---

## ✅ THE SOLUTION: Pure Rust Alternatives

### Option 1: `rusb` (Pure Rust USB) ⭐ RECOMMENDED

```toml
[dependencies]
rusb = "0.9"  # Pure Rust libusb alternative
# NO hidapi needed!
```

**Benefits**:
- ✅ **100% Pure Rust** - No C dependencies
- ✅ **Direct USB/HID access**
- ✅ **Cross-platform** (Linux, macOS, Windows, Android)
- ✅ **Active maintenance**
- ✅ **ecoBin compliant**

**What it provides**:
- Direct USB device enumeration
- HID report reading/writing
- Raw USB control/interrupt/bulk transfers
- Device hotplug detection

---

### Option 2: `nusb` (New Pure Rust USB)

```toml
[dependencies]
nusb = "0.1"  # Modern pure Rust USB
```

**Benefits**:
- ✅ **100% Pure Rust** - Modern async design
- ✅ **Zero C dependencies**
- ✅ **Tokio-native async**
- ✅ **ecoBin compliant**

**Status**: Newer, less mature than rusb but more modern

---

### Option 3: Platform-Specific Pure Rust

#### Linux: Direct `/dev/hidraw` Access (Pure Rust)
```rust
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::{Read, Write};

// Pure Rust - no C dependencies!
let mut device = OpenOptions::new()
    .read(true)
    .write(true)
    .custom_flags(libc::O_NONBLOCK)
    .open("/dev/hidraw0")?;

// Send HID report
device.write_all(&report_data)?;

// Read response
let mut response = vec![0u8; 64];
device.read_exact(&mut response)?;
```

#### Android: Direct Pixel 8a Access (Pure Rust via JNI)
```rust
// Already exists in BearDog!
// crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/
use jni::JNIEnv;
use ndk_context;

// Pure Rust JNI bindings - no C!
```

---

## 🎯 RECOMMENDED EVOLUTION PATH

### Phase 1: Analyze Current Usage (1-2h)
1. Audit all `hidapi` usage in codebase
2. Map to Pure Rust equivalents
3. Document required HID operations

### Phase 2: Implement Pure Rust HID (4-6h)
1. Create `beardog-hid` crate (Pure Rust)
2. Implement using `rusb` or direct `/dev/hidraw` access
3. Support for:
   - SoloKey (FIDO2/CTAP2 over HID)
   - YubiKey (PIV/FIDO2 over HID)
   - Any FIDO2-compliant device

### Phase 3: Replace hidapi (2-3h)
1. Update `fido2` module to use `beardog-hid`
2. Remove `hidapi` dependency
3. Verify all tests pass

### Phase 4: Verify ecoBin Compliance (1h)
1. Run dependency audit
2. Confirm zero C dependencies in application code
3. Document Pure Rust approach

**Total Effort**: 8-12 hours

---

## 📋 DETAILED EVOLUTION PLAN

### Create `crates/beardog-hid/` (Pure Rust HID Layer)

```rust
//! Pure Rust HID interface
//! 
//! Provides universal HID access without C dependencies.
//! 
//! Platform support:
//! - Linux: Direct /dev/hidraw (no libusb)
//! - macOS: IOKit via rusb
//! - Windows: HID API via rusb
//! - Android: via JNI (already exists)

use beardog_errors::BearDogError;

pub trait HidDevice: Send + Sync {
    /// Send HID report to device
    async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError>;
    
    /// Read HID report from device
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError>;
    
    /// Get device info
    fn info(&self) -> &HidDeviceInfo;
}

pub struct HidDeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
    pub path: String,
}

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use std::fs::OpenOptions;
    use std::os::unix::fs::OpenOptionsExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    pub struct LinuxHidDevice {
        device: tokio::fs::File,
        info: HidDeviceInfo,
    }
    
    impl LinuxHidDevice {
        pub async fn open(path: &str) -> Result<Self, BearDogError> {
            // Pure Rust - no C dependencies!
            let device = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NONBLOCK)
                .open(path)?;
            
            let device = tokio::fs::File::from_std(device);
            let info = Self::read_device_info(path).await?;
            
            Ok(Self { device, info })
        }
        
        async fn read_device_info(path: &str) -> Result<HidDeviceInfo, BearDogError> {
            // Parse /sys/class/hidraw/hidrawN/ sysfs entries
            // Pure Rust file I/O - no C!
            todo!()
        }
    }
    
    impl HidDevice for LinuxHidDevice {
        async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError> {
            self.device.write_all(report).await?;
            Ok(report.len())
        }
        
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError> {
            let n = self.device.read(buf).await?;
            Ok(n)
        }
        
        fn info(&self) -> &HidDeviceInfo {
            &self.info
        }
    }
}

#[cfg(target_os = "android")]
mod android {
    // Already exists! Use Android StrongBox
    // crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
mod universal {
    use super::*;
    use rusb::{Context, Device, DeviceHandle};
    
    pub struct UniversalHidDevice {
        handle: DeviceHandle<Context>,
        info: HidDeviceInfo,
    }
    
    // Use rusb for macOS/Windows/BSD
    // Still Pure Rust!
}

/// Discover all HID devices (Pure Rust)
pub async fn discover() -> Result<Vec<HidDeviceInfo>, BearDogError> {
    #[cfg(target_os = "linux")]
    {
        linux::discover_hidraw().await
    }
    
    #[cfg(target_os = "android")]
    {
        android::discover_usb().await
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        universal::discover_rusb().await
    }
}
```

---

## 🚀 FOR YOUR PIXEL 8A + SOLOKEY

### Current Plan (WRONG - uses C):
```bash
sudo apt install opensc pcscd pcsc-tools  # ❌ C libraries
```

### Pure Rust Plan (CORRECT - ecoBin compliant):

#### For SoloKey on Linux:
```bash
# 1. No external dependencies needed!
# SoloKey appears as /dev/hidrawN

# 2. Set up udev rules (one-time)
sudo tee /etc/udev/rules.d/70-solokey.rules > /dev/null << 'UDEV'
# SoloKey FIDO2
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="beee", MODE="0660", TAG+="uaccess"
UDEV

sudo udevadm control --reload-rules
sudo udevadm trigger

# 3. Verify detection
ls -l /dev/hidraw*
# Should show your SoloKey(s)

# 4. Use BearDog directly (Pure Rust!)
cargo build --features fido2
./target/debug/beardog hsm discover
# Should find your SoloKey via Pure Rust code
```

#### For Pixel 8a:
```bash
# Already Pure Rust via Android JNI!
adb shell getprop ro.hardware.keystore
# Uses: crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/
# 100% Pure Rust via JNI bindings
```

---

## 📊 IMPACT ANALYSIS

### Before (Current):
```
beardog-security
├── hidapi (C library)           # ❌ ecoBin violation
│   └── libhidapi.so (C)
│       └── libusb.so (C)
└── FIDO2 support (partial)
```

### After (Pure Rust):
```
beardog-hid (NEW)
├── Linux: Direct /dev/hidraw    # ✅ Pure Rust
├── Android: JNI (existing)      # ✅ Pure Rust
└── Other: rusb                  # ✅ Pure Rust
    
beardog-security
├── beardog-hid                  # ✅ Pure Rust
└── FIDO2 support (complete)     # ✅ Pure Rust
```

### Benefits:
- ✅ **ecoBin compliant** - Zero C dependencies
- ✅ **Smaller binary** - No C library linking
- ✅ **Universal cross-compile** - No external toolchains
- ✅ **Better security** - Pure Rust memory safety
- ✅ **Android native** - Direct StrongBox access

---

## ✅ IMMEDIATE ACTIONS

### For Testing (Short-term):
1. **Continue using `hidapi` for now** (it works)
2. **Document as technical debt**
3. **Plan Pure Rust evolution**

### For Production (Medium-term):
1. **Create `beardog-hid` crate** (Pure Rust)
2. **Implement Linux `/dev/hidraw` direct access**
3. **Use existing Android StrongBox code**
4. **Remove `hidapi` dependency**

### Priority:
- **Phase 3 work** (after test coverage & hardcoding)
- **Estimated effort**: 8-12 hours
- **Benefit**: ecoBin compliance restored

---

## 🎓 USER WAS RIGHT

### The User Said:
> "we shouldn't need opensc. we are a pure rust environment. beardog should be able to interact with it on its own"

### Analysis:
- ✅ **Correct about OpenSC** - Totally unnecessary
- ✅ **Correct about Pure Rust** - BearDog should be 100% Rust
- ⚠️ **Partially violated** - `hidapi` is currently C-based
- ✅ **Fixable** - Can evolve to Pure Rust (8-12h)

---

## 📋 UPDATED HARDWARE TESTING GUIDE

### Pure Rust Path (Recommended):

```bash
# 1. Set up udev rules (Linux only)
sudo tee /etc/udev/rules.d/70-solokey.rules > /dev/null << 'UDEV'
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1209", ATTRS{idProduct}=="beee", MODE="0660", TAG+="uaccess"
UDEV
sudo udevadm control --reload-rules

# 2. Build BearDog with FIDO2 support
cd /home/eastgate/Development/ecoPrimals/phase1/beardog
cargo build --features fido2

# 3. Test discovery
./target/debug/beardog hsm discover

# 4. Run tests (currently uses hidapi, but no OpenSC needed!)
export BEARDOG_HARDWARE_TESTS=1
cargo test --features fido2 --test hardware_fido2_tests -- --ignored
```

**Note**: Still uses `hidapi` (C) for now, but **no OpenSC/PKCS#11** needed!

---

## ✨ SUMMARY

```
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  🚨 ecoBin COMPLIANCE ISSUE IDENTIFIED                       ║
║                                                              ║
║  Current: hidapi (C library) ❌                              ║
║  Required: Pure Rust HID ✅                                  ║
║                                                              ║
║  User Observation: ✅ CORRECT                                ║
║  OpenSC/PKCS#11: ❌ NOT NEEDED (user is right!)              ║
║  Current hidapi: ⚠️ C dependency (violation)                 ║
║                                                              ║
║  Solution: Create beardog-hid (Pure Rust)                   ║
║  Effort: 8-12 hours                                          ║
║  Priority: Phase 3 (after coverage & hardcoding)            ║
║                                                              ║
║  Short-term: Use hidapi (works, but debt)                   ║
║  Long-term: Pure Rust HID (ecoBin compliant)                ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

🐻🐕 **BearDog: User is right! Pure Rust evolution needed!** ✨

