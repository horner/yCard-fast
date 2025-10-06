// Generated from schema.json - DO NOT EDIT MANUALLY
// Run `node generate-code.js` to regenerate

import { CompletionItem, CompletionItemKind } from 'vscode-languageserver/node';

export const YCARD_COMPLETION_ITEMS: CompletionItem[] = [
  {
    label: 'uid',
    kind: CompletionItemKind.Property,
    data: 1,
    detail: 'Optional unique identifier',
    insertText: 'uid: '
  },
  {
    label: 'name',
    kind: CompletionItemKind.Property,
    data: 2,
    detail: 'Contact name information',
    insertText: 'name: '
  },
  {
    label: 'phones',
    kind: CompletionItemKind.Property,
    data: 3,
    detail: 'Phone numbers',
    insertText: 'phones: '
  },
  {
    label: 'emails',
    kind: CompletionItemKind.Property,
    data: 4,
    detail: 'Email addresses',
    insertText: 'emails: '
  },
  {
    label: 'addresses',
    kind: CompletionItemKind.Property,
    data: 5,
    detail: 'Physical addresses',
    insertText: 'addresses: '
  },
  {
    label: 'metadata',
    kind: CompletionItemKind.Property,
    data: 6,
    detail: 'Processing metadata',
    insertText: 'metadata: '
  },
  {
    label: 'mobile',
    kind: CompletionItemKind.Property,
    data: 7,
    detail: 'Mobile phone (shorthand)',
    insertText: 'mobile: "+1 555 123 4567"'
  },
  {
    label: 'home',
    kind: CompletionItemKind.Property,
    data: 8,
    detail: 'Home phone (shorthand)',
    insertText: 'home: "+1 555 123 4567"'
  },
  {
    label: 'work',
    kind: CompletionItemKind.Property,
    data: 9,
    detail: 'Work phone (shorthand)',
    insertText: 'work: "+1 555 123 4567"'
  },
  {
    label: 'fax',
    kind: CompletionItemKind.Property,
    data: 10,
    detail: 'Fax phone (shorthand)',
    insertText: 'fax: "+1 555 123 4567"'
  },
  {
    label: 'pager',
    kind: CompletionItemKind.Property,
    data: 11,
    detail: 'Pager phone (shorthand)',
    insertText: 'pager: "+1 555 123 4567"'
  },
  {
    label: 'main',
    kind: CompletionItemKind.Property,
    data: 12,
    detail: 'Main phone (shorthand)',
    insertText: 'main: "+1 555 123 4567"'
  },
  {
    label: 'email',
    kind: CompletionItemKind.Property,
    data: 13,
    detail: 'Email address (shorthand)',
    insertText: 'email: "example@domain.com"'
  },
];
