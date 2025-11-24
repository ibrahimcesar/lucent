# Fumadocs Setup for Lucent

This document describes the Fumadocs documentation site setup for Lucent.

## ✅ Completed Setup

### 1. **Project Structure**

```
docs/
├── content/docs/          # All documentation MDX files
│   ├── index.mdx          # Home page
│   ├── installation.mdx
│   ├── quick-start.mdx
│   ├── metrics.mdx
│   ├── thresholds.mdx
│   ├── snapshots.mdx
│   ├── maintainability-index.mdx
│   ├── html-reports.mdx
│   ├── plugin-guide.mdx
│   ├── config-file-guide.mdx
│   ├── ci-quality-gates.mdx
│   ├── languages.mdx
│   ├── roadmap.mdx
│   └── meta.json          # Navigation structure
├── src/app/               # Next.js application
│   ├── layout.tsx         # Root layout
│   ├── page.tsx           # Landing page
│   ├── global.css         # Tailwind styles
│   ├── docs/
│   │   ├── layout.tsx     # Docs layout
│   │   └── [[...slug]]/
│   │       └── page.tsx   # Dynamic docs pages
│   ├── layout.config.tsx  # Site configuration
│   └── source.ts          # Content source
├── public/                # Static assets
│   └── .nojekyll         # GitHub Pages config
├── package.json
├── next.config.mjs
├── tsconfig.json
├── tailwind.config.ts
├── source.config.ts       # Fumadocs configuration
└── README.md
```

### 2. **Documentation Files**

All existing documentation has been converted to MDX format with proper frontmatter:

- ✅ **Getting Started**
  - Installation guide
  - Quick start tutorial

- ✅ **Core Concepts**
  - Metrics documentation
  - Thresholds & quality gates
  - Snapshots & history tracking
  - Maintainability Index guide

- ✅ **Features**
  - HTML reports guide
  - Plugin developer guide

- ✅ **Examples**
  - Configuration file guide
  - CI/CD quality gates

- ✅ **Reference**
  - Supported languages
  - Roadmap

### 3. **GitHub Actions Workflow**

Created `.github/workflows/deploy-docs.yml` for automatic deployment to GitHub Pages:

- Triggers on push to `main` branch
- Monitors changes in `website/`, `docs/`, `examples/`, and markdown files
- Builds and deploys to GitHub Pages
- Uses Next.js static export for optimal performance

### 4. **Configuration**

- **Next.js**: Configured for static export with proper base path
- **Tailwind CSS**: Integrated with Fumadocs presets
- **TypeScript**: Properly typed with path mapping
- **Fumadocs**: Latest version with MDX support

## 🚀 Development

### Install Dependencies

```bash
cd docs
npm install
```

### Run Development Server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000)

### Build for Production

```bash
npm run build
```

Output will be in the `out/` directory.

## 📝 Adding New Documentation

1. Create a new `.mdx` file in `content/docs/`:

```mdx
---
title: Your Page Title
description: Brief description for SEO
---

# Your Page Title

Content goes here...
```

2. Update `content/docs/meta.json` to include the page in navigation:

```json
{
  "title": "Documentation",
  "pages": [
    "index",
    "your-new-page",  // Add here
    ...
  ]
}
```

3. The page will be automatically picked up on next build!

## 🎨 Customization

### Branding

Edit `src/app/layout.config.tsx`:

```tsx
export const baseOptions: BaseLayoutProps = {
  nav: {
    title: 'Lucent',  // Change site title
  },
  links: [...],  // Customize nav links
  githubUrl: '...',  // GitHub repository
};
```

### Home Page

Edit `src/app/page.tsx` to customize the landing page.

### Styling

Edit `src/app/global.css` for custom styles.

## 🌐 Deployment

### GitHub Pages

1. **Enable GitHub Pages** in repository settings
2. **Set source** to "GitHub Actions"
3. **Push to main** branch - deployment is automatic!

The site will be available at:
`https://anthropics.github.io/lucent/`

### Manual Deployment

```bash
npm run build
# Upload the `out/` directory to any static hosting
```

## 🔧 Troubleshooting

### Build fails with "Module not found"

Run `npx fumadocs-mdx` to regenerate source files.

### Pages not showing

1. Check that `.mdx` files have proper frontmatter
2. Verify the file is listed in `content/docs/meta.json`
3. Rebuild with `npm run build`

### Styling issues

Clear Next.js cache:
```bash
rm -rf .next
npm run dev
```

## 📚 Documentation

- [Fumadocs](https://fumadocs.vercel.app/docs)
- [Next.js](https://nextjs.org/docs)
- [Tailwind CSS](https://tailwindcss.com/docs)

## ✨ Features

- 🎨 **Beautiful UI** - Modern, responsive design
- 🌙 **Dark Mode** - Automatic dark/light mode
- 🔍 **Search** - Full-text search (coming soon)
- 📱 **Mobile Friendly** - Responsive on all devices
- ⚡ **Fast** - Static site generation
- 🎯 **SEO Optimized** - Proper meta tags and sitemap

## 📦 Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Documentation**: Fumadocs 13
- **Styling**: Tailwind CSS 3
- **Deployment**: GitHub Pages
- **Type Safety**: TypeScript 5

---

**Status**: ✅ Ready for deployment!
