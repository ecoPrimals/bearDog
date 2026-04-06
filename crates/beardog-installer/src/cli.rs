// SPDX-License-Identifier: AGPL-3.0-or-later

//! CLI parsing helpers shared with the `beardog-installer` binary.

use crate::PrimalName;
use anyhow::Result;

/// Parse primal names from a comma-separated string (used by install/validate/uninstall).
///
/// # Errors
///
/// Returns an error if any comma-separated token is not a known primal name.
pub fn parse_primals(primals_str: Option<String>) -> Result<Vec<PrimalName>> {
    match primals_str {
        Some(s) => {
            let mut primals = Vec::new();
            for name in s.split(',') {
                let name = name.trim();
                match PrimalName::parse_name(name) {
                    Some(primal) => primals.push(primal),
                    None => anyhow::bail!("Unknown primal: {name}"),
                }
            }
            Ok(primals)
        }
        None => Ok(PrimalName::well_known()),
    }
}

/// Format a primal list for human-readable CLI output.
pub fn primals_to_string(primals: &[PrimalName]) -> String {
    primals
        .iter()
        .map(PrimalName::display_name)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_primals_none_uses_well_known() {
        let list = parse_primals(None).expect("parse");
        assert!(!list.is_empty());
    }

    #[test]
    fn parse_primals_single_valid_slug() {
        let list = parse_primals(Some("beardog".to_string())).expect("parse");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name(), "beardog");
    }

    #[test]
    fn parse_primals_comma_separated_trims_whitespace() {
        let list = parse_primals(Some(" beardog , songbird ".to_string())).expect("parse");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name(), "beardog");
        assert_eq!(list[1].name(), "songbird");
    }

    #[test]
    fn parse_primals_rejects_invalid_token() {
        let err = parse_primals(Some("ok,bad token".to_string())).expect_err("invalid");
        assert!(
            err.to_string().contains("Unknown primal"),
            "unexpected: {err}"
        );
    }

    #[test]
    fn primals_to_string_joins_display_names() {
        let p = vec![PrimalName::new("foo-bar"), PrimalName::new("baz")];
        let s = primals_to_string(&p);
        assert!(s.contains("Foo Bar"));
        assert!(s.contains("Baz"));
    }

    #[test]
    fn parse_primals_empty_string_rejects() {
        let err = parse_primals(Some(String::new())).expect_err("empty string has no valid slugs");
        assert!(
            err.to_string().contains("Unknown primal") || err.to_string().contains("primal"),
            "unexpected: {err}"
        );
    }

    #[test]
    fn parse_primals_empty_list_of_commas() {
        let err = parse_primals(Some(",,,".to_string())).expect_err("no valid tokens");
        assert!(err.to_string().contains("Unknown primal"), "{err}");
    }

    #[test]
    fn primals_to_string_empty_slice() {
        let s = primals_to_string(&[]);
        assert!(s.is_empty());
    }
}
