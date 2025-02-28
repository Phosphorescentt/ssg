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

impl FileTree {}

pub fn create_file_tree(path: PathBuf) -> FileTree {
    let paths = fs::read_dir(path.clone()).unwrap();

    let mut files = Vec::new();
    let mut directories = Vec::new();

    for path in paths {
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
            if p.path().is_dir() {
                let directory = create_file_tree(p.path());
                directories.push(directory)
            }
        }
    }

    return FileTree {
        path,
        directories,
        files,
    };
}
