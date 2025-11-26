import * as vscode from 'vscode';
import * as https from 'https';

export enum LicenseTier {
    Free = 'free',
    Pro = 'pro',
    Team = 'team',
    Enterprise = 'enterprise'
}

export interface LicenseStatus {
    tier: LicenseTier;
    active: boolean;
    expiresAt?: string;
    features: string[];
}

const FEATURE_MATRIX: Record<LicenseTier, string[]> = {
    [LicenseTier.Free]: [
        'basic-metrics',
        'inline-lenses',
        'status-bar',
        'file-analysis'
    ],
    [LicenseTier.Pro]: [
        'basic-metrics',
        'inline-lenses',
        'status-bar',
        'file-analysis',
        'custom-rules',
        'auto-fix',
        'html-reports',
        'all-languages',
        'ast-rules',
        'metric-rules'
    ],
    [LicenseTier.Team]: [
        'basic-metrics',
        'inline-lenses',
        'status-bar',
        'file-analysis',
        'custom-rules',
        'auto-fix',
        'html-reports',
        'all-languages',
        'ast-rules',
        'metric-rules',
        'team-dashboard',
        'git-trends',
        'historical-tracking',
        'slack-integration'
    ],
    [LicenseTier.Enterprise]: [
        'basic-metrics',
        'inline-lenses',
        'status-bar',
        'file-analysis',
        'custom-rules',
        'auto-fix',
        'html-reports',
        'all-languages',
        'ast-rules',
        'metric-rules',
        'team-dashboard',
        'git-trends',
        'historical-tracking',
        'slack-integration',
        'self-hosted',
        'white-label',
        'sso',
        'sla',
        'custom-rules-dev'
    ]
};

export class LicenseManager {
    private static instance: LicenseManager;
    private cachedStatus: LicenseStatus | null = null;
    private lastCheck: number = 0;
    private readonly CACHE_DURATION = 3600000; // 1 hour

    private constructor() {}

    static getInstance(): LicenseManager {
        if (!LicenseManager.instance) {
            LicenseManager.instance = new LicenseManager();
        }
        return LicenseManager.instance;
    }

    /**
     * Get current license status
     */
    async getStatus(): Promise<LicenseStatus> {
        const now = Date.now();

        // Return cached if still valid
        if (this.cachedStatus && (now - this.lastCheck) < this.CACHE_DURATION) {
            return this.cachedStatus;
        }

        const config = vscode.workspace.getConfiguration('lucent');
        const apiKey = config.get<string>('apiKey');

        if (!apiKey) {
            // No API key = Free tier
            return this.getFreeStatus();
        }

        try {
            const status = await this.verifyLicense(apiKey);
            this.cachedStatus = status;
            this.lastCheck = now;
            return status;
        } catch (error) {
            console.error('License verification failed:', error);
            return this.getFreeStatus();
        }
    }

    /**
     * Check if a feature is available in current tier
     */
    async hasFeature(feature: string): Promise<boolean> {
        const status = await this.getStatus();
        return status.features.includes(feature);
    }

    /**
     * Get tier display name
     */
    getTierName(tier: LicenseTier): string {
        const names = {
            [LicenseTier.Free]: 'Free',
            [LicenseTier.Pro]: 'Pro',
            [LicenseTier.Team]: 'Team',
            [LicenseTier.Enterprise]: 'Enterprise'
        };
        return names[tier];
    }

    /**
     * Prompt user to upgrade
     */
    async promptUpgrade(feature: string): Promise<void> {
        const status = await this.getStatus();
        const currentTier = this.getTierName(status.tier);

        const message = `"${feature}" is not available in ${currentTier} tier. Upgrade to unlock this feature.`;
        const action = await vscode.window.showInformationMessage(
            message,
            'View Plans',
            'Enter License Key',
            'Dismiss'
        );

        if (action === 'View Plans') {
            vscode.env.openExternal(vscode.Uri.parse('https://lucent.dev/pricing'));
        } else if (action === 'Enter License Key') {
            await this.promptForLicenseKey();
        }
    }

    /**
     * Prompt user to enter license key
     */
    async promptForLicenseKey(): Promise<void> {
        const key = await vscode.window.showInputBox({
            prompt: 'Enter your Lucent license key',
            placeHolder: 'lucent_xxxxxxxxxxxxxxxxxxxxxxxx',
            password: true,
            validateInput: (value) => {
                if (!value || value.length < 10) {
                    return 'License key is too short';
                }
                if (!value.startsWith('lucent_')) {
                    return 'Invalid license key format';
                }
                return null;
            }
        });

        if (key) {
            // Verify before saving
            try {
                const status = await this.verifyLicense(key);
                if (status.active) {
                    const config = vscode.workspace.getConfiguration('lucent');
                    await config.update('apiKey', key, vscode.ConfigurationTarget.Global);
                    this.cachedStatus = status;
                    this.lastCheck = Date.now();

                    vscode.window.showInformationMessage(
                        `Successfully activated ${this.getTierName(status.tier)} license!`
                    );
                } else {
                    vscode.window.showErrorMessage('License key is not active or has expired');
                }
            } catch (error) {
                vscode.window.showErrorMessage('Invalid license key');
            }
        }
    }

    /**
     * Clear cached license status
     */
    clearCache(): void {
        this.cachedStatus = null;
        this.lastCheck = 0;
    }

    /**
     * Verify license with API
     */
    private async verifyLicense(apiKey: string): Promise<LicenseStatus> {
        return new Promise((resolve, reject) => {
            const data = JSON.stringify({ api_key: apiKey });

            const options = {
                hostname: 'api.lucent.dev',
                port: 443,
                path: '/v1/verify',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Content-Length': data.length,
                    'User-Agent': 'lucent-vscode/0.1.0'
                }
            };

            const req = https.request(options, (res) => {
                let body = '';

                res.on('data', (chunk) => {
                    body += chunk;
                });

                res.on('end', () => {
                    try {
                        const response = JSON.parse(body);

                        if (res.statusCode === 200 && response.active) {
                            resolve({
                                tier: response.tier as LicenseTier,
                                active: response.active,
                                expiresAt: response.expires_at,
                                features: FEATURE_MATRIX[response.tier as LicenseTier] || []
                            });
                        } else {
                            reject(new Error('License verification failed'));
                        }
                    } catch (error) {
                        reject(error);
                    }
                });
            });

            req.on('error', (error) => {
                reject(error);
            });

            req.write(data);
            req.end();
        });
    }

    /**
     * Get free tier status
     */
    private getFreeStatus(): LicenseStatus {
        return {
            tier: LicenseTier.Free,
            active: true,
            features: FEATURE_MATRIX[LicenseTier.Free]
        };
    }
}

/**
 * Feature guard decorator
 */
export function requiresFeature(feature: string) {
    return function (
        target: any,
        propertyKey: string,
        descriptor: PropertyDescriptor
    ) {
        const originalMethod = descriptor.value;

        descriptor.value = async function (...args: any[]) {
            const licenseManager = LicenseManager.getInstance();
            const hasFeature = await licenseManager.hasFeature(feature);

            if (!hasFeature) {
                await licenseManager.promptUpgrade(feature);
                return;
            }

            return originalMethod.apply(this, args);
        };

        return descriptor;
    };
}
