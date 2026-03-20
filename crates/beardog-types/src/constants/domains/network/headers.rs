// SPDX-License-Identifier: AGPL-3.0-only

//! HTTP header names, content types, and default header values.

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
/// Standard proxy header for original client IP chain
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
/// `application/x-www-form-urlencoded` content type
pub const APPLICATION_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
/// `multipart/form-data` content type
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
