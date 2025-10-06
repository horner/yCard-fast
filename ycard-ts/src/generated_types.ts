// Generated from schema.json - DO NOT EDIT MANUALLY
// Run `node generate-code.js` to regenerate

export type PhoneType = 
  | 'home'
  | 'work'
  | 'mobile'
  | 'fax'
  | 'pager'
  | 'main'
  | 'other'
  | { custom: string };

export type EmailType = 
  | 'home'
  | 'work'
  | 'other'
  | { custom: string };

export type AddressType = 
  | 'home'
  | 'work'
  | 'other'
  | { custom: string };

export type DiagnosticLevel = 
  | 'Error'
  | 'Warning'
  | 'Info'
  | 'Hint'
  | { custom: string };

export type PhonesStyle = 
  | 'canonical'
  | 'shorthand'
  | 'auto'
  | { custom: string };

export type ValidationMode = 
  | 'lenient'
  | 'strict'
  | { custom: string };

// Enum constants for compatibility
export const PhoneTypeValues = {
  Home: 'home',
  Work: 'work',
  Mobile: 'mobile',
  Fax: 'fax',
  Pager: 'pager',
  Main: 'main',
  Other: 'other',
} as const;

export const EmailTypeValues = {
  Home: 'home',
  Work: 'work',
  Other: 'other',
} as const;

export const AddressTypeValues = {
  Home: 'home',
  Work: 'work',
  Other: 'other',
} as const;

export const DiagnosticLevelValues = {
  Error: 'Error',
  Warning: 'Warning',
  Info: 'Info',
  Hint: 'Hint',
} as const;

export const PhonesStyleValues = {
  Canonical: 'canonical',
  Shorthand: 'shorthand',
  Auto: 'auto',
} as const;

export const ValidationModeValues = {
  Lenient: 'lenient',
  Strict: 'strict',
} as const;

// Shorthand field arrays
export const PHONE_SHORTHAND_KEYS = [
  'mobile',
  'cell',
  'móvil',
  'portable',
  '携帯',
  'home',
  'casa',
  'domicile',
  '自宅',
  'work',
  'trabajo',
  'travail',
  'bureau',
  '勤務',
  'fax',
  'pager',
  'main',
  'principal',
] as const;

export const EMAIL_SHORTHAND_KEYS = [
  'email',
] as const;

