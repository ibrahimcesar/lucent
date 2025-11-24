import * as vscode from 'vscode';
import { LucentAnalyzer, FileMetrics } from '../analyzer';

/**
 * Diagnostics Provider - Reports code quality issues
 *
 * Integrates with VS Code's Problems panel to show:
 * - High complexity warnings
 * - Low maintainability errors
 * - Deep nesting issues
 * - Code smell detections
 */
export class DiagnosticsProvider {
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor(private analyzer: LucentAnalyzer) {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('lucent');
    }

    async updateDiagnostics(uri: vscode.Uri): Promise<void> {
        const config = vscode.workspace.getConfiguration('lucent');

        if (!config.get('enabled')) {
            this.diagnosticCollection.clear();
            return;
        }

        const metrics = await this.analyzer.analyzeFile(uri.fsPath);

        if (!metrics) {
            this.diagnosticCollection.delete(uri);
            return;
        }

        const diagnostics = this.createDiagnostics(metrics, config);
        this.diagnosticCollection.set(uri, diagnostics);
    }

    private createDiagnostics(
        metrics: FileMetrics,
        config: vscode.WorkspaceConfiguration
    ): vscode.Diagnostic[] {
        const diagnostics: vscode.Diagnostic[] = [];
        const thresholds = config.get('thresholds', {
            maxComplexity: 10,
            maxNesting: 4,
            minMaintainability: 65
        });

        const severityLevel = this.getSeverityLevel(
            config.get('diagnosticSeverity', 'warning')
        );

        // Check complexity
        if (metrics.cyclomatic_complexity > thresholds.maxComplexity) {
            diagnostics.push(this.createComplexityDiagnostic(
                metrics.cyclomatic_complexity,
                thresholds.maxComplexity,
                severityLevel
            ));
        }

        // Check maintainability
        if (metrics.maintainability_index < thresholds.minMaintainability) {
            diagnostics.push(this.createMaintainabilityDiagnostic(
                metrics.maintainability_index,
                thresholds.minMaintainability,
                severityLevel
            ));
        }

        // Check nesting depth
        if (metrics.max_nesting_depth > thresholds.maxNesting) {
            diagnostics.push(this.createNestingDiagnostic(
                metrics.max_nesting_depth,
                thresholds.maxNesting,
                vscode.DiagnosticSeverity.Warning
            ));
        }

        // Add code smell diagnostics
        metrics.code_smells.forEach(smell => {
            diagnostics.push(this.createCodeSmellDiagnostic(smell));
        });

        // Check function length (if available)
        const maxFunctionLength = thresholds.maxFunctionLength || 50;
        if (metrics.total_lines > maxFunctionLength) {
            diagnostics.push(this.createLengthDiagnostic(
                metrics.total_lines,
                maxFunctionLength
            ));
        }

        return diagnostics;
    }

    private createComplexityDiagnostic(
        actual: number,
        threshold: number,
        severity: vscode.DiagnosticSeverity
    ): vscode.Diagnostic {
        const range = new vscode.Range(0, 0, 0, 0);
        const message = `High cyclomatic complexity: ${actual.toFixed(1)} (threshold: ${threshold})`;

        const diagnostic = new vscode.Diagnostic(range, message, severity);
        diagnostic.source = 'Lucent';
        diagnostic.code = 'high-complexity';

        // Add related information
        diagnostic.relatedInformation = [
            new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.parse(''), range),
                'Consider breaking this into smaller functions to reduce complexity'
            )
        ];

        return diagnostic;
    }

    private createMaintainabilityDiagnostic(
        actual: number,
        threshold: number,
        severity: vscode.DiagnosticSeverity
    ): vscode.Diagnostic {
        const range = new vscode.Range(0, 0, 0, 0);
        const message = `Low maintainability index: ${actual.toFixed(0)} (threshold: ${threshold})`;

        const diagnostic = new vscode.Diagnostic(range, message, severity);
        diagnostic.source = 'Lucent';
        diagnostic.code = 'low-maintainability';

        diagnostic.relatedInformation = [
            new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.parse(''), range),
                'Refactor to improve code maintainability'
            )
        ];

        return diagnostic;
    }

    private createNestingDiagnostic(
        actual: number,
        threshold: number,
        severity: vscode.DiagnosticSeverity
    ): vscode.Diagnostic {
        const range = new vscode.Range(0, 0, 0, 0);
        const message = `Deep nesting detected: ${actual} levels (threshold: ${threshold})`;

        const diagnostic = new vscode.Diagnostic(range, message, severity);
        diagnostic.source = 'Lucent';
        diagnostic.code = 'deep-nesting';

        diagnostic.relatedInformation = [
            new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.parse(''), range),
                'Extract nested logic into separate functions'
            )
        ];

        return diagnostic;
    }

    private createCodeSmellDiagnostic(smell: any): vscode.Diagnostic {
        const line = smell.line || 0;
        const range = new vscode.Range(line, 0, line, Number.MAX_VALUE);

        const severity = smell.severity === 'High'
            ? vscode.DiagnosticSeverity.Warning
            : smell.severity === 'Medium'
            ? vscode.DiagnosticSeverity.Information
            : vscode.DiagnosticSeverity.Hint;

        const diagnostic = new vscode.Diagnostic(range, smell.message, severity);
        diagnostic.source = 'Lucent';
        diagnostic.code = smell.type.toLowerCase();

        return diagnostic;
    }

    private createLengthDiagnostic(
        actual: number,
        threshold: number
    ): vscode.Diagnostic {
        const range = new vscode.Range(0, 0, 0, 0);
        const message = `Function exceeds recommended length: ${actual} lines (recommended: ${threshold})`;

        const diagnostic = new vscode.Diagnostic(
            range,
            message,
            vscode.DiagnosticSeverity.Hint
        );
        diagnostic.source = 'Lucent';
        diagnostic.code = 'long-function';

        diagnostic.relatedInformation = [
            new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.parse(''), range),
                'Consider extracting sub-functions for better readability'
            )
        ];

        return diagnostic;
    }

    private getSeverityLevel(level: string): vscode.DiagnosticSeverity {
        switch (level) {
            case 'error':
                return vscode.DiagnosticSeverity.Error;
            case 'warning':
                return vscode.DiagnosticSeverity.Warning;
            case 'information':
                return vscode.DiagnosticSeverity.Information;
            case 'hint':
                return vscode.DiagnosticSeverity.Hint;
            default:
                return vscode.DiagnosticSeverity.Warning;
        }
    }

    dispose(): void {
        this.diagnosticCollection.dispose();
    }
}
