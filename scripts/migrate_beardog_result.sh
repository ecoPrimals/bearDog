#!/bin/bash
# Safe BearDogResult migration script
# Migrates one file at a time and tests compilation

set -e  # Exit on error

FILE="$1"

if [ -z "$FILE" ]; then
    echo "Usage: $0 <file_to_migrate>"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File not found: $FILE"
    exit 1
fi

echo "🔄 Migrating: $FILE"

# Backup the file
cp "$FILE" "$FILE.backup"

# Check if file already has BearDogError import
if ! grep -q "use beardog_errors::BearDogError" "$FILE"; then
    # Find the first 'use' statement and add import after it
    if grep -q "^use " "$FILE"; then
        sed -i '0,/^use /s//use beardog_errors::BearDogError;\nuse /' "$FILE"
    else
        # No use statements, add at beginning after doc comments
        sed -i '/^[^\/]/i use beardog_errors::BearDogError;' "$FILE"
    fi
    echo "  ✅ Added BearDogError import"
fi

# Perform the replacement
sed -i 's/BearDogResult</Result</g' "$FILE"
sed -i 's/BearDogResult>/Result<(), BearDogError>>/g' "$FILE"

# Count changes
CHANGES=$(diff -u "$FILE.backup" "$FILE" | grep "^-.*BearDogResult\|^+.*Result<" | wc -l)
echo "  📝 Made $((CHANGES / 2)) replacements"

# Test compilation
echo "  🧪 Testing compilation..."
if cargo check --all-features 2>&1 | grep -q "error\[E"; then
    echo "  ❌ Compilation failed! Reverting..."
    mv "$FILE.backup" "$FILE"
    exit 1
else
    echo "  ✅ Compilation successful!"
    rm "$FILE.backup"
    exit 0
fi

