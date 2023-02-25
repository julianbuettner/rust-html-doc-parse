// I might regret this,
// but here we have a single enumeration
// of all languages.
// So no passing around of strings, which might
// be upper case, abbrevations, etc.
// It's only useful for syntax highlighting anyways.
// The only crates making trouble will be FFI ones
// for niche languages I suppose.
// Feel free to add languages.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Bash,
    C,
    Cpp,
    Go,
    Html,
    Ini,
    Java,
    JavaScript,
    Json,
    Kotlin,
    Lua,
    Matlab,
    Perl,
    Php,
    Python,
    R,
    Ruby,
    Rust,
    Sql,
    Swift,
    Toml,
    TypeScript,
    Xml,
    Yaml,
}

impl Language {
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "bash" | "shell" | "sh" => Some(Self::Bash),
            "c" => Some(Self::C),
            "cpp" | "c++" => Some(Self::Cpp),
            "go" | "golang" => Some(Self::Go),
            "html" => Some(Self::Html),
            "ini" => Some(Self::Ini),
            "java" => Some(Self::Java),
            "javascript" | "js" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "kotlin" => Some(Self::Kotlin),
            "lua" => Some(Self::Lua),
            "matlab" => Some(Self::Matlab),
            "perl" => Some(Self::Perl),
            "php" => Some(Self::Php),
            "python" | "py" => Some(Self::Python),
            "r" => Some(Self::R),
            "ruby" => Some(Self::Ruby),
            "rust" | "rs" => Some(Self::Rust),
            "sql" => Some(Self::Sql),
            "swift" => Some(Self::Swift),
            "toml" => Some(Self::Toml),
            "typescript" | "ts" => Some(Self::TypeScript),
            "xml" => Some(Self::Xml),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}
