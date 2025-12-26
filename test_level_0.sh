#!/bin/bash
# Quick test script to verify all Level 0 demos compile

set -e

echo "🧪 Testing All Level 0 Demos"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd "$(dirname "$0")/showcase/00-local-primal"

demos=(
  "01-hello-beardog"
  "02-hsm-discovery"
  "03-key-constraints"
  "04-entropy-mixing"
  "05-key-lineage"
  "06-btsp-tunnel"
)

total=${#demos[@]}
passed=0
failed=0

for demo in "${demos[@]}"; do
  echo "Testing: $demo"
  cd "$demo"
  
  if cargo build --release > /dev/null 2>&1; then
    echo "  ✅ Compiles successfully"
    ((passed++))
  else
    echo "  ❌ Compilation failed"
    ((failed++))
  fi
  
  cd ..
  echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Results:"
echo "  Total: $total demos"
echo "  Passed: $passed ✅"
echo "  Failed: $failed"
echo ""

if [ $failed -eq 0 ]; then
  echo "🎉 All Level 0 demos compile successfully!"
  exit 0
else
  echo "❌ Some demos failed to compile"
  exit 1
fi

