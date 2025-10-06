use crate::schema::YCard;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Required field missing: {field}")]
    MissingRequired { field: String },
    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },
    #[error("Schema violation: {message}")]
    SchemaViolation { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub code: Option<String>,
    pub range: Option<Range>,
    pub fixes: Vec<CodeFix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFix {
    pub title: String,
    pub kind: String,
    pub edit: TextEdit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone)]
pub enum ValidationMode {
    Lenient,
    Strict,
    SchemaOnly,
}

pub struct Validator {
    mode: ValidationMode,
}

impl Validator {
    pub fn new(mode: ValidationMode) -> Self {
        Self { mode }
    }

    /// Validate yCard and return diagnostics
    pub fn validate(&self, ycard: &YCard) -> Result<Vec<Diagnostic>, ValidationError> {
        let mut diagnostics = Vec::new();

        match self.mode {
            ValidationMode::Lenient => {
                self.validate_lenient(ycard, &mut diagnostics)?;
            }
            ValidationMode::Strict => {
                self.validate_strict(ycard, &mut diagnostics)?;
            }
            ValidationMode::SchemaOnly => {
                self.validate_schema_only(ycard, &mut diagnostics)?;
            }
        }

        Ok(diagnostics)
    }

    fn validate_lenient(&self, ycard: &YCard, diagnostics: &mut Vec<Diagnostic>) -> Result<(), ValidationError> {
        // Check basic structure
        if ycard.version == 0 {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: "Version should be specified (defaulting to 1)".to_string(),
                code: Some("version-missing".to_string()),
                range: None,
                fixes: vec![CodeFix {
                    title: "Add version: 1".to_string(),
                    kind: "quickfix".to_string(),
                    edit: TextEdit {
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: 0, character: 0 },
                        },
                        new_text: "version: 1\n".to_string(),
                    },
                }],
            });
        }

        // Validate phones
        if let Some(phones) = &ycard.phones {
            for (_i, phone) in phones.iter().enumerate() {
                if !phone.number.starts_with('+') {
                    diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: format!("Phone number should be in E.164 format: {}", phone.number),
                        code: Some("phone-format".to_string()),
                        range: None,
                        fixes: vec![],
                    });
                }
            }
        }

        // Validate emails
        if let Some(emails) = &ycard.emails {
            for email in emails {
                if !email.address.contains('@') {
                    diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("Invalid email address: {}", email.address),
                        code: Some("email-invalid".to_string()),
                        range: None,
                        fixes: vec![],
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_strict(&self, ycard: &YCard, diagnostics: &mut Vec<Diagnostic>) -> Result<(), ValidationError> {
        // All lenient validations become errors in strict mode
        self.validate_lenient(ycard, diagnostics)?;

        // Convert warnings to errors
        for diagnostic in diagnostics.iter_mut() {
            if matches!(diagnostic.level, DiagnosticLevel::Warning) {
                diagnostic.level = DiagnosticLevel::Error;
            }
        }

        // Additional strict validations
        if ycard.name.is_none() && ycard.phones.is_none() && ycard.emails.is_none() {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: "At least one of name, phones, or emails must be present".to_string(),
                code: Some("empty-contact".to_string()),
                range: None,
                fixes: vec![],
            });
        }

        Ok(())
    }

    fn validate_schema_only(&self, ycard: &YCard, diagnostics: &mut Vec<Diagnostic>) -> Result<(), ValidationError> {
        // Only validate against the canonical schema structure
        if ycard.version > 1 {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Unsupported version: {}", ycard.version),
                code: Some("version-unsupported".to_string()),
                range: None,
                fixes: vec![],
            });
        }

        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new(ValidationMode::Lenient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;

    #[test]
    fn test_lenient_validation() {
        use crate::schema::Phone;
        use crate::generated_types::PhoneType;
        
        let validator = Validator::new(ValidationMode::Lenient);
        // Create a YCard with bad phone format to trigger lenient validation warning
        let ycard = YCard {
            version: 1,
            uid: None,
            name: None,
            phones: Some(vec![Phone {
                number: "123-456-7890".to_string(), // Bad format - should be E.164
                r#type: vec![PhoneType::Other],
                ext: None,
                preferred: None,
                label: None,
            }]),
            emails: None,
            addresses: None,
            metadata: None,
        };
        
        let diagnostics = validator.validate(&ycard).unwrap();
        // Should generate warning about bad phone format
        assert!(!diagnostics.is_empty());
        assert!(matches!(diagnostics[0].level, DiagnosticLevel::Warning));
    }

    #[test]
    fn test_strict_validation() {
        let validator = Validator::new(ValidationMode::Strict);
        let ycard = YCard::default();
        
        let diagnostics = validator.validate(&ycard).unwrap();
        assert!(!diagnostics.is_empty());
        // Should have error for empty contact
        assert!(diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Error)));
    }
}