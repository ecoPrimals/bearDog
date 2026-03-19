#!/usr/bin/env bash
# Pattern Batch Fixer - Apply idiomatic Rust patterns
# Date: November 28, 2025
# Purpose: Fix explicit_iter_loop and other simple patterns

set -euo pipefail

echo "🐻 BearDog Pattern Batch Fixer"
echo "================================"
echo

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

fixes=0

echo "Phase 1: Explicit Iter Loop Patterns"
echo "------------------------------------"

# Find and fix .iter() in for loops (sample - would need more sophisticated for production)
# This is a demonstration - real implementation would use rust-analyzer or similar

echo "  → Finding explicit .iter() patterns..."
iter_count=$(grep -r "for .* in .*\.iter()" crates --include="*.rs" | wc -l)
echo "  → Found $iter_count potential patterns"
echo "  → Manual review recommended for safety"

echo
echo "Phase 2: Cast Lossless Patterns"
echo "-------------------------------"

# Find as casts that could use From/Into
echo "  → Finding lossless cast patterns..."
cast_count=$(grep -rE " as (f64|f32|i64|u64|i32|u32)" crates --include="*.rs" | grep -v test | wc -l)
echo "  → Found $cast_count potential casts"
echo "  → Manual review recommended"

echo
echo "Phase 3: Range Contains Patterns"
echo "--------------------------------"

# This pattern was already identified in our audit
echo "  → Looking for manual range checks..."
range_count=$(grep -rE ">=.*&&.*<=" crates --include="*.rs" | wc -l)
echo "  → Found $range_count potential patterns"

echo
echo "================================"
echo "Summary:"
echo "  Iter loops:     $iter_count patterns identified"
echo "  Cast lossless:  $cast_count patterns identified"
echo "  Range contains: $range_count patterns identified"
echo
echo -e "${YELLOW}Note: These require manual review and targeted fixes${NC}"
echo -e "${GREEN}Recommendation: Use clippy suggestions for guidance${NC}"
echo
echo "Run: cargo clippy -- -W clippy::explicit_iter_loop -W clippy::cast_lossless"

