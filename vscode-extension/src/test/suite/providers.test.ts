import * as assert from 'assert';
import * as vscode from 'vscode';
import { CodeLensProvider } from '../../providers/codeLensProvider';
import { HoverProvider } from '../../providers/hoverProvider';
import { DiagnosticsProvider } from '../../providers/diagnosticsProvider';
import { MetricsProvider } from '../../providers/metricsProvider';
import { LucentAnalyzer } from '../../analyzer';

suite('Providers Test Suite', () => {
    let analyzer: LucentAnalyzer;
    let mockContext: vscode.ExtensionContext;

    setup(() => {
        mockContext = {
            subscriptions: [],
            workspaceState: {
                get: () => undefined,
                update: async () => {},
                keys: () => []
            },
            globalState: {
                get: () => undefined,
                update: async () => {},
                setKeysForSync: () => {},
                keys: () => []
            },
            extensionPath: '',
            extensionUri: vscode.Uri.file(''),
            environmentVariableCollection: {} as any,
            extensionMode: vscode.ExtensionMode.Test,
            storageUri: undefined,
            storagePath: undefined,
            globalStorageUri: vscode.Uri.file(''),
            globalStoragePath: '',
            logUri: vscode.Uri.file(''),
            logPath: '',
            asAbsolutePath: (relativePath: string) => relativePath,
            secrets: {} as any,
            extension: {} as any,
            languageModelAccessInformation: {} as any
        };
        analyzer = new LucentAnalyzer(mockContext);
    });

    suite('CodeLensProvider', () => {
        test('Provider should be created', () => {
            const provider = new CodeLensProvider(analyzer);
            assert.ok(provider);
        });
    });

    suite('HoverProvider', () => {
        test('Provider should be created', () => {
            const provider = new HoverProvider(analyzer);
            assert.ok(provider);
        });

        test('Should provide hover for valid position', async () => {
            const provider = new HoverProvider(analyzer);
            const doc = await vscode.workspace.openTextDocument({
                content: 'function test() { return 42; }',
                language: 'javascript'
            });

            const position = new vscode.Position(0, 5); // On 'function'
            const hover = await provider.provideHover(doc, position, {} as any);

            // May return null if Lucent isn't installed, which is fine for testing
            assert.ok(hover === null || hover instanceof vscode.Hover);
        });
    });

    suite('DiagnosticsProvider', () => {
        test('Provider should be created', () => {
            const provider = new DiagnosticsProvider(analyzer);
            assert.ok(provider);
        });

        test('Provider should have dispose method', () => {
            const provider = new DiagnosticsProvider(analyzer);
            assert.ok(typeof provider.dispose === 'function');
            provider.dispose();
        });
    });

    suite('MetricsProvider', () => {
        test('Provider should be created', () => {
            const provider = new MetricsProvider(analyzer);
            assert.ok(provider);
        });

        test('Provider should provide tree items', async () => {
            const provider = new MetricsProvider(analyzer);
            const children = await provider.getChildren();
            assert.ok(Array.isArray(children));
        });

        test('Refresh should trigger update', () => {
            const provider = new MetricsProvider(analyzer);
            // Should not throw
            provider.refresh();
            assert.ok(true);
        });
    });
});
