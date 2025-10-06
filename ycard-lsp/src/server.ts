#!/usr/bin/env node

import {
  createConnection,
  TextDocuments,
  Diagnostic,
  DiagnosticSeverity,
  ProposedFeatures,
  InitializeParams,
  DidChangeConfigurationNotification,
  CompletionItem,
  CompletionItemKind,
  TextDocumentPositionParams,
  TextDocumentSyncKind,
  InitializeResult,
  DocumentDiagnosticReportKind,
  type DocumentDiagnosticReport,
  CodeAction,
  CodeActionKind,
  CodeActionParams,
  Command,
  ExecuteCommandParams,
  WorkspaceEdit,
  TextEdit,
  Range,
  Position,
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import * as ycard from './ycard-wrapper';
import { YCARD_COMPLETION_ITEMS } from './generated_completions';
import * as fs from 'fs';
import * as path from 'path';

// Create a connection for the server, using Node's IPC as a transport.
const connection = createConnection(ProposedFeatures.all);

// Create a simple text document manager.
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;

// Interface for LSP settings
interface YCardSettings {
  locale: string;
  lenient: boolean;
  phonesStyle: 'canonical' | 'shorthand' | 'auto';
  i18n: {
    aliasPacks: string[];
  };
}

// Default settings
const defaultSettings: YCardSettings = {
  locale: 'en',
  lenient: true,
  phonesStyle: 'canonical',
  i18n: {
    aliasPacks: []
  }
};

let globalSettings: YCardSettings = defaultSettings;

// Cache the settings of all open documents
const documentSettings: Map<string, Thenable<YCardSettings>> = new Map();

connection.onInitialize((params: InitializeParams) => {
  const capabilities = params.capabilities;

  // Does the client support the `workspace/configuration` request?
  hasConfigurationCapability = !!(
    capabilities.workspace && !!capabilities.workspace.configuration
  );
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );
  hasDiagnosticRelatedInformationCapability = !!(
    capabilities.textDocument &&
    capabilities.textDocument.publishDiagnostics &&
    capabilities.textDocument.publishDiagnostics.relatedInformation
  );

  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      // Tell the client that this server supports code completion.
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: [':']
      },
      // Support code actions
      codeActionProvider: {
        codeActionKinds: [
          CodeActionKind.QuickFix,
          CodeActionKind.Source,
          CodeActionKind.SourceOrganizeImports
        ]
      },
      // Support execute command
      executeCommandProvider: {
        commands: [
          'ycard.reloadAliases',
          'ycard.canonicalizeDocument'
        ]
      },
      // Support document formatting
      documentFormattingProvider: true,
      // Support hover
      hoverProvider: true,
      // Support diagnostics
      diagnosticProvider: {
        interFileDependencies: false,
        workspaceDiagnostics: false
      }
    }
  };

  if (hasWorkspaceFolderCapability) {
    result.capabilities.workspace = {
      workspaceFolders: {
        supported: true
      }
    };
  }

  return result;
});

connection.onInitialized(() => {
  if (hasConfigurationCapability) {
    // Register for all configuration changes.
    connection.client.register(DidChangeConfigurationNotification.type, undefined);
  }
  if (hasWorkspaceFolderCapability) {
    connection.workspace.onDidChangeWorkspaceFolders(_event => {
      connection.console.log('Workspace folder change event received.');
    });
  }

  // Load initial alias packs
  loadAliasPacks(globalSettings.i18n.aliasPacks);
});

connection.onDidChangeConfiguration(change => {
  if (hasConfigurationCapability) {
    // Reset all cached document settings
    documentSettings.clear();
  } else {
    globalSettings = <YCardSettings>(
      (change.settings.ycard || defaultSettings)
    );
  }

  // Revalidate all open documents
  documents.all().forEach(validateTextDocument);
});

function getDocumentSettings(resource: string): Thenable<YCardSettings> {
  if (!hasConfigurationCapability) {
    return Promise.resolve(globalSettings);
  }
  let result = documentSettings.get(resource);
  if (!result) {
    result = connection.workspace.getConfiguration({
      scopeUri: resource,
      section: 'ycard'
    });
    documentSettings.set(resource, result);
  }
  return result;
}

// Only keep settings for open documents
documents.onDidClose(e => {
  documentSettings.delete(e.document.uri);
});

// The content of a text document has changed. This event is emitted
// when the text document first opened or when its content has changed.
documents.onDidChangeContent(change => {
  validateTextDocument(change.document);
});

async function loadAliasPacks(packPaths: string[]): Promise<void> {
  for (const packPath of packPaths) {
    try {
      const fullPath = path.resolve(packPath);
      if (fs.existsSync(fullPath)) {
        const packContent = await fs.promises.readFile(fullPath, 'utf-8');
        await ycard.loadAliasPack(packContent);
        connection.console.log(`Loaded alias pack: ${packPath}`);
      } else {
        connection.console.log(`Alias pack not found: ${packPath}`);
      }
    } catch (error) {
      connection.console.error(`Failed to load alias pack ${packPath}: ${error}`);
    }
  }
}

async function validateTextDocument(textDocument: TextDocument): Promise<void> {
  // Get the settings for this document
  const settings = await getDocumentSettings(textDocument.uri);
  
  const text = textDocument.getText();
  const diagnostics: Diagnostic[] = [];

  try {
    // Set locale for this validation
    await ycard.setDefaultLocale(settings.locale);
    
    // Parse the document
    const parsed = await ycard.parse(text, settings.locale);
    
    // Validate
    const validationMode = settings.lenient 
      ? ycard.ValidationMode.Lenient 
      : ycard.ValidationMode.Strict;
    
    const ycardDiagnostics = await ycard.validate(parsed, validationMode);
    
    // Convert yCard diagnostics to LSP diagnostics
    for (const ycardDiag of ycardDiagnostics) {
      const diagnostic: Diagnostic = {
        severity: convertDiagnosticLevel(ycardDiag.level),
        range: ycardDiag.range || {
          start: { line: 0, character: 0 },
          end: { line: 0, character: 0 }
        },
        message: ycardDiag.message,
        source: 'yCard',
        code: ycardDiag.code
      };
      
      diagnostics.push(diagnostic);
    }
    
  } catch (error) {
    // Parse error - create diagnostic
    const diagnostic: Diagnostic = {
      severity: DiagnosticSeverity.Error,
      range: {
        start: { line: 0, character: 0 },
        end: { line: 0, character: 0 }
      },
      message: `Parse error: ${error}`,
      source: 'yCard'
    };
    
    diagnostics.push(diagnostic);
  }

  // Send the computed diagnostics to VSCode.
  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

function convertDiagnosticLevel(level: ycard.DiagnosticLevelType): DiagnosticSeverity {
  switch (level) {
    case 'Error':
      return DiagnosticSeverity.Error;
    case 'Warning':
      return DiagnosticSeverity.Warning;
    case 'Info':
      return DiagnosticSeverity.Information;
    case 'Hint':
      return DiagnosticSeverity.Hint;
    default:
      return DiagnosticSeverity.Information;
  }
}

// This handler provides the initial list of the completion items.
connection.onCompletion(
  (_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    // Generated completion items from schema
    return YCARD_COMPLETION_ITEMS;
  }
);

// This handler resolves additional information for the item selected in
// the completion list.
connection.onCompletionResolve(
  (item: CompletionItem): CompletionItem => {
    if (item.data === 1) {
      item.detail = 'yCard format version (always 1 for v1)';
      item.documentation = 'Specifies the yCard format version. Currently only version 1 is supported.';
    } else if (item.data === 2) {
      item.detail = 'Contact name information';
      item.documentation = 'Can be a simple string or structured object with givenName, familyName, etc.';
    }
    return item;
  }
);

// Code actions (quick fixes, refactorings)
connection.onCodeAction((params: CodeActionParams): CodeAction[] => {
  const actions: CodeAction[] = [];
  const document = documents.get(params.textDocument.uri);
  
  if (!document) {
    return actions;
  }

  // Add "Canonicalize Document" action
  const canonicalizeAction: CodeAction = {
    title: 'Canonicalize yCard',
    kind: CodeActionKind.SourceOrganizeImports,
    command: {
      title: 'Canonicalize yCard',
      command: 'ycard.canonicalizeDocument',
      arguments: [params.textDocument.uri]
    }
  };
  actions.push(canonicalizeAction);

  // Add quick fixes based on diagnostics
  for (const diagnostic of params.context.diagnostics) {
    if (diagnostic.source === 'yCard' && diagnostic.code) {
      if (diagnostic.code === 'version-missing') {
        const fix: CodeAction = {
          title: 'Add version: 1',
          kind: CodeActionKind.QuickFix,
          edit: {
            changes: {
              [params.textDocument.uri]: [
                {
                  range: { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } },
                  newText: 'version: 1\n'
                }
              ]
            }
          }
        };
        actions.push(fix);
      }
    }
  }

  return actions;
});

// Execute command handler
connection.onExecuteCommand(async (params: ExecuteCommandParams) => {
  if (params.command === 'ycard.reloadAliases') {
    const settings = globalSettings;
    await loadAliasPacks(settings.i18n.aliasPacks);
    connection.window.showInformationMessage('yCard aliases reloaded');
    
    // Revalidate all documents
    documents.all().forEach(validateTextDocument);
    
  } else if (params.command === 'ycard.canonicalizeDocument') {
    const uri = params.arguments?.[0] as string;
    const document = documents.get(uri);
    
    if (document) {
      try {
        const settings = await getDocumentSettings(uri);
        await ycard.setDefaultLocale(settings.locale);
        
        const text = document.getText();
        const parsed = await ycard.parse(text, settings.locale);
        const formatted = await ycard.format(parsed, ycard.PhonesStyle.Canonical);
        
        const edit: WorkspaceEdit = {
          changes: {
            [uri]: [
              {
                range: {
                  start: { line: 0, character: 0 },
                  end: { line: document.lineCount, character: 0 }
                },
                newText: formatted
              }
            ]
          }
        };
        
        await connection.workspace.applyEdit(edit);
        connection.window.showInformationMessage('yCard canonicalized');
        
      } catch (error) {
        connection.window.showErrorMessage(`Failed to canonicalize: ${error}`);
      }
    }
  }
});

// Document formatting
connection.onDocumentFormatting(async (params) => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return [];
  }

  try {
    const settings = await getDocumentSettings(params.textDocument.uri);
    await ycard.setDefaultLocale(settings.locale);
    
    const text = document.getText();
    const parsed = await ycard.parse(text, settings.locale);
    const phoneStyle = settings.phonesStyle === 'shorthand' 
      ? ycard.PhonesStyle.Shorthand 
      : ycard.PhonesStyle.Canonical;
    const formatted = await ycard.format(parsed, phoneStyle);
    
    const textEdit: TextEdit = {
      range: {
        start: { line: 0, character: 0 },
        end: { line: document.lineCount, character: 0 }
      },
      newText: formatted
    };
    
    return [textEdit];
    
  } catch (error) {
    connection.console.error(`Format error: ${error}`);
    return [];
  }
});

// Hover support
connection.onHover(async (params) => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return null;
  }

  // Simple hover support - would be enhanced with actual position analysis
  const word = 'mobile'; // This would extract the word at position
  
  if (word === 'mobile') {
    return {
      contents: {
        kind: 'markdown',
        value: '**Mobile Phone** (shorthand)\n\nExpands to `phones` array with `type: [mobile]`'
      }
    };
  }

  return null;
});

// Make the text document manager listen on the connection
// for open, change and close text document events
documents.listen(connection);

// Listen on the connection
connection.listen();