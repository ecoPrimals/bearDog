# 🐕 BearDog Mobile HSM: Pixel 8 + GrapheneOS Setup

Turn your Pixel 8 into a secure mobile HSM node in the EcoPrimals ecosystem!

## 🎯 Quick Start

### Prerequisites
- **Google Pixel 8** (or newer)
- **GrapheneOS** installed
- **Android NDK** installed
- **ADB** tools available

### 1. Install GrapheneOS
Follow the official GrapheneOS installation guide for Pixel 8:
```bash
# Enable OEM unlocking and USB debugging
# Flash GrapheneOS using the web installer
# https://grapheneos.org/install/
```

### 2. Setup Development Environment
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Android NDK
# Download from: https://developer.android.com/ndk/downloads
export ANDROID_NDK_HOME=/path/to/android-ndk

# Install ADB
sudo apt install android-tools-adb  # Ubuntu/Debian
# or
brew install android-platform-tools  # macOS
```

### 3. Build and Deploy BearDog
```bash
# Clone BearDog
git clone https://github.com/ecoprimal/beardog.git
cd beardog

# Build for Pixel 8 + GrapheneOS
./scripts/build_android.sh --target pixel8 --os graphene --deploy --test
```

### 4. Test on Device
```bash
# Test basic functionality
adb shell /data/local/tmp/beardog/beardog-cli hsm info

# Test StrongBox integration
adb shell /data/local/tmp/beardog/beardog-cli hsm generate-key --type ecc-p256 --id test_key

# Test biometric authentication
adb shell /data/local/tmp/beardog/beardog-cli hsm sign --key test_key --data "hello world" --biometric

# Run BiomeOS integration demo
adb shell /data/local/tmp/beardog/biomeos_mobile_integration_demo
```

## 📱 Features

### Hardware Security
- **Titan M Security Chip**: Hardware-backed key generation
- **StrongBox HSM**: FIPS 140-2 Level 3 equivalent
- **Hardware Attestation**: Cryptographic proof of key authenticity
- **Biometric Authentication**: Fingerprint/face unlock integration

### GrapheneOS Benefits
- **Enhanced Security**: Hardened Android with privacy controls
- **Verified Boot**: Complete boot verification pipeline
- **Privacy Controls**: User-controlled data sharing
- **Open Source**: Full transparency and auditability

### BearDog Integration
- **Universal HSM**: Works with any primal in the ecosystem
- **Mobile Node**: Participate in distributed security
- **Cross-Primal Auth**: Secure communication between services
- **Audit Trails**: Complete security operation logging

## 🔐 Security Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Android Application                          │
│                     (BearDog Mobile)                           │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│                BearDog Security Provider                       │
│              (Universal HSM Interface)                         │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│              Android Keystore Service                          │
│                 (GrapheneOS Enhanced)                          │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────────┐
│                 Android StrongBox                              │
│              (Titan M Security Chip)                          │
└─────────────────────────────────────────────────────────────────┘
```

## 🌐 BiomeOS Integration (Coming Soon)

Your Pixel 8 will become a mobile BiomeOS node capable of:

- **Secure Authentication**: Hardware-backed identity
- **Distributed Security**: Provide security services to other nodes
- **Mobile Compute**: Participate in distributed computing
- **Edge Storage**: Encrypted storage for the ecosystem

## 🔧 Development

### Build Options
```bash
# Debug build
./scripts/build_android.sh --build-type debug

# Deploy without tests
./scripts/build_android.sh --deploy

# Test on device
./scripts/build_android.sh --test
```

### Configuration
Edit `~/.config/beardog/mobile.toml` on device:
```toml
[hsm]
primary_hsm = "android_strongbox"
require_biometric = true
attestation_required = true

[device_detection]
manufacturer = "Google"
model = "Pixel 8"
strongbox_implementation = "titan_m"
```

## 🚀 What's Next?

1. **Get your Pixel 8** (on the way!)
2. **Install GrapheneOS** 
3. **Deploy BearDog** using this guide
4. **Test HSM integration**
5. **Contribute to BiomeOS mobile integration**

## 📊 Performance

### Expected Performance on Pixel 8
- **Key Generation**: ~50-100ms (ECC P-256)
- **Signing**: ~5-15ms (ECDSA)
- **Biometric Auth**: ~200-500ms
- **Attestation**: ~100-300ms

### Security Guarantees
- **Hardware-backed keys**: Never leave secure hardware
- **Verified boot**: Green state with locked bootloader
- **Biometric protection**: User presence validation
- **Audit trails**: Complete operation logging

---

**🎉 Ready to turn your Pixel 8 into a secure mobile HSM node!**

For more details, see:
- [Full Deployment Guide](docs/mobile/PIXEL_8_GRAPHENE_DEPLOYMENT_GUIDE.md)
- [BiomeOS Integration Demo](examples/biomeos_mobile_integration_demo.rs)
- [Android Build Script](scripts/build_android.sh) 