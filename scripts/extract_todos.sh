#!/bin/bash
# TODO Extraction and Categorization Script
# Created: November 3, 2025
# Purpose: Extract all TODO/FIXME/HACK markers and categorize by priority

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$PROJECT_ROOT/technical_debt_analysis"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 BearDog Technical Debt Extraction${NC}"
echo "================================================"
echo "Timestamp: $TIMESTAMP"
echo "Project Root: $PROJECT_ROOT"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Extract all TODOs with context
echo -e "${YELLOW}📝 Extracting TODO markers...${NC}"
rg "TODO" --context 2 --no-heading --color never \
   --type rust \
   --glob '!target/**' \
   --glob '!archive/**' \
   "$PROJECT_ROOT/crates" > "$OUTPUT_DIR/todos_raw_${TIMESTAMP}.txt" || true

# Extract FIXMEs
echo -e "${YELLOW}🔧 Extracting FIXME markers...${NC}"
rg "FIXME" --context 2 --no-heading --color never \
   --type rust \
   --glob '!target/**' \
   --glob '!archive/**' \
   "$PROJECT_ROOT/crates" > "$OUTPUT_DIR/fixmes_raw_${TIMESTAMP}.txt" || true

# Extract HACKs
echo -e "${YELLOW}⚠️  Extracting HACK markers...${NC}"
rg "HACK" --context 2 --no-heading --color never \
   --type rust \
   --glob '!target/**' \
   --glob '!archive/**' \
   "$PROJECT_ROOT/crates" > "$OUTPUT_DIR/hacks_raw_${TIMESTAMP}.txt" || true

# Extract BUGs
echo -e "${YELLOW}🐛 Extracting BUG markers...${NC}"
rg "BUG" --context 2 --no-heading --color never \
   --type rust \
   --glob '!target/**' \
   --glob '!archive/**' \
   "$PROJECT_ROOT/crates" > "$OUTPUT_DIR/bugs_raw_${TIMESTAMP}.txt" || true

# Count totals
TODO_COUNT=$(rg "TODO" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')
FIXME_COUNT=$(rg "FIXME" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')
HACK_COUNT=$(rg "HACK" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')
BUG_COUNT=$(rg "BUG" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')

TOTAL_DEBT=$((TODO_COUNT + FIXME_COUNT + HACK_COUNT + BUG_COUNT))

# Generate summary report
SUMMARY_FILE="$OUTPUT_DIR/TECHNICAL_DEBT_SUMMARY_${TIMESTAMP}.md"

cat > "$SUMMARY_FILE" << EOF
# 🔍 Technical Debt Analysis Report
**Generated**: $(date)
**Project**: BearDog v3.0.0
**Scope**: All production code (excluding archive/)

---

## 📊 SUMMARY

\`\`\`yaml
Total Markers:    $TOTAL_DEBT
TODO:            $TODO_COUNT ($(echo "scale=1; $TODO_COUNT*100/$TOTAL_DEBT" | bc)%)
FIXME:           $FIXME_COUNT ($(echo "scale=1; $FIXME_COUNT*100/$TOTAL_DEBT" | bc)%)
HACK:            $HACK_COUNT ($(echo "scale=1; $HACK_COUNT*100/$TOTAL_DEBT" | bc)%)
BUG:             $BUG_COUNT ($(echo "scale=1; $BUG_COUNT*100/$TOTAL_DEBT" | bc)%)
\`\`\`

---

## 📁 FILES GENERATED

1. \`todos_raw_${TIMESTAMP}.txt\` - All TODO markers with context
2. \`fixmes_raw_${TIMESTAMP}.txt\` - All FIXME markers with context
3. \`hacks_raw_${TIMESTAMP}.txt\` - All HACK markers with context
4. \`bugs_raw_${TIMESTAMP}.txt\` - All BUG markers with context
5. \`TECHNICAL_DEBT_SUMMARY_${TIMESTAMP}.md\` - This summary

---

## 🎯 TOP 10 FILES BY TODO COUNT

\`\`\`
EOF

# Find top 10 files by TODO count
echo "$(rg "TODO" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | sort -t: -k2 -rn | head -10)" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF
\`\`\`

---

## 🔴 PRIORITY CATEGORIZATION

### Automatic Categorization Rules:
- **P0 (CRITICAL)**: Contains "blocking", "critical", "urgent", "broken", "BUG"
- **P1 (HIGH)**: Contains "FIXME", "important", "should", "needed"
- **P2 (MEDIUM)**: Contains "TODO", "improvement", "refactor"
- **P3 (LOW)**: Everything else, test TODOs

### P0 (Critical) - Estimated Count:
EOF

# Count P0 items (contains blocking, critical, urgent, broken, or is a BUG)
P0_COUNT=$(rg -i "TODO.*(blocking|critical|urgent|broken)|BUG" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')
echo "~$P0_COUNT items" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF

### P1 (High) - Estimated Count:
EOF

# Count P1 items (FIXME or important)
P1_COUNT=$(rg -i "FIXME|TODO.*(important|should|needed)" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F: '{sum+=$2} END {print sum}')
echo "~$P1_COUNT items" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF

### P2 (Medium) - Estimated Count:
EOF

P2_COUNT=$(($TODO_COUNT - $P0_COUNT - $P1_COUNT))
echo "~$P2_COUNT items" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF

---

## 📈 RECOMMENDED ACTIONS

### Immediate (This Week):
1. Review all P0 items and create GitHub issues
2. Address all BUG markers ($BUG_COUNT items)
3. Review HACK markers ($HACK_COUNT items) for proper solutions

### Short-Term (2 Weeks):
1. Create action items for all FIXME markers ($FIXME_COUNT items)
2. Begin systematic P1 TODO resolution
3. Document any deferred technical debt

### Long-Term (2 Months):
1. Reduce TODO count by 50%
2. Eliminate all HACK markers
3. Maintain < 1,000 total technical debt items

---

## 🔍 ANALYSIS BY CRATE

### Top Debt-Heavy Crates:
\`\`\`
EOF

# Count by crate (top 10)
echo "$(rg "TODO|FIXME|HACK|BUG" --count --type rust --glob '!target/**' --glob '!archive/**' "$PROJECT_ROOT/crates" | awk -F/ '{crate=$6} {counts[crate]+=$NF} END {for (c in counts) print counts[c], c}' | sort -rn | head -10)" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF
\`\`\`

---

## 📝 NEXT STEPS

1. **Review this summary** and the generated raw files
2. **Categorize P0 items** manually for accuracy
3. **Create GitHub issues** for all P0 and P1 items
4. **Schedule sprints** for technical debt reduction
5. **Track progress** weekly

---

**Report Generated**: $(date)
**Location**: $OUTPUT_DIR
**Next Analysis**: After 2 weeks or 20% debt reduction

🐻🔐 **BearDog: Systematically eliminating technical debt!**
EOF

# Print summary to console
echo ""
echo -e "${GREEN}✅ Extraction Complete!${NC}"
echo "================================================"
echo -e "${BLUE}📊 Results:${NC}"
echo "  TODO:    $TODO_COUNT"
echo "  FIXME:   $FIXME_COUNT"
echo "  HACK:    $HACK_COUNT"
echo "  BUG:     $BUG_COUNT"
echo "  ─────────────────"
echo "  TOTAL:   $TOTAL_DEBT"
echo ""
echo -e "${BLUE}📁 Output Directory:${NC} $OUTPUT_DIR"
echo -e "${BLUE}📄 Summary Report:${NC} $SUMMARY_FILE"
echo ""
echo -e "${YELLOW}💡 Next: Review $SUMMARY_FILE and categorize P0 items${NC}"

