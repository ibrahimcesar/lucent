package com.lucent.plugin.inspections

import com.intellij.codeInspection.*
import com.intellij.openapi.project.Project
import com.intellij.psi.PsiElementVisitor
import com.intellij.psi.PsiFile
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.settings.LucentSettings

/**
 * Inspection that warns about high cyclomatic complexity.
 *
 * This inspection analyzes files using Lucent and reports
 * when complexity exceeds the configured threshold.
 */
class ComplexityInspection : LocalInspectionTool() {

    override fun getDisplayName(): String = "High cyclomatic complexity"

    override fun getGroupDisplayName(): String = "Lucent Code Metrics"

    override fun getShortName(): String = "LucentComplexityInspection"

    override fun isEnabledByDefault(): Boolean = true

    override fun buildVisitor(
        holder: ProblemsHolder,
        isOnTheFly: Boolean
    ): PsiElementVisitor {
        return object : PsiElementVisitor() {
            override fun visitFile(file: PsiFile) {
                if (!isOnTheFly) return

                val virtualFile = file.virtualFile ?: return
                val project = file.project
                val settings = LucentSettings.getInstance()

                if (!settings.enableInspections) return

                val projectService = LucentProjectService.getInstance(project)
                val metrics = projectService.getMetrics(virtualFile) ?: return

                val threshold = settings.complexityThreshold

                if (metrics.cyclomaticComplexity > threshold) {
                    val severity = when {
                        metrics.cyclomaticComplexity > threshold * 2 -> ProblemHighlightType.ERROR
                        metrics.cyclomaticComplexity > threshold * 1.5 -> ProblemHighlightType.WARNING
                        else -> ProblemHighlightType.WEAK_WARNING
                    }

                    holder.registerProblem(
                        file,
                        "High cyclomatic complexity: ${metrics.cyclomaticComplexity} (threshold: $threshold). " +
                            "Consider breaking this file into smaller, more focused modules.",
                        severity
                    )
                }
            }
        }
    }
}
