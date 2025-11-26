import { spawn } from 'child_process';
import { writeFileSync, mkdirSync, rmSync, existsSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

/**
 * File metrics from Lucent analysis
 */
export interface FileMetric {
  path: string;
  language: string;
  codeLines: number;
  commentLines: number;
  blankLines: number;
  totalLines: number;
  cyclomaticComplexity: number;
  cognitiveComplexity: number;
  maxNestingDepth: number;
  maintainabilityIndex: number;
  maintainabilityGrade: string;
  functionCount?: number;
  codeSmells: CodeSmell[];
}

/**
 * Code smell detected by Lucent
 */
export interface CodeSmell {
  name: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  line?: number;
}

/**
 * Summary metrics across all files
 */
export interface MetricsSummary {
  totalFiles: number;
  totalLines: number;
  totalCodeLines: number;
  avgCyclomaticComplexity: number;
  avgCognitiveComplexity: number;
  avgMaintainability: number;
  highComplexityFiles: number;
  lowMaintainabilityFiles: number;
  criticalIssues: number;
  warnings: number;
}

/**
 * Complete Lucent analysis result
 */
export interface LucentMetrics {
  files: FileMetric[];
  summary: MetricsSummary;
}

/**
 * Analyze files using Lucent CLI
 */
export async function analyzeFiles(
  files: { path: string; content: string }[]
): Promise<LucentMetrics> {
  // Create a temporary directory for the files
  const tempDir = join(tmpdir(), `lucent-analysis-${Date.now()}`);
  mkdirSync(tempDir, { recursive: true });

  try {
    // Write files to temp directory
    for (const file of files) {
      const filePath = join(tempDir, file.path);
      const fileDir = join(filePath, '..');
      mkdirSync(fileDir, { recursive: true });
      writeFileSync(filePath, file.content);
    }

    // Run Lucent
    const result = await runLucent(tempDir);
    return result;

  } finally {
    // Clean up temp directory
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  }
}

/**
 * Run Lucent CLI on a directory
 */
async function runLucent(directory: string): Promise<LucentMetrics> {
  return new Promise((resolve, reject) => {
    const lucentPath = process.env.LUCENT_PATH || 'lucent';
    const args = ['analyze', directory, '--format', 'json'];

    const child = spawn(lucentPath, args);

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    child.on('close', (code) => {
      if (code !== 0) {
        // Lucent may exit non-zero for threshold violations
        // Still try to parse the output
        if (!stdout) {
          reject(new Error(`Lucent exited with code ${code}: ${stderr}`));
          return;
        }
      }

      try {
        const parsed = JSON.parse(stdout);
        const metrics = transformLucentOutput(parsed);
        resolve(metrics);
      } catch (err) {
        reject(new Error(`Failed to parse Lucent output: ${err}`));
      }
    });

    child.on('error', (err) => {
      reject(new Error(`Failed to run Lucent: ${err.message}`));
    });
  });
}

/**
 * Transform Lucent JSON output to our metrics format
 */
function transformLucentOutput(output: any): LucentMetrics {
  const files: FileMetric[] = [];

  // Handle the Lucent output format
  const fileMetrics = output.files || output.metrics || [];

  for (const file of fileMetrics) {
    files.push({
      path: file.path || file.file,
      language: file.language || 'unknown',
      codeLines: file.code_lines || file.codeLines || 0,
      commentLines: file.comment_lines || file.commentLines || 0,
      blankLines: file.blank_lines || file.blankLines || 0,
      totalLines: file.total_lines || file.totalLines || 0,
      cyclomaticComplexity: file.cyclomatic_complexity || file.cyclomaticComplexity || 0,
      cognitiveComplexity: file.cognitive_complexity || file.cognitiveComplexity || 0,
      maxNestingDepth: file.max_nesting_depth || file.nestingDepth || 0,
      maintainabilityIndex: file.maintainability_index || file.maintainabilityIndex || 100,
      maintainabilityGrade: file.maintainability_grade || getMaintainabilityGrade(file.maintainability_index || 100),
      functionCount: file.function_count || file.functionCount,
      codeSmells: transformCodeSmells(file.code_smells || file.codeSmells || []),
    });
  }

  // Calculate summary
  const summary = calculateSummary(files);

  return { files, summary };
}

/**
 * Transform code smells from Lucent format
 */
function transformCodeSmells(smells: any[]): CodeSmell[] {
  return smells.map(smell => ({
    name: smell.name || smell.type || 'Unknown',
    severity: normalizeSeverity(smell.severity),
    description: smell.description || smell.message || '',
    line: smell.line,
  }));
}

/**
 * Normalize severity string
 */
function normalizeSeverity(severity: string): CodeSmell['severity'] {
  const s = (severity || '').toLowerCase();
  if (s === 'critical' || s === 'error') return 'critical';
  if (s === 'high' || s === 'warning') return 'high';
  if (s === 'medium' || s === 'moderate') return 'medium';
  return 'low';
}

/**
 * Get maintainability grade from index
 */
function getMaintainabilityGrade(index: number): string {
  if (index >= 85) return 'A';
  if (index >= 65) return 'B';
  if (index >= 50) return 'C';
  if (index >= 25) return 'D';
  return 'F';
}

/**
 * Calculate summary statistics
 */
function calculateSummary(files: FileMetric[]): MetricsSummary {
  if (files.length === 0) {
    return {
      totalFiles: 0,
      totalLines: 0,
      totalCodeLines: 0,
      avgCyclomaticComplexity: 0,
      avgCognitiveComplexity: 0,
      avgMaintainability: 100,
      highComplexityFiles: 0,
      lowMaintainabilityFiles: 0,
      criticalIssues: 0,
      warnings: 0,
    };
  }

  const totalFiles = files.length;
  const totalLines = files.reduce((sum, f) => sum + f.totalLines, 0);
  const totalCodeLines = files.reduce((sum, f) => sum + f.codeLines, 0);

  const avgCyclomaticComplexity = files.reduce((sum, f) => sum + f.cyclomaticComplexity, 0) / totalFiles;
  const avgCognitiveComplexity = files.reduce((sum, f) => sum + f.cognitiveComplexity, 0) / totalFiles;
  const avgMaintainability = files.reduce((sum, f) => sum + f.maintainabilityIndex, 0) / totalFiles;

  const highComplexityFiles = files.filter(f => f.cyclomaticComplexity > 20).length;
  const lowMaintainabilityFiles = files.filter(f => f.maintainabilityIndex < 50).length;

  let criticalIssues = 0;
  let warnings = 0;

  for (const file of files) {
    for (const smell of file.codeSmells) {
      if (smell.severity === 'critical') criticalIssues++;
      else if (smell.severity === 'high') warnings++;
    }
  }

  return {
    totalFiles,
    totalLines,
    totalCodeLines,
    avgCyclomaticComplexity,
    avgCognitiveComplexity,
    avgMaintainability,
    highComplexityFiles,
    lowMaintainabilityFiles,
    criticalIssues,
    warnings,
  };
}

/**
 * Analyze a single file content directly (without CLI)
 * Used as fallback if Lucent CLI is not available
 */
export function analyzeFileContent(content: string, filename: string): FileMetric {
  const lines = content.split('\n');
  const totalLines = lines.length;

  // Basic analysis without tree-sitter
  let codeLines = 0;
  let commentLines = 0;
  let blankLines = 0;
  let maxNesting = 0;
  let currentNesting = 0;

  const openBraces = /[{(\[]/g;
  const closeBraces = /[})\]]/g;

  for (const line of lines) {
    const trimmed = line.trim();

    if (trimmed === '') {
      blankLines++;
    } else if (trimmed.startsWith('//') || trimmed.startsWith('#') || trimmed.startsWith('*')) {
      commentLines++;
    } else {
      codeLines++;

      // Simple nesting detection
      const opens = (line.match(openBraces) || []).length;
      const closes = (line.match(closeBraces) || []).length;
      currentNesting += opens - closes;
      maxNesting = Math.max(maxNesting, currentNesting);
    }
  }

  // Estimate complexity based on code patterns
  const ifMatches = content.match(/\b(if|else|elif|elsif|case|when)\b/g) || [];
  const loopMatches = content.match(/\b(for|while|loop|each|do)\b/g) || [];
  const logicMatches = content.match(/(\&\&|\|\||\?)/g) || [];

  const cyclomaticComplexity = 1 + ifMatches.length + loopMatches.length + logicMatches.length;

  // Cognitive complexity (simplified)
  const cognitiveComplexity = Math.round(cyclomaticComplexity * 0.8 + maxNesting * 2);

  // Maintainability Index (simplified formula)
  const halsteadVolume = codeLines * Math.log2(codeLines + 1);
  const mi = Math.max(0, Math.min(100,
    171 - 5.2 * Math.log(halsteadVolume + 1) - 0.23 * cyclomaticComplexity - 16.2 * Math.log(codeLines + 1)
  ));

  return {
    path: filename,
    language: getLanguageFromFilename(filename),
    codeLines,
    commentLines,
    blankLines,
    totalLines,
    cyclomaticComplexity,
    cognitiveComplexity,
    maxNestingDepth: maxNesting,
    maintainabilityIndex: Math.round(mi * 10) / 10,
    maintainabilityGrade: getMaintainabilityGrade(mi),
    codeSmells: [],
  };
}

/**
 * Get language from filename
 */
function getLanguageFromFilename(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || '';
  const langMap: Record<string, string> = {
    rs: 'Rust',
    py: 'Python',
    js: 'JavaScript',
    ts: 'TypeScript',
    tsx: 'TypeScript',
    jsx: 'JavaScript',
    go: 'Go',
    java: 'Java',
    c: 'C',
    cpp: 'C++',
    h: 'C',
    hpp: 'C++',
    rb: 'Ruby',
    php: 'PHP',
    swift: 'Swift',
    kt: 'Kotlin',
    scala: 'Scala',
    cs: 'C#',
    lua: 'Lua',
    zig: 'Zig',
    nim: 'Nim',
  };
  return langMap[ext] || 'Unknown';
}
