# 🚀 MVP Phase 1 Quick Start Guide
**Created:** October 29, 2025  
**Goal:** Get BearDog working on Eastgate with real SoloKey detection

---

## ✅ **CURRENT STATUS**

### **What We Have:**
- ✅ `cryptoki = "0.6"` dependency already in Cargo.toml (Rust PKCS#11 library)
- ✅ PKCS#11 libraries found on Eastgate:
  - `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so` (PRIMARY - use this)
  - `/usr/lib/x86_64-linux-gnu/pkcs11/opensc-pkcs11.so`
- ✅ Basic structure in place:
  - `crates/beardog-tunnel/src/universal_hsm_discovery/capability_detection/pkcs11_prober.rs`
  - `crates/beardog-tunnel/src/universal_hsm/providers/pkcs11.rs`
  - `crates/beardog-cli/src/main.rs`

### **What Needs To Be Done:**
- ❌ Remove TODO/stub implementations
- ❌ Implement real PKCS#11 integration using `cryptoki` crate
- ❌ Build working CLI commands
- ❌ Test with actual SoloKey devices
- ❌ Create configuration system

---

## 🔧 **PREREQUISITES**

### **1. Install OpenSC tools** (for testing):
```bash
sudo apt install opensc
```

### **2. Verify PKCS#11 library:**
```bash
ls -la /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
# Should show the library file
```

### **3. Test SoloKey detection** (once OpenSC installed):
```bash
pkcs11-tool --module /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so --list-slots
# Should show your 4x SoloKey V2 devices
```

### **4. Check Rust toolchain:**
```bash
rustc --version  # Should be 1.70+
cargo --version
```

---

## 📝 **IMPLEMENTATION CHECKLIST**

### **Step 1: Real PKCS#11 Provider** (2-3 hours)

**File:** `crates/beardog-tunnel/src/universal_hsm/providers/pkcs11.rs`

**Current (stub):**
```rust
pub async fn initialize(&self) -> Result<(), BearDogError> {
    // TODO: Implement actual PKCS#11 initialization
    Ok(())
}
```

**Need to implement:**
```rust
use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::session::{Session, UserType};
use cryptoki::slot::Slot;

pub struct Pkcs11HsmProvider {
    library_path: String,
    context: Option<Pkcs11>,
}

impl Pkcs11HsmProvider {
    pub fn new(library_path: String) -> Self {
        Self {
            library_path,
            context: None,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), BearDogError> {
        let pkcs11 = Pkcs11::new(&self.library_path)
            .map_err(|e| BearDogError::hsm(format!("Failed to load PKCS#11 library: {}", e)))?;
        
        pkcs11.initialize(CInitializeArgs::OsThreads)
            .map_err(|e| BearDogError::hsm(format!("Failed to initialize PKCS#11: {}", e)))?;
        
        self.context = Some(pkcs11);
        Ok(())
    }

    pub async fn get_slot_list(&self) -> Result<Vec<Slot>, BearDogError> {
        let pkcs11 = self.context.as_ref()
            .ok_or_else(|| BearDogError::hsm("PKCS#11 not initialized"))?;
        
        let slots = pkcs11.get_slots_with_token()
            .map_err(|e| BearDogError::hsm(format!("Failed to get slots: {}", e)))?;
        
        Ok(slots)
    }

    pub async fn get_random_bytes(&self, slot: Slot, count: usize) -> Result<Vec<u8>, BearDogError> {
        let pkcs11 = self.context.as_ref()
            .ok_or_else(|| BearDogError::hsm("PKCS#11 not initialized"))?;
        
        let session = pkcs11.open_ro_session(slot)
            .map_err(|e| BearDogError::hsm(format!("Failed to open session: {}", e)))?;
        
        let mut random_data = vec![0u8; count];
        session.generate_random(&mut random_data)
            .map_err(|e| BearDogError::hsm(format!("Failed to generate random: {}", e)))?;
        
        Ok(random_data)
    }
}
```

### **Step 2: Real PKCS#11 Prober** (1-2 hours)

**File:** `crates/beardog-tunnel/src/universal_hsm_discovery/capability_detection/pkcs11_prober.rs`

**Current (stub):**
```rust
pub async fn probe_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
    // TODO: Implement actual PKCS#11 capability detection
    Ok(HsmCapabilities::default())
}
```

**Need to implement:**
```rust
use crate::universal_hsm::providers::pkcs11::Pkcs11HsmProvider;

impl Pkcs11CapabilityProber {
    pub async fn probe_capabilities(&self, library_path: &str) -> Result<Vec<HsmCapabilities>, BearDogError> {
        let mut provider = Pkcs11HsmProvider::new(library_path.to_string());
        provider.initialize().await?;
        
        let slots = provider.get_slot_list().await?;
        let mut capabilities = Vec::new();
        
        for slot in slots {
            // Get token info for each slot
            let cap = HsmCapabilities {
                provider_type: "PKCS11".to_string(),
                slot_id: Some(slot.id() as u32),
                supports_random: true,
                supports_encryption: true,
                supports_signing: true,
                // ... populate other capabilities
            };
            capabilities.push(cap);
        }
        
        Ok(capabilities)
    }
}
```

### **Step 3: Working CLI** (3-4 hours)

**File:** `crates/beardog-cli/src/main.rs`

**Need to implement:**
```rust
use clap::{Parser, Subcommand};
use beardog_tunnel::universal_hsm_discovery::capability_detection::pkcs11_prober::Pkcs11CapabilityProber;
use beardog_tunnel::universal_hsm::providers::pkcs11::Pkcs11HsmProvider;

#[derive(Parser)]
#[command(name = "beardog")]
#[command(about = "BearDog HSM Management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover available HSMs
    DiscoverHsm {
        /// PKCS#11 library path
        #[arg(short, long, default_value = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")]
        library: String,
    },
    /// Test entropy collection
    TestEntropy {
        /// Slot ID to test
        #[arg(short, long)]
        slot: u32,
        /// Number of bytes to collect
        #[arg(short, long, default_value = "1024")]
        size: usize,
    },
    /// Mix entropy from multiple sources
    MixSeed {
        /// Comma-separated list of slot IDs
        #[arg(short, long)]
        slots: String,
        /// Output file
        #[arg(short, long)]
        output: String,
    },
    /// Show system status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::DiscoverHsm { library } => {
            println!("🔍 Discovering HSMs using: {}", library);
            let prober = Pkcs11CapabilityProber::new()?;
            let capabilities = prober.probe_capabilities(&library).await?;
            
            println!("Found {} HSM device(s):\n", capabilities.len());
            for (i, cap) in capabilities.iter().enumerate() {
                println!("Device {}: Slot {}", i + 1, cap.slot_id.unwrap_or(0));
                println!("  Type: {}", cap.provider_type);
                println!("  Random: {}", if cap.supports_random { "✅" } else { "❌" });
                println!("  Encryption: {}", if cap.supports_encryption { "✅" } else { "❌" });
                println!();
            }
        }
        Commands::TestEntropy { slot, size } => {
            println!("🎲 Testing entropy collection from slot {}...", slot);
            println!("Collecting {} bytes...", size);
            
            // TODO: Implement entropy collection
            println!("✅ Entropy collection successful!");
        }
        Commands::MixSeed { slots, output } => {
            println!("🌀 Mixing entropy from slots: {}", slots);
            println!("Output file: {}", output);
            
            // TODO: Implement entropy mixing
            println!("✅ Seed mixing complete!");
        }
        Commands::Status => {
            println!("🐻 BearDog Status");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Platform: {}", std::env::consts::OS);
            println!("Architecture: {}", std::env::consts::ARCH);
        }
    }
    
    Ok(())
}
```

**Add to `crates/beardog-cli/Cargo.toml`:**
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
beardog-tunnel = { path = "../beardog-tunnel" }
beardog-errors = { path = "../beardog-errors" }
```

### **Step 4: Configuration File** (1-2 hours)

**File:** `configs/eastgate-production.toml`

```toml
[system]
name = "eastgate"
platform = "linux-x86_64"
cores = 20

[hsm.pkcs11]
enabled = true
# Try these paths in order
library_paths = [
    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
    "/usr/lib/x86_64-linux-gnu/pkcs11/opensc-pkcs11.so",
]

# Auto-discover all slots, or specify explicitly:
[[hsm.pkcs11.devices]]
slot = 0
label = "SoloKey-01"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 1
label = "SoloKey-02"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 2
label = "SoloKey-03"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 3
label = "SoloKey-04"
purpose = "entropy"

[entropy]
mixing_algorithm = "sha3-512-kdf"
min_sources = 2
require_hardware = true
bytes_per_source = 256
```

---

## 🧪 **TESTING WORKFLOW**

### **1. Build the CLI:**
```bash
cd /home/eastgate/Development/ecoPrimals/beardog
cargo build --release -p beardog-cli
```

### **2. Test HSM discovery:**
```bash
./target/release/beardog-cli discover-hsm
# Should list all 4 SoloKey devices
```

### **3. Test entropy from one device:**
```bash
./target/release/beardog-cli test-entropy --slot 0 --size 1024
# Should collect 1KB from first SoloKey
```

### **4. Mix entropy from all devices:**
```bash
./target/release/beardog-cli mix-seed --slots 0,1,2,3 --output test-seed.bin
# Should create mixed seed file
```

### **5. Verify the seed file:**
```bash
ls -lh test-seed.bin
hexdump -C test-seed.bin | head
# Should show random-looking data
```

---

## 🎯 **SUCCESS CRITERIA**

Phase 1 is complete when:

- [ ] `beardog-cli discover-hsm` detects all 4 SoloKey V2 devices
- [ ] `beardog-cli test-entropy` successfully collects entropy from each device
- [ ] `beardog-cli mix-seed` creates a valid seed file mixing entropy from all devices
- [ ] No TODO comments remain in PKCS#11 implementation
- [ ] No mock implementations in production code paths
- [ ] Configuration loaded from file (no hardcoded paths)
- [ ] CLI has proper error messages (no panics on errors)

---

## 📊 **ESTIMATED TIMELINE**

| Task | Time | Status |
|------|------|--------|
| PKCS#11 Provider implementation | 2-3h | ⏳ Ready |
| PKCS#11 Prober implementation | 1-2h | ⏳ Ready |
| CLI implementation | 3-4h | ⏳ Ready |
| Configuration system | 1-2h | ⏳ Ready |
| Testing & debugging | 2-3h | ⏳ Ready |
| **TOTAL** | **10-14h** | **Phase 1** |

**Realistic estimate:** 2-3 days of focused work, or 1 long weekend.

---

## 🐛 **TROUBLESHOOTING**

### **If SoloKeys not detected:**
1. Check if devices are connected: `lsusb | grep Solo`
2. Check permissions: May need udev rules for USB access
3. Try different PKCS#11 library paths
4. Check dmesg for USB errors: `dmesg | grep -i usb`

### **If PKCS#11 library fails to load:**
1. Verify library exists: `ls -la /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
2. Check dependencies: `ldd /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
3. Try alternative library path
4. Check library permissions

### **If entropy collection fails:**
1. Check if device is initialized
2. Verify slot ID is correct
3. Check device firmware version
4. Review logs for specific errors

---

## 📚 **REFERENCES**

- **cryptoki crate docs:** https://docs.rs/cryptoki/
- **PKCS#11 spec:** http://docs.oasis-open.org/pkcs11/pkcs11-base/
- **OpenSC project:** https://github.com/OpenSC/OpenSC
- **SoloKeys docs:** https://solokeys.com/

---

## 🚀 **NEXT STEPS**

After Phase 1 completion:
1. Mark mvp-1 as completed
2. Start Phase 2: Multi-source entropy mixing
3. Begin Android build setup
4. Write hardware integration tests

---

**Created:** October 29, 2025  
**Status:** 📋 Ready to implement  
**Next Action:** Implement PKCS#11 provider using cryptoki crate

🐻 **Let's make it real!**

