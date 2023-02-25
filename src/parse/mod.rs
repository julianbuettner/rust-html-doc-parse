mod block;
mod content;
mod error;
pub use block::*;
use scraper::Html;

use crate::DocuPage;

use self::error::HtmlParseError;

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

pub fn parse_html(html: &str) -> Result<DocuPage, HtmlParseError> {
    let document = Html::parse_document(minify(html).as_str());
    if !document.errors.is_empty() {
        return Err(HtmlParseError::InvalidHtml(document.errors.join("\n")));
    }

    todo!()
}
