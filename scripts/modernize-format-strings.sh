#!/bin/bash
# Format String Modernization Script
# Automatically updates old format!() style to modern inline args

echo "🔧 Format String Modernization"
echo "=============================="
echo ""

# Function to modernize format strings in a file
modernize_file() {
    local file=$1
    local changes=0
    
    # Skip if file doesn't exist or is not a Rust file
    if [[ ! -f "$file" || ! "$file" =~ \.rs$ ]]; then
        return 0
    fi
    
    # Create backup
    cp "$file" "$file.bak"
    
    # Common patterns to modernize
    # Pattern: format!("text {}", var) -> format!("text {var}")
    # Pattern: format!("text {} more {}", var1, var2) -> format!("text {var1} more {var2}")
    
    # Use perl for more sophisticated regex replacement
    perl -i -pe '
        # Simple single variable case
        s/format!\("([^"]*)\{\}([^"]*)",\s*([a-zA-Z_][a-zA-Z0-9_]*)\)/format!("$1\{$3\}$2")/g;
        
        # Error message pattern: format!("Error: {}", e)
        s/format!\("([^"]*Error[^"]*)\{\}([^"]*)",\s*e\)/format!("$1\{e\}$2")/g;
        
        # Failed pattern: format!("Failed: {}", err)
        s/format!\("([^"]*Failed[^"]*)\{\}([^"]*)",\s*err\)/format!("$1\{err\}$2")/g;
    ' "$file"
    
    # Check if file changed
    if ! diff -q "$file" "$file.bak" > /dev/null 2>&1; then
        changes=$((changes + 1))
        echo "✓ Updated: $file"
    fi
    
    # Remove backup if no changes
    if [ $changes -eq 0 ]; then
        rm "$file.bak"
    fi
    
    return $changes
}

# Process all Rust files in production code
total_files=0
total_changes=0

echo "📁 Processing files..."
echo ""

for crate_dir in crates/beardog-{core,security,auth,tunnel,types,config,utils}; do
    if [ ! -d "$crate_dir" ]; then
        continue
    fi
    
    crate_changes=0
    
    while IFS= read -r file; do
        # Skip test files
        if [[ "$file" == *"_test"* || "$file" == *"/tests/"* ]]; then
            continue
        fi
        
        if modernize_file "$file"; then
            crate_changes=$((crate_changes + $?))
            total_files=$((total_files + 1))
        fi
    done < <(find "$crate_dir" -name "*.rs" -type f)
    
    if [ $crate_changes -gt 0 ]; then
        total_changes=$((total_changes + crate_changes))
        echo "  Crate $(basename $crate_dir): $crate_changes files updated"
    fi
done

echo ""
echo "=============================="
echo "📊 Summary"
echo "=============================="
echo "Files processed: $total_files"
echo "Files updated: $total_changes"
echo ""

if [ $total_changes -gt 0 ]; then
    echo "✅ Modernization complete!"
    echo "   Run 'cargo fmt' to format the changes"
    echo "   Run 'cargo clippy' to verify improvements"
else
    echo "ℹ️  No changes needed"
fi

