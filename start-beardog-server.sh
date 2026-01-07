#!/bin/bash
# Start BearDog Server v0.15.0 with BirdSong v2 API

set -e

cd "$(dirname "$0")"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐻🐕 BearDog Server v0.15.0 Startup Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Configuration
BIND_ADDR="${BEARDOG_BIND_ADDR:-127.0.0.1:9000}"
HSM_MODE="${BEARDOG_HSM_MODE:-software}"
LOG_LEVEL="${RUST_LOG:-info}"

echo "📋 Configuration:"
echo "   Bind Address:  $BIND_ADDR"
echo "   HSM Mode:      $HSM_MODE"
echo "   Log Level:     $LOG_LEVEL"
echo ""

# Export environment
export BEARDOG_BIND_ADDR="$BIND_ADDR"
export BEARDOG_HSM_MODE="$HSM_MODE"
export RUST_LOG="$LOG_LEVEL"

# Check if binary exists
if [ ! -f "primalBins/beardog-server-v0.15.0-with-v2-api" ]; then
    echo "❌ Error: Binary not found at primalBins/beardog-server-v0.15.0-with-v2-api"
    echo ""
    echo "To build the binary:"
    echo "  cargo build --release --bin beardog-server"
    echo "  mkdir -p primalBins"
    echo "  cp target/release/beardog-server primalBins/beardog-server-v0.15.0-with-v2-api"
    exit 1
fi

echo "🚀 Starting BearDog Server..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start server
exec ./primalBins/beardog-server-v0.15.0-with-v2-api

