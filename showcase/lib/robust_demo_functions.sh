#!/usr/bin/env bash
#
# Robust Demo Functions
# Production-grade error handling, validation, and recovery
#

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

#==============================================================================
# Error Handling
#==============================================================================

# Run demo with error handling
run_demo_safe() {
    local demo_name="$1"
    local demo_cmd="$2"
    
    echo -e "${BLUE}▶${NC} Running: $demo_name"
    
    if eval "$demo_cmd"; then
        echo -e "${GREEN}✅ $demo_name: SUCCESS${NC}"
        return 0
    else
        local exit_code=$?
        echo -e "${RED}❌ $demo_name: FAILED (exit code: $exit_code)${NC}"
        echo -e "${YELLOW}   Continuing with other demos...${NC}"
        return $exit_code
    fi
}

# Retry with exponential backoff
retry_with_backoff() {
    local max_attempts=3
    local timeout=1
    local attempt=0
    local exitCode=0

    while [ $attempt -lt $max_attempts ]; do
        if "$@"; then
            return 0
        else
            exitCode=$?
        fi

        if [ $attempt -lt $((max_attempts - 1)) ]; then
            echo -e "${YELLOW}⚠️  Command failed. Retrying in $timeout seconds...${NC}"
            sleep $timeout
            timeout=$((timeout * 2))
        fi
        attempt=$((attempt + 1))
    done

    echo -e "${RED}❌ Command failed after $max_attempts attempts${NC}"
    return $exitCode
}

#==============================================================================
# Receipt Validation
#==============================================================================

# Validate receipt file
validate_receipt() {
    local receipt_file="$1"
    local errors=0
    
    # Check file exists
    if [ ! -f "$receipt_file" ]; then
        echo -e "${RED}❌ Receipt missing: $receipt_file${NC}"
        return 1
    fi
    
    # Validate JSON structure
    if ! jq empty "$receipt_file" 2>/dev/null; then
        echo -e "${RED}❌ Invalid JSON: $receipt_file${NC}"
        return 1
    fi
    
    # Check required fields
    local required_fields=("receipt_id" "operation" "timestamp")
    for field in "${required_fields[@]}"; do
        if ! jq -e ".$field" "$receipt_file" >/dev/null 2>&1; then
            echo -e "${RED}❌ Missing field '$field': $receipt_file${NC}"
            errors=$((errors + 1))
        fi
    done
    
    if [ $errors -eq 0 ]; then
        echo -e "${GREEN}✅ Receipt valid: $(basename "$receipt_file")${NC}"
        return 0
    else
        echo -e "${RED}❌ Receipt has $errors error(s): $receipt_file${NC}"
        return 1
    fi
}

# Validate all receipts in directory
validate_all_receipts() {
    local receipts_dir="$1"
    local total=0
    local valid=0
    local invalid=0
    
    echo -e "${CYAN}🔍 Validating receipts in: $receipts_dir${NC}\n"
    
    if [ ! -d "$receipts_dir" ]; then
        echo -e "${RED}❌ Directory not found: $receipts_dir${NC}"
        return 1
    fi
    
    while IFS= read -r -d '' receipt; do
        total=$((total + 1))
        if validate_receipt "$receipt"; then
            valid=$((valid + 1))
        else
            invalid=$((invalid + 1))
        fi
    done < <(find "$receipts_dir" -name "*.json" -type f -print0)
    
    echo ""
    echo -e "${CYAN}📊 Validation Summary:${NC}"
    echo -e "   Total:   $total"
    echo -e "   ${GREEN}Valid:   $valid${NC}"
    if [ $invalid -gt 0 ]; then
        echo -e "   ${RED}Invalid: $invalid${NC}"
        return 1
    else
        echo -e "   Invalid: 0"
        return 0
    fi
}

#==============================================================================
# Hardware Detection
#==============================================================================

# Detect hardware with graceful fallback
detect_hardware_safe() {
    local hardware_type="$1"
    
    case "$hardware_type" in
        solokeys)
            if command -v lsusb >/dev/null 2>&1 && lsusb 2>/dev/null | grep -qi solo; then
                local count=$(lsusb 2>/dev/null | grep -ci solo)
                echo -e "${GREEN}✅ SoloKeys detected: $count device(s)${NC}"
                return 0
            else
                echo -e "${YELLOW}⚠️  No SoloKeys detected (optional for this demo)${NC}"
                return 1
            fi
            ;;
        pixel)
            if command -v adb >/dev/null 2>&1 && adb devices 2>/dev/null | grep -q "device$"; then
                local device_id=$(adb devices | grep "device$" | awk '{print $1}')
                echo -e "${GREEN}✅ Pixel detected via ADB: $device_id${NC}"
                return 0
            else
                echo -e "${YELLOW}⚠️  No Pixel detected via ADB (optional)${NC}"
                return 1
            fi
            ;;
        software-hsm)
            if command -v softhsm2-util >/dev/null 2>&1; then
                echo -e "${GREEN}✅ SoftHSM2 available${NC}"
                return 0
            else
                echo -e "${YELLOW}⚠️  SoftHSM2 not found (will use native software HSM)${NC}"
                return 0  # Still OK, we have native implementation
            fi
            ;;
        beardog-cli)
            local beardog="${2:-./target/release/beardog}"
            if [ -f "$beardog" ] && [ -x "$beardog" ]; then
                local version=$("$beardog" --version 2>/dev/null | head -1)
                echo -e "${GREEN}✅ BearDog CLI available: $version${NC}"
                return 0
            else
                echo -e "${RED}❌ BearDog CLI not found or not executable: $beardog${NC}"
                return 1
            fi
            ;;
        *)
            echo -e "${RED}❌ Unknown hardware type: $hardware_type${NC}"
            return 1
            ;;
    esac
}

# Comprehensive hardware check
check_all_hardware() {
    local beardog_cli="${1:-./target/release/beardog}"
    local all_ok=0
    
    echo -e "${CYAN}🔍 Checking hardware availability...${NC}\n"
    
    # Critical: BearDog CLI
    if ! detect_hardware_safe "beardog-cli" "$beardog_cli"; then
        echo -e "${RED}❌ CRITICAL: BearDog CLI not available${NC}"
        return 1
    fi
    
    # Optional: SoloKeys
    detect_hardware_safe "solokeys" || all_ok=1
    
    # Optional: Pixel
    detect_hardware_safe "pixel" || all_ok=1
    
    # Optional: SoftHSM
    detect_hardware_safe "software-hsm" || true
    
    echo ""
    if [ $all_ok -eq 0 ]; then
        echo -e "${GREEN}✅ All hardware checks passed${NC}"
    else
        echo -e "${YELLOW}⚠️  Some optional hardware not detected (demos will adapt)${NC}"
    fi
    
    return 0
}

#==============================================================================
# Progress Indicators
#==============================================================================

# Show progress spinner
show_progress() {
    local pid=$1
    local description="$2"
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0

    while kill -0 $pid 2>/dev/null; do
        i=$(((i + 1) % 10))
        printf "\r${spin:$i:1} %s" "$description"
        sleep 0.1
    done
    printf "\r${GREEN}✅ %s${NC}\n" "$description"
}

#==============================================================================
# Logging
#==============================================================================

# Initialize logging
init_logging() {
    local log_dir="${1:-.}"
    LOG_FILE="$log_dir/showcase-debug-$(date +%s).log"
    
    mkdir -p "$log_dir"
    echo "# BearDog Showcase Log - $(date -Iseconds)" > "$LOG_FILE"
    echo "Log file: $LOG_FILE"
}

# Log functions
log_debug() {
    echo "[DEBUG $(date -Iseconds)] $*" >> "${LOG_FILE:-/dev/null}"
}

log_info() {
    echo "[INFO $(date -Iseconds)] $*" | tee -a "${LOG_FILE:-/dev/null}"
}

log_error() {
    echo -e "${RED}[ERROR $(date -Iseconds)] $*${NC}" | tee -a "${LOG_FILE:-/dev/null}"
}

log_success() {
    echo -e "${GREEN}[SUCCESS $(date -Iseconds)] $*${NC}" | tee -a "${LOG_FILE:-/dev/null}"
}

#==============================================================================
# Smoke Tests
#==============================================================================

# Run smoke tests before demos
smoke_test() {
    local beardog="${1:-./target/release/beardog}"
    
    echo -e "${CYAN}🔍 Running smoke tests...${NC}\n"
    
    # Test: BearDog CLI exists
    if [ ! -f "$beardog" ]; then
        echo -e "${RED}❌ BearDog CLI not found at: $beardog${NC}"
        echo -e "${YELLOW}   Build it with: cargo build --release${NC}"
        return 1
    fi
    echo -e "${GREEN}✅ BearDog CLI found${NC}"
    
    # Test: CLI is executable
    if [ ! -x "$beardog" ]; then
        echo -e "${RED}❌ BearDog CLI not executable: $beardog${NC}"
        return 1
    fi
    echo -e "${GREEN}✅ BearDog CLI is executable${NC}"
    
    # Test: CLI responds
    if ! "$beardog" --version &>/dev/null; then
        echo -e "${RED}❌ BearDog CLI doesn't respond to --version${NC}"
        return 1
    fi
    echo -e "${GREEN}✅ BearDog CLI responds${NC}"
    
    # Test: Can list keys (may fail, that's OK)
    if "$beardog" key list &>/dev/null; then
        echo -e "${GREEN}✅ Can list keys${NC}"
    else
        echo -e "${YELLOW}⚠️  Key listing failed (may be normal if no keys exist)${NC}"
    fi
    
    # Test: jq available (for receipt validation)
    if command -v jq >/dev/null 2>&1; then
        echo -e "${GREEN}✅ jq available (for receipt validation)${NC}"
    else
        echo -e "${YELLOW}⚠️  jq not found (receipt validation will be limited)${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}✅ All smoke tests passed${NC}"
    return 0
}

#==============================================================================
# Cleanup
#==============================================================================

# Clean up demo outputs
cleanup_demos() {
    local force="${1:-false}"
    local outputs_dir="${2:-outputs}"
    
    echo -e "${CYAN}🧹 Cleaning up demo outputs...${NC}"
    
    if [ "$force" != "true" ]; then
        echo -e "${YELLOW}This will delete:${NC}"
        echo "  - Auto-session outputs"
        echo "  - Session receipts"
        echo "  - Demo logs"
        echo ""
        read -p "Continue? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Cleanup cancelled"
            return 0
        fi
    fi
    
    # Remove auto-session outputs
    if [ -d "$outputs_dir" ]; then
        find "$outputs_dir" -type d -name "auto-session-*" -exec rm -rf {} + 2>/dev/null || true
        find "$outputs_dir" -type d -name "receipts-session-*" -exec rm -rf {} + 2>/dev/null || true
    fi
    
    # Remove log files
    find . -maxdepth 1 -name "*.log" -type f -delete 2>/dev/null || true
    find . -maxdepth 1 -name "demo-run-*.log" -type f -delete 2>/dev/null || true
    find . -maxdepth 1 -name "full-demo-run-*.log" -type f -delete 2>/dev/null || true
    
    echo -e "${GREEN}✅ Cleanup complete${NC}"
}

#==============================================================================
# Export functions
#==============================================================================

# Make functions available to scripts that source this file
export -f run_demo_safe
export -f retry_with_backoff
export -f validate_receipt
export -f validate_all_receipts
export -f detect_hardware_safe
export -f check_all_hardware
export -f show_progress
export -f init_logging
export -f log_debug
export -f log_info
export -f log_error
export -f log_success
export -f smoke_test
export -f cleanup_demos

