pub mod model;
pub mod parse;
extern crate derive_builder;

pub use model::*;
pub use parse::parse_html;
