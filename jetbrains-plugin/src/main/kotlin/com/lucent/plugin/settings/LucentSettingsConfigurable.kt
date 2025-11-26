package com.lucent.plugin.settings

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.options.Configurable
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import com.lucent.plugin.services.LucentService
import javax.swing.JComponent
import javax.swing.JPanel
import javax.swing.JSpinner
import javax.swing.SpinnerNumberModel

/**
 * Settings UI for Lucent plugin configuration.
 *
 * Accessible via Settings > Tools > Lucent
 */
class LucentSettingsConfigurable : Configurable {

    private var panel: JPanel? = null

    // UI Components
    private val lucentPathField = TextFieldWithBrowseButton()
    private val enableInspectionsCheckbox = JBCheckBox("Enable code inspections")
    private val enableGutterIconsCheckbox = JBCheckBox("Show gutter icons for metrics")
    private val analyzeOnSaveCheckbox = JBCheckBox("Analyze files on save")
    private val analyzeOnOpenCheckbox = JBCheckBox("Analyze files when opened")

    private val complexityThresholdSpinner = JSpinner(SpinnerNumberModel(20, 1, 100, 1))
    private val cognitiveThresholdSpinner = JSpinner(SpinnerNumberModel(15, 1, 100, 1))
    private val nestingThresholdSpinner = JSpinner(SpinnerNumberModel(4, 1, 20, 1))
    private val maintainabilityThresholdSpinner = JSpinner(SpinnerNumberModel(50.0, 0.0, 100.0, 5.0))
    private val functionLengthSpinner = JSpinner(SpinnerNumberModel(50, 10, 500, 10))

    private val versionLabel = JBLabel()

    override fun getDisplayName(): String = "Lucent"

    override fun createComponent(): JComponent {
        lucentPathField.addBrowseFolderListener(
            "Select Lucent Executable",
            "Select the path to the lucent CLI executable",
            null,
            FileChooserDescriptorFactory.createSingleFileDescriptor()
        )

        // Check Lucent version
        updateVersionLabel()

        panel = FormBuilder.createFormBuilder()
            // General section
            .addLabeledComponent(JBLabel("Lucent executable path:"), lucentPathField, 1, false)
            .addComponentToRightColumn(JBLabel("<html><small>Leave empty to use PATH</small></html>"))
            .addComponent(versionLabel)
            .addSeparator()

            // Feature toggles
            .addComponent(JBLabel("<html><b>Features</b></html>"))
            .addComponent(enableInspectionsCheckbox)
            .addComponent(enableGutterIconsCheckbox)
            .addComponent(analyzeOnOpenCheckbox)
            .addComponent(analyzeOnSaveCheckbox)
            .addSeparator()

            // Thresholds section
            .addComponent(JBLabel("<html><b>Metric Thresholds</b></html>"))
            .addLabeledComponent(JBLabel("Cyclomatic complexity (warning above):"), complexityThresholdSpinner)
            .addLabeledComponent(JBLabel("Cognitive complexity (warning above):"), cognitiveThresholdSpinner)
            .addLabeledComponent(JBLabel("Nesting depth (warning above):"), nestingThresholdSpinner)
            .addLabeledComponent(JBLabel("Maintainability index (warning below):"), maintainabilityThresholdSpinner)
            .addLabeledComponent(JBLabel("Function length (warning above lines):"), functionLengthSpinner)

            .addComponentFillVertically(JPanel(), 0)
            .panel

        return panel!!
    }

    private fun updateVersionLabel() {
        val service = LucentService.getInstance()
        val version = service.getVersion()

        versionLabel.text = if (version != null) {
            "<html><font color='green'>✓ Lucent $version detected</font></html>"
        } else {
            "<html><font color='red'>✗ Lucent not found. Install with: cargo install lucent-code</font></html>"
        }
    }

    override fun isModified(): Boolean {
        val settings = LucentSettings.getInstance()
        return lucentPathField.text != settings.lucentPath ||
            enableInspectionsCheckbox.isSelected != settings.enableInspections ||
            enableGutterIconsCheckbox.isSelected != settings.enableGutterIcons ||
            analyzeOnSaveCheckbox.isSelected != settings.analyzeOnSave ||
            analyzeOnOpenCheckbox.isSelected != settings.analyzeOnOpen ||
            complexityThresholdSpinner.value != settings.complexityThreshold ||
            cognitiveThresholdSpinner.value != settings.cognitiveComplexityThreshold ||
            nestingThresholdSpinner.value != settings.nestingThreshold ||
            maintainabilityThresholdSpinner.value != settings.maintainabilityThreshold ||
            functionLengthSpinner.value != settings.maxFunctionLength
    }

    override fun apply() {
        val settings = LucentSettings.getInstance()

        settings.lucentPath = lucentPathField.text
        settings.enableInspections = enableInspectionsCheckbox.isSelected
        settings.enableGutterIcons = enableGutterIconsCheckbox.isSelected
        settings.analyzeOnSave = analyzeOnSaveCheckbox.isSelected
        settings.analyzeOnOpen = analyzeOnOpenCheckbox.isSelected
        settings.complexityThreshold = complexityThresholdSpinner.value as Int
        settings.cognitiveComplexityThreshold = cognitiveThresholdSpinner.value as Int
        settings.nestingThreshold = nestingThresholdSpinner.value as Int
        settings.maintainabilityThreshold = maintainabilityThresholdSpinner.value as Double
        settings.maxFunctionLength = functionLengthSpinner.value as Int

        // Update version label after path change
        updateVersionLabel()
    }

    override fun reset() {
        val settings = LucentSettings.getInstance()

        lucentPathField.text = settings.lucentPath
        enableInspectionsCheckbox.isSelected = settings.enableInspections
        enableGutterIconsCheckbox.isSelected = settings.enableGutterIcons
        analyzeOnSaveCheckbox.isSelected = settings.analyzeOnSave
        analyzeOnOpenCheckbox.isSelected = settings.analyzeOnOpen
        complexityThresholdSpinner.value = settings.complexityThreshold
        cognitiveThresholdSpinner.value = settings.cognitiveComplexityThreshold
        nestingThresholdSpinner.value = settings.nestingThreshold
        maintainabilityThresholdSpinner.value = settings.maintainabilityThreshold
        functionLengthSpinner.value = settings.maxFunctionLength

        updateVersionLabel()
    }

    override fun disposeUIResources() {
        panel = null
    }
}
