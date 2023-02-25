use serde::{Deserialize, Serialize};

use crate::{Content, TextAtomic};

/// DocuPage is an abstract syntax tree and contains information
/// as well as meta information about a typical page from docs.rs.
/// An example page would be docs.rs/serde/1.0.152/serde/.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocuSource {
    // DocsRs url might be "https://docs.rs/serde/1.0.152/serde/"
    DocsRs { url: String },
    // After running `cargo doc`, documentation
    // of already installed packages
    // can be parsed from filesystem
    Local { filepath: Box<std::path::Path> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrateVersion {
    // For links like
    // https://docs.rs/html_parser/latest/
    // or crates specified by a file system location,
    // git url without commit, the CrateVersion
    // has to be latest
    Latest,
    // 1.2.3-rc5
    Semantic {
        major: u64,             // 1
        minor: u64,             // 2
        patch: u64,             // 3
        suffix: Option<String>, // Some("rc5")
    },
    GitCommit {
        hash: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLocation {
    pub crate_name: String,
    pub crate_version: CrateVersion,
    pub source: DocuSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocsType {
    Crate,
    Enum,
    Module,
    Struct,
    Trait,
}

// Here we have everything that could be considered
// rendered in a markdown style.
// On crates.io we have usually the header
// "Enum html_parser::DomVariant" for example.
// Then there is some introduction generated from markdown
// like example code.
// After that, we have a list of sections for
// Fields, Implementations, Trait Implementations and so on.
// Those go here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocuPageContent {
    pub title: Vec<TextAtomic>,
    pub introduction: Content,
    pub sections: Vec<(String, Content)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyReference {
    pub name: String,
    pub version: CrateVersion,
    pub url: String,
}

// Reference to different versions
// of the same crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReference {
    pub version: CrateVersion,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum License {
    MIT,
    Apache2,
    GPL20,
    GPL30,
    AGPL30,
    Other(String),
}

// Links to related pages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct References {
    pub crates_io: Option<String>,
    pub dependencies: Option<Vec<DependencyReference>>,
    pub owners: Option<Vec<OwnerReference>>,
    pub platforms: Option<Vec<PlatformReference>>,
    pub repository: Option<String>, // e.g. GitHub URL
    pub versions: Option<Vec<VersionReference>>,
}

// Why?
// To only represent valid states
// and to allow derive(Eq).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Percentage {
    v: u32, // how much of 10_000
}

impl Percentage {
    /// 0.25 = 25%
    /// Guaranteed to be between 0% and 100%.
    /// Resolution of 1 / 10_000
    /// Returns Err(()) for out of bound values.
    pub fn from_f32_ratio(r: f32) -> Result<Self, ()> {
        if r < 0. || r > 1. {
            return Err(());
        }
        Ok(Self {
            v: (r * 10_000.).round() as u32,
        })
    }

    pub fn as_f32_ratio(&self) -> f32 {
        self.v as f32 / 10_000.
    }
}

// Everything that is there about the documentation page,
// that has not directly in the markdown styled content.
// For example type, name, version, links to other
// versions, etc.
// Those information might also somewhere in the
// content, but they are usually read from dropdowns, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocuPageMeta {
    pub documentation_percent: Option<Percentage>,
    pub location: PageLocation,
    pub page_type: DocsType,
    pub references: References,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocuPage {
    pub content: DocuPageContent,
    pub meta: DocuPageMeta,
}
