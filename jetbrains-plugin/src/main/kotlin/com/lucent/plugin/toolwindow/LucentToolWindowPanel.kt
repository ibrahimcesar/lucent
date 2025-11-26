package com.lucent.plugin.toolwindow

import com.intellij.icons.AllIcons
import com.intellij.openapi.actionSystem.*
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.fileEditor.FileEditorManager
import com.intellij.openapi.fileEditor.FileEditorManagerListener
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.ui.JBColor
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBPanel
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.table.JBTable
import com.intellij.util.ui.JBUI
import com.lucent.plugin.services.FileMetrics
import com.lucent.plugin.services.LucentProjectService
import com.lucent.plugin.services.LucentService
import java.awt.BorderLayout
import java.awt.Color
import java.awt.Component
import java.awt.GridBagConstraints
import java.awt.GridBagLayout
import javax.swing.*
import javax.swing.table.AbstractTableModel
import javax.swing.table.DefaultTableCellRenderer

/**
 * Main panel for the Lucent Metrics tool window.
 *
 * Displays:
 * - Current file metrics
 * - Project-wide statistics
 * - List of problematic files
 */
class LucentToolWindowPanel(private val project: Project) : JBPanel<LucentToolWindowPanel>(BorderLayout()) {

    private val fileMetricsPanel = FileMetricsPanel()
    private val projectMetricsPanel = ProjectMetricsPanel()
    private val problemFilesTable = ProblemFilesTable(project)

    init {
        setupUI()
        setupListeners()
        refreshCurrentFile()
    }

    private fun setupUI() {
        // Toolbar
        val toolbar = createToolbar()
        add(toolbar.component, BorderLayout.NORTH)

        // Main content with tabs
        val tabbedPane = JTabbedPane()

        // Current file tab
        val currentFilePanel = JBPanel<JBPanel<*>>(BorderLayout()).apply {
            add(fileMetricsPanel, BorderLayout.CENTER)
        }
        tabbedPane.addTab("Current File", currentFilePanel)

        // Project overview tab
        val projectPanel = JBPanel<JBPanel<*>>(BorderLayout()).apply {
            add(projectMetricsPanel, BorderLayout.NORTH)
            add(JBScrollPane(problemFilesTable), BorderLayout.CENTER)
        }
        tabbedPane.addTab("Project", projectPanel)

        add(tabbedPane, BorderLayout.CENTER)
    }

    private fun createToolbar(): ActionToolbar {
        val actionGroup = DefaultActionGroup().apply {
            add(object : AnAction("Refresh", "Refresh metrics", AllIcons.Actions.Refresh) {
                override fun actionPerformed(e: AnActionEvent) {
                    refreshCurrentFile()
                }
            })

            add(object : AnAction("Analyze Project", "Analyze entire project", AllIcons.Actions.Execute) {
                override fun actionPerformed(e: AnActionEvent) {
                    analyzeProject()
                }
            })

            addSeparator()

            add(object : AnAction("Clear Cache", "Clear metrics cache", AllIcons.Actions.GC) {
                override fun actionPerformed(e: AnActionEvent) {
                    LucentProjectService.getInstance(project).clearCache()
                    fileMetricsPanel.clear()
                    projectMetricsPanel.clear()
                    problemFilesTable.clear()
                }
            })
        }

        return ActionManager.getInstance()
            .createActionToolbar("LucentToolbar", actionGroup, true)
    }

    private fun setupListeners() {
        // Listen for file selection changes
        project.messageBus.connect().subscribe(
            FileEditorManagerListener.FILE_EDITOR_MANAGER,
            object : FileEditorManagerListener {
                override fun selectionChanged(event: com.intellij.openapi.fileEditor.FileEditorManagerEvent) {
                    event.newFile?.let { refreshMetrics(it) }
                }
            }
        )
    }

    private fun refreshCurrentFile() {
        val editor = FileEditorManager.getInstance(project).selectedEditor
        val file = editor?.file ?: return
        refreshMetrics(file)
    }

    private fun refreshMetrics(file: VirtualFile) {
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            fileMetricsPanel.showError("Lucent CLI not found. Please install with: cargo install lucent-code")
            return
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Analyzing with Lucent", false) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Analyzing ${file.name}..."
                val projectService = LucentProjectService.getInstance(project)
                val metrics = projectService.refreshMetrics(file)

                ApplicationManager.getApplication().invokeLater {
                    if (metrics != null) {
                        fileMetricsPanel.updateMetrics(metrics)
                    } else {
                        fileMetricsPanel.showError("Could not analyze file")
                    }
                }
            }
        })
    }

    private fun analyzeProject() {
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            projectMetricsPanel.showError("Lucent CLI not found")
            return
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Analyzing Project with Lucent", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Analyzing project..."
                indicator.isIndeterminate = true

                val projectService = LucentProjectService.getInstance(project)
                val result = projectService.analyzeProject()

                ApplicationManager.getApplication().invokeLater {
                    if (result != null) {
                        val stats = projectService.getProjectSummary()
                        if (stats != null) {
                            projectMetricsPanel.updateStats(stats)
                        }

                        // Update problem files
                        val highComplexity = projectService.getHighComplexityFiles()
                        val lowMaintainability = projectService.getLowMaintainabilityFiles()
                        problemFilesTable.updateFiles(highComplexity + lowMaintainability)
                    } else {
                        projectMetricsPanel.showError("Could not analyze project")
                    }
                }
            }
        })
    }
}

/**
 * Panel displaying metrics for a single file.
 */
class FileMetricsPanel : JBPanel<FileMetricsPanel>(GridBagLayout()) {

    private val fileNameLabel = JBLabel()
    private val complexityLabel = JBLabel()
    private val cognitiveLabel = JBLabel()
    private val nestingLabel = JBLabel()
    private val maintainabilityLabel = JBLabel()
    private val gradeLabel = JBLabel()
    private val linesLabel = JBLabel()
    private val functionsLabel = JBLabel()
    private val errorLabel = JBLabel()

    init {
        border = JBUI.Borders.empty(10)
        setupUI()
    }

    private fun setupUI() {
        val gbc = GridBagConstraints().apply {
            anchor = GridBagConstraints.WEST
            insets = JBUI.insets(4)
        }

        // File name
        gbc.gridx = 0; gbc.gridy = 0; gbc.gridwidth = 2
        add(fileNameLabel.apply { font = font.deriveFont(font.size + 2f) }, gbc)

        // Error label (hidden by default)
        gbc.gridy = 1
        errorLabel.foreground = JBColor.RED
        errorLabel.isVisible = false
        add(errorLabel, gbc)

        // Metrics grid
        gbc.gridwidth = 1
        var row = 2

        addMetricRow("Cyclomatic Complexity:", complexityLabel, gbc, row++)
        addMetricRow("Cognitive Complexity:", cognitiveLabel, gbc, row++)
        addMetricRow("Max Nesting Depth:", nestingLabel, gbc, row++)
        addMetricRow("Maintainability Index:", maintainabilityLabel, gbc, row++)
        addMetricRow("Grade:", gradeLabel, gbc, row++)
        addMetricRow("Lines of Code:", linesLabel, gbc, row++)
        addMetricRow("Functions:", functionsLabel, gbc, row++)
    }

    private fun addMetricRow(label: String, valueLabel: JBLabel, gbc: GridBagConstraints, row: Int) {
        gbc.gridx = 0; gbc.gridy = row
        add(JBLabel(label), gbc)
        gbc.gridx = 1
        add(valueLabel, gbc)
    }

    fun updateMetrics(metrics: FileMetrics) {
        errorLabel.isVisible = false

        fileNameLabel.text = metrics.path.substringAfterLast("/")

        complexityLabel.text = metrics.cyclomaticComplexity.toString()
        complexityLabel.foreground = getComplexityColor(metrics.cyclomaticComplexity)

        cognitiveLabel.text = metrics.cognitiveComplexity.toString()

        nestingLabel.text = metrics.maxNestingDepth.toString()
        nestingLabel.foreground = getNestingColor(metrics.maxNestingDepth)

        maintainabilityLabel.text = String.format("%.1f", metrics.maintainabilityIndex)
        maintainabilityLabel.foreground = getMaintainabilityColor(metrics.maintainabilityIndex)

        val grade = metrics.getMaintainabilityGrade()
        gradeLabel.text = grade
        gradeLabel.foreground = getGradeColor(grade)

        linesLabel.text = "${metrics.codeLines} (${metrics.totalLines} total)"
        functionsLabel.text = metrics.functionCount?.toString() ?: "-"
    }

    fun showError(message: String) {
        errorLabel.text = message
        errorLabel.isVisible = true
    }

    fun clear() {
        fileNameLabel.text = ""
        complexityLabel.text = "-"
        cognitiveLabel.text = "-"
        nestingLabel.text = "-"
        maintainabilityLabel.text = "-"
        gradeLabel.text = "-"
        linesLabel.text = "-"
        functionsLabel.text = "-"
        errorLabel.isVisible = false
    }

    private fun getComplexityColor(complexity: Int): Color = when {
        complexity <= 10 -> JBColor.GREEN
        complexity <= 20 -> JBColor.YELLOW
        complexity <= 30 -> JBColor.ORANGE
        else -> JBColor.RED
    }

    private fun getNestingColor(nesting: Int): Color = when {
        nesting <= 3 -> JBColor.GREEN
        nesting <= 4 -> JBColor.YELLOW
        else -> JBColor.RED
    }

    private fun getMaintainabilityColor(mi: Double): Color = when {
        mi >= 85 -> JBColor.GREEN
        mi >= 65 -> JBColor.YELLOW
        mi >= 50 -> JBColor.ORANGE
        else -> JBColor.RED
    }

    private fun getGradeColor(grade: String): Color = when (grade) {
        "A" -> JBColor.GREEN
        "B" -> JBColor.CYAN
        "C" -> JBColor.YELLOW
        "D" -> JBColor.ORANGE
        else -> JBColor.RED
    }
}

/**
 * Panel displaying project-wide metrics summary.
 */
class ProjectMetricsPanel : JBPanel<ProjectMetricsPanel>(GridBagLayout()) {

    private val totalFilesLabel = JBLabel("-")
    private val totalLinesLabel = JBLabel("-")
    private val avgComplexityLabel = JBLabel("-")
    private val avgMaintainabilityLabel = JBLabel("-")
    private val gradeLabel = JBLabel("-")
    private val issuesLabel = JBLabel("-")
    private val errorLabel = JBLabel()

    init {
        border = JBUI.Borders.empty(10)
        setupUI()
    }

    private fun setupUI() {
        val gbc = GridBagConstraints().apply {
            anchor = GridBagConstraints.WEST
            insets = JBUI.insets(4)
        }

        gbc.gridx = 0; gbc.gridy = 0; gbc.gridwidth = 2
        add(JBLabel("Project Summary").apply { font = font.deriveFont(font.size + 2f) }, gbc)

        errorLabel.foreground = JBColor.RED
        errorLabel.isVisible = false
        gbc.gridy = 1
        add(errorLabel, gbc)

        gbc.gridwidth = 1
        var row = 2

        addRow("Total Files:", totalFilesLabel, gbc, row++)
        addRow("Total Lines:", totalLinesLabel, gbc, row++)
        addRow("Avg Complexity:", avgComplexityLabel, gbc, row++)
        addRow("Avg Maintainability:", avgMaintainabilityLabel, gbc, row++)
        addRow("Overall Grade:", gradeLabel, gbc, row++)
        addRow("Issues Found:", issuesLabel, gbc, row++)
    }

    private fun addRow(label: String, valueLabel: JBLabel, gbc: GridBagConstraints, row: Int) {
        gbc.gridx = 0; gbc.gridy = row
        add(JBLabel(label), gbc)
        gbc.gridx = 1
        add(valueLabel, gbc)
    }

    fun updateStats(stats: com.lucent.plugin.services.ProjectStats) {
        errorLabel.isVisible = false

        totalFilesLabel.text = stats.totalFiles.toString()
        totalLinesLabel.text = "${stats.codeLines} code / ${stats.totalLines} total"
        avgComplexityLabel.text = String.format("%.1f", stats.avgComplexity)
        avgMaintainabilityLabel.text = String.format("%.1f", stats.avgMaintainability)

        val grade = stats.getMaintainabilityGrade()
        gradeLabel.text = grade
        gradeLabel.foreground = when (grade) {
            "A" -> JBColor.GREEN
            "B" -> JBColor.CYAN
            "C" -> JBColor.YELLOW
            "D" -> JBColor.ORANGE
            else -> JBColor.RED
        }

        val issues = stats.highComplexityCount + stats.lowMaintainabilityCount
        issuesLabel.text = "$issues (${stats.highComplexityCount} complexity, ${stats.lowMaintainabilityCount} maintainability)"
        issuesLabel.foreground = if (issues > 0) JBColor.ORANGE else JBColor.GREEN
    }

    fun showError(message: String) {
        errorLabel.text = message
        errorLabel.isVisible = true
    }

    fun clear() {
        totalFilesLabel.text = "-"
        totalLinesLabel.text = "-"
        avgComplexityLabel.text = "-"
        avgMaintainabilityLabel.text = "-"
        gradeLabel.text = "-"
        issuesLabel.text = "-"
        errorLabel.isVisible = false
    }
}

/**
 * Table showing files with metric issues.
 */
class ProblemFilesTable(private val project: Project) : JBTable() {

    private val tableModel = ProblemFilesTableModel()

    init {
        model = tableModel
        setShowGrid(false)
        rowHeight = 24

        // Custom renderer for severity colors
        setDefaultRenderer(Any::class.java, object : DefaultTableCellRenderer() {
            override fun getTableCellRendererComponent(
                table: JTable,
                value: Any?,
                isSelected: Boolean,
                hasFocus: Boolean,
                row: Int,
                column: Int
            ): Component {
                val component = super.getTableCellRendererComponent(table, value, isSelected, hasFocus, row, column)

                if (!isSelected) {
                    val metrics = tableModel.getMetricsAt(row)
                    if (metrics != null) {
                        component.foreground = when {
                            metrics.cyclomaticComplexity > 30 -> JBColor.RED
                            metrics.maintainabilityIndex < 25 -> JBColor.RED
                            metrics.cyclomaticComplexity > 20 -> JBColor.ORANGE
                            metrics.maintainabilityIndex < 50 -> JBColor.ORANGE
                            else -> JBColor.YELLOW
                        }
                    }
                }

                return component
            }
        })
    }

    fun updateFiles(files: List<FileMetrics>) {
        tableModel.setFiles(files.distinctBy { it.path })
    }

    fun clear() {
        tableModel.setFiles(emptyList())
    }
}

class ProblemFilesTableModel : AbstractTableModel() {
    private val columns = arrayOf("File", "Complexity", "MI", "Nesting", "Grade")
    private var files: List<FileMetrics> = emptyList()

    fun setFiles(newFiles: List<FileMetrics>) {
        files = newFiles
        fireTableDataChanged()
    }

    fun getMetricsAt(row: Int): FileMetrics? = files.getOrNull(row)

    override fun getRowCount(): Int = files.size
    override fun getColumnCount(): Int = columns.size
    override fun getColumnName(column: Int): String = columns[column]

    override fun getValueAt(rowIndex: Int, columnIndex: Int): Any {
        val metrics = files[rowIndex]
        return when (columnIndex) {
            0 -> metrics.path.substringAfterLast("/")
            1 -> metrics.cyclomaticComplexity
            2 -> String.format("%.0f", metrics.maintainabilityIndex)
            3 -> metrics.maxNestingDepth
            4 -> metrics.getMaintainabilityGrade()
            else -> ""
        }
    }
}
