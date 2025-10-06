// TypeScript wrapper to bridge yCard WASM exports to LSP server API expectations
import * as wasm from '../../ycard-core/pkg/ycard_core.js';

// Export the main parsing functions with expected API
export function parse(input: string, locale?: string): Promise<any> {
    return Promise.resolve(wasm.parse_ycard_lenient(input, locale));
}

export function setDefaultLocale(locale: string): Promise<void> {
    return Promise.resolve(wasm.set_default_locale(locale));
}

export function loadAliasPack(content: string): Promise<void> {
    return Promise.resolve(wasm.load_alias_pack(content));
}

export function validate(ycard: any, mode?: any): Promise<any[]> {
    // Convert yCard object to JSON string
    const jsonStr = JSON.stringify(ycard);
    
    // Determine validation mode
    const modeStr = (mode === ValidationMode.Strict) ? 'strict' : 'lenient';
    
    return Promise.resolve(wasm.validate_ycard(jsonStr, modeStr));
}

export function format(ycard: any, phonesStyle?: any): Promise<string> {
    // Convert yCard object to JSON string  
    const jsonStr = JSON.stringify(ycard);
    
    // Determine phones style
    let styleStr = 'canonical';
    if (phonesStyle === PhonesStyle.Shorthand) {
        styleStr = 'shorthand';
    } else if (phonesStyle === PhonesStyle.Auto) {
        styleStr = 'auto';
    }
    
    return Promise.resolve(wasm.format_ycard(jsonStr, styleStr));
}

// Export enums expected by LSP server
export const ValidationMode = {
    Lenient: 'lenient',
    Strict: 'strict'
} as const;

export const DiagnosticLevel = {
    Error: 'Error',
    Warning: 'Warning', 
    Info: 'Info',
    Hint: 'Hint'
} as const;

export const PhonesStyle = {
    Canonical: 'canonical',
    Shorthand: 'shorthand',
    Auto: 'auto'
} as const;

// Export types
export type DiagnosticLevelType = typeof DiagnosticLevel[keyof typeof DiagnosticLevel];