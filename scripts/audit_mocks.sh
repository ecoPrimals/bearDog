#!/bin/bash
# Mock Audit Script - Categorize all mock instances
# Part of Deep Debt Elimination Plan

set -e

OUTPUT_CSV="mock_inventory.csv"
SUMMARY_FILE="MOCK_AUDIT_SUMMARY.md"

echo "🔍 BearDog Mock Audit - Starting..."
echo ""

# Create CSV header
echo "File,Line,Mock Type,Category,Context,Action,Priority,Notes" > "$OUTPUT_CSV"

# Counters
TOTAL=0
TEST_MOCKS=0
PROD_MOCKS=0

# Find all mock instances
echo "Scanning for mock instances..."
rg "mock|Mock|MOCK" crates/ -n --type rust | while IFS=: read -r file line content; do
    TOTAL=$((TOTAL + 1))
    
    # Determine category
    if [[ $file == *"test"* ]] || [[ $file == *"tests.rs"* ]] || [[ $file == *"_tests.rs"* ]]; then
        category="TEST"
        action="KEEP"
        priority="LOW"
        notes="Test infrastructure - acceptable"
        TEST_MOCKS=$((TEST_MOCKS + 1))
    else
        category="PRODUCTION"
        action="REPLACE"
        priority="HIGH"
        notes="Needs real implementation"
        PROD_MOCKS=$((PROD_MOCKS + 1))
    fi
    
    # Determine mock type
    type="UNKNOWN"
    if [[ $content == *"MockHsm"* ]] || [[ $content == *"mock_hsm"* ]]; then
        type="HSM_MOCK"
    elif [[ $content == *"MockDiscovery"* ]] || [[ $content == *"mock_discovery"* ]]; then
        type="DISCOVERY_MOCK"
    elif [[ $content == *"MockProvider"* ]] || [[ $content == *"mock_provider"* ]]; then
        type="PROVIDER_MOCK"
    elif [[ $content == *"MockService"* ]] || [[ $content == *"mock_service"* ]]; then
        type="SERVICE_MOCK"
    elif [[ $content == *"MockNetwork"* ]] || [[ $content == *"mock_network"* ]]; then
        type="NETWORK_MOCK"
    elif [[ $content == *"#[derive"* ]] && [[ $content == *"Mock"* ]]; then
        type="MOCK_DERIVE"
    fi
    
    # Clean content for CSV
    clean_content=$(echo "$content" | sed 's/"//g' | sed 's/,//g' | tr -d '\n\r')
    
    echo "$file,$line,$type,$category,\"$clean_content\",$action,$priority,\"$notes\"" >> "$OUTPUT_CSV"
done

# Count actual numbers (since we can't use variables in while loop subshell)
TOTAL=$(grep -v "^File," "$OUTPUT_CSV" | wc -l)
TEST_MOCKS=$(grep ",TEST," "$OUTPUT_CSV" | wc -l)
PROD_MOCKS=$(grep ",PRODUCTION," "$OUTPUT_CSV" | wc -l)

# Generate summary report
cat > "$SUMMARY_FILE" << EOF
# 🔍 Mock Audit Summary
**Date**: $(date +%Y-%m-%d)
**Status**: Complete

---

## 📊 Overview

**Total Mocks Found**: $TOTAL
- **Test Mocks**: $TEST_MOCKS (${TEST_MOCKS}/$TOTAL - $(( TEST_MOCKS * 100 / TOTAL ))%)
- **Production Mocks**: $PROD_MOCKS (${PROD_MOCKS}/$TOTAL - $(( PROD_MOCKS * 100 / TOTAL ))%)

---

## 📋 Breakdown by Type

EOF

# Count by type
echo "### Mock Types" >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"
grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f3 | sort | uniq -c | sort -rn >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

# High-priority production mocks
echo "## 🔴 High-Priority Production Mocks to Replace" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"
echo "These mocks are in production code and need real implementations:" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

grep ",PRODUCTION," "$OUTPUT_CSV" | head -20 | while IFS=, read -r file line type category context action priority notes; do
    echo "- **$file:$line** - $type" >> "$SUMMARY_FILE"
done

# Top files with most mocks
echo "" >> "$SUMMARY_FILE"
echo "## 📁 Top Files with Most Mocks" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"
grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f1 | sort | uniq -c | sort -rn | head -20 >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"

# Action items
cat >> "$SUMMARY_FILE" << EOF

---

## ✅ Next Steps

### Immediate (Week 1)
1. Review production mocks in high-priority files
2. Design real implementations for top 10 production mocks
3. Create migration plan for each category

### Short-term (Week 2-3)
1. Replace HSM mocks with real implementations
2. Replace Discovery mocks with real implementations
3. Replace Provider mocks with real implementations

### Documentation
1. Document all test mocks and their purpose
2. Add comments explaining why test mocks are acceptable
3. Create guidelines for when mocks are appropriate

---

## 📊 Success Criteria

- [ ] All production mocks categorized
- [ ] Implementation plans for all high-priority mocks
- [ ] Test mocks documented
- [ ] Production mocks: $PROD_MOCKS → 0

---

**Full details in**: \`$OUTPUT_CSV\`
EOF

echo ""
echo "✅ Mock Audit Complete!"
echo ""
echo "📊 Summary:"
echo "  Total Mocks: $TOTAL"
echo "  Test Mocks: $TEST_MOCKS (acceptable)"
echo "  Production Mocks: $PROD_MOCKS (need replacement)"
echo ""
echo "📄 Reports generated:"
echo "  - $OUTPUT_CSV"
echo "  - $SUMMARY_FILE"
echo ""
echo "🎯 Next: Review $SUMMARY_FILE for action items"

