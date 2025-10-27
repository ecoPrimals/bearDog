#!/bin/bash
# Enhanced Unwrap Analyzer for BearDog
# Properly excludes #[cfg(test)] blocks and test files

echo "🔍 BearDog Unwrap/Expect Analysis (Enhanced)"
echo "============================================="
echo ""

ROOT="${1:-../crates}"

# Create temp file for analysis
TEMPFILE=$(mktemp)
trap "rm -f $TEMPFILE" EXIT

# Function to check if a line is in a test block
is_in_test_block() {
    local file=$1
    local line_num=$2
    
    # Check if file path contains 'test'
    if [[ "$file" =~ /tests/ ]] || [[ "$file" =~ _test\.rs$ ]] || [[ "$file" =~ test_.*\.rs$ ]]; then
        return 0  # true - it's a test file
    fi
    
    # Check if we're in a #[cfg(test)] block
    # This is a heuristic: check lines before for #[cfg(test)]
    local test_marker=$(awk -v line="$line_num" '
        NR <= line {
            if (/^#\[cfg\(test\)\]/ || /^#\[test\]/) {
                in_test = 1
                test_line = NR
            }
            if (in_test && NR == line) {
                print "IN_TEST"
            }
            # Reset if we hit module boundary
            if (/^(pub )?mod [^{]*\{/ && NR > test_line + 20) {
                in_test = 0
            }
        }
    ' "$file")
    
    if [ "$test_marker" = "IN_TEST" ]; then
        return 0  # true - in test block
    fi
    
    return 1  # false - not in test
}

# Count total unwraps/expects
echo "📊 Analyzing codebase..."
TOTAL_UNWRAP=0
TOTAL_EXPECT=0
PROD_UNWRAP=0
PROD_EXPECT=0
TEST_UNWRAP=0
TEST_EXPECT=0

# Find all unwrap/expect calls
while IFS=: read -r file line_num content; do
    if [[ "$content" =~ \.unwrap\(\) ]]; then
        ((TOTAL_UNWRAP++))
        if is_in_test_block "$file" "$line_num"; then
            ((TEST_UNWRAP++))
        else
            ((PROD_UNWRAP++))
            echo "$file:$line_num" >> "$TEMPFILE"
        fi
    fi
    
    if [[ "$content" =~ \.expect\( ]]; then
        ((TOTAL_EXPECT++))
        if is_in_test_block "$file" "$line_num"; then
            ((TEST_EXPECT++))
        else
            ((PROD_EXPECT++))
            echo "$file:$line_num" >> "$TEMPFILE"
        fi
    fi
done < <(grep -rn "\.unwrap()\|\.expect(" "$ROOT" --include="*.rs" 2>/dev/null)

TOTAL=$((TOTAL_UNWRAP + TOTAL_EXPECT))
PROD_TOTAL=$((PROD_UNWRAP + PROD_EXPECT))
TEST_TOTAL=$((TEST_UNWRAP + TEST_EXPECT))

echo ""
echo "📊 Total Count:"
echo "   .unwrap():  $TOTAL_UNWRAP"
echo "   .expect():  $TOTAL_EXPECT"
echo "   Total:      $TOTAL"
echo ""

echo "📁 By Location:"
echo "   Production:  $PROD_TOTAL ($PROD_UNWRAP unwrap, $PROD_EXPECT expect)"
echo "   Tests:       $TEST_TOTAL ($TEST_UNWRAP unwrap, $TEST_EXPECT expect)"
echo ""

# Top 10 files with most production unwraps
if [ -s "$TEMPFILE" ]; then
    echo "🚨 Top 10 Production Files (unwrap + expect):"
    cut -d: -f1 "$TEMPFILE" | sort | uniq -c | sort -rn | head -10 | \
        awk '{print "   " $1 " - " $2}'
    echo ""
fi

# Critical files to review
echo "🎯 High-Priority Files for Manual Review:"
echo ""

CRITICAL_PATTERNS=(
    "core/system"
    "security/standalone"
    "tunnel/hsm"
    "types/production"
    "ecosystem"
    "discovery"
    "registry"
)

if [ -s "$TEMPFILE" ]; then
    for pattern in "${CRITICAL_PATTERNS[@]}"; do
        COUNT=$(grep "$pattern" "$TEMPFILE" | wc -l)
        if [ $COUNT -gt 0 ]; then
            echo "   $pattern*: $COUNT instances"
        fi
    done
fi

echo ""
echo "💡 Recommendations:"
echo "   1. Focus on production files first ($PROD_TOTAL instances)"
echo "   2. Start with top 10 files above"
echo "   3. Convert to proper Result<T, E> error handling"
echo "   4. Tests can keep unwrap/expect ($TEST_TOTAL instances OK)"
echo ""
echo "📝 To see details for a specific file:"
echo "   grep -n '\.unwrap()\|\.expect(' <file>"
echo ""

# Generate detailed report file
REPORT_FILE="unwrap_analysis_$(date +%Y%m%d_%H%M%S).txt"
echo "📄 Detailed report saved to: $REPORT_FILE"
{
    echo "BearDog Unwrap Analysis Report"
    echo "Generated: $(date)"
    echo ""
    echo "Summary:"
    echo "  Total: $TOTAL ($TOTAL_UNWRAP unwrap, $TOTAL_EXPECT expect)"
    echo "  Production: $PROD_TOTAL ($PROD_UNWRAP unwrap, $PROD_EXPECT expect)"
    echo "  Tests: $TEST_TOTAL ($TEST_UNWRAP unwrap, $TEST_EXPECT expect)"
    echo ""
    echo "Production unwrap locations:"
    if [ -s "$TEMPFILE" ]; then
        sort "$TEMPFILE" | uniq
    else
        echo "  (none found)"
    fi
} > "$REPORT_FILE"

echo ""
