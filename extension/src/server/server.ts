// FlowLang LSP Server

import {
    createConnection,
    TextDocuments,
    Diagnostic,
    DiagnosticSeverity,
    ProposedFeatures,
    InitializeParams,
    TextDocumentSyncKind,
    InitializeResult,
    CompletionItem,
    CompletionItemKind,
    Hover,
    MarkupKind,
    TextEdit,
    DocumentFormattingParams,
    Location,
    Range,
    Position,
    SymbolKind,
    DocumentSymbol,
    SignatureHelp,
    ParameterInformation,
    SignatureInformation,
    Definition,
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import { URI } from 'vscode-uri';
import * as fs from 'fs';
import * as path from 'path';

import { Parser } from './parser';
import { TypeChecker } from './checker';
import { Program, Statement, Expression, FunctionDecl, LetStatement, SealStatement, RitualDecl } from './ast';
import { getEpisodeForError, formatEpisodeMessage, detectEpisodeType } from './episodes';

// Create connection
const connection = createConnection(ProposedFeatures.all);
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

// Document AST cache
const documentASTs: Map<string, Program> = new Map();

// Parsed module exports cache
interface ModuleExport {
    name: string;
    kind: 'function' | 'ritual' | 'constant';
    params?: string;
    returnType?: string;
}
const moduleExportsCache: Map<string, ModuleExport[]> = new Map();

// Parse a module file and extract its exports
function parseModuleExports(filePath: string): ModuleExport[] {
    // Check cache first
    if (moduleExportsCache.has(filePath)) {
        return moduleExportsCache.get(filePath) || [];
    }

    try {
        if (!fs.existsSync(filePath)) {
            return [];
        }

        const content = fs.readFileSync(filePath, 'utf-8');
        const parser = new Parser();
        const ast = parser.parse(content);

        const exports: ModuleExport[] = [];

        for (const stmt of ast.statements) {
            if (stmt.type === 'FunctionDecl' && stmt.isExported) {
                const funcDecl = stmt as FunctionDecl;
                const params = funcDecl.params.map(p => {
                    const typeStr = p.typeAnnotation ? p.typeAnnotation.name || 'Flux' : 'Flux';
                    return `${p.name}: ${typeStr}`;
                }).join(', ');
                const retType = funcDecl.returnType ? funcDecl.returnType.name || 'Flux' : 'Hollow';
                exports.push({
                    name: funcDecl.name,
                    kind: 'function',
                    params,
                    returnType: retType,
                });
            } else if (stmt.type === 'RitualDecl' && stmt.isExported) {
                const ritualDecl = stmt as RitualDecl;
                const params = ritualDecl.params.map(p => {
                    const typeStr = p.typeAnnotation ? p.typeAnnotation.name || 'Flux' : 'Flux';
                    return `${p.name}: ${typeStr}`;
                }).join(', ');
                const retType = ritualDecl.returnType ? ritualDecl.returnType.name || 'Flux' : 'Hollow';
                exports.push({
                    name: ritualDecl.name,
                    kind: 'ritual',
                    params,
                    returnType: retType,
                });
            } else if (stmt.type === 'SealStatement' && stmt.isExported) {
                const sealStmt = stmt as SealStatement;
                exports.push({
                    name: sealStmt.name,
                    kind: 'constant',
                });
            } else if (stmt.type === 'LetStatement' && stmt.isExported) {
                const letStmt = stmt as LetStatement;
                exports.push({
                    name: letStmt.name,
                    kind: 'constant',
                });
            }
        }

        // Cache the results
        moduleExportsCache.set(filePath, exports);
        return exports;
    } catch (error) {
        return [];
    }
}

// Clear module cache when files change
function clearModuleCache(filePath: string): void {
    moduleExportsCache.delete(filePath);
}

connection.onInitialize((params: InitializeParams): InitializeResult => {
    return {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['.', '@', ':'],
            },
            hoverProvider: true,
            // documentFormattingProvider: true,  // Disabled until language is complete

            // 🔮 High Impact LSP Features
            definitionProvider: true,
            referencesProvider: true,
            documentSymbolProvider: true,
            signatureHelpProvider: {
                triggerCharacters: ['(', ','],
            },
        },
    };
});

// Validate documents
documents.onDidChangeContent(change => {
    validateDocument(change.document);
});

async function validateDocument(document: TextDocument): Promise<void> {
    const text = document.getText();
    const diagnostics: Diagnostic[] = [];

    // Parse
    const parser = new Parser();
    const ast = parser.parse(text);
    documentASTs.set(document.uri, ast);

    // Check module imports
    const currentDocUri = URI.parse(document.uri);
    const currentDir = path.dirname(currentDocUri.fsPath);

    for (const imp of ast.imports) {
        const modulePath = imp.fromPath || `std:${imp.module}`;

        // Check for local file imports
        if (modulePath.startsWith('./') || modulePath.startsWith('../') || modulePath.endsWith('.flow')) {
            const resolvedPath = path.resolve(currentDir, modulePath);

            if (!fs.existsSync(resolvedPath)) {
                const episode = getEpisodeForError('ModuleNotFound');
                diagnostics.push({
                    severity: DiagnosticSeverity.Error,
                    range: {
                        start: { line: imp.range.start.line - 1, character: imp.range.start.column - 1 },
                        end: { line: imp.range.end.line - 1, character: imp.range.end.column - 1 },
                    },
                    message: formatEpisodeMessage(episode, `Module not found: '${modulePath}'`),
                    source: 'FlowLang',
                });
            }
        }
        // Check for unknown standard library modules
        else if (modulePath.startsWith('std:')) {
            const KNOWN_STDLIB = [
                'std:file', 'std:json', 'std:math', 'std:net', 'std:time',
                'std:timer', 'std:string', 'std:array', 'std:crypto', 'std:cli',
                'std:os', 'std:color', 'std:path', 'std:url', 'std:web', 'std:stream'
            ];

            if (!KNOWN_STDLIB.includes(modulePath)) {
                const episode = getEpisodeForError('InvalidImport');
                diagnostics.push({
                    severity: DiagnosticSeverity.Error,
                    range: {
                        start: { line: imp.range.start.line - 1, character: imp.range.start.column - 1 },
                        end: { line: imp.range.end.line - 1, character: imp.range.end.column - 1 },
                    },
                    message: formatEpisodeMessage(episode, `Unknown standard library module: '${modulePath}'`),
                    source: 'FlowLang',
                });
            }
        }
    }

    // Add lexer errors with episode styling
    for (const error of parser.lexerErrors) {
        const episode = getEpisodeForError('Syntax');
        diagnostics.push({
            severity: DiagnosticSeverity.Error,
            range: {
                start: { line: error.range.start.line - 1, character: error.range.start.column - 1 },
                end: { line: error.range.end.line - 1, character: error.range.end.column - 1 },
            },
            message: formatEpisodeMessage(episode, error.message),
            source: 'FlowLang',
        });
    }

    // Add parser errors with episode styling
    for (const error of parser.errors) {
        const episode = getEpisodeForError('Syntax');
        diagnostics.push({
            severity: DiagnosticSeverity.Error,
            range: {
                start: { line: error.range.start.line - 1, character: error.range.start.column - 1 },
                end: { line: error.range.end.line - 1, character: error.range.end.column - 1 },
            },
            message: formatEpisodeMessage(episode, error.message),
            source: 'FlowLang',
        });
    }

    // Type check with episode styling
    const checker = new TypeChecker();
    const typeErrors = checker.check(ast);

    for (const error of typeErrors) {
        const episodeType = detectEpisodeType(error.message);
        const episode = getEpisodeForError(episodeType);

        let severity: DiagnosticSeverity;
        switch (error.severity) {
            case 'error':
                severity = DiagnosticSeverity.Error;
                break;
            case 'warning':
                severity = DiagnosticSeverity.Warning;
                break;
            default:
                severity = DiagnosticSeverity.Information;
        }

        diagnostics.push({
            severity,
            range: {
                start: { line: error.range.start.line - 1, character: error.range.start.column - 1 },
                end: { line: error.range.end.line - 1, character: error.range.end.column - 1 },
            },
            message: formatEpisodeMessage(episode, error.message),
            source: 'FlowLang',
        });
    }

    connection.sendDiagnostics({ uri: document.uri, diagnostics });
}

// Stdlib module functions for completion
const STDLIB_COMPLETIONS: Record<string, { name: string; params: string; returnType: string; doc: string }[]> = {
    'std:file': [
        { name: 'read', params: 'path: Silk', returnType: 'Silk', doc: 'Read file contents' },
        { name: 'write', params: 'path: Silk, content: Silk', returnType: 'Hollow', doc: 'Write to file' },
        { name: 'append', params: 'path: Silk, content: Silk', returnType: 'Hollow', doc: 'Append to file' },
        { name: 'exists', params: 'path: Silk', returnType: 'Pulse', doc: 'Check if file exists' },
        { name: 'delete', params: 'path: Silk', returnType: 'Hollow', doc: 'Delete file' },
        { name: 'list', params: 'path: Silk', returnType: 'Constellation', doc: 'List directory contents' },
    ],
    'std:json': [
        { name: 'parse', params: 'json: Silk', returnType: 'Relic', doc: 'Parse JSON string' },
        { name: 'stringify', params: 'value: Flux', returnType: 'Silk', doc: 'Convert to JSON string' },
    ],
    'std:math': [
        { name: 'sin', params: 'x: Ember', returnType: 'Ember', doc: 'Calculate sine' },
        { name: 'cos', params: 'x: Ember', returnType: 'Ember', doc: 'Calculate cosine' },
        { name: 'tan', params: 'x: Ember', returnType: 'Ember', doc: 'Calculate tangent' },
        { name: 'sqrt', params: 'x: Ember', returnType: 'Ember', doc: 'Square root' },
        { name: 'abs', params: 'x: Ember', returnType: 'Ember', doc: 'Absolute value' },
        { name: 'round', params: 'x: Ember', returnType: 'Ember', doc: 'Round to nearest integer' },
        { name: 'floor', params: 'x: Ember', returnType: 'Ember', doc: 'Round down' },
        { name: 'ceil', params: 'x: Ember', returnType: 'Ember', doc: 'Round up' },
        { name: 'pow', params: 'base: Ember, exp: Ember', returnType: 'Ember', doc: 'Power function' },
        { name: 'random', params: '', returnType: 'Ember', doc: 'Random number 0-1' },
        { name: 'min', params: 'a: Ember, b: Ember', returnType: 'Ember', doc: 'Minimum value' },
        { name: 'max', params: 'a: Ember, b: Ember', returnType: 'Ember', doc: 'Maximum value' },
    ],
    'std:net': [
        { name: 'get', params: 'url: Silk', returnType: 'Silk', doc: 'HTTP GET request' },
        { name: 'post', params: 'url: Silk, body: Silk', returnType: 'Silk', doc: 'HTTP POST request' },
        { name: 'put', params: 'url: Silk, body: Silk', returnType: 'Silk', doc: 'HTTP PUT request' },
        { name: 'delete', params: 'url: Silk', returnType: 'Silk', doc: 'HTTP DELETE request' },
    ],
    'std:time': [
        { name: 'now', params: '', returnType: 'Ember', doc: 'Current Unix timestamp' },
        { name: 'format', params: 'ts: Ember, fmt: Silk', returnType: 'Silk', doc: 'Format timestamp' },
        { name: 'iso', params: '', returnType: 'Silk', doc: 'ISO date string' },
        { name: 'sleep', params: 'ms: Ember', returnType: 'Hollow', doc: 'Sleep milliseconds' },
    ],
    'std:timer': [
        { name: 'interval', params: 'ms: Ember, callback: Spell', returnType: 'Flux', doc: 'Repeating timer' },
        { name: 'timeout', params: 'ms: Ember, callback: Spell', returnType: 'Flux', doc: 'One-shot timer' },
        { name: 'clear', params: 'handle: Flux', returnType: 'Pulse', doc: 'Cancel timer' },
    ],
    'std:string': [
        { name: 'len', params: 's: Silk', returnType: 'Ember', doc: 'String length' },
        { name: 'upper', params: 's: Silk', returnType: 'Silk', doc: 'To uppercase' },
        { name: 'lower', params: 's: Silk', returnType: 'Silk', doc: 'To lowercase' },
        { name: 'trim', params: 's: Silk', returnType: 'Silk', doc: 'Trim whitespace' },
        { name: 'split', params: 's: Silk, delim: Silk', returnType: 'Constellation', doc: 'Split string' },
        { name: 'substring', params: 's: Silk, start: Ember, end: Ember', returnType: 'Silk', doc: 'Get substring' },
        { name: 'replace', params: 's: Silk, search: Silk, replacement: Silk', returnType: 'Silk', doc: 'Replace text' },
        { name: 'contains', params: 's: Silk, search: Silk', returnType: 'Pulse', doc: 'Check if contains' },
        { name: 'starts_with', params: 's: Silk, prefix: Silk', returnType: 'Pulse', doc: 'Check if starts with' },
        { name: 'ends_with', params: 's: Silk, suffix: Silk', returnType: 'Pulse', doc: 'Check if ends with' },
    ],
    'std:array': [
        { name: 'len', params: 'arr: Constellation', returnType: 'Ember', doc: 'Array length' },
        { name: 'push', params: 'arr: Constellation, item: Flux', returnType: 'Constellation', doc: 'Add element' },
        { name: 'pop', params: 'arr: Constellation', returnType: 'Flux', doc: 'Remove & return last' },
        { name: 'reverse', params: 'arr: Constellation', returnType: 'Constellation', doc: 'Reverse array' },
        { name: 'sort', params: 'arr: Constellation', returnType: 'Constellation', doc: 'Sort array' },
        { name: 'slice', params: 'arr: Constellation, start: Ember, end: Ember', returnType: 'Constellation', doc: 'Get slice' },
        { name: 'contains', params: 'arr: Constellation, item: Flux', returnType: 'Pulse', doc: 'Check if contains' },
        { name: 'filter', params: 'arr: Constellation, predicate: Spell', returnType: 'Constellation', doc: 'Filter elements' },
        { name: 'map', params: 'arr: Constellation, transformer: Spell', returnType: 'Constellation', doc: 'Transform elements' },
        { name: 'reduce', params: 'arr: Constellation, reducer: Spell, initial: Flux', returnType: 'Flux', doc: 'Reduce to value' },
        { name: 'find', params: 'arr: Constellation, predicate: Spell', returnType: 'Flux', doc: 'Find element' },
        { name: 'every', params: 'arr: Constellation, predicate: Spell', returnType: 'Pulse', doc: 'Test all' },
        { name: 'some', params: 'arr: Constellation, predicate: Spell', returnType: 'Pulse', doc: 'Test any' },
        { name: 'join', params: 'arr: Constellation, delim: Silk', returnType: 'Silk', doc: 'Join to string' },
    ],
    'std:crypto': [
        { name: 'sha256', params: 'text: Silk', returnType: 'Silk', doc: 'SHA256 hash' },
        { name: 'md5', params: 'text: Silk', returnType: 'Silk', doc: 'MD5 hash' },
        { name: 'uuid', params: '', returnType: 'Silk', doc: 'Generate UUID' },
        { name: 'base64_encode', params: 'text: Silk', returnType: 'Silk', doc: 'Base64 encode' },
        { name: 'base64_decode', params: 'encoded: Silk', returnType: 'Silk', doc: 'Base64 decode' },
    ],
    'std:cli': [
        { name: 'input', params: 'prompt: Silk', returnType: 'Silk', doc: 'Read user input' },
        { name: 'args', params: '', returnType: 'Constellation', doc: 'Get CLI arguments' },
        { name: 'confirm', params: 'prompt: Silk', returnType: 'Pulse', doc: 'Yes/no prompt' },
        { name: 'clear', params: '', returnType: 'Hollow', doc: 'Clear terminal' },
        { name: 'exit', params: 'code: Ember', returnType: 'Hollow', doc: 'Exit program' },
    ],
    'std:os': [
        { name: 'env', params: 'name: Silk', returnType: 'Silk', doc: 'Get environment variable' },
        { name: 'platform', params: '', returnType: 'Silk', doc: 'Get OS platform' },
        { name: 'cwd', params: '', returnType: 'Silk', doc: 'Current working directory' },
    ],
    'std:color': [
        { name: 'red', params: 'text: Silk', returnType: 'Silk', doc: 'Red text' },
        { name: 'green', params: 'text: Silk', returnType: 'Silk', doc: 'Green text' },
        { name: 'blue', params: 'text: Silk', returnType: 'Silk', doc: 'Blue text' },
        { name: 'yellow', params: 'text: Silk', returnType: 'Silk', doc: 'Yellow text' },
        { name: 'bold', params: 'text: Silk', returnType: 'Silk', doc: 'Bold text' },
    ],
    'std:path': [
        { name: 'join', params: 'a: Silk, b: Silk', returnType: 'Silk', doc: 'Join paths' },
        { name: 'basename', params: 'path: Silk', returnType: 'Silk', doc: 'Get file name' },
        { name: 'dirname', params: 'path: Silk', returnType: 'Silk', doc: 'Get directory' },
        { name: 'extname', params: 'path: Silk', returnType: 'Silk', doc: 'Get extension' },
    ],
    'std:url': [
        { name: 'parse', params: 'url: Silk', returnType: 'Relic', doc: 'Parse URL' },
        { name: 'encode', params: 'text: Silk', returnType: 'Silk', doc: 'URL encode' },
        { name: 'decode', params: 'text: Silk', returnType: 'Silk', doc: 'URL decode' },
    ],
    'std:web': [
        { name: 'serve', params: 'port: Ember, handler: Spell', returnType: 'Hollow', doc: 'Start web server' },
        { name: 'route', params: 'method: Silk, path: Silk, handler: Spell', returnType: 'Hollow', doc: 'Add route' },
        { name: 'json_response', params: 'data: Relic', returnType: 'Relic', doc: 'JSON response' },
    ],
};

// Track imported modules per document
const documentModules: Map<string, Map<string, string>> = new Map();

// Completions
connection.onCompletion((params): CompletionItem[] => {
    const items: CompletionItem[] = [];
    const document = documents.get(params.textDocument.uri);
    if (!document) return items;

    const text = document.getText();
    const offset = document.offsetAt(params.position);

    // Get text before cursor to detect context
    const textBefore = text.substring(0, offset);

    // Check if we're after a dot (module access)
    const dotMatch = textBefore.match(/(\w+)\.\s*(\w*)$/);
    if (dotMatch) {
        const moduleName = dotMatch[1];

        // Find this module's path from document imports
        const ast = documentASTs.get(params.textDocument.uri);
        if (ast) {
            for (const imp of ast.imports) {
                const name = imp.alias || imp.module;
                if (name === moduleName) {
                    const modulePath = imp.fromPath || `std:${imp.module}`;

                    // Get stdlib completions
                    const funcs = STDLIB_COMPLETIONS[modulePath];
                    if (funcs) {
                        for (const fn of funcs) {
                            items.push({
                                label: fn.name,
                                kind: CompletionItemKind.Function,
                                detail: `(${fn.params}) -> ${fn.returnType}`,
                                documentation: { kind: MarkupKind.Markdown, value: `**${fn.name}** - ${fn.doc}\n\n\`\`\`flowlang\n${moduleName}.${fn.name}(${fn.params})\n\`\`\`` },
                                insertText: `${fn.name}($1)`,
                                insertTextFormat: 2,
                            });
                        }
                        return items; // Return only module functions
                    }

                    // For standard library modules not in STDLIB_COMPLETIONS, show error
                    if (modulePath.startsWith('std:')) {
                        items.push({
                            label: '(unknown standard library module)',
                            kind: CompletionItemKind.Text,
                            detail: `Module '${modulePath}' not found in standard library`,
                        });
                        return items;
                    }

                    // For user modules (file imports), parse the file and get exports
                    if (modulePath.startsWith('./') || modulePath.startsWith('../') || modulePath.endsWith('.flow')) {
                        // Resolve the file path relative to current document
                        const currentDocUri = URI.parse(params.textDocument.uri);
                        const currentDir = path.dirname(currentDocUri.fsPath);
                        const resolvedPath = path.resolve(currentDir, modulePath);

                        // Parse the module and get exports
                        const moduleExports = parseModuleExports(resolvedPath);

                        if (moduleExports.length > 0) {
                            for (const exp of moduleExports) {
                                if (exp.kind === 'function' || exp.kind === 'ritual') {
                                    items.push({
                                        label: exp.name,
                                        kind: exp.kind === 'ritual' ? CompletionItemKind.Event : CompletionItemKind.Function,
                                        detail: `(${exp.params || ''}) -> ${exp.returnType || 'Hollow'}`,
                                        documentation: {
                                            kind: MarkupKind.Markdown,
                                            value: `**${exp.name}** - ${exp.kind === 'ritual' ? 'Async ritual' : 'Function'} from \`${path.basename(modulePath)}\`\n\n\`\`\`flowlang\n${moduleName}.${exp.name}(${exp.params})\n\`\`\``
                                        },
                                        insertText: `${exp.name}($1)`,
                                        insertTextFormat: 2,
                                    });
                                } else if (exp.kind === 'constant') {
                                    items.push({
                                        label: exp.name,
                                        kind: CompletionItemKind.Constant,
                                        detail: 'Exported constant',
                                        documentation: {
                                            kind: MarkupKind.Markdown,
                                            value: `**${exp.name}** - Constant from \`${path.basename(modulePath)}\``
                                        },
                                    });
                                }
                            }
                            return items;
                        }

                        // Fallback if no exports found from local file
                        items.push({
                            label: '(no exports found)',
                            kind: CompletionItemKind.Text,
                            detail: `Could not find exports in ${modulePath}`,
                        });
                        return items;
                    }

                    // Fallback for other module paths
                    items.push({
                        label: '(no exports found)',
                        kind: CompletionItemKind.Text,
                        detail: `Could not find exports in ${modulePath}`,
                    });
                    return items;
                }
            }
        }

        // If module not found in imports, provide Constellation methods if it could be an array
        const arrayMethods = [
            { name: 'len', doc: 'Get length' },
            { name: 'push', doc: 'Add element' },
            { name: 'pop', doc: 'Remove last element' },
            { name: 'slice', doc: 'Get subset' },
            { name: 'concat', doc: 'Merge arrays' },
            { name: 'constellation', doc: 'Map elements' },
            { name: 'filter', doc: 'Filter elements' },
            { name: 'reduce', doc: 'Reduce to value' },
            { name: 'find', doc: 'Find element' },
            { name: 'every', doc: 'Test all elements' },
            { name: 'some', doc: 'Test any element' },
            { name: 'reverse', doc: 'Reverse array' },
            { name: 'join', doc: 'Join to string' },
        ];
        for (const m of arrayMethods) {
            items.push({
                label: m.name,
                kind: CompletionItemKind.Method,
                detail: `Constellation.${m.name}()`,
                documentation: m.doc,
            });
        }
        return items;
    }

    // Default completions (keywords, types, etc.)
    const keywords = [
        'let', 'seal', 'cast', 'Spell', 'ritual', 'circle', 'from', 'as',
        'in', 'Stance', 'shift', 'abandon', 'otherwise', 'invoke', 'Aura', 'when',
        'enter', 'Phase', 'to', 'until', 'forever', 'return', 'attempt', 'rescue',
        'rupture', 'panic', 'wound', 'ward', 'rebound', 'finally', 'await', 'wait',
        'perform', 'sigil', 'break', 'fracture', 'shatter', 'end',
    ];

    for (const kw of keywords) {
        items.push({
            label: kw,
            kind: CompletionItemKind.Keyword,
            detail: 'FlowLang keyword',
        });
    }

    // Types (Essences)
    const types = [
        { name: 'Ember', desc: 'Number type' },
        { name: 'Silk', desc: 'String type' },
        { name: 'Pulse', desc: 'Boolean type' },
        { name: 'Flux', desc: 'Any type' },
        { name: 'Hollow', desc: 'Void type' },
        { name: 'Constellation', desc: 'Array type' },
        { name: 'Relic', desc: 'Object/Map type' },
        { name: 'Spell', desc: 'Function type' },
    ];

    for (const t of types) {
        items.push({
            label: t.name,
            kind: CompletionItemKind.TypeParameter,
            detail: t.desc,
            documentation: { kind: MarkupKind.Markdown, value: `**${t.name}** - ${t.desc}` },
        });
    }

    // Built-in functions
    const builtins = [
        { name: 'whisper', desc: 'Output message quietly' },
        { name: 'shout', desc: 'Output message normally' },
        { name: 'roar', desc: 'Output message loudly' },
        { name: 'chant', desc: 'Output message with highlight' },
    ];

    for (const fn of builtins) {
        items.push({
            label: fn.name,
            kind: CompletionItemKind.Function,
            detail: fn.desc,
            insertText: `${fn.name}($1)`,
            insertTextFormat: 2,
        });
    }

    // Error types
    const errorTypes = ['Rift', 'Glitch', 'Spirit', 'VoidTear'];
    for (const err of errorTypes) {
        items.push({
            label: err,
            kind: CompletionItemKind.Class,
            detail: 'Error type',
        });
    }

    // Snippets
    items.push({
        label: 'cast Spell',
        kind: CompletionItemKind.Snippet,
        detail: 'Function declaration',
        insertText: 'cast Spell ${1:name}(${2:params}) -> ${3:Hollow} {\n\t$0\n}',
        insertTextFormat: 2,
    });

    items.push({
        label: 'in Stance',
        kind: CompletionItemKind.Snippet,
        detail: 'Conditional statement',
        insertText: 'in Stance (${1:condition}) {\n\t$0\n}',
        insertTextFormat: 2,
    });

    items.push({
        label: 'enter Phase',
        kind: CompletionItemKind.Snippet,
        detail: 'Loop statement',
        insertText: 'enter Phase ${1:i} from ${2:0} to ${3:10} {\n\t$0\n}',
        insertTextFormat: 2,
    });

    items.push({
        label: 'attempt',
        kind: CompletionItemKind.Snippet,
        detail: 'Error handling',
        insertText: 'attempt {\n\t$1\n} rescue Rift as e {\n\t$0\n}',
        insertTextFormat: 2,
    });

    items.push({
        label: 'circle',
        kind: CompletionItemKind.Snippet,
        detail: 'Import module',
        insertText: 'circle ${1:module} from "${2:std:module}"',
        insertTextFormat: 2,
    });

    return items;
});

connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
    return item;
});

// Hover information
connection.onHover((params): Hover | null => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return null;

    const ast = documentASTs.get(params.textDocument.uri);
    if (!ast) return null;

    // Get word at position
    const text = document.getText();
    const offset = document.offsetAt(params.position);

    // Find word boundaries
    let start = offset;
    let end = offset;
    while (start > 0 && /[a-zA-Z0-9_]/.test(text[start - 1])) start--;
    while (end < text.length && /[a-zA-Z0-9_]/.test(text[end])) end++;

    const word = text.substring(start, end);
    if (!word) return null;

    // Check for type - The Grimoire of Essences
    const typeInfo: Record<string, string> = {
        'Ember': `### 📖 Grimoire — Ember
> *"The primordial flame of calculation"*

**Ember** is the essence of numbers — both whole and fractured.

\`\`\`flowlang
let count: Ember = 42
let ratio: Ember = 3.14159
\`\`\`

Embers can be combined with arithmetic rituals: \`+\`, \`-\`, \`*\`, \`/\`, \`%\``,
        'Silk': `### 📖 Grimoire — Silk
> *"Woven threads of meaning"*

**Silk** represents text — delicate strings of characters bound together.

\`\`\`flowlang
let name: Silk = "Adventurer"
let greeting: Silk = \`Hello, world!\`
\`\`\`

Silk strands can be woven together with \`+\``,
        'Pulse': `### 📖 Grimoire — Pulse
> *"The heartbeat of truth"*

**Pulse** is the essence of binary truth — yes or no, true or false.

\`\`\`flowlang
let isAlive: Pulse = both!   -- true
let isFallen: Pulse = negate!  -- false
\`\`\`

Combined with \`both!\` (AND), \`either!\` (OR), \`negate!\` (NOT)`,
        'Flux': `### 📖 Grimoire — Flux
> *"The shapeless void that accepts all forms"*

**Flux** is the essence of chaos — it can become anything.

\`\`\`flowlang
let mystery: Flux = 42
mystery = "now I'm silk"
mystery = [1, 2, 3]
\`\`\`

Use when the form is unknown or ever-changing.`,
        'Hollow': `### 📖 Grimoire — Hollow
> *"The absence that returns nothing"*

**Hollow** represents void — the absence of value.

\`\`\`flowlang
cast Spell announce(Silk msg) -> Hollow {
    shout(msg)
    -- nothing returned
}
\`\`\`

Spells that affect the world but return no essence.`,
        'Constellation': `### 📖 Grimoire — Constellation
> *"A gathering of stars in ordered formation"*

**Constellation<T>** is an array — a sequence of elements.

\`\`\`flowlang
let stars: Constellation<Ember> = [1, 2, 3, 4, 5]
let names: Constellation<Silk> = ["Alice", "Bob"]
\`\`\`

Navigate with \`.len()\`, \`.push()\`, \`.filter()\`, \`.map()\``,
        'Relic': `### 📖 Grimoire — Relic
> *"Ancient artifacts holding key-value secrets"*

**Relic<K, V>** is an object/map — keys bound to values.

\`\`\`flowlang
let artifact: Relic<Silk, Ember> = {
    "power": 100,
    "durability": 50
}
\`\`\`

Access with \`artifact["power"]\` or \`artifact.power\``,
        'Spell': `### 📖 Grimoire — Spell
> *"Incantations that transform reality"*

**Spell** is a function — callable magic.

\`\`\`flowlang
cast Spell greet(Silk name) -> Silk {
    return "Hello, " + name
}
\`\`\`

Invoked by speaking its name: \`greet("World")\``,
    };

    if (typeInfo[word]) {
        return {
            contents: { kind: MarkupKind.Markdown, value: typeInfo[word] },
        };
    }

    // Check for keywords - The Grimoire of Incantations
    const keywordInfo: Record<string, string> = {
        'let': `### 📖 Grimoire — let
> *"A vessel that may be refilled"*

**let** declares a mutable variable — its contents can change.

\`\`\`flowlang
let energy = 100
energy = energy - 25  -- allowed
\`\`\``,
        'seal': `### 📖 Grimoire — seal
> *"Bound by ancient law, unchanging forever"*

**seal** declares an immutable constant — frozen in time.

\`\`\`flowlang
seal MAX_POWER = 9000
-- MAX_POWER = 10000  -- forbidden!
\`\`\``,
        'cast': `### 📖 Grimoire — cast Spell
> *"Speaking the incantation into existence"*

**cast Spell** declares a function — reusable magic.

\`\`\`flowlang
cast Spell heal(Ember amount) -> Ember {
    return amount * 2
}
\`\`\``,
        'ritual': `### 📖 Grimoire — ritual
> *"A ceremony that spans across time itself"*

**ritual** declares an async function — magic that waits.

\`\`\`flowlang
ritual fetchPower ::
    wait 1s
    return 100
end
\`\`\`

Rituals return Promises. Invoke with \`evoke\`.`,
        'Stance': `### 📖 Grimoire — in Stance
> *"The warrior assumes a defensive position"*

**in Stance** is a conditional — branching paths.

\`\`\`flowlang
in Stance (power >> 9000) {
    shout("Over 9000!")
} shift (power >> 5000) {
    shout("Strong!")
} abandon {
    shout("Weak...")
}
\`\`\``,
        'shift': `### 📖 Grimoire — shift
> *"Changing form mid-battle"*

**shift** is else-if — an alternative stance.

\`\`\`flowlang
in Stance (x >> 10) {
    shout("High")
} shift (x >> 5) {
    shout("Medium")
}
\`\`\``,
        'abandon': `### 📖 Grimoire — abandon
> *"When all stances fail, surrender"*

**abandon** is else — the final fallback.

\`\`\`flowlang
in Stance (win) {
    celebrate()
} abandon {
    despair()
}
\`\`\``,
        'Phase': `### 📖 Grimoire — enter Phase
> *"Time loops upon itself"*

**enter Phase** is a loop — repeating actions.

\`\`\`flowlang
enter Phase i from 1 to 10 {
    shout(i)
}

enter Phase item in items {
    process(item)
}

enter Phase until (done) {
    keepWorking()
}
\`\`\``,
        'Aura': `### 📖 Grimoire — invoke Aura
> *"The essence reveals its true form"*

**invoke Aura** is pattern matching — switching on values.

\`\`\`flowlang
invoke Aura element {
    when "fire" -> shout("🔥")
    when "water" -> shout("💧")
    otherwise -> shout("❓")
}
\`\`\``,
        'attempt': `### 📖 Grimoire — attempt
> *"Venturing into dangerous territory"*

**attempt** is try-catch — handling errors.

\`\`\`flowlang
attempt {
    dangerousSpell()
} rescue Rift as e {
    shout("Failed: " + e)
} finally {
    cleanup()
}
\`\`\``,
        'rescue': `### 📖 Grimoire — rescue
> *"A guardian catches the fallen"*

**rescue** catches errors from attempt blocks.

\`\`\`flowlang
attempt {
    riskyOperation()
} rescue Rift as error {
    shout("Caught: " + error)
}
\`\`\``,
        'rupture': `### 📖 Grimoire — rupture
> *"Tearing a hole in reality"*

**rupture** throws an error — breaking execution flow.

\`\`\`flowlang
rupture Rift "Something went wrong!"
\`\`\`

Error types: \`Rift\`, \`Glitch\`, \`Spirit\`, \`VoidTear\``,
        'panic': `### 📖 Grimoire — panic
> *"The ultimate catastrophe"*

**panic** throws an unrecoverable error — stops everything.

\`\`\`flowlang
panic "Critical system failure!"
\`\`\``,
        'ward': `### 📖 Grimoire — ward
> *"A protective barrier of truth"*

**ward** is an assertion — fails if condition is false.

\`\`\`flowlang
ward (power >> 0) else "Power must be positive"
\`\`\``,
        'circle': `### 📖 Grimoire — circle
> *"Drawing the summoning circle"*

**circle** imports a module — bringing external power.

\`\`\`flowlang
circle json from "std:json"
circle math from "std:math"
circle helper from "./helper.flow"
\`\`\``,
        'shout': `### 📖 Grimoire — shout
> *"Projecting your voice to the world"*

**shout** prints to console with newline.

\`\`\`flowlang
shout("Hello, World!")
\`\`\``,
        'whisper': `### 📖 Grimoire — whisper
> *"A quiet murmur"*

**whisper** prints without newline.

\`\`\`flowlang
whisper("Loading")
whisper(".")
whisper(".")
\`\`\``,
        'evoke': `### 📖 Grimoire — evoke
> *"Awaiting the ritual's completion"*

**evoke** is await — waits for async operations.

\`\`\`flowlang
let data = evoke fetchData()
\`\`\``,
        'export': `### 📖 Grimoire — @export
> *"Sharing power with other realms"*

**@export** makes functions/variables available to importers.

\`\`\`flowlang
@export
cast Spell publicSpell() -> Hollow {
    shout("I can be imported!")
}
\`\`\``,
        'sigil': `### 📖 Grimoire — sigil
> *"The sacred rune that binds essence into form"*

**sigil** defines a custom structure — your own type.

\`\`\`flowlang
sigil Character {
    name: Silk
    level: Ember
    skills: Constellation<Silk>
}

let hero = Character {
    name: "Aethon",
    level: 42,
    skills: ["fireball", "heal"]
}
\`\`\`

Access fields with \`hero.name\`, \`hero.level\``,
    };

    if (keywordInfo[word]) {
        return {
            contents: { kind: MarkupKind.Markdown, value: keywordInfo[word] },
        };
    }

    // Check if it's a variable/function in the AST
    for (const stmt of ast.statements) {
        if (stmt.type === 'LetStatement' && stmt.name === word) {
            let typeStr: string;
            let isInferred = false;

            if (stmt.typeAnnotation) {
                typeStr = formatType(stmt.typeAnnotation);
            } else {
                typeStr = inferTypeFromValue(stmt.value);
                isInferred = typeStr !== 'Flux';
            }

            const inferNote = isInferred ? ` *(inferred)*` : '';
            return {
                contents: {
                    kind: MarkupKind.Markdown,
                    value: `**let** ${word}: ${typeStr}${inferNote}\n\nMutable variable`,
                },
            };
        }
        if (stmt.type === 'SealStatement' && stmt.name === word) {
            let typeStr: string;
            let isInferred = false;

            if (stmt.typeAnnotation) {
                typeStr = formatType(stmt.typeAnnotation);
            } else {
                typeStr = inferTypeFromValue(stmt.value);
                isInferred = typeStr !== 'Flux';
            }

            const inferNote = isInferred ? ` *(inferred)*` : '';
            return {
                contents: {
                    kind: MarkupKind.Markdown,
                    value: `**seal** ${word}: ${typeStr}${inferNote}\n\nImmutable constant`,
                },
            };
        }
        if (stmt.type === 'FunctionDecl' && stmt.name === word) {
            const params = stmt.params.map(p => {
                const typeStr = p.typeAnnotation ? formatType(p.typeAnnotation) : 'Flux';
                return `${p.name}: ${typeStr}`;
            }).join(', ');
            const retType = stmt.returnType ? formatType(stmt.returnType) : 'Hollow';
            return {
                contents: {
                    kind: MarkupKind.Markdown,
                    value: `**cast Spell** ${word}(${params}) -> ${retType}`,
                },
            };
        }
        if (stmt.type === 'SigilDecl' && stmt.name === word) {
            const fields = stmt.fields.map(f => {
                const typeStr = formatType(f.typeAnnotation);
                return `    ${f.name}: ${typeStr}`;
            }).join('\n');

            return {
                contents: {
                    kind: MarkupKind.Markdown,
                    value: `**sigil** ${word} {\n${fields}\n}`,
                },
            };
        }
    }

    return null;
});

// 🛡️ Type Inference from Value
function inferTypeFromValue(expr: any): string {
    if (!expr) return 'Flux';

    switch (expr.type) {
        case 'NumberLiteral':
            return 'Ember';
        case 'StringLiteral':
            return 'Silk';
        case 'BooleanLiteral':
            return 'Pulse';
        case 'ArrayLiteral':
            if (expr.elements && expr.elements.length > 0) {
                const elemType = inferTypeFromValue(expr.elements[0]);
                return `Constellation<${elemType}>`;
            }
            return 'Constellation<Flux>';
        case 'RelicLiteral':
            if (expr.entries && expr.entries.length > 0) {
                const valType = inferTypeFromValue(expr.entries[0].value);
                return `Relic<Silk, ${valType}>`;
            }
            return 'Relic<Silk, Flux>';
        case 'NullLiteral':
            return 'Hollow';
        case 'CallExpr':
            // Could check function return type, for now return Flux
            return 'Flux';
        case 'MethodCallExpr':
            return 'Flux';
        case 'Identifier':
            return 'Flux'; // Would need scope tracking
        default:
            return 'Flux';
    }
}

function formatType(typeExpr: any): string {
    if (!typeExpr) return 'Flux';
    if (typeExpr.type === 'simple') return typeExpr.name;
    if (typeExpr.type === 'generic') {
        const params = typeExpr.params.map(formatType).join(', ');
        return `${typeExpr.name}<${params}>`;
    }
    return 'Flux';
}

// ✨ Purification — Code Formatter
connection.onDocumentFormatting((params: DocumentFormattingParams): TextEdit[] => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return [];

    const text = document.getText();
    const formatted = purifyCode(text, params.options.tabSize || 4);

    // Return single edit replacing whole document
    return [{
        range: {
            start: { line: 0, character: 0 },
            end: { line: document.lineCount, character: 0 },
        },
        newText: formatted,
    }];
});

// Purification formatter logic
function purifyCode(source: string, indentSize: number): string {
    const lines = source.split(/\r?\n/);
    const result: string[] = [];
    let indentLevel = 0;
    const indent = ' '.repeat(indentSize);

    // Block starters and enders
    const blockStart = /^(in Stance|shift|abandon|enter Phase|invoke Aura|attempt|rescue|finally|when|otherwise|cast Spell|ritual)\b|{\s*$/;
    const blockEnd = /^(}|end)\s*$/;
    const midBlock = /^(shift|abandon|rescue|finally|otherwise|when)\b|^}\s*(shift|abandon|rescue|finally)/;

    for (let i = 0; i < lines.length; i++) {
        let line = lines[i];

        // Trim whitespace
        let trimmed = line.trim();

        // Skip empty lines but preserve them
        if (trimmed === '') {
            result.push('');
            continue;
        }

        // Format specific patterns
        trimmed = formatLine(trimmed);

        // Handle dedent before mid-block keywords
        if (midBlock.test(trimmed)) {
            indentLevel = Math.max(0, indentLevel - 1);
        }
        // Handle dedent for block end
        else if (blockEnd.test(trimmed)) {
            indentLevel = Math.max(0, indentLevel - 1);
        }

        // Apply indentation
        const formattedLine = indent.repeat(indentLevel) + trimmed;
        result.push(formattedLine);

        // Handle indent after block start
        if (blockStart.test(trimmed) && !trimmed.endsWith('end')) {
            indentLevel++;
        }
        // Re-indent after mid-block keywords (they open new blocks)
        if (midBlock.test(trimmed) && trimmed.endsWith('{')) {
            indentLevel++;
        }
    }

    return result.join('\n');
}

function formatLine(line: string): string {
    // Use placeholders to protect multi-character operators
    const PLACEHOLDER_ARROW = '\x00ARROW\x00';
    const PLACEHOLDER_DOUBLE_COLON = '\x00DCOLON\x00';
    const PLACEHOLDER_GTE = '\x00GTE\x00';
    const PLACEHOLDER_LTE = '\x00LTE\x00';
    const PLACEHOLDER_GT = '\x00GT\x00';
    const PLACEHOLDER_LT = '\x00LT\x00';
    const PLACEHOLDER_COMMENT = '\x00COMMENT\x00';

    // Preserve comment at end of line
    let comment = '';
    const commentMatch = line.match(/--.*$/);
    if (commentMatch) {
        comment = commentMatch[0];
        line = line.substring(0, line.length - comment.length);
    }

    // Replace multi-char operators with placeholders
    line = line.replace(/->/g, PLACEHOLDER_ARROW);
    line = line.replace(/::/g, PLACEHOLDER_DOUBLE_COLON);
    line = line.replace(/>>=|<<=/g, (m) => m === '>>=' ? PLACEHOLDER_GTE : PLACEHOLDER_LTE);
    line = line.replace(/>>/g, PLACEHOLDER_GT);
    line = line.replace(/<</g, PLACEHOLDER_LT);

    // Normalize spacing around single operators (now safe)
    line = line.replace(/\s*(:)\s*(?!\x00)/g, ': ');
    line = line.replace(/\s*(,)\s*/g, ', ');

    // Normalize arithmetic operators - but not minus at start or after operators
    line = line.replace(/(\S)\s*(\+)\s*/g, '$1 + ');
    line = line.replace(/(\S)\s*(\*)\s*/g, '$1 * ');
    line = line.replace(/(\S)\s*(\/)\s*/g, '$1 / ');
    line = line.replace(/(\S)\s*(%)\s*/g, '$1 % ');
    // Minus: only if preceded by non-operator character (not after = or operator)
    line = line.replace(/([^\s=+\-*/%<>])\s*(-)\s*(\S)/g, '$1 - $3');

    // Normalize assignment (but not ==, >=, etc.)
    line = line.replace(/([^=<>!])\s*(=)\s*([^=])/g, '$1 = $3');

    // Restore multi-char operators with proper spacing
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_ARROW.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' -> ');
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_DOUBLE_COLON.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' :: ');
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_GTE.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' >>= ');
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_LTE.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' <<= ');
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_GT.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' >> ');
    line = line.replace(new RegExp(`\\s*${PLACEHOLDER_LT.replace(/\x00/g, '\\x00')}\\s*`, 'g'), ' << ');

    // Normalize is~ and not~
    line = line.replace(/\s*(is~|not~)\s*/g, ' $1 ');

    // Fix double spaces
    line = line.replace(/  +/g, ' ');

    // Fix space before opening paren in function calls (remove extra)
    line = line.replace(/(\w)\s+\(/g, '$1(');

    // Ensure space after keywords before (
    line = line.replace(/\b(in Stance|enter Phase|invoke Aura|attempt|rescue|when)\(/g, '$1 (');

    // Ensure space before {
    line = line.replace(/(\S){/g, '$1 {');

    // Normalize boolean operators
    line = line.replace(/\s*(both!|either!)\s*/g, ' $1 ');

    // Restore comment
    line = line.trim();
    if (comment) {
        line = line + '  ' + comment;
    }

    return line;
}

// ════════════════════════════════════════════════════════════════════════════
// 🔮 HIGH IMPACT LSP FEATURES
// ════════════════════════════════════════════════════════════════════════════

// 📋 Document Symbols — Shows outline in sidebar (Ctrl+Shift+O)
connection.onDocumentSymbol((params): DocumentSymbol[] => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return [];

    const ast = documentASTs.get(params.textDocument.uri);
    if (!ast) return [];

    const symbols: DocumentSymbol[] = [];

    for (const stmt of ast.statements) {
        if (stmt.type === 'FunctionDecl') {
            const funcStmt = stmt as any;
            const stmtLine = stmt.range.start.line - 1;
            const params = funcStmt.params?.map((p: any) => {
                const typeStr = p.typeAnnotation ? formatType(p.typeAnnotation) : 'Flux';
                return `${p.name}: ${typeStr}`;
            }).join(', ') || '';
            const returnType = funcStmt.returnType ? formatType(funcStmt.returnType) : 'Hollow';

            symbols.push({
                name: funcStmt.name,
                detail: `(${params}) -> ${returnType}`,
                kind: SymbolKind.Function,
                range: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: 100 },
                },
                selectionRange: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: funcStmt.name.length + 11 },
                },
            });
        }

        if (stmt.type === 'RitualDecl') {
            const ritualStmt = stmt as any;
            const stmtLine = stmt.range.start.line - 1;
            symbols.push({
                name: ritualStmt.name,
                detail: 'ritual (async)',
                kind: SymbolKind.Event,
                range: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: 100 },
                },
                selectionRange: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: ritualStmt.name.length + 7 },
                },
            });
        }

        if (stmt.type === 'SealStatement') {
            const sealStmt = stmt as any;
            const stmtLine = stmt.range.start.line - 1;
            symbols.push({
                name: sealStmt.name,
                detail: 'seal (constant)',
                kind: SymbolKind.Constant,
                range: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: 100 },
                },
                selectionRange: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: sealStmt.name.length + 5 },
                },
            });
        }

        if (stmt.type === 'SigilDecl') {
            const sigilStmt = stmt as any;
            const stmtLine = stmt.range.start.line - 1;
            symbols.push({
                name: sigilStmt.name,
                detail: 'sigil (struct)',
                kind: SymbolKind.Struct,
                range: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: 100 },
                },
                selectionRange: {
                    start: { line: stmtLine, character: 0 },
                    end: { line: stmtLine, character: sigilStmt.name.length + 6 },
                },
            });
        }
    }

    return symbols;
});

// ⚡ Go to Definition — Jump to symbol definition (Ctrl+Click)
connection.onDefinition((params): Definition | null => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return null;

    const ast = documentASTs.get(params.textDocument.uri);
    if (!ast) return null;

    // Get word at position
    const text = document.getText();
    const offset = document.offsetAt(params.position);

    let start = offset;
    let end = offset;
    while (start > 0 && /[a-zA-Z0-9_]/.test(text[start - 1])) start--;
    while (end < text.length && /[a-zA-Z0-9_]/.test(text[end])) end++;

    const word = text.substring(start, end);
    if (!word) return null;

    // Search for definition in statements
    for (const stmt of ast.statements) {
        if (stmt.type === 'FunctionDecl' && (stmt as any).name === word) {
            const stmtRange = stmt.range;
            return {
                uri: params.textDocument.uri,
                range: {
                    start: { line: stmtRange.start.line - 1, character: stmtRange.start.column - 1 },
                    end: { line: stmtRange.start.line - 1, character: stmtRange.start.column + word.length + 10 },
                },
            };
        }

        if (stmt.type === 'RitualDecl' && (stmt as any).name === word) {
            const stmtRange = stmt.range;
            return {
                uri: params.textDocument.uri,
                range: {
                    start: { line: stmtRange.start.line - 1, character: stmtRange.start.column - 1 },
                    end: { line: stmtRange.start.line - 1, character: stmtRange.start.column + word.length + 6 },
                },
            };
        }

        if (stmt.type === 'LetStatement' && (stmt as any).name === word) {
            const stmtRange = stmt.range;
            return {
                uri: params.textDocument.uri,
                range: {
                    start: { line: stmtRange.start.line - 1, character: stmtRange.start.column - 1 },
                    end: { line: stmtRange.start.line - 1, character: stmtRange.start.column + word.length + 3 },
                },
            };
        }

        if (stmt.type === 'SealStatement' && (stmt as any).name === word) {
            const stmtRange = stmt.range;
            return {
                uri: params.textDocument.uri,
                range: {
                    start: { line: stmtRange.start.line - 1, character: stmtRange.start.column - 1 },
                    end: { line: stmtRange.start.line - 1, character: stmtRange.start.column + word.length + 4 },
                },
            };
        }
    }

    // Check imports (circle statements)
    for (const imp of ast.imports) {
        if (imp.module === word) {
            const line = imp.range?.start?.line || 1;
            return {
                uri: params.textDocument.uri,
                range: {
                    start: { line: line - 1, character: 0 },
                    end: { line: line - 1, character: 50 },
                },
            };
        }
    }

    return null;
});

// 🔍 Find All References — Find all usages of a symbol (Shift+F12)
connection.onReferences((params): Location[] => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return [];

    // Get word at position
    const text = document.getText();
    const offset = document.offsetAt(params.position);

    let start = offset;
    let end = offset;
    while (start > 0 && /[a-zA-Z0-9_]/.test(text[start - 1])) start--;
    while (end < text.length && /[a-zA-Z0-9_]/.test(text[end])) end++;

    const word = text.substring(start, end);
    if (!word) return [];

    const locations: Location[] = [];

    // Search for all occurrences of the word
    const regex = new RegExp(`\\b${word}\\b`, 'g');
    let match;

    while ((match = regex.exec(text)) !== null) {
        const pos = document.positionAt(match.index);
        locations.push({
            uri: params.textDocument.uri,
            range: {
                start: pos,
                end: { line: pos.line, character: pos.character + word.length },
            },
        });
    }

    return locations;
});

// ✍️ Signature Help — Show function parameters while typing
connection.onSignatureHelp((params): SignatureHelp | null => {
    const document = documents.get(params.textDocument.uri);
    if (!document) return null;

    const ast = documentASTs.get(params.textDocument.uri);
    if (!ast) return null;

    // Get the text before cursor to find function name
    const text = document.getText();
    const offset = document.offsetAt(params.position);
    const textBefore = text.substring(0, offset);

    // Find the function name before the opening paren
    const funcCallMatch = textBefore.match(/(\w+)\s*\([^)]*$/);
    if (!funcCallMatch) return null;

    const funcName = funcCallMatch[1];

    // Count commas to determine active parameter
    const afterParen = textBefore.substring(textBefore.lastIndexOf('(') + 1);
    const activeParam = (afterParen.match(/,/g) || []).length;

    // Look up function signature
    for (const stmt of ast.statements) {
        if (stmt.type === 'FunctionDecl' && (stmt as any).name === funcName) {
            const funcStmt = stmt as any;
            const params = funcStmt.params || [];

            const paramLabels = params.map((p: any) => {
                const typeStr = p.typeAnnotation ? formatType(p.typeAnnotation) : 'Flux';
                return `${p.name}: ${typeStr}`;
            });

            const returnType = funcStmt.returnType ? formatType(funcStmt.returnType) : 'Hollow';
            const signature = `${funcName}(${paramLabels.join(', ')}) -> ${returnType}`;

            return {
                signatures: [{
                    label: signature,
                    documentation: {
                        kind: MarkupKind.Markdown,
                        value: `**${funcName}** - Function\n\nReturns: \`${returnType}\``,
                    },
                    parameters: paramLabels.map((label: string) => ({
                        label,
                    })),
                }],
                activeSignature: 0,
                activeParameter: Math.min(activeParam, params.length - 1),
            };
        }
    }

    // Check stdlib functions
    const STDLIB_SIGNATURES: Record<string, { signature: string; params: string[]; doc: string }> = {
        'shout': { signature: 'shout(message: Flux) -> Hollow', params: ['message: Flux'], doc: 'Print to console with newline' },
        'whisper': { signature: 'whisper(message: Flux) -> Hollow', params: ['message: Flux'], doc: 'Print to console without newline' },
        'len': { signature: 'len(collection: Constellation) -> Ember', params: ['collection: Constellation'], doc: 'Get length of array' },
        'push': { signature: 'push(array: Constellation, item: Flux) -> Hollow', params: ['array: Constellation', 'item: Flux'], doc: 'Add item to array' },
        'pop': { signature: 'pop(array: Constellation) -> Flux', params: ['array: Constellation'], doc: 'Remove and return last item' },
    };

    if (STDLIB_SIGNATURES[funcName]) {
        const sig = STDLIB_SIGNATURES[funcName];
        return {
            signatures: [{
                label: sig.signature,
                documentation: {
                    kind: MarkupKind.Markdown,
                    value: sig.doc,
                },
                parameters: sig.params.map(p => ({ label: p })),
            }],
            activeSignature: 0,
            activeParameter: Math.min(activeParam, sig.params.length - 1),
        };
    }

    return null;
});

// Start listening
documents.listen(connection);
connection.listen();
