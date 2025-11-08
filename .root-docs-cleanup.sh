#!/usr/bin/env bash
# Root Documentation Cleanup Script
# Created: November 7, 2025
# Purpose: Organize and archive legacy documentation

set -e

echo "🧹 BearDog Root Documentation Cleanup"
echo "======================================"
echo ""

# Create archive directory
ARCHIVE_DIR="docs/archive/legacy-pre-nov-7-2025"
mkdir -p "$ARCHIVE_DIR"
echo "✅ Archive directory created: $ARCHIVE_DIR"
echo ""

# Define files to archive (legacy files superseded by Nov 7 review)
LEGACY_FILES=(
    "FINAL_STATUS.md"
    "REVIEW_COMPLETE.md"
    "READY_TO_SHIP.md"
    "NEXT_SESSION_START_HERE.md"
    "START_HERE_UNIFICATION_SESSION.md"
    "CLEANUP_FINAL_REPORT.txt"
    "DOCUMENTATION_CLEANUP_COMPLETE.md"
    "DOCS_ORGANIZED.md"
)

echo "📦 Archiving legacy files..."
echo ""

ARCHIVED_COUNT=0
MISSING_COUNT=0

for file in "${LEGACY_FILES[@]}"; do
    if [ -f "$file" ]; then
        mv "$file" "$ARCHIVE_DIR/"
        echo "  ✅ Archived: $file"
        ((ARCHIVED_COUNT++))
    else
        echo "  ℹ️  Not found: $file (skipping)"
        ((MISSING_COUNT++))
    fi
done

echo ""
echo "📊 Summary:"
echo "  - Files archived: $ARCHIVED_COUNT"
echo "  - Files not found: $MISSING_COUNT"
echo "  - Archive location: $ARCHIVE_DIR"
echo ""

# Create index in archive
cat << 'ARCHIVE_README' > "$ARCHIVE_DIR/README.md"
# Legacy Documentation Archive (Pre-Nov 7, 2025)

**Archived**: November 7, 2025  
**Reason**: Superseded by comprehensive review

These files are kept for historical reference but are no longer current.

## Superseded By:
- [README_REVIEW_NOV_7_2025.md](../../README_REVIEW_NOV_7_2025.md)
- [COMPREHENSIVE_UNIFICATION_REVIEW_NOV_7_2025.md](../../COMPREHENSIVE_UNIFICATION_REVIEW_NOV_7_2025.md)

## Archived Files:
- Status reports from previous sessions
- Old completion reports
- Superseded documentation indexes

**Note**: All information from these files is incorporated into the Nov 7, 2025 review.
ARCHIVE_README

echo "✅ Archive index created: $ARCHIVE_DIR/README.md"
echo ""

# List current root docs
echo "📚 Current Root Documentation:"
echo ""
echo "**Start Here** (NEW - November 7, 2025):"
ls -lh README_REVIEW_NOV_7_2025.md 2>/dev/null || echo "  ⚠️  README_REVIEW_NOV_7_2025.md missing!"
ls -lh QUICK_ACTION_GUIDE_NOV_7_2025.md 2>/dev/null || echo "  ⚠️  QUICK_ACTION_GUIDE_NOV_7_2025.md missing!"
ls -lh UNIFICATION_STATUS_FINAL_NOV_7_2025.md 2>/dev/null || echo "  ⚠️  UNIFICATION_STATUS_FINAL_NOV_7_2025.md missing!"
echo ""

echo "**Core Documentation** (Still Current):"
ls -lh README.md 00_START_HERE.md ARCHITECTURE.md CURRENT_STATUS.md 2>/dev/null | head -5 || true
echo ""

echo "✅ Root documentation cleanup complete!"
echo ""
echo "📖 Next steps:"
echo "   1. Read: README_REVIEW_NOV_7_2025.md"
echo "   2. Review: DOCUMENTATION_INDEX_UPDATED_NOV_7_2025.md"
echo "   3. Archive location: $ARCHIVE_DIR"
echo ""
echo "🚀 BearDog documentation is now organized!"
