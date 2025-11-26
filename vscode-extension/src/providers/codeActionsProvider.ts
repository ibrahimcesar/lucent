import * as vscode from 'vscode';
import { LucentAnalyzer, FileMetrics } from '../analyzer';
import { LicenseManager, requiresFeature } from '../license';

export class CodeActionsProvider implements vscode.CodeActionProvider {
    private analyzer: LucentAnalyzer;
    private licenseManager: LicenseManager;

    constructor(analyzer: LucentAnalyzer) {
        this.analyzer = analyzer;
        this.licenseManager = LicenseManager.getInstance();
    }

    async provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];

        // Get file metrics
        const metrics = this.analyzer.getCached(document.uri.fsPath);
        if (!metrics) {
            return actions;
        }

        // Add actions for each diagnostic
        for (const diagnostic of context.diagnostics) {
            if (diagnostic.source !== 'lucent') {
                continue;
            }

            // Add quick fixes based on diagnostic type
            const quickFixes = await this.getQuickFixes(document, diagnostic, metrics);
            actions.push(...quickFixes);
        }

        // Add general refactoring actions
        const refactorActions = await this.getRefactoringActions(document, range, metrics);
        actions.push(...refactorActions);

        return actions;
    }

    /**
     * Get quick fixes for a specific diagnostic
     */
    private async getQuickFixes(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic,
        metrics: FileMetrics
    ): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];

        // Check if auto-fix is available in current tier
        const hasAutoFix = await this.licenseManager.hasFeature('auto-fix');

        if (!hasAutoFix) {
            // Show upgrade prompt action
            const upgradeAction = new vscode.CodeAction(
                'Unlock Auto-Fix (Pro feature)',
                vscode.CodeActionKind.QuickFix
            );
            upgradeAction.command = {
                command: 'lucent.promptUpgrade',
                title: 'Upgrade to Pro',
                arguments: ['auto-fix']
            };
            actions.push(upgradeAction);
            return actions;
        }

        // High complexity fix
        if (diagnostic.message.includes('complexity')) {
            actions.push(this.createExtractFunctionAction(document, diagnostic));
            actions.push(this.createSimplifyLogicAction(document, diagnostic));
        }

        // Deep nesting fix
        if (diagnostic.message.includes('nesting')) {
            actions.push(this.createReduceNestingAction(document, diagnostic));
            actions.push(this.createEarlyReturnAction(document, diagnostic));
        }

        // Long function fix
        if (diagnostic.message.includes('function length')) {
            actions.push(this.createSplitFunctionAction(document, diagnostic));
        }

        // Magic numbers fix
        if (diagnostic.message.includes('magic number')) {
            actions.push(this.createExtractConstantAction(document, diagnostic));
        }

        // Add "Explain this issue" action (always available)
        actions.push(this.createExplainAction(document, diagnostic));

        return actions;
    }

    /**
     * Get refactoring actions for selected code
     */
    private async getRefactoringActions(
        document: vscode.TextDocument,
        range: vscode.Range,
        metrics: FileMetrics
    ): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];

        const hasAutoFix = await this.licenseManager.hasFeature('auto-fix');
        if (!hasAutoFix) {
            return actions;
        }

        // Extract method (if selection is multi-line)
        if (range.end.line - range.start.line > 2) {
            actions.push(this.createExtractMethodRefactor(document, range));
        }

        // Introduce constant (if selection looks like a literal)
        const selectedText = document.getText(range);
        if (this.isLiteral(selectedText)) {
            actions.push(this.createIntroduceConstantRefactor(document, range));
        }

        return actions;
    }

    /**
     * Create "Extract Function" quick fix
     */
    private createExtractFunctionAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Extract complex logic into separate function',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.extractFunction',
            title: 'Extract Function',
            arguments: [document.uri, diagnostic.range]
        };

        action.isPreferred = true;

        return action;
    }

    /**
     * Create "Simplify Logic" quick fix
     */
    private createSimplifyLogicAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Simplify conditional logic',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.simplifyLogic',
            title: 'Simplify Logic',
            arguments: [document.uri, diagnostic.range]
        };

        return action;
    }

    /**
     * Create "Reduce Nesting" quick fix
     */
    private createReduceNestingAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Reduce nesting with guard clauses',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.reduceNesting',
            title: 'Reduce Nesting',
            arguments: [document.uri, diagnostic.range]
        };

        action.isPreferred = true;

        return action;
    }

    /**
     * Create "Early Return" quick fix
     */
    private createEarlyReturnAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Use early return pattern',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.earlyReturn',
            title: 'Early Return',
            arguments: [document.uri, diagnostic.range]
        };

        return action;
    }

    /**
     * Create "Split Function" quick fix
     */
    private createSplitFunctionAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Split into smaller functions',
            vscode.CodeActionKind.Refactor
        );

        action.command = {
            command: 'lucent.splitFunction',
            title: 'Split Function',
            arguments: [document.uri, diagnostic.range]
        };

        return action;
    }

    /**
     * Create "Extract Constant" quick fix
     */
    private createExtractConstantAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Extract magic number to named constant',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.extractConstant',
            title: 'Extract Constant',
            arguments: [document.uri, diagnostic.range]
        };

        action.isPreferred = true;

        return action;
    }

    /**
     * Create "Explain Issue" action
     */
    private createExplainAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Explain this quality issue',
            vscode.CodeActionKind.QuickFix
        );

        action.command = {
            command: 'lucent.explainIssue',
            title: 'Explain Issue',
            arguments: [document.uri, diagnostic]
        };

        return action;
    }

    /**
     * Create extract method refactoring
     */
    private createExtractMethodRefactor(
        document: vscode.TextDocument,
        range: vscode.Range
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Extract to method',
            vscode.CodeActionKind.RefactorExtract
        );

        action.command = {
            command: 'lucent.extractMethod',
            title: 'Extract Method',
            arguments: [document.uri, range]
        };

        return action;
    }

    /**
     * Create introduce constant refactoring
     */
    private createIntroduceConstantRefactor(
        document: vscode.TextDocument,
        range: vscode.Range
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            'Introduce constant',
            vscode.CodeActionKind.RefactorExtract
        );

        action.command = {
            command: 'lucent.introduceConstant',
            title: 'Introduce Constant',
            arguments: [document.uri, range]
        );

        return action;
    }

    /**
     * Check if text is a literal value
     */
    private isLiteral(text: string): boolean {
        text = text.trim();
        // Check for number, string, or boolean literals
        return /^(\d+\.?\d*|\".*\"|'.*'|true|false)$/.test(text);
    }
}

/**
 * Code action command handlers
 */
export class CodeActionCommands {
    static register(context: vscode.ExtensionContext) {
        // Extract Function
        context.subscriptions.push(
            vscode.commands.registerCommand(
                'lucent.extractFunction',
                async (uri: vscode.Uri, range: vscode.Range) => {
                    const document = await vscode.workspace.openTextDocument(uri);
                    const selectedCode = document.getText(range);

                    const functionName = await vscode.window.showInputBox({
                        prompt: 'Enter function name',
                        placeHolder: 'extractedFunction',
                        validateInput: (value) => {
                            if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(value)) {
                                return 'Invalid function name';
                            }
                            return null;
                        }
                    });

                    if (functionName) {
                        const edit = new vscode.WorkspaceEdit();

                        // Replace selected code with function call
                        edit.replace(uri, range, `${functionName}()`);

                        // Insert function definition above
                        const functionDef = `\nfunction ${functionName}() {\n${selectedCode}\n}\n\n`;
                        edit.insert(uri, new vscode.Position(range.start.line - 1, 0), functionDef);

                        await vscode.workspace.applyEdit(edit);
                        vscode.window.showInformationMessage(`Extracted to ${functionName}()`);
                    }
                }
            )
        );

        // Reduce Nesting (Early Return Pattern)
        context.subscriptions.push(
            vscode.commands.registerCommand(
                'lucent.reduceNesting',
                async (uri: vscode.Uri, range: vscode.Range) => {
                    const document = await vscode.workspace.openTextDocument(uri);
                    const code = document.getText(range);

                    // Simple transformation: invert condition and return early
                    // This is a simplified example - production would use AST transformation
                    const transformed = code.replace(
                        /if\s*\((.*?)\)\s*{/g,
                        'if (!($1)) return;\n'
                    );

                    const edit = new vscode.WorkspaceEdit();
                    edit.replace(uri, range, transformed);

                    await vscode.workspace.applyEdit(edit);
                    vscode.window.showInformationMessage('Applied early return pattern');
                }
            )
        );

        // Extract Constant
        context.subscriptions.push(
            vscode.commands.registerCommand(
                'lucent.extractConstant',
                async (uri: vscode.Uri, range: vscode.Range) => {
                    const document = await vscode.workspace.openTextDocument(uri);
                    const value = document.getText(range);

                    const constantName = await vscode.window.showInputBox({
                        prompt: 'Enter constant name',
                        placeHolder: 'MY_CONSTANT',
                        value: 'MY_CONSTANT'
                    });

                    if (constantName) {
                        const edit = new vscode.WorkspaceEdit();

                        // Replace value with constant name
                        edit.replace(uri, range, constantName);

                        // Add constant declaration at top of file
                        const constDecl = `const ${constantName} = ${value};\n`;
                        edit.insert(uri, new vscode.Position(0, 0), constDecl);

                        await vscode.workspace.applyEdit(edit);
                        vscode.window.showInformationMessage(`Created constant ${constantName}`);
                    }
                }
            )
        );

        // Explain Issue
        context.subscriptions.push(
            vscode.commands.registerCommand(
                'lucent.explainIssue',
                async (uri: vscode.Uri, diagnostic: vscode.Diagnostic) => {
                    const explanations: Record<string, string> = {
                        complexity: `**High Complexity Detected**

This code has a cyclomatic complexity score that exceeds the recommended threshold.

**What it means:**
- More decision points (if/else, switch, loops)
- Harder to test (more code paths)
- Higher bug risk

**How to fix:**
1. Extract complex logic into smaller functions
2. Simplify conditional statements
3. Use early returns to reduce nesting
4. Consider using strategy/command patterns

**Target:** Keep complexity < 10 for most functions`,

                        nesting: `**Deep Nesting Detected**

Your code has too many nested levels (loops, conditionals).

**Problems:**
- Hard to read and understand
- Difficult to debug
- Error-prone

**Solutions:**
1. Use guard clauses (early returns)
2. Extract nested blocks into functions
3. Invert conditions when possible
4. Use functional methods (map, filter) instead of loops

**Example:**
Instead of:
\`\`\`
if (x) {
    if (y) {
        if (z) {
            // deep
        }
    }
}
\`\`\`

Use:
\`\`\`
if (!x) return;
if (!y) return;
if (!z) return;
// logic here
\`\`\``,

                        maintainability: `**Low Maintainability Index**

Your code's maintainability score is below 65.

**What contributes:**
- High complexity
- Long functions
- Poor naming
- Lack of comments

**Improvement steps:**
1. Break down large functions
2. Add meaningful comments
3. Use descriptive variable names
4. Reduce coupling
5. Increase cohesion

**Goal:** Aim for MI > 65 (Good) or > 85 (Excellent)`
                    };

                    const messageType = diagnostic.message.toLowerCase();
                    let explanation = 'No explanation available for this issue.';

                    for (const [key, value] of Object.entries(explanations)) {
                        if (messageType.includes(key)) {
                            explanation = value;
                            break;
                        }
                    }

                    const panel = vscode.window.createWebviewPanel(
                        'lucentExplanation',
                        'Quality Issue Explanation',
                        vscode.ViewColumn.Beside,
                        {
                            enableScripts: true
                        }
                    );

                    panel.webview.html = getExplanationHtml(diagnostic.message, explanation);
                }
            )
        );

        // Prompt Upgrade
        context.subscriptions.push(
            vscode.commands.registerCommand(
                'lucent.promptUpgrade',
                async (feature: string) => {
                    const licenseManager = LicenseManager.getInstance();
                    await licenseManager.promptUpgrade(feature);
                }
            )
        );
    }
}

function getExplanationHtml(issue: string, explanation: string): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {
                font-family: var(--vscode-font-family);
                color: var(--vscode-foreground);
                background: var(--vscode-editor-background);
                padding: 20px;
                line-height: 1.6;
            }
            h1 {
                color: var(--vscode-textLink-foreground);
                border-bottom: 2px solid var(--vscode-panel-border);
                padding-bottom: 10px;
            }
            pre {
                background: var(--vscode-textBlockQuote-background);
                padding: 15px;
                border-left: 4px solid var(--vscode-textLink-foreground);
                overflow-x: auto;
            }
            code {
                font-family: var(--vscode-editor-font-family);
            }
            .issue {
                background: var(--vscode-inputValidation-warningBackground);
                padding: 10px;
                border-radius: 4px;
                margin: 20px 0;
            }
        </style>
    </head>
    <body>
        <h1>💡 Code Quality Issue</h1>
        <div class="issue">
            <strong>Issue:</strong> ${issue}
        </div>
        ${explanation.split('\n').join('<br>')}
    </body>
    </html>
    `;
}
