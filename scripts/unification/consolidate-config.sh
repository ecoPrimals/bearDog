#!/bin/bash
# consolidate-config.sh
# Helper script for configuration consolidation
# Usage: ./consolidate-config.sh <ConfigName> <CanonicalPath>

set -e

CONFIG_NAME=$1
CANONICAL_PATH=$2

if [ -z "$CONFIG_NAME" ] || [ -z "$CANONICAL_PATH" ]; then
    echo "Usage: ./consolidate-config.sh <ConfigName> <CanonicalPath>"
    echo "Example: ./consolidate-config.sh MonitoringConfig canonical/monitoring.rs"
    exit 1
fi

echo "🔍 Config Consolidation Helper"
echo "================================"
echo "Config: $CONFIG_NAME"
echo "Canonical: $CANONICAL_PATH"
echo ""

echo "📍 Finding all instances of $CONFIG_NAME..."
INSTANCES=$(grep -r "pub struct $CONFIG_NAME" crates/ --include="*.rs" -l 2>/dev/null || true)

if [ -z "$INSTANCES" ]; then
    echo "❌ No instances found"
    exit 1
fi

COUNT=$(echo "$INSTANCES" | wc -l)
echo "✅ Found $COUNT instances:"
echo ""
echo "$INSTANCES" | nl
echo ""

echo "📝 Consolidation Plan:"
echo "1. Keep canonical: $CANONICAL_PATH"
echo "2. Replace $((COUNT - 1)) duplicate definitions with type aliases"
echo "3. Update imports to use canonical version"
echo "4. Test build after changes"
echo ""

echo "⚠️  Remember to:"
echo "  - Create type aliases for backward compatibility"
echo "  - Test compilation after each change"
echo "  - Commit changes incrementally"
echo "  - Update documentation"
echo ""

echo "✅ Ready for consolidation" 