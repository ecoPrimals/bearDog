#!/bin/bash

# BearDog Android Build Script
# Optimized for Pixel 8 + GrapheneOS deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
TARGET_DEVICE="pixel8"
OS_TYPE="graphene"
BUILD_TYPE="release"
DEPLOY_DEVICE=false
RUN_TESTS=false

# Print usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  --target DEVICE    Target device (pixel8, pixel8pro, android) [default: pixel8]"
    echo "  --os OS_TYPE       OS type (graphene, android, calyxos) [default: graphene]"
    echo "  --build-type TYPE  Build type (release, debug) [default: release]"
    echo "  --deploy          Deploy to connected device via ADB"
    echo "  --test            Run tests on device after deployment"
    echo "  --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --target pixel8 --os graphene --deploy"
    echo "  $0 --build-type debug --test"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET_DEVICE="$2"
            shift 2
            ;;
        --os)
            OS_TYPE="$2"
            shift 2
            ;;
        --build-type)
            BUILD_TYPE="$2"
            shift 2
            ;;
        --deploy)
            DEPLOY_DEVICE=true
            shift
            ;;
        --test)
            RUN_TESTS=true
            shift
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Helper functions
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

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        log_error "Rust is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check if Android NDK is available
    if [ -z "$ANDROID_NDK_HOME" ]; then
        log_error "ANDROID_NDK_HOME is not set. Please set it to your Android NDK path."
        exit 1
    fi
    
    if [ ! -d "$ANDROID_NDK_HOME" ]; then
        log_error "Android NDK directory does not exist: $ANDROID_NDK_HOME"
        exit 1
    fi
    
    # Check if ADB is available (if deploying)
    if [ "$DEPLOY_DEVICE" = true ] && ! command -v adb &> /dev/null; then
        log_error "ADB is not installed. Please install Android SDK tools."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Setup Android build environment
setup_android_env() {
    log_info "Setting up Android build environment..."
    
    # Add Android targets
    rustup target add aarch64-linux-android
    rustup target add armv7-linux-androideabi
    rustup target add x86_64-linux-android
    rustup target add i686-linux-android
    
    # Install cargo-ndk if not already installed
    if ! command -v cargo-ndk &> /dev/null; then
        log_info "Installing cargo-ndk..."
        cargo install cargo-ndk
    fi
    
    # Set up environment variables
    export ANDROID_NDK_HOME="$ANDROID_NDK_HOME"
    export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
    
    log_success "Android build environment setup complete"
}

# Configure build for specific device
configure_device_build() {
    log_info "Configuring build for $TARGET_DEVICE with $OS_TYPE..."
    
    case $TARGET_DEVICE in
        pixel8|pixel8pro)
            export ANDROID_TARGET="aarch64-linux-android"
            export ANDROID_API_LEVEL="33"
            export DEVICE_ARCH="arm64-v8a"
            ;;
        android)
            export ANDROID_TARGET="aarch64-linux-android"
            export ANDROID_API_LEVEL="26"
            export DEVICE_ARCH="arm64-v8a"
            ;;
        *)
            log_error "Unsupported target device: $TARGET_DEVICE"
            exit 1
            ;;
    esac
    
    # Create device-specific config
    cat > .android_config.toml << EOF
[device]
target = "$TARGET_DEVICE"
os_type = "$OS_TYPE"
architecture = "$DEVICE_ARCH"
api_level = "$ANDROID_API_LEVEL"

[build]
target = "$ANDROID_TARGET"
build_type = "$BUILD_TYPE"

[features]
strongbox = true
biometric = true
attestation = true
graphene_optimizations = $([ "$OS_TYPE" = "graphene" ] && echo "true" || echo "false")
EOF
    
    log_success "Device configuration complete"
}

# Build BearDog for Android
build_beardog() {
    log_info "Building BearDog for Android ($BUILD_TYPE)..."
    
    # Set build flags based on build type
    if [ "$BUILD_TYPE" = "release" ]; then
        BUILD_FLAGS="--release"
    else
        BUILD_FLAGS=""
    fi
    
    # Build main binary
    log_info "Building BearDog CLI..."
    cargo build --target "$ANDROID_TARGET" $BUILD_FLAGS --bin beardog-cli
    
    # Build examples
    log_info "Building BiomeOS integration demo..."
    cargo build --target "$ANDROID_TARGET" $BUILD_FLAGS --example biomeos_mobile_integration_demo
    
    # Build tests if requested
    if [ "$RUN_TESTS" = true ]; then
        log_info "Building tests..."
        cargo build --target "$ANDROID_TARGET" $BUILD_FLAGS --tests
    fi
    
    log_success "Build completed successfully"
}

# Check if device is connected
check_device_connection() {
    log_info "Checking device connection..."
    
    if ! adb devices | grep -q "device$"; then
        log_error "No Android device connected. Please connect your device and enable USB debugging."
        exit 1
    fi
    
    # Get device info
    DEVICE_MODEL=$(adb shell getprop ro.product.model 2>/dev/null || echo "Unknown")
    DEVICE_MANUFACTURER=$(adb shell getprop ro.product.manufacturer 2>/dev/null || echo "Unknown")
    ANDROID_VERSION=$(adb shell getprop ro.build.version.release 2>/dev/null || echo "Unknown")
    
    log_info "Connected device: $DEVICE_MANUFACTURER $DEVICE_MODEL (Android $ANDROID_VERSION)"
    
    # Check if it's a Pixel device
    if [[ "$DEVICE_MANUFACTURER" == "Google" && "$DEVICE_MODEL" == *"Pixel"* ]]; then
        log_success "Pixel device detected - optimal StrongBox support"
    else
        log_warning "Non-Pixel device detected - StrongBox support may be limited"
    fi
    
    # Check for GrapheneOS
    if adb shell getprop ro.build.flavor 2>/dev/null | grep -q "graphene"; then
        log_success "GrapheneOS detected - enhanced security features available"
    elif [ "$OS_TYPE" = "graphene" ]; then
        log_warning "GrapheneOS expected but not detected"
    fi
}

# Deploy to device
deploy_to_device() {
    log_info "Deploying BearDog to device..."
    
    # Create device directory
    adb shell mkdir -p /data/local/tmp/beardog
    
    # Push main binary
    BUILD_DIR="target/$ANDROID_TARGET/$BUILD_TYPE"
    adb push "$BUILD_DIR/beardog-cli" /data/local/tmp/beardog/
    adb shell chmod 755 /data/local/tmp/beardog/beardog-cli
    
    # Push examples
    adb push "$BUILD_DIR/examples/biomeos_mobile_integration_demo" /data/local/tmp/beardog/
    adb shell chmod 755 /data/local/tmp/beardog/biomeos_mobile_integration_demo
    
    # Push tests if built
    if [ "$RUN_TESTS" = true ]; then
        for test_binary in "$BUILD_DIR/deps/beardog"*; do
            if [ -f "$test_binary" ] && [ -x "$test_binary" ]; then
                test_name=$(basename "$test_binary")
                adb push "$test_binary" /data/local/tmp/beardog/
                adb shell chmod 755 "/data/local/tmp/beardog/$test_name"
            fi
        done
    fi
    
    # Create configuration file
    cat > /tmp/beardog_mobile.toml << EOF
[hsm]
primary_hsm = "android_strongbox"
fallback_hsm = "software"

[hsm.android_strongbox]
enabled = true
require_user_presence = true
require_biometric = true
attestation_required = true
key_protection_level = "strongbox"

[device_detection]
manufacturer = "$DEVICE_MANUFACTURER"
model = "$DEVICE_MODEL"
android_version = "$ANDROID_VERSION"
os_type = "$OS_TYPE"

[security]
verified_boot_required = $([ "$OS_TYPE" = "graphene" ] && echo "true" || echo "false")
bootloader_locked = $([ "$OS_TYPE" = "graphene" ] && echo "true" || echo "false")
os_verification = "$OS_TYPE"
EOF
    
    adb push /tmp/beardog_mobile.toml /data/local/tmp/beardog/config.toml
    
    log_success "Deployment complete"
}

# Run tests on device
run_device_tests() {
    log_info "Running tests on device..."
    
    # Test basic functionality
    log_info "Testing basic functionality..."
    adb shell /data/local/tmp/beardog/beardog-cli --help
    
    # Test HSM capabilities
    log_info "Testing HSM capabilities..."
    adb shell /data/local/tmp/beardog/beardog-cli hsm info
    
    # Run BiomeOS integration demo
    log_info "Running BiomeOS integration demo..."
    adb shell /data/local/tmp/beardog/biomeos_mobile_integration_demo
    
    # Run unit tests if available
    if [ "$RUN_TESTS" = true ]; then
        log_info "Running unit tests..."
        for test_binary in /data/local/tmp/beardog/beardog*; do
            if adb shell "[ -f '$test_binary' ] && [ -x '$test_binary' ]"; then
                test_name=$(basename "$test_binary")
                if [[ "$test_name" == *"test"* ]]; then
                    log_info "Running $test_name..."
                    adb shell "$test_binary" || log_warning "Test $test_name failed"
                fi
            fi
        done
    fi
    
    log_success "Tests completed"
}

# Clean up
cleanup() {
    log_info "Cleaning up..."
    rm -f .android_config.toml
    rm -f /tmp/beardog_mobile.toml
    log_success "Cleanup complete"
}

# Main execution
main() {
    echo "🐕 BearDog Android Build Script"
    echo "================================"
    echo "Target: $TARGET_DEVICE"
    echo "OS: $OS_TYPE"
    echo "Build: $BUILD_TYPE"
    echo "Deploy: $DEPLOY_DEVICE"
    echo "Test: $RUN_TESTS"
    echo ""
    
    check_prerequisites
    setup_android_env
    configure_device_build
    build_beardog
    
    if [ "$DEPLOY_DEVICE" = true ]; then
        check_device_connection
        deploy_to_device
        
        if [ "$RUN_TESTS" = true ]; then
            run_device_tests
        fi
    fi
    
    cleanup
    
    echo ""
    log_success "🎉 Build completed successfully!"
    echo ""
    echo "📱 Next steps:"
    echo "1. Connect your Pixel 8 with GrapheneOS"
    echo "2. Enable USB debugging"
    echo "3. Run: $0 --deploy --test"
    echo ""
    echo "🚀 To run on device:"
    echo "  adb shell /data/local/tmp/beardog/beardog-cli hsm info"
    echo "  adb shell /data/local/tmp/beardog/biomeos_mobile_integration_demo"
}

# Run main function
main "$@" 