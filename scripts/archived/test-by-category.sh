#!/bin/bash
# Test Runner by Category
# Created: October 26, 2025

set -e

CATEGORY="${1:-all}"

echo "🧪 Running BearDog Tests by Category: $CATEGORY"
echo "================================================"

case "$CATEGORY" in
    "unit")
        echo "Running Unit Tests..."
        cargo test --lib --bins
        ;;
    
    "integration")
        echo "Running Integration Tests..."
        cargo test --tests -- --skip e2e:: --skip chaos::
        ;;
    
    "e2e")
        echo "Running E2E Tests..."
        cargo test --test '*' e2e::
        cargo test --tests e2e_
        ;;
    
    "chaos")
        echo "Running Chaos Tests..."
        cargo test --test '*' chaos::
        cargo test --tests chaos_
        ;;
    
    "security")
        echo "Running Security Tests..."
        cargo test --lib --tests -- security
        cargo test --lib --tests -- crypto
        cargo test --lib --tests -- auth
        ;;
    
    "fast")
        echo "Running Fast Tests Only..."
        cargo test --lib --bins -- --skip slow
        ;;
    
    "all")
        echo "Running All Tests..."
        cargo test --workspace --all-targets
        ;;
    
    *)
        echo "Unknown category: $CATEGORY"
        echo ""
        echo "Available categories:"
        echo "  unit         - Unit tests only"
        echo "  integration  - Integration tests"
        echo "  e2e          - End-to-end tests"
        echo "  chaos        - Chaos/fault injection tests"
        echo "  security     - Security-related tests"
        echo "  fast         - Fast tests only (exclude slow)"
        echo "  all          - All tests (default)"
        exit 1
        ;;
esac

echo ""
echo "✅ Tests completed for category: $CATEGORY"

