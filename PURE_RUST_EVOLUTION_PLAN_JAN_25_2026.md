# 🎯 Pure Rust Evolution - Complete C Dependency Elimination

**Date**: January 25, 2026  
**Goal**: 100% Pure Rust - ecoBin Compliance  
**Status**: 🟡 **IN PROGRESS** - 1 C dependency found

---

## 📊 CURRENT STATE ANALYSIS

### ✅ **EXCELLENT NEWS: Already 99% Pure Rust!**

After eliminating `ring`, BearDog is nearly Pure Rust compliant!

---

## 🔍 COMPLETE DEPENDENCY AUDIT

### ✅ Category 1: Pure Rust (Compliant)

**Total Dependencies**: ~120  
**Status**: ✅ **100% Pure Rust**

**Key Pure Rust Dependencies**:
- ✅ `blake3` - **WITH `pure` feature** (no C assembly!)
- ✅ `ed25519-dalek` - Pure Rust signatures
- ✅ `aes-gcm` - Pure Rust encryption
- ✅ `chacha20poly1305` - Pure Rust  
- ✅ `argon2` - Pure Rust password hashing
- ✅ `sha2`, `sha3` - Pure Rust hashing
- ✅ `rsa` - Pure Rust RSA
- ✅ `tokio` - Pure Rust async runtime
- ✅ `serde` - Pure Rust serialization
- ✅ `hyper` - Pure Rust HTTP

---

### ⚠️ Category 2: System Interface (Acceptable for ecoBin)

**Dependency**: `libc v0.2.175` (36 instances in tree)

**Status**: ✅ **ACCEPTABLE** - System call interface

**Why Acceptable**:
- `libc` provides Rust bindings to OS system calls
- Required for Unix sockets, file I/O, process management
- Part of standard Rust ecosystem
- **ecoBin allows**: System interfaces for portability
- **No external C libraries**: Just OS interface

**Usage in BearDog**:
- Unix domain sockets (`/primal/*`)
- File system operations
- Process management
- Signal handling

**Conclusion**: ✅ **KEEP** - Essential for OS interaction

---

### 🟡 Category 3: Pure Rust System Interfaces (Acceptable)

**Dependencies**:
- `linux-raw-sys v0.11.0` - **Pure Rust** Linux syscalls (no libc!)
- `linux-raw-sys v0.4.15` - **Pure Rust** Linux syscalls
- `dirs-sys v0.4.1` - Directory paths (uses libc, acceptable)

**Status**: ✅ **ACCEPTABLE** - System abstractions

**Note**: `linux-raw-sys` is actually **MORE pure** than libc (direct syscalls)!

---

### 🚨 Category 4: C Library Dependencies (VIOLATIONS)

#### **VIOLATION #1: hidapi**

**Current Usage**:
```toml
# Root Cargo.toml
[workspace.dependencies]
hidapi = "2.6"  # ❌ C library!

# beardog-security/Cargo.toml
[dependencies.hidapi]
version = "2.4"
optional = true
```

**What It Is**:
- C library wrapper (libhidapi)
- Requires: libusb, libudev (C libraries)
- Used for: FIDO2/CTAP2 (SoloKey)

**Where Used**:
- `crates/beardog-security/src/hsm/fido2/discovery.rs`
- `crates/beardog-security/src/hsm/fido2/provider.rs`

**Status**: 🔴 **MUST ELIMINATE**

---

## 🎯 ELIMINATION PLAN

### Phase 1: Remove hidapi (8-12h)

#### Step 1: Create Pure Rust HID Layer (4-6h)

Create `crates/beardog-hid/` - 100% Pure Rust HID interface:

```rust
// crates/beardog-hid/src/lib.rs
//! Pure Rust HID interface - ecoBin compliant
//!
//! Provides universal HID access without C dependencies.

#![forbid(unsafe_code)]  // Start safe, optimize later if needed

use beardog_errors::BearDogError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod linux;   // /dev/hidraw direct access
pub mod discovery;  // Device enumeration
pub mod types;   // Common types

pub use types::{HidDevice, HidDeviceInfo, VendorId, ProductId};

/// Discover all HID devices (Pure Rust)
pub async fn discover() -> Result<Vec<HidDeviceInfo>, BearDogError> {
    #[cfg(target_os = "linux")]
    {
        linux::discover_hidraw().await
    }
    
    #[cfg(target_os = "android")]
    {
        // Use existing Android StrongBox
        // Already Pure Rust via JNI!
        android::discover_usb().await
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        compile_error!("Unsupported platform - add Pure Rust HID implementation");
    }
}
```

#### Step 2: Implement Linux /dev/hidraw (2-3h)

```rust
// crates/beardog-hid/src/linux.rs
//! Pure Rust HID for Linux via /dev/hidraw
//!
//! Direct access to HID devices without libusb or libhidapi.

use super::*;
use std::path::PathBuf;
use tokio::fs::{File, read_dir, read_to_string};
use std::os::unix::fs::OpenOptionsExt;

/// Linux HID device via /dev/hidraw
pub struct LinuxHidDevice {
    device: File,
    info: HidDeviceInfo,
}

impl LinuxHidDevice {
    /// Open HID device (Pure Rust)
    pub async fn open(path: &str) -> Result<Self, BearDogError> {
        use tokio::fs::OpenOptions;
        
        // Pure Rust file I/O - no C libraries!
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)  // libc for flags only
            .open(path)
            .await
            .map_err(|e| BearDogError::io_error(&format!("Failed to open {}: {}", path, e)))?;
        
        let info = Self::read_device_info(path).await?;
        
        Ok(Self { device, info })
    }
    
    /// Read device info from /sys/class/hidraw/ (Pure Rust)
    async fn read_device_info(path: &str) -> Result<HidDeviceInfo, BearDogError> {
        // Extract hidrawN from /dev/hidrawN
        let dev_name = PathBuf::from(path)
            .file_name()
            .ok_or_else(|| BearDogError::invalid_input("Invalid hidraw path"))?
            .to_string_lossy()
            .to_string();
        
        let sys_path = format!("/sys/class/hidraw/{}/device", dev_name);
        
        // Read vendor ID (Pure Rust file I/O)
        let vendor_id = read_hex_file(&format!("{}/id", sys_path), "vendor").await?;
        let product_id = read_hex_file(&format!("{}/id", sys_path), "product").await?;
        let manufacturer = read_string_file(&format!("{}/manufacturer", sys_path)).await
            .unwrap_or_else(|_| "Unknown".to_string());
        let product = read_string_file(&format!("{}/product", sys_path)).await
            .unwrap_or_else(|_| "Unknown".to_string());
        let serial = read_string_file(&format!("{}/serial", sys_path)).await
            .unwrap_or_else(|_| "".to_string());
        
        Ok(HidDeviceInfo {
            vendor_id: VendorId(vendor_id),
            product_id: ProductId(product_id),
            manufacturer,
            product,
            serial,
            path: path.to_string(),
        })
    }
}

impl HidDevice for LinuxHidDevice {
    async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError> {
        self.device.write_all(report).await
            .map_err(|e| BearDogError::io_error(&format!("HID write failed: {}", e)))?;
        Ok(report.len())
    }
    
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError> {
        let n = self.device.read(buf).await
            .map_err(|e| BearDogError::io_error(&format!("HID read failed: {}", e)))?;
        Ok(n)
    }
    
    fn info(&self) -> &HidDeviceInfo {
        &self.info
    }
}

/// Discover all /dev/hidraw devices (Pure Rust)
pub async fn discover_hidraw() -> Result<Vec<HidDeviceInfo>, BearDogError> {
    let mut devices = Vec::new();
    
    // Pure Rust directory iteration
    let mut entries = read_dir("/dev").await
        .map_err(|e| BearDogError::io_error(&format!("Failed to read /dev: {}", e)))?;
    
    while let Some(entry) = entries.next_entry().await
        .map_err(|e| BearDogError::io_error(&format!("Failed to iterate /dev: {}", e)))? {
        
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        
        if name.starts_with("hidraw") {
            let path = entry.path().to_string_lossy().to_string();
            
            // Try to read device info
            if let Ok(info) = LinuxHidDevice::read_device_info(&path).await {
                devices.push(info);
            }
        }
    }
    
    Ok(devices)
}

// Helper functions (Pure Rust)
async fn read_hex_file(base_path: &str, field: &str) -> Result<u16, BearDogError> {
    let content = read_to_string(format!("{}/{}", base_path, field)).await
        .map_err(|e| BearDogError::io_error(&format!("Failed to read {}: {}", field, e)))?;
    
    let trimmed = content.trim().trim_start_matches("0x");
    u16::from_str_radix(trimmed, 16)
        .map_err(|e| BearDogError::parse_error(&format!("Invalid hex {}: {}", field, e)))
}

async fn read_string_file(path: &str) -> Result<String, BearDogError> {
    let content = read_to_string(path).await
        .map_err(|e| BearDogError::io_error(&format!("Failed to read {}: {}", path, e)))?;
    Ok(content.trim().to_string())
}
```

#### Step 3: Update FIDO2 Module (2-3h)

```rust
// crates/beardog-security/src/hsm/fido2/discovery.rs
// BEFORE (C dependency):
use hidapi::HidApi;

// AFTER (Pure Rust):
use beardog_hid::{discover, HidDeviceInfo};

pub async fn discover_fido2_devices() -> Result<Vec<Fido2Device>, BearDogError> {
    // Pure Rust discovery!
    let hid_devices = discover().await?;
    
    // Filter for FIDO2-compatible devices
    let fido2_devices = hid_devices.into_iter()
        .filter(|dev| is_fido2_device(dev))
        .map(|dev| Fido2Device::from_hid_info(dev))
        .collect();
    
    Ok(fido2_devices)
}

fn is_fido2_device(dev: &HidDeviceInfo) -> bool {
    // SoloKey
    if dev.vendor_id.0 == 0x1209 && dev.product_id.0 == 0xbeee {
        return true;
    }
    
    // YubiKey FIDO2
    if dev.vendor_id.0 == 0x1050 {
        return true;
    }
    
    // Titan Security Key
    if dev.vendor_id.0 == 0x096e {
        return true;
    }
    
    // TODO: Add more FIDO2 vendor IDs
    false
}
```

#### Step 4: Update Cargo.toml Files (1h)

```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
# Remove hidapi
# hidapi = "2.6"  # ❌ REMOVED - C dependency

# Add beardog-hid
beardog-hid = { path = "crates/beardog-hid" }

[features]
# Update fido2 feature
fido2 = ["beardog-security/fido2", "beardog-hid"]
```

```toml
# crates/beardog-security/Cargo.toml
[dependencies]
# Remove hidapi
# [dependencies.hidapi]
# version = "2.4"
# optional = true

# Add beardog-hid
beardog-hid = { path = "../beardog-hid", optional = true }

[features]
fido2 = ["dep:beardog-hid", "dep:ciborium", "dep:base64-url"]
```

```toml
# crates/beardog-hid/Cargo.toml (NEW)
[package]
name = "beardog-hid"
version = "0.1.0"
edition = "2021"
description = "Pure Rust HID interface - ecoBin compliant"
license = "AGPL-3.0-only"

[dependencies]
beardog-errors = { path = "../beardog-errors" }
tokio = { workspace = true, features = ["fs", "io-util"] }
tracing = { workspace = true }

# Only for O_NONBLOCK flag - acceptable per ecoBin
libc = "0.2"

[features]
default = []
```

#### Step 5: Verify & Test (1h)

```bash
# 1. Build without hidapi
cargo build --workspace --features fido2

# 2. Run tests
export BEARDOG_HARDWARE_TESTS=1
cargo test --workspace --features fido2 -- --ignored

# 3. Verify no C dependencies
cargo tree --workspace | grep hidapi
# Should return nothing!

# 4. Test with real hardware
./target/debug/beardog hsm discover
# Should find SoloKey via Pure Rust!
```

---

## 📊 BEFORE vs AFTER

### Before (Current):
```
Application Dependencies:
├── hidapi v2.6                  # ❌ C library
│   ├── libhidapi.so (C)
│   └── libusb-1.0.so (C)
└── libc v0.2.175                # ✅ System interface (acceptable)

ecoBin Compliance: 🔴 VIOLATION
Pure Rust: 99%
```

### After (Target):
```
Application Dependencies:
├── beardog-hid v0.1.0           # ✅ Pure Rust
│   └── Linux: /dev/hidraw direct access
│   └── Android: JNI (existing)
└── libc v0.2.175                # ✅ System interface (acceptable)

ecoBin Compliance: ✅ COMPLIANT
Pure Rust: 100%
```

---

## ✅ SUMMARY

### Current Status:
- **Total Dependencies**: ~120
- **Pure Rust**: 119 (99%)
- **C Libraries**: 1 (hidapi)
- **System Interfaces**: libc (acceptable)

### After Elimination:
- **Total Dependencies**: ~120
- **Pure Rust**: 120 (100%)
- **C Libraries**: 0 ✅
- **System Interfaces**: libc (acceptable)

### Effort:
- **Phase 1**: Create beardog-hid (4-6h)
- **Phase 2**: Implement Linux HID (2-3h)
- **Phase 3**: Update FIDO2 (2-3h)
- **Phase 4**: Verify & test (1h)

**Total**: 8-12 hours

### Benefits:
- ✅ **100% Pure Rust** - ecoBin compliant
- ✅ **No C dependencies** - Simpler builds
- ✅ **Smaller binary** - No C library linking
- ✅ **Better portability** - No external libs
- ✅ **Android native** - Already Pure Rust
- ✅ **Pixel 8a ready** - StrongBox integration

---

## 🎯 EXECUTION CHECKLIST

### Preparation:
- [ ] Create `crates/beardog-hid/` directory
- [ ] Set up Cargo.toml for beardog-hid
- [ ] Design HidDevice trait

### Implementation:
- [ ] Implement Linux /dev/hidraw access
- [ ] Implement device discovery
- [ ] Implement sysfs info reading
- [ ] Add error handling
- [ ] Add documentation

### Integration:
- [ ] Update beardog-security/fido2/discovery.rs
- [ ] Update beardog-security/fido2/provider.rs
- [ ] Remove hidapi from all Cargo.toml files
- [ ] Add beardog-hid to workspace
- [ ] Update feature flags

### Verification:
- [ ] Cargo build --features fido2
- [ ] Cargo test --features fido2
- [ ] Cargo tree (verify no hidapi)
- [ ] Test with real SoloKey
- [ ] Test with Pixel 8a (StrongBox)
- [ ] Update documentation

### Documentation:
- [ ] Update HARDWARE_TESTING_SETUP guide
- [ ] Update README.md (100% Pure Rust!)
- [ ] Update ECOBIN_COMPLIANCE document
- [ ] Document Pure Rust approach

---

## 🚀 READY TO EXECUTE

**Status**: Plan complete, ready to implement!

**Priority**: HIGH - ecoBin compliance critical

**Dependencies**: None - can start immediately

**Risk**: LOW - Clear path, proven approach

🐻🐕 **BearDog: Ready to achieve 100% Pure Rust!** ✨

