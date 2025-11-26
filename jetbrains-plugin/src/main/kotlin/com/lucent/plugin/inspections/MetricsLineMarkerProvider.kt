package com.lucent.plugin.inspections

import com.intellij.codeInsight.daemon.LineMarkerInfo
import com.intellij.codeInsight.daemon.LineMarkerProvider
import com.intellij.openapi.editor.markup.GutterIconRenderer
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.ui.JBColor
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.services.Severity
import com.lucent.plugin.settings.LucentSettings
import java.awt.Color
import java.awt.Component
import java.awt.Graphics
import javax.swing.Icon

/**
 * Line marker provider that shows complexity indicators in the gutter.
 *
 * Displays colored circles next to files based on their metrics:
 * - Green: Good metrics
 * - Yellow: Warning level
 * - Orange: High complexity
 * - Red: Critical issues
 */
class MetricsLineMarkerProvider : LineMarkerProvider {

    override fun getLineMarkerInfo(element: PsiElement): LineMarkerInfo<*>? {
        // Only process the first element in a file
        if (element.parent !is PsiFile) return null
        if (element.prevSibling != null) return null

        val settings = LucentSettings.getInstance()
        if (!settings.enableGutterIcons) return null

        val file = element.containingFile ?: return null
        val virtualFile = file.virtualFile ?: return null

        val projectService = LucentProjectService.getInstance(element.project)
        val metrics = projectService.getMetrics(virtualFile) ?: return null

        // Determine overall severity based on metrics
        val complexitySeverity = metrics.getComplexitySeverity()
        val nestingSeverity = metrics.getNestingSeverity()
        val maintainabilitySeverity = when {
            metrics.maintainabilityIndex >= 85 -> Severity.LOW
            metrics.maintainabilityIndex >= 65 -> Severity.MEDIUM
            metrics.maintainabilityIndex >= 50 -> Severity.HIGH
            else -> Severity.CRITICAL
        }

        // Use the worst severity
        val worstSeverity = listOf(complexitySeverity, nestingSeverity, maintainabilitySeverity)
            .maxByOrNull { it.ordinal } ?: Severity.LOW

        val icon = MetricsIcon(worstSeverity)
        val tooltip = buildTooltip(metrics)

        return LineMarkerInfo(
            element,
            element.textRange,
            icon,
            { tooltip },
            null,
            GutterIconRenderer.Alignment.RIGHT,
            { "Lucent Metrics" }
        )
    }

    private fun buildTooltip(metrics: com.lucent.plugin.services.FileMetrics): String {
        val grade = metrics.getMaintainabilityGrade()
        return buildString {
            appendLine("<html><body>")
            appendLine("<b>Lucent Metrics</b><br/>")
            appendLine("<table>")
            appendLine("<tr><td>Cyclomatic Complexity:</td><td><b>${metrics.cyclomaticComplexity}</b></td></tr>")
            appendLine("<tr><td>Cognitive Complexity:</td><td><b>${metrics.cognitiveComplexity}</b></td></tr>")
            appendLine("<tr><td>Max Nesting Depth:</td><td><b>${metrics.maxNestingDepth}</b></td></tr>")
            appendLine("<tr><td>Maintainability Index:</td><td><b>${String.format("%.1f", metrics.maintainabilityIndex)} ($grade)</b></td></tr>")
            appendLine("<tr><td>Lines of Code:</td><td>${metrics.codeLines}</td></tr>")

            metrics.functionCount?.let {
                appendLine("<tr><td>Functions:</td><td>$it</td></tr>")
            }

            val todoCount = (metrics.todoCount ?: 0) + (metrics.fixmeCount ?: 0) + (metrics.hackCount ?: 0)
            if (todoCount > 0) {
                appendLine("<tr><td>TODOs/FIXMEs:</td><td>$todoCount</td></tr>")
            }

            appendLine("</table>")
            appendLine("</body></html>")
        }
    }
}

/**
 * Custom icon for displaying metrics severity in the gutter.
 */
class MetricsIcon(private val severity: Severity) : Icon {
    private val size = 12

    override fun paintIcon(c: Component?, g: Graphics, x: Int, y: Int) {
        val color = when (severity) {
            Severity.LOW -> JBColor(Color(0x4CAF50), Color(0x81C784))      // Green
            Severity.MEDIUM -> JBColor(Color(0xFFC107), Color(0xFFD54F))   // Yellow
            Severity.HIGH -> JBColor(Color(0xFF9800), Color(0xFFB74D))     // Orange
            Severity.CRITICAL -> JBColor(Color(0xF44336), Color(0xE57373)) // Red
        }

        g.color = color
        g.fillOval(x + 2, y + 2, size - 4, size - 4)

        // Draw border
        g.color = color.darker()
        g.drawOval(x + 2, y + 2, size - 4, size - 4)
    }

    override fun getIconWidth(): Int = size

    override fun getIconHeight(): Int = size
}
