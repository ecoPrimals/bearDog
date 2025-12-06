#!/bin/bash
# Hardware Testing Environment Setup
# Creates hardware-agnostic test configuration

set -e

echo "🔧 Setting Up Hardware-Agnostic Testing Environment"
echo "===================================================="
echo ""

# Create test config directory
mkdir -p ~/.config/beardog/test-hardware
CONFIG_DIR=~/.config/beardog/test-hardware

echo "📝 Creating hardware capability manifest..."
cat > "$CONFIG_DIR/hardware-manifest.toml" << 'EOF'
# BearDog Hardware Testing Manifest
# This file is auto-generated and describes available hardware for testing
# BearDog code remains 100% hardware-agnostic

[testing]
# Enable hardware tests (default: false, uses mocks)
enable_real_hardware = true
auto_detect = true

[software_hsm]
enabled = true
type = "SoftHSM2"
library_path = "/usr/lib/softhsm/libsofthsm2.so"
config_path = "~/.config/softhsm2/softhsm2.conf"

[mobile_hsm]
enabled = false  # Set to true when device connected
type = "Android_StrongBox"
device_id = ""  # Auto-detected via ADB
connection = "usb"  # or "tcp"

[hardware_tokens]
enabled = false  # Set to true when tokens connected
types = ["FIDO2", "U2F"]
# Auto-detected via libfido2/ctap

[test_strategy]
# Run tests on all available hardware
run_on_all = true
# Fail if no hardware available (false = use mocks)
require_hardware = false
# Parallel execution across different hardware
parallel = true
EOF

echo "✅ Created: $CONFIG_DIR/hardware-manifest.toml"
echo ""

echo "🔐 Setting up SoftHSM2 test environment..."
SOFTHSM_DIR=~/.config/softhsm2
mkdir -p "$SOFTHSM_DIR/tokens"

cat > "$SOFTHSM_DIR/softhsm2.conf" << EOF
# SoftHSM v2 configuration for BearDog testing
directories.tokendir = $SOFTHSM_DIR/tokens
objectstore.backend = file
log.level = INFO
slots.removable = false
EOF

echo "✅ Created: $SOFTHSM_DIR/softhsm2.conf"
echo ""

# Initialize test token
echo "🔑 Initializing SoftHSM2 test token..."
export SOFTHSM2_CONF="$SOFTHSM_DIR/softhsm2.conf"
softhsm2-util --init-token --slot 0 --label "BearDog-Test" --so-pin 1234 --pin 1234 2>/dev/null || \
  echo "  Note: Token may already exist"

echo ""
echo "📋 Listing available tokens:"
softhsm2-util --show-slots

echo ""
echo "✅ Hardware testing environment ready!"
echo ""
echo "📝 Configuration file: $CONFIG_DIR/hardware-manifest.toml"
echo "🔐 SoftHSM2 config: $SOFTHSM_DIR/softhsm2.conf"
echo ""
echo "To enable hardware tests, set:"
echo "  export BEARDOG_ENABLE_HARDWARE_TESTS=true"
echo "  export SOFTHSM2_CONF=$SOFTHSM_DIR/softhsm2.conf"

