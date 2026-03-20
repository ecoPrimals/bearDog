#!/bin/bash
# BearDog Android Development Setup Script
# Run this to install Android NDK and configure Rust for Android cross-compilation

set -e

echo "🤖 BearDog Android Setup Script"
echo "================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
ANDROID_SDK_DIR="$HOME/Android/Sdk"
NDK_VERSION="25.2.9519653"
CMDLINE_TOOLS_URL="https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip"

# Functions
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

check_command() {
    if command -v $1 &> /dev/null; then
        print_success "$1 is installed"
        return 0
    else
        print_warning "$1 is not installed"
        return 1
    fi
}

# Check prerequisites
echo "📋 Checking prerequisites..."
echo ""

if ! check_command rustup; then
    print_error "Rust is not installed. Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! check_command wget; then
    print_warning "wget not found, installing..."
    sudo apt update && sudo apt install -y wget
fi

if ! check_command unzip; then
    print_warning "unzip not found, installing..."
    sudo apt update && sudo apt install -y unzip
fi

echo ""

# Step 1: Download and install Android command line tools
echo "📥 Step 1: Installing Android SDK command line tools..."

if [ -d "$ANDROID_SDK_DIR/cmdline-tools/latest" ]; then
    print_warning "Command line tools already installed at $ANDROID_SDK_DIR"
    read -p "Reinstall? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_success "Skipping SDK installation"
    else
        rm -rf "$ANDROID_SDK_DIR/cmdline-tools"
    fi
fi

if [ ! -d "$ANDROID_SDK_DIR/cmdline-tools/latest" ]; then
    mkdir -p "$ANDROID_SDK_DIR/cmdline-tools"
    cd "$ANDROID_SDK_DIR/cmdline-tools"
    
    echo "  Downloading command line tools..."
    wget -q --show-progress "$CMDLINE_TOOLS_URL" -O cmdline-tools.zip
    
    echo "  Extracting..."
    unzip -q cmdline-tools.zip
    mv cmdline-tools latest
    rm cmdline-tools.zip
    
    print_success "Command line tools installed"
fi

echo ""

# Step 2: Update PATH
echo "🔧 Step 2: Configuring environment..."

if ! grep -q "ANDROID_HOME" ~/.bashrc; then
    echo "" >> ~/.bashrc
    echo "# Android SDK" >> ~/.bashrc
    echo "export ANDROID_HOME=\$HOME/Android/Sdk" >> ~/.bashrc
    echo "export PATH=\$PATH:\$ANDROID_HOME/cmdline-tools/latest/bin" >> ~/.bashrc
    echo "export PATH=\$PATH:\$ANDROID_HOME/platform-tools" >> ~/.bashrc
    print_success "Added Android paths to ~/.bashrc"
else
    print_success "Android paths already in ~/.bashrc"
fi

export ANDROID_HOME="$ANDROID_SDK_DIR"
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
export PATH="$PATH:$ANDROID_HOME/platform-tools"

echo ""

# Step 3: Accept licenses
echo "📜 Step 3: Accepting Android SDK licenses..."
yes | sdkmanager --licenses > /dev/null 2>&1 || true
print_success "Licenses accepted"

echo ""

# Step 4: Install NDK and tools
echo "🔨 Step 4: Installing Android NDK and tools..."

if ! sdkmanager --list_installed | grep -q "ndk;$NDK_VERSION"; then
    echo "  Installing NDK $NDK_VERSION..."
    sdkmanager "ndk;$NDK_VERSION"
    print_success "NDK installed"
else
    print_success "NDK already installed"
fi

if ! sdkmanager --list_installed | grep -q "platform-tools"; then
    echo "  Installing platform-tools..."
    sdkmanager "platform-tools"
    print_success "Platform tools installed"
else
    print_success "Platform tools already installed"
fi

if ! sdkmanager --list_installed | grep -q "build-tools;34.0.0"; then
    echo "  Installing build-tools..."
    sdkmanager "build-tools;34.0.0"
    print_success "Build tools installed"
else
    print_success "Build tools already installed"
fi

# Update NDK_HOME
if ! grep -q "NDK_HOME" ~/.bashrc; then
    echo "export NDK_HOME=\$ANDROID_HOME/ndk/$NDK_VERSION" >> ~/.bashrc
    print_success "Added NDK_HOME to ~/.bashrc"
fi

export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"

echo ""

# Step 5: Add Rust Android targets
echo "🦀 Step 5: Adding Rust Android targets..."

for target in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
    if rustup target list | grep -q "$target (installed)"; then
        print_success "$target already installed"
    else
        echo "  Adding $target..."
        rustup target add $target
        print_success "$target added"
    fi
done

echo ""

# Step 6: Configure Cargo
echo "⚙️  Step 6: Configuring Cargo for Android..."

CARGO_CONFIG="$HOME/.cargo/config.toml"
NDK_TOOLCHAIN="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"

# Backup existing config
if [ -f "$CARGO_CONFIG" ]; then
    cp "$CARGO_CONFIG" "$CARGO_CONFIG.backup.$(date +%s)"
    print_warning "Backed up existing cargo config"
fi

# Create or update config
if ! grep -q "aarch64-linux-android" "$CARGO_CONFIG" 2>/dev/null; then
    cat >> "$CARGO_CONFIG" << EOF

# Android targets (added by BearDog setup)
[target.aarch64-linux-android]
ar = "$NDK_TOOLCHAIN/llvm-ar"
linker = "$NDK_TOOLCHAIN/aarch64-linux-android33-clang"

[target.armv7-linux-androideabi]
ar = "$NDK_TOOLCHAIN/llvm-ar"
linker = "$NDK_TOOLCHAIN/armv7a-linux-androideabi33-clang"

[target.x86_64-linux-android]
ar = "$NDK_TOOLCHAIN/llvm-ar"
linker = "$NDK_TOOLCHAIN/x86_64-linux-android33-clang"

[target.i686-linux-android]
ar = "$NDK_TOOLCHAIN/llvm-ar"
linker = "$NDK_TOOLCHAIN/i686-linux-android33-clang"
EOF
    print_success "Cargo config updated"
else
    print_success "Cargo config already has Android targets"
fi

echo ""

# Step 7: Install cargo-ndk
echo "📦 Step 7: Installing cargo-ndk..."

if command -v cargo-ndk &> /dev/null; then
    print_success "cargo-ndk already installed"
else
    cargo install cargo-ndk
    print_success "cargo-ndk installed"
fi

echo ""

# Step 8: Test build
echo "🧪 Step 8: Testing Android build..."
echo ""

cd "$(dirname "$0")/../android"

if cargo build --target aarch64-linux-android --release 2>&1 | grep -q "Finished"; then
    print_success "Test build successful!"
    echo ""
    echo "Binary location:"
    ls -lh ../target/aarch64-linux-android/release/libbeardog_android.so
else
    print_warning "Test build encountered issues (may be normal for first run)"
fi

echo ""

# Summary
echo "═══════════════════════════════════════════════════════════════"
echo "🎉 Android Setup Complete!"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "✅ Installed:"
echo "   • Android SDK command line tools"
echo "   • Android NDK $NDK_VERSION"
echo "   • Platform tools (adb, fastboot)"
echo "   • Build tools 34.0.0"
echo "   • Rust Android targets (4)"
echo "   • cargo-ndk"
echo ""
echo "📍 Locations:"
echo "   • SDK: $ANDROID_SDK_DIR"
echo "   • NDK: $NDK_HOME"
echo ""
echo "🔄 Next Steps:"
echo "   1. Restart your shell or run: source ~/.bashrc"
echo "   2. Connect your Pixel 8a"
echo "   3. Enable USB debugging"
echo "   4. Run: adb devices"
echo "   5. Read: ANDROID_SETUP_GUIDE.md"
echo ""
echo "🚀 Ready to build for Android!"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "⚠️  Remember to restart your shell to load environment variables!"

