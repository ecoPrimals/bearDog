#!/bin/bash
# Quick Test Fix Script - Adds async to tokio::test functions
# Usage: ./quick_test_fix.sh <test_file>

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <test_file>"
    exit 1
fi

FILE="$1"

if [ ! -f "$FILE" ]; then
    echo "Error: File $FILE not found"
    exit 1
fi

echo "Fixing: $FILE"

# Pattern 1: #[tokio::test] followed by fn (not async fn)
# Replace 'fn test' with 'async fn test' after #[tokio::test]
sed -i 's/\(#\[tokio::test\]\)/\1\n\/\/ FIXED: Added async keyword/' "$FILE"
sed -i '/#\[tokio::test\]/,/^fn / s/^fn \(test[^(]*\)/async fn \1/' "$FILE"

# Remove the FIXED comments we added (they were just markers)
sed -i '/\/\/ FIXED: Added async keyword/d' "$FILE"

echo "✅ Fixed: $FILE"

