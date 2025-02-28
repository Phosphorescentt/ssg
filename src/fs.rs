use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct File {
    path: PathBuf,
}

#[derive(Debug)]
pub struct Directory {
    path: PathBuf,
    files: Vec<File>,
    directories: Vec<Directory>,
}

pub fn create_file_tree(path: PathBuf) -> Directory {
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

    return Directory {
        path,
        directories,
        files,
    };
}
