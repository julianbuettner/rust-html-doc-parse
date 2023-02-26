use pretty_assertions::assert_eq;
use rust_html_doc_parse::{
    self, BlockContainer, Content, CrateVersion, DocsType, DocuPage, DocuPageContent, DocuPageMeta,
    DocuSource, PageLocation, References, TextAtomic, TextAtomicBuilder,
};
use std::path::PathBuf;

#[test]
fn fail() {
    // Open the HTML to get a visual.
    // https://docs.rs/rand/0.8.5/rand/struct.Error.html
    let docs = include_str!("resources/docs.rs_rand_0.8.5_rand_struct.Error.html");
    let parsed = rust_html_doc_parse::parse_html(&docs).unwrap();
    let expected = DocuPage {
        content: DocuPageContent {
            title: vec![],
            introduction: Content(vec![
                BlockContainer::Heading1(vec![
                    TextAtomic::simple(&"Struct "),
                    TextAtomic::simple(&"rand").with_some_url(&"index.html"),
                    TextAtomic::simple(&"::"),
                    TextAtomic::simple(&"Error").with_some_url(&"#"),
                ]),
                BlockContainer::Code {
                    code: "pub struct Error { /* private fields */ }".to_string(),
                    language: None,
                },
                BlockContainer::Paragraph(vec![TextAtomic::simple(
                    &"Error type of random number generators",
                )]),
                BlockContainer::Paragraph(vec![
                    TextAtomic::simple(&"In order to be compatible with "),
                    TextAtomicBuilder::new(&"std").code(true).build(),
                    TextAtomic::simple(&" and "),
                    TextAtomicBuilder::new(&"no_std").code(true).build(),
                    TextAtomic::simple(&", this type has two possible implementations: with "),
                    TextAtomicBuilder::new(&"std").code(true).build(),
                    TextAtomic::simple(&" a boxed "),
                    TextAtomicBuilder::new(&"Error").code(true).build(),
                    TextAtomic::simple(&" trait object is stored, while with "),
                    TextAtomicBuilder::new(&"no_std").code(true).build(),
                    TextAtomic::simple(&" we merely store an error code."),
                ]),
            ]),
            sections: vec![],
        },
        meta: DocuPageMeta {
            documentation_percent: None,
            location: PageLocation {
                crate_name: "".to_string(),
                crate_version: CrateVersion::Latest,
                source: DocuSource::Local {
                    filepath: Box::new(PathBuf::new()),
                },
            },
            page_type: DocsType::Enum,
            references: References {
                crates_io: None,
                dependencies: None,
                owners: None,
                platforms: None,
                repository: None,
                versions: None,
            },
            title: "".to_string(),
        },
    };
    assert_eq!(parsed, expected);
}
