import { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';
import jwt from 'jsonwebtoken';
import fs from 'fs';
import { Octokit } from '@octokit/rest';

/**
 * Verify GitHub webhook signature
 */
export function verifyWebhookSignature(req: Request, res: Response, next: NextFunction) {
  const signature = req.headers['x-hub-signature-256'] as string;
  const secret = process.env.GITHUB_WEBHOOK_SECRET;

  if (!secret) {
    console.warn('⚠️  GITHUB_WEBHOOK_SECRET not set, skipping signature verification');
    return next();
  }

  if (!signature) {
    return res.status(401).json({ error: 'Missing signature header' });
  }

  const body = JSON.stringify(req.body);
  const expectedSignature = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');

  if (!crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(expectedSignature))) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  next();
}

/**
 * Get the private key for JWT signing
 */
function getPrivateKey(): string {
  // Try environment variable first
  if (process.env.GITHUB_PRIVATE_KEY) {
    return process.env.GITHUB_PRIVATE_KEY.replace(/\\n/g, '\n');
  }

  // Try file path
  const keyPath = process.env.GITHUB_PRIVATE_KEY_PATH || './private-key.pem';
  if (fs.existsSync(keyPath)) {
    return fs.readFileSync(keyPath, 'utf8');
  }

  throw new Error('No private key found. Set GITHUB_PRIVATE_KEY or GITHUB_PRIVATE_KEY_PATH');
}

/**
 * Generate a JWT for GitHub App authentication
 */
export function generateAppJwt(): string {
  const appId = process.env.GITHUB_APP_ID;
  if (!appId) {
    throw new Error('GITHUB_APP_ID environment variable is required');
  }

  const privateKey = getPrivateKey();
  const now = Math.floor(Date.now() / 1000);

  return jwt.sign(
    {
      iat: now - 60, // Issued 60 seconds ago (clock drift)
      exp: now + 600, // Expires in 10 minutes
      iss: appId,
    },
    privateKey,
    { algorithm: 'RS256' }
  );
}

/**
 * Get an installation access token for a specific installation
 */
export async function getInstallationToken(installationId: number): Promise<string> {
  const appJwt = generateAppJwt();

  const octokit = new Octokit({
    auth: appJwt,
  });

  const response = await octokit.apps.createInstallationAccessToken({
    installation_id: installationId,
  });

  return response.data.token;
}

/**
 * Create an authenticated Octokit instance for an installation
 */
export async function createInstallationOctokit(installationId: number): Promise<Octokit> {
  const token = await getInstallationToken(installationId);
  return new Octokit({ auth: token });
}
