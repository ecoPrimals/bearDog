//! Tests for UniBin operational modes
//!
//! Testing CLI structure and mode dispatching.

#[cfg(test)]
mod tests {
    use std::process::Command;

    /// Get the path to the beardog binary
    fn beardog_bin() -> String {
        let mut path = std::env::current_exe().unwrap();
        path.pop(); // Remove test binary name
        path.pop(); // Remove 'deps' directory
        path.push("beardog");
        path.to_string_lossy().to_string()
    }

    #[test]
    fn test_help_command() {
        let output = Command::new(beardog_bin())
            .arg("--help")
            .output()
            .expect("Failed to execute beardog");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify help output contains expected content
        assert!(stdout.contains("BearDog"));
        assert!(stdout.contains("server"));
        assert!(stdout.contains("daemon"));
        assert!(stdout.contains("client"));
        assert!(stdout.contains("doctor"));
    }

    #[test]
    fn test_version_command() {
        let output = Command::new(beardog_bin())
            .arg("--version")
            .output()
            .expect("Failed to execute beardog");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify version output
        assert!(stdout.contains("beardog"));
        assert!(stdout.contains("0.9.0") || stdout.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_doctor_mode_basic() {
        let output = Command::new(beardog_bin())
            .arg("doctor")
            .output()
            .expect("Failed to execute beardog doctor");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify doctor output contains health check information
        assert!(stdout.contains("Health Check") || stdout.contains("Version"));
        assert!(stdout.contains("✅") || stdout.contains("OK"));
    }

    #[test]
    fn test_doctor_mode_json() {
        let output = Command::new(beardog_bin())
            .arg("doctor")
            .arg("--format")
            .arg("json")
            .output()
            .expect("Failed to execute beardog doctor --format json");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify JSON output
        assert!(stdout.contains("{"));
        assert!(stdout.contains("status"));
        assert!(stdout.contains("version"));
    }

    #[test]
    fn test_doctor_mode_comprehensive() {
        let output = Command::new(beardog_bin())
            .arg("doctor")
            .arg("--comprehensive")
            .output()
            .expect("Failed to execute beardog doctor --comprehensive");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify comprehensive output
        assert!(stdout.contains("System") || stdout.contains("Crypto"));
    }

    #[test]
    fn test_invalid_command() {
        let output = Command::new(beardog_bin())
            .arg("invalid-command")
            .output()
            .expect("Failed to execute beardog");

        // Should fail with error
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Verify error message
        assert!(stderr.contains("error") || stderr.contains("unrecognized"));
    }

    #[test]
    fn test_server_help() {
        let output = Command::new(beardog_bin())
            .arg("server")
            .arg("--help")
            .output()
            .expect("Failed to execute beardog server --help");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify server mode help
        assert!(stdout.contains("server") || stdout.contains("socket"));
        assert!(stdout.contains("--socket") || stdout.contains("--family-id"));
    }

    #[test]
    fn test_daemon_help() {
        let output = Command::new(beardog_bin())
            .arg("daemon")
            .arg("--help")
            .output()
            .expect("Failed to execute beardog daemon --help");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify daemon mode help
        assert!(stdout.contains("daemon") || stdout.contains("background"));
    }

    #[test]
    fn test_client_help() {
        let output = Command::new(beardog_bin())
            .arg("client")
            .arg("--help")
            .output()
            .expect("Failed to execute beardog client --help");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify client mode help
        assert!(stdout.contains("client") || stdout.contains("endpoint"));
    }

    #[test]
    fn test_log_level_flag() {
        let output = Command::new(beardog_bin())
            .arg("--log-level")
            .arg("debug")
            .arg("doctor")
            .output()
            .expect("Failed to execute beardog with --log-level");

        assert!(output.status.success());
    }
}

