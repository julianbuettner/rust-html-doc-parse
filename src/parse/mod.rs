mod content;
mod error;
mod meta;


use scraper::{ElementRef, Html, Selector};

use crate::{Content, DocuPage};

use self::{content::parse_to_content, error::HtmlParseError, meta::parse_meta_from_html};

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

fn get_main_content(element: &ElementRef) -> Result<Content, HtmlParseError> {
    let selector = Selector::parse("#main-content").unwrap();
    let content = element.select(&selector).collect::<Vec<ElementRef>>();
    if content.len() != 1 {
        return Err(HtmlParseError::ElementCountNotOne(
            "#main-content",
            content.len(),
        ));
    }
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
        meta: parse_meta_from_html(&document)?,
    })
}
