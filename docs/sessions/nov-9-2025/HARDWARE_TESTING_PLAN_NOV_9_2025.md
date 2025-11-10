# 🔐 Hardware Testing Plan - November 9, 2025

**Your Hardware**:
- 2x SoloKey Hackers (FIDO2/PKCS#11)
- 1x Pixel 8a with GrapheneOS (StrongBox/Titan M2)
- Tower: i9-12900K system

**BearDog Status**: Production Ready for HSM Testing ✅

---

## 🎯 Phase 1: SoloKey Testing (USB HSM)

### Prerequisites

```bash
# Install dependencies
sudo apt update
sudo apt install opensc pcscd pcsc-tools libpcsclite-dev

# Start PC/SC daemon
sudo systemctl enable pcscd
sudo systemctl start pcscd
sudo systemctl status pcscd

# Install udev rules for SoloKeys (if needed)
sudo wget https://raw.githubusercontent.com/solokeys/solo2-cli/main/70-solo2.rules \
  -O /etc/udev/rules.d/70-solo2.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Test 1: Device Discovery

```bash
cd /home/eastgate/Development/ecoPrimals/beardog

# Build the CLI
cargo build --release --bin beardog

# Plug in BOTH SoloKeys

# Discover all HSM devices
./target/release/beardog discover-hsm

# Expected output:
# ✅ Found 2 HSM device(s):
# Device #1 - Slot: 0
# Device #2 - Slot: 1
```

### Test 2: Raw Detection

```bash
# Check PC/SC sees the devices
pcsc_scan

# List all smart card readers
opensc-tool --list-readers

# You should see both SoloKeys
```

### Test 3: Entropy Generation

```bash
# Test first SoloKey
./target/release/beardog test-entropy --slot 0 --size 256

# Test second SoloKey  
./target/release/beardog test-entropy --slot 1 --size 256

# Compare entropy quality between devices
```

### Test 4: Mixed Entropy (MULTI-SOURCE!)

```bash
# Combine entropy from BOTH SoloKeys
./target/release/beardog mix-seed \
  --slots 0,1 \
  --output /tmp/mixed-entropy.bin

# Verify the output
ls -lh /tmp/mixed-entropy.bin
hexdump -C /tmp/mixed-entropy.bin | head -20
```

### Test 5: Key Generation

```bash
# Generate a key using SoloKey (if PIV is initialized)
# Note: SoloKeys need PIV initialization first

# Check if PIV is initialized
opensc-tool --reader 0 --send-apdu 00:A4:04:00:09:A0:00:00:03:08:00:00:10:00
```

---

## 📱 Phase 2: Pixel 8a Testing (Android StrongBox)

### Prerequisites

```bash
# Enable USB debugging on Pixel 8a
# Settings → About phone → Tap "Build number" 7 times
# Settings → System → Developer options → Enable USB debugging

# Connect Pixel to tower via USB
# Accept the USB debugging prompt on phone

# Verify connection
adb devices
# Should show your Pixel 8a

# Check StrongBox availability
adb shell pm list features | grep strongbox
# Should show: feature:android.hardware.strongbox_keystore
```

### Test 6: StrongBox Detection

```bash
# Check Titan M2 chip
adb shell getprop ro.hardware.keystore
# Should show hardware-backed keystore

# Verify GrapheneOS
adb shell getprop ro.build.fingerprint
# Should show GrapheneOS signature

# Check security patch
adb shell getprop ro.build.version.security_patch
```

### Test 7: Build BearDog for Android

```bash
cd /home/eastgate/Development/ecoPrimals/beardog

# Add Android targets (if not already added)
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Install cargo-ndk (if not installed)
cargo install cargo-ndk

# Build for Android (Pixel 8a is aarch64)
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release

# Or use the Android gradle build
cd android
./gradlew assembleRelease
```

### Test 8: Deploy to Pixel

```bash
# Install the APK
adb install -r app/build/outputs/apk/release/app-release.apk

# Launch the app
adb shell am start -n com.beardog.mobile/.MainActivity

# Check logs
adb logcat | grep BearDog
```

### Test 9: StrongBox Key Generation

From the Android app or via ADB:

```bash
# Generate a key in StrongBox
adb shell am broadcast -a com.beardog.GENERATE_KEY \
  --es key_id "test_key_1" \
  --es key_type "EC"

# Verify key was created in hardware
adb shell dumpsys android.security.keystore

# Look for "SecurityLevel: StrongBox"
```

---

## 🔥 Phase 3: Multi-Device Federation

### Test 10: Combine ALL THREE Devices!

**Goal**: Mix entropy from both SoloKeys AND Pixel 8a's Titan M2!

```rust
// Example Rust code (pseudo-code for now)
use beardog_security::hsm::{Pkcs11Provider, AndroidStrongBoxProvider};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize SoloKey 1
    let solokey1 = Pkcs11Provider::new("/usr/lib/opensc-pkcs11.so", 0)?;
    
    // Initialize SoloKey 2
    let solokey2 = Pkcs11Provider::new("/usr/lib/opensc-pkcs11.so", 1)?;
    
    // Initialize Pixel StrongBox (via network or USB tethering)
    let pixel = AndroidStrongBoxProvider::connect("192.168.1.X:8443").await?;
    
    // Generate entropy from all 3 sources
    let entropy1 = solokey1.generate_random(32).await?;
    let entropy2 = solokey2.generate_random(32).await?;
    let entropy3 = pixel.generate_random(32).await?;
    
    // Mix all three sources using SHA3-512
    let master_seed = mix_entropy(&[entropy1, entropy2, entropy3])?;
    
    println!("✅ Generated master seed from 3 hardware sources!");
    println!("   - SoloKey #1 (FIDO2)");
    println!("   - SoloKey #2 (FIDO2)");
    println!("   - Pixel 8a Titan M2 (StrongBox)");
    
    Ok(())
}
```

---

## 📊 Expected Results

### SoloKeys:
- ✅ Device detection via PKCS#11
- ✅ Entropy generation (256+ bytes)
- ✅ Multi-device entropy mixing
- ⚠️ Key generation (requires PIV initialization)

### Pixel 8a:
- ✅ StrongBox availability detection
- ✅ Titan M2 chip recognition
- ✅ Hardware-backed key generation
- ✅ Key attestation
- ✅ TEE operations

### Combined:
- 🎯 **3-source hardware entropy mixing**
- 🎯 **Multi-HSM key backup**
- 🎯 **Federated cryptographic operations**

---

## 🔧 Troubleshooting

### SoloKeys Not Detected?

```bash
# Check USB connection
lsusb | grep -i solo

# Check PC/SC daemon
sudo systemctl status pcscd

# Restart PC/SC
sudo systemctl restart pcscd

# Check permissions
sudo usermod -a -G pcscd $USER
# Log out and back in
```

### Pixel Not Connecting?

```bash
# Check ADB connection
adb devices

# Restart ADB server
adb kill-server
adb start-server

# Check USB mode on phone
# Should be "File Transfer" or "PTP", not "Charging only"
```

### PIV Not Initialized on SoloKeys?

```bash
# Initialize PIV applet (if needed)
yubico-piv-tool -a reset
# Note: This works with SoloKeys too!

# Or use OpenSC
pkcs15-init --erase-card --create-pkcs15

# Set a PIN
pkcs15-init --store-pin --auth-id 01
```

---

## 📝 Testing Checklist

### Phase 1: SoloKeys (USB)
- [ ] PC/SC daemon running
- [ ] Both SoloKeys detected
- [ ] Entropy generation working
- [ ] Mixed entropy from both keys
- [ ] CLI commands functional

### Phase 2: Pixel 8a (Android)
- [ ] ADB connection established
- [ ] StrongBox feature detected
- [ ] Titan M2 recognized
- [ ] BearDog APK built
- [ ] App installed on device
- [ ] Key generation in StrongBox

### Phase 3: Multi-Device
- [ ] All 3 devices accessible
- [ ] Entropy from each device
- [ ] Combined entropy mixing
- [ ] Federated key operations

---

## 🎯 Next Steps After Testing

1. **Document Results** - Capture what works/doesn't work
2. **Performance Metrics** - Measure operation latencies
3. **Integration Testing** - Test with actual BearDog workflows
4. **Security Validation** - Verify hardware backing
5. **Federation Testing** - Multi-device coordination

---

## 🚀 Let's Start!

**Recommended Testing Order**:
1. Start with SoloKeys (easier, immediate feedback)
2. Move to Pixel 8a (more complex setup)
3. Combine all devices (advanced)

**First Command to Run**:
```bash
cd /home/eastgate/Development/ecoPrimals/beardog
cargo build --release --bin beardog
./target/release/beardog discover-hsm
```

---

**Status**: READY FOR HARDWARE TESTING ✅  
**Hardware**: 2x SoloKey + 1x Pixel 8a = 3 HSM sources 🔐  
**BearDog**: Production-ready HSM integration 🚀

