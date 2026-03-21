#!/usr/bin/env bash
# BearDog Production Health Check Script
# Verifies all systems are ready for production deployment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BEARDOG_URL="${BEARDOG_URL:-http://localhost:8443}"
TIMEOUT="${TIMEOUT:-10}"
VERBOSE="${VERBOSE:-false}"

echo -e "${BLUE}🐻🐕 BearDog Production Health Check${NC}"
echo "=================================================="

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function for verbose logging
log_verbose() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}[VERBOSE]${NC} $1"
    fi
}

# Function to check HTTP endpoint
check_endpoint() {
    local url="$1"
    local name="$2"
    
    log_verbose "Checking $name at $url"
    
    if curl -s --max-time "$TIMEOUT" "$url" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ $name${NC} - OK"
        return 0
    else
        echo -e "${RED}❌ $name${NC} - FAILED"
        return 1
    fi
}

# Function to check service health
check_service_health() {
    local service="$1"
    
    if systemctl is-active --quiet "$service" 2>/dev/null; then
        echo -e "${GREEN}✅ $service service${NC} - Active"
        return 0
    else
        echo -e "${RED}❌ $service service${NC} - Not active"
        return 1
    fi
}

# Function to check disk space
check_disk_space() {
    local threshold=90
    local usage
    usage=$(df / | awk 'NR==2 {print $5}' | sed 's/%//')
    
    if [[ $usage -lt $threshold ]]; then
        echo -e "${GREEN}✅ Disk space${NC} - ${usage}% used"
        return 0
    else
        echo -e "${RED}❌ Disk space${NC} - ${usage}% used (threshold: ${threshold}%)"
        return 1
    fi
}

# Function to check memory usage
check_memory() {
    local threshold=90
    local usage
    usage=$(free | awk 'FNR==2{printf "%.0f", $3/($3+$4)*100}')
    
    if [[ $usage -lt $threshold ]]; then
        echo -e "${GREEN}✅ Memory usage${NC} - ${usage}% used"
        return 0
    else
        echo -e "${RED}❌ Memory usage${NC} - ${usage}% used (threshold: ${threshold}%)"
        return 1
    fi
}

# Function to check load average
check_load() {
    local cpu_count
    local load
    cpu_count=$(nproc)
    load=$(uptime | awk -F'load average:' '{ print $2 }' | cut -d, -f1 | sed 's/^[ \t]*//')
    
    # Convert to integer for comparison
    load_int=$(echo "$load" | cut -d. -f1)
    
    if [[ $load_int -lt $cpu_count ]]; then
        echo -e "${GREEN}✅ Load average${NC} - $load (CPUs: $cpu_count)"
        return 0
    else
        echo -e "${YELLOW}⚠️  Load average${NC} - $load (CPUs: $cpu_count)"
        return 0  # Warning, not failure
    fi
}

# Main health checks
failed_checks=0

echo -e "\n${BLUE}System Health:${NC}"
check_disk_space || ((failed_checks++))
check_memory || ((failed_checks++))
check_load

echo -e "\n${BLUE}BearDog Core Services:${NC}"
check_endpoint "$BEARDOG_URL/api/v1/health" "Core API Health" || ((failed_checks++))
check_endpoint "$BEARDOG_URL/api/v1/status" "System Status" || ((failed_checks++))
check_endpoint "$BEARDOG_URL/metrics" "Metrics Endpoint" || ((failed_checks++))

echo -e "\n${BLUE}Security Services:${NC}"
check_endpoint "$BEARDOG_URL/api/v1/security/status" "Security Engine" || ((failed_checks++))
check_endpoint "$BEARDOG_URL/api/v1/auth/health" "Authentication" || ((failed_checks++))

echo -e "\n${BLUE}Monitoring Services:${NC}"
check_endpoint "$BEARDOG_URL/api/v1/monitoring/health" "Monitoring Engine" || ((failed_checks++))

echo -e "\n${BLUE}Dependencies:${NC}"
if command_exists psql; then
    echo -e "${GREEN}✅ PostgreSQL client${NC} - Available"
else
    echo -e "${RED}❌ PostgreSQL client${NC} - Not found"
    ((failed_checks++))
fi

if command_exists redis-cli; then
    echo -e "${GREEN}✅ Redis client${NC} - Available"
else
    echo -e "${YELLOW}⚠️  Redis client${NC} - Not found (optional)"
fi

# Configuration checks
echo -e "\n${BLUE}Configuration:${NC}"
required_env_vars=(
    "BEARDOG_DATABASE_URL"
    "BEARDOG_SECRET_KEY"
    "BEARDOG_ENCRYPTION_KEY"
)

for var in "${required_env_vars[@]}"; do
    if [[ -n "${!var:-}" ]]; then
        echo -e "${GREEN}✅ $var${NC} - Configured"
    else
        echo -e "${RED}❌ $var${NC} - Missing"
        ((failed_checks++))
    fi
done

# Final summary
echo -e "\n=================================================="
if [[ $failed_checks -eq 0 ]]; then
    echo -e "${GREEN}🎉 All health checks passed! BearDog is ready for production.${NC}"
    exit 0
else
    echo -e "${RED}❌ $failed_checks health check(s) failed. Please review before deployment.${NC}"
    exit 1
fi 