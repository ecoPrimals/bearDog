#!/bin/bash
# track-progress.sh
# Tracks configuration consolidation progress

set -e

BASELINE=958
CURRENT_COUNT=$(grep -r "pub struct.*Config" crates/ --include="*.rs" 2>/dev/null | wc -l)
REDUCTION=$((BASELINE - CURRENT_COUNT))
PERCENT=$((REDUCTION * 100 / BASELINE))

echo "📊 Consolidation Progress Report"
echo "================================="
echo ""
echo "📈 Configuration Structs:"
echo "   Baseline:  $BASELINE configs"
echo "   Current:   $CURRENT_COUNT configs"
echo "   Reduced:   $REDUCTION configs (-$PERCENT%)"
echo ""

# Calculate progress bar
PROGRESS=$((PERCENT * 30 / 100))
REMAINING=$((30 - PROGRESS))
BAR=$(printf '█%.0s' $(seq 1 $PROGRESS))
SPACE=$(printf '░%.0s' $(seq 1 $REMAINING))

echo "   Progress:  [$BAR$SPACE] $PERCENT%"
echo ""

# Week targets
echo "🎯 Week Targets:"
echo "   Week 2: 698 configs (-18%)"
echo "   Week 3: 298 configs (-65%)"
echo "   Week 4:  50 configs (-94%)"
echo ""

if [ $CURRENT_COUNT -le 698 ]; then
    echo "✅ Week 2 target achieved!"
elif [ $CURRENT_COUNT -le 786 ]; then
    echo "🔄 On track for Week 2 target"
else
    echo "⏳ Week 2 in progress"
fi

echo ""
echo "📅 Last updated: $(date '+%Y-%m-%d %H:%M:%S')" 