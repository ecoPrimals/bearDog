#!/bin/bash
set -e

echo "🔄 BearDog Error Pattern Migration"
echo "=================================="
echo

# Find all Rust files and apply the error pattern replacements
find ./crates -name "*.rs" -type f | while read -r file; do
    echo "Processing: $file"
    
    # Replace old struct-style error patterns with new canonical constructors
    sed -i 's/BearDogError::InvalidInput\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::invalid_input(\1)/g' "$file"
    sed -i 's/BearDogError::Configuration\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::configuration(\1)/g' "$file"
    sed -i 's/BearDogError::Internal\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::internal(\1)/g' "$file"
    sed -i 's/BearDogError::NotFound\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::not_found(\1)/g' "$file"
    sed -i 's/BearDogError::Workflow\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::validation(\1)/g' "$file"
    sed -i 's/BearDogError::Network\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::network(\1)/g' "$file"
    sed -i 's/BearDogError::Storage\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::storage(\1)/g' "$file"
    sed -i 's/BearDogError::Authentication\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::authentication(\1)/g' "$file"
    sed -i 's/BearDogError::Timeout\s*{\s*message:\s*\([^}]*\)\s*}/BearDogError::timeout(\1)/g' "$file"
    
    # Handle multiline patterns - more complex replacement
    perl -i -pe 's/BearDogError::(InvalidInput|Configuration|Internal|NotFound|Workflow|Network|Storage|Authentication|Timeout)\s*\{\s*message:\s*([^}]+)\s*\}/BearDogError::lc($1)($2)/ge' "$file"
done

echo "✅ Error pattern migration completed"
echo
echo "🧪 Testing compilation..."
if cargo check --workspace --quiet; then
    echo "✅ Compilation successful after error pattern migration"
else
    echo "❌ Some compilation issues remain - manual review needed"
fi 