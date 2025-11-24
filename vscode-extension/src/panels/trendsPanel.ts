import * as vscode from 'vscode';
import { LucentAnalyzer } from '../analyzer';

interface TrendData {
    commits: Array<{
        hash: string;
        message: string;
        author: string;
        timestamp: number;
        complexity: number;
        maintainability: number;
        lines: number;
    }>;
    overall_trend: string;
}

/**
 * Trends Panel - Git history visualization with quality trends
 */
export class TrendsPanel {
    public static currentPanel: TrendsPanel | undefined;
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
                    case 'refresh':
                        await this.refreshTrends();
                        break;
                    case 'changeLimit':
                        await this.refreshTrends(message.limit);
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
        if (TrendsPanel.currentPanel) {
            TrendsPanel.currentPanel._panel.reveal(column);
            TrendsPanel.currentPanel.refreshTrends();
            return;
        }

        // Otherwise, create a new panel
        const panel = vscode.window.createWebviewPanel(
            'lucentTrends',
            'Lucent Quality Trends',
            column || vscode.ViewColumn.Two,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [
                    vscode.Uri.joinPath(extensionUri, 'media')
                ]
            }
        );

        TrendsPanel.currentPanel = new TrendsPanel(panel, analyzer, extensionUri);
    }

    public async refreshTrends(limit: number = 10) {
        await this._update(limit);
    }

    private async _update(limit: number = 10) {
        const webview = this._panel.webview;

        this._panel.title = 'Lucent Quality Trends';
        this._panel.webview.html = await this._getHtmlForWebview(webview, limit);
    }

    private async _getHtmlForWebview(webview: vscode.Webview, limit: number): Promise<string> {
        try {
            const trends = await this.analyzer.getGitTrends(limit);

            if (!trends || !trends.commits || trends.commits.length === 0) {
                return this._getNoDataHtml();
            }

            return this._getTrendsHtml(trends, webview);
        } catch (error) {
            return this._getErrorHtml(error);
        }
    }

    private _getNoDataHtml(): string {
        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Quality Trends</title>
            <style>${this._getStyles()}</style>
        </head>
        <body>
            <div class="empty-state">
                <h1>📈 Lucent Quality Trends</h1>
                <p>No Git history available</p>
                <p class="hint">Make sure you're in a Git repository with commit history</p>
            </div>
        </body>
        </html>`;
    }

    private _getErrorHtml(error: any): string {
        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Quality Trends</title>
            <style>${this._getStyles()}</style>
        </head>
        <body>
            <div class="error-state">
                <h1>⚠️ Analysis Failed</h1>
                <p>Unable to fetch Git trends</p>
                <p class="error-message">${error.message || 'Unknown error'}</p>
                <button onclick="retry()">Retry</button>
            </div>
            <script>
                const vscode = acquireVsCodeApi();
                function retry() {
                    vscode.postMessage({ command: 'refresh' });
                }
            </script>
        </body>
        </html>`;
    }

    private _getTrendsHtml(trends: TrendData, webview: vscode.Webview): string {
        const complexityData = trends.commits.map(c => c.complexity).reverse();
        const maintainabilityData = trends.commits.map(c => c.maintainability).reverse();
        const labels = trends.commits.map(c => c.hash.substring(0, 7)).reverse();

        const trendEmoji = trends.overall_trend === 'Improving' ? '📈' :
                          trends.overall_trend === 'Degrading' ? '📉' : '➡️';
        const trendColor = trends.overall_trend === 'Improving' ? 'var(--vscode-charts-green)' :
                          trends.overall_trend === 'Degrading' ? 'var(--vscode-charts-red)' :
                          'var(--vscode-charts-yellow)';

        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lucent Quality Trends</title>
            <style>${this._getStyles()}</style>
            <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
        </head>
        <body>
            <div class="container">
                <header>
                    <h1>📈 Quality Trends</h1>
                    <div class="actions">
                        <select id="limitSelect" onchange="changeLimit()">
                            <option value="5">Last 5 commits</option>
                            <option value="10" selected>Last 10 commits</option>
                            <option value="20">Last 20 commits</option>
                            <option value="50">Last 50 commits</option>
                        </select>
                        <button onclick="refresh()">🔄 Refresh</button>
                    </div>
                </header>

                <div class="trend-overview">
                    <div class="trend-card" style="border-left-color: ${trendColor}">
                        <div class="trend-emoji">${trendEmoji}</div>
                        <div class="trend-label">Overall Trend</div>
                        <div class="trend-value">${trends.overall_trend}</div>
                    </div>
                    <div class="trend-card">
                        <div class="trend-emoji">📊</div>
                        <div class="trend-label">Total Commits</div>
                        <div class="trend-value">${trends.commits.length}</div>
                    </div>
                    <div class="trend-card">
                        <div class="trend-emoji">📝</div>
                        <div class="trend-label">Latest Complexity</div>
                        <div class="trend-value">${trends.commits[trends.commits.length - 1].complexity.toFixed(1)}</div>
                    </div>
                    <div class="trend-card">
                        <div class="trend-emoji">🎯</div>
                        <div class="trend-label">Latest MI</div>
                        <div class="trend-value">${trends.commits[trends.commits.length - 1].maintainability.toFixed(0)}</div>
                    </div>
                </div>

                <div class="charts">
                    <div class="chart-container">
                        <h2>Complexity Over Time</h2>
                        <canvas id="complexityChart"></canvas>
                    </div>

                    <div class="chart-container">
                        <h2>Maintainability Index Over Time</h2>
                        <canvas id="maintainabilityChart"></canvas>
                    </div>

                    <div class="chart-container">
                        <h2>Combined Quality Metrics</h2>
                        <canvas id="combinedChart"></canvas>
                    </div>
                </div>

                <div class="section">
                    <h2>📜 Recent Commits</h2>
                    <div class="commits-list">
                        ${this._getCommitsHtml(trends.commits)}
                    </div>
                </div>
            </div>

            <script>
                const vscode = acquireVsCodeApi();

                function refresh() {
                    vscode.postMessage({ command: 'refresh' });
                }

                function changeLimit() {
                    const limit = parseInt(document.getElementById('limitSelect').value);
                    vscode.postMessage({ command: 'changeLimit', limit: limit });
                }

                // Chart.js configuration
                Chart.defaults.color = getComputedStyle(document.body).getPropertyValue('color');
                Chart.defaults.borderColor = 'rgba(128, 128, 128, 0.2)';

                // Complexity chart
                new Chart(document.getElementById('complexityChart'), {
                    type: 'line',
                    data: {
                        labels: ${JSON.stringify(labels)},
                        datasets: [{
                            label: 'Cyclomatic Complexity',
                            data: ${JSON.stringify(complexityData)},
                            borderColor: 'rgb(0, 122, 204)',
                            backgroundColor: 'rgba(0, 122, 204, 0.1)',
                            tension: 0.3,
                            fill: true
                        }]
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {
                            legend: { display: false },
                            tooltip: { mode: 'index', intersect: false }
                        },
                        scales: {
                            y: {
                                beginAtZero: false,
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            },
                            x: {
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            }
                        }
                    }
                });

                // Maintainability chart
                new Chart(document.getElementById('maintainabilityChart'), {
                    type: 'line',
                    data: {
                        labels: ${JSON.stringify(labels)},
                        datasets: [{
                            label: 'Maintainability Index',
                            data: ${JSON.stringify(maintainabilityData)},
                            borderColor: 'rgb(137, 209, 133)',
                            backgroundColor: 'rgba(137, 209, 133, 0.1)',
                            tension: 0.3,
                            fill: true
                        }]
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {
                            legend: { display: false },
                            tooltip: { mode: 'index', intersect: false }
                        },
                        scales: {
                            y: {
                                beginAtZero: false,
                                min: 0,
                                max: 100,
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            },
                            x: {
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            }
                        }
                    }
                });

                // Combined chart
                new Chart(document.getElementById('combinedChart'), {
                    type: 'bar',
                    data: {
                        labels: ${JSON.stringify(labels)},
                        datasets: [
                            {
                                label: 'Complexity',
                                data: ${JSON.stringify(complexityData)},
                                backgroundColor: 'rgba(0, 122, 204, 0.6)',
                                yAxisID: 'y'
                            },
                            {
                                label: 'Maintainability',
                                data: ${JSON.stringify(maintainabilityData)},
                                backgroundColor: 'rgba(137, 209, 133, 0.6)',
                                yAxisID: 'y1'
                            }
                        ]
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        interaction: {
                            mode: 'index',
                            intersect: false
                        },
                        scales: {
                            y: {
                                type: 'linear',
                                display: true,
                                position: 'left',
                                title: { display: true, text: 'Complexity' },
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            },
                            y1: {
                                type: 'linear',
                                display: true,
                                position: 'right',
                                title: { display: true, text: 'Maintainability Index' },
                                min: 0,
                                max: 100,
                                grid: { drawOnChartArea: false }
                            },
                            x: {
                                grid: { color: 'rgba(128, 128, 128, 0.1)' }
                            }
                        }
                    }
                });
            </script>
        </body>
        </html>`;
    }

    private _getCommitsHtml(commits: TrendData['commits']): string {
        return commits.slice().reverse().map((commit, index) => {
            const prevCommit = index < commits.length - 1 ? commits[commits.length - index - 2] : null;

            let complexityDelta = '';
            let miDelta = '';

            if (prevCommit) {
                const complexityChange = commit.complexity - prevCommit.complexity;
                const miChange = commit.maintainability - prevCommit.maintainability;

                if (complexityChange !== 0) {
                    const arrow = complexityChange > 0 ? '↑' : '↓';
                    const color = complexityChange > 0 ? 'var(--vscode-charts-red)' : 'var(--vscode-charts-green)';
                    complexityDelta = `<span style="color: ${color}">${arrow} ${Math.abs(complexityChange).toFixed(1)}</span>`;
                }

                if (miChange !== 0) {
                    const arrow = miChange > 0 ? '↑' : '↓';
                    const color = miChange > 0 ? 'var(--vscode-charts-green)' : 'var(--vscode-charts-red)';
                    miDelta = `<span style="color: ${color}">${arrow} ${Math.abs(miChange).toFixed(0)}</span>`;
                }
            }

            const date = new Date(commit.timestamp * 1000).toLocaleDateString();

            return `
                <div class="commit-item">
                    <div class="commit-header">
                        <span class="commit-hash">${commit.hash.substring(0, 7)}</span>
                        <span class="commit-author">${commit.author}</span>
                        <span class="commit-date">${date}</span>
                    </div>
                    <div class="commit-message">${commit.message}</div>
                    <div class="commit-metrics">
                        <span class="metric">Complexity: ${commit.complexity.toFixed(1)} ${complexityDelta}</span>
                        <span class="metric">MI: ${commit.maintainability.toFixed(0)} ${miDelta}</span>
                        <span class="metric">Lines: ${commit.lines}</span>
                    </div>
                </div>
            `;
        }).join('');
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
                max-width: 1400px;
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
                align-items: center;
            }

            select {
                background: var(--vscode-dropdown-background);
                color: var(--vscode-dropdown-foreground);
                border: 1px solid var(--vscode-dropdown-border);
                padding: 6px 12px;
                border-radius: 4px;
                cursor: pointer;
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

            .trend-overview {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                gap: 20px;
                margin-bottom: 30px;
            }

            .trend-card {
                background: var(--vscode-editor-inactiveSelectionBackground);
                border-radius: 8px;
                padding: 20px;
                border-left: 4px solid var(--vscode-charts-blue);
                text-align: center;
            }

            .trend-emoji {
                font-size: 32px;
                margin-bottom: 8px;
            }

            .trend-label {
                font-size: 12px;
                opacity: 0.8;
                margin-bottom: 6px;
            }

            .trend-value {
                font-size: 24px;
                font-weight: 700;
            }

            .charts {
                display: grid;
                grid-template-columns: 1fr;
                gap: 30px;
                margin-bottom: 30px;
            }

            .chart-container {
                background: var(--vscode-editor-inactiveSelectionBackground);
                border-radius: 8px;
                padding: 20px;
                height: 300px;
            }

            .chart-container canvas {
                max-height: 250px;
            }

            .section {
                background: var(--vscode-editor-inactiveSelectionBackground);
                border-radius: 8px;
                padding: 20px;
                margin-bottom: 30px;
            }

            .commits-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .commit-item {
                background: var(--vscode-editor-background);
                border-radius: 6px;
                padding: 12px;
                border-left: 3px solid var(--vscode-charts-blue);
            }

            .commit-header {
                display: flex;
                gap: 12px;
                align-items: center;
                margin-bottom: 6px;
                font-size: 12px;
            }

            .commit-hash {
                font-family: monospace;
                background: var(--vscode-editor-inactiveSelectionBackground);
                padding: 2px 6px;
                border-radius: 3px;
                font-weight: 600;
            }

            .commit-author {
                opacity: 0.8;
            }

            .commit-date {
                margin-left: auto;
                opacity: 0.6;
            }

            .commit-message {
                font-size: 13px;
                margin-bottom: 8px;
            }

            .commit-metrics {
                display: flex;
                gap: 16px;
                font-size: 12px;
                opacity: 0.8;
            }

            .metric {
                display: flex;
                align-items: center;
                gap: 4px;
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

            .error-message {
                color: var(--vscode-charts-red);
                font-family: monospace;
                font-size: 12px;
            }
        `;
    }

    public dispose() {
        TrendsPanel.currentPanel = undefined;

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
