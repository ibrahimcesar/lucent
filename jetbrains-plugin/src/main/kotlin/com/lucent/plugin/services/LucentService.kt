package com.lucent.plugin.services

import com.google.gson.Gson
import com.google.gson.annotations.SerializedName
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.Logger
import com.lucent.plugin.settings.LucentSettings
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.util.concurrent.TimeUnit

/**
 * Application-level service for interacting with the Lucent CLI.
 *
 * This service provides methods to:
 * - Check if Lucent is installed
 * - Analyze files and directories
 * - Parse Lucent JSON output
 */
@Service(Service.Level.APP)
class LucentService {
    private val logger = Logger.getInstance(LucentService::class.java)
    private val gson = Gson()

    companion object {
        fun getInstance(): LucentService =
            ApplicationManager.getApplication().getService(LucentService::class.java)
    }

    /**
     * Check if Lucent CLI is available in PATH
     */
    fun isLucentInstalled(): Boolean {
        return try {
            val settings = LucentSettings.getInstance()
            val lucentPath = settings.lucentPath.ifEmpty { "lucent" }

            val process = ProcessBuilder(lucentPath, "--version")
                .redirectErrorStream(true)
                .start()

            val result = process.waitFor(5, TimeUnit.SECONDS)
            result && process.exitValue() == 0
        } catch (e: Exception) {
            logger.warn("Lucent CLI not found: ${e.message}")
            false
        }
    }

    /**
     * Get Lucent version
     */
    fun getVersion(): String? {
        return try {
            val settings = LucentSettings.getInstance()
            val lucentPath = settings.lucentPath.ifEmpty { "lucent" }

            val process = ProcessBuilder(lucentPath, "--version")
                .redirectErrorStream(true)
                .start()

            val reader = BufferedReader(InputStreamReader(process.inputStream))
            val output = reader.readText().trim()
            process.waitFor(5, TimeUnit.SECONDS)

            // Parse version from output like "lucent 0.0.1"
            output.substringAfter("lucent ").trim().ifEmpty { null }
        } catch (e: Exception) {
            logger.warn("Failed to get Lucent version: ${e.message}")
            null
        }
    }

    /**
     * Analyze a single file and return metrics
     */
    fun analyzeFile(filePath: String): FileMetrics? {
        return try {
            val settings = LucentSettings.getInstance()
            val lucentPath = settings.lucentPath.ifEmpty { "lucent" }

            val process = ProcessBuilder(lucentPath, filePath, "--format", "json", "--per-file")
                .redirectErrorStream(false)
                .start()

            val reader = BufferedReader(InputStreamReader(process.inputStream))
            val output = reader.readText()
            val completed = process.waitFor(30, TimeUnit.SECONDS)

            if (!completed || process.exitValue() != 0) {
                val errorReader = BufferedReader(InputStreamReader(process.errorStream))
                val error = errorReader.readText()
                logger.warn("Lucent analysis failed: $error")
                return null
            }

            // Parse JSON output
            val result = gson.fromJson(output, LucentOutput::class.java)
            result?.files?.firstOrNull()
        } catch (e: Exception) {
            logger.error("Failed to analyze file: ${e.message}", e)
            null
        }
    }

    /**
     * Analyze a directory and return project metrics
     */
    fun analyzeDirectory(dirPath: String): LucentOutput? {
        return try {
            val settings = LucentSettings.getInstance()
            val lucentPath = settings.lucentPath.ifEmpty { "lucent" }

            val process = ProcessBuilder(lucentPath, dirPath, "--format", "json", "--per-file")
                .directory(File(dirPath))
                .redirectErrorStream(false)
                .start()

            val reader = BufferedReader(InputStreamReader(process.inputStream))
            val output = reader.readText()
            val completed = process.waitFor(120, TimeUnit.SECONDS)

            if (!completed || process.exitValue() != 0) {
                val errorReader = BufferedReader(InputStreamReader(process.errorStream))
                val error = errorReader.readText()
                logger.warn("Lucent analysis failed: $error")
                return null
            }

            gson.fromJson(output, LucentOutput::class.java)
        } catch (e: Exception) {
            logger.error("Failed to analyze directory: ${e.message}", e)
            null
        }
    }

    /**
     * Show notification to user
     */
    fun notify(title: String, content: String, type: NotificationType) {
        NotificationGroupManager.getInstance()
            .getNotificationGroup("Lucent Notifications")
            .createNotification(title, content, type)
            .notify(null)
    }
}

// Data classes for Lucent JSON output

data class LucentOutput(
    val summary: ProjectSummary?,
    val files: List<FileMetrics>?,
    @SerializedName("by_language")
    val byLanguage: Map<String, LanguageMetrics>?
)

data class ProjectSummary(
    @SerializedName("total_files")
    val totalFiles: Int,
    @SerializedName("total_lines")
    val totalLines: Int,
    @SerializedName("code_lines")
    val codeLines: Int,
    @SerializedName("comment_lines")
    val commentLines: Int,
    @SerializedName("blank_lines")
    val blankLines: Int,
    @SerializedName("avg_complexity")
    val avgComplexity: Double,
    @SerializedName("avg_maintainability")
    val avgMaintainability: Double
)

data class LanguageMetrics(
    @SerializedName("file_count")
    val fileCount: Int,
    @SerializedName("total_lines")
    val totalLines: Int,
    @SerializedName("code_lines")
    val codeLines: Int,
    @SerializedName("avg_cyclomatic")
    val avgCyclomatic: Double,
    @SerializedName("avg_cognitive")
    val avgCognitive: Double,
    @SerializedName("avg_maintainability_index")
    val avgMaintainabilityIndex: Double
)

data class FileMetrics(
    val path: String,
    val language: String?,

    @SerializedName("total_lines")
    val totalLines: Int,

    @SerializedName("code_lines")
    val codeLines: Int,

    @SerializedName("comment_lines")
    val commentLines: Int,

    @SerializedName("blank_lines")
    val blankLines: Int,

    @SerializedName("cyclomatic_complexity")
    val cyclomaticComplexity: Int,

    @SerializedName("cognitive_complexity")
    val cognitiveComplexity: Int,

    @SerializedName("max_nesting_depth")
    val maxNestingDepth: Int,

    @SerializedName("maintainability_index")
    val maintainabilityIndex: Double,

    val entropy: Double?,

    @SerializedName("function_count")
    val functionCount: Int?,

    @SerializedName("avg_function_length")
    val avgFunctionLength: Double?,

    @SerializedName("todo_count")
    val todoCount: Int?,

    @SerializedName("fixme_count")
    val fixmeCount: Int?,

    @SerializedName("hack_count")
    val hackCount: Int?,

    @SerializedName("code_smells")
    val codeSmells: List<CodeSmell>?
) {
    /**
     * Get maintainability grade (A-F)
     */
    fun getMaintainabilityGrade(): String {
        return when {
            maintainabilityIndex >= 85 -> "A"
            maintainabilityIndex >= 65 -> "B"
            maintainabilityIndex >= 50 -> "C"
            maintainabilityIndex >= 25 -> "D"
            else -> "F"
        }
    }

    /**
     * Get complexity severity
     */
    fun getComplexitySeverity(): Severity {
        return when {
            cyclomaticComplexity <= 10 -> Severity.LOW
            cyclomaticComplexity <= 20 -> Severity.MEDIUM
            cyclomaticComplexity <= 30 -> Severity.HIGH
            else -> Severity.CRITICAL
        }
    }

    /**
     * Get nesting severity
     */
    fun getNestingSeverity(): Severity {
        return when {
            maxNestingDepth <= 3 -> Severity.LOW
            maxNestingDepth <= 4 -> Severity.MEDIUM
            maxNestingDepth <= 5 -> Severity.HIGH
            else -> Severity.CRITICAL
        }
    }
}

data class CodeSmell(
    @SerializedName("smell_type")
    val smellType: String,
    val description: String,
    val severity: String,
    val line: Int?,
    val suggestion: String?
)

enum class Severity {
    LOW, MEDIUM, HIGH, CRITICAL
}
