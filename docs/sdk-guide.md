# yCard SDK Guide

## TypeScript SDK

### Installation

```bash
npm install @ycard/core
```

### Basic Usage

```typescript
import { parse, format, validate, ValidationMode } from '@ycard/core';

// Parse yCard
const ycard = await parse(`
version: 1
name: Jane Doe  
mobile: 555-123-4567
`, 'en');

// Format to canonical YAML
const formatted = await format(ycard);
console.log(formatted);

// Validate
const diagnostics = await validate(ycard, ValidationMode.Lenient);
console.log(diagnostics);
```

### Loading Custom Aliases

```typescript
import { loadAliasPack, setDefaultLocale } from '@ycard/core';

// Load custom alias pack
const customAliases = {
  "version": "2025.10.05",
  "locales": {
    "en-corp": {
      "keyAliases": {
        "emp_id": "uid",
        "full_name": "name.displayName"
      }
    }
  }
};

await loadAliasPack(JSON.stringify(customAliases));
await setDefaultLocale('en-corp');
```

### Web Browser Usage

```html
<script type="module">
import { parse } from './node_modules/@ycard/core/dist/index.js';

// WASM loads automatically
const result = await parse('name: John Doe\nphone: 555-1234');
console.log(result);
</script>
```

## Python SDK

### Installation

```bash
pip install ycard
```

### Basic Usage

```python
import ycard

# Parse yCard
card = ycard.parse("""
version: 1
name: Jane Doe
mobile: 555-123-4567
""", locale='en')

# Format
formatted = ycard.format(card)
print(formatted)

# Validate
diagnostics = ycard.validate(card, mode='lenient')
for diag in diagnostics:
    print(f"{diag['level']}: {diag['message']}")
```

### Custom Alias Packs

```python
import ycard
import json

# Load custom aliases
with open('custom-aliases.json', 'r') as f:
    aliases = f.read()

ycard.load_alias_pack(aliases)
ycard.set_default_locale('es')

# Now use Spanish aliases
card = ycard.parse("""
nombre: Juan Pérez
teléfono: +34 600 123 456
""")
```

## Go SDK

### Installation

```bash
go get github.com/yabl-lang/ycard-go
```

### Basic Usage

```go
package main

import (
    "context"
    "fmt"
    "log"
    
    "github.com/yabl-lang/ycard-go"
)

func main() {
    ctx := context.Background()
    
    // Initialize yCard engine
    engine, err := ycard.New(ctx)
    if err != nil {
        log.Fatal(err)
    }
    defer engine.Close()
    
    // Parse yCard
    input := `
version: 1
name: Jane Doe
mobile: 555-123-4567
`
    
    card, err := engine.Parse(ctx, input, &ycard.ParseOptions{
        Locale: "en",
        Mode:   ycard.ModeLenient,
    })
    if err != nil {
        log.Fatal(err)
    }
    
    // Format
    formatted, err := engine.Format(ctx, card, &ycard.FormatOptions{
        PhonesStyle: ycard.PhonesStyleCanonical,
    })
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Println(formatted)
}
```

### Custom Aliases

```go
// Load alias pack
aliasData, err := os.ReadFile("aliases.json")
if err != nil {
    log.Fatal(err)
}

err = engine.LoadAliasPack(ctx, aliasData)
if err != nil {
    log.Fatal(err)  
}

err = engine.SetDefaultLocale(ctx, "fr")
if err != nil {
    log.Fatal(err)
}
```

## CLI Usage

### Installation

```bash
# From source
cargo install --path ycard-cli

# Or download binary
curl -L https://github.com/yabl-lang/ycard-cli/releases/latest/download/ycard-linux-x64 -o ycard
chmod +x ycard
```

### Commands

#### Parse

```bash
# Parse and format
ycard parse contact.ycard

# Output JSON AST
ycard parse contact.ycard --json-ast

# Specify locale
ycard parse contact.ycard --locale=fr

# Use custom alias pack  
ycard parse contact.ycard --alias-pack=./team-aliases.json
```

#### Format

```bash
# Format in place
ycard fmt contact.ycard --write

# Shorthand phone style
ycard fmt contact.ycard --phones-style=shorthand  

# Relocalize keys
ycard fmt contact.ycard --relocalize-keys=es
```

#### Validate

```bash
# Lenient validation (default)
ycard check contact.ycard

# Strict validation
ycard check contact.ycard --strict
```

#### Global Options

```bash
# Use custom alias pack for all commands
ycard --alias-pack=./aliases.json parse contact.ycard

# Disable bundled aliases
ycard --no-bundled-aliases parse contact.ycard

# Verbose output
ycard -v parse contact.ycard
```

## LSP Integration

### VS Code

1. Install the yCard extension from the marketplace
2. Configure in `settings.json`:

```json
{
  "ycard.locale": "en",
  "ycard.lenient": true,
  "ycard.phonesStyle": "canonical",
  "ycard.i18n.aliasPacks": [
    "/path/to/custom-aliases.json"
  ]
}
```

### Neovim

```lua
require'lspconfig'.ycard.setup{
  cmd = {"ycard-lsp"},
  filetypes = {"ycard", "yaml"},
  settings = {
    ycard = {
      locale = "en",
      lenient = true,
      i18n = {
        aliasPacks = {"/path/to/aliases.json"}
      }
    }
  }
}
```

### Emacs (lsp-mode)

```elisp
(use-package lsp-mode
  :config
  (add-to-list 'lsp-language-id-configuration '(ycard-mode . "ycard"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "ycard-lsp")
                    :major-modes '(ycard-mode yaml-mode)
                    :server-id 'ycard-lsp)))
```

## Error Handling

### TypeScript

```typescript
import { parse, ParseError } from '@ycard/core';

try {
  const card = await parse(input);
} catch (error) {
  if (error instanceof ParseError) {
    console.error('Parse failed:', error.message);
    console.error('Diagnostics:', error.diagnostics);
  }
}
```

### Python

```python
import ycard

try:
    card = ycard.parse(input)
except ycard.ParseError as e:
    print(f"Parse error: {e}")
    for diag in e.diagnostics:
        print(f"  {diag['level']}: {diag['message']}")
```

### Go

```go
card, err := engine.Parse(ctx, input, nil)
if err != nil {
    var parseErr *ycard.ParseError
    if errors.As(err, &parseErr) {
        fmt.Printf("Parse failed: %v\n", parseErr)
        for _, diag := range parseErr.Diagnostics {
            fmt.Printf("  %s: %s\n", diag.Level, diag.Message)
        }
    }
}
```

## Advanced Features

### Custom Validation

```typescript
// Custom validation with business rules
const diagnostics = await validate(ycard, ValidationMode.Strict);

// Add custom checks
if (!ycard.emails?.some(e => e.type.includes('work'))) {
  diagnostics.push({
    level: DiagnosticLevel.Warning,
    message: 'Work email recommended for business contacts',
    code: 'work-email-missing'
  });
}
```

### Batch Processing

```typescript
// Process multiple yCards
const results = await Promise.all(
  inputs.map(async input => {
    try {
      const card = await parse(input.content, input.locale);
      return { success: true, card };
    } catch (error) {
      return { success: false, error };
    }
  })
);
```

### Performance Optimization

```python
# Reuse engine instance for better performance
engine = ycard.Engine()

# Process many cards
for card_text in card_texts:
    card = engine.parse(card_text)
    # ... process card
```

## Migration from Other Formats

### From vCard

```typescript
// Partial vCard conversion helper
function vCardToYCard(vcard: string): Partial<YCard> {
  // This would be a community contribution
  // Basic field mapping logic
  return {
    version: 1,
    name: extractNameFromVCard(vcard),
    phones: extractPhonesFromVCard(vcard),
    emails: extractEmailsFromVCard(vcard)
  };
}
```

### From JSON Contacts

```python
def json_contact_to_ycard(json_contact: dict) -> dict:
    """Convert generic JSON contact to yCard format"""
    return {
        'version': 1,
        'name': {
            'displayName': json_contact.get('full_name'),
            'givenName': [json_contact.get('first_name')] if json_contact.get('first_name') else None,
            'familyName': [json_contact.get('last_name')] if json_contact.get('last_name') else None,
        },
        'phones': [
            {'number': phone, 'type': ['mobile']} 
            for phone in json_contact.get('phones', [])
        ],
        'emails': [
            {'address': email, 'type': ['home']}
            for email in json_contact.get('emails', [])
        ]
    }
```