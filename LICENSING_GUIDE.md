# 🐻 BearDog Licensing Guide

## 🎯 **Our Refined Philosophy: Open Code, Crypto-Locked Functions**

BearDog uses a **refined licensing strategy** that maximizes transparency while ensuring sustainability:

- **📖 100% Open Source Code** - All code is AGPL, completely transparent, no black boxes
- **🔐 Crypto-Locked External Functions** - External integrations require BearDog-signed licenses  
- **🆓 Free Signed Licenses** - For basic users, universities, research that contributes back
- **💰 Enterprise Licenses** - Large companies pay (they can afford it), funding ecosystem development

---

## ✅ **COMPLETELY FREE & OPEN**

### 📖 **All Source Code (AGPL-3.0)**
- **Complete transparency** - every line of security code is visible
- **No proprietary components** - you can audit, modify, contribute
- **Full BearDog platform** - encryption, threat detection, compliance, workflows
- **All cryptographic implementations** - AES-256-GCM, ChaCha20Poly1305, RSA, post-quantum
- **Complete API and tooling** - REST API, CLI, configuration management
- **Native Rust integrations** - PostgreSQL, Redis, Tokio, Axum, etc.

### 🔓 **Core Functionality (Always Available)**
- **Security engines** - All threat detection and compliance logic
- **Encryption/decryption** - All cryptographic operations  
- **Authentication** - JWT, session management, rate limiting
- **Workflow processing** - Multi-party approvals, audit trails
- **Database operations** - PostgreSQL, SQLite via SQLx
- **Logging and monitoring** - Structured logging, native metrics
- **Configuration** - TOML, environment variables, validation

---

## 🔐 **CRYPTO-LOCKED EXTERNAL FUNCTIONS**

These functions exist in the open source code but are **cryptographically locked** until you have a valid BearDog-signed license:

### 🔍 **Monitoring & Observability**
- `prometheus_export` - Export metrics to Prometheus servers
- `grafana_dashboards` - Generate Grafana dashboard configs  
- `splunk_integration` - Send events to Splunk SIEM
- `datadog_metrics` - DataDog monitoring integration
- `newrelic_apm` - New Relic APM data export

### ☁️ **Cloud Provider Services**  
- `aws_kms_integration` - AWS Key Management Service
- `azure_keyvault` - Azure Key Vault operations
- `gcp_kms` - Google Cloud KMS integration
- `aws_secrets_manager` - AWS Secrets Manager
- `azure_secrets` - Azure Key Vault Secrets

### 🏢 **Enterprise Identity Systems**
- `active_directory` - Microsoft Active Directory
- `ldap_integration` - LDAP directory services  
- `okta_sso` - Okta single sign-on
- `auth0_integration` - Auth0 identity platform
- `ping_identity` - PingIdentity services

### 🔐 **Hardware Security Modules**
- `thales_hsm` - Thales nShield HSM integration
- `safenet_hsm` - SafeNet HSM operations
- `aws_cloudhsm` - AWS CloudHSM  
- `azure_dedicated_hsm` - Azure Dedicated HSM

### 🗄️ **Enterprise Databases**
- `oracle_database` - Oracle Database integration
- `mssql_integration` - Microsoft SQL Server
- `db2_integration` - IBM Db2 operations
- `mongodb_atlas` - MongoDB Atlas cloud service

---

## 🆓 **FREE SIGNED LICENSES**

We provide **completely free BearDog-signed licenses** for:

### ✅ **Always Free Categories**
- **👤 Individual developers** - Personal projects, learning, small apps
- **🏫 Universities & education** - All educational institutions  
- **🔬 Research institutions** - That contribute findings back to community
- **🌍 Non-profit organizations** - Social good and community projects
- **📂 Open source projects** - Community-driven development
- **🏢 Small businesses** - < 50 employees, reasonable usage limits

### 📋 **Free License Terms**
- **✅ 5-year validity** - Long-term stability for projects
- **✅ All external functions** - Full access to enterprise integrations
- **✅ Reasonable usage limits** - Generous quotas for non-enterprise use
- **✅ Community support** - GitHub issues, forums, documentation
- **✅ No hidden costs** - Completely free, no credit card required

---

## 💰 **ENTERPRISE LICENSES** 

Large enterprises pay for licenses because:
- **💪 They can afford it** - Enterprise budgets support ecosystem development
- **🎯 They get value** - Professional integrations with their existing systems
- **🛡️ Premium support** - SLA guarantees, priority assistance, custom development
- **📈 Funding innovation** - License fees fund continued open source development

### 🏢 **Enterprise Benefits**
- **All external functions unlocked**
- **No usage limits** - Scale to enterprise needs
- **Priority support** - Business hours or 24/7 premium support
- **SLA guarantees** - Uptime and response time commitments
- **Custom integrations** - Tailored solutions for specific needs
- **Legal protection** - Enterprise-grade licensing and indemnification

---

## 🔧 **How It Works**

### **For Open Source Development (Always Free)**
```rust
// This works immediately - no license needed
use beardog::encryption::EncryptionEngine;
use beardog::threat_detection::ThreatEngine;
use beardog::compliance::ComplianceEngine;

// All core functionality is completely free
let core = BearDogCore::new().await?;
let encrypted = core.encrypt_data(sensitive_data).await?;
let threats = core.detect_threats(security_events).await?;
let compliance = core.check_compliance(audit_data).await?;
```

### **For External Functions (License Required)**
```rust
// External functions are crypto-locked
use beardog::external::prometheus_export;

// License validation happens automatically
match core.enable_prometheus_export(config).await {
    Ok(_) => {
        println!("✅ Prometheus export enabled with valid license");
        // Function works normally
    },
    Err(e) => {
        println!("🔒 License required: {}", e);
        // Clear instructions on getting free or paid license
    }
}
```

### **License Loading**
```rust
// Load your BearDog-signed license
let license_json = std::fs::read_to_string("beardog-license.json")?;
core.license_manager().load_signed_license(&license_json)?;

// Now external functions work seamlessly
core.export_to_prometheus().await?; // ✅ Works with valid license
```

---

## 📞 **Getting Your License**

### 🆓 **Free Licenses**
**Contact:** [free-licenses@beardog-security.com](mailto:free-licenses@beardog-security.com)

**What to include:**
- Organization name and type (university, research, non-profit, etc.)
- Contact email
- Brief description of your use case
- Which external functions you need
- For research: How you'll contribute back to the community

**Processing time:** Usually same day for qualifying organizations

### 💼 **Enterprise Licenses**  
**Contact:** [enterprise@beardog-security.com](mailto:enterprise@beardog-security.com)

**What we provide:**
- Custom pricing based on your needs
- Professional implementation support
- SLA guarantees and premium support
- Legal review and enterprise contracts
- Training and onboarding assistance

### 🧪 **Trial Licenses**
**Contact:** [trial@beardog-security.com](mailto:trial@beardog-security.com)

- **30-day free trial** of all external functions
- **No credit card required**
- **Full feature access** for evaluation
- **Easy conversion** to permanent license

---

## 💡 **Why This Model Works**

### **🎯 For Individual Developers**
- **Zero barriers** - All security code is free and open
- **No vendor lock-in** - You can fork, modify, contribute
- **Enterprise-grade tools** - Same quality as Fortune 500 companies use
- **Free external integrations** - Get signed licenses at no cost

### **🏫 For Universities & Research**
- **Complete access** - All functions available for free
- **Long-term stability** - 5-year licenses for multi-year projects  
- **No budget constraints** - Education and research shouldn't be limited by cost
- **Knowledge sharing** - Contribute discoveries back to benefit everyone

### **🏢 For Enterprises**
- **Pay only for external integrations** - Core platform is always free
- **Support ecosystem development** - License fees fund continued innovation
- **Professional support** - Get help when you need it most
- **Custom solutions** - Tailored integrations for your specific needs

### **🌍 For the Ecosystem**
- **Sustainable development** - Enterprise licenses fund ongoing work
- **Open innovation** - All improvements benefit everyone
- **No fragmentation** - Single codebase serves all users
- **Community growth** - More users = more contributors = better security

---

## ⚖️ **Legal Summary**

### **Open Source Components (AGPL-3.0)**
- **All source code** - Complete transparency and freedom
- **Core functionality** - Always available, no restrictions
- **Derivative works** - Must also be AGPL (copyleft protection)
- **Commercial use** - Allowed with AGPL compliance

### **Licensed Components (Proprietary)**
- **External function licenses** - Cryptographically signed by BearDog
- **License verification** - Built into the code, tamper-resistant
- **Free for qualifying users** - Individuals, education, research, non-profits
- **Paid for enterprises** - Large companies support ecosystem development

### **The Bottom Line**
**If you're learning, researching, or building open source projects → Everything is FREE**

**If you're a large enterprise needing external integrations → You pay to support the ecosystem**

**Everyone gets the same high-quality, transparent, secure code.**

---

*BearDog: Democratizing enterprise security through open source innovation and fair licensing.* 