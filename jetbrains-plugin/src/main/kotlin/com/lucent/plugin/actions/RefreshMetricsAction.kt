package com.lucent.plugin.actions

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.lucent.plugin.services.LucentProjectService

/**
 * Action to refresh metrics for the current file.
 */
class RefreshMetricsAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE) ?: return

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Refreshing Metrics", false) {
            override fun run(indicator: ProgressIndicator) {
                val projectService = LucentProjectService.getInstance(project)
                projectService.refreshMetrics(file)
            }
        })
    }

    override fun update(e: AnActionEvent) {
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE)
        e.presentation.isEnabled = e.project != null && file != null && !file.isDirectory
    }
}
