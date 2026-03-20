#!/usr/bin/env bash
# Find Hardcoding Script - Identifies all hardcoded references for elimination
#
# Usage: ./scripts/find-hardcoding.sh [--primal|--vendor|--numeric|--all]

set -euo pipefail

COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_ROOT"

echo -e "${COLOR_BLUE}🔍 Hardcoding Detection Tool${COLOR_RESET}"
echo -e "${COLOR_BLUE}================================${COLOR_RESET}\n"

# Function to count and display results
count_and_display() {
    local pattern="$1"
    local description="$2"
    local color="$3"
    
    echo -e "${color}Searching for: ${description}${COLOR_RESET}"
    
    if command -v rg &> /dev/null; then
        count=$(rg -i "$pattern" --type rust --count crates/ 2>/dev/null | awk -F: '{sum+=$2} END {print sum}' || echo "0")
        files=$(rg -i "$pattern" --type rust --files-with-matches crates/ 2>/dev/null | wc -l || echo "0")
        
        echo -e "  ${COLOR_YELLOW}Matches:${COLOR_RESET} $count across $files files"
        
        if [ "$count" -gt 0 ]; then
            echo -e "  ${COLOR_YELLOW}Sample locations:${COLOR_RESET}"
            rg -i "$pattern" --type rust --max-count 5 --heading --line-number crates/ 2>/dev/null | head -20 || true
        fi
    else
        echo -e "  ${COLOR_RED}ripgrep (rg) not found, install for better results${COLOR_RESET}"
        count=$(grep -r -i "$pattern" --include="*.rs" crates/ 2>/dev/null | wc -l || echo "0")
        echo -e "  ${COLOR_YELLOW}Matches:${COLOR_RESET} $count (approximate)"
    fi
    
    echo ""
}

# Parse command line arguments
MODE="${1:---all}"

case "$MODE" in
    --primal)
        echo -e "${COLOR_RED}❌ PRIMAL NAME HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_RED}===========================${COLOR_RESET}\n"
        
        count_and_display '\b(songbird|toadstool|squirrel|nestgate)\b' "Other Primal Names (songbird, toadstool, squirrel, nestgate)" "$COLOR_RED"
        count_and_display '"beardog[^-]' "Hardcoded 'beardog' (without UUID)" "$COLOR_RED"
        count_and_display 'primal_name\s*=\s*"(songbird|toadstool|squirrel|nestgate|beardog)"' "Hardcoded primal name assignments" "$COLOR_RED"
        ;;
        
    --vendor)
        echo -e "${COLOR_YELLOW}⚠️  VENDOR/PLATFORM HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_YELLOW}==============================${COLOR_RESET}\n"
        
        count_and_display '\b(kubernetes|k8s)\b' "Kubernetes references" "$COLOR_YELLOW"
        count_and_display '\bconsul\b' "Consul references" "$COLOR_YELLOW"
        count_and_display '\b(docker|container)\b' "Docker/Container references" "$COLOR_YELLOW"
        count_and_display '\b(etcd|zookeeper)\b' "etcd/Zookeeper references" "$COLOR_YELLOW"
        count_and_display '\b(vault|hashicorp)\b' "Vault references" "$COLOR_YELLOW"
        count_and_display '\b(redis|memcached)\b' "Redis/Memcached references" "$COLOR_YELLOW"
        count_and_display '\b(postgres|postgresql|mongodb)\b' "Database vendor references" "$COLOR_YELLOW"
        ;;
        
    --numeric)
        echo -e "${COLOR_BLUE}🔢 NUMERIC HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_BLUE}======================${COLOR_RESET}\n"
        
        count_and_display ':\s*\d{4,5}\b' "Hardcoded ports (format: :8080)" "$COLOR_BLUE"
        count_and_display '\bport\s*=\s*\d{4,5}\b' "Port assignments" "$COLOR_BLUE"
        count_and_display '\b(8080|8443|5432|6379|27017|3000|9090)\b' "Common hardcoded ports" "$COLOR_BLUE"
        count_and_display 'Duration::from_secs\(\d+\)' "Hardcoded durations" "$COLOR_BLUE"
        count_and_display 'timeout:\s*\d+' "Hardcoded timeouts" "$COLOR_BLUE"
        ;;
        
    --all|*)
        echo -e "${COLOR_RED}❌ PRIMAL NAME HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_RED}===========================${COLOR_RESET}\n"
        
        count_and_display '\b(songbird|toadstool|squirrel|nestgate)\b' "Other Primal Names" "$COLOR_RED"
        count_and_display '"beardog[^-]' "Hardcoded 'beardog' (without UUID)" "$COLOR_RED"
        
        echo -e "\n${COLOR_YELLOW}⚠️  VENDOR/PLATFORM HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_YELLOW}==============================${COLOR_RESET}\n"
        
        count_and_display '\b(kubernetes|k8s|consul|docker|etcd|vault)\b' "Vendor platforms" "$COLOR_YELLOW"
        
        echo -e "\n${COLOR_BLUE}🔢 NUMERIC HARDCODING${COLOR_RESET}"
        echo -e "${COLOR_BLUE}======================${COLOR_RESET}\n"
        
        count_and_display ':\s*\d{4,5}\b' "Hardcoded ports" "$COLOR_BLUE"
        ;;
esac

echo -e "\n${COLOR_GREEN}📊 Summary${COLOR_RESET}"
echo -e "${COLOR_GREEN}==========${COLOR_RESET}\n"

# Overall statistics
total_primal=$(rg -i '\b(songbird|toadstool|squirrel|nestgate|beardog)\b' --type rust crates/ 2>/dev/null | wc -l || echo "0")
total_vendor=$(rg -i '\b(kubernetes|k8s|consul|docker|etcd|vault|redis|postgres|mongodb)\b' --type rust crates/ 2>/dev/null | wc -l || echo "0")
total_numeric=$(rg ':\s*\d{4,5}\b' --type rust crates/ 2>/dev/null | wc -l || echo "0")

echo -e "${COLOR_YELLOW}Primal name references:${COLOR_RESET} $total_primal"
echo -e "${COLOR_YELLOW}Vendor references:${COLOR_RESET} $total_vendor"
echo -e "${COLOR_YELLOW}Numeric hardcoding:${COLOR_RESET} $total_numeric"
echo -e "${COLOR_YELLOW}Total hardcoding issues:${COLOR_RESET} $((total_primal + total_vendor + total_numeric))"

echo -e "\n${COLOR_GREEN}📝 Next Steps${COLOR_RESET}"
echo -e "${COLOR_GREEN}=============${COLOR_RESET}\n"
echo "1. Review hardcoding hotspots above"
echo "2. Use templates in ecosystem-templates/ for migration patterns"
echo "3. Run: ./scripts/migrate-hardcoding.sh to apply automated fixes"
echo "4. Test with: ./scripts/test-zero-knowledge-deployment.sh"
echo ""
echo "See: HARDCODING_ELIMINATION_PLAN.md for detailed migration strategy"
echo "See: ZERO_KNOWLEDGE_DEPLOYMENT_GUIDE.md for deployment patterns"

