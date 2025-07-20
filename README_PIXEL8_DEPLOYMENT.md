# BearDog Pure Rust Android Deployment

Deploy BearDog HSM system on Pixel 8 devices running GrapheneOS using **100% Rust** - no Java, no shell scripts, no external dependencies.

## 🚀 Quick Start

### Prerequisites
- **Linux development machine** (for Android cross-compilation)
- **Android NDK** (download and set `ANDROID_NDK_HOME`)
- **Pixel 8** device with USB debugging enabled
- **ADB** (Android Debug Bridge)
- **Rust toolchain** with cargo

### One-Command Deployment

```bash
# Install the deployment tool
cargo install --path crates/beardog-deploy

# Deploy to your connected Pixel 8
deploy-pixel8 full --release
```

### Step-by-Step Deployment

1. **Check System**
   ```bash
   deploy-pixel8 check
   ```

2. **Build Application**
   ```bash
   deploy-pixel8 build --release
   ```

3. **Deploy to Device**
   ```bash
   deploy-pixel8 deploy --release
   ```

4. **Run on Device**
   ```bash
   deploy-pixel8 run
   ```

5. **Monitor Logs**
   ```bash
   deploy-pixel8 logs --follow
   ```

## 🛠️ Detailed Setup

### 1. Development Environment

#### Install Android NDK
```bash
# Download from: https://developer.android.com/ndk/downloads
# Extract and set environment variable:
export ANDROID_NDK_HOME=/path/to/android-ndk
```

#### Install Rust Targets
```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

### 2. Device Preparation

#### Enable Developer Options (Pixel 8)
1. Go to **Settings** > **About phone**
2. Tap **Build number** 7 times
3. Go to **Settings** > **System** > **Developer options**
4. Enable **USB debugging**

#### Connect Device
```bash
# Verify connection
adb devices

# Should show your device:
# ABC123DEF456    device
```

### 3. Build Configuration

#### Android Library (`android/Cargo.toml`)
- **Pure Rust** Android shared library
- NDK integration via `ndk-glue`
- StrongBox HSM support
- Targets Android API 28+ (required for StrongBox)

#### Example Application (`examples/pixel8_native_app.rs`)
- Comprehensive HSM testing
- Performance benchmarks  
- Ecosystem identity generation
- GrapheneOS optimization

## 📱 Device Compatibility

### Optimal Configuration
- **Hardware**: Google Pixel 8
- **OS**: GrapheneOS (privacy-focused Android)
- **Security**: Titan M security chip
- **API Level**: 28+ (Android 9+)

### Compatibility Matrix

| Device | StrongBox | Titan M | GrapheneOS | Status |
|--------|-----------|---------|------------|--------|
| Pixel 8 | ✅ | ✅ | ✅ | **Optimal** |
| Pixel 7 | ✅ | ✅ | ✅ | **Good** |
| Pixel 6 | ✅ | ✅ | ✅ | **Good** |
| Other | ❓ | ❌ | ❓ | **Limited** |

## 🔧 Deployment Tool Commands

### Full Workflow
```bash
# Complete deployment process
deploy-pixel8 full [--release]
```

### Individual Steps
```bash
# System check
deploy-pixel8 check [--device-only]

# Build only
deploy-pixel8 build [--release] [--target aarch64-linux-android]

# Deploy only (skip build)
deploy-pixel8 deploy [--release] [--skip-build]

# Run application
deploy-pixel8 run [--args "arg1 arg2"]

# Monitor logs
deploy-pixel8 logs [--filter "BearDog"] [--follow]
```

### Advanced Options
```bash
# Verbose output
deploy-pixel8 --verbose full

# Custom project directory
deploy-pixel8 --project-root /path/to/beardog full

# Release build with custom target
deploy-pixel8 build --release --target aarch64-linux-android
```

## 🏗️ Architecture Overview

### Pure Rust Stack
```
┌─────────────────────────────────────┐
│          BearDog HSM System         │
├─────────────────────────────────────┤
│        Pure Rust Android App        │
├─────────────────────────────────────┤
│         Android NDK (ndk-rs)        │
├─────────────────────────────────────┤
│       Android StrongBox API         │
├─────────────────────────────────────┤
│         Titan M Security Chip       │
└─────────────────────────────────────┘
```

### Key Components

1. **Deployment Tool** (`crates/beardog-deploy`)
   - Pure Rust CLI tool
   - Cross-compilation management
   - Device communication
   - Log monitoring

2. **Android Library** (`android/`)
   - Native Android shared library
   - NDK integration via `ndk-glue`
   - Hardware abstraction layer

3. **HSM Implementation** (`crates/beardog-tunnel`)
   - Android StrongBox provider
   - Titan M integration
   - Hardware key operations

4. **Example Application** (`examples/`)
   - Comprehensive testing
   - Performance benchmarks
   - Real-world usage patterns

## 🔐 Security Features

### Hardware Security Module
- **StrongBox Keystore**: Hardware-backed key storage
- **Titan M Integration**: Google's security chip
- **Hardware Attestation**: Cryptographic proof of key provenance
- **TEE Operations**: Trusted Execution Environment

### GrapheneOS Enhancements
- **Verified Boot**: Cryptographic boot verification
- **Hardened Kernel**: Enhanced exploit mitigations
- **Privacy Controls**: Fine-grained permission management
- **Network Hardening**: Reduced attack surface

### BearDog Security
- **Zero-Copy Crypto**: Memory-safe operations
- **Async Security**: Non-blocking cryptographic operations
- **Ecosystem Identity**: Decentralized authentication
- **Recovery System**: Hardware-backed recovery

## 🧪 Testing & Validation

### Automated Tests
```bash
# Run HSM integration tests
cargo test --package beardog-tunnel --features android-hsm

# Performance benchmarks
cargo bench --package beardog-config
```

### Manual Validation
```bash
# Deploy and test
deploy-pixel8 full --release

# Monitor comprehensive testing
deploy-pixel8 logs --filter "Phase" --follow
```

### Expected Output
```
🐻 BearDog Pure Rust Android App Starting
========================================
📱 Pixel 8 Configuration:
   Titan M Required: true
   Green Boot Required: true
   Attestation Enabled: true
   Security Level: Maximum
   Performance Mode: MaxSecurity
🚀 Initializing BearDog HSM system...
✅ HSM System Initialized:
   HSM ID: pixel8-strongbox-abc123
   Vendor: Google
   Model: Titan M
   Anchor Key: beardog-anchor-key
🧪 Testing core HSM operations...
   ✅ Data signed: 64 bytes
   ✅ Signature verification passed
   ✅ HSM Health: Healthy
⚡ Running performance tests...
   ✅ Performance test completed:
   📊 Total time: 2.150s
   📊 Operations/second: 18.60
   📊 Average latency: 53.75ms
🎉 BearDog Android demo completed successfully!
```

## 🚨 Troubleshooting

### Common Issues

#### NDK Not Found
```bash
# Error: ANDROID_NDK_HOME not set
export ANDROID_NDK_HOME=/path/to/ndk
```

#### Device Not Detected
```bash
# Check USB debugging is enabled
adb devices

# If unauthorized, accept on device
```

#### Build Failures
```bash
# Clean build
cargo clean
deploy-pixel8 build --release
```

#### Permission Denied
```bash
# Ensure executable permissions
adb shell chmod 755 /data/local/tmp/libpixel8_native_app.so
```

### Debug Mode
```bash
# Enable verbose logging
deploy-pixel8 --verbose full

# Monitor all logs
deploy-pixel8 logs --filter "" --follow
```

## 📈 Performance Optimization

### Release Builds
```bash
# Always use release mode for production
deploy-pixel8 full --release
```

### Hardware Optimization
- Uses Titan M for all cryptographic operations
- Zero-copy memory management
- Async/await for non-blocking operations
- SIMD acceleration where available

### GrapheneOS Tuning
- Disables unnecessary services
- Optimizes memory allocation
- Enables hardware security features
- Reduces system overhead

## 🤝 Contributing

### Development Workflow
1. Make changes to BearDog code
2. Test with `deploy-pixel8 check`
3. Build with `deploy-pixel8 build`
4. Deploy and test on device
5. Monitor with `deploy-pixel8 logs`

### Adding New Features
1. Implement in appropriate crate
2. Add Android-specific code if needed
3. Update deployment tool if required
4. Test on real hardware
5. Update documentation

## 📚 Additional Resources

- [BearDog Architecture Overview](docs/architecture/DECENTRALIZED_ARCHITECTURE_DESIGN.md)
- [Android StrongBox Documentation](https://source.android.com/security/keystore)
- [GrapheneOS Security Features](https://grapheneos.org/features)
- [Pixel 8 Hardware Security](docs/mobile/PIXEL_8_GRAPHENE_DEPLOYMENT_GUIDE.md)

## 🏆 Why Pure Rust?

- **Memory Safety**: No buffer overflows or use-after-free
- **Performance**: Zero-cost abstractions and optimal compilation
- **Security**: Type system prevents entire classes of vulnerabilities
- **Maintainability**: Single language for entire stack
- **Deployment**: No runtime dependencies or complex setups
- **Cross-Platform**: Same code runs on development machine and device 