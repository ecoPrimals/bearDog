#!/usr/bin/env bash
# Robust Demo Functions Library
#
# Usage: Source this file in your demo scripts
#   source ../lib/robust_demo_functions.sh

# Parse common flags
AUTO_MODE=false
for arg in "$@"; do
    case $arg in
        --auto|--non-interactive|-a)
            AUTO_MODE=true
            shift
            ;;
    esac
done

# Colors
export GREEN='\033[0;32m'
export BLUE='\033[0;34m'
export YELLOW='\033[1;33m'
export RED='\033[0;31m'
export CYAN='\033[0;36m'
export MAGENTA='\033[0;35m'
export NC='\033[0m'

# Logging functions
log_step() {
    echo -e "${BLUE}→${NC} $1"
}

log_success() {
    echo -e "${GREEN}✅${NC} $1"
}

log_info() {
    echo -e "${YELLOW}ℹ️${NC}  $1"
}

log_error() {
    echo -e "${RED}❌${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠️${NC}  $1"
}

log_highlight() {
    echo -e "${CYAN}▶${NC} $1"
}

log_capability() {
    echo -e "${MAGENTA}  •${NC} $1"
}

print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Wait for user (or auto-continue)
wait_for_user() {
    if [ "$AUTO_MODE" = true ]; then
        echo ""
        log_info "Auto-mode: Continuing in 2 seconds..."
        sleep 2
        echo ""
    else
        echo ""
        read -p "Press ENTER to continue..." -r
        echo ""
    fi
}

# Quick pause for readability in auto mode
auto_pause() {
    local seconds=${1:-1}
    if [ "$AUTO_MODE" = true ]; then
        sleep "$seconds"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Validate file exists
validate_file() {
    local file="$1"
    if [ ! -f "$file" ]; then
        log_error "File not found: $file"
        return 1
    fi
    return 0
}

# Validate directory exists
validate_dir() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        log_error "Directory not found: $dir"
        return 1
    fi
    return 0
}

# Create output directory
create_output_dir() {
    local dir="$1"
    mkdir -p "$dir"
    log_success "Output directory: $dir"
}

# Run command with error handling
run_cmd() {
    local cmd="$*"
    log_info "Running: $cmd"
    if eval "$cmd"; then
        log_success "Command succeeded"
        return 0
    else
        log_error "Command failed: $cmd"
        return 1
    fi
}

# Cleanup function
cleanup() {
    if [ -n "${CLEANUP_FILES:-}" ]; then
        log_info "Cleaning up temporary files..."
        # shellcheck disable=SC2086
        rm -f $CLEANUP_FILES 2>/dev/null || true
    fi
}

# Register cleanup on exit
trap cleanup EXIT

# Show usage if needed
show_usage() {
    local script_name="$1"
    cat << EOF
Usage: $script_name [OPTIONS]

OPTIONS:
    --auto, -a, --non-interactive    Run in automatic mode (no prompts)
    --help, -h                       Show this help message

EXAMPLES:
    # Interactive mode (default)
    ./$script_name

    # Automatic mode (no prompts)
    ./$script_name --auto

EOF
}

# Check for help flag
for arg in "$@"; do
    case $arg in
        --help|-h)
            show_usage "$(basename "$0")"
            exit 0
            ;;
    esac
done

# Export AUTO_MODE for child processes
export AUTO_MODE

# Print mode indicator
if [ "$AUTO_MODE" = true ]; then
    log_info "🤖 Running in AUTO MODE (non-interactive)"
    echo ""
else
    log_info "👤 Running in INTERACTIVE MODE"
    log_info "   (Use --auto for non-interactive mode)"
    echo ""
fi
