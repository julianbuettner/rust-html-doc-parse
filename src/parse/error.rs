#[derive(Debug)]
pub enum HtmlParseError {
    ElementCountNotOne(&'static str, usize),
    InvalidHtml(String),
    PageTypeUnknown(String),
}

impl HtmlParseError {
    pub fn invalid_html<T>(s: T) -> Self
    where
        T: ToString,
    {
        Self::InvalidHtml(s.to_string())
    }
}
