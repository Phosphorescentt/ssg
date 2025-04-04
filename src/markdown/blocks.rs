use crate::markdown::{Html, ToHtml, ToHtmlFmtDebug};
use std::fmt::Debug;

#[derive(Debug)]
pub struct TextElement {
    pub content: String,
}

#[derive(Debug)]
pub struct MarkdownTree {
    pub blocks: Vec<Box<dyn ToHtmlFmtDebug>>,
}

#[derive(Debug)]
pub struct HeaderElement {
    pub content: String,
    pub level: usize,
}

#[derive(Debug)]
pub struct ParagraphElement {
    pub content: String,
}

impl ToHtml for TextElement {
    fn to_html(&self) -> Html {
        return Html(self.content.clone());
    }
}
impl ToHtmlFmtDebug for TextElement {}

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
        return Html(self.content.clone());
        // return self.content.to_html();
    }
}
impl ToHtmlFmtDebug for ParagraphElement {}

impl ToHtml for HeaderElement {
    fn to_html(&self) -> Html {
        let tag = match self.level {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            4 => "h4",
            5 => "h5",
            6 => "h6",
            _ => "h6",
        };
        let content = self.content.clone();
        let elements = vec!["<", tag.clone(), ">", content.as_str(), "</", tag, ">"];
        Html(elements.join(""))
    }
}
impl ToHtmlFmtDebug for HeaderElement {}
