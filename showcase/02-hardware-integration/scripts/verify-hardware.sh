#!/usr/bin/env bash
#
# Hardware Verification Script for Phase 2
# Detects Solo V2 keys, Pixel 8a, and Software HSM

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🔍 BearDog Hardware Detection${NC}"
echo -e "${CYAN}==============================${NC}\n"

# Track overall status
ALL_READY=true

#==============================================================================
# Check Solo V2 Keys
#==============================================================================

echo -e "${BLUE}🔑 Solo V2 Keys:${NC}"

if command -v lsusb &> /dev/null; then
    SOLO_COUNT=$(lsusb | grep -i "solo\|1050:0407" | wc -l)
    
    if [ "$SOLO_COUNT" -eq 2 ]; then
        echo -e "  ${GREEN}✅ Found 2 Solo V2 keys${NC}"
        lsusb | grep -i "solo\|1050:0407" | sed 's/^/  ├─ /'
    elif [ "$SOLO_COUNT" -eq 1 ]; then
        echo -e "  ${YELLOW}⚠️  Found 1 Solo V2 key (expected 2)${NC}"
        lsusb | grep -i "solo\|1050:0407" | sed 's/^/  ├─ /'
        ALL_READY=false
    else
        echo -e "  ${YELLOW}⏳ No Solo V2 keys detected yet${NC}"
        echo "  └─ Check: Are they plugged in?"
        ALL_READY=false
    fi
    
    # Try solo2 tool if available
    if command -v solo2 &> /dev/null; then
        echo "  └─ Running: solo2 ls"
        solo2 ls 2>/dev/null | sed 's/^/     /' || echo "     (solo2 tool check failed)"
    fi
else
    echo -e "  ${YELLOW}⚠️  lsusb not available (install usbutils)${NC}"
fi

#==============================================================================
# Check Pixel 8a
#==============================================================================

echo ""
echo -e "${BLUE}📱 Pixel 8a:${NC}"

if command -v adb &> /dev/null; then
    # Check if any device is connected
    ADB_DEVICES=$(adb devices 2>/dev/null | grep -v "List" | grep "device$" | wc -l)
    
    if [ "$ADB_DEVICES" -gt 0 ]; then
        echo -e "  ${GREEN}✅ Device connected via ADB${NC}"
        
        # Get device info
        DEVICE_ID=$(adb devices | grep "device$" | awk '{print $1}' | head -1)
        echo "  ├─ Device ID: $DEVICE_ID"
        
        # Check device model
        MODEL=$(adb shell getprop ro.product.model 2>/dev/null | tr -d '\r')
        if [ -n "$MODEL" ]; then
            echo "  ├─ Model: $MODEL"
        fi
        
        # Check for StrongBox support
        KEYSTORE=$(adb shell getprop ro.hardware.keystore 2>/dev/null | tr -d '\r')
        if [ "$KEYSTORE" = "trusty" ]; then
            echo -e "  ├─ StrongBox: ${GREEN}✅ Available (Trusty TEE)${NC}"
        else
            echo -e "  ├─ StrongBox: ${YELLOW}⚠️  Status unclear (keystore: $KEYSTORE)${NC}"
        fi
        
        # Check Android version
        ANDROID_VER=$(adb shell getprop ro.build.version.release 2>/dev/null | tr -d '\r')
        if [ -n "$ANDROID_VER" ]; then
            echo "  ├─ Android: $ANDROID_VER"
        fi
        
        # Check if it's GrapheneOS
        BUILD_ID=$(adb shell getprop ro.build.id 2>/dev/null | tr -d '\r')
        if echo "$BUILD_ID" | grep -qi "graphene"; then
            echo -e "  └─ OS: ${GREEN}GrapheneOS ✅${NC}"
        else
            echo "  └─ OS: $BUILD_ID"
        fi
    else
        echo -e "  ${YELLOW}⏳ No device detected via ADB${NC}"
        echo "  └─ Status: May still be booting or USB debugging not enabled"
        echo ""
        echo "  ${CYAN}ℹ️  If Pixel is connected but not detected:${NC}"
        echo "     1. Ensure USB debugging is enabled in Developer Options"
        echo "     2. Accept the USB debugging prompt on phone"
        echo "     3. Try: adb kill-server && adb start-server"
    fi
else
    echo -e "  ${YELLOW}⚠️  ADB not installed${NC}"
    echo "  └─ Install: sudo apt install android-tools-adb"
    ALL_READY=false
fi

#==============================================================================
# Check Software HSM
#==============================================================================

echo ""
echo -e "${BLUE}💻 Software HSM:${NC}"

if command -v softhsm2-util &> /dev/null; then
    echo -e "  ${GREEN}✅ SoftHSM2 installed${NC}"
    
    # Show slots
    SLOT_INFO=$(softhsm2-util --show-slots 2>/dev/null || echo "")
    if [ -n "$SLOT_INFO" ]; then
        TOKEN_COUNT=$(echo "$SLOT_INFO" | grep -c "Slot " || echo "0")
        echo "  ├─ Configured slots: $TOKEN_COUNT"
        
        # Show first token if exists
        if [ "$TOKEN_COUNT" -gt 0 ]; then
            FIRST_TOKEN=$(echo "$SLOT_INFO" | grep "Label:" | head -1 | sed 's/.*Label: *//')
            echo "  └─ Example token: $FIRST_TOKEN"
        fi
    else
        echo "  └─ No tokens configured yet (will be created in demo)"
    fi
else
    echo -e "  ${RED}❌ SoftHSM2 not installed${NC}"
    echo "  └─ Install: sudo apt install softhsm2"
    ALL_READY=false
fi

#==============================================================================
# Check Optional Tools
#==============================================================================

echo ""
echo -e "${BLUE}🛠️  Optional Tools:${NC}"

# Check for solo2 CLI
if command -v solo2 &> /dev/null; then
    echo -e "  ${GREEN}✅ solo2 CLI available${NC}"
else
    echo -e "  ${YELLOW}⚠️  solo2 CLI not found (optional but recommended)${NC}"
    echo "  └─ Install: cargo install solo2-cli"
fi

# Check for openssl
if command -v openssl &> /dev/null; then
    OPENSSL_VER=$(openssl version | awk '{print $2}')
    echo -e "  ${GREEN}✅ OpenSSL $OPENSSL_VER${NC}"
else
    echo -e "  ${YELLOW}⚠️  OpenSSL not found${NC}"
fi

#==============================================================================
# Summary
#==============================================================================

echo ""
echo -e "${CYAN}═══════════════════════════════${NC}"
echo ""

if [ "$ALL_READY" = true ] && [ "$ADB_DEVICES" -gt 0 ]; then
    echo -e "${GREEN}✅ ALL HARDWARE READY FOR PHASE 2!${NC}"
    echo ""
    echo "You can now run:"
    echo "  ${CYAN}./demo-comparison.sh${NC}      # Full HSM comparison"
    echo "  ${CYAN}./demo-solo-v2.sh${NC}         # Solo keys only"
    echo "  ${CYAN}./demo-strongbox.sh${NC}       # Pixel StrongBox only"
    echo "  ${CYAN}./demo-human-entropy.sh${NC}   # Entropy collection"
elif [ "$ADB_DEVICES" -eq 0 ]; then
    echo -e "${YELLOW}⏳ ALMOST READY - Waiting for Pixel${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Wait for Pixel to finish booting"
    echo "  2. Enable USB debugging if not already"
    echo "  3. Run this script again: ./scripts/verify-hardware.sh"
    echo ""
    echo "Once Pixel is ready, you can run:"
    echo "  ${CYAN}./demo-comparison.sh${NC}"
else
    echo -e "${YELLOW}⚠️  SOME COMPONENTS MISSING${NC}"
    echo ""
    echo "Action items:"
    [ "$SOLO_COUNT" -ne 2 ] && echo "  - Connect both Solo V2 keys"
    ! command -v softhsm2-util &> /dev/null && echo "  - Install SoftHSM2: sudo apt install softhsm2"
    ! command -v adb &> /dev/null && echo "  - Install ADB: sudo apt install android-tools-adb"
    echo ""
    echo "You can still run some demos:"
    [ "$SOLO_COUNT" -gt 0 ] && echo "  ${CYAN}./demo-solo-v2.sh${NC}     # If Solo keys detected"
    command -v softhsm2-util &> /dev/null && echo "  ${CYAN}./demo-software.sh${NC}    # Software HSM demo"
fi

echo ""
echo -e "${CYAN}════════════════════════════════════════${NC}"

# Return appropriate exit code
if [ "$ALL_READY" = true ] && [ "$ADB_DEVICES" -gt 0 ]; then
    exit 0
else
    exit 1
fi

