use scraper::{ElementRef, Node, Selector};

use crate::{BlockContainer, Content, Language, TextAtomic, TextAtomicBuilder};

use super::error::HtmlParseError;

/// Find and parse the main content
/// of the documentation page,
/// from title to bottom.

struct AutoMerger {
    current: Option<BlockContainer>,
    result: Vec<BlockContainer>,
}

// RecursiveResult might contain
// unfinished recursive results, which have
// to be combined to a greater structure.
enum RecursiveResult {
    TableRow(Vec<Vec<TextAtomic>>),
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
    pub fn is_blocks(&self) -> bool {
        match self {
            Self::Blocks(_) => true,
            _ => false,
        }
    }
    pub fn atomics(self) -> Option<Vec<TextAtomic>> {
        match self {
            Self::Atomics(p) => Some(p),
            _ => None,
        }
    }
    pub fn blocks(self) -> Option<Vec<BlockContainer>> {
        match self {
            Self::Blocks(b) => Some(b),
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
            RecursiveResult::TableRow(_) => {
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

fn get_language_of_code(e: &ElementRef) -> Option<Language> {
    None
}

fn element_with_atomic_children_to_container(
    element: &ElementRef,
    children: Vec<TextAtomic>,
) -> Result<BlockContainer, HtmlParseError> {
    match element.value().name() {
        "h1" => Ok(BlockContainer::Heading1(children)),
        "h2" => Ok(BlockContainer::Heading2(children)),
        "h3" => Ok(BlockContainer::Heading3(children)),
        "h4" | "h5" | "h6" => Ok(BlockContainer::Heading4(children)),
        "code" => Ok(BlockContainer::Code {
            code: children,
            language: get_language_of_code(element),
        }),
        "p" | _ => Ok(BlockContainer::Paragraph(children)),
    }
}

fn element_with_block_children_to_container(
    element: &ElementRef,
    children: Vec<BlockContainer>,
) -> Result<Vec<BlockContainer>, HtmlParseError> {
    match element.value().name() {
        "code" => todo!("TODO: code container from block children?"),
        "quote" => Ok(vec![BlockContainer::Quote(children)]),
        "ul" => Ok(vec![BlockContainer::BulletPoints {
            points: children,
            enumerated: false,
        }]),
        "ol" => Ok(vec![BlockContainer::BulletPoints {
            points: children,
            enumerated: true,
        }]),
        _ => Ok(children),
    }
}
fn parse_to_content_recursively(
    element: &ElementRef,
) -> Result<Option<RecursiveResult>, HtmlParseError> {
    let mut children_options = Vec::new();
    for child in element.children() {
        match child.value() {
            Node::Text(t) => children_options.push(Some(RecursiveResult::Atomics(vec![
                // TODO: Get style by walking up the tree recursively
                TextAtomicBuilder::new(&t.to_string()).build(),
            ]))),
            Node::Element(_) => {
                children_options.push(parse_to_content_recursively(
                    &ElementRef::wrap(child).unwrap(),
                )?);
            }
            _ => (),
        }
    }
    let children_result: Vec<RecursiveResult> =
        children_options.into_iter().filter_map(|x| x).collect();

    // If one of the children already returned a full and complete block,
    // they can no longer be part of a homogenic bigger structure.
    // Make neighboring atomics to blocks as well.
    let at_least_one_block = children_result.iter().any(|x| x.is_blocks());
    if at_least_one_block {
        return Ok(Some(RecursiveResult::Blocks(
            bundle_resursive_results_to_block_vec(children_result)?,
        )));
    }

    let all_children_are_partials = !children_result.iter().any(|x| !x.is_atomics());
    debug_assert!(all_children_are_partials);

    let atomics: Vec<TextAtomic> = children_result
        .into_iter()
        .filter_map(|x| x.atomics())
        .flatten()
        .collect();

    fn ok_some_block(b: BlockContainer) -> Result<Option<RecursiveResult>, HtmlParseError> {
        Ok(Some(RecursiveResult::Blocks(vec![b])))
    }

    match element.value().name() {
        "h1" => ok_some_block(BlockContainer::Heading1(atomics)),
        "h2" => ok_some_block(BlockContainer::Heading2(atomics)),
        "h3" => ok_some_block(BlockContainer::Heading3(atomics)),
        "h4" | "h5" | "h6" => ok_some_block(BlockContainer::Heading4(atomics)),
        "code" => ok_some_block(BlockContainer::Code {
            code: atomics,
            language: get_language_of_code(element),
        }),
        "summary" | "button" => Ok(None),
        "p" | _ => ok_some_block(BlockContainer::Paragraph(atomics)),
    }
}

fn parse_to_content(element: &ElementRef) -> Result<Content, HtmlParseError> {
    match parse_to_content_recursively(element)? {
        Some(RecursiveResult::Atomics(atomics)) => {
            Ok(Content(vec![BlockContainer::Paragraph(atomics)]))
        }
        Some(RecursiveResult::Blocks(b)) => Ok(Content(b)),
        Some(RecursiveResult::TableRow(_)) => todo!(),
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
