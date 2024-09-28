use std::{ffi::OsStr, fs::read_to_string, path::PathBuf};

use walkdir::WalkDir;

pub fn get_example_sources() -> impl Iterator<Item = (PathBuf, String)> {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("examples");

    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| match e.path().extension().and_then(OsStr::to_str) {
            Some(p) => p == "pca",
            None => false,
        })
        .map(|e| (PathBuf::from(e.path()), read_to_string(e.path()).unwrap()))
}
