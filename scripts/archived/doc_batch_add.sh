#!/bin/bash
# Documentation Batch Addition Tool
# Helps rapidly add documentation to undocumented items

set -e

echo "🔍 Scanning for undocumented items..."

# Get list of files with the most warnings
cargo doc --workspace --no-deps 2>&1 | \
    grep "warning:" | \
    grep "missing documentation" | \
    awk -F: '{print $1}' | \
    sort | uniq -c | \
    sort -rn | \
    head -20

echo ""
echo "📊 Total documentation warnings:"
cargo doc --workspace --no-deps 2>&1 | grep -c "warning:" || echo "0"

echo ""
echo "💡 Top files to document:"
cargo doc --workspace --no-deps 2>&1 | \
    grep "missing documentation" | \
    awk -F: '{print $1":"$2}' | \
    sort | uniq -c | \
    sort -rn | \
    head -10

echo ""
echo "🎯 Suggested approach:"
echo "1. Focus on beardog-core first (most warnings)"
echo "2. Then beardog-types (high usage)"
echo "3. Then specialized crates"
echo ""
echo "Use: cargo doc --workspace --no-deps 2>&1 | grep 'missing documentation' > docs_todo.txt"

