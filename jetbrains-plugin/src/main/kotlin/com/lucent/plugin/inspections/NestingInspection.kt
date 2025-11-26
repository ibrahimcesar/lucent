package com.lucent.plugin.inspections

import com.intellij.codeInspection.*
import com.intellij.psi.PsiElementVisitor
import com.intellij.psi.PsiFile
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.settings.LucentSettings

/**
 * Inspection that warns about deeply nested code.
 *
 * Deep nesting makes code harder to read and understand.
 * This inspection reports files where the maximum nesting
 * depth exceeds the configured threshold.
 */
class NestingInspection : LocalInspectionTool() {

    override fun getDisplayName(): String = "Deep nesting detected"

    override fun getGroupDisplayName(): String = "Lucent Code Metrics"

    override fun getShortName(): String = "LucentNestingInspection"

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

                val threshold = settings.nestingThreshold

                if (metrics.maxNestingDepth > threshold) {
                    val severity = when {
                        metrics.maxNestingDepth > threshold + 3 -> ProblemHighlightType.ERROR
                        metrics.maxNestingDepth > threshold + 1 -> ProblemHighlightType.WARNING
                        else -> ProblemHighlightType.WEAK_WARNING
                    }

                    holder.registerProblem(
                        file,
                        "Deep nesting detected: ${metrics.maxNestingDepth} levels (threshold: $threshold). " +
                            "Consider using early returns, extracting functions, or restructuring logic.",
                        severity
                    )
                }
            }
        }
    }
}
