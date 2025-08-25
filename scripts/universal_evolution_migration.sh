#!/bin/bash
# Universal Evolution Migration Script
# Migrates hardcoded platform references to vendor-agnostic patterns

set -e

echo "🌍 Starting Universal Evolution Migration..."

# Backup current state
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
echo "📦 Creating backup in $BACKUP_DIR"
mkdir -p "$BACKUP_DIR"
cp -r crates/ "$BACKUP_DIR/"
cp -r tests/ "$BACKUP_DIR/"

echo "🔄 Migrating StrongBoxImplementation to HardwareSecurityImplementation..."

# Replace StrongBoxImplementation with HardwareSecurityImplementation
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/StrongBoxImplementation/HardwareSecurityImplementation/g' {} +

echo "🔄 Migrating Android-specific references to universal patterns..."

# Replace AndroidStrongBox with MobileHardware
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/AndroidStrongBox/MobileHardware/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/android_strongbox/mobile_hardware/g' {} +

echo "🔄 Migrating iOS-specific references to universal patterns..."

# Replace iOS-specific references
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/IosSecureEnclave/DesktopHardware/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/ios_secure_enclave/desktop_hardware/g' {} +

echo "🔄 Migrating vendor-specific enum variants..."

# Replace specific implementations with Universal
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/TitanM/Universal/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/SamsungKnox/Universal/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/AppleSecureEnclave/Universal/g' {} +

echo "🔄 Migrating hardcoded vendor names..."

# Replace hardcoded vendor strings
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Google"/"Universal Hardware Provider"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Samsung"/"Universal Hardware Provider"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Apple"/"Universal Hardware Provider"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Pixel 8a"/"Universal Mobile Device"/g' {} +

echo "🔄 Migrating platform-specific HSM interface types..."

# Update HSM interface types to be universal
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/HsmInterfaceType::AndroidStrongBox/HsmInterfaceType::MobileHardware/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/HsmInterfaceType::IosSecureEnclave/HsmInterfaceType::DesktopHardware/g' {} +

echo "🔄 Migrating SmartphoneType variants..."

# Update SmartphoneType to use Universal
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/SmartphoneType::Android/SmartphoneType::Universal/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/SmartphoneType::IPhone/SmartphoneType::Universal/g' {} +

echo "🔄 Migrating SecureEnclaveType variants..."

# Update SecureEnclaveType to use HardwareSecurity
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/SecureEnclaveType::AndroidStrongBox/SecureEnclaveType::HardwareSecurity/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/SecureEnclaveType::IosSecureEnclave/SecureEnclaveType::HardwareSecurity/g' {} +

echo "🔄 Cleaning up hardcoded device models..."

# Replace specific device models with universal patterns
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"iPhone 14 Pro"/"Universal Mobile Device"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Galaxy S23"/"Universal Mobile Device"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"Pixel 7"/"Universal Mobile Device"/g' {} +

echo "🔄 Migrating HSM provider types..."

# Update ProviderType usage
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/ProviderType::AndroidStrongBox/ProviderType::MobileHardware/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/ProviderType::IosSecureEnclave/ProviderType::DesktopHardware/g' {} +

echo "🔄 Finalizing universal patterns..."

# Replace any remaining platform-specific strings
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"`Android`"/"`Universal`"/g' {} +
find crates/ tests/ examples/ -name "*.rs" -type f -exec sed -i 's/"`iOS`"/"`Universal`"/g' {} +

echo "✅ Universal Evolution Migration Complete!"
echo "📊 Running compilation check..."

if cargo check --workspace --quiet; then
    echo "✅ Migration successful - codebase compiles cleanly"
    echo "📦 Backup available in: $BACKUP_DIR"
else
    echo "❌ Migration issues detected - check compilation errors"
    echo "🔄 Backup available for rollback in: $BACKUP_DIR"
    exit 1
fi

echo "🌍 BearDog is now UNIVERSALLY VENDOR-AGNOSTIC! 🎉" 