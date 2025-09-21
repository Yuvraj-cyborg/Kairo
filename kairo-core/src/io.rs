//Some input ouput shit here

//Walk dir needed for recurssivee reading markdowns and then catching them in a vector maybe,
// So basically vector of Paths of each markdown , ex: vec![PathBuf::from("path/to/file.md"), PathBuf::from("path/to/another.md")]

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

pub fn read_markdowns(dir: String) -> Vec<PathBuf> {
    let mut markdowns = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension() == Some("md".as_ref()) {
            println!("Found markdown file: {:?}", &entry);
            markdowns.push(entry.into_path());
        }
    }
    markdowns
}

pub fn read_file(path: &Path) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn write_file(path: &Path, content: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

