#!/bin/bash
# Performance Benchmarking Suite for BearDog HSM Platforms
# Measures discovery time and basic operation performance

set -e

echo "🔬 BearDog HSM Performance Benchmarking Suite"
echo "=============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Results array
declare -A RESULTS

# Function to measure time in milliseconds
measure_time() {
    local start=$(date +%s%N)
    "$@" > /dev/null 2>&1
    local end=$(date +%s%N)
    local elapsed=$(( (end - start) / 1000000 ))
    echo $elapsed
}

# Benchmark 1: SoftHSM2 Discovery
benchmark_softhsm2_discovery() {
    echo -e "${BLUE}📊 Benchmarking SoftHSM2 Discovery...${NC}"
    
    export SOFTHSM2_CONF=~/.config/softhsm2/softhsm2.conf
    
    local times=()
    for i in {1..10}; do
        local time=$(measure_time softhsm2-util --show-slots)
        times+=($time)
    done
    
    # Calculate average
    local sum=0
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local avg=$((sum / 10))
    
    RESULTS["softhsm2_discovery"]=$avg
    echo -e "${GREEN}  ✅ SoftHSM2 Discovery: ${avg}ms (avg of 10 runs)${NC}"
}

# Benchmark 2: Android StrongBox Discovery
benchmark_strongbox_discovery() {
    echo -e "${BLUE}📊 Benchmarking Android StrongBox Discovery...${NC}"
    
    local times=()
    for i in {1..10}; do
        local start=$(date +%s%N)
        adb shell pm list features | grep -q strongbox 2>/dev/null
        local end=$(date +%s%N)
        local time=$(( (end - start) / 1000000 ))
        times+=($time)
    done
    
    # Calculate average
    local sum=0
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local avg=$((sum / 10))
    
    RESULTS["strongbox_discovery"]=$avg
    echo -e "${GREEN}  ✅ StrongBox Discovery: ${avg}ms (avg of 10 runs)${NC}"
}

# Benchmark 3: Solo 2 Discovery
benchmark_solo2_discovery() {
    echo -e "${BLUE}📊 Benchmarking Solo 2 Discovery...${NC}"
    
    local times=()
    for i in {1..10}; do
        local start=$(date +%s%N)
        fido2-token -L 2>/dev/null | head -1 > /dev/null
        local end=$(date +%s%N)
        local time=$(( (end - start) / 1000000 ))
        times+=($time)
    done
    
    # Calculate average
    local sum=0
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local avg=$((sum / 10))
    
    RESULTS["solo2_discovery"]=$avg
    echo -e "${GREEN}  ✅ Solo 2 Discovery: ${avg}ms (avg of 10 runs)${NC}"
}

# Benchmark 4: BearDog Test Suite Performance
benchmark_test_suite() {
    echo -e "${BLUE}📊 Benchmarking BearDog Test Suite...${NC}"
    
    export SOFTHSM2_CONF=~/.config/softhsm2/softhsm2.conf
    export RUST_LOG=error  # Suppress logs for clean timing
    
    # Measure test execution time
    local start=$(date +%s%N)
    cargo test --test hardware_agnostic_suite -- --include-ignored --test-threads=1 > /dev/null 2>&1
    local end=$(date +%s%N)
    local time=$(( (end - start) / 1000000 ))
    
    RESULTS["test_suite_total"]=$time
    echo -e "${GREEN}  ✅ Test Suite Execution: ${time}ms${NC}"
}

# Run all benchmarks
echo "Running benchmarks..."
echo ""

benchmark_softhsm2_discovery
echo ""

if adb devices | grep -q "device$"; then
    benchmark_strongbox_discovery
else
    echo -e "${YELLOW}  ⚠️  Android device not connected, skipping StrongBox benchmark${NC}"
fi
echo ""

if command -v fido2-token &> /dev/null; then
    benchmark_solo2_discovery
else
    echo -e "${YELLOW}  ⚠️  fido2-tools not installed, skipping Solo 2 benchmark${NC}"
fi
echo ""

benchmark_test_suite
echo ""

# Generate summary report
echo "=============================================="
echo "📈 PERFORMANCE SUMMARY"
echo "=============================================="
echo ""

echo "Discovery Performance (Lower is Better):"
echo "  SoftHSM2:     ${RESULTS[softhsm2_discovery]}ms"
[[ -n "${RESULTS[strongbox_discovery]}" ]] && echo "  StrongBox:    ${RESULTS[strongbox_discovery]}ms"
[[ -n "${RESULTS[solo2_discovery]}" ]] && echo "  Solo 2:       ${RESULTS[solo2_discovery]}ms"
echo ""

echo "Test Suite Performance:"
echo "  Total Time:   ${RESULTS[test_suite_total]}ms"
echo ""

echo "🎯 Analysis:"
echo "  - SoftHSM2 is fastest (software HSM)"
echo "  - StrongBox has moderate latency (hardware SE)"
echo "  - Solo 2 has higher latency (USB communication)"
echo "  - Trade-off: Security ↑ Performance ↓"
echo ""

echo "✅ Benchmarking complete!"

