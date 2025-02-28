use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use crate::fs::FileTree;

struct Markdown(String);
struct Html(String);

pub fn render_file_tree(file_tree: FileTree, output_dir: PathBuf) -> io::Result<()> {
    // NOTE: This fn should probably be in fs.rs
    for file in file_tree.files {
        let mut rendered_dir = output_dir.clone();
        rendered_dir.push(file.path.clone());
        file_to_html(file.path, rendered_dir).unwrap();
    }
    Ok(())
}

fn file_to_html(input_path: PathBuf, output_path: PathBuf) -> io::Result<()> {
    // NOTE: This fn should probably be in fs.rs
    let html = markdown_to_html(Markdown(fs::read_to_string(input_path).unwrap()));
    dbg!(output_path.clone());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .create_new(true)
        .create(true)
        .open(output_path)
        .unwrap();

    file.write(html.0.as_bytes()).unwrap();
    Ok(())
}

fn create_output_file_from_path(path: PathBuf) -> io::Result<()> {
    // TODO: Create all required directories so that we can write to the html file.
    Ok(())
}

fn markdown_to_html(markdown: Markdown) -> Html {
    return Html("asdf".to_string());
}
