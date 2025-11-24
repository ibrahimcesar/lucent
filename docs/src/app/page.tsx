import Link from 'next/link';

export default function HomePage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-gradient-to-b from-zinc-50 to-white dark:from-zinc-950 dark:to-zinc-900">
      <div className="max-w-5xl w-full text-center space-y-8">
        <div className="space-y-4">
          <h1 className="text-6xl font-bold bg-gradient-to-r from-zinc-900 to-zinc-600 dark:from-zinc-100 dark:to-zinc-400 bg-clip-text text-transparent">
            Lucent
          </h1>
          <p className="text-2xl text-zinc-600 dark:text-zinc-400">
            Fast, multi-language code quality analyzer
          </p>
        </div>

        <div className="flex gap-4 justify-center items-center">
          <Link
            href="/docs"
            className="px-6 py-3 bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 rounded-lg font-semibold hover:bg-zinc-800 dark:hover:bg-zinc-200 transition-colors"
          >
            Get Started
          </Link>
          <a
            href="https://github.com/anthropics/lucent"
            target="_blank"
            rel="noopener noreferrer"
            className="px-6 py-3 border-2 border-zinc-900 dark:border-zinc-100 text-zinc-900 dark:text-zinc-100 rounded-lg font-semibold hover:bg-zinc-900 hover:text-white dark:hover:bg-zinc-100 dark:hover:text-zinc-900 transition-colors"
          >
            GitHub
          </a>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-16">
          <div className="p-6 border border-zinc-200 dark:border-zinc-800 rounded-lg">
            <h3 className="text-xl font-semibold mb-2">21+ Languages</h3>
            <p className="text-zinc-600 dark:text-zinc-400">
              Supports Rust, Python, JavaScript, TypeScript, Go, Java, and many more
            </p>
          </div>
          <div className="p-6 border border-zinc-200 dark:border-zinc-800 rounded-lg">
            <h3 className="text-xl font-semibold mb-2">Extensible Plugins</h3>
            <p className="text-zinc-600 dark:text-zinc-400">
              Create custom analyzers and metrics with the plugin API
            </p>
          </div>
          <div className="p-6 border border-zinc-200 dark:border-zinc-800 rounded-lg">
            <h3 className="text-xl font-semibold mb-2">Quality Gates</h3>
            <p className="text-zinc-600 dark:text-zinc-400">
              Enforce code quality standards in your CI/CD pipeline
            </p>
          </div>
        </div>
      </div>
    </main>
  );
}
