import { createInstallationOctokit } from '../auth';

/**
 * Check suite webhook payload (simplified)
 */
interface CheckSuitePayload {
  action: string;
  check_suite: {
    id: number;
    head_sha: string;
    head_branch: string;
    pull_requests: Array<{
      number: number;
      head: { sha: string };
    }>;
  };
  repository: {
    owner: { login: string };
    name: string;
    full_name: string;
  };
  installation: {
    id: number;
  };
}

/**
 * Handle check suite events
 */
export async function handleCheckSuite(payload: CheckSuitePayload) {
  const { action, check_suite: suite, repository, installation } = payload;

  // Only respond to requested or rerequested
  if (!['requested', 'rerequested'].includes(action)) {
    console.log(`⏭️  Skipping check suite action: ${action}`);
    return;
  }

  console.log(`📋 Check suite ${action} for ${repository.full_name}`);
  console.log(`   Head SHA: ${suite.head_sha}`);
  console.log(`   Branch: ${suite.head_branch}`);

  const octokit = await createInstallationOctokit(installation.id);

  // If this check suite is associated with PRs, they will trigger their own analysis
  // For direct pushes to branches, we could analyze here
  if (suite.pull_requests.length === 0) {
    // This is a direct push, not a PR
    // Create a check run for the commit
    await octokit.checks.create({
      owner: repository.owner.login,
      repo: repository.name,
      name: 'Lucent Code Metrics',
      head_sha: suite.head_sha,
      status: 'queued',
      output: {
        title: 'Queued for analysis',
        summary: 'This commit will be analyzed when a pull request is opened.',
      },
    });
  }
}
