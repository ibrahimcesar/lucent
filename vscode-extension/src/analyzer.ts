import * as vscode from 'vscode';
import { execFile } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execFileAsync = promisify(execFile);

export interface FileMetrics {
    path: string;
    total_lines: number;
    code_lines: number;
    comment_lines: number;
    blank_lines: number;
    cyclomatic_complexity: number;
    cognitive_complexity: number;
    max_nesting_depth: number;
    maintainability_index: number;
    entropy: number;
    halstead_volume: number;
    halstead_difficulty: number;
    halstead_effort: number;
    code_smells: CodeSmell[];
}

export interface CodeSmell {
    type: string;
    severity: 'High' | 'Medium' | 'Low';
    message: string;
    line?: number;
}

export interface ProjectMetrics {
    total_files: number;
    total_lines: number;
    code_lines: number;
    comment_lines: number;
    avg_cyclomatic: number;
    avg_maintainability: number;
    by_language: Record<string, LanguageMetrics>;
}

export interface LanguageMetrics {
    file_count: number;
    total_lines: number;
    code_lines: number;
    avg_cyclomatic: number;
    avg_maintainability_index: number;
}

export class LucentAnalyzer {
    private context: vscode.ExtensionContext;
    private cache: Map<string, FileMetrics>;
    private executablePath: string;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.cache = new Map();
        this.executablePath = this.getExecutablePath();
    }

    private getExecutablePath(): string {
        const config = vscode.workspace.getConfiguration('lucent');
        return config.get('executablePath', 'lucent');
    }

    /**
     * Analyze a single file and return metrics
     */
    async analyzeFile(filePath: string): Promise<FileMetrics | null> {
        const config = vscode.workspace.getConfiguration('lucent');

        // Check cache if enabled
        if (config.get('enableCache') && this.cache.has(filePath)) {
            const stat = await vscode.workspace.fs.stat(vscode.Uri.file(filePath));
            const cached = this.cache.get(filePath)!;
            // Simple cache validation - in production you'd want mtime checking
            return cached;
        }

        try {
            const { stdout } = await execFileAsync(
                this.executablePath,
                [filePath, '--format', 'json'],
                { maxBuffer: 10 * 1024 * 1024 }
            );

            const result = JSON.parse(stdout);
            const metrics = this.extractFileMetrics(result, filePath);

            if (config.get('enableCache')) {
                this.cache.set(filePath, metrics);
            }

            return metrics;
        } catch (error) {
            console.error('Error analyzing file:', error);
            return null;
        }
    }

    /**
     * Analyze entire workspace
     */
    async analyzeWorkspace(workspacePath: string): Promise<ProjectMetrics | null> {
        try {
            const { stdout } = await execFileAsync(
                this.executablePath,
                [workspacePath, '--format', 'json'],
                { maxBuffer: 50 * 1024 * 1024 }
            );

            return JSON.parse(stdout);
        } catch (error) {
            console.error('Error analyzing workspace:', error);
            return null;
        }
    }

    /**
     * Get Git trends for the project
     */
    async getGitTrends(limit: number = 10): Promise<any> {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            return null;
        }

        try {
            const { stdout } = await execFileAsync(
                this.executablePath,
                ['git', 'trends', '--limit', limit.toString(), '--format', 'json'],
                {
                    cwd: workspaceFolder.uri.fsPath,
                    maxBuffer: 10 * 1024 * 1024
                }
            );

            return JSON.parse(stdout);
        } catch (error) {
            console.error('Error getting Git trends:', error);
            return null;
        }
    }

    /**
     * Get Git commit history with metrics
     */
    async getGitHistory(limit: number = 10): Promise<any> {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            return null;
        }

        try {
            const { stdout } = await execFileAsync(
                this.executablePath,
                ['git', 'history', '--limit', limit.toString(), '--format', 'json'],
                {
                    cwd: workspaceFolder.uri.fsPath,
                    maxBuffer: 10 * 1024 * 1024
                }
            );

            return JSON.parse(stdout);
        } catch (error) {
            console.error('Error getting Git history:', error);
            return null;
        }
    }

    /**
     * Clear analysis cache
     */
    async clearCache(): Promise<void> {
        this.cache.clear();

        // Also clear Lucent's cache
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (workspaceFolder) {
            try {
                await execFileAsync(
                    'rm',
                    ['-rf', path.join(require('os').homedir(), '.cache', 'lucent')]
                );
            } catch (error) {
                console.error('Error clearing Lucent cache:', error);
            }
        }
    }

    /**
     * Get cached metrics for a file
     */
    getCached(filePath: string): FileMetrics | undefined {
        return this.cache.get(filePath);
    }

    /**
     * Extract file metrics from JSON output
     */
    private extractFileMetrics(result: any, filePath: string): FileMetrics {
        // This is a simplified extraction - adjust based on actual Lucent JSON output
        const fileName = path.basename(filePath);

        return {
            path: filePath,
            total_lines: result.total_lines || 0,
            code_lines: result.code_lines || 0,
            comment_lines: result.comment_lines || 0,
            blank_lines: result.blank_lines || 0,
            cyclomatic_complexity: result.avg_cyclomatic || 0,
            cognitive_complexity: result.avg_cognitive || 0,
            max_nesting_depth: result.avg_nesting_depth || 0,
            maintainability_index: result.avg_maintainability_index || 0,
            entropy: result.avg_entropy || 0,
            code_smells: this.extractCodeSmells(result)
        };
    }

    /**
     * Extract code smells from analysis result
     */
    private extractCodeSmells(result: any): CodeSmell[] {
        const smells: CodeSmell[] = [];

        // High complexity
        if (result.avg_cyclomatic > 10) {
            smells.push({
                type: 'HighComplexity',
                severity: 'High',
                message: `High cyclomatic complexity: ${result.avg_cyclomatic.toFixed(1)}`
            });
        }

        // Deep nesting
        if (result.avg_nesting_depth > 4) {
            smells.push({
                type: 'DeepNesting',
                severity: 'Medium',
                message: `Deep nesting detected: ${result.avg_nesting_depth.toFixed(1)} levels`
            });
        }

        // Low maintainability
        if (result.avg_maintainability_index < 65) {
            smells.push({
                type: 'LowMaintainability',
                severity: 'High',
                message: `Low maintainability index: ${result.avg_maintainability_index.toFixed(0)}`
            });
        }

        return smells;
    }
}
