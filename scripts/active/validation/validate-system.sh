#!/bin/bash
set -euo pipefail

# BearDog 0.9.0 System Validation Script
# Comprehensive validation of BearDog deployment

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
VALIDATION_LOG="/tmp/beardog-validation-$(date +%Y%m%d-%H%M%S).log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BEARDOG_HOME="/opt/beardog"
SERVICE_NAME="beardog"
HEALTH_URL="http://localhost:8080/health"
METRICS_URL="http://localhost:8080/metrics"

# Validation results
PASSED_TESTS=0
FAILED_TESTS=0
WARNING_TESTS=0
TOTAL_TESTS=0

log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}" | tee -a "$VALIDATION_LOG"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}" | tee -a "$VALIDATION_LOG"
    ((WARNING_TESTS++))
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}" | tee -a "$VALIDATION_LOG"
    ((FAILED_TESTS++))
}

info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] INFO: $1${NC}" | tee -a "$VALIDATION_LOG"
}

test_pass() {
    echo -e "${GREEN}✅ PASS: $1${NC}" | tee -a "$VALIDATION_LOG"
    ((PASSED_TESTS++))
    ((TOTAL_TESTS++))
}

test_fail() {
    echo -e "${RED}❌ FAIL: $1${NC}" | tee -a "$VALIDATION_LOG"
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
}

test_warn() {
    echo -e "${YELLOW}⚠️  WARN: $1${NC}" | tee -a "$VALIDATION_LOG"
    ((WARNING_TESTS++))
    ((TOTAL_TESTS++))
}

validate_build_system() {
    log "🔨 Validating Build System"
    
    cd "$PROJECT_ROOT"
    
    # Check if we can build
    info "Testing build system..."
    if cargo check --all --all-features --quiet 2>/dev/null; then
        test_pass "Cargo build system is functional"
    else
        test_fail "Cargo build system has issues"
    fi
    
    # Check Rust version (workspace MSRV)
    local rust_version=$(rustc --version | awk '{print $2}')
    local min_rust="1.93.0"
    info "Rust version: $rust_version"
    if [[ "$rust_version" == "$(printf '%s\n' "$rust_version" "$min_rust" | sort -V | tail -n1)" ]]; then
        test_pass "Rust version is compatible ($rust_version, MSRV $min_rust+)"
    else
        test_fail "Rust version too old ($rust_version, need $min_rust+)"
    fi
    
    # Check for target directory and artifacts
    if [[ -d "target/release" ]] && [[ -f "target/release/beardog" ]]; then
        test_pass "Build artifacts present"
    else
        test_warn "Build artifacts not found (may need to run build)"
    fi
}

validate_system_requirements() {
    log "🖥️ Validating System Requirements"
    
    # Check available memory
    local available_mem=$(free -m | awk 'NR==2{printf "%.0f", $7}')
    info "Available memory: ${available_mem}MB"
    if [[ $available_mem -ge 4096 ]]; then
        test_pass "Sufficient memory available (${available_mem}MB)"
    elif [[ $available_mem -ge 2048 ]]; then
        test_warn "Memory is adequate but below recommended (${available_mem}MB, recommended: 4GB)"
    else
        test_fail "Insufficient memory (${available_mem}MB, minimum: 2GB)"
    fi
    
    # Check disk space
    local available_disk=$(df -BM "$PROJECT_ROOT" | awk 'NR==2 {print $4}' | sed 's/M//')
    info "Available disk space: ${available_disk}MB"
    if [[ $available_disk -ge 2048 ]]; then
        test_pass "Sufficient disk space (${available_disk}MB)"
    else
        test_warn "Limited disk space (${available_disk}MB, recommended: 2GB+)"
    fi
    
    # Check CPU cores
    local cpu_cores=$(nproc)
    info "CPU cores: $cpu_cores"
    if [[ $cpu_cores -ge 2 ]]; then
        test_pass "Sufficient CPU cores ($cpu_cores)"
    else
        test_warn "Limited CPU cores ($cpu_cores, recommended: 2+)"
    fi
}

validate_dependencies() {
    log "📦 Validating Dependencies"
    
    # Check required system commands
    local required_commands=("cargo" "rustc" "systemctl" "curl" "openssl")
    for cmd in "${required_commands[@]}"; do
        if command -v "$cmd" &> /dev/null; then
            test_pass "Command '$cmd' is available"
        else
            test_fail "Required command '$cmd' not found"
        fi
    done
    
    # Check optional but recommended commands
    local optional_commands=("git" "jq" "htop" "netstat")
    for cmd in "${optional_commands[@]}"; do
        if command -v "$cmd" &> /dev/null; then
            test_pass "Optional command '$cmd' is available"
        else
            test_warn "Optional command '$cmd' not found"
        fi
    done
}

validate_installation() {
    log "📦 Validating Installation"
    
    # Check if BearDog user exists
    if id "beardog" &>/dev/null; then
        test_pass "BearDog system user exists"
        
        # Check user properties
        local user_shell=$(getent passwd beardog | cut -d: -f7)
        if [[ "$user_shell" == "/bin/false" ]] || [[ "$user_shell" == "/usr/sbin/nologin" ]]; then
            test_pass "System user has secure shell"
        else
            test_warn "System user shell is not restricted ($user_shell)"
        fi
    else
        test_warn "BearDog system user not found (not deployed yet?)"
    fi
    
    # Check directory structure
    local required_dirs=("$BEARDOG_HOME" "$BEARDOG_HOME/bin" "$BEARDOG_HOME/config" "$BEARDOG_HOME/logs")
    for dir in "${required_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            test_pass "Directory exists: $dir"
        else
            test_warn "Directory missing: $dir"
        fi
    done
    
    # Check binary
    if [[ -f "$BEARDOG_HOME/bin/beardog" ]] && [[ -x "$BEARDOG_HOME/bin/beardog" ]]; then
        test_pass "BearDog binary is installed and executable"
    else
        test_warn "BearDog binary not found or not executable"
    fi
    
    # Check configuration
    if [[ -f "$BEARDOG_HOME/config/production.toml" ]]; then
        test_pass "Production configuration exists"
    else
        test_warn "Production configuration not found"
    fi
}

validate_systemd_service() {
    log "🔧 Validating Systemd Service"
    
    # Check if service file exists
    if [[ -f "/etc/systemd/system/${SERVICE_NAME}.service" ]]; then
        test_pass "Systemd service file exists"
    else
        test_fail "Systemd service file not found"
        return
    fi
    
    # Check if service is enabled
    if systemctl is-enabled "$SERVICE_NAME" &>/dev/null; then
        test_pass "Service is enabled"
    else
        test_warn "Service is not enabled"
    fi
    
    # Check service status
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        test_pass "Service is running"
        
        # Check service health
        local start_time=$(systemctl show "$SERVICE_NAME" --property=ActiveEnterTimestamp --value)
        info "Service started: $start_time"
        
        # Check for recent restarts
        local restart_count=$(systemctl show "$SERVICE_NAME" --property=NRestarts --value)
        if [[ "$restart_count" -eq 0 ]]; then
            test_pass "Service has not restarted (stable)"
        elif [[ "$restart_count" -lt 5 ]]; then
            test_warn "Service has restarted $restart_count times"
        else
            test_fail "Service has restarted $restart_count times (unstable)"
        fi
    else
        test_fail "Service is not running"
    fi
}

validate_network_connectivity() {
    log "🌐 Validating Network Connectivity"
    
    # Check if ports are listening
    local ports=("8080")
    for port in "${ports[@]}"; do
        if netstat -ln 2>/dev/null | grep -q ":$port " || ss -ln 2>/dev/null | grep -q ":$port "; then
            test_pass "Port $port is listening"
        else
            test_warn "Port $port is not listening"
        fi
    done
    
    # Test health endpoint with retries
    info "Testing health endpoint..."
    local max_attempts=5
    local attempt=1
    local health_success=false
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -f -s --max-time 5 "$HEALTH_URL" >/dev/null 2>&1; then
            health_success=true
            break
        fi
        sleep 1
        ((attempt++))
    done
    
    if [[ "$health_success" == true ]]; then
        test_pass "Health endpoint is responding"
        
        # Test health endpoint content
        local health_response=$(curl -s --max-time 5 "$HEALTH_URL" 2>/dev/null || echo "")
        if [[ -n "$health_response" ]]; then
            info "Health response: $health_response"
            if echo "$health_response" | grep -q "healthy\|ok\|up" -i; then
                test_pass "Health endpoint reports healthy status"
            else
                test_warn "Health endpoint response unclear"
            fi
        fi
    else
        test_fail "Health endpoint not responding after $max_attempts attempts"
    fi
}

validate_security_configuration() {
    log "🔒 Validating Security Configuration"
    
    # Check file permissions
    if [[ -f "$BEARDOG_HOME/config/production.toml" ]]; then
        local config_perms=$(stat -c "%a" "$BEARDOG_HOME/config/production.toml")
        if [[ "$config_perms" == "640" ]] || [[ "$config_perms" == "600" ]]; then
            test_pass "Configuration file has secure permissions ($config_perms)"
        else
            test_warn "Configuration file permissions may be too open ($config_perms)"
        fi
    fi
    
    # Check for TLS configuration
    if [[ -d "/etc/beardog/certs" ]]; then
        test_pass "TLS certificate directory exists"
        
        if ls /etc/beardog/certs/*.crt >/dev/null 2>&1; then
            test_pass "TLS certificates found"
        else
            test_warn "No TLS certificates found"
        fi
    else
        test_warn "TLS certificate directory not found"
    fi
    
    # Check firewall status
    if command -v ufw >/dev/null && ufw status | grep -q "Status: active"; then
        test_pass "UFW firewall is active"
    elif command -v firewall-cmd >/dev/null && firewall-cmd --state 2>/dev/null | grep -q "running"; then
        test_pass "Firewalld is running"
    else
        test_warn "No active firewall detected"
    fi
}

validate_performance() {
    log "⚡ Validating Performance"
    
    # Check system load
    local load_avg=$(uptime | awk -F'load average:' '{print $2}' | awk '{print $1}' | sed 's/,//')
    local cpu_cores=$(nproc)
    local load_per_core=$(echo "scale=2; $load_avg / $cpu_cores" | bc -l 2>/dev/null || echo "0")
    
    info "System load: $load_avg (${load_per_core} per core)"
    if (( $(echo "$load_per_core < 1.0" | bc -l 2>/dev/null || echo 0) )); then
        test_pass "System load is normal"
    elif (( $(echo "$load_per_core < 2.0" | bc -l 2>/dev/null || echo 0) )); then
        test_warn "System load is elevated"
    else
        test_fail "System load is high"
    fi
    
    # Check memory usage
    local mem_usage=$(free | awk 'NR==2{printf "%.1f", $3/$2*100}')
    info "Memory usage: ${mem_usage}%"
    if (( $(echo "$mem_usage < 80" | bc -l 2>/dev/null || echo 0) )); then
        test_pass "Memory usage is normal (${mem_usage}%)"
    elif (( $(echo "$mem_usage < 90" | bc -l 2>/dev/null || echo 0) )); then
        test_warn "Memory usage is elevated (${mem_usage}%)"
    else
        test_fail "Memory usage is high (${mem_usage}%)"
    fi
    
    # Test response time
    if curl -f -s --max-time 5 "$HEALTH_URL" >/dev/null 2>&1; then
        local response_time=$(curl -o /dev/null -s -w "%{time_total}" --max-time 5 "$HEALTH_URL" 2>/dev/null || echo "999")
        local response_ms=$(echo "scale=0; $response_time * 1000" | bc -l 2>/dev/null || echo "999")
        
        info "Health endpoint response time: ${response_ms}ms"
        if (( $(echo "$response_ms < 100" | bc -l 2>/dev/null || echo 0) )); then
            test_pass "Response time is excellent (${response_ms}ms)"
        elif (( $(echo "$response_ms < 500" | bc -l 2>/dev/null || echo 0) )); then
            test_pass "Response time is good (${response_ms}ms)"
        else
            test_warn "Response time is slow (${response_ms}ms)"
        fi
    fi
}

validate_logging() {
    log "📝 Validating Logging"
    
    # Check systemd journal
    if journalctl -u "$SERVICE_NAME" --since "1 hour ago" --quiet --no-pager >/dev/null 2>&1; then
        test_pass "Systemd logging is functional"
        
        # Check for recent errors
        local error_count=$(journalctl -u "$SERVICE_NAME" --since "1 hour ago" --no-pager -p err 2>/dev/null | wc -l)
        if [[ "$error_count" -eq 0 ]]; then
            test_pass "No recent errors in logs"
        elif [[ "$error_count" -lt 5 ]]; then
            test_warn "$error_count recent errors found in logs"
        else
            test_fail "$error_count recent errors found in logs"
        fi
    else
        test_warn "Cannot access systemd logs"
    fi
    
    # Check log rotation
    if [[ -f "/etc/logrotate.d/beardog" ]]; then
        test_pass "Log rotation is configured"
    else
        test_warn "Log rotation not configured"
    fi
}

validate_monitoring() {
    log "📊 Validating Monitoring"
    
    # Check if health check script exists
    if [[ -f "$BEARDOG_HOME/bin/health-check.sh" ]] && [[ -x "$BEARDOG_HOME/bin/health-check.sh" ]]; then
        test_pass "Health check script exists and is executable"
        
        # Test the health check script
        if "$BEARDOG_HOME/bin/health-check.sh" >/dev/null 2>&1; then
            test_pass "Health check script reports healthy"
        else
            test_warn "Health check script reports unhealthy"
        fi
    else
        test_warn "Health check script not found"
    fi
    
    # Check metrics endpoint if available
    if curl -f -s --max-time 5 "$METRICS_URL" >/dev/null 2>&1; then
        test_pass "Metrics endpoint is available"
    else
        test_warn "Metrics endpoint not available"
    fi
}

generate_validation_report() {
    log "📋 Generating Validation Report"
    
    local total_score=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    local report_file="/tmp/beardog-validation-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > "$report_file" << EOF
# BearDog 0.9.0 System Validation Report

**Date**: $(date)
**Validation Score**: $total_score% ($PASSED_TESTS/$TOTAL_TESTS tests passed)

## Summary
- ✅ **Passed**: $PASSED_TESTS tests
- ⚠️ **Warnings**: $WARNING_TESTS tests  
- ❌ **Failed**: $FAILED_TESTS tests
- 📊 **Total**: $TOTAL_TESTS tests

## Overall Status
EOF

    if [[ $FAILED_TESTS -eq 0 ]] && [[ $WARNING_TESTS -eq 0 ]]; then
        echo "🎉 **EXCELLENT** - All tests passed!" >> "$report_file"
    elif [[ $FAILED_TESTS -eq 0 ]]; then
        echo "✅ **GOOD** - All critical tests passed with minor warnings" >> "$report_file"
    elif [[ $FAILED_TESTS -lt 3 ]]; then
        echo "⚠️ **NEEDS ATTENTION** - Some issues found" >> "$report_file"
    else
        echo "❌ **CRITICAL ISSUES** - Multiple failures detected" >> "$report_file"
    fi
    
    cat >> "$report_file" << EOF

## Recommendations

EOF

    if [[ $FAILED_TESTS -gt 0 ]]; then
        echo "- Address failed tests before production deployment" >> "$report_file"
    fi
    
    if [[ $WARNING_TESTS -gt 0 ]]; then
        echo "- Review warnings for optimization opportunities" >> "$report_file"
    fi
    
    if [[ $FAILED_TESTS -eq 0 ]] && [[ $WARNING_TESTS -eq 0 ]]; then
        echo "- System is ready for production deployment! 🚀" >> "$report_file"
    fi
    
    echo "" >> "$report_file"
    echo "**Detailed Log**: $VALIDATION_LOG" >> "$report_file"
    
    info "Validation report saved to: $report_file"
    return "$report_file"
}

print_validation_summary() {
    local total_score=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    
    echo
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}  BearDog 0.9.0 Validation Summary      ${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo
    echo -e "${BLUE}Validation Score:${NC} $total_score% ($PASSED_TESTS/$TOTAL_TESTS tests passed)"
    echo -e "${BLUE}Test Results:${NC}"
    echo -e "  ✅ Passed: $PASSED_TESTS"
    echo -e "  ⚠️  Warnings: $WARNING_TESTS"
    echo -e "  ❌ Failed: $FAILED_TESTS"
    echo
    
    if [[ $FAILED_TESTS -eq 0 ]] && [[ $WARNING_TESTS -eq 0 ]]; then
        echo -e "${GREEN}🎉 EXCELLENT - System is production ready!${NC}"
    elif [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${YELLOW}✅ GOOD - System is ready with minor optimizations needed${NC}"
    elif [[ $FAILED_TESTS -lt 3 ]]; then
        echo -e "${YELLOW}⚠️  NEEDS ATTENTION - Address issues before production${NC}"
    else
        echo -e "${RED}❌ CRITICAL ISSUES - System needs significant work${NC}"
    fi
    
    echo
    echo -e "${BLUE}Detailed Log:${NC} $VALIDATION_LOG"
    echo
}

main() {
    log "🔍 Starting BearDog 0.9.0 System Validation"
    
    validate_build_system
    validate_system_requirements
    validate_dependencies
    validate_installation
    validate_systemd_service
    validate_network_connectivity
    validate_security_configuration
    validate_performance
    validate_logging
    validate_monitoring
    
    generate_validation_report
    print_validation_summary
    
    # Exit with appropriate code
    if [[ $FAILED_TESTS -eq 0 ]]; then
        log "✅ Validation completed successfully"
        exit 0
    else
        log "❌ Validation completed with failures"
        exit 1
    fi
}

# Handle script interruption
trap 'echo "Validation interrupted" | tee -a "$VALIDATION_LOG"' INT TERM

# Run main function
main "$@" 