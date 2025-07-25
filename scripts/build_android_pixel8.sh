#!/bin/bash
#
# BearDog Android Pixel 8 Build and Deploy Script
# Builds native StrongBox HSM and deploys to Pixel 8 device
#

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ANDROID_TARGET="aarch64-linux-android"
BUILD_TYPE="release"
DEVICE_NAME="Pixel 8"
ADB_TIMEOUT=30

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Print header
echo -e "${GREEN}"
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                BearDog Android Pixel 8 Builder                ║"
echo "║              Native StrongBox HSM Integration                 ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if Android target is installed
    if ! rustup target list --installed | grep -q "$ANDROID_TARGET"; then
        log_warning "Android target $ANDROID_TARGET not installed. Installing..."
        rustup target add $ANDROID_TARGET
    fi
    
    # Check for Android NDK
    if [ -z "$ANDROID_NDK_HOME" ]; then
        log_error "ANDROID_NDK_HOME not set. Please install Android NDK and set the environment variable."
        exit 1
    fi
    
    if [ ! -d "$ANDROID_NDK_HOME" ]; then
        log_error "Android NDK directory not found: $ANDROID_NDK_HOME"
        exit 1
    fi
    
    # Check for ADB
    if ! command -v adb &> /dev/null; then
        log_error "ADB not found. Please install Android SDK platform tools."
        exit 1
    fi
    
    # Check device connection
    if ! adb devices | grep -q "device$"; then
        log_error "No Android device connected. Please connect your Pixel 8 and enable USB debugging."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Setup environment
setup_environment() {
    log_info "Setting up Android build environment..."
    
    # Set up Android NDK environment
    export CC_aarch64_linux_android="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"
    export CXX_aarch64_linux_android="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang++"
    export AR_aarch64_linux_android="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"
    
    log_success "Environment configured for Android build"
}

# Detect device info
detect_device() {
    log_info "Detecting connected Android device..."
    
    DEVICE_MANUFACTURER=$(adb shell getprop ro.product.manufacturer | tr -d '\r')
    DEVICE_MODEL=$(adb shell getprop ro.product.model | tr -d '\r')
    ANDROID_VERSION=$(adb shell getprop ro.build.version.release | tr -d '\r')
    SECURITY_PATCH=$(adb shell getprop ro.build.version.security_patch | tr -d '\r')
    VERIFIED_BOOT=$(adb shell getprop ro.boot.verifiedbootstate | tr -d '\r')
    
    log_info "Device detected:"
    log_info "  📱 Manufacturer: $DEVICE_MANUFACTURER"
    log_info "  📱 Model: $DEVICE_MODEL"
    log_info "  📱 Android: $ANDROID_VERSION"
    log_info "  🔒 Security patch: $SECURITY_PATCH"
    log_info "  ✅ Verified boot: $VERIFIED_BOOT"
    
    # Check if it's a Pixel device
    if [[ "$DEVICE_MANUFACTURER" == "Google" && "$DEVICE_MODEL" == *"Pixel"* ]]; then
        log_success "Pixel device detected - excellent StrongBox support expected"
        
        # Check for Pixel 8 specifically
        if [[ "$DEVICE_MODEL" == *"Pixel 8"* ]]; then
            log_success "Pixel 8 detected - Titan M2 StrongBox available"
        fi
    else
        log_warning "Non-Pixel device detected - StrongBox support may be limited"
    fi
    
    # Check verified boot state
    if [[ "$VERIFIED_BOOT" == "green" ]]; then
        log_success "Verified boot state: GREEN - optimal security"
    else
        log_warning "Verified boot state: $VERIFIED_BOOT - may impact security"
    fi
}

# Check StrongBox availability
check_strongbox() {
    log_info "Checking StrongBox availability on device..."
    
    # Check for StrongBox keystore feature
    if adb shell pm list features | grep -q "android.hardware.strongbox_keystore"; then
        log_success "StrongBox keystore feature available"
    else
        log_warning "StrongBox keystore feature not found"
    fi
    
    # Check for hardware keystore
    KEYSTORE_IMPL=$(adb shell getprop ro.hardware.keystore 2>/dev/null | tr -d '\r')
    if [[ -n "$KEYSTORE_IMPL" ]]; then
        log_info "Hardware keystore implementation: $KEYSTORE_IMPL"
    fi
    
    # Check for Titan M specific properties
    CITADEL_VERSION=$(adb shell getprop ro.vendor.citadel.version 2>/dev/null | tr -d '\r')
    if [[ -n "$CITADEL_VERSION" ]]; then
        log_success "Titan M (Citadel) version: $CITADEL_VERSION"
    fi
}

# Build for Android
build_android() {
    log_info "Building BearDog for Android ($ANDROID_TARGET)..."
    
    # Build the tunnel crate with Android features
    log_info "Building beardog-tunnel with StrongBox support..."
    cargo build \
        --target $ANDROID_TARGET \
        --$([ "$BUILD_TYPE" = "release" ] && echo "release" || echo "") \
        -p beardog-tunnel \
        --features "android_native,strongbox_hardware,pixel8_optimizations" \
        --verbose
    
    # Build the mobile HSM demo
    log_info "Building mobile HSM demo..."
    cargo build \
        --target $ANDROID_TARGET \
        --$([ "$BUILD_TYPE" = "release" ] && echo "release" || echo "") \
        --example mobile_hsm_demo \
        --features "android_native,strongbox_hardware" \
        --verbose
    
    log_success "Android build completed successfully"
}

# Deploy to device
deploy_to_device() {
    log_info "Deploying BearDog to device..."
    
    # Create device directory
    adb shell mkdir -p /data/local/tmp/beardog
    
    BUILD_DIR="target/$ANDROID_TARGET/$BUILD_TYPE"
    
    # Push the mobile HSM demo
    if [ -f "$BUILD_DIR/examples/mobile_hsm_demo" ]; then
        adb push "$BUILD_DIR/examples/mobile_hsm_demo" /data/local/tmp/beardog/
        adb shell chmod 755 /data/local/tmp/beardog/mobile_hsm_demo
        log_success "Mobile HSM demo deployed"
    else
        log_warning "Mobile HSM demo binary not found"
    fi
    
    # Create device-specific configuration
    create_device_config
    
    log_success "Deployment completed"
}

# Create device-specific configuration
create_device_config() {
    log_info "Creating device-specific configuration..."
    
    cat > /tmp/beardog_pixel8.toml << EOF
# BearDog Pixel 8 Configuration
# Generated automatically for detected device

[device]
manufacturer = "$DEVICE_MANUFACTURER"
model = "$DEVICE_MODEL"
android_version = "$ANDROID_VERSION"
security_patch = "$SECURITY_PATCH"
verified_boot_state = "$VERIFIED_BOOT"

[hsm]
# Primary HSM - Android StrongBox (Pixel 8 optimized)
primary_hsm = "android_strongbox"
require_strongbox = true
enable_titan_m = true
require_user_presence = false  # Can be enabled per operation

[hsm.android_strongbox]
enabled = true
strongbox_required = true
hardware_backed_only = true
attestation_required = true
key_protection_level = "strongbox"

[hsm.software_fallback]
enabled = true
implementation = "rust_software_hsm"
memory_protection = "high"
max_key_count = 1000

[security]
verified_boot_required = $([ "$VERIFIED_BOOT" = "green" ] && echo "true" || echo "false")
min_android_version = "9"
require_hardware_backing = true

[logging]
level = "info"
enable_android_logger = true
EOF

    adb push /tmp/beardog_pixel8.toml /data/local/tmp/beardog/config.toml
    log_success "Device configuration created and deployed"
}

# Run tests on device
run_device_tests() {
    log_info "Running StrongBox tests on device..."
    
    # Test device detection
    log_info "Testing device detection..."
    adb shell "cd /data/local/tmp/beardog && RUST_LOG=info ./mobile_hsm_demo 2>&1" | head -50
    
    log_success "Device tests completed"
}

# Show device capabilities
show_device_capabilities() {
    log_info "Device capabilities summary:"
    echo
    echo "📱 Device Information:"
    echo "   Manufacturer: $DEVICE_MANUFACTURER"
    echo "   Model: $DEVICE_MODEL"
    echo "   Android Version: $ANDROID_VERSION"
    echo "   Security Patch: $SECURITY_PATCH"
    echo
    echo "🔒 Security Features:"
    echo "   Verified Boot: $VERIFIED_BOOT"
    echo "   StrongBox: $(adb shell pm list features | grep -q "android.hardware.strongbox_keystore" && echo "Available" || echo "Not Available")"
    echo "   Hardware Keystore: $(adb shell getprop ro.hardware.keystore 2>/dev/null || echo "Unknown")"
    echo
    echo "🛡️ Titan M Information:"
    CITADEL_VERSION=$(adb shell getprop ro.vendor.citadel.version 2>/dev/null | tr -d '\r')
    if [[ -n "$CITADEL_VERSION" ]]; then
        echo "   Citadel Version: $CITADEL_VERSION"
    else
        echo "   Citadel: Not detected"
    fi
    echo
}

# Main execution
main() {
    # Check if device argument provided
    if [[ "$1" == "--device-info" ]]; then
        if adb devices | grep -q "device$"; then
            detect_device
            check_strongbox
            show_device_capabilities
        else
            log_error "No device connected"
        fi
        exit 0
    fi
    
    # Full build and deploy process
    check_prerequisites
    setup_environment
    detect_device
    check_strongbox
    build_android
    deploy_to_device
    
    # Run tests if requested
    if [[ "$1" == "--test" ]]; then
        run_device_tests
    fi
    
    show_device_capabilities
    
    log_success "🎉 BearDog Android StrongBox build and deployment completed!"
    log_info "To run the mobile HSM demo:"
    log_info "  adb shell 'cd /data/local/tmp/beardog && ./mobile_hsm_demo'"
}

# Run main function with all arguments
main "$@" 