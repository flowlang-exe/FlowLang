// FlowLang VS Code Extension - Client

import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    // Server module path
    const serverModule = context.asAbsolutePath(path.join('out', 'server', 'server.js'));

    // Server options
    const serverOptions: ServerOptions = {
        run: {
            module: serverModule,
            transport: TransportKind.ipc,
        },
        debug: {
            module: serverModule,
            transport: TransportKind.ipc,
            options: { execArgv: ['--nolazy', '--inspect=6009'] },
        },
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'flowlang' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.flow'),
        },
    };

    // Create client
    client = new LanguageClient(
        'flowlang',
        'FlowLang Language Server',
        serverOptions,
        clientOptions
    );

    // Start client
    client.start();

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('flowlang.showInfo', () => {
            vscode.window.showInformationMessage(
                '🌌 FlowLang - A mystical anime-themed scripting language! ⚔️'
            );
        })
    );

    console.log('✨ FlowLang extension activated!');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
