use std::{
    fmt::Debug,
    io::{self},
    path::PathBuf,
};

pub struct Markdown(pub String);
#[derive(Debug)]
pub struct Html(pub String);

trait ToHtml {
    fn to_html(&self) -> Html;
}

trait ToHtmlFmtDebug: ToHtml + Debug {}

#[derive(Debug)]
struct MarkdownTree {
    blocks: Vec<Box<dyn ToHtmlFmtDebug>>,
}

impl MarkdownTree {
    fn new() -> MarkdownTree {
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

#[derive(Debug)]
struct ParagraphElement {
    content: String,
}

impl ToHtml for ParagraphElement {
    fn to_html(&self) -> Html {
        return Html(self.content.clone());
    }
}
impl ToHtmlFmtDebug for ParagraphElement {}

#[derive(Debug)]
struct HeaderElement {
    content: String,
    header_level: u8,
}

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
        let elements = vec!["<", tag.clone(), ">", self.content.as_str(), "</", tag, ">"];
        Html(elements.join(""))
    }
}
impl ToHtmlFmtDebug for HeaderElement {}

pub fn render_file_tree(file_tree: crate::fs::FileTree, output_dir: &PathBuf) -> io::Result<()> {
    // NOTE: This fn should probably be in fs.rs
    crate::fs::create_directory_structure(&file_tree, output_dir).unwrap();

    Ok(())
}

pub fn markdown_to_html(markdown: Markdown) -> Html {
    let parsed = parse_markdown(markdown);
    println!("parsed: {:#?}", parsed);
    return parsed.to_html();
}

#[derive(Debug)]
enum ObjectType {
    None,
    Header,
    Paragraph,
}

#[derive(Debug)]
struct ParserContext {
    current_object_content: String,
    current_object_type: ObjectType,
    header_level: u8,
}

impl ParserContext {
    fn new() -> ParserContext {
        ParserContext {
            current_object_type: ObjectType::None,
            current_object_content: String::new(),
            header_level: 0,
        }
    }

    fn reset(&mut self) -> () {
        self.current_object_type = ObjectType::None;
        self.current_object_content = String::new();
        self.header_level = 0;
    }
}

fn parse_markdown(markdown: Markdown) -> impl ToHtmlFmtDebug {
    // Get all the info for the parser
    let buf = markdown.0.as_str();
    let mut context = ParserContext::new();

    // Set up the tree.
    let mut tree = MarkdownTree::new();
    for char in markdown.0.chars() {
        match context.current_object_type {
            ObjectType::None => {
                if char == '#' {
                    context.current_object_type = ObjectType::Header;
                    context.header_level += 1;
                } else if char.is_alphanumeric() {
                    context.current_object_content.push(char);
                    context.current_object_type = ObjectType::Paragraph;
                }
            }
            ObjectType::Header => {
                if char.is_control() {
                    tree.blocks.push(Box::new(HeaderElement {
                        content: context.current_object_content.clone(),
                        header_level: context.header_level,
                    }));
                    context.reset();
                } else if char == '#' {
                    context.header_level += 1;
                } else if char.is_alphanumeric() || char.is_ascii_whitespace() {
                    context.current_object_content.push(char);
                }
            }
            ObjectType::Paragraph => {
                if char.is_control() {
                    tree.blocks.push(Box::new(ParagraphElement {
                        content: context.current_object_content.clone(),
                    }));
                    context.reset();
                } else {
                    context.current_object_content.push(char)
                }
            }
        }
    }

    tree
}
