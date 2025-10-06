pub mod schema;
pub mod parser;
pub mod formatter;
pub mod validator;
pub mod i18n;
pub mod wasm;

pub use schema::*;
pub use parser::{Parser, ParseError};
pub use formatter::{Formatter, PhonesStyle};
pub use validator::{Validator, ValidationMode, Diagnostic, DiagnosticLevel};
pub use i18n::{AliasManager, AliasPack, LocaleData};

// Re-export main functionality
pub fn parse(input: &str, locale: Option<&str>) -> Result<YCard, ParseError> {
    let parser = Parser::new();
    parser.parse_lenient(input, locale)
}

pub fn parse_strict(input: &str) -> Result<YCard, ParseError> {
    let parser = Parser::new();
    parser.parse_strict(input)
}

pub fn format(ycard: &YCard) -> Result<String, serde_yaml::Error> {
    let formatter = Formatter::new();
    formatter.format(ycard)
}

pub fn validate(ycard: &YCard, mode: ValidationMode) -> Result<Vec<Diagnostic>, validator::ValidationError> {
    let validator = Validator::new(mode);
    validator.validate(ycard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end() {
        let input = r#"
version: 1
name: "Jane Doe"
mobile: "+1 555 123 4567"
email: "jane@example.com"
"#;

        // Parse
        let ycard = parse(input, Some("en")).unwrap();
        assert_eq!(ycard.version, 1);
        assert!(ycard.name.is_some());
        assert!(ycard.phones.is_some());
        assert!(ycard.emails.is_some());

        // Format
        let formatted = format(&ycard).unwrap();
        assert!(formatted.contains("version: 1"));

        // Validate
        let diagnostics = validate(&ycard, ValidationMode::Lenient).unwrap();
        // Should have some warnings about normalization
    }

    #[test]
    fn test_localized_input() {
        let input = r#"
nom: "Dupont"
prénom: "Jean"
portable: "06 12 34 56 78"
"#;

        let ycard = parse(input, Some("fr")).unwrap();
        assert!(ycard.name.is_some());
        assert!(ycard.phones.is_some());
    }
}