import * as vscode from 'vscode';
import { LucentAnalyzer, FileMetrics } from '../analyzer';

/**
 * Hover Provider - Shows detailed metrics on hover
 *
 * Displays rich tooltips with:
 * - Function-level metrics (complexity, maintainability)
 * - Code quality explanations
 * - Refactoring suggestions
 * - Threshold comparisons
 */
export class HoverProvider implements vscode.HoverProvider {
    constructor(private analyzer: LucentAnalyzer) {}

    async provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): Promise<vscode.Hover | null> {
        // Get metrics for current file
        const metrics = await this.analyzer.analyzeFile(document.uri.fsPath);

        if (!metrics) {
            return null;
        }

        // Get the word at the current position
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return null;
        }

        const word = document.getText(wordRange);

        // Check if we're hovering over a function/method name
        const line = document.lineAt(position.line).text;
        const isFunctionDefinition = /\b(function|fn|def|func|method|proc)\b/.test(line);

        if (isFunctionDefinition) {
            return this.createFunctionHover(metrics, word);
        }

        // Default: show file-level metrics
        return this.createFileHover(metrics);
    }

    private createFunctionHover(metrics: FileMetrics, functionName: string): vscode.Hover {
        const markdown = new vscode.MarkdownString();
        markdown.isTrusted = true;
        markdown.supportHtml = true;

        markdown.appendMarkdown(`### 📊 Metrics for \`${functionName}()\`\n\n`);
        markdown.appendMarkdown('---\n\n');

        // Complexity section
        const complexityEmoji = this.getComplexityEmoji(metrics.cyclomatic_complexity);
        markdown.appendMarkdown(`**Cyclomatic Complexity:** ${metrics.cyclomatic_complexity.toFixed(1)} ${complexityEmoji}\n\n`);

        if (metrics.cyclomatic_complexity > 10) {
            markdown.appendMarkdown('⚠️ *High complexity detected*\n\n');
            markdown.appendMarkdown('- Consider breaking into smaller functions\n');
            markdown.appendMarkdown('- Reduce conditional branches\n');
            markdown.appendMarkdown('- Extract complex logic\n\n');
        } else {
            markdown.appendMarkdown('✅ *Complexity is within recommended range*\n\n');
        }

        // Maintainability section
        const miEmoji = this.getMaintainabilityEmoji(metrics.maintainability_index);
        markdown.appendMarkdown(`**Maintainability Index:** ${metrics.maintainability_index.toFixed(0)} ${miEmoji}\n\n`);
        markdown.appendMarkdown(this.getMaintainabilityDescription(metrics.maintainability_index));

        // Lines of code
        markdown.appendMarkdown(`\n**Lines of Code:**\n`);
        markdown.appendMarkdown(`- Total: ${metrics.total_lines}\n`);
        markdown.appendMarkdown(`- Code: ${metrics.code_lines}\n`);
        markdown.appendMarkdown(`- Comments: ${metrics.comment_lines}\n`);
        markdown.appendMarkdown(`- Blank: ${metrics.blank_lines}\n\n`);

        // Nesting depth
        if (metrics.max_nesting_depth > 4) {
            markdown.appendMarkdown(`⚠️ **Deep Nesting:** ${metrics.max_nesting_depth} levels\n\n`);
            markdown.appendMarkdown('Consider flattening nested structures\n\n');
        }

        // Code smells
        if (metrics.code_smells.length > 0) {
            markdown.appendMarkdown(`**Code Smells (${metrics.code_smells.length}):**\n\n`);
            metrics.code_smells.forEach(smell => {
                const severityIcon = smell.severity === 'High' ? '🔴' : smell.severity === 'Medium' ? '🟡' : '🟢';
                markdown.appendMarkdown(`${severityIcon} ${smell.message}\n`);
            });
        }

        markdown.appendMarkdown('\n---\n\n');
        markdown.appendMarkdown('💡 *Click code lens above for detailed analysis*');

        return new vscode.Hover(markdown);
    }

    private createFileHover(metrics: FileMetrics): vscode.Hover {
        const markdown = new vscode.MarkdownString();
        markdown.isTrusted = true;

        markdown.appendMarkdown(`### 📄 File Metrics\n\n`);
        markdown.appendMarkdown('---\n\n');

        // Overall quality
        const qualityScore = this.calculateQualityScore(metrics);
        markdown.appendMarkdown(`**Overall Quality:** ${qualityScore.toFixed(0)}/100 ${this.getQualityEmoji(qualityScore)}\n\n`);

        // Key metrics
        markdown.appendMarkdown(`| Metric | Value | Status |\n`);
        markdown.appendMarkdown(`|--------|-------|--------|\n`);
        markdown.appendMarkdown(`| Complexity | ${metrics.cyclomatic_complexity.toFixed(1)} | ${this.getComplexityStatus(metrics.cyclomatic_complexity)} |\n`);
        markdown.appendMarkdown(`| Maintainability | ${metrics.maintainability_index.toFixed(0)} | ${this.getMaintainabilityStatus(metrics.maintainability_index)} |\n`);
        markdown.appendMarkdown(`| Nesting Depth | ${metrics.max_nesting_depth} | ${this.getNestingStatus(metrics.max_nesting_depth)} |\n`);
        markdown.appendMarkdown(`| Code Smells | ${metrics.code_smells.length} | ${metrics.code_smells.length === 0 ? '✅' : '⚠️'} |\n\n`);

        // Code breakdown
        markdown.appendMarkdown(`**Code Distribution:**\n`);
        const codePercent = (metrics.code_lines / metrics.total_lines * 100).toFixed(1);
        const commentPercent = (metrics.comment_lines / metrics.total_lines * 100).toFixed(1);
        markdown.appendMarkdown(`- Code: ${codePercent}% (${metrics.code_lines} lines)\n`);
        markdown.appendMarkdown(`- Comments: ${commentPercent}% (${metrics.comment_lines} lines)\n`);
        markdown.appendMarkdown(`- Blank: ${metrics.blank_lines} lines\n\n`);

        // Entropy
        markdown.appendMarkdown(`**Information Entropy:** ${metrics.entropy.toFixed(2)}\n`);
        markdown.appendMarkdown(this.getEntropyDescription(metrics.entropy));

        return new vscode.Hover(markdown);
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

    private getQualityEmoji(score: number): string {
        if (score >= 85) return '🟢 Excellent';
        if (score >= 70) return '🟡 Good';
        if (score >= 50) return '🟠 Fair';
        return '🔴 Needs Work';
    }

    private getComplexityStatus(complexity: number): string {
        if (complexity <= 5) return '✅ Simple';
        if (complexity <= 10) return '✅ Acceptable';
        if (complexity <= 15) return '⚠️ Complex';
        return '🔴 Too Complex';
    }

    private getMaintainabilityStatus(mi: number): string {
        if (mi >= 85) return '✅ Excellent';
        if (mi >= 65) return '✅ Good';
        if (mi >= 50) return '⚠️ Fair';
        return '🔴 Poor';
    }

    private getNestingStatus(depth: number): string {
        if (depth <= 3) return '✅ Flat';
        if (depth <= 4) return '✅ Acceptable';
        if (depth <= 6) return '⚠️ Deep';
        return '🔴 Too Deep';
    }

    private getMaintainabilityDescription(mi: number): string {
        if (mi >= 85) {
            return '✅ **Excellent!** Code is highly maintainable.\n\n';
        } else if (mi >= 65) {
            return '✅ **Good.** Code is maintainable with minor improvements possible.\n\n';
        } else if (mi >= 50) {
            return '⚠️ **Fair.** Consider refactoring to improve maintainability.\n\n';
        } else {
            return '🔴 **Needs Work.** Refactoring recommended.\n\n' +
                   '**Suggestions:**\n' +
                   '- Break down complex functions\n' +
                   '- Improve code organization\n' +
                   '- Add meaningful comments\n' +
                   '- Reduce coupling\n\n';
        }
    }

    private getEntropyDescription(entropy: number): string {
        if (entropy < 3) {
            return '*Low entropy - code is simple and predictable*\n\n';
        } else if (entropy < 5) {
            return '*Moderate entropy - balanced complexity*\n\n';
        } else {
            return '*High entropy - code is information-dense*\n\n';
        }
    }

    private calculateQualityScore(metrics: FileMetrics): number {
        // Simple quality score calculation
        let score = 100;

        // Deduct for high complexity (max -30 points)
        if (metrics.cyclomatic_complexity > 10) {
            score -= Math.min(30, (metrics.cyclomatic_complexity - 10) * 3);
        }

        // Deduct for low maintainability (max -40 points)
        if (metrics.maintainability_index < 85) {
            score -= (85 - metrics.maintainability_index) * 0.4;
        }

        // Deduct for deep nesting (max -15 points)
        if (metrics.max_nesting_depth > 4) {
            score -= Math.min(15, (metrics.max_nesting_depth - 4) * 5);
        }

        // Deduct for code smells (max -15 points)
        score -= Math.min(15, metrics.code_smells.length * 3);

        return Math.max(0, score);
    }
}
