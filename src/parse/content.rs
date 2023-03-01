use scraper::{ElementRef, Node, Selector};
use selectors::attr::CaseSensitivity;

use crate::{BlockContainer, Content, Language, TextAtomic, TextAtomicBuilder};

use super::error::HtmlParseError;

/// Find and parse the main content
/// of the documentation page,
/// from title to bottom.

// RecursiveResult might contain
// unfinished recursive results, which have
// to be combined to a greater structure.
enum RecursiveResult {
    TableRows(Vec<Vec<TextAtomic>>),
    Atomics(Vec<TextAtomic>),
    Blocks(Vec<BlockContainer>),
}

impl RecursiveResult {
    pub fn is_atomics(&self) -> bool {
        match self {
            Self::Atomics(_) => true,
            _ => false,
        }
    }
    pub fn _is_blocks(&self) -> bool {
        match self {
            Self::Blocks(_) => true,
            _ => false,
        }
    }
    pub fn is_table_rows(&self) -> bool {
        match self {
            Self::TableRows(_) => true,
            _ => false,
        }
    }
    pub fn atomics(self) -> Option<Vec<TextAtomic>> {
        match self {
            Self::Atomics(p) => Some(p),
            _ => None,
        }
    }
    pub fn _blocks(self) -> Option<Vec<BlockContainer>> {
        match self {
            Self::Blocks(b) => Some(b),
            _ => None,
        }
    }
    pub fn table_rows(self) -> Option<Vec<Vec<TextAtomic>>> {
        match self {
            Self::TableRows(rows) => Some(rows),
            _ => None,
        }
    }
}

fn bundle_resursive_results_to_block_vec(
    c: Vec<RecursiveResult>,
) -> Result<Vec<BlockContainer>, HtmlParseError> {
    let mut result: Vec<BlockContainer> = Vec::new();
    let mut neighboring_atomics = Vec::new();
    for child in c.into_iter() {
        match child {
            RecursiveResult::Atomics(mut v) => neighboring_atomics.append(&mut v),
            RecursiveResult::Blocks(mut b) => {
                if !neighboring_atomics.is_empty() {
                    result.push(BlockContainer::Paragraph(neighboring_atomics));
                    neighboring_atomics = Vec::new();
                }
                result.append(&mut b)
            }
            RecursiveResult::TableRows(_) => {
                return Err(HtmlParseError::invalid_html(
                    "Table row appeared outside of a table.",
                ));
            }
        }
    }
    if !neighboring_atomics.is_empty() {
        result.push(BlockContainer::Paragraph(neighboring_atomics));
    }
    Ok(result)
}

fn get_language_of_code(_e: &ElementRef) -> Option<Language> {
    None
}

fn atomics_to_string(atomics: Vec<TextAtomic>) -> String {
    atomics
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<String>>()
        .join("")
}

enum RecursiveChildrenSituation {
    NoChildren,
    AllBlocks(Vec<BlockContainer>),
    AllAtomics(Vec<Vec<TextAtomic>>),
    AllTableRows(Vec<Vec<Vec<TextAtomic>>>),
}

fn children_to_recursive_children_situation(
    children: Vec<RecursiveResult>,
) -> Result<RecursiveChildrenSituation, HtmlParseError> {
    if children.is_empty() {
        return Ok(RecursiveChildrenSituation::NoChildren);
    }

    let all_atomics = !children.iter().any(|x| !x.is_atomics());
    if all_atomics {
        let atomics: Vec<Vec<TextAtomic>> =
            children.into_iter().map(|x| x.atomics().unwrap()).collect();
        return Ok(RecursiveChildrenSituation::AllAtomics(atomics));
    }

    let all_table_rows = !children.iter().any(|x| !x.is_table_rows());
    if all_table_rows {
        let table: Vec<Vec<Vec<TextAtomic>>> = children
            .into_iter()
            .map(|x| x.table_rows().unwrap())
            .collect();
        return Ok(RecursiveChildrenSituation::AllTableRows(table));
    }

    let blocks = bundle_resursive_results_to_block_vec(children)?;
    Ok(RecursiveChildrenSituation::AllBlocks(blocks))
}

fn is_inline_code(element: &ElementRef) -> bool {
    fn is_preformatted(element: &ElementRef) -> bool {
        if element.value().name() == "pre" {
            return true;
        }
        let parent = element.parent().and_then(ElementRef::wrap);
        if parent.is_none() {
            return false;
        }
        is_preformatted(&parent.unwrap())
    }

    if is_preformatted(element) {
        return false;
    }

    let all_children_are_flat_text = !element.children().any(|x| !x.value().is_text());
    if all_children_are_flat_text {
        return true;
    }
    false
}

fn element_with_atomics_to_recursive_result(
    element: &ElementRef,
    atomics: Vec<Vec<TextAtomic>>,
) -> Result<Option<RecursiveResult>, HtmlParseError> {
    fn ok_some_block(b: BlockContainer) -> Result<Option<RecursiveResult>, HtmlParseError> {
        Ok(Some(RecursiveResult::Blocks(vec![b])))
    }
    fn flatten(v: Vec<Vec<TextAtomic>>) -> Vec<TextAtomic> {
        v.into_iter().flatten().collect()
    }
    match element.value().name() {
        "h1" => ok_some_block(BlockContainer::Heading1(flatten(atomics))),
        "h2" => ok_some_block(BlockContainer::Heading2(flatten(atomics))),
        "h3" => ok_some_block(BlockContainer::Heading3(flatten(atomics))),
        "h4" | "h5" | "h6" => ok_some_block(BlockContainer::Heading4(flatten(atomics))),
        "code" => {
            if is_inline_code(element) {
                Ok(Some(RecursiveResult::Atomics(vec![
                    TextAtomicBuilder::new(&atomics_to_string(flatten(atomics)))
                        .code(true)
                        .build(),
                ])))
            } else {
                ok_some_block(BlockContainer::Code {
                    code: atomics_to_string(flatten(atomics)),
                    language: get_language_of_code(element),
                })
            }
        }
        "summary" | "button" => Ok(None),
        "p" => ok_some_block(BlockContainer::Paragraph(flatten(atomics))),
        "tr" => Ok(Some(RecursiveResult::TableRows(vec![flatten(atomics)]))),
        _ => Ok(Some(RecursiveResult::Atomics(flatten(atomics)))),
    }
}

fn get_href_walking_up_tree(element: &ElementRef) -> Option<String> {
    let href = element.value().attr("href").map(|x| x.to_string());
    if href.is_some() {
        return href;
    }
    element.parent()?;
    let parent = element.parent().and_then(ElementRef::wrap);
    parent?;
    get_href_walking_up_tree(&parent.unwrap())
}

fn is_hidden(element: &ElementRef) -> bool {
    let out_of_band = element
        .value()
        .has_class("out-of-band", CaseSensitivity::AsciiCaseInsensitive);
    vec![out_of_band].iter().any(|x| *x)
}

fn parse_to_content_recursively(
    element: &ElementRef,
) -> Result<Option<RecursiveResult>, HtmlParseError> {
    if is_hidden(element) {
        return Ok(None);
    }
    let mut children_options = Vec::new();
    for child in element.children() {
        match child.value() {
            Node::Text(t) => children_options.push(Some(RecursiveResult::Atomics(vec![
                // TODO: Get style by walking up the tree recursively
                {
                    let href = get_href_walking_up_tree(element);
                    TextAtomic::simple(&t.to_string()).with_url(href)
                },
            ]))),
            Node::Element(_) => {
                children_options.push(parse_to_content_recursively(
                    &ElementRef::wrap(child).unwrap(),
                )?);
            }
            _ => (),
        }
    }

    fn ok_some_block(b: BlockContainer) -> Result<Option<RecursiveResult>, HtmlParseError> {
        Ok(Some(RecursiveResult::Blocks(vec![b])))
    }

    let children: Vec<RecursiveResult> = children_options.into_iter().flatten().collect();

    let situation = children_to_recursive_children_situation(children)?;
    match situation {
        RecursiveChildrenSituation::NoChildren => Ok(None),
        RecursiveChildrenSituation::AllBlocks(blocks) => Ok(Some(RecursiveResult::Blocks(blocks))),
        RecursiveChildrenSituation::AllTableRows(table) => {
            if element.value().name() != "table" {
                Err(HtmlParseError::invalid_html(format!(
                    "Table rows in <{}>. Expected <table>.",
                    element.value().name()
                )))
            } else {
                ok_some_block(BlockContainer::Table(table))
            }
        }
        RecursiveChildrenSituation::AllAtomics(atomics) => {
            Ok(element_with_atomics_to_recursive_result(element, atomics)?)
        }
    }
}

pub fn parse_to_content(element: &ElementRef) -> Result<Content, HtmlParseError> {
    match parse_to_content_recursively(element)? {
        Some(RecursiveResult::Atomics(atomics)) => {
            Ok(Content(vec![BlockContainer::Paragraph(atomics)]))
        }
        Some(RecursiveResult::Blocks(b)) => Ok(Content(b)),
        Some(RecursiveResult::TableRows(_)) => todo!(),
        None => Err(HtmlParseError::InvalidHtml(
            "HTML Element contained no content.".to_string(),
        )),
    }
}

pub fn get_main_content(element: &ElementRef) -> Result<Content, HtmlParseError> {
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
