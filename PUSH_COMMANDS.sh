#!/bin/bash
# BearDog v0.9.0-beta Release Push Script
# Run this script to push the release to GitHub

set -e  # Exit on error

echo "=== BearDog v0.9.0-beta Release Push ==="
echo ""

# Navigate to repository
cd "$(dirname "$0")"

# Verify we're in the right place
if [ ! -f "Cargo.toml" ] || [ ! -d ".git" ]; then
    echo "❌ Error: Not in BearDog repository root"
    exit 1
fi

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"

# Check if tag exists
if git rev-parse v0.9.0-beta >/dev/null 2>&1; then
    echo "✅ Tag v0.9.0-beta exists locally"
else
    echo "❌ Error: Tag v0.9.0-beta not found"
    exit 1
fi

# Show what will be pushed
echo ""
echo "=== Commits to be pushed ==="
git log --oneline origin/$CURRENT_BRANCH..HEAD 2>/dev/null || git log --oneline -5
echo ""

# Confirm
read -p "Push v0.9.0-beta and $CURRENT_BRANCH to origin? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Push cancelled"
    exit 1
fi

# Push tag (force to update)
echo ""
echo "🚀 Pushing tag v0.9.0-beta..."
git push --force origin v0.9.0-beta

# Push branch
echo ""
echo "🚀 Pushing branch $CURRENT_BRANCH..."
git push origin $CURRENT_BRANCH

# Success
echo ""
echo "=== ✅ RELEASE COMPLETE ==="
echo ""
echo "✅ Tag v0.9.0-beta pushed"
echo "✅ Branch $CURRENT_BRANCH pushed"
echo ""
echo "Next steps:"
echo "1. Go to GitHub → Releases"
echo "2. Create release from tag v0.9.0-beta"
echo "3. Use RELEASE_READY_v0.9.0-beta.md for description"
echo "4. Mark as 'pre-release'"
echo "5. Publish!"
echo ""
echo "🐻🔒 BearDog v0.9.0-beta is live!"

