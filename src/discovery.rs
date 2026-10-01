use std::path::{Path, PathBuf};

pub fn collect_json_files(folder: &Path) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(folder)
        .expect("failed to read recipe folder")
        .map(|entry| entry.expect("failed to read entry in recipe folder").path());

    entries
        .flat_map(|path| {
            if path.is_dir() {
                collect_json_files(&path)
            } else if path.extension().is_some_and(|ext| ext == "json") {
                vec![path]
            } else {
                vec![]
            }
        })
        .collect()
}
