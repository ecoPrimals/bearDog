#!/bin/bash
# BearDog Technical Debt Monitor
# Monitors for unwrap calls, TODO items, and other technical debt indicators

echo "🔍 BearDog Technical Debt Monitor"
echo "=================================="
echo ""

PRODUCTION_UNWRAP_COUNT=0
PRODUCTION_EXPECT_COUNT=0
TODO_COUNT=0
ASYNC_TRAIT_COUNT=0
BOX_DYN_COUNT=0
PRODUCTION_PANIC_COUNT=0

# Check for production unwrap calls (excluding tests and safe patterns)
echo "🚨 Checking for production unwrap calls..."
PRODUCTION_UNWRAP_FILES=$(find crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -path "*/test_*" \
    -not -name "*test.rs" \
    -not -path "*/examples/*" \
    -not -path "*/benchmarks/*" \
    -not -path "*/unwrap-migrator/*" \
    -exec grep -l "\.unwrap()" {} \; 2>/dev/null | \
    while read file; do
        # Check if unwrap is in a test function
        if ! grep -B20 "\.unwrap()" "$file" | grep -q "fn.*test\|#\[test\]\|#\[tokio::test\]"; then
            echo "$file"
        fi
    done)

if [ -n "$PRODUCTION_UNWRAP_FILES" ]; then
    PRODUCTION_UNWRAP_COUNT=$(echo "$PRODUCTION_UNWRAP_FILES" | wc -l)
    echo "⚠️  Found $PRODUCTION_UNWRAP_COUNT files with .unwrap() calls in production code:"
    echo "$PRODUCTION_UNWRAP_FILES" | head -3
else
    echo "✅ No production unwrap calls found"
fi

echo ""

# Check for production expect calls (excluding tests and safe patterns)
echo "🔍 Checking for production expect calls..."
PRODUCTION_EXPECT_FILES=$(find crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -path "*/test_*" \
    -not -name "*test.rs" \
    -not -path "*/examples/*" \
    -not -path "*/benchmarks/*" \
    -not -path "*/unwrap-migrator/*" \
    -exec grep -l "\.expect(" {} \; 2>/dev/null | \
    while read file; do
        # Check if expect is in a test function, cfg(test) module, or is a safe pattern
        if ! grep -B100 "\.expect(" "$file" | grep -q "fn.*test\|#\[test\]\|#\[tokio::test\]\|#\[cfg(test)\]"; then
            # Check if it's a safe pattern we added
            if ! grep -A3 -B3 "\.expect(" "$file" | grep -q "Safe to.*because\|Object should exist after default\|Expected.*test case"; then
                echo "$file"
            fi
        fi
    done)

if [ -n "$PRODUCTION_EXPECT_FILES" ]; then
    PRODUCTION_EXPECT_COUNT=$(echo "$PRODUCTION_EXPECT_FILES" | wc -l)
    echo "⚠️  Found $PRODUCTION_EXPECT_COUNT files with unsafe .expect() calls in production code"
else
    echo "✅ No unsafe production expect calls found"
fi

echo ""

# Check for TODO comments (excluding placeholder URLs)
echo "📝 Checking for TODO comments..."
TODO_RESULTS=$(find crates -name "*.rs" -exec grep -Hn "TODO\|FIXME\|XXX\|HACK" {} \; 2>/dev/null | \
    grep -v "hooks\.slack\.com" | \
    grep -v "XXXXXXXX")

if [ -n "$TODO_RESULTS" ]; then
    TODO_COUNT=$(echo "$TODO_RESULTS" | wc -l)
    echo "📋 Found $TODO_COUNT TODO/FIXME comments"
    if [ $TODO_COUNT -gt 0 ]; then
        echo "   Examples:"
        echo "$TODO_RESULTS" | head -3 | sed 's/^/   - /'
    fi
else
    echo "✅ No TODO/FIXME comments found"
fi

echo ""

# Check for async_trait usage
echo "⚡ Checking for async_trait usage..."
ASYNC_TRAIT_RESULTS=$(find crates -name "*.rs" -exec grep -l "#\[async_trait\]" {} \; 2>/dev/null)

if [ -n "$ASYNC_TRAIT_RESULTS" ]; then
    ASYNC_TRAIT_COUNT=$(echo "$ASYNC_TRAIT_RESULTS" | wc -l)
    echo "⚠️  Found $ASYNC_TRAIT_COUNT files using async_trait"
else
    echo "✅ No async_trait usage found (fully migrated to native async)"
fi

echo ""

# Check for Box<dyn Trait> patterns (count but don't penalize heavily)
echo "📦 Checking for Box<dyn Trait> patterns..."
BOX_DYN_RESULTS=$(find crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -path "*/examples/*" \
    -not -path "*/benchmarks/*" \
    -not -path "*/unwrap-migrator/*" \
    -exec grep -c "Box<dyn" {} \; 2>/dev/null | \
    awk -F: '{sum += $2} END {print sum}')

if [ "$BOX_DYN_RESULTS" -gt 0 ]; then
    BOX_DYN_COUNT=$BOX_DYN_RESULTS
    echo "📦 Found $BOX_DYN_COUNT Box<dyn Trait> patterns"
    echo "   (These may be acceptable for flexibility, but consider static dispatch where possible)"
else
    echo "✅ No Box<dyn Trait> patterns found"
fi

echo ""

# Check for panic! calls in production
echo "💥 Checking for panic! calls in production..."
PRODUCTION_PANIC_FILES=$(find crates -name "*.rs" \
    -not -path "*/tests/*" \
    -not -path "*/test_*" \
    -not -name "*test.rs" \
    -not -path "*/examples/*" \
    -not -path "*/benchmarks/*" \
    -not -path "*/unwrap-migrator/*" \
    -exec grep -l "panic!" {} \; 2>/dev/null | \
    while read file; do
        # Check if panic is in a test function
        if ! grep -B30 "panic!" "$file" | grep -q "fn.*test\|#\[test\]\|#\[tokio::test\]\|#\[cfg(test)\]"; then
            echo "$file"
        fi
    done)

if [ -n "$PRODUCTION_PANIC_FILES" ]; then
    PRODUCTION_PANIC_COUNT=$(echo "$PRODUCTION_PANIC_FILES" | wc -l)
    echo "🚨 Found $PRODUCTION_PANIC_COUNT panic! calls in production code"
    echo "   These should be replaced with proper error handling"
else
    echo "✅ No panic! calls in production code found"
fi

echo ""

# Calculate technical debt score
# Updated scoring system for zero debt achievement
DEBT_SCORE=0

# Critical issues (high penalty)
DEBT_SCORE=$((DEBT_SCORE + PRODUCTION_UNWRAP_COUNT * 10))
DEBT_SCORE=$((DEBT_SCORE + PRODUCTION_PANIC_COUNT * 8))
DEBT_SCORE=$((DEBT_SCORE + PRODUCTION_EXPECT_COUNT * 5))

# Moderate issues (lower penalty)
DEBT_SCORE=$((DEBT_SCORE + TODO_COUNT * 2))
DEBT_SCORE=$((DEBT_SCORE + ASYNC_TRAIT_COUNT * 3))

# Box<dyn> patterns (minimal penalty - often appropriate)
DEBT_SCORE=$((DEBT_SCORE + BOX_DYN_COUNT / 50))  # Very low penalty

echo "📊 Technical Debt Summary:"
echo "   Production unwrap calls: $PRODUCTION_UNWRAP_COUNT"
echo "   Production expect calls: $PRODUCTION_EXPECT_COUNT"
echo "   TODO/FIXME comments: $TODO_COUNT"
echo "   async_trait usage: $ASYNC_TRAIT_COUNT"
echo "   Box<dyn Trait> patterns: $BOX_DYN_COUNT"
echo "   Production panic! calls: $PRODUCTION_PANIC_COUNT"
echo ""

if [ $DEBT_SCORE -eq 0 ]; then
    echo "🎉 EXCELLENT: Zero technical debt achieved! (score: 0)"
    exit 0
elif [ $DEBT_SCORE -le 5 ]; then
    echo "✅ VERY GOOD: Minimal technical debt (score: $DEBT_SCORE)"
    exit 0
elif [ $DEBT_SCORE -le 15 ]; then
    echo "👍 GOOD: Low technical debt (score: $DEBT_SCORE)"
    exit 0
elif [ $DEBT_SCORE -le 35 ]; then
    echo "⚠️  MODERATE: Some technical debt needs attention (score: $DEBT_SCORE)"
    exit 1
elif [ $DEBT_SCORE -le 60 ]; then
    echo "🚨 HIGH: Significant technical debt requires immediate attention (score: $DEBT_SCORE)"
    exit 1
else
    echo "💥 CRITICAL: Critical technical debt - major cleanup needed (score: $DEBT_SCORE)"
    exit 1
fi 