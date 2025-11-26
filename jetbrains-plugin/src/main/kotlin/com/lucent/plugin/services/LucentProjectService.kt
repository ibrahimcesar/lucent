package com.lucent.plugin.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import java.util.concurrent.ConcurrentHashMap

/**
 * Project-level service for caching and managing Lucent metrics.
 *
 * This service maintains a cache of file metrics for the current project
 * and provides methods to analyze and retrieve metrics.
 */
@Service(Service.Level.PROJECT)
class LucentProjectService(private val project: Project) {
    private val logger = Logger.getInstance(LucentProjectService::class.java)

    // Cache of file metrics by file path
    private val metricsCache = ConcurrentHashMap<String, FileMetrics>()

    // Last full project analysis result
    private var projectMetrics: LucentOutput? = null

    // Timestamp of last analysis
    private var lastAnalysisTime: Long = 0

    companion object {
        fun getInstance(project: Project): LucentProjectService =
            project.getService(LucentProjectService::class.java)
    }

    /**
     * Get cached metrics for a file, or analyze if not cached
     */
    fun getMetrics(file: VirtualFile): FileMetrics? {
        val path = file.path
        return metricsCache[path] ?: analyzeAndCache(file)
    }

    /**
     * Force re-analyze a file
     */
    fun refreshMetrics(file: VirtualFile): FileMetrics? {
        return analyzeAndCache(file)
    }

    /**
     * Analyze file and cache the result
     */
    private fun analyzeAndCache(file: VirtualFile): FileMetrics? {
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            logger.warn("Lucent CLI is not installed")
            return null
        }

        val metrics = lucentService.analyzeFile(file.path)
        if (metrics != null) {
            metricsCache[file.path] = metrics
        }
        return metrics
    }

    /**
     * Analyze entire project
     */
    fun analyzeProject(): LucentOutput? {
        val lucentService = LucentService.getInstance()

        if (!lucentService.isLucentInstalled()) {
            logger.warn("Lucent CLI is not installed")
            return null
        }

        val basePath = project.basePath ?: return null
        val result = lucentService.analyzeDirectory(basePath)

        if (result != null) {
            projectMetrics = result
            lastAnalysisTime = System.currentTimeMillis()

            // Update cache with all file metrics
            result.files?.forEach { fileMetrics ->
                metricsCache[fileMetrics.path] = fileMetrics
            }
        }

        return result
    }

    /**
     * Get cached project metrics
     */
    fun getProjectMetrics(): LucentOutput? = projectMetrics

    /**
     * Get all cached file metrics
     */
    fun getAllCachedMetrics(): Map<String, FileMetrics> = metricsCache.toMap()

    /**
     * Clear the metrics cache
     */
    fun clearCache() {
        metricsCache.clear()
        projectMetrics = null
        lastAnalysisTime = 0
    }

    /**
     * Invalidate cached metrics for a specific file
     */
    fun invalidateFile(path: String) {
        metricsCache.remove(path)
    }

    /**
     * Get files with high complexity (above threshold)
     */
    fun getHighComplexityFiles(threshold: Int = 20): List<FileMetrics> {
        return metricsCache.values.filter { it.cyclomaticComplexity > threshold }
            .sortedByDescending { it.cyclomaticComplexity }
    }

    /**
     * Get files with low maintainability (below threshold)
     */
    fun getLowMaintainabilityFiles(threshold: Double = 50.0): List<FileMetrics> {
        return metricsCache.values.filter { it.maintainabilityIndex < threshold }
            .sortedBy { it.maintainabilityIndex }
    }

    /**
     * Get files with deep nesting (above threshold)
     */
    fun getDeepNestingFiles(threshold: Int = 4): List<FileMetrics> {
        return metricsCache.values.filter { it.maxNestingDepth > threshold }
            .sortedByDescending { it.maxNestingDepth }
    }

    /**
     * Get project statistics summary
     */
    fun getProjectSummary(): ProjectStats? {
        val files = metricsCache.values.toList()
        if (files.isEmpty()) return null

        return ProjectStats(
            totalFiles = files.size,
            totalLines = files.sumOf { it.totalLines },
            codeLines = files.sumOf { it.codeLines },
            avgComplexity = files.map { it.cyclomaticComplexity.toDouble() }.average(),
            avgMaintainability = files.map { it.maintainabilityIndex }.average(),
            avgNesting = files.map { it.maxNestingDepth.toDouble() }.average(),
            highComplexityCount = files.count { it.cyclomaticComplexity > 20 },
            lowMaintainabilityCount = files.count { it.maintainabilityIndex < 50 }
        )
    }
}

data class ProjectStats(
    val totalFiles: Int,
    val totalLines: Int,
    val codeLines: Int,
    val avgComplexity: Double,
    val avgMaintainability: Double,
    val avgNesting: Double,
    val highComplexityCount: Int,
    val lowMaintainabilityCount: Int
) {
    fun getMaintainabilityGrade(): String {
        return when {
            avgMaintainability >= 85 -> "A"
            avgMaintainability >= 65 -> "B"
            avgMaintainability >= 50 -> "C"
            avgMaintainability >= 25 -> "D"
            else -> "F"
        }
    }
}
