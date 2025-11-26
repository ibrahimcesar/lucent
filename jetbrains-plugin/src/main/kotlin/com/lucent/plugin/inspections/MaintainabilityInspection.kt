package com.lucent.plugin.inspections

import com.intellij.codeInspection.*
import com.intellij.psi.PsiElementVisitor
import com.intellij.psi.PsiFile
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.settings.LucentSettings

/**
 * Inspection that warns about low maintainability index.
 *
 * The Maintainability Index (MI) is a composite metric that considers
 * cyclomatic complexity, Halstead volume, and lines of code.
 * Lower values indicate harder-to-maintain code.
 */
class MaintainabilityInspection : LocalInspectionTool() {

    override fun getDisplayName(): String = "Low maintainability index"

    override fun getGroupDisplayName(): String = "Lucent Code Metrics"

    override fun getShortName(): String = "LucentMaintainabilityInspection"

    override fun isEnabledByDefault(): Boolean = true

    override fun buildVisitor(
        holder: ProblemsHolder,
        isOnTheFly: Boolean
    ): PsiElementVisitor {
        return object : PsiElementVisitor() {
            override fun visitFile(file: PsiFile) {
                if (!isOnTheFly) return

                val virtualFile = file.virtualFile ?: return
                val settings = LucentSettings.getInstance()

                if (!settings.enableInspections) return

                val projectService = LucentProjectService.getInstance(file.project)
                val metrics = projectService.getMetrics(virtualFile) ?: return

                val threshold = settings.maintainabilityThreshold

                if (metrics.maintainabilityIndex < threshold) {
                    val grade = metrics.getMaintainabilityGrade()
                    val severity = when {
                        metrics.maintainabilityIndex < 25 -> ProblemHighlightType.ERROR
                        metrics.maintainabilityIndex < 50 -> ProblemHighlightType.WARNING
                        else -> ProblemHighlightType.WEAK_WARNING
                    }

                    holder.registerProblem(
                        file,
                        "Low maintainability index: ${String.format("%.1f", metrics.maintainabilityIndex)} " +
                            "(Grade: $grade, threshold: $threshold). " +
                            "Consider refactoring to improve code clarity and reduce complexity.",
                        severity
                    )
                }
            }
        }
    }
}
