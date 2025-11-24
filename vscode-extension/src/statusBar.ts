import * as vscode from 'vscode';
import { LucentAnalyzer } from './analyzer';

/**
 * Status Bar Manager - Shows current file metrics in status bar
 */
export class StatusBarManager {
    private statusBarItem: vscode.StatusBarItem;

    constructor(private analyzer: LucentAnalyzer) {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            100
        );
        this.statusBarItem.command = 'lucent.analyzeFile';
        this.statusBarItem.show();
        this.update();
    }

    async update(): Promise<void> {
        const config = vscode.workspace.getConfiguration('lucent');

        if (!config.get('showStatusBar')) {
            this.statusBarItem.hide();
            return;
        }

        const editor = vscode.window.activeTextEditor;

        if (!editor) {
            this.statusBarItem.text = '💎 Lucent';
            this.statusBarItem.tooltip = 'No active file';
            return;
        }

        const metrics = await this.analyzer.analyzeFile(editor.document.uri.fsPath);

        if (!metrics) {
            this.statusBarItem.text = '💎 Lucent: Analysis pending...';
            this.statusBarItem.tooltip = 'Click to analyze';
            return;
        }

        // Format: 💎 Lucent: Complexity X | MI Y 🟢
        const complexityEmoji = this.getComplexityEmoji(metrics.cyclomatic_complexity);
        const miEmoji = this.getMaintainabilityEmoji(metrics.maintainability_index);

        this.statusBarItem.text =
            `💎 Lucent: ` +
            `Complexity ${metrics.cyclomatic_complexity.toFixed(1)} ${complexityEmoji} | ` +
            `MI ${metrics.maintainability_index.toFixed(0)} ${miEmoji}`;

        this.statusBarItem.tooltip = this.createTooltip(metrics);
        this.statusBarItem.show();
    }

    private createTooltip(metrics: any): string {
        return `Lucent Code Metrics\n\n` +
               `Complexity: ${metrics.cyclomatic_complexity.toFixed(1)}\n` +
               `Maintainability: ${metrics.maintainability_index.toFixed(0)}\n` +
               `Lines: ${metrics.total_lines} (${metrics.code_lines} code)\n` +
               `Code Smells: ${metrics.code_smells.length}\n\n` +
               `Click to analyze file`;
    }

    private getComplexityEmoji(complexity: number): string {
        if (complexity <= 5) return '🟢';
        if (complexity <= 10) return '🟡';
        return '🔴';
    }

    private getMaintainabilityEmoji(mi: number): string {
        if (mi >= 85) return '🟢';
        if (mi >= 65) return '🟡';
        return '🔴';
    }

    dispose(): void {
        this.statusBarItem.dispose();
    }
}
