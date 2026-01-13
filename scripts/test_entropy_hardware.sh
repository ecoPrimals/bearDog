#!/usr/bin/env bash
# Hardware Entropy Testing Helper Script
# 
# Quick launcher for testing entropy across your hardware:
# - SoloKey FIDO2
# - Pixel 8a Titan M
# - Software HSM
# - Human interaction

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║    🎲 BearDog Hardware Entropy Testing Suite                  ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check for SoloKey
check_solokey() {
    echo "🔍 Checking for SoloKey..."
    if lsusb | grep -qi "solo"; then
        echo "   ✅ SoloKey detected"
        SOLOKEY_FOUND=1
    else
        echo "   ⚠️  SoloKey not found (insert device or skip FIDO2 tests)"
        SOLOKEY_FOUND=0
    fi
    echo ""
}

# Check for Pixel 8a via ADB
check_pixel() {
    echo "🔍 Checking for Pixel 8a..."
    if command -v adb &> /dev/null; then
        if adb devices | grep -q "44251JEKB04957\|device"; then
            echo "   ✅ Pixel 8a connected via ADB"
            PIXEL_FOUND=1
        else
            echo "   ⚠️  Pixel 8a not connected (connect via USB or skip Android tests)"
            PIXEL_FOUND=0
        fi
    else
        echo "   ⚠️  ADB not installed (skip Android tests)"
        PIXEL_FOUND=0
    fi
    echo ""
}

# Test 1: Software HSM Baseline
test_software() {
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║  Test 1: Software HSM Baseline                                 ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Testing: RustCrypto CSPRNG (baseline performance)"
    echo ""
    
    cargo run --example entropy_hardware_comparison --quiet
}

# Test 2: SoloKey FIDO2
test_solokey() {
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║  Test 2: SoloKey FIDO2 Hardware                                ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    
    if [ "$SOLOKEY_FOUND" -eq 1 ]; then
        echo "Testing: SoloKey hardware entropy generation"
        echo ""
        cargo run --example entropy_hardware_comparison --features fido2 --quiet
    else
        echo "⏭️  Skipped (SoloKey not detected)"
        echo "   To enable: Insert SoloKey and re-run"
    fi
    echo ""
}

# Test 3: Pixel 8a Titan M
test_pixel() {
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║  Test 3: Pixel 8a Titan M (StrongBox)                         ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    
    if [ "$PIXEL_FOUND" -eq 1 ]; then
        echo "Testing: Pixel 8a Titan M hardware entropy"
        echo ""
        
        # Check if already built
        if [ ! -f "target/aarch64-linux-android/release/examples/entropy_hardware_comparison" ]; then
            echo "📦 Building for Android..."
            cargo ndk -t aarch64-linux-android build --example entropy_hardware_comparison --release
        fi
        
        echo "📱 Deploying to Pixel 8a..."
        adb push target/aarch64-linux-android/release/examples/entropy_hardware_comparison \
            /data/local/tmp/entropy_test 2>&1 | grep -v "bytes/s" || true
        
        adb shell chmod +x /data/local/tmp/entropy_test
        
        echo "🚀 Running on device..."
        echo ""
        adb shell /data/local/tmp/entropy_test
    else
        echo "⏭️  Skipped (Pixel 8a not connected)"
        echo "   To enable:"
        echo "   1. Connect Pixel 8a via USB"
        echo "   2. Enable USB debugging"
        echo "   3. Run: adb devices"
        echo "   4. Accept prompt on phone"
    fi
    echo ""
}

# Main menu
show_menu() {
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║  Select Test Mode:                                             ║"
    echo "╠════════════════════════════════════════════════════════════════╣"
    echo "║  1. Quick test (software HSM only)                             ║"
    echo "║  2. Full test (all available hardware)                         ║"
    echo "║  3. SoloKey only                                               ║"
    echo "║  4. Pixel 8a only                                              ║"
    echo "║  5. Interactive (with human entropy)                           ║"
    echo "║  6. Exit                                                       ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    read -p "Enter choice [1-6]: " choice
    
    case $choice in
        1)
            echo ""
            test_software
            ;;
        2)
            echo ""
            test_software
            test_solokey
            test_pixel
            ;;
        3)
            echo ""
            test_solokey
            ;;
        4)
            echo ""
            test_pixel
            ;;
        5)
            echo ""
            echo "╔════════════════════════════════════════════════════════════════╗"
            echo "║  Interactive Mode: Human Entropy Capture                       ║"
            echo "╚════════════════════════════════════════════════════════════════╝"
            echo ""
            echo "You will be prompted to:"
            echo "  • Type on keyboard naturally"
            echo "  • Move mouse around"
            echo "  • Click in different places"
            echo ""
            echo "Target: 30 interactions (takes ~30-60 seconds)"
            echo ""
            read -p "Press ENTER to start..."
            echo ""
            cargo run --example entropy_hardware_comparison
            ;;
        6)
            echo "👋 Goodbye!"
            exit 0
            ;;
        *)
            echo "❌ Invalid choice. Please try again."
            echo ""
            show_menu
            ;;
    esac
}

# Main execution
main() {
    # Hardware detection
    check_solokey
    check_pixel
    
    # Show menu
    show_menu
    
    echo ""
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║  ✅ Testing Complete!                                          ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "📊 Results Summary:"
    echo "   - Check statistical quality (Shannon entropy, Chi-square)"
    echo "   - Compare generation speeds"
    echo "   - Review quality scores"
    echo ""
    echo "📚 For more details, see:"
    echo "   docs/testing-guides/HARDWARE_ENTROPY_TESTING_GUIDE.md"
    echo ""
}

# Run main
main

