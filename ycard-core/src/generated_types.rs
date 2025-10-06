// Generated from schema.json - DO NOT EDIT MANUALLY
// Run `node generate-code.js` to regenerate

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailType {
    Home,
    Work,
    Other,
    #[serde(untagged)]
    Custom(String),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    Home,
    Work,
    Other,
    #[serde(untagged)]
    Custom(String),
}

impl AddressType {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "home" | "casa" | "domicile" | "自宅" => AddressType::Home,
            "work" | "trabajo" | "travail" | "bureau" | "勤務" => AddressType::Work,
            "other" | "otro" | "autre" | "その他" => AddressType::Other,
            _ => AddressType::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
    Hint,
    #[serde(untagged)]
    Custom(String),
}

impl DiagnosticLevel {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "error" | "err" => DiagnosticLevel::Error,
            "warning" | "warn" => DiagnosticLevel::Warning,
            "info" | "information" => DiagnosticLevel::Info,
            "hint" | "suggestion" => DiagnosticLevel::Hint,
            _ => DiagnosticLevel::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhonesStyle {
    Canonical,
    Shorthand,
    Auto,
    #[serde(untagged)]
    Custom(String),
}

impl PhonesStyle {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "canonical" | "full" | "structured" => PhonesStyle::Canonical,
            "shorthand" | "short" | "compact" => PhonesStyle::Shorthand,
            "auto" | "automatic" | "smart" => PhonesStyle::Auto,
            _ => PhonesStyle::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationMode {
    Lenient,
    Strict,
    #[serde(untagged)]
    Custom(String),
}

impl ValidationMode {
    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {
        match s.to_lowercase().as_str() {
            "lenient" | "relaxed" | "permissive" => ValidationMode::Lenient,
            "strict" | "rigid" | "exact" => ValidationMode::Strict,
            _ => ValidationMode::Custom(s.to_string()),
        }
    }
}

// Shorthand field mappings
pub const PHONE_SHORTHAND_KEYS: &[&str] = &[
    "mobile",
    "cell",
    "móvil",
    "portable",
    "携帯",
    "home",
    "casa",
    "domicile",
    "自宅",
    "work",
    "trabajo",
    "travail",
    "bureau",
    "勤務",
    "fax",
    "pager",
    "main",
    "principal",
];

pub const EMAIL_SHORTHAND_KEYS: &[&str] = &[
    "email",
];

