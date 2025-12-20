# Human Entropy Collection - Extensibility Guide

**Date**: December 19, 2025  
**Status**: Production-Ready Foundation  
**Philosophy**: Agnostic, Modular, Sovereign

---

## 🎯 Vision

BearDog's human entropy collection is designed to be **infinitely extensible**. We start with the basics (keyboard and mouse), but the architecture supports **any sensor** that can provide unique, non-fungible human data.

---

## 🏗️ Current Architecture

### **Interaction Capture** (✅ Implemented)

**Module**: `interaction_capture.rs`

**Capabilities**:
- ✅ Keyboard timing capture (keystroke dynamics)
- ✅ Mouse movement patterns (jitter, velocity, acceleration)
- ✅ Real-time quality metrics
- ✅ Terminal UI with progress visualization
- ✅ LiveFeedValidator integration (NO SIMULATION)

**Entropy Sources**:
- Precise timing between interactions (nanosecond resolution)
- Movement deltas (relative, not absolute for privacy)
- Interaction patterns (natural human variation)
- Shannon entropy calculation
- Statistical quality scoring

---

## 🔮 Future Sensor Types

### **1. Biometric Sensors** (Phase 2)

#### **Fingerprint**
```rust
pub struct FingerprintEntropyCollector {
    config: FingerprintConfig,
    live_feed_validator: LiveFeedValidator,
}

impl FingerprintEntropyCollector {
    pub fn collect_fingerprint_entropy(
        &self,
        consent: &InformedConsent,
    ) -> Result<FingerprintEntropy, BearDogError> {
        // Collect minutiae points (NOT the actual fingerprint image)
        // Extract timing of ridge patterns
        // Validate live finger (not photo/mold)
        // Generate entropy from unique ridge characteristics
    }
}
```

**Entropy From**:
- Ridge pattern timing
- Pressure variations
- Temperature gradients
- Capacitance changes (live finger detection)

#### **Voice/Audio**
```rust
pub struct VoiceEntropyCollector {
    config: VoiceConfig,
    live_feed_validator: LiveFeedValidator,
}

impl VoiceEntropyCollector {
    pub fn collect_voice_entropy(
        &self,
        duration: Duration,
        consent: &InformedConsent,
    ) -> Result<VoiceEntropy, BearDogError> {
        // Collect voice sample (with consent)
        // Extract spectral features (NOT voice content)
        // Validate live voice (not recording)
        // Generate entropy from unique vocal characteristics
    }
}
```

**Entropy From**:
- Spectral centroid variations
- Formant frequencies
- Pitch jitter
- Amplitude modulation
- Temporal patterns

---

### **2. Wearable Sensors** (Phase 3)

#### **Heart Rate / EKG**
```rust
pub struct HeartRateEntropyCollector {
    config: HeartRateConfig,
    device_connection: WearableConnection,
    live_feed_validator: LiveFeedValidator,
}

impl HeartRateEntropyCollector {
    pub fn collect_heart_rate_entropy(
        &self,
        duration: Duration,
        consent: &InformedConsent,
    ) -> Result<HeartRateEntropy, BearDogError> {
        // Connect to wearable (Apple Watch, Fitbit, etc.)
        // Collect R-R intervals (heart rate variability)
        // Validate live data (not replayed)
        // Generate entropy from HRV patterns
    }
}
```

**Entropy From**:
- R-R interval variations (HRV)
- Beat-to-beat timing
- Respiratory sinus arrhythmia
- Circadian rhythm patterns

#### **Accelerometer / Gyroscope**
```rust
pub struct MotionEntropyCollector {
    config: MotionConfig,
    device_connection: WearableConnection,
    live_feed_validator: LiveFeedValidator,
}

impl MotionEntropyCollector {
    pub fn collect_motion_entropy(
        &self,
        duration: Duration,
        consent: &InformedConsent,
    ) -> Result<MotionEntropy, BearDogError> {
        // Collect accelerometer/gyroscope data
        // Extract gait patterns, hand tremor, natural movement
        // Validate live data (not simulated)
        // Generate entropy from unique movement signatures
    }
}
```

**Entropy From**:
- Gait patterns
- Hand tremor (natural micro-movements)
- Device orientation changes
- Acceleration/deceleration patterns

---

### **3. Advanced Biometrics** (Phase 4)

#### **DNA Sequencing**
```rust
pub struct DNAEntropyCollector {
    config: DNAConfig,
    sequencer_connection: SequencerConnection,
    live_feed_validator: LiveFeedValidator,
}

impl DNAEntropyCollector {
    pub fn collect_dna_entropy(
        &self,
        consent: &InformedConsent,
    ) -> Result<DNAEntropy, BearDogError> {
        // Connect to portable DNA sequencer
        // Extract SNP (Single Nucleotide Polymorphism) patterns
        // Validate live sample (not synthetic)
        // Generate entropy from unique genetic markers
        // CRITICAL: Privacy-preserving (no full genome storage)
    }
}
```

**Entropy From**:
- SNP patterns (specific markers only)
- Microsatellite variations
- Mitochondrial DNA patterns
- Epigenetic markers

**Privacy Guarantees**:
- Only extract entropy, NOT store DNA sequence
- Use one-way hashing
- No genomic data retention
- Informed consent required

#### **Retinal Scan**
```rust
pub struct RetinalEntropyCollector {
    config: RetinalConfig,
    camera_connection: CameraConnection,
    live_feed_validator: LiveFeedValidator,
}

impl RetinalEntropyCollector {
    pub fn collect_retinal_entropy(
        &self,
        consent: &InformedConsent,
    ) -> Result<RetinalEntropy, BearDogError> {
        // Capture retinal blood vessel pattern
        // Extract unique vascular features
        // Validate live eye (not photo)
        // Generate entropy from vascular patterns
    }
}
```

**Entropy From**:
- Blood vessel branching patterns
- Vessel diameter variations
- Optic disc characteristics
- Foveal avascular zone patterns

---

### **4. Environmental Sensors** (Phase 5)

#### **Ambient Noise**
```rust
pub struct AmbientNoiseEntropyCollector {
    config: AmbientNoiseConfig,
    microphone: MicrophoneConnection,
}

impl AmbientNoiseEntropyCollector {
    pub fn collect_ambient_entropy(
        &self,
        duration: Duration,
    ) -> Result<AmbientEntropy, BearDogError> {
        // Capture ambient environmental noise
        // Extract spectral characteristics
        // Generate entropy from unique acoustic environment
    }
}
```

**Entropy From**:
- Room acoustics
- Background noise patterns
- Environmental sounds
- Acoustic fingerprint of location

---

## 🔧 How to Add a New Sensor

### **Step 1: Define the Collector**

```rust
// File: src/genetics/human_entropy/your_sensor.rs

use beardog_errors::BearDogError;
use crate::genetics::entropy_hierarchy::LiveFeedValidator;

pub struct YourSensorConfig {
    // Sensor-specific configuration
    pub sample_rate: u32,
    pub duration: Duration,
    pub min_quality: f64,
}

pub struct YourSensorEntropyCollector {
    config: YourSensorConfig,
    live_feed_validator: LiveFeedValidator,
}

impl YourSensorEntropyCollector {
    pub fn new(config: YourSensorConfig) -> Self {
        Self {
            config,
            live_feed_validator: LiveFeedValidator::new(),
        }
    }

    pub fn collect_entropy(
        &self,
        consent: &InformedConsent,
    ) -> Result<YourSensorEntropy, BearDogError> {
        // 1. Validate consent
        self.validate_consent(consent)?;

        // 2. Collect LIVE sensor data
        let raw_data = self.collect_live_data()?;

        // 3. CRITICAL: Validate live feed (NO SIMULATION)
        let mut metadata = HashMap::new();
        metadata.insert("sensor_type".to_string(), "your_sensor".to_string());
        metadata.insert("hardware_attestation".to_string(), "true".to_string());
        metadata.insert("anti_replay_nonce".to_string(), Uuid::new_v4().to_string());

        let validation_result = self
            .live_feed_validator
            .validate_live_feed_only(&raw_data, &metadata)?;

        if !validation_result.is_live {
            return Err(BearDogError::security(
                "CRITICAL: Simulated sensor data detected - only live input allowed"
            ));
        }

        // 4. Extract entropy features
        let features = self.extract_features(&raw_data)?;

        // 5. Calculate quality metrics
        let quality = self.calculate_quality(&features)?;

        // 6. Derive entropy bytes
        let entropy_bytes = self.derive_entropy(&features)?;

        Ok(YourSensorEntropy {
            entropy_bytes,
            quality,
            features,
            collection_timestamp: Utc::now(),
        })
    }

    fn collect_live_data(&self) -> Result<Vec<u8>, BearDogError> {
        // Implement sensor-specific data collection
        // MUST be from LIVE hardware, not simulation
        todo!("Implement live data collection for your sensor")
    }

    fn extract_features(&self, data: &[u8]) -> Result<Features, BearDogError> {
        // Extract sensor-specific features
        todo!("Implement feature extraction")
    }

    fn calculate_quality(&self, features: &Features) -> Result<f64, BearDogError> {
        // Calculate entropy quality (0.0-1.0)
        todo!("Implement quality calculation")
    }

    fn derive_entropy(&self, features: &Features) -> Result<Vec<u8>, BearDogError> {
        // Derive entropy bytes using cryptographic hashing
        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        // Hash features to derive entropy
        // ...
        Ok(hasher.finalize().to_vec())
    }
}
```

### **Step 2: Define the Entropy Type**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourSensorEntropy {
    pub entropy_bytes: Vec<u8>,
    pub quality: f64,
    pub features: YourSensorFeatures,
    pub collection_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourSensorFeatures {
    // Sensor-specific features
    pub feature1: f64,
    pub feature2: f64,
    // ...
}
```

### **Step 3: Add to Module Exports**

```rust
// File: src/genetics/human_entropy/mod.rs

pub mod interaction_capture;
pub mod your_sensor; // Add your sensor module

pub use your_sensor::{
    YourSensorConfig,
    YourSensorEntropyCollector,
    YourSensorEntropy,
};
```

### **Step 4: Integrate into CLI** (Optional)

```rust
// File: crates/beardog-cli/src/handlers/entropy.rs

if sensor_type == "your_sensor" {
    let config = YourSensorConfig::default();
    let collector = YourSensorEntropyCollector::new(config);
    let result = collector.collect_entropy(&consent)?;
    // Use result.entropy_bytes
}
```

---

## 🔐 Critical Requirements

### **1. Live Feed Validation** (MANDATORY)

**EVERY** sensor collector MUST:
- ✅ Use `LiveFeedValidator` to verify live data
- ✅ Include hardware attestation metadata
- ✅ Include anti-replay nonce
- ✅ Return error if simulation is detected

### **2. Informed Consent** (MANDATORY)

**EVERY** sensor collector MUST:
- ✅ Require explicit `InformedConsent` from user
- ✅ Document what data is collected
- ✅ Document how entropy is derived
- ✅ Provide opt-out mechanism

### **3. Privacy Preservation** (MANDATORY)

**EVERY** sensor collector MUST:
- ✅ Extract entropy, NOT store raw biometric data
- ✅ Use one-way hashing for entropy derivation
- ✅ Minimize data retention
- ✅ Support data deletion (right to be forgotten)

### **4. Quality Metrics** (RECOMMENDED)

**EVERY** sensor collector SHOULD:
- ✅ Calculate entropy quality score (0.0-1.0)
- ✅ Provide real-time feedback to user
- ✅ Reject low-quality data
- ✅ Document quality thresholds

---

## 🌍 Multi-Modal Fusion

### **Combining Multiple Sensors**

```rust
pub struct MultiModalEntropyCollector {
    interaction: Option<InteractionEntropyCollector>,
    voice: Option<VoiceEntropyCollector>,
    heart_rate: Option<HeartRateEntropyCollector>,
    // ... more sensors
}

impl MultiModalEntropyCollector {
    pub fn collect_fused_entropy(
        &self,
        consent: &InformedConsent,
    ) -> Result<FusedEntropy, BearDogError> {
        let mut entropy_sources = Vec::new();

        // Collect from all available sensors
        if let Some(ref collector) = self.interaction {
            entropy_sources.push(collector.collect_live_interactions()?);
        }

        if let Some(ref collector) = self.voice {
            entropy_sources.push(collector.collect_voice_entropy(Duration::from_secs(5), consent)?);
        }

        // ... collect from other sensors

        // Fuse entropy using cryptographic mixing
        let fused = self.fuse_entropy_sources(entropy_sources)?;

        Ok(fused)
    }

    fn fuse_entropy_sources(&self, sources: Vec<EntropySource>) -> Result<FusedEntropy, BearDogError> {
        // Use SHA3-512 to mix multiple entropy sources
        use sha3::{Digest, Sha3_512};
        let mut hasher = Sha3_512::new();

        for source in sources {
            hasher.update(&source.entropy_bytes);
            hasher.update(&source.quality.to_le_bytes());
        }

        Ok(FusedEntropy {
            entropy_bytes: hasher.finalize().to_vec(),
            sources: sources.len(),
            overall_quality: sources.iter().map(|s| s.quality).sum::<f64>() / sources.len() as f64,
        })
    }
}
```

---

## 📊 Quality Scoring Guidelines

### **Minimum Quality Thresholds**

| Sensor Type | Min Quality | Rationale |
|-------------|-------------|-----------|
| Keyboard/Mouse | 0.7 | Natural human variation |
| Voice | 0.75 | Unique vocal characteristics |
| Fingerprint | 0.85 | High uniqueness, low variation |
| Heart Rate | 0.70 | Natural HRV patterns |
| DNA | 0.95 | Extremely unique, low noise |
| Retinal | 0.90 | Very unique vascular patterns |

### **Quality Calculation**

```rust
fn calculate_overall_quality(
    timing_entropy: f64,
    feature_entropy: f64,
    uniqueness_score: f64,
    live_confidence: f64,
) -> f64 {
    const TIMING_WEIGHT: f64 = 0.3;
    const FEATURE_WEIGHT: f64 = 0.3;
    const UNIQUENESS_WEIGHT: f64 = 0.2;
    const LIVE_WEIGHT: f64 = 0.2;

    timing_entropy * TIMING_WEIGHT
        + feature_entropy * FEATURE_WEIGHT
        + uniqueness_score * UNIQUENESS_WEIGHT
        + live_confidence * LIVE_WEIGHT
}
```

---

## 🚀 Roadmap

### **Phase 1** (✅ Complete)
- ✅ Keyboard and mouse interaction capture
- ✅ Terminal UI with real-time feedback
- ✅ LiveFeedValidator integration
- ✅ Quality metrics and visualization

### **Phase 2** (Q1 2026)
- ⏳ Voice/audio entropy collection
- ⏳ Camera-based entropy (face movement, eye tracking)
- ⏳ Haptic feedback entropy (touch pressure, timing)

### **Phase 3** (Q2 2026)
- ⏳ Wearable integration (Apple Watch, Fitbit, Garmin)
- ⏳ Heart rate variability (HRV) entropy
- ⏳ Accelerometer/gyroscope entropy

### **Phase 4** (Q3 2026)
- ⏳ Advanced biometrics (fingerprint, retinal)
- ⏳ DNA sequencing integration (portable sequencers)
- ⏳ Multi-modal fusion algorithms

### **Phase 5** (Q4 2026)
- ⏳ Environmental sensors (ambient noise, light, temperature)
- ⏳ IoT device integration
- ⏳ Custom sensor SDK for third-party developers

---

## 🎯 Design Principles

1. **Agnostic**: Support ANY sensor that can provide unique human data
2. **Modular**: Each sensor is independent, easy to add/remove
3. **Sovereign**: User owns their biometric data, NOT the system
4. **Privacy-First**: Extract entropy, NOT store raw biometrics
5. **Live-Only**: REFUSE simulated data, enforce with LiveFeedValidator
6. **Quality-Driven**: Measure and report entropy quality
7. **Consent-Based**: Require explicit informed consent
8. **Extensible**: Easy to add new sensors without breaking existing code

---

**🐻 BearDog: Integrity Over Features**  
*Your biometric data, your entropy, your sovereignty.*

**Status**: Foundation Complete ✅  
**Next**: Phase 2 Multi-Modal Sensors  
**Vision**: Infinite Extensibility

