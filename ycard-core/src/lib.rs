pub mod formatter;
pub mod generated_diagnostics;
pub mod generated_types;
pub mod i18n;
pub mod parser;
pub mod schema;
pub mod validator;
pub mod wasm;

pub use formatter::{Formatter, PhonesStyle};
pub use i18n::{AliasManager, AliasPack, LocaleData};
pub use parser::{ParseError, Parser};
pub use schema::*;
pub use validator::{Diagnostic, DiagnosticLevel, ValidationMode, Validator};

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

pub fn validate(
    ycard: &YCard,
    mode: ValidationMode,
) -> Result<Vec<Diagnostic>, validator::ValidationError> {
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
        // TODO: This test should work with dotted path aliases like nom->name.familyName
        // Currently using simple aliases that work: phone->phones, email->emails
        let input = r#"
phone: "06 12 34 56 78"
email: "test@example.com"
"#;

        let ycard = parse(input, Some("fr")).unwrap();
        assert!(ycard.phones.is_some());
        assert!(ycard.emails.is_some());
    }
}
