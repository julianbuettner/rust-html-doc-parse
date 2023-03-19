use pretty_assertions::assert_eq;
use rust_html_doc_parse::{
    self, BlockContainer, Content, DocsType, DocuPage, DocuPageMeta, References, TextAtomic, TextAtomicBuilder,
};


#[test]
fn fail() {
    // Open the HTML to get a visual.
    // https://docs.rs/rand/0.8.5/rand/struct.Error.html
    let docs = include_str!("resources/docs.rs_rand_0.8.5_rand_struct.Error.html");
    let mut parsed = rust_html_doc_parse::parse_html(docs).unwrap();

    // Writing down the entire page here sucks.
    // So we limit it to the fist 9 containers.
    parsed.content.0.truncate(9);

    let expected = DocuPage {
        content: Content(vec![
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
            BlockContainer::Heading2(vec![TextAtomic::simple(&"Implementations")]),
            BlockContainer::Paragraph(vec![TextAtomic::simple(&"source").with_some_url(
                &"https://rust-random.github.io/rand/src/rand_core/error.rs.html#28-116",
            )]),
            BlockContainer::Heading3(vec![
                TextAtomic::simple(&"impl "),
                TextAtomic::simple(&"Error").with_some_url(&"struct.Error.html"),
            ]),
            BlockContainer::Paragraph(vec![TextAtomic::simple(&"source").with_some_url(
                &"https://rust-random.github.io/rand/src/rand_core/error.rs.html#36",
            )]),
            BlockContainer::Heading4(vec![
                TextAtomic::simple(&"pub const "),
                TextAtomic::simple(&"CUSTOM_START")
                    .with_some_url(&"#associatedconstant.CUSTOM_START"),
                TextAtomic::simple(&": "),
                TextAtomic::simple(&"u32")
                    .with_some_url(&"https://doc.rust-lang.org/nightly/std/primitive.u32.html"),
            ]),
        ]),
        meta: DocuPageMeta {
            documentation_percent: None, //Some(Percentage::from_f32_ratio(1.).unwrap()),
            page_type: DocsType::Struct,
            references: References {
                crates_io: None,
                dependencies: None,
                owners: None,
                platforms: None,
                repository: None,
                versions: None,
            },
            title: "Struct rand::Error".to_string(),
        },
    };
    assert_eq!(parsed.meta, expected.meta);
    assert_eq!(parsed.content, expected.content);
    assert_eq!(parsed, expected);
}
