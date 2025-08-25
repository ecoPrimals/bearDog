#!/bin/bash
set -e

# BearDog Comprehensive Test Coverage Report
# Generates detailed coverage metrics and analysis

echo "🔍 BearDog Test Coverage Analysis"
echo "=================================="
echo

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "❌ cargo-tarpaulin not found. Installing..."
    cargo install cargo-tarpaulin
fi

# Create coverage directory
mkdir -p coverage/reports
mkdir -p coverage/html

echo "📊 Running comprehensive test coverage analysis..."
echo

# Generate coverage with detailed output
cargo tarpaulin \
    --verbose \
    --all-features \
    --workspace \
    --timeout 120 \
    --exclude-files "*/target/*" \
    --exclude-files "*/tests/*" \
    --exclude-files "*/benches/*" \
    --exclude-files "*/examples/*" \
    --out Html \
    --out Lcov \
    --out Json \
    --output-dir coverage/ \
    --engine llvm \
    --follow-exec \
    --rustflags="-Cinstrument-coverage" \
    2>&1 | tee coverage/coverage.log

echo
echo "📈 Coverage Summary:"
echo "==================="

# Extract and display key metrics
if [ -f "coverage/tarpaulin-report.json" ]; then
    # Parse coverage percentage from JSON
    COVERAGE=$(cat coverage/tarpaulin-report.json | grep -o '"coverage":[0-9.]*' | head -1 | cut -d':' -f2)
    echo "📊 Overall Coverage: $COVERAGE%"
    
    # Check if we meet our 90% target
    if (( $(echo "$COVERAGE >= 90.0" | bc -l) )); then
        echo "✅ Coverage target MET (≥90%)"
    else
        echo "❌ Coverage target MISSED (<90%)"
        echo "   Target: 90%"
        echo "   Actual: $COVERAGE%"
        echo "   Gap: $(echo "90.0 - $COVERAGE" | bc -l)%"
    fi
else
    echo "❌ Coverage report not generated"
fi

echo
echo "📁 Generated Reports:"
echo "===================="
echo "   HTML Report: coverage/tarpaulin-report.html"
echo "   LCOV Report: coverage/lcov.info"
echo "   JSON Report: coverage/tarpaulin-report.json"
echo "   Full Log: coverage/coverage.log"

echo
echo "🔗 Open HTML report:"
echo "   file://$(pwd)/coverage/tarpaulin-report.html"

echo
echo "📋 Coverage Analysis Complete!" 