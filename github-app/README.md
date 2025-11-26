# Lucent GitHub App

A GitHub App that automatically analyzes code metrics on pull requests using [Lucent](https://github.com/ibrahimcesar/lucent).

## Features

- **Automatic PR Analysis** - Analyzes changed files when PRs are opened or updated
- **Check Runs** - Creates GitHub check runs with pass/fail status
- **PR Comments** - Posts detailed metrics reports as comments (for significant issues)
- **Metrics Tracked**:
  - Cyclomatic Complexity
  - Cognitive Complexity
  - Maintainability Index (A-F grades)
  - Code Smells
  - Nesting Depth

## Setup

### 1. Create a GitHub App

1. Go to **Settings** > **Developer settings** > **GitHub Apps** > **New GitHub App**
2. Configure the app:
   - **Name**: `Lucent Code Metrics` (or your preferred name)
   - **Homepage URL**: Your URL or `https://github.com/ibrahimcesar/lucent`
   - **Webhook URL**: Your server URL + `/webhook` (e.g., `https://your-server.com/webhook`)
   - **Webhook secret**: Generate a secure random string
3. Set permissions:
   - **Repository permissions**:
     - Checks: Read & write
     - Contents: Read
     - Pull requests: Read & write
     - Metadata: Read
4. Subscribe to events:
   - Check suite
   - Pull request
5. Generate and download a private key

### 2. Configure Environment

Copy `.env.example` to `.env` and fill in your values:

```bash
cp .env.example .env
```

```env
GITHUB_APP_ID=123456
GITHUB_WEBHOOK_SECRET=your-webhook-secret
GITHUB_PRIVATE_KEY_PATH=./private-key.pem
PORT=3000
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Run the App

**Development:**
```bash
npm run dev
```

**Production:**
```bash
npm run build
npm start
```

**With Docker:**
```bash
docker-compose up -d
```

## Local Development with Smee

For local development, use [Smee.io](https://smee.io/) to proxy webhooks:

1. Go to https://smee.io/new to create a new channel
2. Add the Smee URL to your `.env`:
   ```env
   WEBHOOK_PROXY_URL=https://smee.io/your-channel
   NODE_ENV=development
   ```
3. Set the Smee URL as your GitHub App's webhook URL
4. Run `npm run dev`

## How It Works

1. **PR Opened/Updated**: When a PR is opened or new commits are pushed:
   - Creates a "Lucent Code Metrics" check run (status: in progress)
   - Fetches the list of changed files
   - Downloads file contents from the PR head branch
   - Runs Lucent analysis on the files
   - Updates the check run with results

2. **Check Run Results**:
   - ✅ **Success**: All metrics within acceptable ranges
   - ⚠️ **Action Required**: High complexity or low maintainability detected
   - ❌ **Failure**: Critical code smells found

3. **PR Comments**: For significant issues, posts a detailed comment with:
   - Summary statistics
   - List of problem files
   - Critical code smells

## Configuration

### Thresholds (Hardcoded)

Current thresholds for determining check status:

| Metric | Warning | Failure |
|--------|---------|---------|
| Cyclomatic Complexity | > 20 | - |
| Maintainability Index | < 50 | < 25 |
| Critical Code Smells | - | > 0 |

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/webhook` | POST | GitHub webhook receiver |

## Deployment

### Docker

```bash
# Build image
docker build -t lucent-github-app .

# Run container
docker run -d \
  -p 3000:3000 \
  -e GITHUB_APP_ID=123456 \
  -e GITHUB_WEBHOOK_SECRET=secret \
  -e GITHUB_PRIVATE_KEY="$(cat private-key.pem)" \
  lucent-github-app
```

### Cloud Platforms

The app can be deployed to:
- **AWS Lambda** (with API Gateway)
- **Google Cloud Run**
- **Azure Functions**
- **Heroku**
- **Railway**
- **Render**

## Architecture

```
github-app/
├── src/
│   ├── index.ts          # Express server entry point
│   ├── auth.ts           # GitHub App authentication
│   ├── webhooks.ts       # Webhook event router
│   ├── handlers/
│   │   ├── pullRequest.ts  # PR event handler
│   │   └── checkSuite.ts   # Check suite handler
│   ├── lucent.ts         # Lucent CLI integration
│   ├── formatter.ts      # Output formatting
│   └── dev.ts            # Development utilities
├── Dockerfile
├── docker-compose.yml
└── package.json
```

## Troubleshooting

### "Lucent not found"

The Docker image includes Lucent. For local development:
```bash
cargo install lucent-code
```

### Webhook signature errors

Ensure `GITHUB_WEBHOOK_SECRET` matches exactly what you configured in GitHub.

### Private key errors

- Ensure the `.pem` file is in the correct location
- Or set `GITHUB_PRIVATE_KEY` environment variable with the key contents (replace newlines with `\n`)

### Check runs not appearing

1. Verify the app is installed on the repository
2. Check that "Checks: Read & write" permission is granted
3. Look at the webhook delivery history in GitHub App settings

## License

MIT License - see [LICENSE](../LICENSE) for details.
