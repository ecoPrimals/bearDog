#!/bin/bash
# Batch BearDogResult migration with progress tracking
# Migrates all production files systematically

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

MIGRATED=0
FAILED=0
SKIPPED=0

echo "🚀 Starting batch BearDogResult migration..."
echo "================================================"

# Function to migrate a single file
migrate_file() {
    local file="$1"
    
    # Skip test files
    if [[ "$file" == *"test"* ]] || [[ "$file" == *"tests/"* ]]; then
        ((SKIPPED++))
        return 0
    fi
    
    # Skip backup files
    if [[ "$file" == *".backup"* ]] || [[ "$file" == *".bak"* ]]; then
        return 0
    fi
    
    # Check if file has BearDogResult
    if ! grep -q "BearDogResult" "$file"; then
        return 0
    fi
    
    echo -ne "${YELLOW}⚙️  Migrating:${NC} $(basename $file)..."
    
    # Backup
    cp "$file" "$file.backup"
    
    # Fix import - handle multi-import pattern
    if grep -q "use beardog_errors::{BearDogError, BearDogResult}" "$file"; then
        sed -i 's/use beardog_errors::{BearDogError, BearDogResult};/use beardog_errors::BearDogError;/' "$file"
    elif grep -q "use beardog_errors::{.*BearDogResult.*}" "$file"; then
        # Remove BearDogResult from multi-import
        sed -i 's/, *BearDogResult//g; s/BearDogResult, *//g' "$file"
    fi
    
    # Replace BearDogResult patterns
    sed -i 's/-> BearDogResult</-> Result</g' "$file"
    sed -i 's/: BearDogResult</: Result</g' "$file"  
    sed -i 's/<BearDogResult</<Result</g' "$file"
    sed -i 's/BearDogResult>/Result<(), BearDogError>>/g' "$file"
    sed -i 's/Result<()>/Result<(), BearDogError>/g' "$file"
    
    # Count changes
    local changes=$(diff -u "$file.backup" "$file" 2>/dev/null | grep "^[-+]" | grep -c "Result\|BearDogResult" || echo "0")
    
    if [ "$changes" -gt 0 ]; then
        echo -e " ${GREEN}✅ $((changes / 2)) replacements${NC}"
        rm "$file.backup"
        ((MIGRATED++))
        return 0
    else
        echo -e " ${YELLOW}⏭️  No changes${NC}"
        mv "$file.backup" "$file"
        return 0
    fi
}

# Find all Rust files with BearDogResult (excluding tests)
FILES=$(grep -r "BearDogResult" crates/ --include="*.rs" --exclude-dir=target | grep -v "test" | cut -d: -f1 | sort | uniq)

TOTAL=$(echo "$FILES" | wc -l)
CURRENT=0

echo "📊 Found $TOTAL files to migrate"
echo "================================================"

for file in $FILES; do
    ((CURRENT++))
    echo -ne "[${CURRENT}/${TOTAL}] "
    migrate_file "$file" || {
        echo -e "${RED}❌ Failed: $file${NC}"
        ((FAILED++))
    }
done

echo "================================================"
echo "📊 Migration Summary:"
echo -e "  ${GREEN}✅ Migrated: $MIGRATED files${NC}"
echo -e "  ${YELLOW}⏭️  Skipped:  $SKIPPED files${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  ${RED}❌ Failed:   $FAILED files${NC}"
fi

echo ""
echo "🧪 Testing compilation..."
if cargo check --workspace --message-format=short 2>&1 | grep -q "error\[E"; then
    echo -e "${RED}❌ Compilation has errors! Please review.${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Compilation successful!${NC}"
    
    # Count remaining
    REMAINING=$(grep -r "BearDogResult" crates/ --include="*.rs" --exclude-dir=target | grep -v "test" | wc -l)
    echo ""
    echo "📊 Remaining BearDogResult usages: $REMAINING"
    exit 0
fi

