use std::fs::OpenOptions;
use std::io::{self, Write};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct FileTree {
    pub path: PathBuf,
    pub files: Vec<File>,
    pub directories: Vec<FileTree>,
}

pub fn prep_output_dir(path: PathBuf) -> io::Result<()> {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        if let Ok(p) = path {
            let pa = p.path();
            match (pa.is_dir(), pa.is_file()) {
                (true, false) => fs::remove_dir_all(pa),
                (false, true) => fs::remove_file(pa),
                _ => panic!("Something went really wrong!"),
            };
        };
    }
    Ok(())
}

pub fn create_file_tree(path: PathBuf) -> FileTree {
    let paths = fs::read_dir(path.clone()).unwrap();

    let mut files = Vec::new();
    let mut directories = Vec::new();

    for path in paths {
        println!("path: {:?}", path);
        if let Ok(p) = path {
            let pa = p.path();
            match (pa.is_dir(), pa.is_file()) {
                (true, false) => {
                    let directory = create_file_tree(p.path());
                    directories.push(directory);
                }
                (false, true) => {
                    let file = File { path: pa };
                    files.push(file);
                }
                (_, _) => panic!("Something went really wrong!"),
            }
        }
    }

    return FileTree {
        path,
        directories,
        files,
    };
}

pub fn create_directory_structure(tree: &FileTree, output_dir: PathBuf) -> io::Result<()> {
    println!("Creating directory structure for {:?}", tree.path);
    let mut new_path = output_dir.clone();
    new_path.push(tree.path.clone());
    if !new_path.exists() {
        fs::create_dir(new_path).unwrap();
        for dir in tree.directories.iter() {
            create_directory_structure(&dir, output_dir.clone()).unwrap();
        }
    }

    for file in &tree.files {
        println!("creating: {:?}", file.path.clone());
        let html = crate::markdown::markdown_to_html(crate::markdown::Markdown(
            fs::read_to_string(file.path.clone()).unwrap(),
        ));
        println!("html: {:?}", html);

        let mut f = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(file.path.clone())
            .unwrap();

        f.write(html.0.as_bytes()).unwrap();
    }

    return Ok(());
}
