import * as assert from 'assert';
import * as vscode from 'vscode';
import { LucentAnalyzer } from '../../analyzer';

suite('LucentAnalyzer Test Suite', () => {
    vscode.window.showInformationMessage('Start LucentAnalyzer tests.');

    let analyzer: LucentAnalyzer;

    suiteSetup(async () => {
        const extension = vscode.extensions.getExtension('lucent.lucent-code-metrics');
        if (extension) {
            await extension.activate();
        }
    });

    setup(() => {
        // Create analyzer with mock context
        const context: vscode.ExtensionContext = {
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
        analyzer = new LucentAnalyzer(context);
    });

    test('Analyzer should be created', () => {
        assert.ok(analyzer);
    });

    test('Cache should be cleared', async () => {
        await analyzer.clearCache();
        // No error should be thrown
        assert.ok(true);
    });

    test('detectExecutable should find lucent or return default', async () => {
        const execPath = await (analyzer as any).detectExecutable();
        assert.ok(execPath);
        assert.ok(typeof execPath === 'string');
    });
});
