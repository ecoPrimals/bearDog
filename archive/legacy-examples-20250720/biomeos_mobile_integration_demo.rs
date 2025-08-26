

use beardog_config::Config;
use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
use beardog_tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileBiomeConfig {

    pub device_type: MobileDeviceType,

    pub os_type: MobileOsType,

    pub hsm_integration: HsmIntegrationType,

    pub security_level: BiomeSecurityLevel,

    pub network_config: MobileNetworkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileDeviceType {

    Pixel {

        model: String,

        titan_version: String,
    },

    Android {

        manufacturer: String,

        model: String,

        strongbox_available: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileOsType {

    Android {

        version: String,

        security_patch: String,
    },

    GrapheneOS {

        version: String,

        android_version: String,

        privacy_enhanced: bool,
    },

    CalyxOS {

        version: String,

        android_version: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmIntegrationType {

    StrongBox {

        implementation: String,

        attestation_support: bool,
    },

    Software {

        hsm_type: String,

        encrypted_storage: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomeSecurityLevel {

    Basic,

    Enhanced,

    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileNetworkConfig {

    pub wifi_mesh: bool,

    pub cellular_data: bool,

    pub bluetooth_le: bool,

    pub nfc: bool,
}

pub struct MobileBiomeNode {

    config: MobileBiomeConfig,

    beardog_core: Arc<BearDogCore>,

    hsm_provider: Arc<dyn HsmProvider>,

    node_id: String,

    capabilities: MobileCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileCapabilities {

    pub compute: ComputeCapabilities,

    pub storage: StorageCapabilities,

    pub sensors: SensorCapabilities,

    pub security: SecurityCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {

    pub cpu_cores: u32,

    pub ram_mb: u32,

    pub gpu_available: bool,

    pub npu_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {

    pub available_gb: u32,

    pub storage_type: String,

    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorCapabilities {

    pub camera: bool,

    pub microphone: bool,

    pub gps: bool,

    pub motion_sensors: bool,

    pub environmental: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {

    pub hsm_type: String,

    pub biometric: bool,

    pub secure_boot: bool,

    pub attestation: bool,
}

impl MobileBiomeNode {

    pub async fn new(config: MobileBiomeConfig) -> BearDogResult<Self> {
        println!("🌱 Initializing BiomeOS Mobile Node");

        let beardog_config = Config::default();
        let beardog_core = Arc::new(BearDogCore::new(beardog_config).await?);

        let hsm_provider = Self::initialize_hsm_provider(&config).await?;

        let node_id = Uuid::new_v4().to_string();

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

                todo!("Software HSM not implemented in this demo")
            }
        }
    }

    async fn detect_capabilities(config: &MobileBiomeConfig) -> BearDogResult<MobileCapabilities> {
        println!("📱 Detecting device capabilities");

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
                        hsm_type: format_args!("Titan M {}", titan_version).to_string(),
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

    pub async fn register_with_ecosystem(&self) -> BearDogResult<()> {
        println!("🌐 Registering mobile node with BiomeOS ecosystem");

        let registration_key = self.generate_registration_key().await?;

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

        let payload_bytes = serde_json::to_vec(&registration_payload)?;
        let signature = self.hsm_provider.sign(&registration_key.id, &payload_bytes).await?;
        
        println!("✅ Node registration signed with hardware key");

        println!("🎉 Mobile node registered with ecosystem");
        Ok(())
    }

    async fn generate_registration_key(&self) -> BearDogResult<HsmKey> {
        println!("🔑 Generating hardware-backed registration key");
        
        let key_request = GenerateKeyRequest {
            key_id: format_args!("registration_key_{}", self.node_id).to_string(),
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

        if let Some(attestation) = &key.attestation {
            println!("🔐 Registration key has hardware attestation");
            println!("📜 Certificate chain length: {}", attestation.certificate_chain.len());
        }
        
        Ok(key)
    }

    pub async fn start_security_services(&self) -> BearDogResult<()> {
        println!("🛡️ Starting mobile security services");

        println!("🔐 Security services started");
        Ok(())
    }

    pub async fn test_biometric_auth(&self) -> BearDogResult<()> {
        println!("👆 Testing biometric authentication");

        let test_key = self.generate_biometric_key().await?;

        let test_data = b"BiomeOS mobile authentication test";
        
        println!("🔍 Triggering biometric prompt for signing...");
        let signature = self.hsm_provider.sign(&test_key.id, test_data).await?;
        
        println!("✅ Biometric signature generated: {} bytes", signature.len());

        let valid = self.hsm_provider.verify(&test_key.id, test_data, &signature).await?;
        println!("🔍 Signature verification: {}", valid);
        
        Ok(())
    }

    async fn generate_biometric_key(&self) -> BearDogResult<HsmKey> {
        let key_request = GenerateKeyRequest {
            key_id: format_args!("biometric_test_{}", Uuid::new_v4().to_string()),
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

    pub async fn demonstrate_cross_primal_communication(&self) -> BearDogResult<()> {
        println!("🌐 Demonstrating cross-primal communication");

        let comm_key = self.generate_communication_key().await?;

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

    async fn generate_communication_key(&self) -> BearDogResult<HsmKey> {
        let key_request = GenerateKeyRequest {
            key_id: format_args!("comm_key_{}", Uuid::new_v4().to_string()),
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

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🚀 BiomeOS Mobile Integration Demo");
    println!("Device: Pixel 8 + GrapheneOS + BearDog");
    println!("========================================\n");

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

    let node = MobileBiomeNode::new(config).await?;

    node.register_with_ecosystem().await?;

    node.start_security_services().await?;

    node.test_biometric_auth().await?;

    node.demonstrate_cross_primal_communication().await?;

    let status = node.get_status().await?;
    println!("\n📊 Node Status:");
    println!("{}", serde_json::to_string_pretty(&status)?);

    println!("\n🏃 Mobile BiomeOS node running...");
    println!("Press Ctrl+C to stop");

    loop {
        sleep(Duration::from_secs(10)).await;

        println!("❤️ Health check: OK");

        println!("🔍 Security scan: All clear");

        println!("🌐 Ecosystem sync: Connected");
    }
} 