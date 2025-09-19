// Network Domain Constants
//
// This module provides network-related constants consolidated from the large unified.rs file.
// It includes connection settings, timeouts, protocols, and network performance tuning.

use std::time::Duration;

/// Network configuration constants and environment-aware defaults
pub mod config {
    /// IPv4 localhost address
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";
    /// IPv6 localhost address  
    pub const LOCALHOST_IPV6: &str = "::1";
    /// Standard localhost name
    pub const LOCALHOST_NAME: &str = "localhost";

    /// Default HTTP port
    pub const DEFAULT_HTTP_PORT: u16 = 8080;
    /// Default HTTPS port
    pub const DEFAULT_HTTPS_PORT: u16 = 8443;
    /// Default `PostgreSQL` port
    pub const DEFAULT_POSTGRES_PORT: u16 = 5432;
    /// Default Grafana port  
    pub const DEFAULT_GRAFANA_PORT: u16 = 3000;

    /// Default API bind address (environment configurable)
    pub const DEFAULT_API_BIND: &str = "0.0.0.0:8080";

    /// Get the default service host from environment or fallback
    pub fn default_service_host() -> String {
        std::env::var("BEARDOG_SERVICE_HOST")
            .or_else(|_| std::env::var("BEARDOG_HOST"))
            .unwrap_or_else(|_| LOCALHOST_NAME.to_string())
    }

    /// Get the default service port from environment or fallback
    pub fn default_service_port() -> u16 {
        std::env::var("BEARDOG_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_HTTP_PORT)
    }

    /// Get the default database URL from environment or fallback
    pub fn default_database_url() -> String {
        std::env::var("DATABASE_URL")
            .or_else(|_| std::env::var("BEARDOG_DATABASE_URL"))
            .unwrap_or_else(|_| {
                format!(
                    "postgresql://{}:{}/beardog",
                    default_service_host(),
                    DEFAULT_POSTGRES_PORT
                )
            })
    }

    /// Get the default discovery endpoint from environment or fallback
    pub fn default_discovery_endpoint() -> String {
        std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
            .or_else(|_| std::env::var("DISCOVERY_URL"))
            .unwrap_or_else(|_| {
                format!(
                    "http://{}:{}/discovery",
                    default_service_host(),
                    default_service_port()
                )
            })
    }

    /// Get the default compute endpoint from environment or fallback
    pub fn default_compute_endpoint() -> String {
        std::env::var("BEARDOG_COMPUTE_ENDPOINT").unwrap_or_else(|_| {
            format!(
                "http://{}:{}/compute",
                default_service_host(),
                default_service_port()
            )
        })
    }

    /// Get the default storage endpoint from environment or fallback  
    pub fn default_storage_endpoint() -> String {
        std::env::var("BEARDOG_STORAGE_ENDPOINT").unwrap_or_else(|_| {
            format!(
                "http://{}:{}/storage",
                default_service_host(),
                default_service_port()
            )
        })
    }
}

/// **DEFAULT NETWORK SETTINGS** - Standard network configuration
pub mod defaults {
    use super::Duration;

    /// Connection defaults
    pub const DEFAULT_API_PORT: u16 = 8080;
    pub const DEFAULT_METRICS_PORT: u16 = 9090;
    pub const DEFAULT_HEALTH_PORT: u16 = 8081;
    pub const DEFAULT_ADMIN_PORT: u16 = 8082;
    pub const DEFAULT_DEBUG_PORT: u16 = 8083;

    /// Timeout defaults
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(60);
    pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);
    pub const DEFAULT_KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(60);

    /// Buffer sizes
    /// Default socket buffer size in bytes (64KB)
    pub const DEFAULT_SOCKET_BUFFER_SIZE: usize = 65536;
    /// Default send buffer size in bytes (32KB)
    pub const DEFAULT_SEND_BUFFER_SIZE: usize = 32768;
    /// Default receive buffer size in bytes (32KB)
    pub const DEFAULT_RECEIVE_BUFFER_SIZE: usize = 32768;
    pub const DEFAULT_BACKLOG_SIZE: u32 = 128;

    /// Connection pool defaults
    /// Default maximum number of concurrent connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;
    /// Default minimum number of connections to maintain
    pub const DEFAULT_MIN_CONNECTIONS: usize = 1;
    /// Default size of the connection pool
    pub const DEFAULT_CONNECTION_POOL_SIZE: usize = 10;
    /// Default maximum number of idle connections to keep
    pub const DEFAULT_MAX_IDLE_CONNECTIONS: usize = 5;

    /// Protocol defaults
    /// Default HTTP protocol version
    pub const DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";
    pub const DEFAULT_TLS_VERSION: &str = "TLSv1.3";
    /// Default `BearDog` protocol version
    pub const DEFAULT_PROTOCOL_VERSION: &str = "2.0";
}

/// **NETWORK ADDRESSES** - Standard network addresses and endpoints
pub mod addresses {
    /// Localhost addresses
    /// IPv4 localhost address
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";
    /// IPv6 localhost address
    pub const LOCALHOST_IPV6: &str = "::1";
    /// IPv4 wildcard address (bind to all interfaces)
    pub const WILDCARD_IPV4: &str = "0.0.0.0";
    /// IPv6 wildcard address (bind to all interfaces)
    pub const WILDCARD_IPV6: &str = "::";

    /// Default bind addresses
    /// Default address to bind services to (all interfaces)
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
    pub const DEFAULT_API_BIND: &str = "0.0.0.0:8080";
    pub const DEFAULT_METRICS_BIND: &str = "0.0.0.0:9090";
    pub const DEFAULT_HEALTH_BIND: &str = "0.0.0.0:8081";

    /// Service discovery addresses
    pub const MULTICAST_ADDRESS: &str = "224.0.0.251";
    /// Configuration constant: broadcast address
    pub const BROADCAST_ADDRESS: &str = "255.255.255.255";

    /// DNS settings
    pub const DEFAULT_DNS_PORT: u16 = 53;
    /// Configuration constant: default dns servers
    pub const DEFAULT_DNS_SERVERS: &[&str] = &["8.8.8.8", "8.8.4.4", "1.1.1.1"];
}

pub mod ports {
    /// Well-known ports (0-1023)
    pub const WELL_KNOWN_PORT_MIN: u16 = 1;
    /// Configuration constant: well known port max
    pub const WELL_KNOWN_PORT_MAX: u16 = 1023;

    /// Registered ports (1024-49151)
    pub const REGISTERED_PORT_MIN: u16 = 1024;
    /// Configuration constant: registered port max
    pub const REGISTERED_PORT_MAX: u16 = 49151;

    /// Dynamic/Private ports (49152-65535)
    pub const DYNAMIC_PORT_MIN: u16 = 49152;
    /// Configuration constant: dynamic port max
    pub const DYNAMIC_PORT_MAX: u16 = 65535;

    /// `BearDog` service port ranges
    pub const BEARDOG_PORT_RANGE_START: u16 = 8080;
    /// Configuration constant: beardog port range end
    pub const BEARDOG_PORT_RANGE_END: u16 = 8099;

    /// Standard service ports
    pub const HTTP_PORT: u16 = 80;
    /// Configuration constant: https port
    pub const HTTPS_PORT: u16 = 443;
    /// Configuration constant: ssh port
    pub const SSH_PORT: u16 = 22;
    /// Configuration constant: ftp port
    pub const FTP_PORT: u16 = 21;
    /// Configuration constant: smtp port
    pub const SMTP_PORT: u16 = 25;
    /// Configuration constant: dns port
    pub const DNS_PORT: u16 = 53;
    /// Configuration constant: dhcp server port
    pub const DHCP_SERVER_PORT: u16 = 67;
    /// Configuration constant: dhcp client port
    pub const DHCP_CLIENT_PORT: u16 = 68;
    /// Configuration constant: snmp port
    pub const SNMP_PORT: u16 = 161;
    /// Configuration constant: syslog port
    pub const SYSLOG_PORT: u16 = 514;
}

/// **NETWORK TIMEOUTS** - Various network timeout configurations
pub mod timeouts {
    use super::Duration;

    /// Connection timeouts
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: handshake timeout
    pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
    /// Configuration constant: tls handshake timeout
    pub const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: keep alive timeout
    pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: idle connection timeout
    pub const IDLE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(300);

    /// I/O timeouts
    pub const READ_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: write timeout
    pub const WRITE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: send timeout
    pub const SEND_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: receive timeout
    pub const RECEIVE_TIMEOUT: Duration = Duration::from_secs(60);

    /// Request/Response timeouts
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: response timeout
    pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: http request timeout
    pub const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: http response timeout
    pub const HTTP_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

    /// DNS and resolution timeouts
    pub const DNS_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(5);
    /// Configuration constant: hostname resolution timeout
    pub const HOSTNAME_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(10);

    /// Retry and backoff timeouts
    pub const RETRY_TIMEOUT: Duration = Duration::from_millis(100);
    /// Configuration constant: max retry timeout
    pub const MAX_RETRY_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: backoff timeout
    pub const BACKOFF_TIMEOUT: Duration = Duration::from_millis(500);

    /// Health check timeouts
    pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
    /// Configuration constant: ping timeout
    pub const PING_TIMEOUT: Duration = Duration::from_secs(1);
    /// Configuration constant: heartbeat timeout
    pub const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
}

/// **NETWORK LIMITS** - Network-related limits and constraints
pub mod limits {
    /// Connection limits
    pub const MAX_CONNECTIONS: usize = 10000;
    /// Configuration constant: min connections
    pub const MIN_CONNECTIONS: usize = 1;
    /// Configuration constant: max connections per ip
    pub const MAX_CONNECTIONS_PER_IP: usize = 100;
    /// Configuration constant: max concurrent connections
    pub const MAX_CONCURRENT_CONNECTIONS: usize = 1000;

    /// Buffer and message limits
    pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024; // 16MB
    /// Configuration constant: min message size
    pub const MIN_MESSAGE_SIZE: usize = 1;
    /// Configuration constant: max header size
    pub const MAX_HEADER_SIZE: usize = 8192;
    /// Configuration constant: max url length
    pub const MAX_URL_LENGTH: usize = 2048;
    /// Configuration constant: max query string length
    pub const MAX_QUERY_STRING_LENGTH: usize = 4096;

    /// Request limits
    pub const MAX_REQUEST_SIZE: usize = 100 * 1024 * 1024; // 100MB
    /// Configuration constant: max response size
    pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024; // 100MB
    /// Configuration constant: max upload size
    pub const MAX_UPLOAD_SIZE: usize = 1024 * 1024 * 1024; // 1GB
    /// Configuration constant: max download size
    pub const MAX_DOWNLOAD_SIZE: usize = 1024 * 1024 * 1024; // 1GB

    /// Rate limiting
    pub const MAX_REQUESTS_PER_SECOND: u32 = 1000;
    /// Configuration constant: max requests per minute
    pub const MAX_REQUESTS_PER_MINUTE: u32 = 60000;
    /// Configuration constant: max requests per hour
    pub const MAX_REQUESTS_PER_HOUR: u32 = 3_600_000;
    /// Configuration constant: default rate limit
    pub const DEFAULT_RATE_LIMIT: u32 = 100;

    /// Bandwidth limits
    pub const MAX_BANDWIDTH_BYTES_PER_SEC: u64 = 100 * 1024 * 1024; // 100MB/s
    /// Configuration constant: default bandwidth limit
    pub const DEFAULT_BANDWIDTH_LIMIT: u64 = 10 * 1024 * 1024; // 10MB/s

    /// Connection pool limits
    pub const MAX_CONNECTION_POOL_SIZE: usize = 100;
    /// Configuration constant: min connection pool size
    pub const MIN_CONNECTION_POOL_SIZE: usize = 1;
    /// Configuration constant: max idle connections
    pub const MAX_IDLE_CONNECTIONS: usize = 10;
    /// Configuration constant: min idle connections
    pub const MIN_IDLE_CONNECTIONS: usize = 0;
}

/// **PROTOCOL CONSTANTS** - Protocol-specific constants
pub mod protocols {
    /// HTTP constants
    pub mod http {
        /// HTTP version 1.0 protocol identifier
        pub const HTTP_1_0: &str = "HTTP/1.0";
        /// HTTP version 1.1 protocol identifier
        pub const HTTP_1_1: &str = "HTTP/1.1";
        /// HTTP version 2.0 protocol identifier
        pub const HTTP_2_0: &str = "HTTP/2.0";
        /// HTTP version 3.0 protocol identifier
        pub const HTTP_3_0: &str = "HTTP/3.0";

        /// HTTP methods
        pub const GET: &str = "GET";
        /// Configuration constant: post
        pub const POST: &str = "POST";
        /// Configuration constant: put
        pub const PUT: &str = "PUT";
        /// Configuration constant: delete
        pub const DELETE: &str = "DELETE";
        /// Configuration constant: head
        pub const HEAD: &str = "HEAD";
        /// Configuration constant: options
        pub const OPTIONS: &str = "OPTIONS";
        /// Configuration constant: patch
        pub const PATCH: &str = "PATCH";
        /// Configuration constant: trace
        pub const TRACE: &str = "TRACE";
        /// Configuration constant: connect
        pub const CONNECT: &str = "CONNECT";

        /// HTTP status code ranges
        pub const SUCCESS_MIN: u16 = 200;
        /// Configuration constant: success max
        pub const SUCCESS_MAX: u16 = 299;
        /// Configuration constant: redirect min
        pub const REDIRECT_MIN: u16 = 300;
        /// Configuration constant: redirect max
        pub const REDIRECT_MAX: u16 = 399;
        /// Configuration constant: client error min
        pub const CLIENT_ERROR_MIN: u16 = 400;
        /// Configuration constant: client error max
        pub const CLIENT_ERROR_MAX: u16 = 499;
        /// Configuration constant: server error min
        pub const SERVER_ERROR_MIN: u16 = 500;
        /// Configuration constant: server error max
        pub const SERVER_ERROR_MAX: u16 = 599;

        /// Common HTTP status codes
        pub const OK: u16 = 200;
        /// Configuration constant: created
        pub const CREATED: u16 = 201;
        /// Configuration constant: accepted
        pub const ACCEPTED: u16 = 202;
        /// Configuration constant: no content
        pub const NO_CONTENT: u16 = 204;
        /// Configuration constant: bad request
        pub const BAD_REQUEST: u16 = 400;
        /// Configuration constant: unauthorized
        pub const UNAUTHORIZED: u16 = 401;
        pub const FORBIDDEN: u16 = 403;
        /// Configuration constant: not found
        pub const NOT_FOUND: u16 = 404;
        /// Configuration constant: method not allowed
        pub const METHOD_NOT_ALLOWED: u16 = 405;
        /// Configuration constant: conflict
        pub const CONFLICT: u16 = 409;
        /// Configuration constant: internal server error
        pub const INTERNAL_SERVER_ERROR: u16 = 500;
        /// Configuration constant: not implemented
        pub const NOT_IMPLEMENTED: u16 = 501;
        /// Configuration constant: bad gateway
        pub const BAD_GATEWAY: u16 = 502;
        /// Configuration constant: service unavailable
        pub const SERVICE_UNAVAILABLE: u16 = 503;
        /// Configuration constant: gateway timeout
        pub const GATEWAY_TIMEOUT: u16 = 504;
    }

    /// TLS/SSL constants
    pub mod tls {
        /// TLS version 1.0 - deprecated, insecure
        pub const TLS_1_0: &str = "TLSv1.0";
        /// TLS version 1.1 - deprecated, insecure
        pub const TLS_1_1: &str = "TLSv1.1";
        /// TLS version 1.2 - secure but older standard
        pub const TLS_1_2: &str = "TLSv1.2";
        pub const TLS_1_3: &str = "TLSv1.3";

        /// Cipher suites (simplified names)
        pub const AES_128_GCM: &str = "AES128-GCM-SHA256";
        /// AES-256 with GCM mode and SHA-384 - high security cipher suite
        pub const AES_256_GCM: &str = "AES256-GCM-SHA384";
        /// ChaCha20-Poly1305 cipher suite - quantum-resistant option
        pub const CHACHA20_POLY1305: &str = "CHACHA20-POLY1305";

        /// Default TLS settings
        pub const DEFAULT_TLS_VERSION: &str = TLS_1_3;
        /// Configuration constant: min tls version
        pub const MIN_TLS_VERSION: &str = TLS_1_2;
    }

    /// WebSocket constants
    pub mod websocket {
        /// Configuration constant: websocket version
        pub const WEBSOCKET_VERSION: u8 = 13;
        /// Configuration constant: websocket magic string
        pub const WEBSOCKET_MAGIC_STRING: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

        /// WebSocket opcodes
        pub const OPCODE_CONTINUATION: u8 = 0x0;
        /// Configuration constant: opcode text
        pub const OPCODE_TEXT: u8 = 0x1;
        /// Configuration constant: opcode binary
        pub const OPCODE_BINARY: u8 = 0x2;
        /// Configuration constant: opcode close
        pub const OPCODE_CLOSE: u8 = 0x8;
        /// Configuration constant: opcode ping
        pub const OPCODE_PING: u8 = 0x9;
        /// Configuration constant: opcode pong
        pub const OPCODE_PONG: u8 = 0xA;
    }

    /// TCP constants
    pub mod tcp {
        /// Configuration constant: tcp nodelay
        pub const TCP_NODELAY: bool = true;
        /// Configuration constant: tcp keepalive
        pub const TCP_KEEPALIVE: bool = true;
        /// Configuration constant: tcp reuseaddr
        pub const TCP_REUSEADDR: bool = true;
        /// Configuration constant: tcp reuseport
        pub const TCP_REUSEPORT: bool = false;

        /// TCP window sizes
        pub const DEFAULT_WINDOW_SIZE: u32 = 65536;
        /// Configuration constant: max window size
        pub const MAX_WINDOW_SIZE: u32 = 1_048_576; // 1MB
        /// Configuration constant: min window size
        pub const MIN_WINDOW_SIZE: u32 = 4096;
    }

    /// UDP constants
    pub mod udp {
        /// Configuration constant: max udp packet size
        pub const MAX_UDP_PACKET_SIZE: usize = 65507;
        /// Configuration constant: default udp buffer size
        pub const DEFAULT_UDP_BUFFER_SIZE: usize = 8192;
        /// Configuration constant: udp broadcast
        pub const UDP_BROADCAST: bool = false;
        /// Configuration constant: udp multicast ttl
        pub const UDP_MULTICAST_TTL: u32 = 1;
    }
}

/// **NETWORK HEADERS** - Standard HTTP headers and values
pub mod headers {
    /// Request headers
    pub const ACCEPT: &str = "Accept";
    /// Configuration constant: accept encoding
    pub const ACCEPT_ENCODING: &str = "Accept-Encoding";
    /// Configuration constant: accept language
    pub const ACCEPT_LANGUAGE: &str = "Accept-Language";
    /// Configuration constant: authorization
    pub const AUTHORIZATION: &str = "Authorization";
    /// Configuration constant: cache control
    pub const CACHE_CONTROL: &str = "Cache-Control";
    /// Configuration constant: content type
    pub const CONTENT_TYPE: &str = "Content-Type";
    /// Configuration constant: content length
    pub const CONTENT_LENGTH: &str = "Content-Length";
    /// Configuration constant: content encoding
    pub const CONTENT_ENCODING: &str = "Content-Encoding";
    /// Configuration constant: user agent
    pub const USER_AGENT: &str = "User-Agent";
    /// Configuration constant: host
    pub const HOST: &str = "Host";
    /// Configuration constant: origin
    pub const ORIGIN: &str = "Origin";
    /// Configuration constant: referer
    pub const REFERER: &str = "Referer";
    pub const X_FORWARDED_FOR: &str = "X-Forwarded-For";
    /// Configuration constant: x real ip
    pub const X_REAL_IP: &str = "X-Real-IP";

    /// Response headers
    pub const SERVER: &str = "Server";
    /// Configuration constant: date
    pub const DATE: &str = "Date";
    /// Configuration constant: etag
    pub const ETAG: &str = "ETag";
    /// Configuration constant: expires
    pub const EXPIRES: &str = "Expires";
    /// Configuration constant: last modified
    pub const LAST_MODIFIED: &str = "Last-Modified";
    /// Configuration constant: location
    pub const LOCATION: &str = "Location";
    /// Configuration constant: set cookie
    pub const SET_COOKIE: &str = "Set-Cookie";
    /// Configuration constant: vary
    pub const VARY: &str = "Vary";

    /// CORS headers
    pub const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";
    /// Configuration constant: access control allow methods
    pub const ACCESS_CONTROL_ALLOW_METHODS: &str = "Access-Control-Allow-Methods";
    /// Configuration constant: access control allow headers
    pub const ACCESS_CONTROL_ALLOW_HEADERS: &str = "Access-Control-Allow-Headers";
    /// Configuration constant: access control expose headers
    pub const ACCESS_CONTROL_EXPOSE_HEADERS: &str = "Access-Control-Expose-Headers";
    /// Configuration constant: access control max age
    pub const ACCESS_CONTROL_MAX_AGE: &str = "Access-Control-Max-Age";
    /// Configuration constant: access control allow credentials
    pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: &str = "Access-Control-Allow-Credentials";

    /// Security headers
    pub const X_CONTENT_TYPE_OPTIONS: &str = "X-Content-Type-Options";
    /// Configuration constant: x frame options
    pub const X_FRAME_OPTIONS: &str = "X-Frame-Options";
    /// Configuration constant: x xss protection
    pub const X_XSS_PROTECTION: &str = "X-XSS-Protection";
    /// Configuration constant: strict transport security
    pub const STRICT_TRANSPORT_SECURITY: &str = "Strict-Transport-Security";
    /// Configuration constant: content security policy
    pub const CONTENT_SECURITY_POLICY: &str = "Content-Security-Policy";

    /// `BearDog` custom headers
    pub const X_BEARDOG_VERSION: &str = "X-BearDog-Version";
    /// Configuration constant: x beardog request id
    pub const X_BEARDOG_REQUEST_ID: &str = "X-BearDog-Request-ID";
    /// Configuration constant: x beardog trace id
    pub const X_BEARDOG_TRACE_ID: &str = "X-BearDog-Trace-ID";
    /// Configuration constant: x beardog span id
    pub const X_BEARDOG_SPAN_ID: &str = "X-BearDog-Span-ID";

    /// Content types
    pub const APPLICATION_JSON: &str = "application/json";
    /// Configuration constant: application xml
    pub const APPLICATION_XML: &str = "application/xml";
    pub const APPLICATION_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
    pub const MULTIPART_FORM_DATA: &str = "multipart/form-data";
    /// Configuration constant: text plain
    pub const TEXT_PLAIN: &str = "text/plain";
    /// Configuration constant: text html
    pub const TEXT_HTML: &str = "text/html";
    /// Configuration constant: text css
    pub const TEXT_CSS: &str = "text/css";
    /// Configuration constant: text javascript
    pub const TEXT_JAVASCRIPT: &str = "text/javascript";
    /// Configuration constant: application octet stream
    pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

    /// Default header values
    pub const DEFAULT_USER_AGENT: &str = concat!("BearDog/", env!("CARGO_PKG_VERSION"));
    /// Configuration constant: default server
    pub const DEFAULT_SERVER: &str = concat!("BearDog/", env!("CARGO_PKG_VERSION"));
    /// Configuration constant: default accept
    pub const DEFAULT_ACCEPT: &str = "application/json, text/plain, */*";
    /// Configuration constant: default accept encoding
    pub const DEFAULT_ACCEPT_ENCODING: &str = "gzip, deflate, br";
    /// Configuration constant: default content type
    pub const DEFAULT_CONTENT_TYPE: &str = APPLICATION_JSON;
}

/// **NETWORK INTERVALS** - Network-related timing intervals
pub mod intervals {
    use super::Duration;

    /// Connection management intervals
    pub const CONNECTION_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    /// Configuration constant: idle connection cleanup
    pub const IDLE_CONNECTION_CLEANUP: Duration = Duration::from_secs(60);
    /// Configuration constant: connection pool cleanup
    pub const CONNECTION_POOL_CLEANUP: Duration = Duration::from_secs(300);

    /// Health check intervals
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    /// Configuration constant: ping interval
    pub const PING_INTERVAL: Duration = Duration::from_secs(60);
    /// Configuration constant: heartbeat interval
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

    /// Retry intervals
    pub const RETRY_INTERVAL: Duration = Duration::from_millis(100);
    /// Configuration constant: exponential backoff base
    pub const EXPONENTIAL_BACKOFF_BASE: Duration = Duration::from_millis(100);
    /// Configuration constant: max retry interval
    pub const MAX_RETRY_INTERVAL: Duration = Duration::from_secs(30);

    /// Metrics collection intervals
    pub const NETWORK_METRICS_INTERVAL: Duration = Duration::from_secs(15);
    /// Configuration constant: bandwidth metrics interval
    pub const BANDWIDTH_METRICS_INTERVAL: Duration = Duration::from_secs(5);
    /// Configuration constant: connection metrics interval
    pub const CONNECTION_METRICS_INTERVAL: Duration = Duration::from_secs(10);

    /// Keep-alive intervals
    pub const TCP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(60);
    /// Configuration constant: http keepalive interval
    pub const HTTP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(120);
    /// Configuration constant: websocket ping interval
    pub const WEBSOCKET_PING_INTERVAL: Duration = Duration::from_secs(30);
}

/// **LOAD BALANCING CONSTANTS** - Load balancing and traffic distribution
pub mod load_balancing {
    use std::time::Duration;
    /// Load balancing algorithms
    pub const ROUND_ROBIN: &str = "round_robin";
    /// Configuration constant: least connections
    pub const LEAST_CONNECTIONS: &str = "least_connections";
    /// Configuration constant: weighted round robin
    pub const WEIGHTED_ROUND_ROBIN: &str = "weighted_round_robin";
    /// Configuration constant: ip hash
    pub const IP_HASH: &str = "ip_hash";
    /// Configuration constant: random
    pub const RANDOM: &str = "random";
    /// Configuration constant: least response time
    pub const LEAST_RESPONSE_TIME: &str = "least_response_time";

    /// Health check settings
    pub const DEFAULT_HEALTH_CHECK_PATH: &str = "/health";
    /// Configuration constant: default health check method
    pub const DEFAULT_HEALTH_CHECK_METHOD: &str = "GET";
    /// Configuration constant: default health check interval
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    /// Configuration constant: default health check timeout
    pub const DEFAULT_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

    /// Failover settings
    pub const DEFAULT_MAX_FAILS: u32 = 3;
    /// Configuration constant: default fail timeout
    pub const DEFAULT_FAIL_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: default recovery time
    pub const DEFAULT_RECOVERY_TIME: Duration = Duration::from_secs(30);

    /// Session affinity
    pub const SESSION_COOKIE_NAME: &str = "BEARDOG_SESSION";
    /// Configuration constant: session header name
    pub const SESSION_HEADER_NAME: &str = "X-BearDog-Session";
    /// Configuration constant: default session timeout
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);
}

/// **CIRCUIT BREAKER CONSTANTS** - Circuit breaker pattern settings
pub mod circuit_breaker {
    use super::Duration;

    /// Circuit breaker states
    pub const STATE_CLOSED: &str = "closed";
    /// Configuration constant: state open
    pub const STATE_OPEN: &str = "open";
    /// Configuration constant: state half open
    pub const STATE_HALF_OPEN: &str = "half_open";

    /// Default thresholds
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
    /// Configuration constant: default success threshold
    pub const DEFAULT_SUCCESS_THRESHOLD: u32 = 3;
    /// Configuration constant: default timeout
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: default half open max calls
    pub const DEFAULT_HALF_OPEN_MAX_CALLS: u32 = 3;

    /// Monitoring intervals
    pub const METRICS_WINDOW: Duration = Duration::from_secs(60);
    /// Configuration constant: reset timeout
    pub const RESET_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: state check interval
    pub const STATE_CHECK_INTERVAL: Duration = Duration::from_secs(10);
}

/// **RATE LIMITING CONSTANTS** - Rate limiting and throttling
pub mod rate_limiting {
    use super::Duration;

    /// Rate limiting algorithms
    pub const TOKEN_BUCKET: &str = "token_bucket";
    /// Configuration constant: leaky bucket
    pub const LEAKY_BUCKET: &str = "leaky_bucket";
    /// Configuration constant: fixed window
    pub const FIXED_WINDOW: &str = "fixed_window";
    /// Configuration constant: sliding window
    pub const SLIDING_WINDOW: &str = "sliding_window";

    /// Default limits
    pub const DEFAULT_REQUESTS_PER_SECOND: u32 = 100;
    /// Configuration constant: default burst size
    pub const DEFAULT_BURST_SIZE: u32 = 200;
    /// Configuration constant: default window size
    pub const DEFAULT_WINDOW_SIZE: Duration = Duration::from_secs(60);

    /// Rate limiting headers
    pub const X_RATELIMIT_LIMIT: &str = "X-RateLimit-Limit";
    /// Configuration constant: x ratelimit remaining
    pub const X_RATELIMIT_REMAINING: &str = "X-RateLimit-Remaining";
    /// Configuration constant: x ratelimit reset
    pub const X_RATELIMIT_RESET: &str = "X-RateLimit-Reset";
    /// Configuration constant: x ratelimit retry after
    pub const X_RATELIMIT_RETRY_AFTER: &str = "X-RateLimit-Retry-After";

    /// Key extractors
    pub const KEY_IP_ADDRESS: &str = "ip_address";
    /// Configuration constant: key user id
    pub const KEY_USER_ID: &str = "user_id";
    /// Configuration constant: key api key
    pub const KEY_API_KEY: &str = "api_key";
    /// Configuration constant: key session
    pub const KEY_SESSION: &str = "session";
}

// Re-export commonly used constants for convenience
pub use addresses::{DEFAULT_DNS_PORT, DEFAULT_METRICS_BIND};
pub use defaults::{DEFAULT_API_PORT, DEFAULT_METRICS_PORT};
pub use limits::{MAX_CONNECTIONS, MAX_HEADER_SIZE};
pub use timeouts::{CONNECTION_TIMEOUT, REQUEST_TIMEOUT};

/// **ADDITIONAL NETWORK MODULES** - Extended network constants
pub mod api {
    pub use super::defaults::DEFAULT_HTTP_VERSION;
    /// API-related network constants
    pub use super::defaults::{DEFAULT_API_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT};
    pub use super::timeouts::{REQUEST_TIMEOUT, RESPONSE_TIMEOUT};
}

/// Node networking constants
pub mod nodes {
    pub use super::defaults::{DEFAULT_CONNECTION_POOL_SIZE, DEFAULT_MAX_CONNECTIONS};
    pub use super::timeouts::{HEARTBEAT_TIMEOUT, KEEP_ALIVE_TIMEOUT};

    /// Node-specific constants
    pub const DEFAULT_NODE_DISCOVERY_PORT: u16 = 8090;
    /// Configuration constant: default cluster port
    pub const DEFAULT_CLUSTER_PORT: u16 = 8091;
}

/// HTTP-specific constants
pub mod http {
    pub use super::defaults::{DEFAULT_HTTP_VERSION, DEFAULT_TLS_VERSION};
    pub use super::timeouts::{HTTP_REQUEST_TIMEOUT, HTTP_RESPONSE_TIMEOUT};

    /// HTTP headers
    pub const CONTENT_TYPE_JSON: &str = "application/json";
    /// Configuration constant: content type binary
    pub const CONTENT_TYPE_BINARY: &str = "application/octet-stream";
    /// Configuration constant: user agent
    pub const USER_AGENT: &str = "BearDog/3.0";
}

/// Service discovery constants
pub mod services {
    pub use super::defaults::{DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT};
    pub use super::timeouts::DNS_RESOLUTION_TIMEOUT;

    /// Service types
    pub const SERVICE_TYPE_API: &str = "api";
    /// Configuration constant: service type metrics
    pub const SERVICE_TYPE_METRICS: &str = "metrics";
    /// Configuration constant: service type health
    pub const SERVICE_TYPE_HEALTH: &str = "health";
}
