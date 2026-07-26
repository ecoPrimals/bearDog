// SPDX-License-Identifier: AGPL-3.0-or-later

// Helper functions for constraint verification

/// Simple glob-style path matching
///
/// Supports:
/// - `*` matches any sequence within a path segment
/// - `**` matches any sequence across path segments
/// - Exact matches
///
/// # Examples
/// ```
/// # use beardog_types::genetics_constraints_helpers::path_matches;
/// assert!(path_matches("data/file.txt", "data/*"));
/// assert!(path_matches("data/subdir/file.txt", "data/**"));
/// assert!(path_matches("data/file.txt", "data/file.txt"));
/// assert!(!path_matches("other/file.txt", "data/*"));
/// ```
#[must_use]
pub fn path_matches(path: &str, pattern: &str) -> bool {
    // Handle exact matches
    if path == pattern {
        return true;
    }

    // Handle ** (matches across segments)
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0].trim_end_matches('/');
            let suffix = parts[1].trim_start_matches('/');

            let matches_prefix = prefix.is_empty() || path.starts_with(prefix);
            let matches_suffix = suffix.is_empty() || path.ends_with(suffix);

            return matches_prefix && matches_suffix;
        }
    }

    // Handle * (matches within segment)
    if pattern.contains('*') && !pattern.contains("**") {
        return glob_match_simple(path, pattern);
    }

    // No wildcards, must be exact
    false
}

/// Simple glob matching within segments
///
/// The `*` wildcard only matches within a single path segment (doesn't cross '/')
fn glob_match_simple(path: &str, pattern: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();

    if parts.is_empty() {
        return path.is_empty();
    }

    let mut pos = 0;

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            // First part must match at start
            if !path[pos..].starts_with(part) {
                return false;
            }
            pos += part.len();
        } else if i == parts.len() - 1 {
            // Last part must match at end
            if !path.ends_with(part) {
                return false;
            }
            // Check if we can reach the end without crossing path separators
            // (unless there are no wildcards between segments)
            if let Some(found_pos) = path[pos..].find(part) {
                // Ensure we don't cross path separators when matching with *
                let matched_section = &path[pos..pos + found_pos];
                if matched_section.contains('/') {
                    return false;
                }
                pos += found_pos;
            } else {
                return false;
            }
        } else {
            // Middle parts must exist somewhere in the current segment
            if let Some(found_pos) = path[pos..].find(part) {
                // Ensure we don't cross path separators when matching with *
                let matched_section = &path[pos..pos + found_pos];
                if matched_section.contains('/') {
                    return false;
                }
                pos += found_pos + part.len();
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(path_matches("data/file.txt", "data/file.txt"));
        assert!(!path_matches("data/other.txt", "data/file.txt"));
    }

    #[test]
    fn test_single_wildcard() {
        assert!(path_matches("data/file.txt", "data/*.txt"));
        assert!(path_matches("data/file.csv", "data/*.csv"));
        assert!(!path_matches("data/file.txt", "data/*.csv"));
        assert!(!path_matches("data/sub/file.txt", "data/*.txt"));
    }

    #[test]
    fn test_double_wildcard() {
        assert!(path_matches("data/file.txt", "data/**"));
        assert!(path_matches("data/sub/file.txt", "data/**"));
        assert!(path_matches("data/sub/deep/file.txt", "data/**"));
        assert!(!path_matches("other/file.txt", "data/**"));
    }

    #[test]
    fn test_double_wildcard_with_suffix() {
        assert!(path_matches("data/sub/file.txt", "**/file.txt"));
        assert!(path_matches("file.txt", "**/file.txt"));
        assert!(!path_matches("data/sub/other.txt", "**/file.txt"));
    }

    #[test]
    fn test_complex_patterns() {
        assert!(path_matches("data/2024/file.txt", "data/*/*.txt"));
        assert!(path_matches("raw_data/temperature.nc", "raw_data/*"));
        assert!(!path_matches("processed/temperature.nc", "raw_data/*"));
    }
}
