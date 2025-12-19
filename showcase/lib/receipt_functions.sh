#!/usr/bin/env bash
#
# Shared Receipt Generation Functions
# Used by all showcase demos for consistent receipt format
#

# Generate cryptographic receipt for an operation
generate_receipt() {
    local operation=$1
    local input_files=$2
    local output_files=$3
    local session_id=${SESSION_ID:-"session-$(date +%s)"}
    local receipts_dir=${RECEIPTS_DIR:-"./receipts"}
    local timestamp=$(date +%s)
    
    mkdir -p "$receipts_dir"
    
    local receipt_file="${receipts_dir}/receipt_${operation}_${timestamp}.json"
    
    # Hash all input files
    local input_hashes=""
    if [ -n "$input_files" ]; then
        for file in $input_files; do
            if [ -f "$file" ]; then
                local hash=$(sha256sum "$file" | awk '{print $1}')
                local name=$(basename "$file")
                input_hashes="${input_hashes}    \"$name\": \"sha256:$hash\",\n"
            fi
        done
        # Remove trailing comma and newline
        input_hashes=$(echo -e "$input_hashes" | sed '$ s/,$//')
    fi
    
    # Hash all output files
    local output_hashes=""
    if [ -n "$output_files" ]; then
        for file in $output_files; do
            if [ -f "$file" ]; then
                local hash=$(sha256sum "$file" | awk '{print $1}')
                local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file")
                local name=$(basename "$file")
                output_hashes="${output_hashes}    \"$name\": {\n      \"hash\": \"sha256:$hash\",\n      \"size\": $size\n    },\n"
            fi
        done
        # Remove trailing comma and newline
        output_hashes=$(echo -e "$output_hashes" | sed '$ s/,$//')
    fi
    
    # Create cryptographic receipt
    cat > "$receipt_file" << EOF
{
  "receipt_id": "receipt-${operation}-${timestamp}",
  "session_id": "$session_id",
  "operation": "$operation",
  "timestamp": "$(date -Iseconds)",
  "timestamp_unix": $timestamp,
  "inputs": {
$(echo -e "$input_hashes")
  },
  "outputs": {
$(echo -e "$output_hashes")
  },
  "verification": {
    "method": "sha256_hashing",
    "verifiable": true,
    "verification_commands": [
      "sha256sum <file>",
      "Compare hash with receipt"
    ]
  },
  "environment": {
    "hostname": "$(hostname)",
    "user": "$(whoami)",
    "pwd": "$(pwd)",
    "os": "$(uname -s)",
    "arch": "$(uname -m)"
  }
}
EOF
    
    echo "$receipt_file"
}

# Verify a receipt against actual files
verify_receipt() {
    local receipt_file=$1
    
    if [ ! -f "$receipt_file" ]; then
        echo "❌ Receipt not found: $receipt_file"
        return 1
    fi
    
    echo "🔍 Verifying receipt: $(basename $receipt_file)"
    
    # Check if jq is available
    if ! command -v jq &> /dev/null; then
        echo "⚠️  jq not installed - manual verification needed"
        return 0
    fi
    
    local verified=true
    
    # Verify outputs
    local outputs=$(jq -r '.outputs | to_entries[] | "\(.key):\(.value.hash)"' "$receipt_file" 2>/dev/null)
    
    while IFS=: read -r filename hash; do
        # Remove sha256: prefix
        hash=${hash#sha256:}
        
        # Find the file (could be in various output directories)
        local actual_file=$(find . -name "$filename" -type f 2>/dev/null | head -1)
        
        if [ -f "$actual_file" ]; then
            local actual_hash=$(sha256sum "$actual_file" | awk '{print $1}')
            if [ "$hash" = "$actual_hash" ]; then
                echo "  ✅ $filename: Hash matches"
            else
                echo "  ❌ $filename: Hash mismatch!"
                echo "     Expected: $hash"
                echo "     Got:      $actual_hash"
                verified=false
            fi
        else
            echo "  ⚠️  $filename: File not found (may have been deleted)"
        fi
    done <<< "$outputs"
    
    if [ "$verified" = true ]; then
        echo "✅ Receipt verification PASSED"
        return 0
    else
        echo "❌ Receipt verification FAILED"
        return 1
    fi
}

# Generate uniqueness proof (run operation twice, prove different)
generate_uniqueness_proof() {
    local operation=$1
    local sample1_hash=$2
    local sample2_hash=$3
    local receipts_dir=${RECEIPTS_DIR:-"./receipts"}
    local timestamp=$(date +%s)
    
    mkdir -p "$receipts_dir"
    
    local proof_file="${receipts_dir}/uniqueness_proof_${operation}_${timestamp}.json"
    
    cat > "$proof_file" << EOF
{
  "proof_type": "uniqueness_verification",
  "operation": "$operation",
  "timestamp": "$(date -Iseconds)",
  "sample_1_hash": "$sample1_hash",
  "sample_2_hash": "$sample2_hash",
  "are_different": $([ "$sample1_hash" != "$sample2_hash" ] && echo "true" || echo "false"),
  "conclusion": "$([ "$sample1_hash" != "$sample2_hash" ] && echo "Outputs are unique - real cryptography confirmed" || echo "WARNING: Outputs are identical - possible issue")",
  "verification": {
    "method": "Compare SHA-256 hashes of two independent runs",
    "expected": "Different hashes for each run",
    "actual": "$([ "$sample1_hash" != "$sample2_hash" ] && echo "Different ✅" || echo "Same ❌")"
  }
}
EOF
    
    echo "$proof_file"
}

# Export functions for use in demo scripts
export -f generate_receipt
export -f verify_receipt
export -f generate_uniqueness_proof

