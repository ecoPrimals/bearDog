// SPDX-License-Identifier: AGPL-3.0-or-later

//! IANA-style port ranges and well-known ports.

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

/// Default Consul agent HTTP API port (official `HashiCorp` default).
pub const DEFAULT_CONSUL_HTTP_PORT: u16 = 8500;
