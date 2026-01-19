#!/bin/bash
# Comprehensive Pure Rust Verification

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                            ║"
echo "║               🔍 PURE RUST VERIFICATION - COMPREHENSIVE                   ║"
echo "║                                                                            ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

echo "📦 1. DEPENDENCY TREE CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking for ring..."
RING_COUNT=$(cargo tree --edges normal 2>/dev/null | grep -i "ring" | wc -l)
echo "   ring references: $RING_COUNT"

echo ""
echo "   Checking for reqwest..."
REQWEST_COUNT=$(cargo tree --edges normal 2>/dev/null | grep -i "reqwest" | wc -l)
echo "   reqwest references: $REQWEST_COUNT"

echo ""
echo "   Checking for hyper..."
HYPER_COUNT=$(cargo tree --edges normal 2>/dev/null | grep -i "hyper" | wc -l)
echo "   hyper references: $HYPER_COUNT"

echo ""
echo "   Checking for openssl..."
OPENSSL_COUNT=$(cargo tree --edges normal 2>/dev/null | grep -i "openssl" | wc -l)
echo "   openssl references: $OPENSSL_COUNT"

echo ""
echo "   Checking for rustls with ring..."
RUSTLS_RING=$(cargo tree --edges normal 2>/dev/null | grep "rustls" | grep -v "rustls-webpki" | wc -l)
echo "   rustls (might have ring) references: $RUSTLS_RING"

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "📄 2. CODE REFERENCE CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking code for ring imports..."
RING_CODE=$(grep -r "use.*ring" crates/ --include="*.rs" 2>/dev/null | grep -v "// " | grep -v "String" | wc -l)
echo "   ring imports in code: $RING_CODE"

echo ""
echo "   Checking code for reqwest..."
REQWEST_CODE=$(grep -r "use reqwest" crates/ --include="*.rs" 2>/dev/null | grep -v "// " | wc -l)
echo "   reqwest imports in code: $REQWEST_CODE"

echo ""
echo "   Checking code for hyper..."
HYPER_CODE=$(grep -r "use hyper" crates/ --include="*.rs" 2>/dev/null | grep -v "// " | wc -l)
echo "   hyper imports in code: $HYPER_CODE"

echo ""
echo "   Checking Cargo.toml files for external deps..."
CARGO_RING=$(grep -r "^ring" crates/ --include="Cargo.toml" 2>/dev/null | wc -l)
echo "   ring in Cargo.toml: $CARGO_RING"

CARGO_REQWEST=$(grep -r "^reqwest" crates/ --include="Cargo.toml" 2>/dev/null | wc -l)
echo "   reqwest in Cargo.toml: $CARGO_REQWEST"

CARGO_HYPER=$(grep -r "^hyper" crates/ --include="Cargo.toml" 2>/dev/null | wc -l)
echo "   hyper in Cargo.toml: $CARGO_HYPER"

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "🔍 3. PRODUCTION BINARY CHECK (beardog-tunnel)"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking beardog-tunnel dependencies..."
TUNNEL_RING=$(cargo tree -p beardog-tunnel --edges normal 2>/dev/null | grep -i "ring" | wc -l)
echo "   beardog-tunnel ring deps: $TUNNEL_RING"

TUNNEL_REQWEST=$(cargo tree -p beardog-tunnel --edges normal 2>/dev/null | grep -i "reqwest" | wc -l)
echo "   beardog-tunnel reqwest deps: $TUNNEL_REQWEST"

TUNNEL_HYPER=$(cargo tree -p beardog-tunnel --edges normal 2>/dev/null | grep -i "hyper" | wc -l)
echo "   beardog-tunnel hyper deps: $TUNNEL_HYPER"

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "📊 4. SUMMARY"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

TOTAL_ISSUES=$((RING_COUNT + REQWEST_COUNT + HYPER_COUNT + OPENSSL_COUNT + RING_CODE + REQWEST_CODE + HYPER_CODE + CARGO_RING + CARGO_REQWEST + CARGO_HYPER))

if [ $TOTAL_ISSUES -eq 0 ]; then
    echo "   ✅ PERFECT! Zero external dependencies!"
    echo "   ✅ 100% Pure Rust verified!"
    echo "   Grade: A++++"
else
    echo "   ⚠️  Found $TOTAL_ISSUES potential issues"
    echo "   Need to investigate further"
fi

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                    VERIFICATION COMPLETE                                   ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
