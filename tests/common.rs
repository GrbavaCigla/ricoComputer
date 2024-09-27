use std::{ffi::OsStr, fs::read_to_string, path::PathBuf};

use walkdir::WalkDir;

pub fn get_example_sources() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("examples");

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| match e.path().extension().and_then(OsStr::to_str) {
            Some(p) => p == "pca",
            None => false,
        }).map(|e| read_to_string(e.path()))
    {

    }
}
