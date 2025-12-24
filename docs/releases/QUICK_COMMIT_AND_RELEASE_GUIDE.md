# 🚀 Quick Commit & Release Guide for ecoPrimals Teams

**Share this with teams for clean commits and binary distribution**

---

## 📝 Step 1: Commit and Push All Changes

```bash
# Navigate to your project
cd /path/to/your/project

# Check status
git status

# Add all changes
git add -A

# Commit with descriptive message
git commit -m "🎉 Your descriptive message here"

# Push to your current branch
git push origin $(git branch --show-current)
```

### If you have uncommitted changes:
```bash
# Stage everything
git add -A

# Commit
git commit -m "✨ Feature complete - [describe what you did]"

# Push
git push origin $(git branch --show-current)
```

---

## 🧹 Step 2: Trim Loose/Stale Branches

### List all branches:
```bash
# See local branches
git branch

# See remote branches
git branch -r

# See all branches with last commit info
git branch -vv
```

### Delete local branches:
```bash
# Delete a merged branch
git branch -d branch-name

# Force delete an unmerged branch (careful!)
git branch -D branch-name
```

### Delete remote branches:
```bash
# Delete a remote branch
git push origin --delete branch-name

# Or the shorter version
git push origin :branch-name
```

### Clean up tracking branches:
```bash
# Remove references to deleted remote branches
git fetch --prune

# Or use the shorthand
git fetch -p
```

### Quick cleanup script:
```bash
# Show merged branches (safe to delete)
git branch --merged | grep -v "\*" | grep -v "main" | grep -v "master"

# Delete all merged branches (except main/master)
git branch --merged | grep -v "\*" | grep -v "main" | grep -v "master" | xargs -n 1 git branch -d
```

---

## 📦 Step 3: Clean Binary Release Process

### The BearDog Method (Recommended)

**Instead of pushing binaries to the repo, use GitHub Releases:**

```bash
# 1. Build your release binary
cargo build --release
# Or for your project: npm run build, make release, etc.

# 2. Create a checksum
sha256sum target/release/your-binary > your-binary.sha256

# 3. Test the binary
./target/release/your-binary --version

# 4. Create a Git tag for this checkpoint
git tag -a v0.1.0-integration -m "Integration checkpoint - $(date +%Y-%m-%d)"

# 5. Push the tag
git push origin v0.1.0-integration

# 6. Create GitHub Release with binary (Option A: CLI)
gh release create v0.1.0-integration \
  target/release/your-binary \
  your-binary.sha256 \
  --title "Integration Checkpoint - $(date +%Y-%m-%d)" \
  --notes "Ready for integration testing" \
  --prerelease

# 6. Alternative: Create GitHub Release (Option B: Web UI)
# Go to: https://github.com/your-org/your-repo/releases/new
# Select your tag, upload binary, publish
```

### Install GitHub CLI (if needed):
```bash
# Ubuntu/Debian
sudo apt install gh

# Authenticate
gh auth login --web
```

### Share with other teams:
```
📦 Binary available:
https://github.com/your-org/your-repo/releases/tag/v0.1.0-integration

Download:
wget https://github.com/your-org/your-repo/releases/download/v0.1.0-integration/your-binary
chmod +x your-binary
./your-binary --version
```

---

## ✅ Best Practices Checklist

### Before Committing:
- [ ] All tests passing
- [ ] Code builds without errors
- [ ] No sensitive data (passwords, keys) in code
- [ ] Meaningful commit message

### Before Pushing:
- [ ] Pull latest changes: `git pull origin $(git branch --show-current)`
- [ ] Resolve any conflicts
- [ ] Tests still pass after merge

### For Releases:
- [ ] Build in release mode (optimized)
- [ ] Test the binary
- [ ] Create checksum
- [ ] Use Git tags (not branches)
- [ ] Upload to GitHub Releases (not repo)
- [ ] Mark as pre-release if for testing

---

## 🔧 Common Issues & Solutions

### "Push rejected - non-fast-forward"
```bash
# Pull first, then push
git pull origin $(git branch --show-current)
git push origin $(git branch --show-current)
```

### "Large files warning"
```bash
# Don't push binaries to repo
# Use GitHub Releases instead (see Step 3)
```

### "Need sudo for gh install"
```bash
# Use GUI sudo
pkexec apt install gh

# Or download from: https://github.com/cli/cli/releases
```

### "Can't delete branch - not fully merged"
```bash
# Check what's not merged
git log main..branch-name

# If you're sure, force delete
git branch -D branch-name
```

---

## 📊 Quick Reference

### Commit & Push:
```bash
git add -A
git commit -m "Your message"
git push origin $(git branch --show-current)
```

### Create Release:
```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
gh release create v1.0.0 path/to/binary --prerelease
```

### Clean Branches:
```bash
git fetch -p
git branch -d old-branch
git push origin --delete old-branch
```

---

## 🎯 TL;DR for Teams

**To commit everything:**
```bash
git add -A && git commit -m "Your message" && git push origin $(git branch --show-current)
```

**To release a binary:**
```bash
# Don't push binary to repo!
# Create tag, then GitHub Release:
git tag -a v1.0.0 -m "Release"
git push origin v1.0.0
gh release create v1.0.0 your-binary --prerelease
```

**To clean branches:**
```bash
git fetch -p
git branch --merged | grep -v "main" | xargs git branch -d
```

---

## 📧 Contact

Questions? Open an issue or ask in team chat!

**Example from BearDog**: https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23

---

**Keep repos clean, binaries separate, and integrations smooth!** 🚀

🐻 ecoPrimals Best Practices

