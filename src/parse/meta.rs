use scraper::{ElementRef, Html, Selector};

use crate::{DocsType, DocuPageMeta, References};

use super::error::HtmlParseError;

fn get_references(_root: &ElementRef) -> Result<References, HtmlParseError> {
    Ok(References {
        crates_io: None,
        dependencies: None,
        owners: None,
        platforms: None,
        repository: None,
        versions: None,
    })
}

fn get_title(root: &ElementRef) -> Result<String, HtmlParseError> {
    let selector = Selector::parse(".fqn").unwrap();
    let content = root.select(&selector).collect::<Vec<ElementRef>>();
    if content.len() != 1 {
        return Err(HtmlParseError::ElementCountNotOne(".fqn", content.len()));
    }
    Ok(content[0]
        .text()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(""))
}

pub fn parse_meta_from_html(html: &Html) -> Result<DocuPageMeta, HtmlParseError> {
    let root = html.root_element();
    Ok(DocuPageMeta {
        documentation_percent: None,
        title: get_title(&root)?,
        page_type: DocsType::Struct,
        references: get_references(&root)?,
    })
}
