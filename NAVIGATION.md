# 🗺️ BearDog Navigation Guide

**Single Source of Truth for Project Navigation**

---

## 🎯 **Quick Links**

### **For New Users**
1. **[README.md](README.md)** - Start here! Project overview
2. **[QUICK_START.md](QUICK_START.md)** - Get running in 5 minutes
3. **[docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)** - Detailed setup guide

### **For Developers**
1. **[BEARDOG_CODING_STANDARDS.md](BEARDOG_CODING_STANDARDS.md)** - Code standards
2. **[docs/DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md)** - Contributing guide
3. **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture
4. **[BEARDOG_QUICK_REFERENCE.md](BEARDOG_QUICK_REFERENCE.md)** - API quick reference

### **For Operators**
1. **[docs/PRODUCTION_DEPLOYMENT_GUIDE.md](docs/PRODUCTION_DEPLOYMENT_GUIDE.md)** - Deployment
2. **[SECURITY.md](SECURITY.md)** - Security policies
3. **[configs/README.md](configs/README.md)** - Configuration guide

---

## 📚 **Documentation Structure**

### **Root Level Docs**
```
/
├── README.md                        # Project overview (START HERE)
├── NAVIGATION.md                    # This file - navigation guide
├── QUICK_START.md                   # 5-minute quick start
├── ARCHITECTURE.md                  # System architecture
├── CHANGELOG.md                     # Version history
├── SECURITY.md                      # Security policies
├── LICENSE                          # License information
├── BEARDOG_CODING_STANDARDS.md      # Code standards & conventions
└── BEARDOG_QUICK_REFERENCE.md       # API quick reference
```

### **Documentation Directory** (`docs/`)
```
docs/
├── GETTING_STARTED.md               # Detailed installation & setup
├── DEVELOPER_GUIDE.md               # Contributing guide
├── API_DOCUMENTATION.md             # API reference
├── PRODUCTION_DEPLOYMENT_GUIDE.md   # Production deployment
├── ECOSYSTEM_INTEGRATION_GUIDE.md   # Integration patterns
├── PERFORMANCE_GUIDE.md             # Performance optimization
├── IDIOMATIC_RUST_GUIDE.md          # Rust best practices
│
├── architecture/                    # Architecture documentation
├── guides/                          # Detailed guides
├── api/                             # API documentation
├── security/                        # Security documentation
└── archive/                         # Historical documents
```

### **Configuration** (`configs/`)
```
configs/
├── README.md                        # Configuration guide
├── beardog-config.toml              # Main configuration
├── production.toml                  # Production config
├── development.env                  # Development environment
└── example-beardog-config.toml      # Example config template
```

### **Specifications** (`specs/`)
```
specs/
├── current/                         # Current specifications
│   ├── integration/                 # Integration specs
│   └── security/                    # Security specs
└── archive/                         # Historical specs
```

---

## 🔍 **Finding What You Need**

### **"How do I...?"**

| Task | Where to Look |
|------|--------------|
| Get Started | [README.md](README.md) → [QUICK_START.md](QUICK_START.md) |
| Install & Configure | [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) → [configs/README.md](configs/README.md) |
| Use the CLI | [README.md](README.md#quick-start) |
| Understand Architecture | [ARCHITECTURE.md](ARCHITECTURE.md) → [docs/architecture/](docs/architecture/) |
| Integrate with HSMs | [docs/guides/HSM_INTEGRATION_GUIDE.md](docs/guides/HSM_INTEGRATION_GUIDE.md) |
| Deploy to Production | [docs/PRODUCTION_DEPLOYMENT_GUIDE.md](docs/PRODUCTION_DEPLOYMENT_GUIDE.md) |
| Contribute Code | [docs/DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md) → [BEARDOG_CODING_STANDARDS.md](BEARDOG_CODING_STANDARDS.md) |
| Report Security Issues | [SECURITY.md](SECURITY.md) |
| Find API Reference | [BEARDOG_QUICK_REFERENCE.md](BEARDOG_QUICK_REFERENCE.md) → [docs/API_DOCUMENTATION.md](docs/API_DOCUMENTATION.md) |

---

## 🎯 **By Role**

### **User** (Using BearDog CLI)
1. [README.md](README.md) - Overview
2. [QUICK_START.md](QUICK_START.md) - Installation
3. CLI help: `./target/release/beardog --help`

### **Developer** (Contributing Code)
1. [docs/DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md) - How to contribute
2. [BEARDOG_CODING_STANDARDS.md](BEARDOG_CODING_STANDARDS.md) - Code style
3. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
4. [docs/IDIOMATIC_RUST_GUIDE.md](docs/IDIOMATIC_RUST_GUIDE.md) - Rust patterns

### **Integrator** (Building on BearDog)
1. [docs/API_DOCUMENTATION.md](docs/API_DOCUMENTATION.md) - API reference
2. [docs/ECOSYSTEM_INTEGRATION_GUIDE.md](docs/ECOSYSTEM_INTEGRATION_GUIDE.md) - Integration
3. [docs/guides/](docs/guides/) - Integration guides
4. [examples/](examples/) - Code examples

### **Operator** (Running in Production)
1. [docs/PRODUCTION_DEPLOYMENT_GUIDE.md](docs/PRODUCTION_DEPLOYMENT_GUIDE.md) - Deployment
2. [configs/README.md](configs/README.md) - Configuration
3. [SECURITY.md](SECURITY.md) - Security policies

### **Architect** (Understanding Design)
1. [ARCHITECTURE.md](ARCHITECTURE.md) - High-level architecture
2. [docs/architecture/](docs/architecture/) - Detailed architecture
3. [specs/](specs/) - Technical specifications
4. [whitePaper/](whitePaper/) - Research & concepts

---

## 📂 **Directory Structure**

```
beardog/
├── README.md                    # 👈 START HERE
├── NAVIGATION.md                # 👈 YOU ARE HERE
├── crates/                      # Source code (26 crates)
├── docs/                        # Documentation
├── specs/                       # Specifications
├── configs/                     # Configuration files
├── tests/                       # Integration tests
├── examples/                    # Usage examples
├── scripts/                     # Utility scripts
├── tools/                       # Development tools
└── whitePaper/                  # Research papers
```

---

## 💡 **Tips**

### **Finding Code**
```bash
# Search for specific functionality
rg "pattern" crates/

# Find a specific crate
ls crates/beardog-*

# View crate structure
tree crates/beardog-core -L 2
```

### **Finding Documentation**
```bash
# Search all markdown files
rg "search term" -g "*.md"

# List all guides
ls docs/guides/

# Generate Rust docs
cargo doc --workspace --no-deps --open
```

### **Running Tests**
```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo llvm-cov --workspace
```

---

## 📊 **Project Status**

| Metric | Value |
|--------|-------|
| Test Coverage | 78.64% |
| Tests Passing | 500+ |
| Rust Purity | 100% |
| Unsafe Code | 0 |

---

**Need help?** Check [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) or open an issue!

---

**🐻 BearDog** - *Clear Navigation for Sovereign Security*
