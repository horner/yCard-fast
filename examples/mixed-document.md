# Mixed Markdown Document with yCard Integration

This document demonstrates both fenced yCard blocks and contextual extraction.

## Team Directory

Here's our team lead:

```ycard
version: 1
name:
  givenName: [Sarah]
  familyName: [Johnson]
  displayName: "Sarah Johnson"
phones:
  - number: "+15551234567"
    type: [work]
    ext: "1001"
  - number: "+15559876543"
    type: [mobile]
    preferred: true
emails:
  - address: "sarah.johnson@company.com"
    type: [work]
    preferred: true
  - address: "sarah@personal.com"
    type: [home]
addresses:
  - type: [work]
    components:
      street: "123 Business Ave"
      locality: "Tech City"
      region: "CA"
      postalCode: "94000"
      country: "US"
metadata:
  locale: "en-US"
  source: "corporate-directory"
```

## Contact

name: Alex Chen
mobile: "+1 555 987 6543"
work: "+1 555 555 0123"
email: "alex.chen@company.com"

## International Team Member

```ycard
nom: "Dubois"
prénom: "Marie"
portable: "06 12 34 56 78"
bureau: "+33 1 23 45 67 89"
email: "marie.dubois@company.fr"
```

## Kontakt

name: "Hans Müller"
telefon: "+49 30 12345678"
email: "hans.mueller@company.de"

## 連絡先

name: "田中太郎"
携帯: "090-1234-5678"
email: "tanaka@company.jp"

---

This document contains multiple yCard blocks that can be extracted and processed by the YABL markdown integration system.