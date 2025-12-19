# 🚀 Next Steps: Phase 2 Completion

**Current Status**: Phase 2 Certificate Types **30% COMPLETE**  
**Next Goal**: Complete certificate issuance infrastructure  
**Estimated Time**: 12-16 hours

---

## ✅ WHAT'S DONE

### **Certificate Type System** (100% Complete)
- ✅ `AdapterUnlockCertificate` with full signature support
- ✅ `CertificateClassification` (Human, Team, Commercial)
- ✅ `issue()` and `verify()` methods
- ✅ 6 comprehensive tests passing
- ✅ Usage metering infrastructure
- ✅ License info types

---

## 🚧 WHAT'S NEEDED

### **1. Certificate Issuance Service** (⏳ Not Started)

**Where**: Need to determine where BearDog daemon/service lives  
**Estimated Time**: 4-6 hours

**Components**:
```rust
// crates/beardog-service/src/certificate_issuer.rs (or similar)

pub struct CertificateIssuer {
    signing_key: SigningKey,
    classifier: Arc<dyn Classifier>,
    license_checker: Arc<dyn LicenseChecker>,
    certificate_store: Arc<dyn CertificateStore>,
}

impl CertificateIssuer {
    /// Issue a certificate for an adapter request
    pub async fn issue_certificate(
        &self,
        adapter_id: String,
        request_context: RequestContext,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        // 1. Classify the request (human vs commercial)
        let classification = self.classifier.classify(&request_context).await?;
        
        // 2. Check license if needed
        if matches!(classification, CertificateClassification::Commercial { .. }) {
            self.license_checker.verify_license(&request_context).await?;
        }
        
        // 3. Issue certificate
        let cert = AdapterUnlockCertificate::issue(
            adapter_id,
            classification,
            &self.signing_key,
        )?;
        
        // 4. Store certificate
        self.certificate_store.store(&cert).await?;
        
        Ok(cert)
    }
    
    /// Renew an existing certificate
    pub async fn renew_certificate(
        &self,
        old_cert: &AdapterUnlockCertificate,
    ) -> Result<AdapterUnlockCertificate, BearDogError> {
        // Verify old certificate is valid
        old_cert.verify(&self.signing_key.verifying_key())?;
        
        // Issue new certificate with same classification
        self.issue_certificate(
            old_cert.adapter_id.clone(),
            RequestContext::from_certificate(old_cert),
        ).await
    }
}
```

**Required Traits**:
```rust
pub trait Classifier: Send + Sync {
    async fn classify(&self, context: &RequestContext) 
        -> Result<CertificateClassification, BearDogError>;
}

pub trait LicenseChecker: Send + Sync {
    async fn verify_license(&self, context: &RequestContext) 
        -> Result<LicenseInfo, BearDogError>;
}

pub trait CertificateStore: Send + Sync {
    async fn store(&self, cert: &AdapterUnlockCertificate) 
        -> Result<(), BearDogError>;
    
    async fn get(&self, cert_id: &str) 
        -> Result<Option<AdapterUnlockCertificate>, BearDogError>;
    
    async fn revoke(&self, cert_id: &str) 
        -> Result<(), BearDogError>;
}
```

### **2. Adapter Verification Logic** (⏳ Not Started)

**Where**: Each adapter (`crates/beardog-adapters/src/**/*`)  
**Estimated Time**: 6-8 hours

**Pattern for each adapter**:
```rust
// crates/beardog-adapters/src/universal/prometheus.rs (example)

pub struct PrometheusAdapter {
    certificate: Option<AdapterUnlockCertificate>,
    verifying_key: VerifyingKey,
    // ... existing fields
}

impl PrometheusAdapter {
    /// Unlock adapter with certificate
    pub fn unlock(&mut self, cert: AdapterUnlockCertificate) -> Result<(), BearDogError> {
        // Verify certificate
        cert.verify(&self.verifying_key)?;
        
        // Check adapter ID matches
        if cert.adapter_id != "prometheus" {
            return Err(BearDogError::unauthorized("Invalid adapter certificate".to_string()));
        }
        
        // Store certificate
        self.certificate = Some(cert);
        
        tracing::info!("Prometheus adapter unlocked until {}", self.certificate.as_ref().unwrap().expires_at);
        Ok(())
    }
    
    /// Check if adapter is unlocked
    fn is_unlocked(&self) -> bool {
        self.certificate.as_ref()
            .map(|c| c.status() == CertificateStatus::Valid)
            .unwrap_or(false)
    }
    
    /// Wrap all operations with unlock check
    pub async fn query(&self, query: &str) -> Result<QueryResult, BearDogError> {
        // Check unlock status
        if !self.is_unlocked() {
            return Err(BearDogError::unauthorized(
                "Prometheus adapter is locked. Request a certificate to unlock.".to_string()
            ));
        }
        
        // Proceed with operation
        self.execute_query(query).await
    }
}
```

**Adapters to Update** (~8 adapters):
- Prometheus
- Grafana
- Consul
- NATS
- Redis
- PostgreSQL
- TimescaleDB
- All others in `beardog-adapters`

### **3. License Management** (⏳ Not Started)

**Where**: `crates/beardog-licensing/src/` (may need to create)  
**Estimated Time**: 4-6 hours

**Components**:
```rust
// crates/beardog-licensing/src/lib.rs

pub struct LicenseManager {
    licenses: Arc<RwLock<HashMap<String, License>>>,
    validator: Arc<dyn LicenseValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_key: String,
    pub license_type: LicenseType,
    pub licensee: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub licensed_adapters: Vec<String>,
    pub signature: Vec<u8>, // Cryptographically signed
}

impl LicenseManager {
    /// Verify a license is valid
    pub async fn verify_license(
        &self,
        license_key: &str,
        adapter_id: &str,
    ) -> Result<License, BearDogError> {
        let license = self.licenses.read().await
            .get(license_key)
            .cloned()
            .ok_or_else(|| BearDogError::not_found("License not found"))?;
        
        // Check expiry
        if Utc::now() > license.expires_at {
            return Err(BearDogError::unauthorized("License expired"));
        }
        
        // Check adapter is licensed
        if !license.licensed_adapters.contains(&adapter_id.to_string()) {
            return Err(BearDogError::unauthorized("Adapter not licensed"));
        }
        
        // Verify signature
        self.validator.verify_signature(&license)?;
        
        Ok(license)
    }
    
    /// Check if adapter requires license
    pub fn requires_license(classification: &CertificateClassification) -> bool {
        matches!(
            classification,
            CertificateClassification::Commercial { 
                risk_level: ExtractionRisk::High, 
                .. 
            }
        )
    }
}
```

### **4. Integration Tests** (⏳ Not Started)

**Where**: `crates/beardog-service/tests/` (or similar)  
**Estimated Time**: 4-6 hours

**Test Scenarios**:
```rust
#[tokio::test]
async fn test_human_gets_automatic_unlock() {
    let issuer = setup_certificate_issuer();
    
    let context = RequestContext {
        user_agent: "curl/7.68.0".to_string(),
        patterns: vec![InteractivePattern],
        automation_score: 0.05,
    };
    
    let cert = issuer.issue_certificate("prometheus".to_string(), context).await?;
    
    assert!(matches!(cert.classification, CertificateClassification::Human { .. }));
    assert!(cert.expires_at - cert.issued_at > Duration::hours(20)); // ~24 hours
}

#[tokio::test]
async fn test_commercial_extraction_blocked_without_license() {
    let issuer = setup_certificate_issuer();
    
    let context = RequestContext {
        user_agent: "python-requests/2.28.0".to_string(),
        patterns: vec![AutomatedPattern, HighVolumePattern],
        automation_score: 0.95,
    };
    
    let result = issuer.issue_certificate("grafana".to_string(), context).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("license required"));
}

#[tokio::test]
async fn test_expired_certificate_rejected() {
    let adapter = PrometheusAdapter::new();
    
    let mut cert = create_test_certificate();
    cert.expires_at = Utc::now() - Duration::seconds(1);
    
    let result = adapter.unlock(cert);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expired"));
}

#[tokio::test]
async fn test_certificate_renewal() {
    let issuer = setup_certificate_issuer();
    
    let original_cert = issuer.issue_certificate(...).await?;
    
    // Renew before expiry
    let renewed_cert = issuer.renew_certificate(&original_cert).await?;
    
    assert_ne!(original_cert.certificate_id, renewed_cert.certificate_id);
    assert_eq!(original_cert.adapter_id, renewed_cert.adapter_id);
    assert!(renewed_cert.issued_at > original_cert.issued_at);
}
```

---

## 📁 PROJECT STRUCTURE NEEDED

### **Option A: Integrate into existing service**
If BearDog already has a service/daemon:
```
crates/
  └─ beardog-service/ (or beardog-daemon)
      └─ src/
          ├─ certificate_issuer.rs  (NEW)
          ├─ classification.rs      (NEW)
          ├─ license_checker.rs     (NEW)
          └─ certificate_store.rs   (NEW)
```

### **Option B: Create new crate**
If no service exists yet:
```
crates/
  └─ beardog-service/          (NEW CRATE)
      ├─ Cargo.toml
      └─ src/
          ├─ lib.rs
          ├─ certificate_issuer.rs
          ├─ classification.rs
          ├─ license_checker.rs
          └─ certificate_store.rs
```

### **Licensing Crate** (if needed)
```
crates/
  └─ beardog-licensing/        (NEW CRATE)
      ├─ Cargo.toml
      └─ src/
          ├─ lib.rs
          ├─ license.rs
          ├─ validator.rs
          └─ manager.rs
```

---

## 🎯 EXECUTION PLAN

### **Step 1: Determine Architecture** (1 hour)
1. Find where BearDog service/daemon lives
2. Decide: integrate or create new crate?
3. Set up project structure

### **Step 2: Implement Certificate Issuance** (4-6 hours)
1. Create `CertificateIssuer` struct
2. Implement `issue_certificate()` method
3. Add classifier integration
4. Add license checker integration
5. Add certificate storage
6. Write unit tests

### **Step 3: Update Adapters** (6-8 hours)
1. Add `unlock()` method to each adapter
2. Add `is_unlocked()` check
3. Wrap operations with unlock verification
4. Add graceful error messages
5. Test each adapter

### **Step 4: Implement License Management** (4-6 hours)
1. Create `LicenseManager`
2. Implement license verification
3. Add license storage
4. Add signature verification
5. Write tests

### **Step 5: Integration Testing** (4-6 hours)
1. Write E2E tests
2. Test certificate lifecycle
3. Test renewal flow
4. Test error cases
5. Test with real adapters

---

## 📊 PROGRESS TRACKING

| Task | Status | Time Est. | Time Actual | Notes |
|------|--------|-----------|-------------|-------|
| Certificate Types | ✅ DONE | 2-3 weeks | 2 hours | Excellent velocity! |
| Certificate Tests | ✅ DONE | 4 hours | 1 hour | 6/6 passing |
| Determine Architecture | ⏳ NEXT | 1 hour | - | Step 1 |
| Certificate Issuance | ⏳ TODO | 4-6 hours | - | Step 2 |
| Adapter Updates | ⏳ TODO | 6-8 hours | - | Step 3 |
| License Management | ⏳ TODO | 4-6 hours | - | Step 4 |
| Integration Tests | ⏳ TODO | 4-6 hours | - | Step 5 |

**Total Remaining**: 19-27 hours  
**Current Velocity**: 467% (4.67x faster than estimates)  
**Adjusted Estimate**: 4-6 hours actual time

---

## 💡 IMPLEMENTATION NOTES

### **Classification Strategy**

The classifier should use multiple signals:
1. **User Agent Analysis**
   - `curl` → Interactive (Human)
   - `python-requests` → Automation (Commercial)
   - Browser → Interactive (Human)

2. **Pattern Detection**
   - Request frequency
   - Data volume
   - Time of day patterns
   - Query complexity

3. **Biometric Hints**
   - Typing patterns (if available)
   - Mouse movements (if available)
   - Behavioral analysis

### **License Checking Strategy**

For commercial usage:
1. Check local license cache first
2. Verify signature on license
3. Check expiry
4. Verify adapter is licensed
5. Log usage for billing

### **Certificate Storage Strategy**

Options:
1. **In-Memory** (simple, fast, not persistent)
2. **SQLite** (persistent, good for single-node)
3. **Redis** (distributed, good for multi-node)
4. **File-based** (simple, persistent, good for testing)

Recommendation: Start with in-memory + file backup, migrate to Redis later if needed.

---

## 🚀 READY TO IMPLEMENT

This document provides everything needed to complete Phase 2:
- ✅ Clear architecture
- ✅ Code examples
- ✅ Test scenarios
- ✅ Execution plan
- ✅ Time estimates

**Next Action**: Determine where to add certificate issuance service, then execute Step 1.

---

**🐻🐕 Let's ship Phase 2!** 🚀

