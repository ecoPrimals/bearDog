#!/bin/bash
# BearDog ARM64 Android Build & Deploy Script
# For Pixel 8a deployment with isomorphic IPC

set -e  # Exit on error

echo "🐻🐕 BearDog ARM64 Android Build & Deploy"
echo "========================================="
echo ""

# Step 1: Clean previous build
echo "🧹 Step 1/5: Cleaning previous build..."
cargo clean
echo "✅ Clean complete"
echo ""

# Step 2: Build for ARM64 Android
echo "🔨 Step 2/5: Building for ARM64 Android..."
echo "   Target: aarch64-linux-android"
echo "   Profile: release"
echo "   Binary: beardog (beardog-tunnel)"
echo ""
cross build --target aarch64-linux-android --release --bin beardog -p beardog-tunnel
echo "✅ Build complete"
echo ""

# Step 3: Verify binary
echo "📊 Step 3/5: Verifying binary..."
BINARY_PATH="target/aarch64-linux-android/release/beardog"
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ ERROR: Binary not found at $BINARY_PATH"
    exit 1
fi
ls -lh "$BINARY_PATH"
echo "✅ Binary verified"
echo ""

# Step 4: Check ADB connection
echo "📱 Step 4/5: Checking ADB connection..."
if ! adb devices | grep -q "device$"; then
    echo "❌ ERROR: No Android device connected via ADB"
    echo "   Please connect Pixel 8a and enable USB debugging"
    exit 1
fi
echo "✅ Device connected"
echo ""

# Step 5: Deploy to Pixel
echo "🚀 Step 5/5: Deploying to Pixel..."
echo "   Pushing binary to /data/local/tmp/..."
adb push "$BINARY_PATH" /data/local/tmp/beardog
echo "   Setting executable permissions..."
adb shell "chmod +x /data/local/tmp/beardog"
echo "✅ Deploy complete"
echo ""

# Verify deployment
echo "📋 Verifying deployment..."
adb shell "ls -lh /data/local/tmp/beardog"
echo ""

# Display success message
echo "╔════════════════════════════════════════════════════════════════════╗"
echo "║                                                                    ║"
echo "║          ✅ BEARDOG DEPLOYED TO PIXEL 8A SUCCESSFULLY!            ║"
echo "║                                                                    ║"
echo "╚════════════════════════════════════════════════════════════════════╝"
echo ""
echo "📝 Next Steps:"
echo ""
echo "1. Set up environment on Pixel:"
echo "   adb shell \"cd /data/local/tmp && mkdir -p run\""
echo "   adb shell \"export FAMILY_ID=pixel_tower NODE_ID=pixel_node1 XDG_RUNTIME_DIR=/data/local/tmp/run\""
echo ""
echo "2. Start beardog server:"
echo "   adb shell \"cd /data/local/tmp && FAMILY_ID=pixel_tower NODE_ID=pixel_node1 XDG_RUNTIME_DIR=/data/local/tmp/run RUST_LOG=info ./beardog server > beardog.log 2>&1 &\""
echo ""
echo "3. Monitor logs:"
echo "   adb shell \"tail -f /data/local/tmp/beardog.log\""
echo ""
echo "4. Expected TCP fallback logs:"
echo "   - '⚠️  Unix sockets unavailable'"
echo "   - 'Detected platform constraint, adapting...'"
echo "   - 'Falling back to TCP IPC'"
echo "   - '✅ TCP server listening on 127.0.0.1:XXXXX'"
echo ""
echo "🎉 Isomorphic IPC will automatically adapt to Android constraints!"
echo ""
