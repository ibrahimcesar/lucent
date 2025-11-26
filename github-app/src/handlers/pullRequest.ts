import { createInstallationOctokit } from '../auth';
import { analyzeFiles, LucentMetrics } from '../lucent';
import { formatMetricsComment, formatCheckRunOutput } from '../formatter';

/**
 * Pull request webhook payload (simplified)
 */
interface PullRequestPayload {
  action: string;
  number: number;
  pull_request: {
    number: number;
    head: {
      sha: string;
      ref: string;
    };
    base: {
      sha: string;
      ref: string;
    };
    title: string;
    changed_files: number;
  };
  repository: {
    owner: { login: string };
    name: string;
    full_name: string;
    clone_url: string;
    default_branch: string;
  };
  installation: {
    id: number;
  };
}

/**
 * Handle pull request events
 */
export async function handlePullRequest(payload: PullRequestPayload) {
  const { action, pull_request: pr, repository, installation } = payload;

  // Only analyze on open, synchronize (new commits), or reopened
  if (!['opened', 'synchronize', 'reopened'].includes(action)) {
    console.log(`⏭️  Skipping PR action: ${action}`);
    return;
  }

  console.log(`🔍 Analyzing PR #${pr.number}: ${pr.title}`);
  console.log(`   Repository: ${repository.full_name}`);
  console.log(`   Head SHA: ${pr.head.sha}`);
  console.log(`   Changed files: ${pr.changed_files}`);

  const octokit = await createInstallationOctokit(installation.id);

  // Create a check run to show we're analyzing
  const checkRun = await octokit.checks.create({
    owner: repository.owner.login,
    repo: repository.name,
    name: 'Lucent Code Metrics',
    head_sha: pr.head.sha,
    status: 'in_progress',
    started_at: new Date().toISOString(),
    output: {
      title: 'Analyzing code metrics...',
      summary: 'Lucent is analyzing the changed files in this pull request.',
    },
  });

  try {
    // Get the list of changed files
    const { data: files } = await octokit.pulls.listFiles({
      owner: repository.owner.login,
      repo: repository.name,
      pull_number: pr.number,
    });

    // Filter to supported file types
    const supportedExtensions = [
      '.rs', '.py', '.js', '.ts', '.tsx', '.jsx',
      '.go', '.java', '.c', '.cpp', '.h', '.hpp',
      '.rb', '.php', '.swift', '.kt', '.scala',
      '.cs', '.lua', '.zig', '.nim'
    ];

    const analyzableFiles = files.filter(f =>
      f.status !== 'removed' &&
      supportedExtensions.some(ext => f.filename.endsWith(ext))
    );

    if (analyzableFiles.length === 0) {
      await octokit.checks.update({
        owner: repository.owner.login,
        repo: repository.name,
        check_run_id: checkRun.data.id,
        status: 'completed',
        conclusion: 'neutral',
        completed_at: new Date().toISOString(),
        output: {
          title: 'No analyzable files',
          summary: 'No supported source files were modified in this pull request.',
        },
      });
      return;
    }

    // Fetch file contents and analyze
    const fileContents: { path: string; content: string }[] = [];

    for (const file of analyzableFiles) {
      try {
        const { data } = await octokit.repos.getContent({
          owner: repository.owner.login,
          repo: repository.name,
          path: file.filename,
          ref: pr.head.sha,
        });

        if ('content' in data && data.type === 'file') {
          fileContents.push({
            path: file.filename,
            content: Buffer.from(data.content, 'base64').toString('utf8'),
          });
        }
      } catch (err) {
        console.warn(`⚠️  Could not fetch ${file.filename}:`, err);
      }
    }

    // Run Lucent analysis
    const metrics = await analyzeFiles(fileContents);

    // Format the results
    const checkOutput = formatCheckRunOutput(metrics, analyzableFiles.length);

    // Determine conclusion based on metrics
    const conclusion = determineConclusion(metrics);

    // Update check run with results
    await octokit.checks.update({
      owner: repository.owner.login,
      repo: repository.name,
      check_run_id: checkRun.data.id,
      status: 'completed',
      conclusion,
      completed_at: new Date().toISOString(),
      output: checkOutput,
    });

    // Optionally post a PR comment with detailed metrics
    if (shouldPostComment(metrics)) {
      const comment = formatMetricsComment(metrics, pr.head.sha);
      await octokit.issues.createComment({
        owner: repository.owner.login,
        repo: repository.name,
        issue_number: pr.number,
        body: comment,
      });
    }

    console.log(`✅ Analysis complete for PR #${pr.number}`);

  } catch (error) {
    console.error(`❌ Analysis failed:`, error);

    // Update check run with failure
    await octokit.checks.update({
      owner: repository.owner.login,
      repo: repository.name,
      check_run_id: checkRun.data.id,
      status: 'completed',
      conclusion: 'failure',
      completed_at: new Date().toISOString(),
      output: {
        title: 'Analysis failed',
        summary: `Error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      },
    });
  }
}

/**
 * Determine check conclusion based on metrics
 */
function determineConclusion(metrics: LucentMetrics): 'success' | 'failure' | 'neutral' | 'action_required' {
  // Critical issues = failure
  if (metrics.summary.criticalIssues > 0) {
    return 'failure';
  }

  // High complexity files = action required
  if (metrics.summary.highComplexityFiles > 0) {
    return 'action_required';
  }

  // Low maintainability = action required
  if (metrics.summary.avgMaintainability < 50) {
    return 'action_required';
  }

  // Some warnings = neutral (pass but note issues)
  if (metrics.summary.warnings > 0) {
    return 'neutral';
  }

  return 'success';
}

/**
 * Determine if we should post a detailed comment
 * (only for significant issues to avoid noise)
 */
function shouldPostComment(metrics: LucentMetrics): boolean {
  return (
    metrics.summary.criticalIssues > 0 ||
    metrics.summary.highComplexityFiles > 0 ||
    metrics.summary.avgMaintainability < 50
  );
}
