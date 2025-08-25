#!/bin/bash

# BearDog Configuration Modernization Script
# Systematically replaces deprecated config imports with canonical ones

set -e

echo "🔄 Starting BearDog configuration modernization..."

# Define the root directory
ROOT_DIR="/home/eastgate/Development/ecoPrimals/beardog"
cd "$ROOT_DIR"

echo "📂 Working in: $ROOT_DIR"

# Function to replace imports in files
replace_imports() {
    local pattern="$1"
    local replacement="$2"
    local description="$3"
    
    echo "🔍 Replacing: $description"
    echo "   Pattern: $pattern"
    echo "   Replacement: $replacement"
    
    # Find and replace in .rs files
    find . -name "*.rs" -type f -exec grep -l "$pattern" {} \; | while read -r file; do
        echo "   📝 Updating: $file"
        sed -i "s|$pattern|$replacement|g" "$file"
    done
    
    echo "   ✅ Completed: $description"
    echo ""
}

# Core configuration imports
replace_imports \
    "beardog_types::config::" \
    "beardog_types::canonical::configuration::" \
    "Core config module path"

# Specific config types
replace_imports \
    "beardog_types::config::app::" \
    "beardog_types::canonical::configuration::" \
    "App config types"

replace_imports \
    "beardog_types::config::api::" \
    "beardog_types::canonical::configuration::" \
    "API config types"

replace_imports \
    "beardog_types::config::audit::" \
    "beardog_types::canonical::configuration::" \
    "Audit config types"

replace_imports \
    "beardog_types::config::security::" \
    "beardog_types::canonical::configuration::" \
    "Security config types"

replace_imports \
    "beardog_types::config::network::" \
    "beardog_types::canonical::configuration::" \
    "Network config types"

replace_imports \
    "beardog_types::config::performance::" \
    "beardog_types::canonical::configuration::" \
    "Performance config types"

replace_imports \
    "beardog_types::config::production::" \
    "beardog_types::canonical::configuration::" \
    "Production config types"

replace_imports \
    "beardog_types::config::monitoring::" \
    "beardog_types::canonical::monitoring::" \
    "Monitoring config types"

replace_imports \
    "beardog_types::config::tunnel::" \
    "beardog_types::canonical::configuration::" \
    "Tunnel config types"

replace_imports \
    "beardog_types::config::discovery::" \
    "beardog_types::canonical::configuration::" \
    "Discovery config types"

replace_imports \
    "beardog_types::config::integration::" \
    "beardog_types::canonical::configuration::" \
    "Integration config types"

replace_imports \
    "beardog_types::config::compliance::" \
    "beardog_types::canonical::configuration::" \
    "Compliance config types"

replace_imports \
    "beardog_types::config::unified::" \
    "beardog_types::canonical::configuration::" \
    "Unified config types"

# Other deprecated modules
replace_imports \
    "beardog_types::services::" \
    "beardog_types::canonical::services::" \
    "Services module"

replace_imports \
    "beardog_types::monitoring::" \
    "beardog_types::canonical::monitoring::" \
    "Monitoring module"

replace_imports \
    "beardog_types::security::" \
    "beardog_types::canonical::security::" \
    "Security module"

replace_imports \
    "beardog_types::network::" \
    "beardog_types::canonical::network::" \
    "Network module"

replace_imports \
    "beardog_types::providers::" \
    "beardog_types::canonical::providers::" \
    "Providers module"

replace_imports \
    "beardog_types::crypto::" \
    "beardog_types::canonical::crypto::" \
    "Crypto module"

replace_imports \
    "beardog_types::hsm::" \
    "beardog_types::canonical::hsm::" \
    "HSM module"

replace_imports \
    "beardog_types::workflow::" \
    "beardog_types::canonical::workflow::" \
    "Workflow module"

replace_imports \
    "beardog_types::capabilities::" \
    "beardog_types::canonical::capabilities::" \
    "Capabilities module"

replace_imports \
    "beardog_types::health::" \
    "beardog_types::canonical::health_status::" \
    "Health module"

replace_imports \
    "beardog_types::metrics::" \
    "beardog_types::canonical::metrics::" \
    "Metrics module"

echo "🎉 Configuration modernization complete!"
echo ""
echo "📊 Summary:"
echo "   ✅ All deprecated config imports updated to canonical paths"
echo "   ✅ Module paths standardized"
echo "   ✅ Codebase ready for deprecated module removal"
echo ""
echo "🔍 Next steps:"
echo "   1. Run 'cargo check' to verify compilation"
echo "   2. Remove deprecated modules from beardog-types"
echo "   3. Update Cargo.toml dependencies if needed" 