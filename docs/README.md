# Lucent Documentation Website

This directory contains the Fumadocs-based documentation website for Lucent.

## 🚀 Quick Start

### Development

```bash
cd docs
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build

```bash
npm run build
```

The static site will be generated in the `out/` directory.

### Production

```bash
npm run start
```

## 📁 Structure

```
docs/
├── content/
│   └── docs/           # Documentation MDX files
│       ├── index.mdx
│       ├── installation.mdx
│       ├── quick-start.mdx
│       ├── metrics.mdx
│       └── ...
├── src/
│   └── app/            # Next.js app directory
│       ├── layout.tsx
│       ├── page.tsx
│       ├── docs/
│       │   ├── layout.tsx
│       │   └── [[...slug]]/
│       │       └── page.tsx
│       ├── layout.config.tsx
│       └── source.ts
├── public/             # Static assets
├── source.config.ts    # Fumadocs configuration
├── package.json
└── next.config.mjs
```

## 📝 Adding Documentation

1. Create a new `.mdx` file in `content/docs/`
2. Add frontmatter:
   ```mdx
   ---
   title: Your Page Title
   description: Brief description
   ---

   # Content starts here
   ```
3. Update `content/docs/meta.json` to include your page in the sidebar

## 🎨 Customization

- **Theme**: Edit `src/app/layout.config.tsx`
- **Home Page**: Edit `src/app/page.tsx`
- **Styles**: Edit `src/app/global.css`

## 🚢 Deployment

The site is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

- **Workflow**: `.github/workflows/deploy-docs.yml`
- **URL**: `https://anthropics.github.io/lucent/` (when published)

### Manual Deployment

```bash
npm run build
# Upload the `out/` directory to your hosting service
```

## 📚 Documentation

- [Fumadocs Documentation](https://fumadocs.vercel.app/docs)
- [Next.js Documentation](https://nextjs.org/docs)

## 🛠️ Tech Stack

- **Framework**: Next.js 14
- **Documentation**: Fumadocs
- **Styling**: Tailwind CSS
- **Deployment**: GitHub Pages

## 📄 License

MIT
