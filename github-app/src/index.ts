import express from 'express';
import { createWebhookHandler } from './webhooks';
import { verifyWebhookSignature } from './auth';
import { setupSmeeProxy } from './dev';

const app = express();
const port = process.env.PORT || 3000;

// Parse JSON bodies
app.use(express.json());

// Health check endpoint
app.get('/health', (_req, res) => {
  res.json({ status: 'healthy', app: 'lucent-github-app' });
});

// Webhook endpoint
app.post('/webhook', verifyWebhookSignature, createWebhookHandler());

// Start server
async function start() {
  // Set up Smee proxy for local development
  if (process.env.NODE_ENV === 'development' && process.env.WEBHOOK_PROXY_URL) {
    await setupSmeeProxy(process.env.WEBHOOK_PROXY_URL, `http://localhost:${port}/webhook`);
  }

  app.listen(port, () => {
    console.log(`
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║   💎 Lucent GitHub App                                     ║
║                                                            ║
║   Server running on http://localhost:${port}                 ║
║                                                            ║
║   Endpoints:                                               ║
║   • GET  /health  - Health check                           ║
║   • POST /webhook - GitHub webhook receiver                ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
    `);
  });
}

start().catch(console.error);
