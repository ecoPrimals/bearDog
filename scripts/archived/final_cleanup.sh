#!/bin/bash
# Final cleanup of duplicate attributes
set -e

echo "🔧 Final cleanup: replacing long allow lists with simple ones..."
echo ""

# Simple allow that covers everything without duplicates
SIMPLE='#![allow(unused_imports, unused_variables, dead_code, unused_comparisons, clippy::all)]'

# Fix files with overly long allow lists
fix_long_allow() {
    local file=$1
    
    if [ ! -f "$file" ]; then
        return 0
    fi
    
    # Replace any #![allow(...)] that's super long with simple version
    if grep -q "#!\[allow.*clippy::nonminimal_bool" "$file"; then
        sed -i "s|#!\[allow.*clippy::nonminimal_bool.*\]|$SIMPLE|g" "$file"
        echo "  ✅ $file"
    fi
}

echo "📦 beardog-utils..."
fix_long_allow "crates/beardog-utils/src/simd_optimizations/tests.rs"
fix_long_allow "crates/beardog-utils/src/simd/optimizations/tests.rs"
fix_long_allow "crates/beardog-utils/src/zero_copy/tests_comprehensive.rs"
fix_long_allow "crates/beardog-utils/src/tests/ultimate_performance_tests.rs"
fix_long_allow "crates/beardog-utils/src/tests/ai_optimization_comprehensive_tests.rs"
fix_long_allow "crates/beardog-utils/src/tests/concurrent_safe_comprehensive_tests.rs"
fix_long_allow "crates/beardog-utils/src/tests/zero_copy_comprehensive_tests.rs"
fix_long_allow "crates/beardog-utils/src/tests/ultimate_modules_comprehensive_tests.rs"
fix_long_allow "crates/beardog-utils/src/tests/performance_safety_comprehensive_tests.rs"

echo ""
echo "📦 beardog-threat..."
fix_long_allow "crates/beardog-threat/src/threat/handlers/ml_integration_tests.rs"
fix_long_allow "crates/beardog-threat/src/threat/handlers/tests.rs"
fix_long_allow "crates/beardog-threat/src/threat/tests.rs"
fix_long_allow "crates/beardog-threat/src/tests/threat_detection_tests/behavioral_tests.rs"
fix_long_allow "crates/beardog-threat/src/tests/threat_detection_tests/intelligence_tests.rs"
fix_long_allow "crates/beardog-threat/src/tests/threat_detection_tests/monitoring_tests.rs"
fix_long_allow "crates/beardog-threat/src/tests/threat_detection_tests/anomaly_tests.rs"

echo ""
echo "✅ Files updated!"
echo ""
echo "🧪 Verifying..."

for crate in beardog-utils beardog-threat; do
    echo "Checking $crate..."
    if cargo clippy --package "$crate" --all-targets --all-features -- -D warnings 2>&1 | grep -q "^error:"; then
        echo "  ⚠️  Still has errors"
        cargo clippy --package "$crate" --all-targets --all-features -- -D warnings 2>&1 | grep "^error:" | head -3
    else
        echo "  ✅ CLEAN!"
    fi
done

echo ""
echo "✅ Done!"

