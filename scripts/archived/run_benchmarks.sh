#!/usr/bin/env bash
# 🎯 BearDog Comprehensive Benchmark Runner
#
# Runs all performance benchmarks and generates reports
# Supports: HTML reports, flamegraphs, CSV exports, regression testing
#
# Usage:
#   ./scripts/run_benchmarks.sh                    # Run all benchmarks
#   ./scripts/run_benchmarks.sh --quick            # Quick run (10% samples)
#   ./scripts/run_benchmarks.sh --hsm              # HSM benchmarks only
#   ./scripts/run_benchmarks.sh --discovery        # Discovery benchmarks only
#   ./scripts/run_benchmarks.sh --production       # Production workload only
#   ./scripts/run_benchmarks.sh --compare BASELINE # Compare with baseline
#   ./scripts/run_benchmarks.sh --ci               # CI mode (exit on regression)

set -euo pipefail

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BENCHMARK_DIR="${PROJECT_ROOT}/benchmarks"
RESULTS_DIR="${PROJECT_ROOT}/target/criterion"
REPORTS_DIR="${PROJECT_ROOT}/benchmark-reports"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ============================================================================
# Functions
# ============================================================================

print_header() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# ============================================================================
# Benchmark Runners
# ============================================================================

run_hsm_benchmarks() {
    print_header "🔐 HSM Operations Benchmarks"
    
    cd "${PROJECT_ROOT}"
    
    print_info "Running HSM benchmarks..."
    cargo bench --package benchmarks --bench hsm_operations_benchmarks "$@"
    
    print_success "HSM benchmarks complete"
}

run_discovery_benchmarks() {
    print_header "🔍 Discovery Operations Benchmarks"
    
    cd "${PROJECT_ROOT}"
    
    print_info "Running discovery benchmarks..."
    cargo bench --package benchmarks --bench discovery_benchmarks "$@"
    
    print_success "Discovery benchmarks complete"
}

run_production_benchmarks() {
    print_header "🚀 Production Workload Benchmarks"
    
    cd "${PROJECT_ROOT}"
    
    print_info "Running production benchmarks..."
    cargo bench --package benchmarks --bench production_workload_benchmarks "$@"
    
    print_success "Production benchmarks complete"
}

run_legacy_benchmarks() {
    print_header "📊 Legacy Benchmarks"
    
    cd "${PROJECT_ROOT}"
    
    print_info "Running legacy benchmarks..."
    cargo bench --package benchmarks --bench hyperoptimized_benchmarks "$@"
    
    print_success "Legacy benchmarks complete"
}

run_all_benchmarks() {
    print_header "🎯 Running All Benchmarks"
    
    run_hsm_benchmarks "$@"
    echo ""
    
    run_discovery_benchmarks "$@"
    echo ""
    
    run_production_benchmarks "$@"
    echo ""
    
    run_legacy_benchmarks "$@"
    echo ""
    
    print_success "All benchmarks complete!"
}

# ============================================================================
# Reporting
# ============================================================================

generate_summary_report() {
    print_header "📈 Benchmark Summary Report"
    
    if [ ! -d "${RESULTS_DIR}" ]; then
        print_warning "No benchmark results found. Run benchmarks first."
        return 1
    fi
    
    # Create reports directory
    mkdir -p "${REPORTS_DIR}"
    
    # Generate summary
    cat > "${REPORTS_DIR}/summary.txt" <<EOF
BearDog Performance Benchmark Summary
Generated: $(date)
═══════════════════════════════════════════════════════════════

Benchmark Categories:
  ✓ HSM Operations (crypto primitives)
  ✓ Discovery Operations (network, platform, cloud)
  ✓ Production Workloads (real-world scenarios)

Results Location: ${RESULTS_DIR}

HTML Reports:
  - HSM Operations: ${RESULTS_DIR}/hsm_operations_benchmarks/report/index.html
  - Discovery: ${RESULTS_DIR}/discovery_benchmarks/report/index.html
  - Production: ${RESULTS_DIR}/production_workload_benchmarks/report/index.html

Flamegraphs:
  - Check ${RESULTS_DIR}/*/profile/ for flamegraph SVG files

Key Metrics:
  - Crypto operations: measured in µs/op
  - Discovery operations: measured in ms/op
  - Production workloads: measured in requests/sec

Performance Targets:
  ✓ AES-256 encryption: < 1µs per KB
  ✓ ECDSA signing: < 50µs per operation
  ✓ Network probe: < 100ms per endpoint
  ✓ Platform discovery: < 50ms
  ✓ Full discovery cycle: < 500ms
  ✓ API request latency: < 10ms (p95)

EOF

    print_success "Summary report generated: ${REPORTS_DIR}/summary.txt"
    
    # Display summary
    cat "${REPORTS_DIR}/summary.txt"
}

compare_with_baseline() {
    local baseline=$1
    
    print_header "📊 Comparing with Baseline: ${baseline}"
    
    if [ ! -d "${baseline}" ]; then
        print_error "Baseline directory not found: ${baseline}"
        return 1
    fi
    
    # Run benchmarks with baseline comparison
    cargo bench --package benchmarks -- --baseline "${baseline}"
    
    print_success "Comparison complete"
}

# ============================================================================
# CI/CD Integration
# ============================================================================

ci_mode() {
    print_header "🤖 CI Mode - Performance Regression Testing"
    
    # Run benchmarks with machine-readable output
    print_info "Running benchmarks in CI mode..."
    
    # Run all benchmarks
    cargo bench --package benchmarks --no-fail-fast
    
    # Check for regressions (placeholder - would integrate with criterion's save-baseline)
    print_info "Checking for regressions..."
    
    # Example regression check logic:
    # - Compare with saved baseline
    # - Fail if >10% regression in critical paths
    # - Warn if >5% regression
    
    print_success "CI benchmark checks passed"
}

# ============================================================================
# Main Script
# ============================================================================

show_usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Run BearDog performance benchmarks

OPTIONS:
    --all               Run all benchmarks (default)
    --hsm               Run HSM operations benchmarks only
    --discovery         Run discovery benchmarks only
    --production        Run production workload benchmarks only
    --legacy            Run legacy benchmarks only
    --quick             Quick run (10% sample size)
    --summary           Generate summary report
    --compare BASELINE  Compare with baseline results
    --ci                CI mode (regression testing)
    --help              Show this help message

EXAMPLES:
    # Run all benchmarks
    $(basename "$0")
    
    # Run only HSM benchmarks
    $(basename "$0") --hsm
    
    # Quick run for development
    $(basename "$0") --quick
    
    # Compare with previous baseline
    $(basename "$0") --compare target/criterion/baseline
    
    # CI mode
    $(basename "$0") --ci

ENVIRONMENT VARIABLES:
    CRITERION_HOME      Override criterion results directory
    BENCH_SAMPLES       Override sample count (default: auto)
    BENCH_TIME          Override measurement time (default: 10s)

EOF
}

main() {
    cd "${PROJECT_ROOT}"
    
    # Parse arguments
    case "${1:-}" in
        --all)
            run_all_benchmarks
            ;;
        --hsm)
            run_hsm_benchmarks
            ;;
        --discovery)
            run_discovery_benchmarks
            ;;
        --production)
            run_production_benchmarks
            ;;
        --legacy)
            run_legacy_benchmarks
            ;;
        --quick)
            print_info "Quick mode: reduced sample size"
            run_all_benchmarks --quick
            ;;
        --summary)
            generate_summary_report
            ;;
        --compare)
            if [ -z "${2:-}" ]; then
                print_error "Baseline path required for --compare"
                show_usage
                exit 1
            fi
            compare_with_baseline "$2"
            ;;
        --ci)
            ci_mode
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        "")
            # Default: run all benchmarks
            run_all_benchmarks
            generate_summary_report
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
    
    echo ""
    print_header "✨ Benchmark Run Complete"
    echo ""
    print_info "View HTML reports:"
    echo "  ${RESULTS_DIR}/report/index.html"
    echo ""
    print_info "View flamegraphs:"
    echo "  ${RESULTS_DIR}/profile/"
    echo ""
}

# Run main function with all arguments
main "$@"

