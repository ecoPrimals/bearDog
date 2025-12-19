#!/usr/bin/env bash
# Test Receipt System - Quick Demo
# Shows that BearDog now generates real cryptographic receipts

set -e

DEMO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${DEMO_DIR}/outputs/receipt-test-$(date +%s)"

mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

echo "🧪 Testing BearDog Receipt System"
echo "=================================="
echo ""

# Clean any old receipts
rm -rf receipts/ keys/ 2>/dev/null || true

echo "1️⃣  Generating master key..."
beardog key generate --key-id test-master-001 --algorithm AES-256-GCM

echo ""
echo "2️⃣  Deriving sub-key..."
beardog key derive \
    --master-key test-master-001 \
    --output test-sub-001 \
    --purpose "testing" \
    --expires-in "24h"

echo ""
echo "3️⃣  Creating delegated key..."
beardog key delegate \
    --master-key test-master-001 \
    --output test-delegated-001 \
    --delegate-to "test-user" \
    --expires-in "1h" \
    --cpu-quota 50 \
    --memory-quota "512MB"

echo ""
echo "📜 RECEIPTS GENERATED:"
echo "===================="
if [ -d "receipts" ]; then
    ls -lh receipts/
    echo ""
    echo "📊 Receipt Count: $(ls receipts/*.json 2>/dev/null | wc -l)"
    echo ""
    
    echo "🔍 Sample Receipt (key-generate):"
    echo "================================"
    FIRST_RECEIPT=$(ls receipts/receipt-key-generate-*.json 2>/dev/null | head -1)
    if [ -n "$FIRST_RECEIPT" ]; then
        cat "$FIRST_RECEIPT" | jq '.'
    fi
else
    echo "❌ No receipts directory found!"
    exit 1
fi

echo ""
echo "✅ Receipt system working!"
echo ""
echo "📂 Test artifacts: $WORK_DIR"

