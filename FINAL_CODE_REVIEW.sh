#!/bin/bash
# Final Code Review - Find Cleanable Items

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                            ║"
echo "║               🔍 FINAL CODE REVIEW - COMPREHENSIVE                        ║"
echo "║                                                                            ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

echo "📝 1. OUTDATED TODO CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking for outdated TODOs mentioning removed features..."
echo ""

echo "   TODOs mentioning 'reqwest':"
REQWEST_TODOS=$(grep -r "TODO.*reqwest" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $REQWEST_TODOS"
if [ $REQWEST_TODOS -gt 0 ]; then
    grep -r "TODO.*reqwest" crates/ --include="*.rs" 2>/dev/null | head -5
fi

echo ""
echo "   TODOs mentioning 'hyper':"
HYPER_TODOS=$(grep -r "TODO.*hyper" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $HYPER_TODOS"

echo ""
echo "   TODOs mentioning 'HTTP' (uppercase - might be outdated):"
HTTP_TODOS=$(grep -r "TODO.*HTTP" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $HTTP_TODOS"
if [ $HTTP_TODOS -gt 0 ]; then
    grep -r "TODO.*HTTP" crates/ --include="*.rs" 2>/dev/null | head -5
fi

echo ""
echo "   TODOs mentioning 'Consul' or 'etcd':"
CONSUL_TODOS=$(grep -r "TODO.*Consul\|TODO.*etcd" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $CONSUL_TODOS"
if [ $CONSUL_TODOS -gt 0 ]; then
    grep -r "TODO.*Consul\|TODO.*etcd" crates/ --include="*.rs" 2>/dev/null | head -5
fi

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "📦 2. COMMENTED OUT CODE CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking for commented-out use statements..."
COMMENTED_USE=$(grep -r "^[\s]*// use" crates/ --include="*.rs" 2>/dev/null | grep -v "// use case\|// user\|// useful\|// used" | wc -l)
echo "   Found: $COMMENTED_USE commented 'use' statements"

echo ""
echo "   Checking for large commented code blocks..."
COMMENTED_BLOCKS=$(grep -r "^[\s]*//" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Total comment lines: $COMMENTED_BLOCKS (includes docs, need manual review)"

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "🔧 3. DEPRECATED MARKERS CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking for #[deprecated] attributes..."
DEPRECATED_ATTRS=$(grep -r "#\[deprecated" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $DEPRECATED_ATTRS"

echo ""
echo "   Checking for 'DEPRECATED' comments..."
DEPRECATED_COMMENTS=$(grep -r "DEPRECATED" crates/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Found: $DEPRECATED_COMMENTS"
if [ $DEPRECATED_COMMENTS -gt 0 ]; then
    grep -r "DEPRECATED" crates/ --include="*.rs" 2>/dev/null | head -5
fi

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "🗑️ 4. UNUSED IMPORTS CHECK"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "   Checking for unused imports (compiler warnings)..."
echo "   Running: cargo check 2>&1 | grep 'unused import' | wc -l"
UNUSED_IMPORTS=$(cargo check 2>&1 | grep "unused import" | wc -l)
echo "   Found: $UNUSED_IMPORTS unused imports"

echo ""
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

echo "📊 SUMMARY"
echo "   ════════════════════════════════════════════════════════════════════════"
echo ""

TOTAL_ISSUES=$((REQWEST_TODOS + HYPER_TODOS + HTTP_TODOS + CONSUL_TODOS + DEPRECATED_ATTRS + UNUSED_IMPORTS))

echo "   Potential cleanable items:"
echo "   - Outdated TODOs (reqwest): $REQWEST_TODOS"
echo "   - Outdated TODOs (hyper): $HYPER_TODOS"
echo "   - Outdated TODOs (HTTP): $HTTP_TODOS"
echo "   - Outdated TODOs (Consul/etcd): $CONSUL_TODOS"
echo "   - Deprecated attributes: $DEPRECATED_ATTRS"
echo "   - Unused imports: $UNUSED_IMPORTS"
echo "   - Commented use statements: $COMMENTED_USE"
echo "   - DEPRECATED comments: $DEPRECATED_COMMENTS"
echo ""
echo "   TOTAL: $TOTAL_ISSUES items to review"

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                    REVIEW COMPLETE                                         ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
