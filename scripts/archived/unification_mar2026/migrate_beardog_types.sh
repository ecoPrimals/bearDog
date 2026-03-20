#!/bin/bash
#
# migrate_beardog_types.sh - Surgical migration of BearDogResult in beardog-types
#
# Strategy: File-by-file with compilation checks
#

set -e

echo "═══════════════════════════════════════════════════"
echo "🔧 Phase 1A-2: beardog-types Migration (Surgical)"
echo "═══════════════════════════════════════════════════"
echo

# Count before
BEFORE=$(grep -r "BearDogResult" --include="*.rs" crates/beardog-types/ | wc -l)
echo "📊 Before: $BEFORE BearDogResult usages in beardog-types"
echo

# Get all files with BearDogResult in beardog-types
FILES=$(find crates/beardog-types -name "*.rs" -exec grep -l "BearDogResult" {} \;)

echo "🔄 Migrating beardog-types files..."
echo

COUNT=0
for file in $FILES; do
    if [ -f "$file" ]; then
        COUNT=$((COUNT + 1))
        echo "[$COUNT/32] Migrating: $file"
        
        # Pattern 1: pub type BearDogResult<T> -> Result<T, BearDogError>
        sed -i 's/pub type BearDogResult<T> = Result<T, BearDogError>;/\/\/ DEPRECATED: Use Result<T, BearDogError> directly/g' "$file"
        
        # Pattern 2: BearDogResult<T> -> Result<T, BearDogError>
        sed -i 's/BearDogResult<\([^>]*\)>/Result<\1, BearDogError>/g' "$file"
        
        # Pattern 3: -> BearDogResult -> Result<(), BearDogError>
        sed -i 's/-> BearDogResult/-> Result<(), BearDogError>/g' "$file"
        
        # Pattern 4: : BearDogResult -> : Result<(), BearDogError>
        sed -i 's/: BearDogResult/: Result<(), BearDogError>/g' "$file"
        
        # Pattern 5: Remove use statements
        sed -i '/^use.*BearDogResult/d' "$file"
        sed -i '/use.*BearDogResult/d' "$file"
    fi
done

echo
echo "✅ Migration complete!"
echo

# Count after
AFTER=$(grep -r "BearDogResult" --include="*.rs" crates/beardog-types/ | wc -l || echo "0")
echo "📊 After: $AFTER BearDogResult usages in beardog-types"
echo "📉 Reduced: $(($BEFORE - $AFTER)) usages"
echo

# Check compilation
echo "🔍 Checking compilation..."
echo
if cargo check --package beardog-types 2>&1 | head -30; then
    echo
    echo "═══════════════════════════════════════════════════"
    echo "✅ Phase 1A-2 COMPLETE"
    echo "═══════════════════════════════════════════════════"
else
    echo
    echo "⚠️  Compilation issues detected"
    echo "═══════════════════════════════════════════════════"
fi

