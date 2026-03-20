#!/bin/bash
# Test Coverage Expansion Plan
# Generated: December 6, 2025
# Goal: Increase coverage from 78.18% to 90%

set -e

echo "🎯 BearDog Test Coverage Expansion"
echo "=================================="
echo ""

# Current baseline
echo "📊 Current Coverage: 78.18%"
echo "🎯 Target Coverage: 90%"
echo "📈 Gap: 11.82%"
echo ""

# Priority areas
echo "🔍 Priority Test Areas:"
echo "1. AI Hybrid Intelligence (~40% → 80%)"
echo "2. Genetic Algorithms (~70% → 90%)"
echo "3. Network Resilience (~75% → 90%)"
echo "4. HSM Provider Paths (~80% → 95%)"
echo ""

# Generate current coverage report
echo "📊 Generating current coverage report..."
cargo llvm-cov --html --output-dir target/coverage 2>&1 | tail -10

echo ""
echo "✅ Coverage report generated at: target/coverage/index.html"
echo ""
echo "🎯 Next Steps:"
echo "1. Open coverage report: firefox target/coverage/index.html"
echo "2. Identify red/yellow areas"
echo "3. Write tests for uncovered paths"
echo "4. Re-run to verify improvement"
echo ""
echo "📝 Focus on these modules first:"
echo "   - crates/beardog-core/src/ai/"
echo "   - crates/beardog-genetics/src/"
echo "   - crates/beardog-tunnel/src/tunnel/hsm/"
echo ""

