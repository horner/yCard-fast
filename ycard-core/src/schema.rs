use crate::generated_types::{AddressType, EmailType, PhoneType};
use serde::{Deserialize, Serialize};

/// yCard canonical schema types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YCard {
    pub version: u8,
    pub uid: Option<String>,
    pub name: Option<Name>,
    pub phones: Option<Vec<Phone>>,
    pub emails: Option<Vec<Email>>,
    pub addresses: Option<Vec<Address>>,
    pub manager: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Name {
    #[serde(rename = "givenName")]
    pub given_name: Option<Vec<String>>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<Vec<String>>,
    #[serde(rename = "familyName")]
    pub family_name: Option<Vec<String>>,
    #[serde(rename = "honorificPrefix")]
    pub honorific_prefix: Option<Vec<String>>,
    #[serde(rename = "honorificSuffix")]
    pub honorific_suffix: Option<Vec<String>>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub script: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Phone {
    pub number: String, // E.164 format
    pub r#type: Vec<PhoneType>,
    pub ext: Option<String>,
    pub preferred: Option<bool>,
    pub label: Option<String>,
}

// PhoneType now imported from generated_types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Email {
    pub address: String,
    pub r#type: Vec<EmailType>,
    pub preferred: Option<bool>,
}

// EmailType now imported from generated_types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub r#type: Vec<AddressType>,
    pub formatted: Option<String>,
    pub components: Option<AddressComponents>,
}

// AddressType now imported from generated_types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddressComponents {
    pub street: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub country: Option<String>, // ISO-3166 alpha-2
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub locale: Option<String>, // BCP-47
    pub source: Option<String>,
}

impl Default for YCard {
    fn default() -> Self {
        Self {
            version: 1,
            uid: None,
            name: None,
            phones: None,
            emails: None,
            addresses: None,
            manager: None,
            metadata: None,
        }
    }
}

// from_str_with_locale implementations now in generated_types
