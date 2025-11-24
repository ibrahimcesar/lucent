import * as vscode from 'vscode';
import { LucentAnalyzer, FileMetrics, ProjectMetrics } from '../analyzer';

/**
 * Metrics Tree View Provider - Sidebar panel showing file/project metrics
 */
export class MetricsProvider implements vscode.TreeDataProvider<MetricItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<MetricItem | undefined | null> =
        new vscode.EventEmitter<MetricItem | undefined | null>();
    readonly onDidChangeTreeData: vscode.Event<MetricItem | undefined | null> =
        this._onDidChangeTreeData.event;

    constructor(private analyzer: LucentAnalyzer) {}

    refresh(): void {
        this._onDidChangeTreeData.fire(undefined);
    }

    getTreeItem(element: MetricItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: MetricItem): Promise<MetricItem[]> {
        if (!element) {
            // Root level
            return this.getRootItems();
        }

        // Child items
        return element.children || [];
    }

    private async getRootItems(): Promise<MetricItem[]> {
        const editor = vscode.window.activeTextEditor;

        if (!editor) {
            return [
                new MetricItem(
                    'No active file',
                    '',
                    vscode.TreeItemCollapsibleState.None,
                    'info'
                )
            ];
        }

        const metrics = await this.analyzer.analyzeFile(editor.document.uri.fsPath);

        if (!metrics) {
            return [
                new MetricItem(
                    'Analysis unavailable',
                    '',
                    vscode.TreeItemCollapsibleState.None,
                    'error'
                )
            ];
        }

        return this.createMetricItems(metrics);
    }

    private createMetricItems(metrics: FileMetrics): MetricItem[] {
        const items: MetricItem[] = [];

        // File name
        const fileName = new MetricItem(
            `📄 ${metrics.path.split('/').pop()}`,
            '',
            vscode.TreeItemCollapsibleState.Expanded,
            'file'
        );

        // Complexity
        const complexityEmoji = metrics.cyclomatic_complexity > 10 ? '🔴' : metrics.cyclomatic_complexity > 5 ? '🟡' : '🟢';
        const complexity = new MetricItem(
            `${complexityEmoji} Complexity: ${metrics.cyclomatic_complexity.toFixed(1)}`,
            metrics.cyclomatic_complexity > 10 ? 'High complexity' : 'Acceptable',
            vscode.TreeItemCollapsibleState.None,
            'metric'
        );

        // Maintainability
        const miEmoji = metrics.maintainability_index >= 85 ? '🟢' : metrics.maintainability_index >= 65 ? '🟡' : '🔴';
        const maintainability = new MetricItem(
            `${miEmoji} Maintainability: ${metrics.maintainability_index.toFixed(0)}`,
            this.getMaintainabilityLabel(metrics.maintainability_index),
            vscode.TreeItemCollapsibleState.None,
            'metric'
        );

        // Lines of code (expandable)
        const loc = new MetricItem(
            `📏 Lines of Code`,
            `${metrics.total_lines} total`,
            vscode.TreeItemCollapsibleState.Collapsed,
            'category'
        );
        loc.children = [
            new MetricItem(`Code: ${metrics.code_lines}`, '', vscode.TreeItemCollapsibleState.None, 'detail'),
            new MetricItem(`Comments: ${metrics.comment_lines}`, '', vscode.TreeItemCollapsibleState.None, 'detail'),
            new MetricItem(`Blank: ${metrics.blank_lines}`, '', vscode.TreeItemCollapsibleState.None, 'detail')
        ];

        // Code smells (expandable if any)
        if (metrics.code_smells.length > 0) {
            const smells = new MetricItem(
                `⚠️ Code Smells (${metrics.code_smells.length})`,
                'Quality issues detected',
                vscode.TreeItemCollapsibleState.Expanded,
                'warning'
            );
            smells.children = metrics.code_smells.map(smell =>
                new MetricItem(
                    smell.message,
                    smell.type,
                    vscode.TreeItemCollapsibleState.None,
                    'smell'
                )
            );
            items.push(fileName, complexity, maintainability, loc, smells);
        } else {
            const noSmells = new MetricItem(
                '✅ No Code Smells',
                'Code quality is good',
                vscode.TreeItemCollapsibleState.None,
                'success'
            );
            items.push(fileName, complexity, maintainability, loc, noSmells);
        }

        // Additional metrics
        const additional = new MetricItem(
            '📊 Additional Metrics',
            '',
            vscode.TreeItemCollapsibleState.Collapsed,
            'category'
        );
        additional.children = [
            new MetricItem(
                `Nesting Depth: ${metrics.max_nesting_depth}`,
                metrics.max_nesting_depth > 4 ? 'Deep nesting' : 'Good',
                vscode.TreeItemCollapsibleState.None,
                'detail'
            ),
            new MetricItem(
                `Entropy: ${metrics.entropy.toFixed(2)}`,
                'Information density',
                vscode.TreeItemCollapsibleState.None,
                'detail'
            ),
            new MetricItem(
                `Cognitive Complexity: ${metrics.cognitive_complexity.toFixed(1)}`,
                'Mental effort required',
                vscode.TreeItemCollapsibleState.None,
                'detail'
            )
        ];
        items.push(additional);

        return items;
    }

    private getMaintainabilityLabel(mi: number): string {
        if (mi >= 85) return 'Excellent';
        if (mi >= 65) return 'Good';
        if (mi >= 50) return 'Fair';
        return 'Needs improvement';
    }
}

class MetricItem extends vscode.TreeItem {
    children?: MetricItem[];

    constructor(
        label: string,
        description: string,
        collapsibleState: vscode.TreeItemCollapsibleState,
        type: 'file' | 'metric' | 'category' | 'detail' | 'warning' | 'success' | 'error' | 'smell' | 'info',
        children?: MetricItem[]
    ) {
        super(label, collapsibleState);

        // TreeItem has these properties built-in
        if (description) {
            (this as any).description = description;
        }
        (this as any).contextValue = type;
        this.children = children;

        // Set icons based on type
        switch (type) {
            case 'file':
                (this as any).iconPath = new vscode.ThemeIcon('file-code');
                break;
            case 'metric':
                (this as any).iconPath = new vscode.ThemeIcon('symbol-number');
                break;
            case 'category':
                (this as any).iconPath = new vscode.ThemeIcon('folder');
                break;
            case 'warning':
                (this as any).iconPath = new vscode.ThemeIcon('warning');
                break;
            case 'success':
                (this as any).iconPath = new vscode.ThemeIcon('check');
                break;
            case 'error':
                (this as any).iconPath = new vscode.ThemeIcon('error');
                break;
        }
    }
}
