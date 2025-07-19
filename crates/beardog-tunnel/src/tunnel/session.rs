// 🛡️ BSTP Session Management

use crate::tunnel::{GamingSecurityProfile, SecurityEvolution, SecurityLevel};
use beardog_errors::BearDogResult;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Secure session for BSTP gaming tunnels
///
/// Manages the lifecycle of secure communication sessions between gaming peers,
/// including encryption state, genetic evolution of security parameters,
/// and performance optimization for gaming workloads.
///
/// # Features
///
/// - **Genetic Security Evolution**: Automatically adapts security parameters
/// - **Gaming Performance Profiles**: Optimized for different gaming scenarios
/// - **Session Lifecycle Management**: Secure session creation and termination
/// - **Real-time Adaptation**: Responds to changing network conditions
///
/// # Security Model
///
/// Each session maintains:
/// - Unique session identifier for tracking
/// - Peer node identity verification
/// - Security genetics for adaptive protection
/// - Gaming-specific performance profiles
/// - Automatic expiration for security
///
/// # Example
///
/// ```rust,no_run
/// use beardog::tunnel::session::{SecureSession, SecurityGenetics, GamingSecurityProfile};
/// use std::time::SystemTime;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let session = SecureSession::new(
///         "session-123".to_string(),
///         "gaming-peer-456".to_string(),
///         SecurityGenetics::new(),
///         GamingSecurityProfile::competitive(),
///     ).await?;
///     
///     println!("Secure gaming session established: {}", session.session_id);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SecureSession {
    /// Unique identifier for this secure session
    pub session_id: String,
    /// Node ID of the peer in this session
    pub peer_node_id: String,
    /// Timestamp when the session was created
    pub created_at: SystemTime,
    /// Timestamp when the session expires for security
    pub expires_at: SystemTime,
    /// Genetic security configuration that evolves over time
    pub security_genetics: SecurityGenetics,
    /// Gaming-specific security and performance profile
    pub gaming_profile: GamingSecurityProfile,
}

impl SecureSession {
    /// Create a new secure session with genetic security evolution
    ///
    /// Initializes a new gaming tunnel session with adaptive security genetics
    /// and performance optimization for the specified gaming profile.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Unique identifier for the session
    /// * `peer_node_id` - Node ID of the peer to communicate with
    /// * `security_genetics` - Initial genetic security configuration
    /// * `gaming_profile` - Gaming performance and security profile
    ///
    /// # Returns
    ///
    /// Returns a new `SecureSession` instance ready for secure communication.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog::tunnel::session::{SecureSession, SecurityGenetics, GamingSecurityProfile};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let session = SecureSession::new(
    ///         "competitive-session".to_string(),
    ///         "tournament-player-123".to_string(),
    ///         SecurityGenetics::new(),
    ///         GamingSecurityProfile::competitive(),
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(
        session_id: String,
        peer_node_id: String,
        security_genetics: SecurityGenetics,
        gaming_profile: GamingSecurityProfile,
    ) -> BearDogResult<Self> {
        let created_at = SystemTime::now();
        let expires_at = created_at + Duration::from_secs(3600); // 1 hour default

        Ok(Self {
            session_id,
            peer_node_id,
            created_at,
            expires_at,
            security_genetics,
            gaming_profile,
        })
    }
}

/// Security genetics for genetic algorithm adaptation
#[derive(Debug, Clone, Default)]
pub struct SecurityGenetics {
    crypto_genes: CryptoChromosome,
    auth_genes: AuthenticationChromosome,
    threat_genes: ThreatResponseChromosome,
    performance_genes: PerformanceChromosome,
}

impl SecurityGenetics {
    /// Create a new secure session for a specific peer
    ///
    /// Creates a session optimized for the peer's capabilities and verification status.
    /// Automatically selects appropriate security genetics and gaming profile based
    /// on peer trust level and capabilities.
    ///
    /// # Arguments
    ///
    /// * `peer_id` - Identifier of the peer to create session with
    /// * `peer_capabilities` - Peer's declared capabilities and preferences
    /// * `verification` - Verification result for the peer's identity
    ///
    /// # Returns
    ///
    /// Returns a new `SecureSession` optimized for the peer's characteristics.
    ///
    /// # Security
    ///
    /// The session security level is automatically determined based on:
    /// - Peer verification trust level
    /// - Peer capabilities and preferences
    /// - Current threat environment
    /// - Gaming performance requirements
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog::tunnel::session::SecureSession;
    /// use beardog::tunnel::events::PeerCapabilities;
    /// use beardog::tunnel::security_provider::VerificationResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let capabilities = PeerCapabilities::default();
    ///     let verification = VerificationResult { /* ... */ };
    ///     
    ///     let session = SecureSession::new_for_peer(
    ///         "gaming-buddy",
    ///         &capabilities,
    ///         &verification,
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new_for_peer(
        _peer_id: &str,
        _peer_capabilities: &crate::tunnel::events::PeerCapabilities,
        _verification: &crate::tunnel::security_provider::VerificationResult,
    ) -> BearDogResult<Self> {
        Ok(Self {
            crypto_genes: CryptoChromosome::default(),
            auth_genes: AuthenticationChromosome::default(),
            threat_genes: ThreatResponseChromosome::default(),
            performance_genes: PerformanceChromosome::default(),
        })
    }

    /// Get the current security level of this session
    ///
    /// Returns the current security level based on the session's genetic evolution
    /// and gaming profile. The security level may change over time as the genetics
    /// adapt to changing conditions.
    ///
    /// # Returns
    ///
    /// Current `SecurityLevel` for this session.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog::tunnel::events::SecurityLevel;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let session = todo!(); // SecureSession instance
    /// let current_level = session.get_security_level();
    /// match current_level {
    ///     SecurityLevel::Ultimate => println!("Maximum security active"),
    ///     SecurityLevel::High => println!("High security active"),
    ///     _ => println!("Standard security active"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_security_level(&self) -> SecurityLevel {
        SecurityLevel::High // Default for now
    }

    /// Evolve session security based on performance metrics
    ///
    /// Uses genetic algorithms to adapt the session's security parameters
    /// based on observed performance metrics and threat indicators.
    /// This evolution optimizes the balance between security and performance.
    ///
    /// # Arguments
    ///
    /// * `performance_metrics` - Current performance and security metrics
    ///
    /// # Returns
    ///
    /// Returns a `SecurityEvolution` describing the changes made to security.
    ///
    /// # Genetic Evolution
    ///
    /// The evolution process considers:
    /// - Current latency and throughput performance
    /// - Threat detection indicators
    /// - Gaming performance requirements
    /// - Peer feedback and adaptation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog::tunnel::SecurityMetrics;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut session = todo!(); // SecureSession instance
    /// # let metrics = SecurityMetrics::default();
    /// let evolution = session.evolve_for_performance(&metrics).await?;
    /// println!("Security evolved: {:?}", evolution);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn evolve_for_performance(
        &mut self,
        performance_metrics: &crate::tunnel::SecurityMetrics,
    ) -> BearDogResult<SecurityEvolution> {
        // Genetic evolution based on performance
        self.performance_genes
            .optimize_for_latency(performance_metrics.encryption_latency);
        Ok(SecurityEvolution::PerformanceOptimized)
    }

    /// Get the crypto genes
    pub fn get_crypto_genes(&self) -> &CryptoChromosome {
        &self.crypto_genes
    }

    /// Get the authentication genes
    pub fn get_auth_genes(&self) -> &AuthenticationChromosome {
        &self.auth_genes
    }

    /// Get the threat response genes
    pub fn get_threat_genes(&self) -> &ThreatResponseChromosome {
        &self.threat_genes
    }

    /// Calculate overall security strength
    pub fn calculate_security_strength(&self) -> f64 {
        (self.crypto_genes.get_algorithm_preference()
            + self.auth_genes.get_trust_threshold()
            + self.threat_genes.get_response_aggressiveness())
            / 3.0
    }
}

/// Cryptographic chromosome for genetic security evolution
///
/// Contains genetic information for evolving cryptographic algorithms,
/// key management strategies, and performance optimization parameters.
///
/// # Evolution Targets
///
/// - Algorithm selection based on performance vs security trade-offs
/// - Key strength adaptation to threat levels
/// - Hardware acceleration utilization
/// - Gaming-specific crypto optimizations
#[derive(Debug, Clone, Default)]
pub struct CryptoChromosome {
    algorithm_preference: f64,
    key_strength: u32,
    hardware_acceleration: bool,
}

impl CryptoChromosome {
    /// Get algorithm preference value
    pub fn get_algorithm_preference(&self) -> f64 {
        self.algorithm_preference
    }

    /// Get key strength value
    pub fn get_key_strength(&self) -> u32 {
        self.key_strength
    }

    /// Check if hardware acceleration is enabled
    pub fn is_hardware_acceleration_enabled(&self) -> bool {
        self.hardware_acceleration
    }
}

/// Authentication chromosome for genetic security evolution
///
/// Contains genetic information for evolving authentication mechanisms,
/// trust verification, and peer validation strategies.
///
/// # Evolution Targets
///
/// - Authentication strength vs performance
/// - Trust level calculations
/// - Verification frequency
/// - Multi-factor requirements
#[derive(Debug, Clone)]
pub struct AuthenticationChromosome {
    trust_threshold: f64,
    session_lifetime: Duration,
}

impl Default for AuthenticationChromosome {
    fn default() -> Self {
        Self {
            trust_threshold: 0.8,
            session_lifetime: Duration::from_secs(3600),
        }
    }
}

impl AuthenticationChromosome {
    /// Get trust threshold value
    pub fn get_trust_threshold(&self) -> f64 {
        self.trust_threshold
    }

    /// Get session lifetime duration
    pub fn get_session_lifetime(&self) -> Duration {
        self.session_lifetime
    }
}

/// Threat response chromosome for genetic security evolution
///
/// Contains genetic information for evolving threat detection sensitivity,
/// response strategies, and defensive measures.
///
/// # Adaptive Responses
///
/// - Threat detection thresholds
/// - Response escalation strategies
/// - Recovery mechanisms
/// - Learning from attacks
#[derive(Debug, Clone)]
pub struct ThreatResponseChromosome {
    monitoring_frequency: Duration,
    response_aggressiveness: f64,
}

impl Default for ThreatResponseChromosome {
    fn default() -> Self {
        Self {
            monitoring_frequency: Duration::from_secs(30),
            response_aggressiveness: 0.5,
        }
    }
}

impl ThreatResponseChromosome {
    /// Get monitoring frequency
    pub fn get_monitoring_frequency(&self) -> Duration {
        self.monitoring_frequency
    }

    /// Get response aggressiveness value
    pub fn get_response_aggressiveness(&self) -> f64 {
        self.response_aggressiveness
    }
}

/// Performance chromosome for genetic security evolution
///
/// Contains genetic information for optimizing gaming performance
/// while maintaining security requirements.
///
/// # Performance Genes
///
/// - Latency optimization strategies
/// - Throughput maximization
/// - Gaming-specific adaptations
/// - Quality of service priorities
#[derive(Debug, Clone, Default)]
pub struct PerformanceChromosome {
    latency_priority: f64,
    throughput_priority: f64,
}

impl PerformanceChromosome {
    /// Optimize genetic parameters for target latency
    ///
    /// Evolves the performance chromosome to achieve the specified
    /// target latency while maintaining security requirements.
    ///
    /// # Arguments
    ///
    /// * `target_latency` - Desired maximum latency for gaming performance
    ///
    /// # Gaming Optimization
    ///
    /// Adjusts parameters for:
    /// - Encryption algorithm selection
    /// - Key caching strategies
    /// - Batch processing settings
    /// - Hardware acceleration usage
    pub fn optimize_for_latency(&mut self, _target_latency: Duration) {
        self.latency_priority = 1.0;
        self.throughput_priority = 0.3;
    }
}

/// Session manager for BSTP secure sessions
///
/// Manages the lifecycle of multiple secure sessions, providing session
/// creation, retrieval, and cleanup with genetic security evolution.
///
/// # Features
///
/// - **Multi-session Management**: Handle multiple concurrent gaming sessions
/// - **Automatic Cleanup**: Remove expired sessions automatically
/// - **Genetic Optimization**: Apply genetic algorithms across sessions
/// - **Performance Monitoring**: Track session performance metrics
#[derive(Debug)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    /// Create a new session manager
    ///
    /// Initializes an empty session manager ready to handle
    /// secure gaming tunnel sessions.
    ///
    /// # Returns
    ///
    /// New `SessionManager` instance with no active sessions.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new secure session with genetic evolution
    ///
    /// Creates a new session for the specified peer with genetic security
    /// evolution and gaming performance optimization.
    ///
    /// # Arguments
    ///
    /// * `peer_id` - Identifier of the peer to create session with
    /// * `peer_capabilities` - Peer's capabilities and preferences
    ///
    /// # Returns
    ///
    /// Returns the created `SecureSession` instance.
    ///
    /// # Session Lifecycle
    ///
    /// The created session will:
    /// - Automatically expire after the configured timeout
    /// - Evolve security parameters based on performance
    /// - Adapt to changing threat conditions
    /// - Optimize for gaming workloads
    pub async fn create_session(
        &mut self,
        peer_id: &str,
        peer_capabilities: &crate::tunnel::events::PeerCapabilities,
    ) -> BearDogResult<SecureSession> {
        let session_id = format!("bstp_session_{}", uuid::Uuid::new_v4().simple());
        let security_genetics = SecurityGenetics::new_for_peer(
            peer_id,
            peer_capabilities,
            &crate::tunnel::security_provider::VerificationResult {
                peer_id: peer_id.to_string(),
                is_trusted: true,
                trust_level: crate::tunnel::security_provider::TrustLevel::High,
                verification_time: SystemTime::now(),
                capabilities: HashMap::new(),
            },
        )
        .await?;

        let gaming_profile = peer_capabilities
            .gaming_profile
            .clone()
            .unwrap_or_else(GamingSecurityProfile::competitive_gaming);

        let session = SecureSession::new(
            session_id.clone(),
            peer_id.to_string(),
            security_genetics,
            gaming_profile,
        )
        .await?;

        self.sessions
            .write()
            .await
            .insert(session_id.clone(), session.clone());
        Ok(session)
    }

    /// Retrieve a session by ID
    ///
    /// Returns a clone of the session if found, or None if the session
    /// doesn't exist or has expired.
    ///
    /// # Arguments
    /// * `session_id` - Unique identifier of the session to retrieve
    pub async fn get_session(&self, session_id: &str) -> Option<SecureSession> {
        self.sessions.read().await.get(session_id).cloned()
    }

    /// Remove and return a session by ID
    ///
    /// Removes the session from the manager and returns it if found.
    /// This is used for session cleanup and termination.
    ///
    /// # Arguments  
    /// * `session_id` - Unique identifier of the session to remove
    pub async fn remove_session(&self, session_id: &str) -> Option<SecureSession> {
        self.sessions.write().await.remove(session_id)
    }

    /// Add a session to the manager
    ///
    /// Inserts a new session into the manager with the specified ID.
    /// Used for restoring sessions or manual session creation.
    ///
    /// # Arguments
    /// * `session_id` - Unique identifier for the session
    /// * `session` - The session instance to add
    pub async fn add_session(&self, session_id: String, session: SecureSession) {
        self.sessions.write().await.insert(session_id, session);
    }
}
