#!/usr/bin/env bash
#
# BearDog Showcase - Entry Point
# Run this script to start the showcase

set -euo pipefail

echo "🐻 BearDog Showcase"
echo "=================="
echo ""
echo "Welcome to the BearDog cryptography showcase!"
echo ""
echo "📋 Available Phases:"
echo "  ✅ Phase 1: Local Basics (READY NOW)"
echo "  📅 Phase 2: Hardware Integration (requires Solo V2 keys)"
echo "  📅 Phase 3: Network Discovery (requires 2 towers)"
echo "  📅 Phase 4: Distributed Workloads (requires full stack)"
echo ""
echo "🚀 Starting Phase 1..."
echo ""

cd 01-local-basics
exec ./demo.sh
