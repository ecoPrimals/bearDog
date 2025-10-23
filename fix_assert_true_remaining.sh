#!/bin/bash
# Script to fix remaining assert!(true) statements
# Created: October 20, 2025

echo "🔧 Fixing remaining assert!(true) statements..."
echo ""

# Function to replace assert!(true) with TODO comment
fix_file() {
    local file=$1
    echo "Processing: $file"
    
    # Create backup
    cp "$file" "$file.backup"
    
    # Replace assert!(true, "message") with // TODO: message
    sed -i 's/assert!(true, "\([^"]*\)");/\/\/ TODO: \1/' "$file"
    
    # Replace standalone assert!(true) with // TODO: Add real test
    sed -i 's/assert!(true);/\/\/ TODO: Add real test/' "$file"
    
    echo "  ✅ Fixed"
}

# Files with remaining issues (from audit)
FILES=(
    "crates/beardog-core/src/core/tests/initialization_comprehensive_tests.rs"
    "crates/beardog-core/src/zero_knowledge_bootstrap/tests/discovery_comprehensive_tests.rs"
    "crates/beardog-core/src/core/tests/health_monitoring_tests.rs"
    "crates/beardog-tunnel/src/tunnel/hsm/safe_ffi/mod.rs"
    "crates/beardog-genetics/src/genetics/entropy_hierarchy/mod.rs"
    "crates/beardog-tunnel/src/tests/hsm_provider_selection_tests.rs"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        fix_file "$file"
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo ""
echo "✅ All fixes complete!"
echo ""
echo "📊 Verifying with clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1 | grep -E "error|warning" | head -20

echo ""
echo "🧪 Running tests..."
cargo test --workspace --lib 2>&1 | tail -5

echo ""
echo "💾 Backups saved as *.backup (delete after verifying)"
echo "🎯 Next: Review changes and commit"

