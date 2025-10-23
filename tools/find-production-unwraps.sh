#!/bin/bash
# Simple script to find production unwraps (excluding tests)

echo "🔍 Finding production unwraps in beardog..."
echo "=========================================="
echo ""

# Find non-test files with unwraps
production_files=$(find crates/beardog-core/src crates/beardog-adapters/src \
  -name "*.rs" \
  ! -path "*/tests/*" \
  ! -name "*_tests.rs" \
  ! -name "*test*.rs" \
  ! -name "*_test.rs" \
  -exec grep -l "\.unwrap()" {} \; 2>/dev/null | sort)

total_files=0
total_unwraps=0

for file in $production_files; do
  # Count unwraps, but exclude test functions
  unwrap_count=$(grep -n "\.unwrap()" "$file" | \
    awk '{
      # Skip if line contains #[test] marker or is in a test function
      if ($0 !~ /#\[test\]/ && $0 !~ /#\[tokio::test\]/ && $0 !~ /async fn test_/ && $0 !~ /fn test_/) {
        print
      }
    }' | wc -l)
  
  if [ $unwrap_count -gt 0 ]; then
    echo "📄 $file: $unwrap_count unwraps"
    total_files=$((total_files + 1))
    total_unwraps=$((total_unwraps + unwrap_count))
    
    # Show the actual lines
    grep -n "\.unwrap()" "$file" | \
      grep -v "#\[test\]" | \
      grep -v "#\[tokio::test\]" | \
      grep -v "async fn test_" | \
      grep -v "fn test_" | \
      head -5 | \
      sed 's/^/   /'
    
    if [ $(grep -c "\.unwrap()" "$file") -gt 5 ]; then
      echo "   ... (showing first 5 of $(grep -c '\.unwrap()' "$file") total)"
    fi
    echo ""
  fi
done

echo "=========================================="
echo "📊 Summary:"
echo "   Files with production unwraps: $total_files"
echo "   Approximate total unwraps: $total_unwraps"
echo ""
echo "💡 Next steps:"
echo "   1. Review each file and convert unwraps to proper error handling"
echo "   2. Use map_err(|e| BearDogError::...) for Result types"
echo "   3. Use ok_or_else(|| BearDogError::...) for Option types"
echo "   4. Add proper error context with descriptive messages"

