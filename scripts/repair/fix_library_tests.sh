#!/bin/bash
# Fix async keywords in library test modules

echo "Fixing async keywords in library tests..."

# Find all Rust files in crates/*/src/ with tokio::test annotations
find crates -name "*.rs" -path "*/src/*" -type f | while read -r file; do
    if grep -q "#\[tokio::test\]" "$file"; then
        echo "Processing: $file"
        
        # Use perl to add async keyword after tokio::test functions
        # This handles the pattern: #[tokio::test] followed by fn (missing async)
        perl -i -pe 's/(#\[tokio::test\]\s*\n\s*)fn\s+/$1async fn /g unless /async fn/' "$file"
        
        echo "  ✓ Fixed async keywords"
    fi
done

echo "Done! Async keywords fixed in library tests."

