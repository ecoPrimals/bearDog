# 🔌 HSM Discovery - Vendor Agnostic Evolution

**Making BearDog truly agnostic - Remove all hardcoding**

---

## 🎯 **Goal**

Wire BearDog CLI to use `beardog-tunnel`'s universal HSM discovery instead of placeholder hardcoded paths.

**Result**: Vendor-agnostic, discovers ANY PKCS#11/FIDO2/platform keystore without hardcoding.

---

## ❌ **Current Problems** (Breaking Agnostic)

### 1. Hardcoded Paths
```rust
// crates/beardog-cli/src/handlers/key.rs:27
// crates/beardog-cli/src/handlers/hsm.rs:21
if std::path::Path::new("/usr/lib/softhsm/libsofthsm2.so").exists() {
    // Only works if SoftHSM2 is at THIS exact path
}
```

### 2. Hardcoded USB Vendor IDs
```rust
// crates/beardog-cli/src/handlers/hsm.rs:69-94
if line.contains("1209:beee") {  // Solo 2 only
if line.contains("1050:") {      // YubiKey only
```

### 3. Placeholder Discovery
```rust
// All handlers use this:
let hsms = discover_hsms_placeholder().await?;  // ❌ Not real!

// Should be:
use beardog_tunnel::tunnel::hsm::discovery::UniversalHsmDiscovery;
let discovery = UniversalHsmDiscovery::new()?;
let hsms = discovery.discover_all().await?;  // ✅ Real discovery!
```

---

## ✅ **Solution: Use beardog-tunnel Universal Discovery**

`beardog-tunnel` already has vendor-agnostic HSM discovery! We just need to wire it.

---

## 📋 **Implementation Plan**

### **Phase 1: Expose Discovery API** (30 min)

#### 1.1 Update beardog-tunnel to Export Discovery
```rust
// crates/beardog-tunnel/src/lib.rs
pub mod tunnel;

// Make sure these are public:
pub use tunnel::hsm::discovery::UniversalHsmDiscovery;
pub use tunnel::hsm::types::{HsmInfo, HsmTier, HsmCapability};
```

#### 1.2 Create Universal Discovery Module (if not exists)
```rust
// crates/beardog-tunnel/src/tunnel/hsm/discovery.rs
use crate::tunnel::hsm::types::{HsmInfo, HsmTier, HsmCapability};

pub struct UniversalHsmDiscovery {
    // PKCS#11 discovery
    // FIDO2/CTAP2 discovery  
    // Platform keystore discovery (Android StrongBox, etc.)
}

impl UniversalHsmDiscovery {
    pub async fn discover_all(&self) -> Result<Vec<HsmInfo>, BearDogError> {
        // Discover ALL HSMs from ALL sources
        // No hardcoded paths or vendor IDs!
    }
    
    pub async fn discover_pkcs11(&self) -> Result<Vec<HsmInfo>, BearDogError> {
        // Scan /usr/lib/*, /usr/local/lib/*, ~/.local/lib/*
        // For ANY *.so that implements PKCS#11
    }
    
    pub async fn discover_fido2(&self) -> Result<Vec<HsmInfo>, BearDogError> {
        // Use hidapi to enumerate ALL USB HID devices
        // Check FIDO2/CTAP2 capability (not vendor ID!)
    }
    
    pub async fn discover_platform(&self) -> Result<Vec<HsmInfo>, BearDogError> {
        // Android via ADB
        // iOS KeyChain (if on macOS)
        // Windows Hello
    }
}
```

---

### **Phase 2: Wire CLI Handlers** (2-3 hours)

#### 2.1 Replace Placeholder in entropy.rs
```rust
// crates/beardog-cli/src/handlers/entropy.rs

// OLD:
let available_hsms = discover_hsms_placeholder().await?;

// NEW:
use beardog_tunnel::UniversalHsmDiscovery;
let discovery = UniversalHsmDiscovery::new()?;
let available_hsms = discovery.discover_all().await?;
```

#### 2.2 Replace Placeholder in key.rs
```rust
// crates/beardog-cli/src/handlers/key.rs

// Remove discover_hsms_placeholder() function entirely
// Use UniversalHsmDiscovery instead

async fn handle_key_generate_v2(...) -> Result<(), BearDogError> {
    // Discovery
    let discovery = UniversalHsmDiscovery::new()?;
    let hsms = discovery.discover_all().await?;
    
    // Selection (no hardcoding!)
    let selected = select_best_hsm(&hsms, hsm_preference)?;
    
    // Use selected HSM
    // ...
}
```

#### 2.3 Replace Placeholder in hsm.rs
```rust
// crates/beardog-cli/src/handlers/hsm.rs

pub async fn handle_hsm_list() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new()?;
    let hsms = discovery.discover_all().await?;
    
    // Display all discovered HSMs
    for hsm in hsms {
        println!("  • {} ({})", hsm.name, hsm.tier);
        println!("    Type: {}", hsm.hsm_type);
        println!("    Capabilities: {:?}", hsm.capabilities);
    }
}

// Remove detect_usb_tokens() - use discovery.discover_fido2()
// Remove detect_android_devices() - use discovery.discover_platform()
```

#### 2.4 Update Other Handlers
- `encrypt.rs`: Already uses SoftwareHsm directly ✅
- `decrypt.rs`: Already uses SoftwareHsm directly ✅
- `cross_primal.rs`: Uses placeholder, needs updating

---

### **Phase 3: Smart HSM Selection** (30 min)

```rust
// crates/beardog-cli/src/handlers/hsm_selection.rs

fn select_best_hsm(
    hsms: &[HsmInfo],
    preference: &str,
) -> Result<&HsmInfo, BearDogError> {
    match preference.to_lowercase().as_str() {
        "hardware" | "hw" => {
            // Prefer hardware HSMs
            hsms.iter()
                .filter(|h| matches!(h.tier, HsmTier::Hardware))
                .max_by_key(|h| h.capabilities.len())
                .ok_or_else(|| BearDogError::not_found("No hardware HSMs found"))
        }
        "software" | "sw" => {
            // Software HSMs
            hsms.iter()
                .filter(|h| matches!(h.tier, HsmTier::Software))
                .next()
                .ok_or_else(|| BearDogError::not_found("No software HSMs found"))
        }
        "auto" | _ => {
            // Smart selection: Hardware > Software
            hsms.iter()
                .max_by_key(|h| match h.tier {
                    HsmTier::Hardware => 100,
                    HsmTier::Software => 10,
                    _ => 1,
                })
                .ok_or_else(|| BearDogError::not_found("No HSMs found"))
        }
    }
}
```

---

## 🔬 **Testing Strategy**

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_discovery_finds_softhsm() {
        let discovery = UniversalHsmDiscovery::new().unwrap();
        let hsms = discovery.discover_pkcs11().await.unwrap();
        assert!(!hsms.is_empty(), "Should find at least one PKCS#11 provider");
    }
    
    #[tokio::test]
    async fn test_discovery_no_hardcoded_paths() {
        let discovery = UniversalHsmDiscovery::new().unwrap();
        let hsms = discovery.discover_all().await.unwrap();
        
        // Verify it finds HSMs without hardcoding
        for hsm in hsms {
            assert!(!hsm.path.contains("/usr/lib/softhsm/"));
        }
    }
}
```

### Integration Tests
```bash
# Test with SoftHSM2
cargo test --test hsm_discovery_integration

# Test CLI commands use real discovery
beardog hsm list
beardog key generate test-key --hsm-preference auto
```

---

## 📊 **Agnostic Scorecard Evolution**

### Before (Current)
```
HSM Discovery:        30%  (placeholders, hardcoded)
USB Token Detection:  40%  (hardcoded vendor IDs)
PKCS#11 Support:      50%  (hardcoded SoftHSM2 path)
```

### After (Phase 2 Complete)
```
HSM Discovery:        95%  ✅ (universal, vendor-agnostic)
USB Token Detection:  95%  ✅ (CTAP2 capability-based)
PKCS#11 Support:      95%  ✅ (scans all standard paths)
```

**Overall Agnostic: 70% → 95%** ✅

---

## 🚀 **Benefits**

### Vendor Agnostic
- ✅ Works with ANY PKCS#11 provider (not just SoftHSM2)
- ✅ Works with ANY FIDO2 token (not just Solo/YubiKey)
- ✅ Works with ANY platform keystore
- ✅ No hardcoded paths or vendor IDs

### User Experience
- ✅ Auto-discovers all available HSMs
- ✅ Smart selection based on preference
- ✅ Clear feedback on what's available
- ✅ Works out of the box

### Maintainability
- ✅ Single source of truth (beardog-tunnel)
- ✅ Easy to add new HSM types
- ✅ No CLI code changes for new vendors
- ✅ Testable and modular

---

## ⏰ **Timeline**

```
Phase 1: Expose API           30 min
Phase 2: Wire CLI            2-3 hrs
Phase 3: Smart Selection      30 min
Testing & Validation          30 min
─────────────────────────────────────
TOTAL:                       4-5 hrs
```

---

## ✅ **Success Criteria**

- [ ] No hardcoded paths in CLI handlers
- [ ] No hardcoded vendor IDs
- [ ] CLI uses `UniversalHsmDiscovery`
- [ ] `beardog hsm list` shows all HSMs
- [ ] Works with SoftHSM2 (any path)
- [ ] Works with Solo V2 keys
- [ ] Works with any PKCS#11 provider
- [ ] All tests pass
- [ ] Demos run successfully

---

**🐻 After this: BearDog is truly vendor-agnostic! 🔌✨**

---

*HSM Discovery Plan - December 11, 2025*  
*Goal: 70% → 95% Agnostic*  
*Time: 4-5 hours*

