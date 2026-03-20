#!/bin/bash
#
# migrate_non_types.sh - Migrate BearDogResult -> Result<T, BearDogError> in non-beardog-types crates
#
# Usage: ./migrate_non_types.sh
#

set -e

echo "═══════════════════════════════════════════════════"
echo "🔧 Phase 1A-1: Result Type Migration (Non-beardog-types)"
echo "═══════════════════════════════════════════════════"
echo

# Get all Rust files except in beardog-types
FILES=$(find crates -name "*.rs" -type f | grep -v "beardog-types" | grep -v "target")

# Count before
BEFORE=$(grep -r "BearDogResult" --include="*.rs" crates/ | grep -v "beardog-types" | wc -l)
echo "📊 Before: $BEFORE BearDogResult usages (non-beardog-types)"
echo

# Migration patterns
echo "🔄 Applying migrations..."

for file in $FILES; do
    if grep -q "BearDogResult" "$file" 2>/dev/null; then
        echo "  • Migrating: $file"
        
        # Pattern 1: BearDogResult<T> -> Result<T, BearDogError>
        sed -i 's/BearDogResult<\([^>]*\)>/Result<\1, BearDogError>/g' "$file"
        
        # Pattern 2: pub type BearDogResult<T> = ... (type alias definitions)
        sed -i 's/pub type BearDogResult<T> = .*/\/\/ DEPRECATED: Use Result<T, BearDogError> directly/g' "$file"
        
        # Pattern 3: -> BearDogResult -> Result<(), BearDogError>
        sed -i 's/-> BearDogResult/-> Result<(), BearDogError>/g' "$file"
    fi
done

echo
echo "✅ Migration complete!"
echo

# Count after
AFTER=$(grep -r "BearDogResult" --include="*.rs" crates/ | grep -v "beardog-types" | wc -l || echo "0")
echo "📊 After: $AFTER BearDogResult usages (non-beardog-types)"
echo "📉 Reduced: $(($BEFORE - $AFTER)) usages"
echo

# Check for compilation
echo "🔍 Checking compilation..."
if cargo check --workspace 2>&1 | head -20; then
    echo
    echo "═══════════════════════════════════════════════════"
    echo "✅ Phase 1A-1 COMPLETE"
    echo "═══════════════════════════════════════════════════"
else
    echo
    echo "⚠️  Compilation issues detected - review needed"
    echo "═══════════════════════════════════════════════════"
fi

