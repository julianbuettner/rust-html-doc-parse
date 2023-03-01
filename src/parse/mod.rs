mod content;
mod error;
use std::path::PathBuf;

use scraper::{ElementRef, Html, Selector};

use crate::{Content, DocuPage};

use self::{
    content::{get_main_content, parse_to_content},
    error::HtmlParseError,
};

// When working with scraper,
// text elements have a lot of whitespace around them.
// <p>
//     text</p>
// would have the text "\n      text".
//
// To avoid that, we minify all HTML before parsing it with scraper.
// HTML's <pre> tag to preserve whitespace (e.g. code) is respected.
fn minify(html: &str) -> String {
    let config = minify_html::Cfg::default();
    std::str::from_utf8(&minify_html::minify(html.as_bytes(), &config))
        .expect("HTML Minification result is not UTF8")
        .into()
}

fn get_content_and_sections(html: &Html) -> Result<Content, HtmlParseError> {
    let selector = Selector::parse("#main-content").unwrap();
    let content = html.select(&selector).collect::<Vec<ElementRef>>();

    parse_to_content(&content[0])
}

pub fn parse_html(html: &str) -> Result<DocuPage, HtmlParseError> {
    let document = Html::parse_document(minify(html).as_str());

    let real_errors = document
        .errors
        .iter()
        .filter(|x| *x != "Bad character")
        .filter(|x| *x != "Character reference does not end with semicolon")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if !real_errors.is_empty() {
        return Err(HtmlParseError::InvalidHtml(real_errors.join("\n")));
    }

    let main_content = get_main_content(&document.root_element())?;
    Ok(DocuPage {
        content: main_content,
        meta: crate::DocuPageMeta {
            documentation_percent: None,
            location: crate::PageLocation {
                crate_name: "".to_string(),
                crate_version: crate::CrateVersion::Latest,
                source: crate::DocuSource::Local {
                    filepath: Box::new(PathBuf::new()),
                },
            },
            page_type: crate::DocsType::Enum,
            references: crate::References {
                crates_io: None,
                dependencies: None,
                owners: None,
                platforms: None,
                repository: None,
                versions: None,
            },
            title: "".to_string(),
        },
    })
}
