#!/bin/bash
# Fix remaining Result<T> patterns that are missing error type
# Part of Phase 1A: Result Type Migration - Final Cleanup

echo "🔧 Fixing remaining Result<T> patterns..."
echo "=========================================="
echo ""

# Count before
BEFORE=$(cargo check --package beardog-types --lib 2>&1 | grep "error\[E0107\]" | wc -l)
echo "Errors before: $BEFORE"

echo ""
echo "Applying fixes..."

# Fix patterns in beardog-types
cd crates/beardog-types || exit 1

# Pattern 1: Result<T> followed by semicolon (trait definitions)
find . -name "*.rs" -type f -exec sed -i \
    's/\(Result<[A-Za-z_0-9]\+>\);/\1, BearDogError>;/g' {} \;

# Pattern 2: Result<T> followed by opening brace
find . -name "*.rs" -type f -exec sed -i \
    's/\(Result<[A-Za-z_0-9]\+>\) {/\1, BearDogError> {/g' {} \;

# Pattern 3: Result<T> followed by where clause
find . -name "*.rs" -type f -exec sed -i \
    's/\(Result<[A-Za-z_0-9]\+>\)$/\1, BearDogError>/g' {} \;

# Pattern 4: Fix specific common types
find . -name "*.rs" -type f -exec sed -i \
    's/Result<Vec</Result<Vec, BearDogError</g' {} \;

find . -name "*.rs" -type f -exec sed -i \
    's/Result<Option</Result<Option, BearDogError</g' {} \;

find . -name "*.rs" -type f -exec sed -i \
    's/Result<Box</Result<Box, BearDogError</g' {} \;

cd ../.. || exit 1

echo "✅ Patterns applied"
echo ""

# Count after
AFTER=$(cargo check --package beardog-types --lib 2>&1 | grep "error\[E0107\]" | wc -l)
echo "Errors after: $AFTER"
echo "Fixed: $((BEFORE - AFTER))"

if [ "$AFTER" -eq 0 ]; then
    echo ""
    echo "🎉 ALL ERRORS FIXED!"
else
    echo ""
    echo "⚠️  $AFTER errors remaining - may need manual fix"
    echo ""
    echo "To see remaining errors:"
    echo "  cargo check --package beardog-types --lib 2>&1 | grep 'error\[E0107\]' -A 3"
fi

