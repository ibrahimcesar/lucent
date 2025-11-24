import * as vscode from 'vscode';
import { LucentAnalyzer } from '../analyzer';

/**
 * Code Lens Provider - Shows inline metrics above functions/classes
 *
 * Features:
 * - Displays complexity, maintainability, and LOC metrics
 * - Appears at the top of each file
 * - Click to show detailed metrics panel
 * - Automatically updates on file save
 */
export class CodeLensProvider implements vscode.CodeLensProvider {
    private _onDidChangeCodeLenses: vscode.EventEmitter<void> = new vscode.EventEmitter<void>();
    public readonly onDidChangeCodeLenses: vscode.Event<void> = this._onDidChangeCodeLenses.event;

    constructor(private analyzer: LucentAnalyzer) {}

    provideCodeLenses(
        document: vscode.TextDocument,
        token: vscode.CancellationToken
    ): vscode.CodeLens[] | Thenable<vscode.CodeLens[]> {
        const config = vscode.workspace.getConfiguration('lucent');

        if (!config.get('showInlineMetrics')) {
            return [];
        }

        return this.getCodeLenses(document);
    }

    private async getCodeLenses(document: vscode.TextDocument): Promise<vscode.CodeLens[]> {
        const lenses: vscode.CodeLens[] = [];

        // Get metrics for current file
        const metrics = await this.analyzer.analyzeFile(document.uri.fsPath);

        if (!metrics) {
            return lenses;
        }

        // Add code lens at the top of the file
        const topOfDocument = new vscode.Range(0, 0, 0, 0);

        // Complexity lens
        const complexityLens = new vscode.CodeLens(topOfDocument, {
            title: `$(symbol-method) Complexity: ${metrics.cyclomatic_complexity.toFixed(1)}`,
            tooltip: 'Cyclomatic complexity measures code branching',
            command: 'lucent.showMetricsPanel'
        });
        lenses.push(complexityLens);

        // Maintainability lens
        const miColor = this.getMaintainabilityColor(metrics.maintainability_index);
        const maintainabilityLens = new vscode.CodeLens(topOfDocument, {
            title: `$(beaker) Maintainability: ${metrics.maintainability_index.toFixed(0)} ${miColor}`,
            tooltip: 'Maintainability Index (0-100, higher is better)',
            command: 'lucent.showMetricsPanel'
        });
        lenses.push(maintainabilityLens);

        // Lines of code lens
        const locLens = new vscode.CodeLens(topOfDocument, {
            title: `$(file-code) ${metrics.code_lines} LOC`,
            tooltip: `Total: ${metrics.total_lines} lines (${metrics.comment_lines} comments, ${metrics.blank_lines} blank)`,
            command: 'lucent.showMetricsPanel'
        });
        lenses.push(locLens);

        // Code smells lens (if any)
        if (metrics.code_smells.length > 0) {
            const smellsLens = new vscode.CodeLens(topOfDocument, {
                title: `$(warning) ${metrics.code_smells.length} Code Smell${metrics.code_smells.length > 1 ? 's' : ''}`,
                tooltip: 'Click to view code quality issues',
                command: 'lucent.showMetricsPanel'
            });
            lenses.push(smellsLens);
        }

        return lenses;
    }

    private getMaintainabilityColor(mi: number): string {
        if (mi >= 85) return '🟢';
        if (mi >= 65) return '🟡';
        return '🔴';
    }

    refresh(): void {
        this._onDidChangeCodeLenses.fire();
    }
}
