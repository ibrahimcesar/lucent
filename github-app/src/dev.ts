/**
 * Development utilities for local testing
 */

/**
 * Set up Smee.io webhook proxy for local development
 * This allows you to receive GitHub webhooks on localhost
 */
export async function setupSmeeProxy(sourceUrl: string, targetUrl: string): Promise<void> {
  try {
    // Dynamic import to avoid bundling in production
    const SmeeClient = (await import('smee-client')).default;

    const smee = new SmeeClient({
      source: sourceUrl,
      target: targetUrl,
      logger: console,
    });

    smee.start();

    console.log(`
🔗 Smee.io proxy active
   Source: ${sourceUrl}
   Target: ${targetUrl}

   Configure this URL as your GitHub App's webhook URL for testing.
    `);
  } catch (error) {
    console.warn('⚠️  Could not start Smee proxy:', error);
    console.log('   Install smee-client: npm install smee-client');
  }
}
