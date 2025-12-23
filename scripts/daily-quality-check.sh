#!/bin/bash
# Daily Quality Check for BearDog
# Monitors complexity gradients and quality metrics

echo "🔍 BearDog Daily Quality Check - $(date)"
echo "================================================"

# File Size Check (Complexity Gradient Indicator)
echo -e "\n📏 File Size Analysis:"
echo "   Files exceeding 1000 lines:"
find crates/ -name "*.rs" -not -path "*/target/*" -exec wc -l {} \; | \
  awk '$1 > 1000 {count++; print "   ⚠️  " $2 ": " $1 " lines"} 
  END {if(count==0) print "   ✅ No files exceed 1000 lines"; 
       else print "   Total: " count " files need refactoring"}'

# File Size Distribution
echo -e "\n📊 Complexity Distribution:"
find crates/ -name "*.rs" -not -path "*/target/*" -exec wc -l {} \; | \
  awk 'BEGIN {small=0; medium=0; large=0; xlarge=0; sum=0; count=0}
  {sum+=$1; count++; 
   if($1<200) small++; 
   if($1>=200 && $1<500) medium++; 
   if($1>=500 && $1<1000) large++; 
   if($1>=1000) xlarge++} 
  END {print "   Small (<200 lines):    " small " files ✅"; 
       print "   Medium (200-500):      " medium " files ✅"; 
       print "   Large (500-1000):      " large " files 🟡"; 
       print "   X-Large (>1000):       " xlarge " files 🔴";
       if(count>0) print "   Average file size:     " int(sum/count) " lines"}'

# Test Status
echo -e "\n🧪 Test Status:"
cargo test --workspace --quiet 2>&1 | tail -5

# Quick Clippy Check
echo -e "\n🔍 Clippy Status (quick check):"
cargo clippy --workspace --all-targets 2>&1 | \
  grep -E "^(error:|warning:.*generated)" | head -15 || echo "   ✅ Clippy checks passed"

echo -e "\n✅ Daily quality check complete!"
echo "================================================"
