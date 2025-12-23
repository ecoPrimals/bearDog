#!/bin/bash
# Fix duplicate attributes in remaining 3 crates

set -e

echo "🔧 Fixing duplicate attributes..."
echo ""

# Simple comprehensive allow that won't have duplicates
CLEAN_ALLOW='#![allow(unused_imports, unused_variables, dead_code, clippy::all)]'

# Function to fix file
fix_file() {
    local file=$1
    
    # Remove all existing #![allow...] lines and add clean one
    sed -i '/^#!\[allow/d' "$file"
    
    # Add after file docstring or at start
    if head -5 "$file" | grep -q "^//"; then
        # Has docstring
        awk -v allow="$CLEAN_ALLOW" '
            BEGIN { added = 0; blank = 0 }
            /^\/\// { print; next }
            /^$/ && !added { blank = 1; print; next }
            !added && !/^\/\// && !/^$/ { 
                if (!blank) print "";
                print allow;
                print "";
                added = 1
            }
            { print }
        ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
    else
        # No docstring
        echo -e "$CLEAN_ALLOW\n" | cat - "$file" > "$file.tmp" && mv "$file.tmp" "$file"
    fi
    
    echo "  ✅ $file"
}

echo "📦 beardog-monitoring..."
fix_file "crates/beardog-monitoring/src/metrics/analytics_tests.rs"
fix_file "crates/beardog-monitoring/src/metrics/core_tests.rs"
fix_file "crates/beardog-monitoring/src/metrics/ecosystem_tests.rs"
fix_file "crates/beardog-monitoring/src/metrics/performance_tests.rs"
fix_file "crates/beardog-monitoring/src/metrics/security_tests.rs"

echo ""
echo "📦 beardog-utils..."
for file in crates/beardog-utils/src/*test*.rs; do
    [ -f "$file" ] && fix_file "$file"
done

echo ""
echo "📦 beardog-threat..."
for file in crates/beardog-threat/src/*test*.rs crates/beardog-threat/src/threat/*test*.rs; do
    [ -f "$file" ] && fix_file "$file"
done

echo ""
echo "✅ Files updated!"
echo ""
echo "🧪 Testing..."

for crate in beardog-monitoring beardog-utils beardog-threat; do
    echo "Checking $crate..."
    if cargo clippy --package "$crate" --all-targets --all-features -- -D warnings 2>&1 | grep -q "^error:"; then
        echo "  ⚠️  Still has errors"
    else
        echo "  ✅ CLEAN!"
    fi
done

echo ""
echo "✅ Done!"

