use std::{fmt::Debug, iter::Peekable, str::Chars};
mod blocks;

use blocks::{HeaderElement, ParagraphElement, TextElement};

use crate::markdown::blocks::MarkdownTree;

pub struct Markdown(pub String);
#[derive(Debug)]
pub struct Html(pub String);

trait ToHtml {
    fn to_html(&self) -> Html;
}

trait ToHtmlFmtDebug: ToHtml + Debug {}

use std::{
    io::{self},
    path::PathBuf,
};

pub fn render_file_tree(file_tree: crate::fs::FileTree, output_dir: &PathBuf) -> io::Result<()> {
    // NOTE: This fn should probably be in fs.rs
    crate::fs::create_directory_structure(&file_tree, output_dir).unwrap();

    Ok(())
}

pub fn markdown_to_html(markdown: Markdown) -> Html {
    let parsed = parse_markdown_tree(markdown);
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
    current_object_type: ObjectType,
    current_object_content: String,
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

fn parse_markdown_tree(markdown: Markdown) -> MarkdownTree {
    let mut tree = MarkdownTree::new();

    let mut chars_iter = markdown.0.chars().peekable();
    loop {
        if let Some(char) = chars_iter.peek() {
            println!("current char: {}", char);
            if char == &'#' {
                tree.blocks.push(parse_header(&mut chars_iter));
            } else if char.is_alphanumeric() {
                tree.blocks.push(parse_paragraph(&mut chars_iter));
            }
            chars_iter.next();
        } else {
            break;
        }
    }

    return tree;
}

fn parse_header(chars_iter: &mut Peekable<Chars>) -> Box<HeaderElement> {
    let level = chars_iter.take_while(|x| x == &'#').count();
    let content = chars_iter
        .take_while(|x| !x.is_control())
        .collect::<String>();

    return Box::new(HeaderElement { content, level });
}

fn parse_paragraph(chars_iter: &mut Peekable<Chars>) -> Box<ParagraphElement> {
    let content = chars_iter
        .take_while(|x| !x.is_control())
        .collect::<String>();

    return Box::new(ParagraphElement { content });
}
