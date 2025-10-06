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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PhoneType {
    Home,
    Work,
    Mobile,
    Fax,
    Pager,
    Main,
    Other,
    #[serde(untagged)]
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Email {
    pub address: String,
    pub r#type: Vec<EmailType>,
    pub preferred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EmailType {
    Home,
    Work,
    Other,
    #[serde(untagged)]
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub r#type: Vec<AddressType>,
    pub formatted: Option<String>,
    pub components: Option<AddressComponents>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    Home,
    Work,
    Other,
    #[serde(untagged)]
    Custom(String),
}

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
            metadata: None,
        }
    }
}

impl PhoneType {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "home" | "casa" | "domicile" | "自宅" => PhoneType::Home,
            "work" | "trabajo" | "travail" | "bureau" | "勤務" => PhoneType::Work,
            "mobile" | "cell" | "móvil" | "portable" | "携帯" => PhoneType::Mobile,
            "fax" => PhoneType::Fax,
            "pager" => PhoneType::Pager,
            "main" | "principal" => PhoneType::Main,
            "other" | "otro" | "autre" | "その他" => PhoneType::Other,
            _ => PhoneType::Custom(s.to_string()),
        }
    }
}

impl EmailType {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "home" | "casa" | "domicile" | "自宅" => EmailType::Home,
            "work" | "trabajo" | "travail" | "bureau" | "勤務" => EmailType::Work,
            "other" | "otro" | "autre" | "その他" => EmailType::Other,
            _ => EmailType::Custom(s.to_string()),
        }
    }
}