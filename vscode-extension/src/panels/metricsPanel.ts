import * as vscode from 'vscode';
import { LucentAnalyzer, FileMetrics, ProjectMetrics } from '../analyzer';

/**
 * Metrics Panel - Interactive dashboard with charts and detailed analysis
 */
export class MetricsPanel {
    public static currentPanel: MetricsPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];

    private constructor(
        panel: vscode.WebviewPanel,
        private readonly analyzer: LucentAnalyzer,
        private readonly extensionUri: vscode.Uri
    ) {
        this._panel = panel;

        // Set initial HTML content
        this._update();

        // Listen for when the panel is disposed
        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);

        // Handle messages from the webview
        this._panel.webview.onDidReceiveMessage(
            async (message) => {
                switch (message.command) {
                    case 'analyze':
                        await this.refreshMetrics();
                        break;
                    case 'export':
                        await this.exportMetrics(message.format);
                        break;
                }
            },
            null,
            this._disposables
        );
    }

    public static createOrShow(analyzer: LucentAnalyzer, extensionUri: vscode.Uri) {
        const column = vscode.window.activeTextEditor
            ? vscode.window.activeTextEditor.viewColumn
            : undefined;

        // If we already have a panel, show it
        if (MetricsPanel.currentPanel) {
            MetricsPanel.currentPanel._panel.reveal(column);
            MetricsPanel.currentPanel.refreshMetrics();
            return;
        }

        // Otherwise, create a new panel
        const panel = vscode.window.createWebviewPanel(
            'lucentMetrics',
            'Lucent Code Metrics',
            column || vscode.ViewColumn.One,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [
                    vscode.Uri.joinPath(extensionUri, 'media')
                ]
            }
        );

        MetricsPanel.currentPanel = new MetricsPanel(panel, analyzer, extensionUri);
    }

    public async refreshMetrics() {
        await this._update();
    }

    private async _update() {
        const webview = this._panel.webview;

        this._panel.title = 'Lucent Code Metrics';
        this._panel.webview.html = await this._getHtmlForWebview(webview);
    }

    private async _getHtmlForWebview(webview: vscode.Webview): Promise<string> {
        const editor = vscode.window.activeTextEditor;

        if (!editor) {
            return this._getEmptyStateHtml();
        }

        const metrics = await this.analyzer.analyzeFile(editor.document.uri.fsPath);

        if (!metrics) {
            return this._getErrorStateHtml();
        }

        return this._getMetricsHtml(metrics, webview);
    }

    private _getEmptyStateHtml(): string {
        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Code Metrics</title>
            <style>${this._getStyles()}</style>
        </head>
        <body>
            <div class="empty-state">
                <h1>💎 Lucent Code Metrics</h1>
                <p>No active file to analyze</p>
                <p class="hint">Open a code file to see metrics</p>
            </div>
        </body>
        </html>`;
    }

    private _getErrorStateHtml(): string {
        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Code Metrics</title>
            <style>${this._getStyles()}</style>
        </head>
        <body>
            <div class="error-state">
                <h1>⚠️ Analysis Failed</h1>
                <p>Unable to analyze the current file</p>
                <button onclick="retry()">Retry Analysis</button>
            </div>
            <script>
                function retry() {
                    const vscode = acquireVsCodeApi();
                    vscode.postMessage({ command: 'analyze' });
                }
            </script>
        </body>
        </html>`;
    }

    private _getMetricsHtml(metrics: FileMetrics, webview: vscode.Webview): string {
        const fileName = metrics.path.split('/').pop() || 'Unknown';
        const complexityColor = this._getComplexityColor(metrics.cyclomatic_complexity);
        const miColor = this._getMIColor(metrics.maintainability_index);

        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Code Metrics - ${fileName}</title>
            <style>${this._getStyles()}</style>
        </head>
        <body>
            <div class="container">
                <header>
                    <h1>💎 ${fileName}</h1>
                    <div class="actions">
                        <button onclick="analyze()">🔄 Refresh</button>
                        <button onclick="exportData('json')">📊 Export JSON</button>
                    </div>
                </header>

                <div class="overview">
                    <div class="metric-card ${complexityColor}">
                        <div class="metric-label">Cyclomatic Complexity</div>
                        <div class="metric-value">${metrics.cyclomatic_complexity.toFixed(1)}</div>
                        <div class="metric-status">${this._getComplexityLabel(metrics.cyclomatic_complexity)}</div>
                    </div>

                    <div class="metric-card ${miColor}">
                        <div class="metric-label">Maintainability Index</div>
                        <div class="metric-value">${metrics.maintainability_index.toFixed(0)}</div>
                        <div class="metric-status">${this._getMILabel(metrics.maintainability_index)}</div>
                    </div>

                    <div class="metric-card">
                        <div class="metric-label">Cognitive Complexity</div>
                        <div class="metric-value">${metrics.cognitive_complexity.toFixed(1)}</div>
                        <div class="metric-status">Mental effort required</div>
                    </div>

                    <div class="metric-card">
                        <div class="metric-label">Max Nesting Depth</div>
                        <div class="metric-value">${metrics.max_nesting_depth}</div>
                        <div class="metric-status">${metrics.max_nesting_depth > 4 ? 'Deep nesting' : 'Good'}</div>
                    </div>
                </div>

                <div class="section">
                    <h2>📏 Lines of Code</h2>
                    <div class="loc-breakdown">
                        <div class="loc-item">
                            <span class="loc-label">Total Lines</span>
                            <span class="loc-value">${metrics.total_lines}</span>
                            <div class="loc-bar">
                                <div class="loc-fill" style="width: 100%; background: var(--vscode-charts-blue)"></div>
                            </div>
                        </div>
                        <div class="loc-item">
                            <span class="loc-label">Code Lines</span>
                            <span class="loc-value">${metrics.code_lines}</span>
                            <div class="loc-bar">
                                <div class="loc-fill" style="width: ${(metrics.code_lines / metrics.total_lines * 100)}%; background: var(--vscode-charts-green)"></div>
                            </div>
                        </div>
                        <div class="loc-item">
                            <span class="loc-label">Comment Lines</span>
                            <span class="loc-value">${metrics.comment_lines}</span>
                            <div class="loc-bar">
                                <div class="loc-fill" style="width: ${(metrics.comment_lines / metrics.total_lines * 100)}%; background: var(--vscode-charts-yellow)"></div>
                            </div>
                        </div>
                        <div class="loc-item">
                            <span class="loc-label">Blank Lines</span>
                            <span class="loc-value">${metrics.blank_lines}</span>
                            <div class="loc-bar">
                                <div class="loc-fill" style="width: ${(metrics.blank_lines / metrics.total_lines * 100)}%; background: var(--vscode-charts-gray)"></div>
                            </div>
                        </div>
                    </div>
                </div>

                ${this._getCodeSmellsSection(metrics)}

                <div class="section">
                    <h2>📊 Additional Metrics</h2>
                    <div class="metrics-list">
                        <div class="metric-row">
                            <span class="metric-name">Entropy</span>
                            <span class="metric-desc">Information density</span>
                            <span class="metric-val">${metrics.entropy.toFixed(2)}</span>
                        </div>
                        <div class="metric-row">
                            <span class="metric-name">Halstead Volume</span>
                            <span class="metric-desc">Program size</span>
                            <span class="metric-val">${metrics.halstead_volume.toFixed(0)}</span>
                        </div>
                        <div class="metric-row">
                            <span class="metric-name">Halstead Difficulty</span>
                            <span class="metric-desc">Comprehension difficulty</span>
                            <span class="metric-val">${metrics.halstead_difficulty.toFixed(2)}</span>
                        </div>
                    </div>
                </div>

                ${this._getRecommendationsSection(metrics)}
            </div>

            <script>
                const vscode = acquireVsCodeApi();

                function analyze() {
                    vscode.postMessage({ command: 'analyze' });
                }

                function exportData(format) {
                    vscode.postMessage({ command: 'export', format: format });
                }
            </script>
        </body>
        </html>`;
    }

    private _getCodeSmellsSection(metrics: FileMetrics): string {
        if (metrics.code_smells.length === 0) {
            return `
                <div class="section">
                    <h2>✅ Code Smells</h2>
                    <div class="success-message">
                        No code quality issues detected. Great job!
                    </div>
                </div>
            `;
        }

        const smellsHtml = metrics.code_smells.map(smell => `
            <div class="smell-item ${smell.severity.toLowerCase()}">
                <div class="smell-header">
                    <span class="smell-type">${smell.type}</span>
                    <span class="smell-severity">${smell.severity}</span>
                </div>
                <div class="smell-message">${smell.message}</div>
                <div class="smell-location">Line ${smell.line}, Column ${smell.column}</div>
            </div>
        `).join('');

        return `
            <div class="section">
                <h2>⚠️ Code Smells (${metrics.code_smells.length})</h2>
                <div class="smells-list">
                    ${smellsHtml}
                </div>
            </div>
        `;
    }

    private _getRecommendationsSection(metrics: FileMetrics): string {
        const recommendations: string[] = [];

        if (metrics.cyclomatic_complexity > 10) {
            recommendations.push('Consider breaking down complex functions into smaller, more focused ones');
        }
        if (metrics.maintainability_index < 65) {
            recommendations.push('Refactor to improve code maintainability through simplification and better structure');
        }
        if (metrics.max_nesting_depth > 4) {
            recommendations.push('Reduce nesting depth by extracting nested logic into separate functions');
        }
        if (metrics.cognitive_complexity > 15) {
            recommendations.push('Simplify control flow to reduce mental effort required to understand the code');
        }
        if (metrics.comment_lines / metrics.code_lines < 0.1) {
            recommendations.push('Add more documentation to improve code understandability');
        }

        if (recommendations.length === 0) {
            return `
                <div class="section">
                    <h2>💡 Recommendations</h2>
                    <div class="success-message">
                        Code quality is excellent! No specific recommendations.
                    </div>
                </div>
            `;
        }

        const recsHtml = recommendations.map(rec => `
            <li>${rec}</li>
        `).join('');

        return `
            <div class="section">
                <h2>💡 Recommendations</h2>
                <ul class="recommendations-list">
                    ${recsHtml}
                </ul>
            </div>
        `;
    }

    private _getStyles(): string {
        return `
            :root {
                --vscode-charts-blue: #007acc;
                --vscode-charts-green: #89d185;
                --vscode-charts-yellow: #dcdcaa;
                --vscode-charts-red: #f48771;
                --vscode-charts-gray: #808080;
            }

            * {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
            }

            body {
                font-family: var(--vscode-font-family);
                font-size: var(--vscode-font-size);
                color: var(--vscode-foreground);
                background-color: var(--vscode-editor-background);
                padding: 20px;
            }

            .container {
                max-width: 1200px;
                margin: 0 auto;
            }

            header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 30px;
                padding-bottom: 20px;
                border-bottom: 1px solid var(--vscode-panel-border);
            }

            h1 {
                font-size: 24px;
                font-weight: 600;
            }

            h2 {
                font-size: 18px;
                font-weight: 600;
                margin-bottom: 15px;
            }

            .actions {
                display: flex;
                gap: 10px;
            }

            button {
                background: var(--vscode-button-background);
                color: var(--vscode-button-foreground);
                border: none;
                padding: 8px 16px;
                border-radius: 4px;
                cursor: pointer;
                font-size: 13px;
            }

            button:hover {
                background: var(--vscode-button-hoverBackground);
            }

            .overview {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                gap: 20px;
                margin-bottom: 30px;
            }

            .metric-card {
                background: var(--vscode-editor-inactiveSelectionBackground);
                border-radius: 8px;
                padding: 20px;
                border-left: 4px solid var(--vscode-charts-blue);
            }

            .metric-card.excellent {
                border-left-color: var(--vscode-charts-green);
            }

            .metric-card.good {
                border-left-color: var(--vscode-charts-yellow);
            }

            .metric-card.warning {
                border-left-color: var(--vscode-charts-red);
            }

            .metric-label {
                font-size: 12px;
                opacity: 0.8;
                margin-bottom: 8px;
            }

            .metric-value {
                font-size: 36px;
                font-weight: 700;
                margin-bottom: 4px;
            }

            .metric-status {
                font-size: 13px;
                opacity: 0.7;
            }

            .section {
                margin-bottom: 30px;
                padding: 20px;
                background: var(--vscode-editor-inactiveSelectionBackground);
                border-radius: 8px;
            }

            .loc-breakdown {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .loc-item {
                display: grid;
                grid-template-columns: 150px 60px 1fr;
                align-items: center;
                gap: 12px;
            }

            .loc-label {
                font-size: 13px;
            }

            .loc-value {
                font-weight: 600;
                text-align: right;
            }

            .loc-bar {
                height: 8px;
                background: var(--vscode-editor-background);
                border-radius: 4px;
                overflow: hidden;
            }

            .loc-fill {
                height: 100%;
                transition: width 0.3s ease;
            }

            .smells-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .smell-item {
                padding: 12px;
                border-left: 4px solid var(--vscode-charts-yellow);
                background: var(--vscode-editor-background);
                border-radius: 4px;
            }

            .smell-item.error {
                border-left-color: var(--vscode-charts-red);
            }

            .smell-item.warning {
                border-left-color: var(--vscode-charts-yellow);
            }

            .smell-item.info {
                border-left-color: var(--vscode-charts-blue);
            }

            .smell-header {
                display: flex;
                justify-content: space-between;
                margin-bottom: 6px;
            }

            .smell-type {
                font-weight: 600;
                font-size: 13px;
            }

            .smell-severity {
                font-size: 11px;
                text-transform: uppercase;
                opacity: 0.7;
            }

            .smell-message {
                font-size: 13px;
                margin-bottom: 4px;
            }

            .smell-location {
                font-size: 11px;
                opacity: 0.6;
            }

            .metrics-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .metric-row {
                display: grid;
                grid-template-columns: 200px 1fr 100px;
                gap: 12px;
                padding: 8px 0;
                border-bottom: 1px solid var(--vscode-panel-border);
            }

            .metric-row:last-child {
                border-bottom: none;
            }

            .metric-name {
                font-weight: 600;
            }

            .metric-desc {
                opacity: 0.7;
                font-size: 13px;
            }

            .metric-val {
                text-align: right;
                font-weight: 600;
            }

            .recommendations-list {
                list-style-position: inside;
                display: flex;
                flex-direction: column;
                gap: 8px;
            }

            .recommendations-list li {
                padding: 8px;
                background: var(--vscode-editor-background);
                border-radius: 4px;
            }

            .success-message {
                padding: 20px;
                text-align: center;
                color: var(--vscode-charts-green);
                background: var(--vscode-editor-background);
                border-radius: 4px;
            }

            .empty-state, .error-state {
                text-align: center;
                padding: 60px 20px;
            }

            .empty-state h1, .error-state h1 {
                margin-bottom: 12px;
            }

            .empty-state p, .error-state p {
                opacity: 0.7;
                margin-bottom: 8px;
            }

            .hint {
                font-size: 13px;
            }
        `;
    }

    private _getComplexityColor(complexity: number): string {
        if (complexity <= 5) return 'excellent';
        if (complexity <= 10) return 'good';
        return 'warning';
    }

    private _getMIColor(mi: number): string {
        if (mi >= 85) return 'excellent';
        if (mi >= 65) return 'good';
        return 'warning';
    }

    private _getComplexityLabel(complexity: number): string {
        if (complexity <= 5) return 'Low complexity';
        if (complexity <= 10) return 'Moderate complexity';
        return 'High complexity';
    }

    private _getMILabel(mi: number): string {
        if (mi >= 85) return 'Excellent';
        if (mi >= 65) return 'Good';
        if (mi >= 50) return 'Fair';
        return 'Needs improvement';
    }

    private async exportMetrics(format: string) {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const metrics = await this.analyzer.analyzeFile(editor.document.uri.fsPath);
        if (!metrics) return;

        if (format === 'json') {
            const json = JSON.stringify(metrics, null, 2);
            const uri = await vscode.window.showSaveDialog({
                defaultUri: vscode.Uri.file('metrics.json'),
                filters: { 'JSON': ['json'] }
            });

            if (uri) {
                await vscode.workspace.fs.writeFile(uri, Buffer.from(json, 'utf8'));
                vscode.window.showInformationMessage('Metrics exported successfully!');
            }
        }
    }

    public dispose() {
        MetricsPanel.currentPanel = undefined;

        // Clean up our resources
        this._panel.dispose();

        while (this._disposables.length) {
            const disposable = this._disposables.pop();
            if (disposable) {
                disposable.dispose();
            }
        }
    }
}
