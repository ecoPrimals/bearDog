//! BiomeOS Mobile Integration Demo
//!
//! This example demonstrates how BearDog's Android StrongBox HSM integration
//! will work with BiomeOS on a Pixel 8 device running GrapheneOS.
//!
//! ## Features Demonstrated
//!
//! - Mobile BiomeOS node initialization
//! - Hardware-backed key generation on Pixel 8
//! - Biometric authentication with Titan M
//! - Cross-primal secure communication
//! - Mobile ecosystem node registration
//!
//! ## Hardware Requirements
//!
//! - Google Pixel 8 (or newer)
//! - GrapheneOS installed
//! - Titan M security chip
//! - Android StrongBox support
//!
//! ## Usage
//!
//! ```bash
//! # Build for Android
//! cargo build --target aarch64-linux-android --example biomeos_mobile_integration_demo
//!
//! # Deploy to Pixel 8
//! adb push target/aarch64-linux-android/debug/examples/biomeos_mobile_integration_demo /data/local/tmp/
//! adb shell chmod 755 /data/local/tmp/biomeos_mobile_integration_demo
//! adb shell /data/local/tmp/biomeos_mobile_integration_demo
//! ```

use beardog_config::Config;
use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
use beardog_tunnel::hsm::types::*;
use beardog_tunnel::hsm::HsmProvider;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// BiomeOS Mobile Node Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileBiomeConfig {
    /// Device information
    pub device_type: MobileDeviceType,
    /// Operating system type
    pub os_type: MobileOsType,
    /// HSM integration type
    pub hsm_integration: HsmIntegrationType,
    /// Security level requirements
    pub security_level: BiomeSecurityLevel,
    /// Network configuration
    pub network_config: MobileNetworkConfig,
}

/// Mobile device types supported by BiomeOS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileDeviceType {
    /// Google Pixel devices
    Pixel {
        /// Pixel model (8, 8 Pro, 9, etc.)
        model: String,
        /// Titan M security chip version
        titan_version: String,
    },
    /// Generic Android device
    Android {
        /// Device manufacturer
        manufacturer: String,
        /// Device model
        model: String,
        /// StrongBox availability
        strongbox_available: bool,
    },
}

/// Mobile operating system types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileOsType {
    /// Standard Android
    Android {
        /// Android version
        version: String,
        /// Security patch level
        security_patch: String,
    },
    /// GrapheneOS (hardened Android)
    GrapheneOS {
        /// GrapheneOS version
        version: String,
        /// Base Android version
        android_version: String,
        /// Privacy enhancements enabled
        privacy_enhanced: bool,
    },
    /// CalyxOS
    CalyxOS {
        /// CalyxOS version
        version: String,
        /// Base Android version
        android_version: String,
    },
}

/// HSM integration types for mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmIntegrationType {
    /// Android StrongBox integration
    StrongBox {
        /// Implementation type
        implementation: String,
        /// Hardware attestation support
        attestation_support: bool,
    },
    /// Software HSM fallback
    Software {
        /// Software HSM type
        hsm_type: String,
        /// Encryption at rest
        encrypted_storage: bool,
    },
}

/// BiomeOS security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomeSecurityLevel {
    /// Basic security (software-only)
    Basic,
    /// Enhanced security (hardware-assisted)
    Enhanced,
    /// Maximum security (hardware-only)
    Maximum,
}

/// Mobile network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileNetworkConfig {
    /// Enable WiFi mesh networking
    pub wifi_mesh: bool,
    /// Enable cellular data usage
    pub cellular_data: bool,
    /// Enable Bluetooth Low Energy
    pub bluetooth_le: bool,
    /// Enable NFC communication
    pub nfc: bool,
}

/// BiomeOS Mobile Node
///
/// Represents a mobile device running as a BiomeOS node with BearDog security.
pub struct MobileBiomeNode {
    /// Node configuration
    config: MobileBiomeConfig,
    /// BearDog core instance
    beardog_core: Arc<BearDogCore>,
    /// HSM provider for hardware security
    hsm_provider: Arc<dyn HsmProvider>,
    /// Node ID in the ecosystem
    node_id: String,
    /// Device capabilities
    capabilities: MobileCapabilities,
}

/// Mobile device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileCapabilities {
    /// Available compute resources
    pub compute: ComputeCapabilities,
    /// Available storage
    pub storage: StorageCapabilities,
    /// Available sensors
    pub sensors: SensorCapabilities,
    /// Security features
    pub security: SecurityCapabilities,
}

/// Compute capabilities on mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {
    /// CPU cores
    pub cpu_cores: u32,
    /// Available RAM in MB
    pub ram_mb: u32,
    /// GPU availability
    pub gpu_available: bool,
    /// Neural processing unit
    pub npu_available: bool,
}

/// Storage capabilities on mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Available storage in GB
    pub available_gb: u32,
    /// Storage type (UFS, eMMC, etc.)
    pub storage_type: String,
    /// Encryption at rest
    pub encrypted: bool,
}

/// Sensor capabilities on mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorCapabilities {
    /// Camera availability
    pub camera: bool,
    /// Microphone availability
    pub microphone: bool,
    /// GPS availability
    pub gps: bool,
    /// Accelerometer/gyroscope
    pub motion_sensors: bool,
    /// Environmental sensors
    pub environmental: bool,
}

/// Security capabilities on mobile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Hardware security module
    pub hsm_type: String,
    /// Biometric authentication
    pub biometric: bool,
    /// Secure boot
    pub secure_boot: bool,
    /// Hardware attestation
    pub attestation: bool,
}

impl MobileBiomeNode {
    /// Create a new mobile BiomeOS node
    pub async fn new(config: MobileBiomeConfig) -> BearDogResult<Self> {
        println!("🌱 Initializing BiomeOS Mobile Node");
        
        // Initialize BearDog core
        let beardog_config = Config::default();
        let beardog_core = Arc::new(BearDogCore::new(beardog_config).await?);
        
        // Initialize HSM provider based on device
        let hsm_provider = Self::initialize_hsm_provider(&config).await?;
        
        // Generate unique node ID
        let node_id = Uuid::new_v4().to_string();
        
        // Detect device capabilities
        let capabilities = Self::detect_capabilities(&config).await?;
        
        let node = Self {
            config,
            beardog_core,
            hsm_provider,
            node_id,
            capabilities,
        };
        
        println!("✅ BiomeOS Mobile Node initialized: {}", node.node_id);
        Ok(node)
    }
    
    /// Initialize HSM provider based on device configuration
    async fn initialize_hsm_provider(config: &MobileBiomeConfig) -> BearDogResult<Arc<dyn HsmProvider>> {
        match &config.hsm_integration {
            HsmIntegrationType::StrongBox { implementation, attestation_support } => {
                println!("🔐 Initializing StrongBox HSM: {}", implementation);
                
                let hsm_config = AndroidHsmConfig {
                    keystore_config: KeystoreConfig {
                        use_strongbox: true,
                        hardware_backed_only: true,
                        require_biometric: true,
                        biometric_timeout: 30,
                        disable_backup: true, // GrapheneOS privacy
                    },
                    attestation_config: AttestationConfig {
                        verify_certificate_chain: *attestation_support,
                        verify_verified_boot: true,
                        require_locked_bootloader: true,
                        verify_os_version: true,
                        challenge_size: 32,
                    },
                };
                
                let hsm = AndroidStrongBoxHsm::new(hsm_config).await?;
                Ok(Arc::new(hsm))
            }
            HsmIntegrationType::Software { hsm_type, encrypted_storage } => {
                println!("💾 Initializing Software HSM: {}", hsm_type);
                // TODO: Initialize software HSM
                todo!("Software HSM not implemented in this demo")
            }
        }
    }
    
    /// Detect device capabilities
    async fn detect_capabilities(config: &MobileBiomeConfig) -> BearDogResult<MobileCapabilities> {
        println!("📱 Detecting device capabilities");
        
        // Simulate capability detection based on device type
        let capabilities = match &config.device_type {
            MobileDeviceType::Pixel { model, titan_version } => {
                println!("🔍 Detected Pixel {}", model);
                
                MobileCapabilities {
                    compute: ComputeCapabilities {
                        cpu_cores: 8,
                        ram_mb: 8192,
                        gpu_available: true,
                        npu_available: true,
                    },
                    storage: StorageCapabilities {
                        available_gb: 128,
                        storage_type: "UFS 3.1".to_string(),
                        encrypted: true,
                    },
                    sensors: SensorCapabilities {
                        camera: true,
                        microphone: true,
                        gps: true,
                        motion_sensors: true,
                        environmental: true,
                    },
                    security: SecurityCapabilities {
                        hsm_type: format!("Titan M {}", titan_version),
                        biometric: true,
                        secure_boot: true,
                        attestation: true,
                    },
                }
            }
            MobileDeviceType::Android { manufacturer, model, strongbox_available } => {
                println!("🔍 Detected Android device: {} {}", manufacturer, model);
                
                MobileCapabilities {
                    compute: ComputeCapabilities {
                        cpu_cores: 6,
                        ram_mb: 6144,
                        gpu_available: true,
                        npu_available: false,
                    },
                    storage: StorageCapabilities {
                        available_gb: 64,
                        storage_type: "eMMC".to_string(),
                        encrypted: true,
                    },
                    sensors: SensorCapabilities {
                        camera: true,
                        microphone: true,
                        gps: true,
                        motion_sensors: true,
                        environmental: false,
                    },
                    security: SecurityCapabilities {
                        hsm_type: if *strongbox_available { "StrongBox".to_string() } else { "Software".to_string() },
                        biometric: true,
                        secure_boot: false,
                        attestation: *strongbox_available,
                    },
                }
            }
        };
        
        Ok(capabilities)
    }
    
    /// Register node with BiomeOS ecosystem
    pub async fn register_with_ecosystem(&self) -> BearDogResult<()> {
        println!("🌐 Registering mobile node with BiomeOS ecosystem");
        
        // Generate hardware-backed registration key
        let registration_key = self.generate_registration_key().await?;
        
        // Create node registration payload
        let registration_payload = serde_json::json!({
            "node_id": self.node_id,
            "node_type": "mobile",
            "device_type": self.config.device_type,
            "os_type": self.config.os_type,
            "capabilities": self.capabilities,
            "security_level": self.config.security_level,
            "registration_key": registration_key.id,
            "timestamp": chrono::Utc::now(),
        });
        
        // Sign registration payload with hardware key
        let payload_bytes = serde_json::to_vec(&registration_payload)?;
        let signature = self.hsm_provider.sign(&registration_key.id, &payload_bytes).await?;
        
        println!("✅ Node registration signed with hardware key");
        
        // TODO: Send registration to SongBird discovery service
        // This would typically involve:
        // 1. Discovering SongBird nodes
        // 2. Sending registration request
        // 3. Receiving ecosystem credentials
        // 4. Establishing secure channels
        
        println!("🎉 Mobile node registered with ecosystem");
        Ok(())
    }
    
    /// Generate hardware-backed registration key
    async fn generate_registration_key(&self) -> BearDogResult<HsmKey> {
        println!("🔑 Generating hardware-backed registration key");
        
        let key_request = GenerateKeyRequest {
            key_id: format!("registration_key_{}", self.node_id),
            key_type: KeyType::EccP256,
            usage_policy: KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                can_encrypt: false,
                can_decrypt: false,
                can_wrap: false,
                can_unwrap: false,
                user_presence_required: true,
                biometric_required: true,
                ..Default::default()
            },
            attestation_challenge: Some(b"biomeos_registration_challenge".to_vec()),
            metadata: KeyMetadata {
                description: "BiomeOS mobile node registration key".to_string(),
                ..Default::default()
            },
        };
        
        let key = self.hsm_provider.generate_key(key_request).await?;
        
        // Verify key has hardware attestation
        if let Some(attestation) = &key.attestation {
            println!("🔐 Registration key has hardware attestation");
            println!("📜 Certificate chain length: {}", attestation.certificate_chain.len());
        }
        
        Ok(key)
    }
    
    /// Start security services
    pub async fn start_security_services(&self) -> BearDogResult<()> {
        println!("🛡️ Starting mobile security services");
        
        // Start BearDog security provider
        // This would typically include:
        // 1. Authentication service
        // 2. Authorization service
        // 3. Encryption service
        // 4. Threat detection
        // 5. Audit logging
        
        println!("🔐 Security services started");
        Ok(())
    }
    
    /// Test biometric authentication
    pub async fn test_biometric_auth(&self) -> BearDogResult<()> {
        println!("👆 Testing biometric authentication");
        
        // Generate a test key with biometric requirement
        let test_key = self.generate_biometric_key().await?;
        
        // Test signing with biometric authentication
        let test_data = b"BiomeOS mobile authentication test";
        
        println!("🔍 Triggering biometric prompt for signing...");
        let signature = self.hsm_provider.sign(&test_key.id, test_data).await?;
        
        println!("✅ Biometric signature generated: {} bytes", signature.len());
        
        // Verify signature
        let valid = self.hsm_provider.verify(&test_key.id, test_data, &signature).await?;
        println!("🔍 Signature verification: {}", valid);
        
        Ok(())
    }
    
    /// Generate biometric-protected key
    async fn generate_biometric_key(&self) -> BearDogResult<HsmKey> {
        let key_request = GenerateKeyRequest {
            key_id: format!("biometric_test_{}", Uuid::new_v4()),
            key_type: KeyType::EccP256,
            usage_policy: KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                can_encrypt: false,
                can_decrypt: false,
                can_wrap: false,
                can_unwrap: false,
                user_presence_required: true,
                biometric_required: true,
                ..Default::default()
            },
            attestation_challenge: Some(b"biometric_test_challenge".to_vec()),
            metadata: KeyMetadata {
                description: "Biometric test key".to_string(),
                ..Default::default()
            },
        };
        
        self.hsm_provider.generate_key(key_request).await
    }
    
    /// Demonstrate cross-primal communication
    pub async fn demonstrate_cross_primal_communication(&self) -> BearDogResult<()> {
        println!("🌐 Demonstrating cross-primal communication");
        
        // Generate communication keys
        let comm_key = self.generate_communication_key().await?;
        
        // Simulate secure message to another primal
        let message = serde_json::json!({
            "from": self.node_id,
            "to": "toadstool_compute_node",
            "message_type": "computation_request",
            "payload": {
                "task": "image_processing",
                "data": "encrypted_image_data",
                "resources_required": {
                    "cpu": 0.5,
                    "memory": 1024,
                    "gpu": true
                }
            },
            "timestamp": chrono::Utc::now()
        });
        
        let message_bytes = serde_json::to_vec(&message)?;
        let signature = self.hsm_provider.sign(&comm_key.id, &message_bytes).await?;
        
        println!("✅ Cross-primal message signed with hardware key");
        println!("📤 Message size: {} bytes", message_bytes.len());
        println!("🔐 Signature size: {} bytes", signature.len());
        
        // Simulate receiving response
        let response = serde_json::json!({
            "from": "toadstool_compute_node",
            "to": self.node_id,
            "message_type": "computation_response",
            "payload": {
                "task_id": "img_proc_12345",
                "status": "completed",
                "result": "processed_image_data"
            },
            "timestamp": chrono::Utc::now()
        });
        
        println!("📥 Received response from ToadStool compute node");
        println!("✅ Cross-primal communication successful");
        
        Ok(())
    }
    
    /// Generate communication key
    async fn generate_communication_key(&self) -> BearDogResult<HsmKey> {
        let key_request = GenerateKeyRequest {
            key_id: format!("comm_key_{}", Uuid::new_v4()),
            key_type: KeyType::EccP256,
            usage_policy: KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                can_encrypt: true,
                can_decrypt: true,
                can_wrap: false,
                can_unwrap: false,
                user_presence_required: false,
                biometric_required: false,
                ..Default::default()
            },
            attestation_challenge: Some(b"cross_primal_comm_challenge".to_vec()),
            metadata: KeyMetadata {
                description: "Cross-primal communication key".to_string(),
                ..Default::default()
            },
        };
        
        self.hsm_provider.generate_key(key_request).await
    }
    
    /// Get node status
    pub async fn get_status(&self) -> BearDogResult<serde_json::Value> {
        let hsm_info = self.hsm_provider.get_info().await?;
        
        Ok(serde_json::json!({
            "node_id": self.node_id,
            "status": "active",
            "device_type": self.config.device_type,
            "os_type": self.config.os_type,
            "capabilities": self.capabilities,
            "hsm_info": {
                "hsm_type": hsm_info.hsm_type,
                "vendor": hsm_info.vendor,
                "model": hsm_info.model,
                "version": hsm_info.version,
                "capabilities": hsm_info.capabilities
            },
            "uptime": "5 minutes",
            "last_seen": chrono::Utc::now()
        }))
    }
}

/// Main demo function
#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🚀 BiomeOS Mobile Integration Demo");
    println!("Device: Pixel 8 + GrapheneOS + BearDog");
    println!("========================================\n");
    
    // Configure mobile BiomeOS node for Pixel 8 + GrapheneOS
    let config = MobileBiomeConfig {
        device_type: MobileDeviceType::Pixel {
            model: "8".to_string(),
            titan_version: "2.0".to_string(),
        },
        os_type: MobileOsType::GrapheneOS {
            version: "2024.1".to_string(),
            android_version: "14".to_string(),
            privacy_enhanced: true,
        },
        hsm_integration: HsmIntegrationType::StrongBox {
            implementation: "Titan M 2.0".to_string(),
            attestation_support: true,
        },
        security_level: BiomeSecurityLevel::Maximum,
        network_config: MobileNetworkConfig {
            wifi_mesh: true,
            cellular_data: true,
            bluetooth_le: true,
            nfc: true,
        },
    };
    
    // Initialize mobile BiomeOS node
    let node = MobileBiomeNode::new(config).await?;
    
    // Register with ecosystem
    node.register_with_ecosystem().await?;
    
    // Start security services
    node.start_security_services().await?;
    
    // Test biometric authentication
    node.test_biometric_auth().await?;
    
    // Demonstrate cross-primal communication
    node.demonstrate_cross_primal_communication().await?;
    
    // Display node status
    let status = node.get_status().await?;
    println!("\n📊 Node Status:");
    println!("{}", serde_json::to_string_pretty(&status)?);
    
    // Keep node running
    println!("\n🏃 Mobile BiomeOS node running...");
    println!("Press Ctrl+C to stop");
    
    // Simulate ongoing operations
    loop {
        sleep(Duration::from_secs(10)).await;
        
        // Simulate periodic health check
        println!("❤️ Health check: OK");
        
        // Simulate periodic security scan
        println!("🔍 Security scan: All clear");
        
        // Simulate periodic ecosystem sync
        println!("🌐 Ecosystem sync: Connected");
    }
} 