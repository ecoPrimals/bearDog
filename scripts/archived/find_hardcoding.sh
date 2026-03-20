#!/bin/bash
# Find all hardcoded IPs, ports, and URLs in production code

echo "=========================================="
echo "Finding Hardcoded Configuration"
echo "=========================================="
echo ""

# Find hardcoded IPs
echo "=== HARDCODED IPs ==="
grep -rE "(localhost|127\.0\.0\.1|0\.0\.0\.0)" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|example\|doc\|comment" \
  > /tmp/hardcoded_ips.txt

ip_count=$(wc -l < /tmp/hardcoded_ips.txt)
echo "Hardcoded IPs found: $ip_count"
head -15 /tmp/hardcoded_ips.txt
echo "..."
echo ""

# Find hardcoded ports
echo "=== HARDCODED PORTS ==="
grep -rE ":(8080|8081|8082|8083|3000|5432|6379|9090|27017)\b" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|example\|doc\|comment" \
  > /tmp/hardcoded_ports.txt

port_count=$(wc -l < /tmp/hardcoded_ports.txt)
echo "Hardcoded ports found: $port_count"
head -15 /tmp/hardcoded_ports.txt
echo "..."
echo ""

# Priority files
echo "=========================================="
echo "TOP PRIORITY FILES (Most Hardcoding):"
echo "=========================================="

cat /tmp/hardcoded_ips.txt /tmp/hardcoded_ports.txt | \
  cut -d: -f1 | \
  sort | uniq -c | \
  sort -rn | \
  head -20

echo ""
echo "Full results saved to:"
echo "  - /tmp/hardcoded_ips.txt"
echo "  - /tmp/hardcoded_ports.txt"
echo ""

# Show specific critical files
echo "=========================================="
echo "CRITICAL FILES TO FIX:"
echo "=========================================="
echo ""
echo "1. runtime_config.rs:"
grep -n "localhost\|127\.0\.0\.1\|:8080\|:9090" crates/beardog-types/src/canonical/config/runtime_config.rs 2>/dev/null | head -10
echo ""
echo "2. constants/domains/network.rs:"
grep -n "const.*PORT" crates/beardog-types/src/constants/domains/network.rs 2>/dev/null | head -10
echo ""
echo "3. env_config.rs:"
grep -n "localhost\|127\.0\.0\.1\|:5432\|:6379" crates/beardog-utils/src/env_config.rs 2>/dev/null | head -10

