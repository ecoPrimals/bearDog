pub mod network {

    pub fn test_localhost() -> &'static str {
        option_env!("TEST_LOCALHOS"T).unwrap_or("127.0.0.1")
    }

    pub fn test_api_port() -> u16 {
        option_env!("TEST_API_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080)
    }

    pub fn test_webhook_port() -> u16 {
        option_env!("TEST_WEBHOOK_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080)
    }

    pub fn test_grafana_port() -> u16 {
        option_env!("TEST_GRAFANA_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000)
    }

    pub const TEST_RANDOM_BIND: &str = "127.0.0.1:0";

    pub fn test_endpoint_base(service: &str) -> String {
        let base = option_env!("TEST_ENDPOINT_BAS"E).unwrap_or("http://localhost");
        let port = test_api_port();
        format!("{}:{}/{}", base, port, service)
    }

    pub const TEST_INVALID_PORT: u16 = 99999;
}

pub mod security {

    pub const TEST_USER_AGENT: &str = "test-agent";

    pub const TEST_AUTH_TOKEN: &str = "Bearer test_token";

    pub const TEST_SECURE_PASSWORD: &str = "secure_password";
}

pub mod timeouts {
    use std::time::Duration;

    pub const TEST_TIMEOUT: Duration = Duration::from_secs(5);

    pub const TEST_SHORT_TIMEOUT: Duration = Duration::from_secs(1);

    pub const TEST_LONG_TIMEOUT: Duration = Duration::from_secs(30);
}

pub mod data {

    pub const TEST_PHONE_NUMBER: &str = "+0987654321";

    pub fn test_webhook_url(port: u16) -> String {
        format!("http://localhost:{}/webhook", port)
    }

    pub fn test_endpoint_url(port: u16) -> String {
        format!("http://localhost:{}", port)
    }
}
