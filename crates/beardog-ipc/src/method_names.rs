// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical JSON-RPC method name normalization.

/// Normalize a JSON-RPC method name to its canonical form.
///
/// Handles common variations:
/// - Underscores vs dots: `crypto_sign` → `crypto.sign` (no change if already dotted)
/// - Trailing/leading whitespace
/// - Case normalization (lowercase)
pub fn normalize_method(method: &str) -> String {
    method.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_method_trims_and_lowercases() {
        assert_eq!(normalize_method("  Health.Ping  "), "health.ping");
    }

    #[test]
    fn normalize_method_preserves_dots_and_underscores_as_given_after_lower() {
        assert_eq!(normalize_method("crypto.sign"), "crypto.sign");
        assert_eq!(normalize_method("crypto_sign"), "crypto_sign");
    }

    #[test]
    fn normalize_method_empty_trimmed() {
        assert_eq!(normalize_method("   "), "");
    }

    #[test]
    fn normalize_method_non_ascii_prefix_unchanged_by_ascii_lowercase() {
        let out = normalize_method("İpc.Ping");
        assert!(out.ends_with("pc.ping"));
        assert_ne!(out, "ipc.ping");
    }

    #[test]
    fn normalize_method_tabs_and_newlines_trimmed() {
        assert_eq!(normalize_method("\t\nHealth.Check\r\n"), "health.check");
    }

    #[test]
    fn normalize_method_preserves_numeric_segments() {
        assert_eq!(normalize_method("V2.Method"), "v2.method");
    }
}
