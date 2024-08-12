mod cli;
mod inspect;

use clap::Parser;
use cli::Args;
use inspect::Inspector;
use std::fs;
use walkdir::WalkDir;

/// Recursively walk through provided path and return rust files (files with '.rs' suffix).
fn find_rust_files(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .map(|e| e.path().display().to_string())
        .collect()
}

fn main() {
    let args = Args::parse();
    let rust_files = find_rust_files(&args.dir);

    for file in rust_files {
        if let Ok(content) = fs::read_to_string(&file) {
            if let Ok(syntax) = syn::parse_file(&content) {
                let inspector = Inspector::new(syntax, file.clone());
                inspector.inspect_file();
            }
        }
    }
}
