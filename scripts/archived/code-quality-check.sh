#!/bin/bash
# BearDog Code Quality Check Script
# Purpose: Run comprehensive quality checks on the codebase
# Created: November 7, 2025

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ ${NC}$1"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${NC}$1"
}

print_error() {
    echo -e "${RED}✗ ${NC}$1"
}

print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

# Check file sizes
check_file_sizes() {
    print_header "Checking File Sizes"
    
    local max_lines=2000
    local over_limit=0
    
    while IFS= read -r file; do
        local lines=$(wc -l < "$file")
        if [ "$lines" -gt "$max_lines" ]; then
            print_error "$(basename "$file"): $lines lines (exceeds $max_lines)"
            over_limit=$((over_limit + 1))
        fi
    done < <(find "$PROJECT_ROOT/crates" -name "*.rs" -type f)
    
    if [ "$over_limit" -eq 0 ]; then
        print_success "All files under $max_lines lines! ✨"
    else
        print_warning "$over_limit file(s) exceed $max_lines lines"
    fi
    
    # Show largest files
    echo ""
    print_info "Top 5 largest files:"
    find "$PROJECT_ROOT/crates" -name "*.rs" -type f -exec wc -l {} \; | \
        sort -rn | head -5 | \
        while read -r lines file; do
            local rel_file=$(echo "$file" | sed "s|$PROJECT_ROOT/||")
            printf "  %5d lines  %s\n" "$lines" "$rel_file"
        done
}

# Check for TODOs and FIXMEs
check_todos() {
    print_header "Checking TODOs and FIXMEs"
    
    local todo_count=$(grep -r "TODO\|FIXME\|XXX\|HACK" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)
    
    if [ "$todo_count" -eq 0 ]; then
        print_success "No TODOs found! ✨"
    else
        print_info "Found $todo_count TODO/FIXME marker(s)"
        
        echo ""
        print_info "Breakdown by type:"
        echo "  TODO:  $(grep -r "TODO" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "  FIXME: $(grep -r "FIXME" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "  XXX:   $(grep -r "XXX" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "  HACK:  $(grep -r "HACK" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
    fi
}

# Check build warnings
check_warnings() {
    print_header "Checking Build Warnings"
    
    print_info "Building workspace..."
    local warnings=$(cargo build --workspace 2>&1 | grep "warning:" | wc -l)
    
    if [ "$warnings" -eq 0 ]; then
        print_success "No warnings! ✨"
    elif [ "$warnings" -lt 50 ]; then
        print_success "$warnings warning(s) - under target of 50 ✓"
    elif [ "$warnings" -lt 100 ]; then
        print_warning "$warnings warning(s) - close to target"
    else
        print_warning "$warnings warning(s) - above target"
    fi
    
    # Show warning breakdown
    echo ""
    print_info "Warning types:"
    cargo build --workspace 2>&1 | \
        grep "warning:" | \
        sed 's/warning: //' | \
        cut -d' ' -f1-4 | \
        sort | uniq -c | sort -rn | head -5 | \
        sed 's/^/  /'
}

# Check test coverage
check_tests() {
    print_header "Checking Tests"
    
    print_info "Running test suite..."
    if cargo test --workspace --quiet 2>&1 | grep -q "FAILED"; then
        print_error "Some tests failed!"
        cargo test --workspace 2>&1 | grep -A 3 "FAILED"
    else
        local test_count=$(cargo test --workspace 2>&1 | grep "test result:" | tail -1 | grep -oP '\d+ passed' | cut -d' ' -f1)
        print_success "All $test_count tests passed! ✓"
    fi
}

# Check for deprecated items
check_deprecated() {
    print_header "Checking Deprecated Items"
    
    local dep_count=$(grep -r "#\[deprecated" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)
    
    if [ "$dep_count" -eq 0 ]; then
        print_success "No deprecated items! ✨"
    else
        print_info "Found $dep_count deprecated item(s)"
        
        # Show deprecated traits
        echo ""
        print_info "Deprecated traits:"
        grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" -l 2>/dev/null | wc -l | \
            xargs -I {} echo "  {} file(s) using deprecated traits"
    fi
}

# Check clone usage
check_clones() {
    print_header "Checking Clone Usage"
    
    local clone_count=$(grep -r "\.clone()" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)
    
    print_info "Found $clone_count .clone() call(s)"
    
    # Calculate clone density
    local total_lines=$(find "$PROJECT_ROOT/crates" -name "*.rs" -type f -exec wc -l {} \; | awk '{sum+=$1} END {print sum}')
    local clones_per_1k=$((clone_count * 1000 / total_lines))
    
    echo "  Clone density: $clones_per_1k clones per 1000 lines"
    
    if [ "$clones_per_1k" -lt 5 ]; then
        print_success "Clone usage is excellent! ✓"
    elif [ "$clones_per_1k" -lt 10 ]; then
        print_info "Clone usage is good"
    else
        print_warning "Consider reducing clone usage"
    fi
}

# Check Box<dyn> usage
check_dynamic_dispatch() {
    print_header "Checking Dynamic Dispatch"
    
    local dyn_count=$(grep -r "Box<dyn" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)
    
    print_info "Found $dyn_count Box<dyn> instance(s)"
    
    if [ "$dyn_count" -lt 200 ]; then
        print_success "Dynamic dispatch usage is low! ✓"
    elif [ "$dyn_count" -lt 400 ]; then
        print_info "Dynamic dispatch usage is moderate"
    else
        print_warning "Consider enum dispatch pattern for hot paths"
        echo "  See: examples/provider_dispatch_pattern.rs"
    fi
}

# Generate report
generate_report() {
    print_header "Code Quality Report Summary"
    
    local report_file="$PROJECT_ROOT/code-quality-report-$(date +%Y%m%d-%H%M%S).txt"
    
    {
        echo "BearDog Code Quality Report"
        echo "Generated: $(date)"
        echo ""
        
        # File stats
        echo "=== File Statistics ==="
        local total_files=$(find "$PROJECT_ROOT/crates" -name "*.rs" -type f | wc -l)
        local total_lines=$(find "$PROJECT_ROOT/crates" -name "*.rs" -type f -exec wc -l {} \; | awk '{sum+=$1} END {print sum}')
        local avg_lines=$((total_lines / total_files))
        
        echo "Total Rust files: $total_files"
        echo "Total lines: $total_lines"
        echo "Average lines per file: $avg_lines"
        
        # Quality metrics
        echo ""
        echo "=== Quality Metrics ==="
        echo "TODOs/FIXMEs: $(grep -r "TODO\|FIXME" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "Build warnings: $(cargo build --workspace 2>&1 | grep "warning:" | wc -l)"
        echo "Clone operations: $(grep -r "\.clone()" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "Box<dyn> usage: $(grep -r "Box<dyn" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        echo "Deprecated items: $(grep -r "#\[deprecated" "$PROJECT_ROOT/crates" --include="*.rs" | wc -l)"
        
    } > "$report_file"
    
    print_success "Report saved to: $report_file"
    cat "$report_file"
}

# Main execution
main() {
    cd "$PROJECT_ROOT"
    
    echo ""
    echo "🐻 BearDog Code Quality Check"
    echo ""
    
    # Run all checks
    check_file_sizes
    check_todos
    check_warnings
    check_tests
    check_deprecated
    check_clones
    check_dynamic_dispatch
    
    # Generate report
    generate_report
    
    print_header "Quality Check Complete! ✨"
}

# Run main
main "$@"

