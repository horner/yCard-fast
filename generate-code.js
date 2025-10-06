#!/usr/bin/env node

/**
 * yCard Schema Code Generator
 * Generates all type definitions, grammar rules, and validation logic from schema.json
 */

const fs = require('fs');
const path = require('path');

// Load schema
const schemaPath = path.join(__dirname, 'schema.json');
const schema = JSON.parse(fs.readFileSync(schemaPath, 'utf8'));

console.log('🔄 Generating code from schema.json...');

// Generate Rust enums and types
function generateRustTypes() {
  console.log('  📝 Generating Rust types...');
  
  let rustCode = `// Generated from schema.json - DO NOT EDIT MANUALLY
// Run \`node generate-code.js\` to regenerate

use serde::{Deserialize, Serialize};

`;

  // Generate enums
  for (const [enumName, enumDef] of Object.entries(schema.enums)) {
    rustCode += `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\n`;
    rustCode += `#[serde(rename_all = "lowercase")]\n`;
    rustCode += `pub enum ${enumName} {\n`;
    
    for (const variant of enumDef.variants) {
      const variantName = variant.canonical.charAt(0).toUpperCase() + variant.canonical.slice(1);
      rustCode += `    ${variantName},\n`;
    }
    rustCode += `    #[serde(untagged)]\n`;
    rustCode += `    Custom(String),\n}\n\n`;

    // Generate from_str_with_locale method
    rustCode += `impl ${enumName} {\n`;
    rustCode += `    pub fn from_str_with_locale(s: &str, _locale: &str) -> Self {\n`;
    rustCode += `        match s.to_lowercase().as_str() {\n`;
    
    for (const variant of enumDef.variants) {
      const allNames = [variant.canonical, ...variant.aliases];
      // Remove duplicates to avoid unreachable patterns
      const uniqueNames = [...new Set(allNames.map(name => name.toLowerCase()))];
      const matchPattern = uniqueNames.map(name => `"${name}"`).join(' | ');
      const variantName = variant.canonical.charAt(0).toUpperCase() + variant.canonical.slice(1);
      rustCode += `            ${matchPattern} => ${enumName}::${variantName},\n`;
    }
    
    rustCode += `            _ => ${enumName}::Custom(s.to_string()),\n`;
    rustCode += `        }\n    }\n}\n\n`;
  }

  // Generate shorthand constants
  rustCode += `// Shorthand field mappings\n`;
  rustCode += `pub const PHONE_SHORTHAND_KEYS: &[&str] = &[\n`;
  for (const shorthand of schema.shorthandFields.phoneShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      rustCode += `    "${key}",\n`;
    }
  }
  rustCode += `];\n\n`;

  rustCode += `pub const EMAIL_SHORTHAND_KEYS: &[&str] = &[\n`;
  for (const shorthand of schema.shorthandFields.emailShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      rustCode += `    "${key}",\n`;
    }
  }
  rustCode += `];\n\n`;

  // Write to file
  fs.writeFileSync(path.join(__dirname, 'ycard-core/src/generated_types.rs'), rustCode);
  console.log('    ✅ ycard-core/src/generated_types.rs');
}

// Generate TypeScript types
function generateTypeScriptTypes() {
  console.log('  📝 Generating TypeScript types...');
  
  let tsCode = `// Generated from schema.json - DO NOT EDIT MANUALLY
// Run \`node generate-code.js\` to regenerate

`;

  // Generate enums
  for (const [enumName, enumDef] of Object.entries(schema.enums)) {
    tsCode += `export type ${enumName} = \n`;
    
    for (let i = 0; i < enumDef.variants.length; i++) {
      const variant = enumDef.variants[i];
      tsCode += `  | '${variant.canonical}'`;
      if (i < enumDef.variants.length - 1) tsCode += '\n';
    }
    tsCode += `\n  | { custom: string };\n\n`;
  }

  // Generate constants for LSP/wrapper usage
  tsCode += `// Enum constants for compatibility\n`;
  for (const [enumName, enumDef] of Object.entries(schema.enums)) {
    tsCode += `export const ${enumName}Values = {\n`;
    for (const variant of enumDef.variants) {
      const propName = variant.canonical.charAt(0).toUpperCase() + variant.canonical.slice(1);
      tsCode += `  ${propName}: '${variant.canonical}',\n`;
    }
    tsCode += `} as const;\n\n`;
  }

  // Generate shorthand arrays
  tsCode += `// Shorthand field arrays\n`;
  tsCode += `export const PHONE_SHORTHAND_KEYS = [\n`;
  for (const shorthand of schema.shorthandFields.phoneShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      tsCode += `  '${key}',\n`;
    }
  }
  tsCode += `] as const;\n\n`;

  tsCode += `export const EMAIL_SHORTHAND_KEYS = [\n`;
  for (const shorthand of schema.shorthandFields.emailShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      tsCode += `  '${key}',\n`;
    }
  }
  tsCode += `] as const;\n\n`;

  // Write to file
  fs.writeFileSync(path.join(__dirname, 'ycard-ts/src/generated_types.ts'), tsCode);
  console.log('    ✅ ycard-ts/src/generated_types.ts');
}

// Generate Tree-sitter grammar rules
function generateGrammarRules() {
  console.log('  📝 Generating Tree-sitter grammar rules...');
  
  let grammarCode = `// Generated from schema.json - DO NOT EDIT MANUALLY
// Run \`node generate-code.js\` to regenerate

// Phone shorthand keys
const PHONE_SHORTHAND_KEYS = [
`;

  for (const shorthand of schema.shorthandFields.phoneShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      grammarCode += `  '${key}',\n`;
    }
  }
  
  grammarCode += `];

// Email shorthand keys  
const EMAIL_SHORTHAND_KEYS = [
`;

  for (const shorthand of schema.shorthandFields.emailShorthands) {
    const allKeys = [shorthand.key, ...shorthand.aliases];
    for (const key of allKeys) {
      grammarCode += `  '${key}',\n`;
    }
  }

  grammarCode += `];

// Export for use in grammar.js
module.exports = {
  PHONE_SHORTHAND_KEYS,
  EMAIL_SHORTHAND_KEYS
};
`;

  // Write to file
  fs.writeFileSync(path.join(__dirname, 'ycard-grammar/generated_keys.js'), grammarCode);
  console.log('    ✅ ycard-grammar/generated_keys.js');
}

// Generate LSP completion items
function generateLSPCompletions() {
  console.log('  📝 Generating LSP completion items...');
  
  let completionCode = `// Generated from schema.json - DO NOT EDIT MANUALLY
// Run \`node generate-code.js\` to regenerate

import { CompletionItem, CompletionItemKind } from 'vscode-languageserver/node';

export const YCARD_COMPLETION_ITEMS: CompletionItem[] = [
`;

  let dataIndex = 1;

  // Core fields
  for (const field of schema.schema.fields) {
    if (field.name === 'version') continue; // Skip version as it's always required
    
    completionCode += `  {
    label: '${field.name}',
    kind: CompletionItemKind.Property,
    data: ${dataIndex++},
    detail: '${field.description}',
    insertText: '${field.name}: '
  },\n`;
  }

  // Phone shorthands
  for (const shorthand of schema.shorthandFields.phoneShorthands) {
    completionCode += `  {
    label: '${shorthand.key}',
    kind: CompletionItemKind.Property,
    data: ${dataIndex++},
    detail: '${shorthand.defaultType.charAt(0).toUpperCase() + shorthand.defaultType.slice(1)} phone (shorthand)',
    insertText: '${shorthand.key}: "+1 555 123 4567"'
  },\n`;
  }

  // Email shorthands
  for (const shorthand of schema.shorthandFields.emailShorthands) {  
    completionCode += `  {
    label: '${shorthand.key}',
    kind: CompletionItemKind.Property,
    data: ${dataIndex++},
    detail: 'Email address (shorthand)',
    insertText: '${shorthand.key}: "example@domain.com"'
  },\n`;
  }

  completionCode += `];
`;

  // Write to file
  fs.writeFileSync(path.join(__dirname, 'ycard-lsp/src/generated_completions.ts'), completionCode);
  console.log('    ✅ ycard-lsp/src/generated_completions.ts');
}

// Generate diagnostic codes
function generateDiagnosticCodes() {
  console.log('  📝 Generating diagnostic codes...');
  
  let rustCode = `// Generated from schema.json - DO NOT EDIT MANUALLY
// Run \`node generate-code.js\` to regenerate

use crate::generated_types::DiagnosticLevel;

pub struct DiagnosticCode {
    pub code: &'static str,
    pub level: DiagnosticLevel,
    pub message: &'static str,
}

pub const DIAGNOSTIC_CODES: &[DiagnosticCode] = &[
`;

  for (const diag of schema.diagnosticCodes.codes) {
    rustCode += `    DiagnosticCode {
        code: "${diag.code}",
        level: DiagnosticLevel::${diag.level},
        message: "${diag.message}",
    },\n`;
  }

  rustCode += `];
`;

  fs.writeFileSync(path.join(__dirname, 'ycard-core/src/generated_diagnostics.rs'), rustCode);
  console.log('    ✅ ycard-core/src/generated_diagnostics.rs');
}

// Run all generators
function main() {
  try {
    generateRustTypes();
    generateTypeScriptTypes();
    generateGrammarRules();
    generateLSPCompletions();
    generateDiagnosticCodes();
    
    console.log('\n✅ Code generation complete!');
    console.log('📋 Next steps:');
    console.log('  1. Update imports in existing files to use generated types');
    console.log('  2. Remove old hardcoded definitions');
    console.log('  3. Run tests to ensure everything works');
    
  } catch (error) {
    console.error('❌ Code generation failed:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

module.exports = { generateRustTypes, generateTypeScriptTypes, generateGrammarRules, generateLSPCompletions, generateDiagnosticCodes };