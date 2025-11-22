#!/bin/bash
# TODO Cleanup Analysis Script
# Usage: ./scripts/todo-cleanup.sh [command]
# Commands: critical, phase2, test, obsolete, stats, all

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Colors
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}\n"
}

print_critical() {
    echo -e "${RED}🔴 $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}🟡 $1${NC}"
}

print_info() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Critical TODOs - Blocking issues
show_critical() {
    print_section "CRITICAL TODOS (Module Blockers)"
    
    print_critical "1. iOS Secure Enclave Module - DISABLED"
    rg "TEMPORARILY DISABLED.*iOS|TODO.*Re-enable.*iOS" \
        --type rust \
        --context 2 \
        crates/beardog-tunnel/src/tunnel/hsm/mod.rs || true
    
    print_critical "2. Android StrongBox - CORRUPTED"
    rg "TODO.*Fix corruption|TODO.*corruption" \
        --type rust \
        --context 2 \
        crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/ || true
    
    print_critical "3. Mobile Setup - TYPE MISMATCH"
    rg "TODO.*Re-enable after fixing type|TODO.*type mismatch" \
        --type rust \
        --context 2 \
        crates/beardog-tunnel/src/tunnel/hsm/mobile_setup.rs || true
    
    echo ""
    print_info "Action: Fix these 3 issues first (Est: 10 hours)"
}

# Phase 2 TODOs - Planned features
show_phase2() {
    print_section "PHASE 2 TODOS (Planned Features)"
    
    print_warning "FIDO2 / CTAP2 Implementation"
    rg "TODO.*Implement actual CTAP2|TODO.*CTAP2" \
        --type rust \
        --files-with-matches \
        crates/beardog-security/src/hsm/fido2/ | head -10 || true
    
    echo ""
    print_warning "Cloud Provider Detection"
    rg "TODO.*Implement.*cloud KMS|TODO.*cloud" \
        --type rust \
        --files-with-matches \
        crates/beardog-tunnel/src/universal_hsm_discovery/ | head -10 || true
    
    echo ""
    print_warning "Songbird Integration"
    rg "TODO.*Implement actual Songbird|TODO.*Songbird" \
        --type rust \
        --context 1 \
        crates/beardog-core/src/ecosystem_integration/ | head -20 || true
    
    echo ""
    print_info "Action: These are Phase 2 features - add to roadmap"
}

# Test TODOs
show_test_todos() {
    print_section "TEST TODOS (Coverage Improvements)"
    
    print_warning "Test Placeholders"
    rg "TODO.*Add real.*test|TODO.*test" \
        --type rust \
        --ignore-case \
        --files-with-matches \
        crates/ tests/ | head -20 || true
    
    echo ""
    print_info "Action: Address during test coverage sprints"
}

# Potentially obsolete TODOs
show_obsolete() {
    print_section "POTENTIALLY OBSOLETE TODOS"
    
    print_warning "May already be completed:"
    rg "TODO.*Default configuration|TODO.*sensible values|TODO.*should have" \
        --type rust \
        --context 2 || true
    
    echo ""
    print_info "Action: Review and remove if complete"
}

# Statistics
show_stats() {
    print_section "TODO STATISTICS"
    
    echo "Total TODOs by type:"
    echo -n "  TODO:  "
    rg "TODO:" --type rust | wc -l || echo "0"
    echo -n "  FIXME: "
    rg "FIXME:" --type rust | wc -l || echo "0"
    echo -n "  XXX:   "
    rg "XXX:" --type rust | wc -l || echo "0"
    echo -n "  HACK:  "
    rg "HACK:" --type rust | wc -l || echo "0"
    
    echo ""
    echo "TODOs by component:"
    for crate in beardog-tunnel beardog-security beardog-core beardog-types; do
        count=$(rg "TODO:" --type rust "crates/$crate/" 2>/dev/null | wc -l || echo "0")
        printf "  %-20s: %s\n" "$crate" "$count"
    done
    
    echo ""
    echo "Critical keywords:"
    echo -n "  'Re-enable': "
    rg "TODO.*Re-enable" --type rust | wc -l || echo "0"
    echo -n "  'Fix':       "
    rg "TODO.*Fix|FIXME:" --type rust | wc -l || echo "0"
    echo -n "  'Implement': "
    rg "TODO.*Implement" --type rust | wc -l || echo "0"
    
    echo ""
    echo "Context:"
    echo -n "  Production code: "
    rg "TODO:" --type rust crates/ | grep -v "/tests/" | grep -v "_tests.rs" | wc -l || echo "0"
    echo -n "  Test code:       "
    rg "TODO:" --type rust crates/ tests/ | grep -E "/tests/|_tests.rs" | wc -l || echo "0"
}

# Export TODOs to file
export_todos() {
    local output_file="$REPO_ROOT/TODO_REPORT_$(date +%Y%m%d).txt"
    
    print_section "Exporting TODOs to $output_file"
    
    {
        echo "BearDog TODO Report - $(date)"
        echo "================================"
        echo ""
        
        echo "CRITICAL (Blocking):"
        rg "TODO.*Re-enable|TODO.*Fix corruption|TODO.*type mismatch" \
            --type rust --context 1 || true
        
        echo ""
        echo "PHASE 2 (Planned):"
        rg "TODO.*CTAP2|TODO.*Songbird|TODO.*cloud KMS" \
            --type rust --files-with-matches || true
        
        echo ""
        echo "ALL TODOs by file:"
        rg "TODO:|FIXME:|XXX:|HACK:" --type rust --heading || true
        
    } > "$output_file"
    
    print_info "Exported to: $output_file"
}

# Main command dispatcher
case "${1:-all}" in
    critical)
        show_critical
        ;;
    phase2)
        show_phase2
        ;;
    test)
        show_test_todos
        ;;
    obsolete)
        show_obsolete
        ;;
    stats)
        show_stats
        ;;
    export)
        export_todos
        ;;
    all)
        show_critical
        show_phase2
        show_test_todos
        show_obsolete
        show_stats
        ;;
    *)
        echo "Usage: $0 {critical|phase2|test|obsolete|stats|export|all}"
        exit 1
        ;;
esac

echo ""
print_info "Done! See docs/audits/TODO_CLEANUP_ANALYSIS_NOV_12_2025.md for full analysis"

