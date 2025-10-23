#!/bin/bash
# Find all unwrap/expect in production code (excluding tests)

echo "=========================================="
echo "Finding Production Unwraps/Expects"
echo "=========================================="
echo ""

# Find unwraps in production code
grep -r "\.unwrap()\|\.expect(" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|#\[cfg(test)\]\|mod tests" \
  > /tmp/production_unwraps.txt

# Count total
total=$(wc -l < /tmp/production_unwraps.txt)
echo "Total production unwraps found: $total"
echo ""

# Categorize by priority
echo "=== CRITICAL (Security/Crypto/HSM) ==="
grep -E "security|crypto|hsm|key_management|auth|signature|encryption" /tmp/production_unwraps.txt | head -20
critical=$(grep -E "security|crypto|hsm|key_management|auth|signature|encryption" /tmp/production_unwraps.txt | wc -l)
echo "Critical count: $critical"
echo ""

echo "=== HIGH (Config/API/Network) ==="
grep -E "config|api|network|endpoint|runtime" /tmp/production_unwraps.txt | head -20
high=$(grep -E "config|api|network|endpoint|runtime" /tmp/production_unwraps.txt | wc -l)
echo "High priority count: $high"
echo ""

echo "=== MEDIUM (Utils/Validation) ==="
grep -E "util|valid|convert|parse|helper" /tmp/production_unwraps.txt | head -20
medium=$(grep -E "util|valid|convert|parse|helper" /tmp/production_unwraps.txt | wc -l)
echo "Medium priority count: $medium"
echo ""

echo "Full results saved to: /tmp/production_unwraps.txt"
echo ""
echo "=========================================="
echo "Top 25 Most Critical Files:"
echo "=========================================="

# Show files with most unwraps
cat /tmp/production_unwraps.txt | \
  cut -d: -f1 | \
  sort | uniq -c | \
  sort -rn | \
  head -25

echo ""
echo "Run 'cat /tmp/production_unwraps.txt' to see full list"

