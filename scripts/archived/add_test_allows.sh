#!/bin/bash
# Add allow attributes to all test files
# This handles pedantic clippy warnings in test code

set -e

echo "🔧 Adding allow attributes to test files..."

# Common pedantic warnings to allow in tests
ALLOW_LINE='#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code, clippy::clone_on_copy, clippy::single_char_pattern, clippy::no_effect_underscore_binding)]'

# Find all test files in beardog-core
find crates/beardog-core/src -name "*test*.rs" -type f | while read file; do
    # Check if file already has allow attribute
    if ! grep -q "#!\[allow" "$file"; then
        # Add after the first docstring or at the top
        if head -1 "$file" | grep -q "^//!"; then
            # Has docstring - find where it ends and add after
            awk -v allow="$ALLOW_LINE" '
                BEGIN { added = 0 }
                /^\/\/!/ { print; next }
                !added && !/^\/\/!/ && !/^$/ { print ""; print allow; print ""; added = 1 }
                { print }
            ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
        else
            # No docstring - add at top
            echo -e "$ALLOW_LINE\n" | cat - "$file" > "$file.tmp" && mv "$file.tmp" "$file"
        fi
        echo "  ✅ Added allows to: $file"
    fi
done

# Handle test modules in non-test files
find crates/beardog-core/src -name "*.rs" -type f ! -name "*test*.rs" | while read file; do
    # Check if file has #[cfg(test)] modules
    if grep -q "#\[cfg(test)\]" "$file"; then
        # Add allow before #[cfg(test)] if not already there
        if ! grep -B 1 "#\[cfg(test)\]" "$file" | grep -q "#\[allow"; then
            sed -i '/#\[cfg(test)\]/i #[allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, dead_code)]' "$file"
            echo "  ✅ Added allows to test module in: $file"
        fi
    fi
done

# Also add to test modules in ai/tests/mod.rs to fix missing docs
if [ -f "crates/beardog-core/src/ai/tests/mod.rs" ]; then
    if ! grep -q "#!\[allow(missing_docs)\]" "crates/beardog-core/src/ai/tests/mod.rs"; then
        sed -i '1i #![allow(missing_docs)]' "crates/beardog-core/src/ai/tests/mod.rs"
        echo "  ✅ Added missing_docs allow to ai/tests/mod.rs"
    fi
fi

echo ""
echo "✅ All test files updated!"

