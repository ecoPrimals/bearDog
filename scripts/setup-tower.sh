#!/bin/bash
# BearDog Tower Setup Script
# Automates installation of all dependencies for a fresh tower
# Version: 1.0.0
# Date: November 1, 2025

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }

echo "🐻 BearDog Tower Setup"
echo "======================"
echo ""
print_info "This script will install all dependencies needed for BearDog"
echo ""

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    print_info "Detected platform: Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    print_info "Detected platform: macOS"
else
    print_error "Unsupported platform: $OSTYPE"
    exit 1
fi

# Check if running as root (don't want to!)
if [[ $EUID -eq 0 ]]; then
   print_error "This script should NOT be run as root"
   print_info "Run as normal user (script will use sudo when needed)"
   exit 1
fi

echo ""
print_info "Installing system dependencies..."
echo ""

if [[ "$PLATFORM" == "linux" ]]; then
    # Linux (Debian/Ubuntu)
    
    # Check if we have sudo
    if ! command -v sudo &> /dev/null; then
        print_error "sudo not found. Please install sudo first."
        exit 1
    fi
    
    # Update package list
    print_info "Updating package list..."
    sudo apt update
    
    # Install PC/SC smart card infrastructure
    print_info "Installing PC/SC smart card support..."
    sudo apt install -y pcscd pcsc-tools libccid
    
    # Install PKCS#11 libraries
    print_info "Installing PKCS#11 libraries..."
    sudo apt install -y opensc opensc-pkcs11
    
    # Install hardware token tools
    print_info "Installing hardware token tools..."
    sudo apt install -y yubico-piv-tool
    
    # Install SoftHSM for testing/development
    print_info "Installing SoftHSM (software HSM)..."
    sudo apt install -y softhsm2
    
    # Start and enable PC/SC daemon
    print_info "Starting PC/SC daemon..."
    sudo systemctl start pcscd
    sudo systemctl enable pcscd
    
    print_success "System packages installed!"
    
elif [[ "$PLATFORM" == "macos" ]]; then
    # macOS
    
    # Check if we have brew
    if ! command -v brew &> /dev/null; then
        print_error "Homebrew not found. Please install it first:"
        echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
        exit 1
    fi
    
    # Install PKCS#11 libraries
    print_info "Installing PKCS#11 libraries..."
    brew install opensc
    
    # Install hardware token tools
    print_info "Installing hardware token tools..."
    brew install yubico-piv-tool
    
    # Install SoftHSM
    print_info "Installing SoftHSM..."
    brew install softhsm
    
    # Note: PC/SC is built into macOS
    print_success "System packages installed!"
    print_info "Note: PC/SC (pcscd) is built into macOS"
fi

echo ""
print_info "Configuring SoftHSM..."
echo ""

# Create SoftHSM configuration directory
SOFTHSM_DIR="$HOME/.config/softhsm2"
SOFTHSM_TOKENS="$SOFTHSM_DIR/tokens"
SOFTHSM_CONF="$SOFTHSM_DIR/softhsm2.conf"

mkdir -p "$SOFTHSM_TOKENS"

# Create SoftHSM configuration
cat > "$SOFTHSM_CONF" << EOF
# SoftHSM v2 Configuration for BearDog
directories.tokendir = $SOFTHSM_TOKENS
objectstore.backend = file
log.level = INFO
EOF

print_success "Created SoftHSM configuration at: $SOFTHSM_CONF"

# Set environment variable
export SOFTHSM2_CONF="$SOFTHSM_CONF"

# Add to shell profile (if not already there)
SHELL_RC="$HOME/.bashrc"
if [[ "$PLATFORM" == "macos" ]]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q "SOFTHSM2_CONF" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# BearDog SoftHSM Configuration" >> "$SHELL_RC"
    echo "export SOFTHSM2_CONF=\"$SOFTHSM_CONF\"" >> "$SHELL_RC"
    print_success "Added SOFTHSM2_CONF to $SHELL_RC"
fi

echo ""
print_info "Initializing default SoftHSM token..."
echo ""

# Initialize a default token for testing
softhsm2-util --init-token --free --label "BearDog-Default" \
  --so-pin 12345678 --pin 87654321 2>&1

print_success "Default SoftHSM token initialized!"
print_info "Label: BearDog-Default"
print_info "SO-PIN: 12345678"
print_info "User PIN: 87654321"

echo ""
print_info "Verifying installation..."
echo ""

# Check if pcscd is running (Linux only)
if [[ "$PLATFORM" == "linux" ]]; then
    if systemctl is-active --quiet pcscd; then
        print_success "PC/SC daemon is running"
    else
        print_warning "PC/SC daemon not running"
    fi
fi

# Check SoftHSM
if softhsm2-util --show-slots > /dev/null 2>&1; then
    print_success "SoftHSM is working"
else
    print_warning "SoftHSM may have issues"
fi

# List available PKCS#11 libraries
echo ""
print_info "Available PKCS#11 libraries:"
echo ""

if [[ "$PLATFORM" == "linux" ]]; then
    if [ -f "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so" ]; then
        echo "  ✅ OpenSC: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"
    fi
    if [ -f "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so" ]; then
        echo "  ✅ SoftHSM: /usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so"
    fi
elif [[ "$PLATFORM" == "macos" ]]; then
    if [ -f "/usr/local/lib/opensc-pkcs11.so" ]; then
        echo "  ✅ OpenSC: /usr/local/lib/opensc-pkcs11.so"
    fi
    if [ -f "/usr/local/lib/softhsm/libsofthsm2.so" ]; then
        echo "  ✅ SoftHSM: /usr/local/lib/softhsm/libsofthsm2.so"
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_success "🎉 Tower setup complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
print_info "Next steps:"
echo ""
echo "  1. Build BearDog:"
echo "     cargo build --release -p beardog-cli"
echo ""
echo "  2. Test BearDog CLI:"
echo "     ./target/release/beardog status"
echo ""
echo "  3. Discover HSM devices:"
echo "     ./target/release/beardog discover-hsm"
echo ""
echo "  Or with SoftHSM explicitly:"
echo "     ./target/release/beardog discover-hsm --library /usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so"
echo ""
print_info "Plug in your hardware tokens (SoloKey, YubiKey, etc.) and run discover-hsm!"
echo ""
print_success "🐻 Happy sovereign computing! 🔐"
echo ""

