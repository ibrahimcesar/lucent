import { Request, Response } from 'express';
import { handlePullRequest } from './handlers/pullRequest';
import { handleCheckSuite } from './handlers/checkSuite';

/**
 * GitHub webhook event types we handle
 */
type WebhookEvent =
  | 'pull_request'
  | 'check_suite'
  | 'check_run'
  | 'push'
  | 'installation'
  | 'ping';

/**
 * Create the main webhook handler
 */
export function createWebhookHandler() {
  return async (req: Request, res: Response) => {
    const event = req.headers['x-github-event'] as WebhookEvent;
    const deliveryId = req.headers['x-github-delivery'] as string;
    const payload = req.body;

    console.log(`📥 Received webhook: ${event} (${deliveryId})`);

    try {
      switch (event) {
        case 'ping':
          console.log('🏓 Ping received from GitHub');
          res.json({ message: 'pong' });
          break;

        case 'installation':
          await handleInstallation(payload);
          res.json({ message: 'Installation event processed' });
          break;

        case 'pull_request':
          await handlePullRequest(payload);
          res.json({ message: 'Pull request event processed' });
          break;

        case 'check_suite':
          await handleCheckSuite(payload);
          res.json({ message: 'Check suite event processed' });
          break;

        case 'check_run':
          // Re-run requests
          if (payload.action === 'rerequested') {
            await handleCheckRerun(payload);
          }
          res.json({ message: 'Check run event processed' });
          break;

        case 'push':
          // Could analyze on push to default branch
          console.log(`📤 Push to ${payload.ref}`);
          res.json({ message: 'Push event received' });
          break;

        default:
          console.log(`⏭️  Ignoring event: ${event}`);
          res.json({ message: `Event ${event} not handled` });
      }
    } catch (error) {
      console.error(`❌ Error handling ${event}:`, error);
      res.status(500).json({
        error: 'Internal server error',
        message: error instanceof Error ? error.message : 'Unknown error'
      });
    }
  };
}

/**
 * Handle app installation events
 */
async function handleInstallation(payload: any) {
  const action = payload.action;
  const installation = payload.installation;
  const sender = payload.sender.login;

  switch (action) {
    case 'created':
      console.log(`✅ App installed by ${sender} on ${installation.account.login}`);
      break;
    case 'deleted':
      console.log(`🗑️  App uninstalled from ${installation.account.login}`);
      break;
    case 'suspend':
      console.log(`⏸️  App suspended on ${installation.account.login}`);
      break;
    case 'unsuspend':
      console.log(`▶️  App unsuspended on ${installation.account.login}`);
      break;
  }
}

/**
 * Handle check run re-run requests
 */
async function handleCheckRerun(payload: any) {
  console.log(`🔄 Check run re-requested for ${payload.repository.full_name}`);
  // Re-run the analysis for the associated PR/commit
}
