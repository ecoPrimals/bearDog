#!/usr/bin/env bash
#
# Validate All Receipts from Demo Runs
# Ensures all receipts are properly formatted and complete
#

set -euo pipefail

SHOWCASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Load robust functions
source "${SHOWCASE_DIR}/../lib/robust_demo_functions.sh"

# Find most recent auto-session
LATEST_SESSION=$(find "${SHOWCASE_DIR}/outputs" -maxdepth 1 -type d -name "auto-session-*" | sort -r | head -1)

if [ -z "$LATEST_SESSION" ]; then
    echo "❌ No demo sessions found"
    echo "   Run ./run-all-demos-auto.sh first"
    exit 1
fi

echo "🔍 Validating receipts from: $(basename "$LATEST_SESSION")"
echo ""

# Validate all receipts
if validate_all_receipts "$LATEST_SESSION/receipts"; then
    echo ""
    echo "✅ All receipts validated successfully!"
    exit 0
else
    echo ""
    echo "❌ Some receipts failed validation"
    exit 1
fi

