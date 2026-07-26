#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# build-ipa.sh — Build bearDog for iOS and package as IPA
#
# Usage:
#   ./ios/build-ipa.sh [--release]
#
# Prerequisites:
#   - cargo-cross installed (cargo install cargo-cross)
#   - aarch64-apple-ios target installed (rustup target add aarch64-apple-ios)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
IOS_DIR="$SCRIPT_DIR/beardog-ios"
BUNDLE_ID="org.ecoprimals.beardog"
APP_NAME="BearDog"

PROFILE="debug"
CARGO_PROFILE_FLAG=""
if [[ "${1:-}" == "--release" ]]; then
    PROFILE="release"
    CARGO_PROFILE_FLAG="--release"
fi

TARGET="aarch64-apple-ios"
TARGET_DIR="$PROJECT_ROOT/target/$TARGET/$PROFILE"
BUILD_DIR="$PROJECT_ROOT/target/ios-build"
APP_DIR="$BUILD_DIR/Payload/$APP_NAME.app"
IPA_PATH="$BUILD_DIR/beardog.ipa"

echo "=== bearDog iOS Build ==="
echo "Profile: $PROFILE"
echo "Target:  $TARGET"
echo ""

echo "[1/4] Cross-compiling for $TARGET..."
cd "$PROJECT_ROOT"
cargo cross build --target "$TARGET" $CARGO_PROFILE_FLAG

echo "[2/4] Assembling .app bundle..."
rm -rf "$BUILD_DIR"
mkdir -p "$APP_DIR"

cp "$TARGET_DIR/beardog" "$APP_DIR/beardog"
cp "$IOS_DIR/Info.plist" "$APP_DIR/Info.plist"

chmod +x "$APP_DIR/beardog"

echo "[3/4] Packaging IPA..."
cd "$BUILD_DIR"
zip -r -q "$IPA_PATH" Payload/

SIGNED_IPA="$BUILD_DIR/beardog-signed.ipa"

# Sign if certificate and profile are available
CERT_PEM="${BEARDOG_IOS_CERT:-}"
PROVISION="${BEARDOG_IOS_PROVISION:-}"
CERT_KEY="${BEARDOG_IOS_KEY:-}"

if [[ -n "$CERT_PEM" && -n "$PROVISION" && -n "$CERT_KEY" ]]; then
    echo "[4/5] Signing IPA with Apple Developer certificate..."
    zsign -k "$CERT_KEY" -c "$CERT_PEM" -m "$PROVISION" \
          -b "$BUNDLE_ID" -e "$IOS_DIR/Entitlements.plist" \
          -o "$SIGNED_IPA" "$IPA_PATH"
    echo "[5/5] Installing to device..."
    ideviceinstaller --install "$SIGNED_IPA"
    echo ""
    echo "Signed IPA: $SIGNED_IPA"
else
    echo "[4/4] Ad-hoc signing (won't install on device)..."
    zsign -a -f -b "$BUNDLE_ID" -e "$IOS_DIR/Entitlements.plist" \
          -o "$SIGNED_IPA" "$IPA_PATH" 2>/dev/null || true
    echo ""
    echo "Unsigned IPA: $IPA_PATH"
    echo ""
    echo "To sign with Apple Developer certificate, set:"
    echo "  export BEARDOG_IOS_KEY=/path/to/private-key.pem"
    echo "  export BEARDOG_IOS_CERT=/path/to/certificate.pem"
    echo "  export BEARDOG_IOS_PROVISION=/path/to/profile.mobileprovision"
    echo ""
    echo "Then re-run this script, or sign manually:"
    echo "  zsign -k \$BEARDOG_IOS_KEY -c \$BEARDOG_IOS_CERT -m \$BEARDOG_IOS_PROVISION \\"
    echo "        -b $BUNDLE_ID -e $IOS_DIR/Entitlements.plist \\"
    echo "        -o $SIGNED_IPA $IPA_PATH"
    echo "  ideviceinstaller --install $SIGNED_IPA"
fi

echo ""
echo "Binary size: $(du -sh "$APP_DIR/beardog" | cut -f1)"
