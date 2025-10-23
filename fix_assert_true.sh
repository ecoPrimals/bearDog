#!/bin/bash
# Script to fix assert!(true) errors across the codebase
# Replaces assert!(true) with proper test completion or removes them

echo "Fixing assert!(true) errors..."

# Find all files with assert!(true)
FILES=$(grep -r "assert!(true)" crates/ --include="*.rs" -l)

for file in $FILES; do
    echo "Processing: $file"
    # Replace assert!(true); with // Test passes (placeholder removed)
    sed -i 's/assert!(true);/\/\/ Test passes (placeholder removed)/g' "$file"
    # Replace assert!(true) in match arms or conditionals
    sed -i 's/assert!(true, /\/\/ Matched expected variant: /g' "$file"
done

echo "Done! Fixed assert!(true) errors."
echo "Run: cargo clippy --workspace -- -D warnings to verify"

