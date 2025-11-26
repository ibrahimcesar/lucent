package com.lucent.plugin.actions

import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.wm.ToolWindowManager
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.services.LucentService

/**
 * Action to analyze the entire project with Lucent.
 *
 * Available from the Analyze menu.
 */
class AnalyzeProjectAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            lucentService.notify(
                "Lucent Not Found",
                "Please install Lucent CLI with: cargo install lucent-code",
                NotificationType.ERROR
            )
            return
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Analyzing Project with Lucent", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Analyzing project..."
                indicator.isIndeterminate = true

                val projectService = LucentProjectService.getInstance(project)
                val result = projectService.analyzeProject()

                if (result != null) {
                    val stats = projectService.getProjectSummary()
                    val fileCount = stats?.totalFiles ?: 0
                    val grade = stats?.getMaintainabilityGrade() ?: "?"

                    lucentService.notify(
                        "Project Analysis Complete",
                        "Analyzed $fileCount files. Overall grade: $grade",
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
                        "Could not analyze project",
                        NotificationType.WARNING
                    )
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        e.presentation.isEnabled = e.project != null
    }
}
