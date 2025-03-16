use std::{
    io::{self},
    path::PathBuf,
};

pub struct Markdown(pub String);
#[derive(Debug)]
pub struct Html(pub String);

pub fn render_file_tree(file_tree: crate::fs::FileTree, output_dir: &PathBuf) -> io::Result<()> {
    // NOTE: This fn should probably be in fs.rs
    crate::fs::create_directory_structure(&file_tree, output_dir).unwrap();

    Ok(())
}

pub fn markdown_to_html(markdown: Markdown) -> Html {
    return Html("html!".to_string());
}
