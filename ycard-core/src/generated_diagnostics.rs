// Generated from schema.json - DO NOT EDIT MANUALLY
// Run `node generate-code.js` to regenerate

use crate::generated_types::DiagnosticLevel;

pub struct DiagnosticCode {
    pub code: &'static str,
    pub level: DiagnosticLevel,
    pub message: &'static str,
}

pub const DIAGNOSTIC_CODES: &[DiagnosticCode] = &[
    DiagnosticCode {
        code: "phone-normalized",
        level: DiagnosticLevel::Warning,
        message: "Phone number normalized to E.164 format",
    },
    DiagnosticCode {
        code: "shorthand-expanded",
        level: DiagnosticLevel::Info,
        message: "Shorthand field expanded to structured form",
    },
    DiagnosticCode {
        code: "phone-format",
        level: DiagnosticLevel::Error,
        message: "Phone number should be in E.164 format",
    },
    DiagnosticCode {
        code: "email-invalid",
        level: DiagnosticLevel::Error,
        message: "Invalid email address",
    },
    DiagnosticCode {
        code: "empty-contact",
        level: DiagnosticLevel::Error,
        message: "At least one of name, phones, or emails must be present",
    },
];
