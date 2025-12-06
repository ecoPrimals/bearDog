#!/bin/bash
# Complete Workspace Cleanup Script
# Archives old documentation and backup code to reduce false positives

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║        🧹  WORKSPACE CLEANUP & ARCHIVE SCRIPT  🧹         ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Create archive directories
PARENT_ARCHIVE="../../archive/beardog-docs-nov-2025"
INTERNAL_ARCHIVE="./archive/deprecated-code-nov-2025"

mkdir -p "$PARENT_ARCHIVE"
mkdir -p "$INTERNAL_ARCHIVE"

echo "📦 PHASE 1: Archive Old Documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Archive old session docs (keep only nov-28-2025)
echo "📂 Archiving old session documents..."
for dir in docs/sessions/nov-{14,15,16,17,18,19,20,21}*; do
    if [ -d "$dir" ]; then
        echo "  Moving: $dir"
        mv "$dir" "$PARENT_ARCHIVE/" 2>/dev/null || true
    fi
done

# Archive old audit docs (keep only recent/relevant)
echo "📂 Archiving old audit documents..."
for dir in docs/audits/2025-11-* docs/audits/nov-{14,15,16,17,18,19,20,21,22}*; do
    if [ -d "$dir" ]; then
        echo "  Moving: $dir"
        mv "$dir" "$PARENT_ARCHIVE/" 2>/dev/null || true
    fi
done

echo ""
echo "📦 PHASE 2: Archive Deprecated/Backup Code"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Find and archive backup directories
echo "🔍 Looking for backup/archive code..."
find crates -type d \( -name "*backup*" -o -name "*_backup" -o -name "backup_*" \) 2>/dev/null | while read dir; do
    if [ -d "$dir" ]; then
        echo "  Moving: $dir"
        mv "$dir" "$INTERNAL_ARCHIVE/" 2>/dev/null || true
    fi
done

# Find and archive .old/.bak files
echo "🔍 Looking for .old and .bak files..."
find crates -type f \( -name "*.old" -o -name "*.bak" -o -name "*~" \) 2>/dev/null | while read file; do
    if [ -f "$file" ]; then
        echo "  Moving: $file"
        mv "$file" "$INTERNAL_ARCHIVE/" 2>/dev/null || true
    fi
done

echo ""
echo "📦 PHASE 3: Clean Analysis Directories"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Keep only current analysis, archive old ones
echo "🔍 Cleaning old analysis directories..."
if [ -d "target/analysis" ]; then
    echo "  Archiving: target/analysis (old)"
    mv target/analysis "$INTERNAL_ARCHIVE/" 2>/dev/null || true
fi

# Keep current: target/hardcoding-analysis/ and target/debt-analysis/
echo "  ✅ Keeping: target/hardcoding-analysis/ (current)"
echo "  ✅ Keeping: target/debt-analysis/ (current)"

echo ""
echo "📦 PHASE 4: Archive Old Action Plans"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Move old action plans to parent archive (keep only latest)
echo "📂 Archiving old action plans..."
if [ -d "docs/action-plans" ]; then
    # Archive specific old plans, keep current ones
    for file in docs/action-plans/HARDCODING_ELIMINATION_PHASE_1*.md; do
        if [ -f "$file" ]; then
            echo "  Moving: $file"
            mv "$file" "$PARENT_ARCHIVE/" 2>/dev/null || true
        fi
    done
fi

echo ""
echo "📊 PHASE 5: Generate Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Count archived items
docs_archived=$(ls "$PARENT_ARCHIVE" 2>/dev/null | wc -l)
code_archived=$(find "$INTERNAL_ARCHIVE" -type f -o -type d 2>/dev/null | wc -l)

echo "Summary:"
echo "  Documentation archived:  $docs_archived items"
echo "  Code/backups archived:   $code_archived items"
echo ""

# Current workspace status
echo "Current Workspace:"
echo "  Session docs:     $(ls -d docs/sessions/*/ 2>/dev/null | wc -l) directories"
echo "  Audit docs:       $(ls -d docs/audits/*/ 2>/dev/null | wc -l) directories"
echo "  Root reports:     $(ls *NOV_28_2025.md 2>/dev/null | wc -l) files"
echo ""

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                  ✅  CLEANUP COMPLETE  ✅                 ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "📁 Archives located at:"
echo "  - Documentation: $PARENT_ARCHIVE"
echo "  - Code/Backups:  $INTERNAL_ARCHIVE"
echo ""
echo "✨ Workspace is now clean and focused!"

