#!/bin/bash
# Identify Critical Unwraps in Production Code
# Excludes test files and ranks by priority

set -euo pipefail

echo "🔍 BEARDOG CRITICAL UNWRAP ANALYSIS"
echo "==================================="
echo ""

# Colors
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Count total unwraps
total_unwraps=$(grep -r "\.unwrap()\|\.expect(" crates/ 2>/dev/null | wc -l)
echo "📊 Total unwraps/expects: $total_unwraps"

# Count production unwraps (exclude tests)
prod_unwraps=$(grep -r "\.unwrap()\|\.expect(" crates/ 2>/dev/null | grep -v test | wc -l)
echo "🚨 Production unwraps: $prod_unwraps"

test_unwraps=$((total_unwraps - prod_unwraps))
echo "✅ Test unwraps: $test_unwraps"
echo ""

echo "🎯 TOP 20 FILES WITH PRODUCTION UNWRAPS"
echo "========================================"
echo ""

# Find production unwraps by file
grep -r "\.unwrap()\|\.expect(" crates/ 2>/dev/null | \
  grep -v test | \
  cut -d: -f1 | \
  sort | \
  uniq -c | \
  sort -rn | \
  head -20 | \
  while read count file; do
    # Priority based on module
    priority="P3"
    if echo "$file" | grep -q "beardog-security"; then
      priority="${RED}P1${NC}"
    elif echo "$file" | grep -q "beardog-tunnel.*hsm"; then
      priority="${RED}P1${NC}"
    elif echo "$file" | grep -q "beardog-core"; then
      priority="${YELLOW}P2${NC}"
    fi
    
    echo -e "${priority} - $count unwraps: $file"
  done

echo ""
echo "📋 PRIORITY 1: SECURITY MODULE"
echo "=============================="
echo ""

# Security module unwraps
security_files=$(grep -r "\.unwrap()\|\.expect(" crates/beardog-security/ 2>/dev/null | \
  grep -v test | \
  cut -d: -f1 | \
  sort | \
  uniq -c | \
  sort -rn | \
  head -10)

if [ -n "$security_files" ]; then
  echo "$security_files"
else
  echo "✅ No production unwraps in security module!"
fi

echo ""
echo "📋 PRIORITY 1: HSM MODULE"
echo "========================="
echo ""

# HSM module unwraps
hsm_files=$(grep -r "\.unwrap()\|\.expect(" crates/beardog-tunnel/src/tunnel/hsm/ 2>/dev/null | \
  grep -v test | \
  cut -d: -f1 | \
  sort | \
  uniq -c | \
  sort -rn | \
  head -10)

if [ -n "$hsm_files" ]; then
  echo "$hsm_files"
else
  echo "✅ No production unwraps in HSM module!"
fi

echo ""
echo "📋 PRIORITY 2: CORE MODULE"
echo "=========================="
echo ""

# Core module unwraps
core_files=$(grep -r "\.unwrap()\|\.expect(" crates/beardog-core/ 2>/dev/null | \
  grep -v test | \
  cut -d: -f1 | \
  sort | \
  uniq -c | \
  sort -rn | \
  head -10)

if [ -n "$core_files" ]; then
  echo "$core_files"
else
  echo "✅ No production unwraps in core module!"
fi

echo ""
echo "🔧 SAMPLE FIXES NEEDED"
echo "====================="
echo ""

# Show first 5 actual unwrap locations
echo "Example unwraps to fix:"
grep -rn "\.unwrap()" crates/beardog-security/src/ 2>/dev/null | \
  grep -v test | \
  head -5 | \
  while IFS=: read -r file line code; do
    echo "  $file:$line"
    echo "    $code"
    echo ""
  done

echo "💡 RECOMMENDATIONS"
echo "=================="
echo ""
echo "1. Start with beardog-security module (highest priority)"
echo "2. Focus on files with most unwraps (biggest impact)"
echo "3. Convert unwraps to proper error propagation:"
echo "   - Change: value.unwrap()"
echo "   - To: value.map_err(|e| BearDogError::from(e))?"
echo ""
echo "4. Add tests for new error paths"
echo "5. Verify error messages are helpful"
echo ""
echo "📝 DETAILED REPORT"
echo "=================="
echo ""
echo "Run these commands for specific files:"
echo ""
echo "# Show all unwraps in a specific file:"
echo "grep -n '\.unwrap()\|\.expect(' crates/beardog-security/src/lib.rs"
echo ""
echo "# Count by type:"
echo "grep -r '\.unwrap()' crates/ | grep -v test | wc -l"
echo "grep -r '\.expect(' crates/ | grep -v test | wc -l"
echo ""

echo "✅ Analysis complete!"

