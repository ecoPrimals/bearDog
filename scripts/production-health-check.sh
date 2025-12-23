#!/bin/bash

# 🐻 BearDog Production Health Check Script
# Version: 3.0
# Date: January 2025
# Purpose: Comprehensive production readiness validation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BEARDOG_ROOT="${BEARDOG_ROOT:-$(pwd)}"
PRODUCTION_CONFIG="${BEARDOG_ROOT}/configs/production.toml"
HEALTH_ENDPOINT="${HEALTH_ENDPOINT:-http://localhost:8080/health}"
LOG_FILE="${BEARDOG_ROOT}/logs/health-check-$(date +%Y%m%d-%H%M%S).log"

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# Functions
log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

success() {
    log "${GREEN}✅ $1${NC}"
    ((PASSED_CHECKS++))
}

warning() {
    log "${YELLOW}⚠️  $1${NC}"
    ((WARNING_CHECKS++))
}

error() {
    log "${RED}❌ $1${NC}"
    ((FAILED_CHECKS++))
}

info() {
    log "${BLUE}ℹ️  $1${NC}"
}

check_command() {
    local cmd="$1"
    local description="$2"
    ((TOTAL_CHECKS++))
    
    if command -v "$cmd" >/dev/null 2>&1; then
        success "$description: $cmd found"
    else
        error "$description: $cmd not found"
    fi
}

check_file() {
    local file="$1"
    local description="$2"
    ((TOTAL_CHECKS++))
    
    if [[ -f "$file" ]]; then
        success "$description: $file exists"
    else
        error "$description: $file missing"
    fi
}

check_directory() {
    local dir="$1"
    local description="$2"
    ((TOTAL_CHECKS++))
    
    if [[ -d "$dir" ]]; then
        success "$description: $dir exists"
    else
        error "$description: $dir missing"
    fi
}

check_port() {
    local port="$1"
    local description="$2"
    ((TOTAL_CHECKS++))
    
    if netstat -tlnp 2>/dev/null | grep -q ":$port "; then
        success "$description: Port $port is listening"
    else
        warning "$description: Port $port not listening (service may not be running)"
    fi
}

# Main health check function
main() {
    log "${BLUE}🐻 BearDog Production Health Check Starting...${NC}"
    log "=================================================="
    
    # Create logs directory if it doesn't exist
    mkdir -p "${BEARDOG_ROOT}/logs"
    
    # 1. System Requirements Check
    info "1️⃣  Checking System Requirements..."
    
    check_command "cargo" "Rust Cargo"
    check_command "rustc" "Rust Compiler"
    
    # Check Rust version
    ((TOTAL_CHECKS++))
    if command -v rustc >/dev/null 2>&1; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        if [[ "$RUST_VERSION" > "1.70" ]]; then
            success "Rust version: $RUST_VERSION (>= 1.70 required)"
        else
            error "Rust version: $RUST_VERSION (< 1.70, upgrade required)"
        fi
    else
        error "Rust not installed"
    fi
    
    # Check system resources
    ((TOTAL_CHECKS++))
    AVAILABLE_MEM=$(free -m | awk 'NR==2{printf "%.1f", $7/1024}')
    if (( $(echo "$AVAILABLE_MEM > 2.0" | bc -l) )); then
        success "Available memory: ${AVAILABLE_MEM}GB (>= 2GB required)"
    else
        warning "Available memory: ${AVAILABLE_MEM}GB (< 2GB, may impact performance)"
    fi
    
    ((TOTAL_CHECKS++))
    AVAILABLE_DISK=$(df -BG "$BEARDOG_ROOT" | awk 'NR==2 {print $4}' | sed 's/G//')
    if [[ "$AVAILABLE_DISK" -gt 5 ]]; then
        success "Available disk space: ${AVAILABLE_DISK}GB (>= 5GB required)"
    else
        error "Available disk space: ${AVAILABLE_DISK}GB (< 5GB required)"
    fi
    
    # 2. Build System Check
    info "2️⃣  Checking Build System..."
    
    check_file "${BEARDOG_ROOT}/Cargo.toml" "Root Cargo.toml"
    check_directory "${BEARDOG_ROOT}/crates" "Crates directory"
    check_directory "${BEARDOG_ROOT}/target" "Target directory"
    
    # Check if release build exists
    ((TOTAL_CHECKS++))
    if find "${BEARDOG_ROOT}/target/release" -name "beardog*" -type f -executable 2>/dev/null | grep -q .; then
        success "Release binaries found in target/release"
    else
        warning "No release binaries found (run: cargo build --workspace --release)"
    fi
    
    # 3. Configuration Check
    info "3️⃣  Checking Configuration..."
    
    check_file "${BEARDOG_ROOT}/configs/beardog-config-template.toml" "Configuration template"
    check_directory "${BEARDOG_ROOT}/configs/environments" "Environment configs"
    
    if [[ -f "$PRODUCTION_CONFIG" ]]; then
        success "Production configuration exists: $PRODUCTION_CONFIG"
        
        # Validate TOML syntax
        ((TOTAL_CHECKS++))
        if command -v toml >/dev/null 2>&1; then
            if toml get "$PRODUCTION_CONFIG" . >/dev/null 2>&1; then
                success "Production TOML syntax is valid"
            else
                error "Production TOML syntax is invalid"
            fi
        else
            warning "TOML validator not available (install: cargo install toml-cli)"
        fi
    else
        warning "Production configuration not found: $PRODUCTION_CONFIG"
    fi
    
    # 4. Security Check
    info "4️⃣  Checking Security Configuration..."
    
    check_directory "${BEARDOG_ROOT}/configs/environments" "Environment configurations"
    
    # Check for sensitive files permissions
    ((TOTAL_CHECKS++))
    if [[ -f "${BEARDOG_ROOT}/configs/production.env" ]]; then
        PERMS=$(stat -c "%a" "${BEARDOG_ROOT}/configs/production.env")
        if [[ "$PERMS" == "600" ]] || [[ "$PERMS" == "640" ]]; then
            success "Production environment file permissions: $PERMS (secure)"
        else
            warning "Production environment file permissions: $PERMS (consider 600 or 640)"
        fi
    else
        warning "Production environment file not found"
    fi
    
    # 5. Network Check
    info "5️⃣  Checking Network Configuration..."
    
    check_port "8080" "Default BearDog port"
    
    # Check if health endpoint is accessible (if service is running)
    ((TOTAL_CHECKS++))
    if curl -s -f "$HEALTH_ENDPOINT" >/dev/null 2>&1; then
        success "Health endpoint accessible: $HEALTH_ENDPOINT"
    else
        warning "Health endpoint not accessible: $HEALTH_ENDPOINT (service may not be running)"
    fi
    
    # 6. Dependencies Check
    info "6️⃣  Checking Dependencies..."
    
    # Check for common system dependencies
    check_command "openssl" "OpenSSL"
    check_command "curl" "cURL"
    check_command "systemctl" "Systemd"
    
    # Check Cargo dependencies
    ((TOTAL_CHECKS++))
    cd "$BEARDOG_ROOT"
    if cargo tree >/dev/null 2>&1; then
        success "Cargo dependency tree is valid"
    else
        error "Cargo dependency issues detected"
    fi
    
    # 7. Compilation Check
    info "7️⃣  Checking Compilation Status..."
    
    ((TOTAL_CHECKS++))
    cd "$BEARDOG_ROOT"
    if timeout 300 cargo check --workspace --release >/dev/null 2>&1; then
        success "Workspace compiles successfully"
    else
        error "Workspace compilation failed"
    fi
    
    # 8. Documentation Check
    info "8️⃣  Checking Documentation..."
    
    check_file "${BEARDOG_ROOT}/README.md" "Main README"
    check_file "${BEARDOG_ROOT}/PRODUCTION_DEPLOYMENT_GUIDE.md" "Production deployment guide"
    check_directory "${BEARDOG_ROOT}/docs" "Documentation directory"
    
    # 9. Monitoring Check
    info "9️⃣  Checking Monitoring Setup..."
    
    check_directory "${BEARDOG_ROOT}/logs" "Logs directory"
    
    # Check for monitoring configuration
    ((TOTAL_CHECKS++))
    if grep -q "monitoring" "${BEARDOG_ROOT}/configs/beardog-config-template.toml" 2>/dev/null; then
        success "Monitoring configuration found in template"
    else
        warning "No monitoring configuration found"
    fi
    
    # 10. Final Summary
    info "🔟 Health Check Summary"
    log "=================================================="
    
    TOTAL_SCORE=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    
    log "📊 Results Summary:"
    log "   Total Checks: $TOTAL_CHECKS"
    log "   ✅ Passed: $PASSED_CHECKS"
    log "   ⚠️  Warnings: $WARNING_CHECKS"
    log "   ❌ Failed: $FAILED_CHECKS"
    log "   📈 Score: $TOTAL_SCORE%"
    
    if [[ $TOTAL_SCORE -ge 90 ]]; then
        success "🎉 EXCELLENT: System is production ready!"
        exit 0
    elif [[ $TOTAL_SCORE -ge 75 ]]; then
        warning "🟡 GOOD: System is mostly ready, address warnings"
        exit 0
    elif [[ $TOTAL_SCORE -ge 60 ]]; then
        warning "🟠 FAIR: System needs attention before production"
        exit 1
    else
        error "🔴 POOR: System is not ready for production"
        exit 2
    fi
}

# Script execution
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi 