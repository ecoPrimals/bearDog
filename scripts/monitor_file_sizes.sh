#!/usr/bin/env bash
# File Size Monitoring Script for BearDog
# Alerts when files approach the 2000 line limit

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
THRESHOLD_WARNING=1800  # Warn at 1800 lines
THRESHOLD_CRITICAL=1900 # Critical at 1900 lines
THRESHOLD_MAX=2000      # Max limit

echo "🔍 BearDog File Size Monitor"
echo "================================"
echo "Workspace: $WORKSPACE_ROOT"
echo "Thresholds: Warning=${THRESHOLD_WARNING}, Critical=${THRESHOLD_CRITICAL}, Max=${THRESHOLD_MAX}"
echo ""

# Find all Rust files and count lines
files_warning=0
files_critical=0
files_compliant=0

echo "📏 Scanning Rust files..."
echo ""

# Create temporary file for results
temp_file=$(mktemp)

find "$WORKSPACE_ROOT/crates" -name "*.rs" -type f | while read -r file; do
    line_count=$(wc -l < "$file")
    relative_path="${file#$WORKSPACE_ROOT/}"
    
    if [ "$line_count" -ge "$THRESHOLD_MAX" ]; then
        echo "🚨 EXCEEDS_MAX|$line_count|$relative_path" >> "$temp_file"
    elif [ "$line_count" -ge "$THRESHOLD_CRITICAL" ]; then
        echo "⚠️  CRITICAL|$line_count|$relative_path" >> "$temp_file"
    elif [ "$line_count" -ge "$THRESHOLD_WARNING" ]; then
        echo "⚠️  WARNING|$line_count|$relative_path" >> "$temp_file"
    fi
done

# Display results
if [ -f "$temp_file" ] && [ -s "$temp_file" ]; then
    echo "⚠️  Files Requiring Attention:"
    echo "================================"
    
    # Sort by line count (descending)
    sort -t'|' -k2 -rn "$temp_file" | while IFS='|' read -r status lines path; do
        printf "%s %5d lines: %s\n" "$status" "$lines" "$path"
        
        case "$status" in
            *EXCEEDS_MAX*)
                ((files_critical++)) || true
                ;;
            *CRITICAL*)
                ((files_critical++)) || true
                ;;
            *WARNING*)
                ((files_warning++)) || true
                ;;
        esac
    done
    
    echo ""
    echo "📊 Summary:"
    echo "  Critical (≥${THRESHOLD_CRITICAL}): $files_critical files"
    echo "  Warning (≥${THRESHOLD_WARNING}): $files_warning files"
    
    # Return non-zero if critical files found
    if [ "$files_critical" -gt 0 ]; then
        echo ""
        echo "❌ CRITICAL: Some files are approaching or exceeding the 2000 line limit!"
        rm -f "$temp_file"
        exit 1
    elif [ "$files_warning" -gt 0 ]; then
        echo ""
        echo "⚠️  WARNING: Some files are approaching the 2000 line limit. Consider monitoring or splitting."
        rm -f "$temp_file"
        exit 0
    fi
else
    echo "✅ All files are well within the 2000 line limit!"
    echo "   (No files exceed ${THRESHOLD_WARNING} lines)"
fi

rm -f "$temp_file"

echo ""
echo "✅ File size monitoring complete" 