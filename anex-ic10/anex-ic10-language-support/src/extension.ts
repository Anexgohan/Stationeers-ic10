// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import * as path from 'path';
import * as net from 'net';
import {
    DidChangeConfigurationNotification,
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    StreamInfo,
    ExecuteCommandParams
} from 'vscode-languageclient/node';

function getLSPIC10Configurations(): vscode.WorkspaceConfiguration {
    return vscode.workspace.getConfiguration('ic10.lsp');
}

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

    // Activate Notification through VSCode Notifications
    vscode.window.showInformationMessage('IC10 Language Server is now active!');


    const serverBinary = process.platform === "win32" ? "ic10lsp.exe" : "ic10lsp";

    // The server is implemented in the upstream language server
    const serverModule = context.asAbsolutePath(path.join('bin', serverBinary));

    const config = vscode.workspace.getConfiguration();

    const useRemoteLanguageServer = config.get('ic10.useRemoteLanguageServer') as boolean;

    let serverOptions: ServerOptions;

    if (useRemoteLanguageServer) {

        const remoteLanguageServerHost = config.get('ic10.remoteLanguageServerHost') as string;
        const remoteLanguageServerPort = config.get('ic10.remoteLanguageServerPort') as number;

        let connectionInfo = {
            host: remoteLanguageServerHost,
            port: remoteLanguageServerPort
        };
        serverOptions = () => {
            // Connect to language server via socket
            let socket = net.connect(connectionInfo);
            let result: StreamInfo = {
                writer: socket,
                reader: socket
            };
            return Promise.resolve(result);
        };
    }
    else {
        serverOptions = {
            run: { command: serverModule },
            debug: { command: serverModule },
        };
    }

    // Options to control the language client
    const clientOptions: LanguageClientOptions = {
        // Register the server for IC10 MIPS-like language documents
        documentSelector: [
            { scheme: 'file', language: 'ic10' },
            { scheme: 'untitled', language: 'ic10' }
        ],
    };

    // Create the language client and start the client.
    const lc = new LanguageClient(
        'ic10',
        'IC10 Language Server',
        serverOptions,
        clientOptions
    )

    // Push the disposable to the context's subscriptions so that the
    // client can be deactivated on extension deactivation
    //context.subscriptions.push(disposable);
    lc.start().then(() => {
        context.subscriptions.push(lc);
    });

    // Initial config
    lc.sendNotification(DidChangeConfigurationNotification.type, { settings: getLSPIC10Configurations() });

    // Register configuration changes to sendNotification.
    vscode.workspace.onDidChangeConfiguration((e: vscode.ConfigurationChangeEvent) => {
        if (e.affectsConfiguration('ic10.lsp')) {
            lc.sendNotification(DidChangeConfigurationNotification.type, { settings: getLSPIC10Configurations() });
        }
    })

    // Register commands
    context.subscriptions.push(vscode.commands.registerCommand('ic10.lsp.restart', () => {
        vscode.window.showInformationMessage('Restarting IC10 Language Server...');
        lc.stop().then(() => lc.start());
    }    ));

    // Register ic10.lsp.version command
    context.subscriptions.push(vscode.commands.registerCommand('ic10.lsp.version', () => {
        // ExecuteCommandOptions
        const options: ExecuteCommandParams = {
            command: 'version',
            arguments: []
        };

        lc.sendRequest('workspace/executeCommand', options);
    }   ));

}

// This method is called when your extension is deactivated
export function deactivate() { }
