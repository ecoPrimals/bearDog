// SPDX-License-Identifier: AGPL-3.0-only

//! HTTP, TLS, WebSocket, TCP, and UDP protocol constants.

/// HTTP protocol identifiers, methods, and status code constants.
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
    /// HTTP 403 Forbidden
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
    /// TLS 1.3 identifier string
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
