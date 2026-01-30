//! Unit tests for doctor handler

#[cfg(test)]
mod tests {
    
    use crate::DoctorArgs;

    #[test]
    fn test_doctor_args_creation() {
        let args = DoctorArgs {
            comprehensive: true,
            format: "json".to_string(),
            component: Some("crypto".to_string()),
        };

        assert!(args.comprehensive);
        assert_eq!(args.format, "json");
        assert_eq!(args.component, Some("crypto".to_string()));
    }

    #[test]
    fn test_doctor_args_defaults() {
        let args = DoctorArgs {
            comprehensive: false,
            format: "text".to_string(),
            component: None,
        };

        assert!(!args.comprehensive);
        assert_eq!(args.format, "text");
        assert!(args.component.is_none());
    }

    #[test]
    fn test_doctor_args_all_formats() {
        let formats = vec!["text", "json"];

        for format in formats {
            let args = DoctorArgs {
                comprehensive: false,
                format: format.to_string(),
                component: None,
            };
            assert_eq!(args.format, format);
        }
    }

    #[test]
    fn test_doctor_args_components() {
        let components = vec!["entropy", "storage", "hsm", "server", "crypto"];

        for component in components {
            let args = DoctorArgs {
                comprehensive: false,
                format: "text".to_string(),
                component: Some(component.to_string()),
            };
            assert_eq!(args.component, Some(component.to_string()));
        }
    }

    #[test]
    fn test_doctor_args_clone() {
        let args1 = DoctorArgs {
            comprehensive: true,
            format: "json".to_string(),
            component: Some("hsm".to_string()),
        };

        let args2 = args1.clone();
        assert_eq!(args1.comprehensive, args2.comprehensive);
        assert_eq!(args1.format, args2.format);
        assert_eq!(args1.component, args2.component);
    }
}
