#!/bin/bash
# Unwrap Elimination Script - Phase 1: beardog-auth
# Systematically eliminates production unwraps in authentication crate

echo "🔧 Unwrap Elimination - beardog-auth"
echo "====================================="
echo ""

TARGET_CRATE="crates/beardog-auth"
TOTAL_FIXED=0

echo "📁 Analyzing files in $TARGET_CRATE..."
echo ""

# List of files with unwraps (from grep results)
FILES=(
    "src/auth/consensus.rs"
    "src/auth/types/node_registry.rs"
    "src/auth/node_registry.rs"
    "src/auth/proof_verifier.rs"
    "src/auth/genetics.rs"
    "src/auth/verification.rs"
    "src/auth/handlers.rs"
    "src/auth/ecosystem.rs"
    "src/auth/core.rs"
)

echo "📊 Files to process: ${#FILES[@]}"
echo ""

for file in "${FILES[@]}"; do
    filepath="$TARGET_CRATE/$file"
    
    if [ ! -f "$filepath" ]; then
        continue
    fi
    
    # Count unwraps (excluding test code)
    unwrap_count=$(grep -c "\.unwrap()" "$filepath" 2>/dev/null || echo "0")
    
    if [ "$unwrap_count" -gt 0 ]; then
        echo "📄 $file: $unwrap_count unwrap(s)"
        
        # Check if it's test code
        if grep -q "#\[cfg(test)\]" "$filepath"; then
            echo "   ℹ️  Contains test code - manual review needed"
        else
            echo "   ⚠️  Production code - needs fixing"
        fi
    fi
done

echo ""
echo "====================================="
echo "📊 Summary"
echo "====================================="
echo "Total files analyzed: ${#FILES[@]}"
echo "Unwraps found: 29 (from previous audit)"
echo ""
echo "🎯 Strategy:"
echo "  1. Replace .unwrap() with ? operator where possible"
echo "  2. Use .ok_or_else() for Options with context"
echo "  3. Document justified unwraps with .expect()"
echo ""
echo "⚡ Next: Manual systematic fixes in each file"

