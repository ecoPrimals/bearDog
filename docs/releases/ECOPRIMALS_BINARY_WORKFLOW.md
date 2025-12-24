# 🦅 ecoPrimals Binary Distribution Workflow

**Updated**: December 24, 2025  
**For**: All ecoPrimals Teams

---

## 📋 Our Binary Distribution Process

### **Two-Stage Distribution**:

1. **Local Staging**: `../phase2/phase1bins/` (for team testing)
2. **GitHub Releases**: Official checkpoints (for staging team deployments)

---

## 🔄 The Workflow

### **Step 1: Build Release Binary**

```bash
# Navigate to your project
cd /home/eastgate/Development/ecoPrimals/beardog

# Build release
cargo build --release

# Test it works
./target/release/beardog --version
```

### **Step 2: Create Checksum**

```bash
# Generate checksum
sha256sum target/release/beardog > beardog.sha256

# Show checksum
cat beardog.sha256
```

### **Step 3: Copy to Shared Bins Location**

```bash
# Copy binary with version tag
cp target/release/beardog ../phase2/phase1bins/beardog-v0.9.0-dec23

# Copy checksum
cp beardog.sha256 ../phase2/phase1bins/beardog-v0.9.0-dec23.sha256

# Verify
ls -lh ../phase2/phase1bins/
```

**Purpose**: Other ecoPrimals teams can test locally before GitHub release.

### **Step 4: Create GitHub Release**

```bash
# Create git tag
git tag -a v0.9.0-integration-dec23 -m "Integration checkpoint"
git push origin v0.9.0-integration-dec23

# Create GitHub release with binary
gh release create v0.9.0-integration-dec23 \
  target/release/beardog \
  beardog.sha256 \
  --title "Integration Checkpoint - Dec 23, 2025" \
  --notes "Ready for staging team deployment" \
  --prerelease

# Get release URL
echo "Release URL:"
gh release view v0.9.0-integration-dec23 --web
```

**Purpose**: Official checkpoint that staging team can pull for deployments.

### **Step 5: Notify Staging Team**

```bash
# Send message:
"New BearDog release available:
https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23

Staging team can update when ready."
```

---

## 📁 Directory Structure

```
/home/eastgate/Development/ecoPrimals/
├── beardog/                    (BearDog source)
│   ├── target/release/beardog  (built binary)
│   └── beardog.sha256          (checksum)
│
├── phase2/
│   └── phase1bins/             (shared binary location)
│       ├── beardog-v0.9.0-dec23       (versioned binary)
│       ├── beardog-v0.9.0-dec23.sha256 (checksum)
│       ├── songbird-v1.2.0-dec20      (other team binaries)
│       ├── nestgate-v0.8.0-dec22      (other team binaries)
│       └── ...
│
├── songbird/                   (Songbird source)
├── nestgate/                   (Nestgate source)
├── toadstool/                  (Toadstool source)
└── squirrel/                   (Squirrel source)
```

---

## 🎯 When to Update

### **Local Bins** (`../phase2/phase1bins/`):

**Update when**:
- Daily builds for team testing
- Integration testing with other components
- Quick iterations between teams
- Pre-release testing

**Who uses**:
- Other ecoPrimals teams
- Local integration testing
- Development environments

### **GitHub Releases**:

**Update when**:
- Official integration checkpoint
- Ready for staging deployment
- Feature complete
- Major milestones

**Who uses**:
- Staging team (for deployments)
- External partners
- Production releases
- Official checkpoints

---

## 📊 Version Naming Convention

### **Local Bins** (flexible):
```
{project}-v{version}-{date}

Examples:
- beardog-v0.9.0-dec23
- songbird-v1.2.0-dec20
- nestgate-v0.8.0-dec22
```

### **GitHub Tags** (semantic):
```
v{major}.{minor}.{patch}-{type}-{date}

Examples:
- v0.9.0-integration-dec23
- v1.0.0-beta
- v1.0.0-rc1
- v1.0.0 (production)

Types:
- integration  (for integration testing)
- beta        (feature complete, testing)
- rc          (release candidate)
- (none)      (production release)
```

---

## 🔄 Staging Team Update Process

### **When Staging Team Gets Notified**:

```bash
# 1. Download from GitHub release
wget https://github.com/ecoPrimals/bearDog/releases/download/v0.9.0-integration-dec23/beardog

# 2. Verify checksum
wget https://github.com/ecoPrimals/bearDog/releases/download/v0.9.0-integration-dec23/beardog.sha256
sha256sum -c beardog.sha256

# 3. Deploy to staging environment
chmod +x beardog
./deploy-to-staging.sh beardog

# 4. Run smoke tests
./staging-smoke-tests.sh

# 5. Notify team if issues
```

**Staging team pulls when ready** - we don't push to them!

---

## ✅ Quick Reference

### **For Daily Development**:
```bash
# Build
cargo build --release

# Copy to shared bins
cp target/release/beardog ../phase2/phase1bins/beardog-v0.9.0-$(date +%b%d)

# Notify teams in Slack
"New BearDog binary in phase1bins: beardog-v0.9.0-dec24"
```

### **For Official Releases**:
```bash
# Build
cargo build --release

# Create checksum
sha256sum target/release/beardog > beardog.sha256

# Copy to shared bins
cp target/release/beardog ../phase2/phase1bins/beardog-v0.9.0-dec23
cp beardog.sha256 ../phase2/phase1bins/beardog-v0.9.0-dec23.sha256

# Create GitHub release
git tag -a v0.9.0-integration-dec23 -m "Integration checkpoint"
git push origin v0.9.0-integration-dec23
gh release create v0.9.0-integration-dec23 \
  target/release/beardog \
  beardog.sha256 \
  --prerelease

# Notify staging team
"New release: https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23"
```

---

## 📧 Notification Templates

### **For Team Integration Testing**:
```
Subject: New BearDog Binary Available (Local Testing)

Hi Team,

New BearDog binary ready for integration testing:

Location: ../phase2/phase1bins/beardog-v0.9.0-dec23

Quick test:
  ../phase2/phase1bins/beardog-v0.9.0-dec23 --version

Changes:
- [List major changes]

Let me know if you hit any issues!
```

### **For Staging Team**:
```
Subject: BearDog v0.9.0 Ready for Staging Deployment

Hi Staging Team,

New BearDog release is ready for deployment:

🔗 Release: https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23

Download:
  wget https://github.com/ecoPrimals/bearDog/releases/download/v0.9.0-integration-dec23/beardog

Checksum:
  SHA256: 4593ed2da6e386c5c11476d53af1390a33bb7a8c47b2b93e041df90434b3bd20

Please update staging when convenient.

Tests passing: 493/497 (99.2%)
Grade: A (92/100)

Deploy when ready!
```

---

## 🎯 Best Practices

### **DO**:
- ✅ Always create checksums
- ✅ Version binaries clearly
- ✅ Copy to shared bins for team testing
- ✅ Create GitHub releases for official checkpoints
- ✅ Test before copying to shared location
- ✅ Notify teams after updates
- ✅ Let staging team pull when ready

### **DON'T**:
- ❌ Push binaries directly to git repo
- ❌ Overwrite existing binaries without versioning
- ❌ Skip checksum generation
- ❌ Deploy to staging without team notification
- ❌ Force staging team to update immediately
- ❌ Delete old binaries (keep for rollback)

---

## 🔍 Current Status

### **Latest BearDog Release**:
```
Version:     v0.9.0-integration-dec23
Date:        December 23, 2025
Location:    ../phase2/phase1bins/beardog-v0.9.0-dec23
GitHub:      https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23
Size:        4.5 MB
Checksum:    4593ed2da6e386c5c11476d53af1390a33bb7a8c47b2b93e041df90434b3bd20
Status:      ✅ Ready for integration testing
Grade:       A (92/100)
Tests:       493/497 passing (99.2%)
```

---

## 📚 Related Documentation

- `BINARY_DISTRIBUTION_GUIDE.md` - Detailed binary distribution
- `QUICK_COMMIT_AND_RELEASE_GUIDE.md` - Full workflow guide
- `TEAM_QUICK_MESSAGE.txt` - Copy/paste templates
- `RELEASE_COMPLETE_DEC_23_2025.md` - Latest release info

---

## 🤝 Team Coordination

### **Questions?**
- Slack: #ecoprimals-integration
- For BearDog: #beardog-dev
- For releases: #releases

### **Staging Team Contact**:
- When to notify: After GitHub release
- How they update: Pull from GitHub releases
- Timeline: At their discretion
- Smoke tests: Their responsibility

---

## 🎊 Summary

**Simple workflow**:
1. Build → Test locally
2. Copy to `../phase2/phase1bins/` → Teams test
3. GitHub Release → Staging team deploys (when ready)

**Remember**: 
- Shared bins = quick team iteration
- GitHub releases = official checkpoints
- Staging team pulls (we don't push!)

---

🐻 **ecoPrimals: Clean, coordinated, sovereign!** 🦅

**Last Updated**: December 24, 2025

