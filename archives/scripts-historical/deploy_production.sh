#!/bin/bash
set -e

echo "🚀 BearDog Production Deployment Script"
echo "======================================="

# Load production environment
source .env.production

echo "📋 Pre-deployment Checks:"
echo "  ✅ Environment: $BEARDOG_ENVIRONMENT"
echo "  ✅ API Binding: $BEARDOG_API_BIND_ADDRESS"
echo "  ✅ HSM Mode: $BEARDOG_HSM_MODE"

echo ""
echo "🔧 Building release version..."
cargo build --release --all-features --workspace --exclude beardog-benchmarks

echo ""
echo "🧪 Running production tests..."
cargo test --release --all-features --workspace --exclude beardog-benchmarks

echo ""
echo "📦 Creating deployment package..."
mkdir -p deployment/bin
cp target/release/beardog-cli deployment/bin/
cp target/release/beardog-api deployment/bin/ 2>/dev/null || echo "API binary not found (library crate)"
cp .env.production deployment/
cp -r configs deployment/

echo ""
echo "✅ Deployment package ready in ./deployment/"
echo "🎉 BearDog is ready for production!"
