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

    // Register ic10.showRelated command
    context.subscriptions.push(vscode.commands.registerCommand('ic10.showRelated', async (instruction?: string) => {
        // If instruction not provided, try to get current word at cursor
        if (!instruction) {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showInformationMessage('No active editor found');
                return;
            }
            
            const position = editor.selection.active;
            const range = editor.document.getWordRangeAtPosition(position);
            if (!range) {
                vscode.window.showInformationMessage('No instruction found at cursor');
                return;
            }
            
            instruction = editor.document.getText(range);
        }

        // Map of instruction to related instructions (simplified version for demo)
        const relatedInstructions: { [key: string]: string[] } = {
            'add': ['sub', 'mul', 'div', 'mod'],
            'sub': ['add', 'mul', 'div', 'mod'],
            'mul': ['add', 'sub', 'div', 'mod'],
            'div': ['add', 'sub', 'mul', 'mod'],
            'l': ['s', 'lb', 'sb', 'lr', 'ls', 'ld', 'sd'],
            's': ['l', 'lb', 'sb', 'lr', 'ls', 'ld', 'sd'],
            'beq': ['bne', 'blt', 'bgt', 'ble', 'bge', 'breq', 'beqz'],
            'bne': ['beq', 'blt', 'bgt', 'ble', 'bge', 'brne', 'bnez']
        };

        const related = relatedInstructions[instruction.toLowerCase()];
        if (!related || related.length === 0) {
            vscode.window.showInformationMessage(`No related instructions found for '${instruction}'`);
            return;
        }

        const selectedInstruction = await vscode.window.showQuickPick(
            related.map(instr => ({
                label: instr,
                description: `Related to ${instruction}`
            })),
            {
                placeHolder: `Instructions related to ${instruction}`,
                canPickMany: false
            }
        );

        if (selectedInstruction) {
            // Insert the selected instruction at cursor
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const position = editor.selection.active;
                editor.edit(editBuilder => {
                    editBuilder.insert(position, selectedInstruction.label);
                });
            }
        }
    }));

    // Register ic10.searchCategory command
    context.subscriptions.push(vscode.commands.registerCommand('ic10.searchCategory', async (category?: string) => {
        // Map of categories to instructions
        const categories: { [key: string]: string[] } = {
            'Arithmetic': ['add', 'sub', 'mul', 'div', 'mod', 'abs', 'sqrt'],
            'Device I/O': ['l', 's', 'lr', 'ls', 'ld', 'sd', 'ss'],
            'Batch Operations': ['lb', 'sb', 'lbn', 'lbs', 'sbn', 'sbs'],
            'Branching': ['beq', 'bne', 'blt', 'bgt', 'ble', 'bge', 'beqz', 'bnez'],
            'Control Flow': ['j', 'jr', 'jal'],
            'Comparison': ['slt', 'sgt', 'sle', 'sge', 'seq', 'sne'],
            'Logic': ['and', 'or', 'xor', 'nor']
        };

        if (!category) {
            // Show category picker first
            const selectedCategory = await vscode.window.showQuickPick(
                Object.keys(categories).map(cat => ({
                    label: cat,
                    description: `${categories[cat].length} instructions`
                })),
                {
                    placeHolder: 'Select instruction category',
                    canPickMany: false
                }
            );

            if (!selectedCategory) {
                return;
            }
            category = selectedCategory.label;
        }

        const instructions = categories[category];
        if (!instructions || instructions.length === 0) {
            vscode.window.showInformationMessage(`No instructions found in category '${category}'`);
            return;
        }

        const selectedInstruction = await vscode.window.showQuickPick(
            instructions.map(instr => ({
                label: instr,
                description: `${category} instruction`
            })),
            {
                placeHolder: `${category} instructions`,
                canPickMany: false
            }
        );

        if (selectedInstruction) {
            // Insert the selected instruction at cursor
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const position = editor.selection.active;
                editor.edit(editBuilder => {
                    editBuilder.insert(position, selectedInstruction.label);
                });
            }
        }
    }));

}

// This method is called when your extension is deactivated
export function deactivate() { }
