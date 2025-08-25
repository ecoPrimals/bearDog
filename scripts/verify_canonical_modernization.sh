#!/bin/bash
# BearDog Canonical Modernization Verification Script
# Verifies the completion of type unification, trait consolidation, and modernization

set -e

echo "🔍 BearDog Canonical Modernization Verification"
echo "=============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
WARNINGS=0

check_result() {
    local description="$1"
    local result="$2"
    local warning="$3"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    if [ "$result" = "PASS" ]; then
        echo -e "${GREEN}✅ PASS${NC}: $description"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    elif [ "$result" = "WARN" ]; then
        echo -e "${YELLOW}⚠️  WARN${NC}: $description"
        if [ -n "$warning" ]; then
            echo -e "   ${YELLOW}→${NC} $warning"
        fi
        WARNINGS=$((WARNINGS + 1))
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    else
        echo -e "${RED}❌ FAIL${NC}: $description"
        if [ -n "$warning" ]; then
            echo -e "   ${RED}→${NC} $warning"
        fi
    fi
}

echo -e "${BLUE}📊 CANONICAL TYPE SYSTEM VERIFICATION${NC}"
echo "--------------------------------------------"

# Check if canonical traits exist
if [ -f "crates/beardog-traits/src/canonical.rs" ]; then
    check_result "Canonical traits file exists" "PASS"
else
    check_result "Canonical traits file exists" "FAIL" "crates/beardog-traits/src/canonical.rs not found"
fi

# Check if type aliases are centralized
if [ -f "crates/beardog-types/src/aliases.rs" ]; then
    check_result "Centralized type aliases file exists" "PASS"
else
    check_result "Centralized type aliases file exists" "FAIL" "crates/beardog-types/src/aliases.rs not found"
fi

# Check for fragmented HsmProvider traits (should be deprecated)
hsm_provider_count=$(grep -r "pub trait HsmProvider" crates/ --include="*.rs" | wc -l)
if [ "$hsm_provider_count" -le 3 ]; then
    check_result "HsmProvider trait consolidation" "PASS" "Found $hsm_provider_count definitions (expected ≤3 with deprecations)"
else
    check_result "HsmProvider trait consolidation" "WARN" "Found $hsm_provider_count definitions, some may need deprecation"
fi

# Check for fragmented WorkflowProcessor traits (should be deprecated)
workflow_processor_count=$(grep -r "pub trait WorkflowProcessor" crates/ --include="*.rs" | wc -l)
if [ "$workflow_processor_count" -le 4 ]; then
    check_result "WorkflowProcessor trait consolidation" "PASS" "Found $workflow_processor_count definitions (expected ≤4 with deprecations)"
else
    check_result "WorkflowProcessor trait consolidation" "WARN" "Found $workflow_processor_count definitions, some may need deprecation"
fi

# Check for fragmented DiscoveryBackend traits
discovery_backend_count=$(grep -r "pub trait.*Discovery" crates/ --include="*.rs" | wc -l)
if [ "$discovery_backend_count" -le 8 ]; then
    check_result "Discovery trait consolidation" "PASS" "Found $discovery_backend_count discovery-related traits"
else
    check_result "Discovery trait consolidation" "WARN" "Found $discovery_backend_count discovery-related traits, consider further consolidation"
fi

echo ""
echo -e "${BLUE}📏 FILE SIZE COMPLIANCE VERIFICATION${NC}"
echo "-------------------------------------------"

# Check for files exceeding 2000 lines
large_files=$(find crates/ -name "*.rs" -not -path "*/target/*" -exec wc -l {} + | awk '$1 > 2000 {print $2 " (" $1 " lines)"}' | head -10)

if [ -z "$large_files" ]; then
    check_result "All source files under 2000 lines" "PASS"
else
    check_result "All source files under 2000 lines" "FAIL" "Files exceeding limit found"
    echo -e "   ${RED}Large files:${NC}"
    echo "$large_files" | while read line; do
        echo -e "   ${RED}→${NC} $line"
    done
fi

echo ""
echo -e "${BLUE}🚀 MODERNIZATION PATTERN VERIFICATION${NC}"
echo "---------------------------------------------"

# Check for async_trait usage (should be minimized)
async_trait_count=$(grep -r "use async_trait::async_trait" crates/ --include="*.rs" | wc -l)
if [ "$async_trait_count" -le 20 ]; then
    check_result "async_trait usage minimized" "PASS" "Found $async_trait_count usages (acceptable for legacy compatibility)"
elif [ "$async_trait_count" -le 50 ]; then
    check_result "async_trait usage minimized" "WARN" "Found $async_trait_count usages, consider migration to native async fn"
else
    check_result "async_trait usage minimized" "FAIL" "Found $async_trait_count usages, significant migration needed"
fi

# Check for deprecated attribute usage
deprecated_count=$(grep -r "#\[deprecated" crates/ --include="*.rs" | wc -l)
if [ "$deprecated_count" -gt 0 ]; then
    check_result "Deprecation attributes in use" "PASS" "Found $deprecated_count deprecation markers (good for migration)"
else
    check_result "Deprecation attributes in use" "WARN" "No deprecation markers found, ensure fragmented traits are marked"
fi

# Check for canonical imports
canonical_imports=$(grep -r "use beardog_traits::canonical" crates/ --include="*.rs" | wc -l)
if [ "$canonical_imports" -gt 0 ]; then
    check_result "Canonical trait imports in use" "PASS" "Found $canonical_imports canonical imports"
else
    check_result "Canonical trait imports in use" "WARN" "No canonical imports found, migration may be incomplete"
fi

echo ""
echo -e "${BLUE}🔧 CONFIGURATION SYSTEM VERIFICATION${NC}"
echo "-------------------------------------"

# Check for unified constants
if [ -f "crates/beardog-types/src/constants/unified.rs" ]; then
    check_result "Unified constants system exists" "PASS"
else
    check_result "Unified constants system exists" "FAIL" "Unified constants file not found"
fi

# Check for configuration consolidation
config_files=$(find crates/beardog-types/src/config/ -name "*.rs" 2>/dev/null | wc -l)
if [ "$config_files" -gt 10 ]; then
    check_result "Configuration system modularized" "PASS" "Found $config_files configuration modules"
else
    check_result "Configuration system modularized" "WARN" "Found only $config_files configuration modules"
fi

echo ""
echo -e "${BLUE}📈 FINAL ASSESSMENT${NC}"
echo "-------------------"

# Calculate success rate
success_rate=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))

echo -e "Total Checks: ${BLUE}$TOTAL_CHECKS${NC}"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Warnings: ${YELLOW}$WARNINGS${NC}"
echo -e "Success Rate: ${GREEN}$success_rate%${NC}"

echo ""

if [ "$success_rate" -ge 90 ]; then
    echo -e "${GREEN}🎉 EXCELLENT${NC}: BearDog canonical modernization is ${GREEN}highly successful${NC}!"
    echo -e "   The codebase demonstrates excellent unification and modernization."
elif [ "$success_rate" -ge 80 ]; then
    echo -e "${YELLOW}👍 GOOD${NC}: BearDog canonical modernization is ${YELLOW}largely successful${NC}!"
    echo -e "   Minor improvements recommended for complete modernization."
elif [ "$success_rate" -ge 70 ]; then
    echo -e "${YELLOW}⚠️  MODERATE${NC}: BearDog canonical modernization shows ${YELLOW}good progress${NC}."
    echo -e "   Additional work needed to complete full modernization."
else
    echo -e "${RED}❌ NEEDS WORK${NC}: BearDog canonical modernization ${RED}requires attention${NC}."
    echo -e "   Significant improvements needed for modernization goals."
fi

echo ""
echo -e "${BLUE}📋 NEXT STEPS RECOMMENDATION${NC}"
echo "-----------------------------"

if [ "$success_rate" -ge 90 ]; then
    echo "✅ Focus on performance optimization and documentation updates"
    echo "✅ Consider ecosystem-wide adoption of these patterns"
    echo "✅ Create migration guides for other ecoPrimals projects"
elif [ "$success_rate" -ge 80 ]; then
    echo "🔧 Complete remaining trait consolidations"
    echo "🔧 Migrate remaining async_trait usage to native async fn"
    echo "🔧 Verify all canonical imports are working correctly"
else
    echo "⚠️  Complete file size compliance (split large files)"
    echo "⚠️  Finish trait consolidation and deprecation"
    echo "⚠️  Implement comprehensive type alias migration"
fi

echo ""
echo "Verification complete! 🚀" 