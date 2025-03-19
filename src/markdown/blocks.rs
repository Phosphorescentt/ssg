use crate::markdown::{Html, ToHtml, ToHtmlFmtDebug};
use std::fmt::Debug;

#[derive(Debug)]
pub struct MarkdownTree {
    pub blocks: Vec<Box<dyn ToHtmlFmtDebug>>,
}

#[derive(Debug)]
pub struct HeaderElement {
    pub content: MarkdownTree,
    pub header_level: u8,
}

#[derive(Debug)]
pub struct ParagraphElement {
    pub content: MarkdownTree,
}

impl MarkdownTree {
    pub fn new() -> MarkdownTree {
        return MarkdownTree { blocks: Vec::new() };
    }
}
impl ToHtml for MarkdownTree {
    fn to_html(&self) -> Html {
        let mut content = String::new();
        for block in &self.blocks {
            let html = block.to_html();
            content.push_str(html.0.as_str());
        }
        return Html(content);
    }
}
impl ToHtmlFmtDebug for MarkdownTree {}

impl ToHtml for ParagraphElement {
    fn to_html(&self) -> Html {
        return self.content.to_html();
    }
}
impl ToHtmlFmtDebug for ParagraphElement {}

impl ToHtml for HeaderElement {
    fn to_html(&self) -> Html {
        let tag = match self.header_level {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            4 => "h4",
            5 => "h5",
            6 => "h6",
            _ => "h6",
        };
        let content = self.content.to_html();
        let elements = vec!["<", tag.clone(), ">", content.0.as_str(), "</", tag, ">"];
        Html(elements.join(""))
    }
}
impl ToHtmlFmtDebug for HeaderElement {}
