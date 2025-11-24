import * as vscode from 'vscode';
import { LucentAnalyzer } from './analyzer';
import { MetricsProvider } from './providers/metricsProvider';
import { CodeLensProvider } from './providers/codeLensProvider';
import { HoverProvider } from './providers/hoverProvider';
import { DiagnosticsProvider } from './providers/diagnosticsProvider';
import { StatusBarManager } from './statusBar';
import { MetricsPanel } from './panels/metricsPanel';
import { TrendsPanel } from './panels/trendsPanel';

let analyzer: LucentAnalyzer;
let diagnosticsProvider: DiagnosticsProvider;
let statusBarManager: StatusBarManager;

export function activate(context: vscode.ExtensionContext) {
    console.log('Lucent extension is now active');

    // Initialize core components
    analyzer = new LucentAnalyzer(context);
    diagnosticsProvider = new DiagnosticsProvider(analyzer);
    statusBarManager = new StatusBarManager(analyzer);

    // Register providers
    registerProviders(context);

    // Register commands
    registerCommands(context);

    // Setup event handlers
    setupEventHandlers(context);

    // Initial analysis if enabled
    const config = vscode.workspace.getConfiguration('lucent');
    if (config.get('analyzeOnOpen')) {
        analyzeActiveEditor();
    }
}

function registerProviders(context: vscode.ExtensionContext) {
    // Metrics tree view provider
    const metricsProvider = new MetricsProvider(analyzer);
    context.subscriptions.push(
        vscode.window.registerTreeDataProvider('lucent.metricsView', metricsProvider)
    );

    // Code lens provider (inline metrics)
    const codeLensProvider = new CodeLensProvider(analyzer);
    context.subscriptions.push(
        vscode.languages.registerCodeLensProvider(
            { scheme: 'file' },
            codeLensProvider
        )
    );

    // Hover provider (detailed tooltips)
    const hoverProvider = new HoverProvider(analyzer);
    context.subscriptions.push(
        vscode.languages.registerHoverProvider(
            { scheme: 'file' },
            hoverProvider
        )
    );
}

function registerCommands(context: vscode.ExtensionContext) {
    // Analyze current file
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.analyzeFile', async () => {
            await analyzeActiveEditor();
        })
    );

    // Analyze entire workspace
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.analyzeWorkspace', async () => {
            await vscode.window.withProgress(
                {
                    location: vscode.ProgressLocation.Notification,
                    title: 'Analyzing workspace with Lucent...',
                    cancellable: false
                },
                async () => {
                    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                    if (workspaceFolder) {
                        await analyzer.analyzeWorkspace(workspaceFolder.uri.fsPath);
                        vscode.window.showInformationMessage('Workspace analysis complete!');
                    }
                }
            );
        })
    );

    // Show metrics panel
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.showMetricsPanel', () => {
            MetricsPanel.createOrShow(analyzer, context.extensionUri);
        })
    );

    // Show trends panel
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.showTrends', () => {
            TrendsPanel.createOrShow(analyzer, context.extensionUri);
        })
    );

    // Clear cache
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.clearCache', async () => {
            await analyzer.clearCache();
            vscode.window.showInformationMessage('Lucent cache cleared');
        })
    );

    // Toggle inline diagnostics
    context.subscriptions.push(
        vscode.commands.registerCommand('lucent.toggleInlineDiagnostics', () => {
            const config = vscode.workspace.getConfiguration('lucent');
            const current = config.get('showInlineMetrics');
            config.update('showInlineMetrics', !current, vscode.ConfigurationTarget.Global);
            vscode.window.showInformationMessage(
                `Inline metrics ${!current ? 'enabled' : 'disabled'}`
            );
        })
    );
}

function setupEventHandlers(context: vscode.ExtensionContext) {
    // Analyze on save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (document) => {
            const config = vscode.workspace.getConfiguration('lucent');
            if (config.get('analyzeOnSave')) {
                await analyzer.analyzeFile(document.uri.fsPath);
                await diagnosticsProvider.updateDiagnostics(document.uri);
                statusBarManager.update();
            }
        })
    );

    // Update on active editor change
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(async (editor) => {
            if (editor) {
                const config = vscode.workspace.getConfiguration('lucent');
                if (config.get('analyzeOnOpen')) {
                    await analyzer.analyzeFile(editor.document.uri.fsPath);
                }
                statusBarManager.update();
            }
        })
    );

    // Update on configuration change
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration((e) => {
            if (e.affectsConfiguration('lucent')) {
                statusBarManager.update();
            }
        })
    );
}

async function analyzeActiveEditor() {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Window,
                title: 'Analyzing with Lucent...'
            },
            async () => {
                await analyzer.analyzeFile(editor.document.uri.fsPath);
                await diagnosticsProvider.updateDiagnostics(editor.document.uri);
                statusBarManager.update();
            }
        );
    }
}

export function deactivate() {
    diagnosticsProvider?.dispose();
    statusBarManager?.dispose();
}
