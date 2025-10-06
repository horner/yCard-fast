# yCard Specification v1

## Overview

yCard is a human-friendly, YAML-like contact format designed for the YABL ecosystem. It provides a balance between human readability and machine parseability, with comprehensive internationalization support.

## Design Principles

1. **Human-first**: Readable and writable by humans
2. **Lenient ingest**: Accept common variations and shortcuts
3. **Canonical output**: Emit consistent, normalized format
4. **International**: Support 16+ languages with aliases
5. **WASM-first**: Single artifact runs everywhere

## Schema

### Core Structure

```yaml
version: 1                    # Required, always 1 for v1
uid?: string                  # Optional unique identifier
name?: Name                   # Contact name information
phones?: Phone[]              # Phone numbers
emails?: Email[]              # Email addresses  
addresses?: Address[]         # Physical addresses
metadata?: Metadata           # Processing metadata
```

### Name Object

```yaml
name:
  givenName?: string[]        # Given names (first names)
  middleName?: string[]       # Middle names
  familyName?: string[]       # Family names (surnames)
  honorificPrefix?: string[]  # Titles (Dr., Ms., etc.)
  honorificSuffix?: string[]  # Suffixes (Jr., III, etc.)
  displayName?: string        # Full display name
  script?: string             # Script tag (Latn, Hani, etc.)
```

### Phone Object

```yaml
phones:
  - number: string            # E.164 format (+15551234567)
    type: PhoneType[]         # [home, work, mobile, fax, pager, main, other]
    ext?: string              # Extension
    preferred?: boolean       # Primary phone
    label?: string            # Custom label
```

### Email Object

```yaml
emails:
  - address: string           # Email address
    type: EmailType[]         # [home, work, other]
    preferred?: boolean       # Primary email
```

### Address Object

```yaml
addresses:
  - type: AddressType[]       # [home, work, other]
    formatted?: string        # Display format
    components?:              # Structured components
      street?: string
      locality?: string       # City
      region?: string         # State/Province
      postalCode?: string     # ZIP/Postal code
      country?: string        # ISO-3166 alpha-2 (US, GB, etc.)
```

### Metadata Object

```yaml
metadata:
  locale?: string             # BCP-47 locale (en-US, fr-CA, etc.)
  source?: string             # Original source
```

## Shorthand Syntax

yCard supports shorthand fields that expand to full `phones` arrays:

```yaml
# Shorthand
mobile: "+1 555 123 4567"
home: "+1 555 987 6543"

# Expands to:
phones:
  - number: "+15551234567"
    type: [mobile]
    preferred: true
  - number: "+15559876543"
    type: [home]
```

Supported shorthand keys:
- `mobile`, `cell` → `phones[].type=[mobile]`
- `home` → `phones[].type=[home]`
- `work` → `phones[].type=[work]`
- `fax` → `phones[].type=[fax]`
- `pager` → `phones[].type=[pager]`
- `main` → `phones[].type=[main]`

## Internationalization

### Alias System

yCard supports localized field names through a hierarchical alias system:

```yaml
# French aliases
nom: "Dupont"           # → name.familyName
prénom: "Jean"          # → name.givenName  
portable: "06 12 34 56" # → phones[].type=[mobile]
```

### Locale Chain Resolution

Aliases follow BCP-47 fallback chains:
- `fr-CA` → `fr` → `root`
- `zh-Hans-CN` → `zh-Hans` → `zh` → `root`

### Supported Locales (v1)

Base support for 16+ locales:
- **en** (English)
- **fr** (French)
- **es** (Spanish)
- **de** (German)
- **ja** (Japanese)
- **zh** (Chinese)
- **pt** (Portuguese)
- **it** (Italian)
- **ru** (Russian)
- **ko** (Korean)
- **ar** (Arabic)
- **hi** (Hindi)
- **th** (Thai)
- **vi** (Vietnamese)
- **tr** (Turkish)
- **pl** (Polish)

## Processing Modes

### Lenient Mode (Default)

- Accept variations in whitespace, casing, diacritics
- Map localized keys via alias tables
- Normalize phone numbers to E.164
- Coerce types (string → array when needed)
- Generate warnings for non-canonical input

### Strict Mode

- Require exact canonical format
- Reject unknown fields
- All normalization warnings become errors
- Require E.164 phone numbers
- Validate email addresses

### Schema-Only Mode

- Only validate against canonical schema
- Skip content validation (phones, emails, etc.)
- Useful for structural checks

## Markdown Integration

### Fenced Code Blocks

````markdown
```ycard
name: Jane Doe
mobile: 555-123-4567
```
````

### Contextual Blocks

```markdown
## Contact
name: John Smith
home: +44 20 7946 0958
```

Supported headings (case-insensitive):
- English: Contact, Contacts
- French: Contact, Contacts
- German: Kontakt
- Spanish: Contacto, Contactos
- Japanese: 連絡先
- Chinese: 连络

## Examples

### Basic Contact

```yaml
version: 1
name: "Jane Doe"
mobile: "+1 555 123 4567"
email: "jane@example.com"
```

### Structured Contact

```yaml
version: 1
name:
  givenName: [Jane]
  familyName: [Doe]
  displayName: "Jane Doe"
phones:
  - number: "+15551234567"
    type: [mobile]
    preferred: true
  - number: "+15559876543"
    type: [work]
    ext: "123"
emails:
  - address: "jane@personal.com"
    type: [home]
    preferred: true
  - address: "j.doe@company.com"
    type: [work]
addresses:
  - type: [home]
    components:
      street: "123 Main St"
      locality: "Anytown"
      region: "CA"
      postalCode: "12345"
      country: "US"
metadata:
  locale: "en-US"
```

### Localized (French)

```yaml
nom: "Dupont"
prénom: "Jean"
portable: "06 12 34 56 78"
bureau: "+33 1 23 45 67 89"
adresse: "123 Rue de la Paix, Paris"
```

Canonicalizes to:

```yaml
version: 1
name:
  familyName: [Dupont]
  givenName: [Jean]
  displayName: "Jean Dupont"
phones:
  - number: "+33612345678"
    type: [mobile]
    preferred: true
  - number: "+33123456789"
    type: [work]
addresses:
  - formatted: "123 Rue de la Paix, Paris"
    type: [home]
metadata:
  locale: "fr"
```

## Validation Rules

### Required Fields

- `version` must be present and equal to 1
- At least one of `name`, `phones`, or `emails` must be present (strict mode)

### Phone Number Rules

- Must be valid E.164 format in canonical output
- Extensions extracted from patterns like "ext. 123", "x123"
- Country codes inferred based on locale when missing

### Email Rules

- Must contain `@` character
- Domain part should be valid (basic check)
- Local part preserves case sensitivity

### Address Rules

- `country` field should be ISO-3166 alpha-2 code
- Components validated based on country-specific rules (future)

## Error Handling

### Parse Errors

```json
{
  "level": "error",
  "message": "Invalid YAML syntax at line 5",
  "code": "yaml-syntax-error",
  "range": {
    "start": {"line": 4, "character": 0},
    "end": {"line": 4, "character": 20}
  }
}
```

### Validation Warnings

```json
{
  "level": "warning", 
  "message": "Phone number normalized to E.164: +15551234567",
  "code": "phone-normalized",
  "fixes": [
    {
      "title": "Use E.164 format",
      "kind": "quickfix",
      "edit": {
        "range": {"start": {"line": 2, "character": 8}, "end": {"line": 2, "character": 20}},
        "newText": "\"+15551234567\""
      }
    }
  ]
}
```

## Performance Targets

- Parse 1,000 small contacts (<2KB) in ≤300ms
- Single large contact (50KB) in ≤50ms native, ≤90ms WASM
- LSP response time ≤40ms for 95th percentile

## Security Considerations

- Input size limits (configurable)
- Unicode normalization to prevent spoofing
- Timeout limits for WASM execution
- No arbitrary code execution