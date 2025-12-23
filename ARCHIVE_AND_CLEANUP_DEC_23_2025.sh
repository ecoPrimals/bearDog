#!/bin/bash
# Archive and Cleanup Script - December 23, 2025
# Moves old docs, archives, and backup code to parent ../archive for fossil record

set -e

echo "🧹 BearDog Workspace Cleanup - December 23, 2025"
echo "================================================"
echo ""

# Define colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the parent archive directory
PARENT_ARCHIVE="/home/eastgate/Development/ecoPrimals/archive"
BEARDOG_ARCHIVE="${PARENT_ARCHIVE}/beardog-dec-23-2025"

echo -e "${BLUE}Creating archive directory structure...${NC}"
mkdir -p "${BEARDOG_ARCHIVE}/docs"
mkdir -p "${BEARDOG_ARCHIVE}/showcase"
mkdir -p "${BEARDOG_ARCHIVE}/old-status-reports"
mkdir -p "${BEARDOG_ARCHIVE}/old-session-reports"
mkdir -p "${BEARDOG_ARCHIVE}/backup-code"

echo ""
echo -e "${YELLOW}Phase 1: Archive old documentation${NC}"
echo "-----------------------------------"

# Archive docs/archive (already archived docs)
if [ -d "docs/archive" ]; then
    echo "Moving docs/archive to parent archive..."
    mv docs/archive "${BEARDOG_ARCHIVE}/docs/archive-old"
fi

# Archive old session reports from docs
if [ -d "docs/session-reports" ]; then
    echo "Moving docs/session-reports..."
    mv docs/session-reports "${BEARDOG_ARCHIVE}/docs/"
fi

if [ -d "docs/sessions" ]; then
    echo "Moving docs/sessions..."
    mv docs/sessions "${BEARDOG_ARCHIVE}/docs/"
fi

# Archive old audits (keep latest)
if [ -d "docs/audits" ]; then
    echo "Archiving old audits (keeping README)..."
    mkdir -p "${BEARDOG_ARCHIVE}/docs/audits"
    # Keep the latest comprehensive audit, archive older ones
    find docs/audits -name "*.md" -not -name "README.md" -not -name "COMPREHENSIVE_AUDIT_REPORT_DEC_17_2025.md" -exec mv {} "${BEARDOG_ARCHIVE}/docs/audits/" \;
fi

echo ""
echo -e "${YELLOW}Phase 2: Archive showcase materials${NC}"
echo "------------------------------------"

# Archive showcase (demos and validation reports)
if [ -d "showcase" ]; then
    echo "Moving showcase directory (keeping as fossil record)..."
    mv showcase "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 3: Archive old status reports${NC}"
echo "------------------------------------"

# Archive old status/bootstrap reports (keep current ones)
echo "Moving old Genesis bootstrap reports..."
[ -f "GENESIS_BOOTSTRAP_STATUS_DEC_22_2025.md" ] && mv GENESIS_BOOTSTRAP_STATUS_DEC_22_2025.md "${BEARDOG_ARCHIVE}/old-status-reports/"
[ -f "GENESIS_BOOTSTRAP_WEEK1_COMPLETE_DEC_22_2025.md" ] && mv GENESIS_BOOTSTRAP_WEEK1_COMPLETE_DEC_22_2025.md "${BEARDOG_ARCHIVE}/old-status-reports/"
[ -f "GENESIS_BOOTSTRAP_WEEK2_INTEGRATION_DEC_22_2025.md" ] && mv GENESIS_BOOTSTRAP_WEEK2_INTEGRATION_DEC_22_2025.md "${BEARDOG_ARCHIVE}/old-status-reports/"
[ -f "MIXED_ENTROPY_HANDOFF_SUMMARY.md" ] && mv MIXED_ENTROPY_HANDOFF_SUMMARY.md "${BEARDOG_ARCHIVE}/old-status-reports/"
[ -f "SHOWCASE_BTSP_BIRDSONG_VALIDATION_COMPLETE.md" ] && mv SHOWCASE_BTSP_BIRDSONG_VALIDATION_COMPLETE.md "${BEARDOG_ARCHIVE}/old-status-reports/"

echo ""
echo -e "${YELLOW}Phase 4: Clean up old scripts and tools${NC}"
echo "----------------------------------------"

# Archive old validation scripts
if [ -f "FINAL_VALIDATION_SUMMARY.sh" ]; then
    echo "Moving old validation scripts..."
    mv FINAL_VALIDATION_SUMMARY.sh "${BEARDOG_ARCHIVE}/"
fi

if [ -f "CLEANUP_WORKSPACE.sh" ]; then
    echo "Moving old cleanup script..."
    mv CLEANUP_WORKSPACE.sh "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 5: Archive receipts and logs${NC}"
echo "-----------------------------------"

# Archive receipts (transaction records)
if [ -d "receipts" ]; then
    echo "Moving receipts directory..."
    mv receipts "${BEARDOG_ARCHIVE}/"
fi

# Archive audit.log if exists
if [ -f "audit.log" ]; then
    echo "Moving audit.log..."
    mv audit.log "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 6: Archive old planning docs${NC}"
echo "-----------------------------------"

# Archive old planning docs (keep current roadmap)
if [ -d "planning" ]; then
    echo "Moving old planning directory..."
    mv planning "${BEARDOG_ARCHIVE}/"
fi

# Archive old features docs
if [ -d "features" ]; then
    echo "Moving features directory..."
    mv features "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 7: Clean up test artifacts${NC}"
echo "---------------------------------"

# Clean up any test artifacts
echo "Removing test artifacts..."
find . -name "*.profraw" -type f -delete 2>/dev/null || true
find . -name "*.profdata" -type f -delete 2>/dev/null || true

echo ""
echo -e "${YELLOW}Phase 8: Archive old guides (keep essential)${NC}"
echo "---------------------------------------------"

# Archive old guides that are superseded
if [ -d "guides" ]; then
    echo "Moving old guides directory..."
    mv guides "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 9: Archive testing directory${NC}"
echo "-----------------------------------"

# Archive testing directory (keep tests/ for actual test code)
if [ -d "testing" ]; then
    echo "Moving testing docs directory..."
    mv testing "${BEARDOG_ARCHIVE}/"
fi

echo ""
echo -e "${YELLOW}Phase 10: Clean up duplicate documentation${NC}"
echo "-------------------------------------------"

# Archive old integration guides (keep latest)
[ -f "GENESIS_INTEGRATION_GUIDE_FOR_SONGBIRD.md" ] && mv GENESIS_INTEGRATION_GUIDE_FOR_SONGBIRD.md "${BEARDOG_ARCHIVE}/old-status-reports/"

echo ""
echo -e "${GREEN}✅ Cleanup Complete!${NC}"
echo ""
echo "Summary:"
echo "--------"
echo "Archived to: ${BEARDOG_ARCHIVE}"
echo ""
echo "What was archived:"
echo "  ✓ Old documentation archives"
echo "  ✓ Showcase materials (demos, validation)"
echo "  ✓ Old status reports"
echo "  ✓ Old session reports"
echo "  ✓ Planning documents"
echo "  ✓ Features documentation"
echo "  ✓ Old guides"
echo "  ✓ Testing documentation"
echo "  ✓ Receipts and logs"
echo "  ✓ Old scripts"
echo ""
echo "What remains (clean workspace):"
echo "  ✓ Core documentation (docs/)"
echo "  ✓ Source code (crates/, src/)"
echo "  ✓ Tests (tests/)"
echo "  ✓ Examples (examples/)"
echo "  ✓ Specs (specs/)"
echo "  ✓ Current status docs (README, STATUS, etc.)"
echo "  ✓ Latest audit reports"
echo "  ✓ Configuration (configs/)"
echo ""
echo -e "${BLUE}Next: Review changes, then commit and push${NC}"
echo ""

