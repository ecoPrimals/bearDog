// Core types for security provider bridge functionality

#[derive(Debug, Clone)]
    /// The payload value
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


    pub payload: &'a [u8],
    /// The signature value
    pub signature: &'a [u8],
    /// The algorithm value
    pub algorithm: &'a str,
}

#[derive(Debug, Clone)]
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The last accessed value
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
    pub user_id: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Collection of permissions
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
    },
    Sign {
        key_handle: VendorKeyHandle,
        data: Vec<u8>,
    },
    Verify {
        key_handle: VendorKeyHandle,
        data: Vec<u8>,
        signature: Vec<u8>,
    },
    GetCapabilities,
    HealthCheck,
}

impl VendorOperation {
    /// Operation Type operation.
    pub fn operation_type(&self) -> &'static str {
        match self {
            VendorOperation::GenerateKey { .. } => "generate_key",
            VendorOperation::Sign { .. } => "sign",
            VendorOperation::Verify { .. } => "verify",
            VendorOperation::GetCapabilities => "get_capabilities",
            VendorOperation::HealthCheck => "health_check",
        }
    }
}

pub enum VendorOperationResult {
    KeyGenerated { key_handle: VendorKeyHandle },
    KeyGenerated { key_handle: VendorKeyHandle },
    KeyGenerated { key_handle: VendorKeyHandle },
    DataSigned { signature: Vec<u8> },
    SignatureVerified { valid: bool },
    Capabilities { capabilities: VendorCapabilities },
    HealthStatus { health: VendorHealthStatus },
}

// Define the types locally since they're not available in beardog-types yet
#[derive(Debug, Clone)]
    /// The algorithm value
    pub algorithm: String,
    /// Number of key_size
    pub key_size: u32,
    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
    /// Number of key_size
    pub key_size: u32,
}

#[derive(Debug, Clone)]
    /// Number of max_key_size
    pub max_key_size: u32,
    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,
    /// Whether fips_certified is enabled
    pub fips_certified: bool,
    /// Name of the vendor
    pub vendor_name: String,
    /// The firmware version value
    pub firmware_version: String,
}

#[derive(Debug, Clone)]
pub enum VendorHealthStatus {
    /// Represents healthy variant
    Healthy,
    /// Represents unhealthy variant
    Unhealthy,
    /// State indicating degraded
    Degraded,
    /// Unknown or undefined state
    Unknown,
}
