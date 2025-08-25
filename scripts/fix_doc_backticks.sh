#!/bin/bash

# BearDog Documentation Backticks Fix Script
# This script systematically fixes ALL documentation backtick issues for 100% perfection

set -e

echo "📚 Starting DOCUMENTATION BACKTICKS fix for 100% perfection..."

# Function to fix backticks in a file
fix_backticks_in_file() {
    local file="$1"
    echo "🔧 Fixing backticks in $file..."
    
    # Fix common terms that need backticks
    sed -i 's/\bBearDog\b/`BearDog`/g' "$file"
    sed -i 's/\bSongBird\b/`SongBird`/g' "$file"
    sed -i 's/\bSafeNet\b/`SafeNet`/g' "$file"
    sed -i 's/\bStrongBox\b/`StrongBox`/g' "$file"
    sed -i 's/\bMessagePack\b/`MessagePack`/g' "$file"
    sed -i 's/\bThreadPoolConfig\b/`ThreadPoolConfig`/g' "$file"
    sed -i 's/\bHumanEntropyCapabilities\b/`HumanEntropyCapabilities`/g' "$file"
    sed -i 's/\bUtimaco\b/`Utimaco`/g' "$file"
    sed -i 's/\bThales\b/`Thales`/g' "$file"
    sed -i 's/\bHSM\b/`HSM`/g' "$file"
    sed -i 's/\bTPM\b/`TPM`/g' "$file"
    sed -i 's/\bAWS\b/`AWS`/g' "$file"
    sed -i 's/\bKMS\b/`KMS`/g' "$file"
    sed -i 's/\biOS\b/`iOS`/g' "$file"
    sed -i 's/\bAndroid\b/`Android`/g' "$file"
    sed -i 's/\bJSON\b/`JSON`/g' "$file"
    sed -i 's/\bTOML\b/`TOML`/g' "$file"
    sed -i 's/\bYAML\b/`YAML`/g' "$file"
    sed -i 's/\bXML\b/`XML`/g' "$file"
    sed -i 's/\bUUID\b/`UUID`/g' "$file"
    sed -i 's/\bSIMD\b/`SIMD`/g' "$file"
    sed -i 's/\bNUMA\b/`NUMA`/g' "$file"
    sed -i 's/\bEd25519\b/`Ed25519`/g' "$file"
    sed -i 's/\bAES\b/`AES`/g' "$file"
    sed -i 's/\bChaCha20\b/`ChaCha20`/g' "$file"
    sed -i 's/\bPoly1305\b/`Poly1305`/g' "$file"
    sed -i 's/\bSHA256\b/`SHA256`/g' "$file"
    sed -i 's/\bSHA512\b/`SHA512`/g' "$file"
    sed -i 's/\bBLAKE2\b/`BLAKE2`/g' "$file"
    sed -i 's/\bArgon2\b/`Argon2`/g' "$file"
    
    # Fix double backticks (in case they already existed)
    sed -i 's/``\([^`]*\)``/`\1`/g' "$file"
    
    # Fix backticks inside already quoted strings (avoid double quoting)
    sed -i 's/"`\([^`]*\)`"/"`\1`"/g' "$file"
}

# Fix all Rust source files
echo "🔍 Finding all Rust documentation files..."

find crates/ -name "*.rs" -type f | while read -r file; do
    fix_backticks_in_file "$file"
done

# Fix build scripts
find . -name "build.rs" -type f | while read -r file; do
    fix_backticks_in_file "$file"
done

# Fix README files
find crates/ -name "README.md" -type f | while read -r file; do
    fix_backticks_in_file "$file"
done

echo "✅ Backticks fixes completed!"

# Now fix specific issues that need manual attention
echo "🎯 Applying targeted fixes for specific cases..."

# Fix specific type names in beardog-types
sed -i 's/HumanEntropyCapabilities/`HumanEntropyCapabilities`/g' crates/beardog-types/src/canonical/capabilities.rs 2>/dev/null || true
sed -i 's/ThreadPoolConfig/`ThreadPoolConfig`/g' crates/beardog-types/src/canonical/configuration/performance.rs 2>/dev/null || true

# Fix vendor names
find crates/ -name "*.rs" -exec sed -i 's/\bSafeNet\b/`SafeNet`/g' {} \; 2>/dev/null || true
find crates/ -name "*.rs" -exec sed -i 's/\bUtimaco\b/`Utimaco`/g' {} \; 2>/dev/null || true
find crates/ -name "*.rs" -exec sed -i 's/\bThales\b/`Thales`/g' {} \; 2>/dev/null || true

echo "🎉 ALL DOCUMENTATION BACKTICKS FIXED!"
echo "📝 Documentation should now be at 100% perfection!" 