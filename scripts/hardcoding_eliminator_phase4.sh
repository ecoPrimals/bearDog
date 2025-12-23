#!/bin/bash
# Hardcoding Elimination - Systematic Analysis and Fix Script
# Based on ZERO_HARDCODING_SPECIFICATION.md

set -e

echo "═══════════════════════════════════════════════════════════"
echo "   🔧 BEARDOG HARDCODING ELIMINATION - Phase 4"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Create output directory
ANALYSIS_DIR="target/hardcoding-analysis"
mkdir -p "$ANALYSIS_DIR"

# Timestamp for this run
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$ANALYSIS_DIR/hardcoding_report_$TIMESTAMP.md"

echo "# Hardcoding Analysis Report" > "$REPORT_FILE"
echo "**Date**: $(date)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Function to count and report
count_and_report() {
    local pattern="$1"
    local description="$2"
    local output_file="$3"
    
    echo "Analyzing: $description..."
    
    # Search excluding test files and docs
    grep -rn "$pattern" crates/ \
        --include="*.rs" \
        --exclude-dir="target" \
        --exclude-dir="archive" \
        | grep -v "/tests/" \
        | grep -v "_tests.rs" \
        | grep -v "_test.rs" \
        | grep -v "test_" \
        > "$output_file" 2>/dev/null || true
    
    count=$(wc -l < "$output_file" || echo "0")
    echo "  Found: $count instances"
    
    echo "### $description" >> "$REPORT_FILE"
    echo "**Count**: $count instances" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    if [ "$count" -gt 0 ]; then
        echo "**Top 10 Files**:" >> "$REPORT_FILE"
        echo "\`\`\`" >> "$REPORT_FILE"
        cut -d: -f1 "$output_file" | sort | uniq -c | sort -rn | head -10 >> "$REPORT_FILE"
        echo "\`\`\`" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
    fi
}

echo ""
echo "📊 PHASE 1: PRODUCTION CODE ANALYSIS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Analyze localhost patterns
count_and_report 'localhost:[0-9]' \
    "Localhost with Port" \
    "$ANALYSIS_DIR/localhost_with_port.txt"

# Analyze IP addresses
count_and_report '127\.0\.0\.1' \
    "Loopback IP (127.0.0.1)" \
    "$ANALYSIS_DIR/loopback_ip.txt"

count_and_report '0\.0\.0\.0' \
    "All Interfaces (0.0.0.0)" \
    "$ANALYSIS_DIR/all_interfaces.txt"

# Analyze port constants
count_and_report 'const.*PORT.*:.*u16.*=.*[0-9]{4}' \
    "Port Constants" \
    "$ANALYSIS_DIR/port_constants.txt"

# Analyze specific common ports
count_and_report ':8080\|:8081\|:8082\|:8083' \
    "Common Service Ports (8080-8083)" \
    "$ANALYSIS_DIR/common_ports.txt"

count_and_report ':5432\|:6379\|:27017' \
    "Database Ports (PostgreSQL/Redis/MongoDB)" \
    "$ANALYSIS_DIR/database_ports.txt"

echo ""
echo "📋 PHASE 2: CRITICAL FILES ANALYSIS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Critical files from documentation
CRITICAL_FILES=(
    "crates/beardog-types/src/canonical/config/runtime_config.rs"
    "crates/beardog-types/src/constants/domains/network.rs"
    "crates/beardog-config/src/domains/network_ports.rs"
    "crates/beardog-config/src/domains/network_hosts.rs"
    "crates/beardog-config/src/domains/network_addresses.rs"
)

echo "## Critical Files Analysis" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

for file in "${CRITICAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "Checking: $file"
        
        # Count hardcoded values in this file
        hardcoded_count=$(grep -E '(localhost|127\.0\.0\.1|0\.0\.0\.0|:[0-9]{4})' "$file" | wc -l || echo "0")
        
        echo "### \`$(basename $file)\`" >> "$REPORT_FILE"
        echo "**Path**: \`$file\`" >> "$REPORT_FILE"
        echo "**Hardcoded Values**: $hardcoded_count" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        if [ "$hardcoded_count" -gt 0 ]; then
            echo "\`\`\`" >> "$REPORT_FILE"
            grep -n -E '(localhost|127\.0\.0\.1|0\.0\.0\.0|:[0-9]{4})' "$file" | head -15 >> "$REPORT_FILE"
            echo "\`\`\`" >> "$REPORT_FILE"
        else
            echo "✅ **CLEAN** - No hardcoded values found!" >> "$REPORT_FILE"
        fi
        echo "" >> "$REPORT_FILE"
    fi
done

echo ""
echo "📊 PHASE 3: SUMMARY STATISTICS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Calculate totals
total_localhost=$(wc -l < "$ANALYSIS_DIR/localhost_with_port.txt" || echo "0")
total_loopback=$(wc -l < "$ANALYSIS_DIR/loopback_ip.txt" || echo "0")
total_all_interfaces=$(wc -l < "$ANALYSIS_DIR/all_interfaces.txt" || echo "0")
total_ports=$(wc -l < "$ANALYSIS_DIR/common_ports.txt" || echo "0")
total_db_ports=$(wc -l < "$ANALYSIS_DIR/database_ports.txt" || echo "0")

grand_total=$((total_localhost + total_loopback + total_all_interfaces + total_ports + total_db_ports))

echo "## Summary Statistics" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "| Category | Count |" >> "$REPORT_FILE"
echo "|----------|-------|" >> "$REPORT_FILE"
echo "| Localhost with Port | $total_localhost |" >> "$REPORT_FILE"
echo "| Loopback IP (127.0.0.1) | $total_loopback |" >> "$REPORT_FILE"
echo "| All Interfaces (0.0.0.0) | $total_all_interfaces |" >> "$REPORT_FILE"
echo "| Common Service Ports | $total_ports |" >> "$REPORT_FILE"
echo "| Database Ports | $total_db_ports |" >> "$REPORT_FILE"
echo "| **TOTAL (Production)** | **$grand_total** |" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "Summary:"
echo "  Localhost with Port:        $total_localhost"
echo "  Loopback IP (127.0.0.1):    $total_loopback"
echo "  All Interfaces (0.0.0.0):   $total_all_interfaces"
echo "  Common Service Ports:       $total_ports"
echo "  Database Ports:             $total_db_ports"
echo "  ═══════════════════════════"
echo "  TOTAL (Production Code):    $grand_total"

echo ""
echo "🎯 PHASE 4: ACTION ITEMS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## Action Items" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ "$grand_total" -gt 0 ]; then
    echo "🔴 **WORK REQUIRED**: $grand_total hardcoded values found" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "### Recommended Actions:" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "1. **Review** detailed files in \`$ANALYSIS_DIR/\`" >> "$REPORT_FILE"
    echo "2. **Prioritize** critical files (runtime_config, network constants)" >> "$REPORT_FILE"
    echo "3. **Migrate** to \`BEARDOG_CONFIG\` or environment variables" >> "$REPORT_FILE"
    echo "4. **Test** each migration thoroughly" >> "$REPORT_FILE"
    echo "5. **Verify** zero hardcoding spec compliance" >> "$REPORT_FILE"
    
    echo "🔴 WORK REQUIRED: $grand_total hardcoded values need migration"
    echo ""
    echo "Priority Actions:"
    echo "  1. Review detailed analysis in: $ANALYSIS_DIR/"
    echo "  2. Start with critical files"
    echo "  3. Migrate to BEARDOG_CONFIG"
    echo "  4. Test thoroughly"
else
    echo "✅ **PRODUCTION CODE CLEAN**: No hardcoded values in production!" >> "$REPORT_FILE"
    echo ""
    echo "✅ EXCELLENT: Production code is clean!"
fi

echo "" >> "$REPORT_FILE"
echo "## Next Steps" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "- [ ] Review this report" >> "$REPORT_FILE"
echo "- [ ] Assign files to team members" >> "$REPORT_FILE"
echo "- [ ] Create PRs for each file" >> "$REPORT_FILE"
echo "- [ ] Update tests to use config" >> "$REPORT_FILE"
echo "- [ ] Verify with \`cargo test --workspace\`" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "---" >> "$REPORT_FILE"
echo "*Generated by: scripts/hardcoding_eliminator_phase4.sh*" >> "$REPORT_FILE"

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "   ✅ ANALYSIS COMPLETE"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "📄 Report saved to: $REPORT_FILE"
echo ""
echo "📂 Detailed analysis files:"
echo "   $ANALYSIS_DIR/localhost_with_port.txt"
echo "   $ANALYSIS_DIR/loopback_ip.txt"
echo "   $ANALYSIS_DIR/all_interfaces.txt"
echo "   $ANALYSIS_DIR/port_constants.txt"
echo "   $ANALYSIS_DIR/common_ports.txt"
echo "   $ANALYSIS_DIR/database_ports.txt"
echo ""
echo "🎯 Next: Review report and begin systematic elimination"
echo ""

