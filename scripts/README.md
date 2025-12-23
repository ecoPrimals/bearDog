# 🛠️ BearDog Scripts Directory

**Organized collection of development, deployment, and maintenance scripts**

## 📁 Directory Structure

### **🚀 Active Scripts**
Scripts that are currently maintained and used in production workflows:

- **`active/deployment/`** - Production deployment scripts
  - `deploy-production.sh` - Main production deployment
  - `production_deploy.sh` - Alternative deployment script

- **`active/validation/`** - System validation and security scripts  
  - `validate-system.sh` - Comprehensive system validation
  - `security_audit.sh` - Security audit and compliance checks

- **`active/maintenance/`** - Ongoing maintenance utilities
  - (Reserved for future maintenance scripts)

### **📦 Archived Scripts**
Historical scripts organized by category - kept for reference:

- **`archived/migration/`** - Codebase migration and unification scripts
- **`archived/pedantic/`** - Code quality and linting automation
- **`archived/performance/`** - Performance analysis and optimization
- **`archived/modernization/`** - Legacy code modernization utilities
- **`archived/production/`** - Historical production scripts

### **🧰 Utility Scripts**
Standalone utility scripts:

- **`beardog_unwrap_migrator.rs`** - Unwrap migration utility
- **`pixel8a_simple_hsm_test.rs`** - HSM testing for Pixel devices

## 🎯 **Usage Guidelines**

### **For Production Use**
```bash
# Deploy to production
./active/deployment/deploy-production.sh

# Run system validation
./active/validation/validate-system.sh

# Security audit
./active/validation/security_audit.sh
```

### **For Development**
```bash
# Build unwrap migrator
cargo build --bin beardog_unwrap_migrator

# Run HSM tests
cargo run --bin pixel8a_simple_hsm_test
```

## 📋 **Script Maintenance**

### **Active Scripts Policy**
- Must be tested and validated
- Should have documentation headers
- Regular maintenance and updates
- Production-ready quality

### **Archived Scripts Policy**
- Historical reference only
- No longer actively maintained
- May contain deprecated patterns
- Useful for understanding evolution

## 🔍 **Finding Scripts**

### **By Purpose**
- **Deployment**: Check `active/deployment/`
- **Validation**: Check `active/validation/`
- **Historical Reference**: Check `archived/` subdirectories

### **By Technology**
- **Shell Scripts**: `.sh` files for system operations
- **Python Scripts**: `.py` files for automation (mostly archived)
- **Rust Programs**: `.rs` files for compiled utilities

## 🚀 **Contributing New Scripts**

1. **Active Scripts**: Place in appropriate `active/` subdirectory
2. **Add Documentation**: Include purpose and usage in script header
3. **Test Thoroughly**: Ensure production-ready quality
4. **Update README**: Document new scripts here

## 📊 **Script Categories**

| Category | Count | Status | Purpose |
|----------|-------|--------|---------|
| **Active Deployment** | 2 | ✅ Maintained | Production deployment |
| **Active Validation** | 2 | ✅ Maintained | System validation |
| **Utility Programs** | 2 | ✅ Maintained | Development utilities |
| **Archived Migration** | ~30 | 📦 Archived | Historical migration |
| **Archived Pedantic** | ~15 | 📦 Archived | Code quality automation |
| **Archived Performance** | ~10 | 📦 Archived | Performance optimization |

---

**📁 Clean, Organized, Production-Ready Scripts Directory** 🛠️ 