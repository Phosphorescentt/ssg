use std::fmt::Debug;
mod blocks;

use blocks::{HeaderElement, ParagraphElement};

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

fn parse_markdown(markdown: Markdown) -> MarkdownTree {
    println!("=========================");
    println!("parsing: {}", markdown.0);
    let mut context = ParserContext::new();
    let mut tree = MarkdownTree::new();

    for char in markdown.0.chars() {
        println!("ctx.current_object_type: {:?}", context.current_object_type);
        println!(
            "ctx.current_object_content: {}",
            context.current_object_content
        );
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
                    context.current_object_content.push(char);
                    tree.blocks.push(Box::new(HeaderElement {
                        content: parse_markdown(Markdown(context.current_object_content.clone())),
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
                    context.current_object_content.push(char);
                    tree.blocks.push(Box::new(ParagraphElement {
                        content: parse_markdown(Markdown(context.current_object_content.clone())),
                    }));
                    context.reset();
                } else {
                    context.current_object_content.push(char)
                }
            }
        }
    }

    // Finalise remaining content

    tree
}

fn parse_markdown_tree(markdown: Markdown) -> MarkdownTree {
    let mut context = ParserContext::new();
    let mut tree = MarkdownTree::new();

    let chars_iter = markdown.0.chars();
}

fn parse_header_content(markdown: Markdown) -> HeaderElement {
    // pass
}
