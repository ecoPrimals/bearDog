#!/bin/bash
# Unwrap Audit Script - Production Code Only
# Filters out test code to identify real production unwraps

echo "🔍 BearDog Unwrap Audit - Production Code Only"
echo "==============================================="
echo ""

# Function to check if a file is production code (not test)
is_production_code() {
    local file=$1
    # Exclude test files
    if [[ $file == *"/tests/"* ]] || [[ $file == *"_tests.rs" ]] || [[ $file == *"_test.rs" ]] || [[ $file == *"/test_"* ]]; then
        return 1
    fi
    return 0
}

# Count unwraps in production code
echo "📊 Scanning for unwraps in production code..."
echo ""

total_production_unwraps=0
total_test_unwraps=0

# Scan each crate
for crate_dir in crates/beardog-core crates/beardog-security crates/beardog-auth crates/beardog-tunnel; do
    if [ ! -d "$crate_dir" ]; then
        continue
    fi
    
    crate_name=$(basename "$crate_dir")
    prod_count=0
    test_count=0
    
    # Find all Rust files
    while IFS= read -r file; do
        unwrap_count=$(grep -c "\.unwrap()" "$file" 2>/dev/null || echo "0")
        
        if [ "$unwrap_count" -gt 0 ]; then
            if is_production_code "$file"; then
                prod_count=$((prod_count + unwrap_count))
                # Show files with high unwrap counts
                if [ "$unwrap_count" -gt 5 ]; then
                    echo "⚠️  $file: $unwrap_count unwraps"
                fi
            else
                test_count=$((test_count + unwrap_count))
            fi
        fi
    done < <(find "$crate_dir" -name "*.rs" -type f)
    
    total_production_unwraps=$((total_production_unwraps + prod_count))
    total_test_unwraps=$((total_test_unwraps + test_count))
    
    echo ""
    echo "📦 $crate_name:"
    echo "   Production: $prod_count unwraps"
    echo "   Tests: $test_count unwraps"
done

echo ""
echo "==============================================="
echo "📊 FINAL SUMMARY"
echo "==============================================="
echo "Production unwraps: $total_production_unwraps"
echo "Test unwraps: $total_test_unwraps"
echo "Total: $((total_production_unwraps + total_test_unwraps))"
echo ""
echo "Production percentage: $(( total_production_unwraps * 100 / (total_production_unwraps + total_test_unwraps) ))%"
echo "Test percentage: $(( total_test_unwraps * 100 / (total_production_unwraps + total_test_unwraps) ))%"
echo ""

if [ "$total_production_unwraps" -gt 100 ]; then
    echo "⚠️  HIGH: $total_production_unwraps production unwraps need review"
elif [ "$total_production_unwraps" -gt 50 ]; then
    echo "⚠️  MEDIUM: $total_production_unwraps production unwraps need review"
else
    echo "✅ LOW: $total_production_unwraps production unwraps (acceptable)"
fi

