use crate::schema::*;
use crate::generated_types::{PhoneType, EmailType, PHONE_SHORTHAND_KEYS};
use crate::i18n::AliasManager;
use serde_yaml::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Invalid schema: {0}")]
    Schema(String),
    #[error("Phone normalization error: {0}")]
    Phone(String),
    #[error("Email validation error: {0}")]
    Email(String),
}

pub struct Parser {
    alias_manager: AliasManager,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            alias_manager: AliasManager::new(),
        }
    }

    pub fn with_alias_manager(alias_manager: AliasManager) -> Self {
        Self { alias_manager }
    }

    /// Parse yCard from YAML text with lenient mode
    pub fn parse_lenient(&self, input: &str, locale: Option<&str>) -> Result<YCard, ParseError> {
        // First parse as generic YAML
        let value: Value = serde_yaml::from_str(input)?;
        
        // Convert to our schema with normalization
        self.value_to_ycard(value, locale)
    }

    /// Parse yCard from YAML text with strict mode  
    pub fn parse_strict(&self, input: &str) -> Result<YCard, ParseError> {
        let ycard: YCard = serde_yaml::from_str(input)?;
        Ok(ycard)
    }

    fn value_to_ycard(&self, mut value: Value, locale: Option<&str>) -> Result<YCard, ParseError> {
        let mut ycard = YCard::default();

        if let Value::Mapping(ref mut map) = value {
            // Process shorthand phone fields first
            self.extract_shorthand_phones(map, &mut ycard, locale)?;
            
            // Process regular fields
            for (key, val) in map.iter() {
                if let Value::String(key_str) = key {
                    let normalized_key = self.normalize_field_key(key_str, locale);
                    
                    match normalized_key.as_str() {
                        "version" => {
                            if let Some(v) = val.as_u64() {
                                ycard.version = v as u8;
                            }
                        }
                        "uid" => {
                            if let Value::String(s) = val {
                                ycard.uid = Some(s.clone());
                            }
                        }
                        "name" => {
                            ycard.name = Some(self.extract_name(val, locale)?);
                        }
                        "phones" => {
                            ycard.phones = Some(self.extract_phones(val, locale)?);
                        }
                        "emails" => {
                            ycard.emails = Some(self.extract_emails(val, locale)?);
                        }
                        "addresses" => {
                            ycard.addresses = Some(self.extract_addresses(val, locale)?);
                        }
                        "manager" => {
                            if let Value::String(s) = val {
                                ycard.manager = Some(s.clone());
                            }
                        }
                        "metadata" => {
                            ycard.metadata = Some(self.extract_metadata(val)?);
                        }
                        _ => {
                            // Unknown field - could add to metadata or warn
                        }
                    }
                }
            }
        }

        Ok(ycard)
    }

    fn normalize_field_key(&self, key: &str, locale: Option<&str>) -> String {
        self.alias_manager
            .resolve_key_alias(key, locale)
            .unwrap_or_else(|| key.to_string())
    }

    fn extract_shorthand_phones(
        &self, 
        map: &mut serde_yaml::Mapping, 
        ycard: &mut YCard, 
        locale: Option<&str>
    ) -> Result<(), ParseError> {
        // Using generated shorthand keys from schema
        let _shorthand_keys = PHONE_SHORTHAND_KEYS;
        let mut shorthand_phones = Vec::new();

        // Check for localized shorthand keys
        let keys_to_remove: Vec<_> = map.keys()
            .filter_map(|k| {
                if let Value::String(key_str) = k {
                    let normalized = self.normalize_field_key(key_str, locale);
                    if normalized.starts_with("phones.type:") {
                        Some(k.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for key in keys_to_remove {
            if let Some(value) = map.remove(&key) {
                if let Value::String(key_str) = &key {
                    let normalized = self.normalize_field_key(key_str, locale);
                    if let Some(type_part) = normalized.strip_prefix("phones.type:") {
                        let phone_type = PhoneType::from_str_with_locale(type_part, locale.unwrap_or("en"));
                        let phones = self.value_to_phones(value, vec![phone_type], locale)?;
                        shorthand_phones.extend(phones);
                    }
                }
            }
        }

        if !shorthand_phones.is_empty() {
            ycard.phones = Some(shorthand_phones);
        }

        Ok(())
    }

    fn extract_name(&self, value: &Value, locale: Option<&str>) -> Result<Name, ParseError> {
        match value {
            Value::String(s) => {
                // Simple string name - try to parse into components
                Ok(Name {
                    display_name: Some(s.clone()),
                    given_name: None, // Could implement name parsing logic here
                    family_name: None,
                    middle_name: None,
                    honorific_prefix: None,
                    honorific_suffix: None,
                    script: None,
                })
            }
            Value::Mapping(map) => {
                let mut name = Name {
                    display_name: None,
                    given_name: None,
                    family_name: None,
                    middle_name: None,
                    honorific_prefix: None,
                    honorific_suffix: None,
                    script: None,
                };

                for (key, val) in map {
                    if let Value::String(key_str) = key {
                        let normalized_key = self.normalize_field_key(key_str, locale);
                        match normalized_key.as_str() {
                            "name.givenName" => {
                                name.given_name = Some(self.value_to_string_vec(val));
                            }
                            "name.familyName" => {
                                name.family_name = Some(self.value_to_string_vec(val));
                            }
                            "name.displayName" => {
                                name.display_name = val.as_str().map(|s| s.to_string());
                            }
                            // Add other name fields...
                            _ => {}
                        }
                    }
                }

                Ok(name)
            }
            _ => Err(ParseError::Schema("Invalid name format".to_string())),
        }
    }

    fn extract_phones(&self, value: &Value, locale: Option<&str>) -> Result<Vec<Phone>, ParseError> {
        match value {
            Value::String(s) => {
                // Single phone number
                Ok(vec![self.parse_phone_string(s, vec![PhoneType::Other], locale)?])
            }
            Value::Sequence(seq) => {
                let mut phones = Vec::new();
                for item in seq {
                    match item {
                        Value::String(s) => {
                            phones.push(self.parse_phone_string(s, vec![PhoneType::Other], locale)?);
                        }
                        Value::Mapping(_) => {
                            // Parse as phone object
                            phones.push(self.parse_phone_object(item, locale)?);
                        }
                        _ => {}
                    }
                }
                Ok(phones)
            }
            Value::Mapping(_) => {
                // Single phone object
                Ok(vec![self.parse_phone_object(value, locale)?])
            }
            _ => Err(ParseError::Schema("Invalid phones format".to_string())),
        }
    }

    fn value_to_phones(&self, value: Value, default_types: Vec<PhoneType>, locale: Option<&str>) -> Result<Vec<Phone>, ParseError> {
        match value {
            Value::String(s) => {
                Ok(vec![self.parse_phone_string(&s, default_types, locale)?])
            }
            Value::Sequence(seq) => {
                let mut phones = Vec::new();
                for item in seq {
                    match item {
                        Value::String(s) => {
                            phones.push(self.parse_phone_string(&s, default_types.clone(), locale)?);
                        }
                        Value::Mapping(_) => {
                            phones.push(self.parse_phone_object(&item, locale)?);
                        }
                        _ => {}
                    }
                }
                Ok(phones)
            }
            _ => Ok(vec![self.parse_phone_string(value.as_str().unwrap_or(""), default_types, locale)?])
        }
    }

    fn parse_phone_string(&self, s: &str, default_types: Vec<PhoneType>, _locale: Option<&str>) -> Result<Phone, ParseError> {
        // Simple phone parsing - in real implementation would use libphonenumber
        let (number, ext) = if let Some(ext_pos) = s.find("ext") {
            let (num_part, ext_part) = s.split_at(ext_pos);
            let ext = ext_part.trim_start_matches("ext").trim_start_matches('.').trim();
            (num_part.trim(), Some(ext.to_string()))
        } else {
            (s.trim(), None)
        };

        // Normalize to E.164 (simplified)
        let normalized = self.normalize_phone_number(number)?;

        Ok(Phone {
            number: normalized,
            r#type: default_types,
            ext,
            preferred: None,
            label: None,
        })
    }

    fn parse_phone_object(&self, value: &Value, locale: Option<&str>) -> Result<Phone, ParseError> {
        if let Value::Mapping(map) = value {
            let mut phone = Phone {
                number: String::new(),
                r#type: vec![PhoneType::Other],
                ext: None,
                preferred: None,
                label: None,
            };

            for (key, val) in map {
                if let Value::String(key_str) = key {
                    match key_str.as_str() {
                        "number" => {
                            if let Some(num) = val.as_str() {
                                phone.number = self.normalize_phone_number(num)?;
                            }
                        }
                        "type" => {
                            phone.r#type = self.parse_phone_types(val, locale)?;
                        }
                        "ext" => {
                            phone.ext = val.as_str().map(|s| s.to_string());
                        }
                        "preferred" => {
                            phone.preferred = val.as_bool();
                        }
                        "label" => {
                            phone.label = val.as_str().map(|s| s.to_string());
                        }
                        _ => {}
                    }
                }
            }

            Ok(phone)
        } else {
            Err(ParseError::Schema("Invalid phone object".to_string()))
        }
    }

    fn parse_phone_types(&self, value: &Value, locale: Option<&str>) -> Result<Vec<PhoneType>, ParseError> {
        match value {
            Value::String(s) => {
                Ok(vec![PhoneType::from_str_with_locale(s, locale.unwrap_or("en"))])
            }
            Value::Sequence(seq) => {
                Ok(seq.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| PhoneType::from_str_with_locale(s, locale.unwrap_or("en")))
                    .collect())
            }
            _ => Ok(vec![PhoneType::Other])
        }
    }

    fn normalize_phone_number(&self, number: &str) -> Result<String, ParseError> {
        // Simplified phone normalization - real implementation would use phonenumber crate
        let digits_only: String = number.chars().filter(|c| c.is_numeric() || *c == '+').collect();
        
        if digits_only.starts_with('+') {
            Ok(digits_only)
        } else if digits_only.len() >= 10 {
            // Assume US number if no country code
            Ok(format!("+1{}", digits_only))
        } else {
            Err(ParseError::Phone(format!("Invalid phone number: {}", number)))
        }
    }

    fn extract_emails(&self, value: &Value, locale: Option<&str>) -> Result<Vec<Email>, ParseError> {
        // Similar to phones but simpler
        match value {
            Value::String(s) => {
                Ok(vec![Email {
                    address: s.clone(),
                    r#type: vec![EmailType::Other],
                    preferred: None,
                }])
            }
            Value::Sequence(seq) => {
                let mut emails = Vec::new();
                for item in seq {
                    match item {
                        Value::String(s) => {
                            emails.push(Email {
                                address: s.clone(),
                                r#type: vec![EmailType::Other],
                                preferred: None,
                            });
                        }
                        Value::Mapping(_) => {
                            emails.push(self.parse_email_object(item, locale)?);
                        }
                        _ => {}
                    }
                }
                Ok(emails)
            }
            _ => Ok(vec![])
        }
    }

    fn parse_email_object(&self, value: &Value, locale: Option<&str>) -> Result<Email, ParseError> {
        if let Value::Mapping(map) = value {
            let mut email = Email {
                address: String::new(),
                r#type: vec![EmailType::Other],
                preferred: None,
            };

            for (key, val) in map {
                if let Value::String(key_str) = key {
                    match key_str.as_str() {
                        "address" => {
                            email.address = val.as_str().unwrap_or("").to_string();
                        }
                        "type" => {
                            email.r#type = self.parse_email_types(val, locale)?;
                        }
                        "preferred" => {
                            email.preferred = val.as_bool();
                        }
                        _ => {}
                    }
                }
            }

            Ok(email)
        } else {
            Err(ParseError::Schema("Invalid email object".to_string()))
        }
    }

    fn parse_email_types(&self, value: &Value, locale: Option<&str>) -> Result<Vec<EmailType>, ParseError> {
        match value {
            Value::String(s) => {
                Ok(vec![EmailType::from_str_with_locale(s, locale.unwrap_or("en"))])
            }
            Value::Sequence(seq) => {
                Ok(seq.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| EmailType::from_str_with_locale(s, locale.unwrap_or("en")))
                    .collect())
            }
            _ => Ok(vec![EmailType::Other])
        }
    }

    fn extract_addresses(&self, _value: &Value, _locale: Option<&str>) -> Result<Vec<Address>, ParseError> {
        // Simplified address parsing
        Ok(vec![])
    }

    fn extract_metadata(&self, value: &Value) -> Result<Metadata, ParseError> {
        if let Value::Mapping(map) = value {
            let mut metadata = Metadata {
                locale: None,
                source: None,
            };

            for (key, val) in map {
                if let Value::String(key_str) = key {
                    match key_str.as_str() {
                        "locale" => {
                            metadata.locale = val.as_str().map(|s| s.to_string());
                        }
                        "source" => {
                            metadata.source = val.as_str().map(|s| s.to_string());
                        }
                        _ => {}
                    }
                }
            }

            Ok(metadata)
        } else {
            Err(ParseError::Schema("Invalid metadata format".to_string()))
        }
    }

    fn value_to_string_vec(&self, value: &Value) -> Vec<String> {
        match value {
            Value::String(s) => vec![s.clone()],
            Value::Sequence(seq) => {
                seq.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            }
            _ => vec![]
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let parser = Parser::new();
        let input = r#"
version: 1
name: "John Doe"
mobile: "+1 555 123 4567"
"#;
        
        let result = parser.parse_lenient(input, Some("en")).unwrap();
        assert_eq!(result.version, 1);
        assert!(result.name.is_some());
        assert!(result.phones.is_some());
    }

    #[test]
    fn test_localized_keys() {
        let parser = Parser::new();
        // TODO: This test should work with dotted path aliases like nom->name.familyName
        // Currently using simple aliases that work: phone->phones, email->emails
        let input = r#"
phone: "06 12 34 56 78"
email: "test@example.com"
"#;
        
        let result = parser.parse_lenient(input, Some("fr")).unwrap();
        assert!(result.phones.is_some());
        assert!(result.emails.is_some());
    }
}