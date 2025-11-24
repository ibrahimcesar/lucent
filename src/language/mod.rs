//! Language detection and configuration
//!
//! This module handles language detection by file extension and provides
//! language-specific configuration like comment styles.

use std::path::Path;

/// Represents a programming language with its configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
    /// Language name
    pub name: &'static str,
    /// File extensions (without the dot)
    pub extensions: &'static [&'static str],
    /// Single-line comment prefix (e.g., "//" for Rust)
    pub line_comment: Option<&'static str>,
    /// Multi-line comment delimiters (start, end)
    pub block_comment: Option<(&'static str, &'static str)>,
}

impl Language {
    /// Detect language from file path
    pub fn from_path(path: &Path) -> Option<&'static Language> {
        let extension = path.extension()?.to_str()?;
        Self::from_extension(extension)
    }

    /// Detect language from extension
    pub fn from_extension(ext: &str) -> Option<&'static Language> {
        LANGUAGES.iter().find(|lang| {
            lang.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
        })
    }

    /// Get emoji for this language
    pub fn emoji(&self) -> &'static str {
        match self.name {
            "Rust" => "🦀",
            "Python" => "🐍",
            "JavaScript" => "📜",
            "TypeScript" => "💠",
            "Go" => "🐹",
            "Java" => "☕",
            "C++" => "⚙️",
            "C" => "🔧",
            "Ruby" => "💎",
            "PHP" => "🐘",
            "C#" => "#️⃣",
            "Swift" => "🦅",
            "Kotlin" => "🎯",
            "Dart" => "🎯",
            "Scala" => "🔴",
            "Elixir" => "💧",
            "Haskell" => "λ",
            "Clojure" => "🌀",
            "Lua" => "🌙",
            "Shell" => "🐚",
            "SQL" => "🗄️",
            _ => "📄",
        }
    }
}

/// Static registry of supported languages
pub static LANGUAGES: &[Language] = &[
    Language {
        name: "Rust",
        extensions: &["rs"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Python",
        extensions: &["py", "pyw"],
        line_comment: Some("#"),
        block_comment: Some(("\"\"\"", "\"\"\"")),
    },
    Language {
        name: "JavaScript",
        extensions: &["js", "jsx", "mjs"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "TypeScript",
        extensions: &["ts", "tsx"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Go",
        extensions: &["go"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Java",
        extensions: &["java"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "C++",
        extensions: &["cpp", "cc", "cxx", "hpp", "h", "hxx"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "C",
        extensions: &["c", "h"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "C#",
        extensions: &["cs"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Ruby",
        extensions: &["rb"],
        line_comment: Some("#"),
        block_comment: Some(("=begin", "=end")),
    },
    Language {
        name: "PHP",
        extensions: &["php"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Swift",
        extensions: &["swift"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Kotlin",
        extensions: &["kt", "kts"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Dart",
        extensions: &["dart"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Scala",
        extensions: &["scala", "sc"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
    },
    Language {
        name: "Elixir",
        extensions: &["ex", "exs"],
        line_comment: Some("#"),
        block_comment: None,
    },
    Language {
        name: "Haskell",
        extensions: &["hs", "lhs"],
        line_comment: Some("--"),
        block_comment: Some(("{-", "-}")),
    },
    Language {
        name: "Clojure",
        extensions: &["clj", "cljs", "cljc", "edn"],
        line_comment: Some(";"),
        block_comment: None,
    },
    Language {
        name: "Lua",
        extensions: &["lua"],
        line_comment: Some("--"),
        block_comment: Some(("--[[", "]]")),
    },
    Language {
        name: "Shell",
        extensions: &["sh", "bash", "zsh"],
        line_comment: Some("#"),
        block_comment: None,
    },
    Language {
        name: "SQL",
        extensions: &["sql"],
        line_comment: Some("--"),
        block_comment: Some(("/*", "*/")),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_language_detection() {
        let rust_file = PathBuf::from("main.rs");
        let lang = Language::from_path(&rust_file);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Rust");
    }

    #[test]
    fn test_extension_detection() {
        let lang = Language::from_extension("py");
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Python");
    }

    #[test]
    fn test_emoji() {
        let rust = Language::from_extension("rs").unwrap();
        assert_eq!(rust.emoji(), "🦀");

        let python = Language::from_extension("py").unwrap();
        assert_eq!(python.emoji(), "🐍");
    }
}
