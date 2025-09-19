

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde_json::Value;
use std::collections::HashMap;

pub struct ProtocolTranslator {

    translation_rules: HashMap<String, TranslationRule>,

    config: super::config::ProtocolTranslatorConfig,
}

#[derive(Debug, Clone)]
    to_protocol: String,
    field_mappings: HashMap<String, String>,
}

impl ProtocolTranslator {

/// New operation.
    /// Creates a new instance
    pub fn new(config: super::config::ProtocolTranslatorConfig) -> Self {
        let mut translation_rules = HashMap::with_capacity(16);

        Self::initialize_default_rules(Value,
        from_protocol: &str,
        to_protocol: &str,
    ) -> Result<Value, BearDogError> {
        let rule_key = format!("{from_protocol}_{to_protocol}");

        if let Some(Value,
        rule: &TranslationRule,
    ) -> Result<Value, BearDogError> {
        if let Value::Object(ref mut map) = message {
            let mut new_map = serde_json::Map::new();

            for (old_key, value) in map.iter() {
                let new_key = rule.field_mappings.get(old_key).unwrap_or(old_key);
                new_map.insert(new_key.clone(), value.clone());
            }

            Ok(Value::Object(&mut HashMap<&str, TranslationRule>) {

        let mut http_to_grpc_mappings = HashMap::with_capacity(16);
        http_to_grpc_mappings.insert("content-type".to_string(), "content_type");
        http_to_grpc_mappings.insert("user-agent".to_string(), "user_agent");

        rules.insert(
            "http_grpc".to_string(),
            TranslationRule {
                from_protocol: "http".to_string(),
                to_protocol: "grpc".to_string(),
        );

        let mut rest_to_graphql_mappings = HashMap::with_capacity(16);
        rest_to_graphql_mappings.insert("id".to_string(), "id");
        rest_to_graphql_mappings.insert("name".to_string(), "name");

        rules.insert(
            "rest_graphql".to_string(),
            TranslationRule {
                from_protocol: "rest".to_string(),
                to_protocol: "graphql".to_string(),
        );
    }

/// Add Translation Rule operation.
    pub fn add_translation_rule(&str,
        to: &str,
        mappings: HashMap<&str, &str>,
    ) {
        let rule_key = format!("{from}_{to}");
        let rule = TranslationRule {
            from_protocol: from.to_string(),
            to_protocol: to.to_string(), rule);
    }
}
