#!/bin/bash
# BearDog Migration Helper Script
# Purpose: Automate common migration tasks for v4.0.0
# Created: November 7, 2025

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓ ${NC}$1"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${NC}$1"
}

print_error() {
    echo -e "${RED}✗ ${NC}$1"
}

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}\n"
}

# Usage information
show_usage() {
    cat << EOF
BearDog Migration Helper

Usage: $0 <command> [options]

Commands:
    audit           - Audit deprecated trait usage
    find-traits     - Find all deprecated trait imports
    count           - Count deprecated items by file
    migrate-file    - Migrate specific file to new traits
    backup          - Create backup before migration
    verify          - Verify build and tests after migration
    help            - Show this help message

Examples:
    $0 audit
    $0 find-traits
    $0 count
    $0 migrate-file crates/beardog-core/src/service_discovery/mod.rs
    $0 backup
    $0 verify

EOF
}

# Audit deprecated trait usage
audit_deprecated() {
    print_header "Auditing Deprecated Trait Usage"
    
    print_info "Searching for deprecated canonical trait imports..."
    
    # Find all deprecated trait imports
    local count=$(grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" 2>/dev/null | wc -l)
    
    if [ "$count" -eq 0 ]; then
        print_success "No deprecated trait imports found! ✨"
        return 0
    fi
    
    print_warning "Found $count deprecated trait import(s)"
    echo ""
    
    print_info "Files using deprecated traits:"
    grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" -l 2>/dev/null | \
        sed "s|$PROJECT_ROOT/||" | \
        while read -r file; do
            local file_count=$(grep "use beardog_traits::canonical::" "$PROJECT_ROOT/$file" | wc -l)
            echo "  - $file ($file_count import(s))"
        done
    
    echo ""
    print_info "Most common deprecated imports:"
    grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" -h 2>/dev/null | \
        sort | uniq -c | sort -rn | head -5 | \
        sed 's/^/  /'
}

# Find all deprecated trait imports
find_deprecated_traits() {
    print_header "Finding Deprecated Trait Imports"
    
    grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" -n 2>/dev/null || \
        print_success "No deprecated imports found!"
}

# Count deprecated items by file
count_by_file() {
    print_header "Counting Deprecated Items by File"
    
    print_info "Top 10 files with deprecated imports:"
    echo ""
    
    grep -r "use beardog_traits::canonical::" "$PROJECT_ROOT/crates" --include="*.rs" 2>/dev/null | \
        cut -d: -f1 | \
        sort | uniq -c | sort -rn | head -10 | \
        while read -r count file; do
            local rel_file=$(echo "$file" | sed "s|$PROJECT_ROOT/||")
            printf "  %3d  %s\n" "$count" "$rel_file"
        done
}

# Migrate specific file
migrate_file() {
    local file="$1"
    
    if [ -z "$file" ]; then
        print_error "File path required"
        echo "Usage: $0 migrate-file <file-path>"
        exit 1
    fi
    
    if [ ! -f "$file" ]; then
        print_error "File not found: $file"
        exit 1
    fi
    
    print_header "Migrating File: $file"
    
    # Check if file has deprecated imports
    if ! grep -q "use beardog_traits::canonical::" "$file" 2>/dev/null; then
        print_success "File already migrated or has no deprecated imports"
        return 0
    fi
    
    # Create backup
    local backup_file="${file}.backup-$(date +%Y%m%d-%H%M%S)"
    cp "$file" "$backup_file"
    print_info "Backup created: $backup_file"
    
    # Perform migration (safe replacements)
    print_info "Migrating imports..."
    
    # Replace canonical imports with unified imports
    sed -i 's|use beardog_traits::canonical::|use beardog_types::canonical::providers_unified::traits::|g' "$file"
    
    print_success "Migration complete!"
    print_info "Please review changes and test!"
    
    # Show diff
    if command -v diff &> /dev/null; then
        echo ""
        print_info "Changes made:"
        diff -u "$backup_file" "$file" || true
    fi
}

# Create backup
create_backup() {
    print_header "Creating Project Backup"
    
    local backup_dir="$PROJECT_ROOT/backups"
    local backup_name="beardog-backup-$(date +%Y%m%d-%H%M%S)"
    local backup_path="$backup_dir/$backup_name"
    
    mkdir -p "$backup_dir"
    
    print_info "Creating backup: $backup_name"
    
    # Create tarball of crates directory
    cd "$PROJECT_ROOT"
    tar -czf "$backup_path.tar.gz" \
        --exclude='target' \
        --exclude='node_modules' \
        --exclude='.git' \
        crates/ Cargo.toml Cargo.lock
    
    print_success "Backup created: $backup_path.tar.gz"
    
    # Show backup size
    local size=$(du -h "$backup_path.tar.gz" | cut -f1)
    print_info "Backup size: $size"
}

# Verify build and tests
verify_build() {
    print_header "Verifying Build and Tests"
    
    print_info "Building workspace..."
    if cargo build --workspace --quiet 2>&1 | grep -q "error:"; then
        print_error "Build failed!"
        exit 1
    fi
    print_success "Build successful"
    
    print_info "Running tests..."
    if cargo test --workspace --quiet 2>&1 | grep -q "FAILED"; then
        print_error "Tests failed!"
        exit 1
    fi
    print_success "All tests passed"
    
    print_info "Running clippy..."
    cargo clippy --workspace --quiet -- -D warnings 2>&1 || {
        print_warning "Clippy found issues (see output above)"
    }
    
    print_success "Verification complete! ✨"
}

# Main script logic
main() {
    if [ $# -eq 0 ]; then
        show_usage
        exit 0
    fi
    
    case "$1" in
        audit)
            audit_deprecated
            ;;
        find-traits)
            find_deprecated_traits
            ;;
        count)
            count_by_file
            ;;
        migrate-file)
            shift
            migrate_file "$@"
            ;;
        backup)
            create_backup
            ;;
        verify)
            verify_build
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            print_error "Unknown command: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

# Run main function
main "$@"

