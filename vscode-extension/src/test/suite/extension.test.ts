import * as assert from 'assert';
import * as vscode from 'vscode';

suite('Extension Test Suite', () => {
    vscode.window.showInformationMessage('Start all tests.');

    test('Extension should be present', () => {
        const extension = vscode.extensions.getExtension('lucent.lucent-code-metrics');
        assert.ok(extension);
    });

    test('Extension should activate', async () => {
        const extension = vscode.extensions.getExtension('lucent.lucent-code-metrics');
        if (extension) {
            await extension.activate();
            assert.ok(extension.isActive);
        }
    });

    test('Commands should be registered', async () => {
        const commands = await vscode.commands.getCommands(true);

        const lucentCommands = [
            'lucent.analyzeFile',
            'lucent.analyzeWorkspace',
            'lucent.showMetricsPanel',
            'lucent.showTrends',
            'lucent.clearCache',
            'lucent.toggleInlineDiagnostics'
        ];

        lucentCommands.forEach(cmd => {
            assert.ok(
                commands.includes(cmd),
                `Command ${cmd} should be registered`
            );
        });
    });

    test('Configuration should be available', () => {
        const config = vscode.workspace.getConfiguration('lucent');
        assert.ok(config);

        // Check default values
        assert.strictEqual(config.get('enabled'), true);
        assert.strictEqual(config.get('showInlineMetrics'), true);
        assert.strictEqual(config.get('analyzeOnSave'), true);
    });

    test('Configuration should be updatable', async () => {
        const config = vscode.workspace.getConfiguration('lucent');
        const originalValue = config.get('showInlineMetrics');

        await config.update('showInlineMetrics', false, vscode.ConfigurationTarget.Global);
        assert.strictEqual(config.get('showInlineMetrics'), false);

        // Restore original value
        await config.update('showInlineMetrics', originalValue, vscode.ConfigurationTarget.Global);
    });
});
