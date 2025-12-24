# 🎬 BearDog Showcase Examples

**Status**: Minimal showcase (Dec 24, 2025)  
**Location**: `showcase/`

---

## 📊 Current Status

### Available Examples

#### 05-mixed-entropy ✅
**Location**: `showcase/05-mixed-entropy/`  
**Status**: ✅ Working  
**Purpose**: Demonstrates mixed entropy (human + machine) for key generation

```bash
cd showcase/05-mixed-entropy
cargo run
```

---

## 📚 Archived Showcase Content

**Note**: Extensive showcase content was archived on December 23, 2025.

**Archive Location**: `../archive/beardog-dec-23-2025/showcase/`

### Archived Examples Include:

1. **00-local-primal** - Local primal operations
2. **01-local-basics** - Basic local crypto operations
3. **01-local-capabilities** - Capability demonstrations
4. **02-ecosystem-integration** - Ecosystem integration demos
   - Songbird BTSP integration
   - Genesis bootstrap
   - BirdSong encryption
   - Lineage tracking
5. **02-hardware-integration** - HSM integration examples
6. **02-hsm-comparison** - HSM comparison demos
7. **03-constraint-demos** - Self-enforcing key constraints
8. **03-genesis-bootstrap** - Genesis bootstrap examples
9. **03-human-entropy** - Human entropy collection

**Total Archived**: ~30 showcase directories with comprehensive demos

---

## 🚀 Restoring Showcase Content

If you need the full showcase content:

```bash
# Option 1: Copy from archive
cp -r ../archive/beardog-dec-23-2025/showcase/* showcase/

# Option 2: Check git history
git log --all --full-history -- "showcase/*"

# Option 3: Restore specific examples
git checkout <commit-hash> -- showcase/<example-name>
```

---

## 📝 Creating New Showcase Examples

### Structure

```
showcase/
├── XX-category-name/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
```

### Template

```toml
# Cargo.toml
[package]
name = "beardog-showcase-example"
version = "0.1.0"
edition = "2021"

[dependencies]
beardog-core = { path = "../../crates/beardog-core" }
beardog-security = { path = "../../crates/beardog-security" }
# Add other dependencies as needed
```

---

## 🎯 Showcase Categories

### Planned Categories

1. **00-local-primal** - Local operations
2. **01-local-basics** - Basic functionality
3. **02-ecosystem-integration** - Multi-primal scenarios
4. **03-advanced-features** - Advanced capabilities
5. **04-production-patterns** - Production-ready patterns
6. **05-mixed-entropy** - ✅ Current example

---

## 📚 Documentation References

### Related Documentation
- [../README.md](../README.md) - Main project README
- [../START_HERE.md](../START_HERE.md) - Getting started guide
- [../examples/README.md](../examples/README.md) - Code examples

### Archived Documentation
- `../archive/beardog-dec-23-2025/showcase/00_SHOWCASE_INDEX.md` - Complete showcase index
- `../archive/beardog-dec-23-2025/showcase/00_START_HERE.md` - Archived start guide

---

## ✅ Git Status

**Current**: Showcase directory is now properly tracked by git  
**Updated**: .gitignore explicitly includes `!showcase/` and `!showcase/**/*`  
**Result**: All showcase examples will be included in commits and releases

---

## 🤝 Contributing

To add a new showcase example:

1. Create a new directory: `showcase/XX-example-name/`
2. Add `Cargo.toml` and `README.md`
3. Implement the example in `src/main.rs`
4. Test it: `cargo run`
5. Document it in this README
6. Commit: `git add showcase/XX-example-name/`

---

## 📞 Questions?

- **Main README**: [../README.md](../README.md)
- **Getting Started**: [../START_HERE.md](../START_HERE.md)
- **Examples**: [../examples/](../examples/)

---

**Last Updated**: December 24, 2025  
**Maintainer**: BearDog Team

🐻 **BearDog Showcase - Learn by Example** 🎬

