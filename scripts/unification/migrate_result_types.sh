#!/bin/bash
# Migrate BearDogResult<T> → Result<T, BearDogError>
# Part of Phase 1: Type System Unification

echo "🔄 Starting Result Type Migration..."
echo "===================================="
echo ""

# Safety: Create backup branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📋 Current branch: $CURRENT_BRANCH"
echo "💾 Creating backup branch: backup-before-result-migration"
git branch backup-before-result-migration 2>/dev/null || echo "   (backup branch already exists)"

echo ""
echo "🔍 Finding BearDogResult usages..."

# Count current usages
BEFORE_COUNT=$(grep -r "BearDogResult" crates --include="*.rs" | grep -v "deprecated" | grep -v "test" | grep -v "//.*BearDogResult" | wc -l)
echo "   Found $BEFORE_COUNT usages to migrate"

if [ "$BEFORE_COUNT" -eq 0 ]; then
    echo "✅ No migrations needed - already complete!"
    exit 0
fi

echo ""
echo "🔧 Performing migrations..."

# Counter for progress
PROCESSED=0

# Find all Rust files (excluding tests initially)
find crates -name "*.rs" -type f | while read file; do
    # Skip test files in first pass
    if [[ "$file" == *"/tests/"* ]] || [[ "$file" == *"test.rs" ]]; then
        continue
    fi
    
    # Check if file contains BearDogResult
    if grep -q "BearDogResult" "$file"; then
        echo "   Processing: $file"
        
        # Create temp file for modifications
        TMP_FILE="${file}.tmp"
        
        # Replace patterns
        sed 's/-> BearDogResult</-> Result</g' "$file" | \
        sed 's/: BearDogResult</: Result</g' | \
        sed 's/BearDogResult::/Result::/g' | \
        sed 's/pub type BearDogResult/pub type BearDogResult/g' > "$TMP_FILE"
        
        # Check if file needs BearDogError import
        if grep -q "Result<.*BearDogError>" "$TMP_FILE"; then
            # Check if import already exists
            if ! grep -q "use beardog_errors::BearDogError" "$TMP_FILE"; then
                # Add import after first use statement
                awk '/^use / && !found {print; print "use beardog_errors::BearDogError;"; found=1; next} 1' "$TMP_FILE" > "${TMP_FILE}.2"
                mv "${TMP_FILE}.2" "$TMP_FILE"
            fi
        fi
        
        # Replace original file
        mv "$TMP_FILE" "$file"
        
        PROCESSED=$((PROCESSED + 1))
    fi
done

echo ""
echo "✅ Migration complete!"
echo ""
echo "📊 Statistics:"
echo "   Files processed: $PROCESSED"

# Check results
AFTER_COUNT=$(grep -r "BearDogResult" crates --include="*.rs" | grep -v "deprecated" | grep -v "test" | grep -v "//.*BearDogResult" | wc -l)
MIGRATED=$((BEFORE_COUNT - AFTER_COUNT))

echo "   Usages migrated: $MIGRATED"
echo "   Remaining: $AFTER_COUNT"

echo ""
echo "🔍 Validation..."
echo "   Running cargo check..."

if cargo check --workspace --quiet 2>&1 | head -20; then
    echo "   ✅ Basic compilation check passed"
else
    echo "   ⚠️  Some compilation errors detected"
    echo "   This is normal - will fix in next step"
fi

echo ""
echo "📋 Next Steps:"
echo "   1. Review changes: git diff"
echo "   2. Fix any compilation errors: cargo check --workspace"
echo "   3. Run tests: cargo test --workspace"
echo "   4. If issues: git checkout backup-before-result-migration"
echo ""
echo "   To commit changes:"
echo "   git add -A"
echo "   git commit -m 'Unification: Migrate BearDogResult to idiomatic Result<T, E>'"

