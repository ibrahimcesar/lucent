package com.lucent.plugin.settings

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage
import com.intellij.util.xmlb.XmlSerializerUtil

/**
 * Persistent settings for the Lucent plugin.
 *
 * Stores user preferences for:
 * - Lucent CLI path
 * - Metric thresholds
 * - Feature toggles
 */
@State(
    name = "LucentSettings",
    storages = [Storage("LucentSettings.xml")]
)
@Service(Service.Level.APP)
class LucentSettings : PersistentStateComponent<LucentSettings> {

    // Lucent CLI path (empty = use PATH)
    var lucentPath: String = ""

    // Enable/disable features
    var enableInspections: Boolean = true
    var enableGutterIcons: Boolean = true
    var analyzeOnSave: Boolean = false
    var analyzeOnOpen: Boolean = true

    // Metric thresholds
    var complexityThreshold: Int = 20
    var cognitiveComplexityThreshold: Int = 15
    var nestingThreshold: Int = 4
    var maintainabilityThreshold: Double = 50.0
    var maxFunctionLength: Int = 50

    companion object {
        fun getInstance(): LucentSettings =
            ApplicationManager.getApplication().getService(LucentSettings::class.java)
    }

    override fun getState(): LucentSettings = this

    override fun loadState(state: LucentSettings) {
        XmlSerializerUtil.copyBean(state, this)
    }
}
