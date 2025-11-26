package com.lucent.plugin.actions

import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.wm.ToolWindowManager
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.services.LucentService

/**
 * Action to analyze the current file with Lucent.
 *
 * Available from:
 * - Analyze menu
 * - Editor context menu
 * - Keyboard shortcut (Ctrl+Alt+L)
 */
class AnalyzeFileAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE) ?: return
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            lucentService.notify(
                "Lucent Not Found",
                "Please install Lucent CLI with: cargo install lucent-code",
                NotificationType.ERROR
            )
            return
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Analyzing with Lucent", false) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Analyzing ${file.name}..."

                val projectService = LucentProjectService.getInstance(project)
                val metrics = projectService.refreshMetrics(file)

                if (metrics != null) {
                    val grade = metrics.getMaintainabilityGrade()
                    lucentService.notify(
                        "Lucent Analysis Complete",
                        "${file.name}: Complexity=${metrics.cyclomaticComplexity}, MI=${String.format("%.1f", metrics.maintainabilityIndex)} ($grade)",
                        NotificationType.INFORMATION
                    )

                    // Open tool window
                    com.intellij.openapi.application.ApplicationManager.getApplication().invokeLater {
                        val toolWindow = ToolWindowManager.getInstance(project).getToolWindow("Lucent Metrics")
                        toolWindow?.show()
                    }
                } else {
                    lucentService.notify(
                        "Analysis Failed",
                        "Could not analyze ${file.name}",
                        NotificationType.WARNING
                    )
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE)
        e.presentation.isEnabled = file != null && !file.isDirectory
    }
}
