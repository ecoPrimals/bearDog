#!/bin/bash
# validate-imports.sh
# Validates all imports resolve correctly after consolidation

set -e

echo "🔍 Import Validation"
echo "===================="
echo ""

echo "📦 Checking workspace compilation..."
if cargo check --workspace --all-targets 2>&1 | tee /tmp/cargo_check.log | grep -q "error"; then
    echo ""
    echo "❌ Compilation errors found:"
    echo ""
    grep -A 5 "error" /tmp/cargo_check.log || true
    echo ""
    echo "🔍 Checking for import issues..."
    if grep -E "unresolved import|cannot find type|use of undeclared" /tmp/cargo_check.log; then
        echo ""
        echo "❌ Import errors detected"
        exit 1
    fi
    echo ""
    echo "❌ Other compilation errors present"
    exit 1
else
    echo "✅ All imports valid - compilation successful"
    exit 0
fi 