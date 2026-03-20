#!/bin/bash
# TODO Audit Script - Categorize all TODOs, FIXMEs, and technical debt
# Part of Deep Debt Elimination Plan

set -e

OUTPUT_CSV="todo_inventory.csv"
SUMMARY_FILE="TODO_AUDIT_SUMMARY.md"

echo "🔍 BearDog TODO Audit - Starting..."
echo ""

# Create CSV header
echo "File,Line,Type,Priority,Estimated Hours,Category,Description" > "$OUTPUT_CSV"

# Counters
TOTAL=0
CRITICAL=0
HIGH=0
MEDIUM=0
LOW=0

# Find all TODOs
echo "Scanning for TODOs, FIXMEs, HACKs, XXXs..."
rg "TODO|FIXME|XXX|HACK" crates/ -n --type rust | while IFS=: read -r file line content; do
    TOTAL=$((TOTAL + 1))
    
    # Extract TODO type
    type="TODO"
    if [[ $content == *"FIXME"* ]]; then
        type="FIXME"
    elif [[ $content == *"XXX"* ]]; then
        type="XXX"
    elif [[ $content == *"HACK"* ]]; then
        type="HACK"
    fi
    
    # Determine priority based on keywords and context
    priority="MEDIUM"
    hours="4"
    category="GENERAL"
    
    # Critical priority
    if [[ $content == *"CRITICAL"* ]] || [[ $content == *"BLOCKER"* ]] || [[ $content == *"URGENT"* ]]; then
        priority="CRITICAL"
        hours="8"
        CRITICAL=$((CRITICAL + 1))
    # High priority
    elif [[ $content == *"HSM"* ]] || [[ $content == *"security"* ]] || [[ $content == *"crypto"* ]] || \
         [[ $content == *"discovery"* ]] || [[ $content == *"network"* ]]; then
        priority="HIGH"
        hours="6"
        category="INFRASTRUCTURE"
        HIGH=$((HIGH + 1))
    # Low priority
    elif [[ $content == *"nice to have"* ]] || [[ $content == *"future"* ]] || [[ $content == *"optimization"* ]]; then
        priority="LOW"
        hours="2"
        category="ENHANCEMENT"
        LOW=$((LOW + 1))
    else
        MEDIUM=$((MEDIUM + 1))
    fi
    
    # Categorize by domain
    if [[ $file == *"hsm"* ]]; then
        category="HSM"
    elif [[ $file == *"discovery"* ]]; then
        category="DISCOVERY"
    elif [[ $file == *"network"* ]]; then
        category="NETWORK"
    elif [[ $file == *"crypto"* ]]; then
        category="CRYPTO"
    elif [[ $file == *"test"* ]]; then
        category="TESTING"
    elif [[ $file == *"security"* ]]; then
        category="SECURITY"
    fi
    
    # Extract description
    description=$(echo "$content" | sed 's/.*TODO:\s*//' | sed 's/.*FIXME:\s*//' | sed 's/.*XXX:\s*//' | sed 's/.*HACK:\s*//')
    clean_description=$(echo "$description" | sed 's/"//g' | sed 's/,//g' | tr -d '\n\r')
    
    echo "$file,$line,$type,$priority,$hours,$category,\"$clean_description\"" >> "$OUTPUT_CSV"
done

# Count actual numbers
TOTAL=$(grep -v "^File," "$OUTPUT_CSV" | wc -l)
CRITICAL=$(grep ",CRITICAL," "$OUTPUT_CSV" | wc -l)
HIGH=$(grep ",HIGH," "$OUTPUT_CSV" | wc -l)
MEDIUM=$(grep ",MEDIUM," "$OUTPUT_CSV" | wc -l)
LOW=$(grep ",LOW," "$OUTPUT_CSV" | wc -l)

# Calculate effort
TOTAL_HOURS=$(grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f5 | awk '{sum+=$1} END {print sum}')

# Generate summary report
cat > "$SUMMARY_FILE" << EOF
# 🔍 TODO Audit Summary
**Date**: $(date +%Y-%m-%d)
**Status**: Complete

---

## 📊 Overview

**Total TODOs Found**: $TOTAL
**Estimated Effort**: $TOTAL_HOURS hours

### By Priority
- 🔴 **Critical**: $CRITICAL items (~$(( CRITICAL * 8 )) hours)
- 🟡 **High**: $HIGH items (~$(( HIGH * 6 )) hours)
- 🟢 **Medium**: $MEDIUM items (~$(( MEDIUM * 4 )) hours)
- ⚪ **Low**: $LOW items (~$(( LOW * 2 )) hours)

---

## 🔴 Critical Priority TODOs

These block production deployment and must be resolved first:

EOF

grep ",CRITICAL," "$OUTPUT_CSV" | nl -w3 -s'. ' | while IFS=, read -r num file line type priority hours category description; do
    echo "### $num $file:$line" >> "$SUMMARY_FILE"
    echo "**Category**: $category | **Type**: $type | **Effort**: $hours hours" >> "$SUMMARY_FILE"
    echo "$description" >> "$SUMMARY_FILE"
    echo "" >> "$SUMMARY_FILE"
done

# High priority TODOs
cat >> "$SUMMARY_FILE" << EOF

---

## 🟡 High Priority TODOs

These are needed for production quality:

EOF

grep ",HIGH," "$OUTPUT_CSV" | head -10 | nl -w3 -s'. ' | while IFS=, read -r num file line type priority hours category description; do
    echo "- **$file:$line** [$category] - $description" >> "$SUMMARY_FILE"
done

# Breakdown by category
cat >> "$SUMMARY_FILE" << EOF

---

## 📋 Breakdown by Category

EOF

echo '```' >> "$SUMMARY_FILE"
grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f6 | sort | uniq -c | sort -rn >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"

# Breakdown by type
cat >> "$SUMMARY_FILE" << EOF

---

## 📋 Breakdown by Type

EOF

echo '```' >> "$SUMMARY_FILE"
grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f3 | sort | uniq -c | sort -rn >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"

# Top files with most TODOs
cat >> "$SUMMARY_FILE" << EOF

---

## 📁 Top Files with Most TODOs

EOF

echo '```' >> "$SUMMARY_FILE"
grep -v "^File," "$OUTPUT_CSV" | cut -d',' -f1 | sort | uniq -c | sort -rn | head -15 >> "$SUMMARY_FILE"
echo '```' >> "$SUMMARY_FILE"

# Action plan
cat >> "$SUMMARY_FILE" << EOF

---

## ✅ Action Plan

### Week 1: Critical TODOs ($CRITICAL items, ~$(( CRITICAL * 8 )) hours)
$(grep ",CRITICAL," "$OUTPUT_CSV" | cut -d',' -f1,7 | sed 's/,/ - /' | sed 's/^/- [ ] /')

### Week 2-3: High Priority TODOs ($HIGH items, ~$(( HIGH * 6 )) hours)
Focus on:
- HSM implementations
- Discovery implementations
- Network implementations
- Security features

### Week 4-6: Medium Priority TODOs ($MEDIUM items, ~$(( MEDIUM * 4 )) hours)
Systematic cleanup of remaining items

### Future: Low Priority TODOs ($LOW items, ~$(( LOW * 2 )) hours)
Enhancement and optimization opportunities

---

## 📊 Success Criteria

- [ ] All Critical TODOs resolved ($CRITICAL → 0)
- [ ] All High Priority TODOs resolved ($HIGH → 0)
- [ ] 80% of Medium TODOs resolved
- [ ] Documentation updated
- [ ] Tests added for all implementations

---

## 📈 Progress Tracking

Track progress weekly:

\`\`\`bash
# Run this script weekly to track progress
./scripts/audit_todos.sh

# Compare with previous week
diff TODO_AUDIT_SUMMARY.md TODO_AUDIT_SUMMARY_PREV_WEEK.md
\`\`\`

---

**Full details in**: \`$OUTPUT_CSV\`
EOF

echo ""
echo "✅ TODO Audit Complete!"
echo ""
echo "📊 Summary:"
echo "  Total TODOs: $TOTAL"
echo "  Critical: $CRITICAL ($(( CRITICAL * 8 )) hours)"
echo "  High: $HIGH ($(( HIGH * 6 )) hours)"
echo "  Medium: $MEDIUM ($(( MEDIUM * 4 )) hours)"
echo "  Low: $LOW ($(( LOW * 2 )) hours)"
echo "  Total Effort: $TOTAL_HOURS hours"
echo ""
echo "📄 Reports generated:"
echo "  - $OUTPUT_CSV"
echo "  - $SUMMARY_FILE"
echo ""
echo "🎯 Next: Review $SUMMARY_FILE and prioritize action items"

