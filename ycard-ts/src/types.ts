/**
 * TypeScript definitions for yCard schema
 */

export interface YCard {
  version: number;
  uid?: string;
  name?: Name;
  phones?: Phone[];
  emails?: Email[];
  addresses?: Address[];
  metadata?: Metadata;
}

export interface Name {
  givenName?: string[];
  middleName?: string[];
  familyName?: string[];
  honorificPrefix?: string[];
  honorificSuffix?: string[];
  displayName?: string;
  script?: string;
}

export interface Phone {
  number: string; // E.164 format
  type: PhoneType[];
  ext?: string;
  preferred?: boolean;
  label?: string;
}

export type PhoneType = 
  | 'home' 
  | 'work' 
  | 'mobile' 
  | 'fax' 
  | 'pager' 
  | 'main' 
  | 'other'
  | { custom: string };

export interface Email {
  address: string;
  type: EmailType[];
  preferred?: boolean;
}

export type EmailType = 
  | 'home' 
  | 'work' 
  | 'other'
  | { custom: string };

export interface Address {
  type: AddressType[];
  formatted?: string;
  components?: AddressComponents;
}

export type AddressType = 
  | 'home' 
  | 'work' 
  | 'other'
  | { custom: string };

export interface AddressComponents {
  street?: string;
  locality?: string;
  region?: string;
  postalCode?: string;
  country?: string; // ISO-3166 alpha-2
}

export interface Metadata {
  locale?: string; // BCP-47
  source?: string;
}

export interface Diagnostic {
  level: DiagnosticLevel;
  message: string;
  code?: string;
  range?: Range;
  fixes: CodeFix[];
}

export enum DiagnosticLevel {
  Error = 'error',
  Warning = 'warning',
  Info = 'info',
  Hint = 'hint'
}

export interface Range {
  start: Position;
  end: Position;
}

export interface Position {
  line: number;
  character: number;
}

export interface CodeFix {
  title: string;
  kind: string;
  edit: TextEdit;
}

export interface TextEdit {
  range: Range;
  newText: string;
}

export enum ValidationMode {
  Lenient = 0,
  Strict = 1,
  SchemaOnly = 2
}

export enum PhonesStyle {
  Canonical = 'canonical',
  Shorthand = 'shorthand',
  Auto = 'auto'
}