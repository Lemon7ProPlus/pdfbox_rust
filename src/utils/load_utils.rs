use lopdf::Document;
use std::{
    collections::HashSet,
    path::PathBuf,
};
use walkdir::WalkDir;

pub mod prelude {
    pub use super::load_pdfs;
}

pub fn load_pdfs(
    input_paths: Vec<PathBuf>,
    min_depth: usize,
    max_depth: usize,
    exclude_files: Vec<PathBuf>,
) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    // exclude files list
    let mut excludes: HashSet<String> =
        HashSet::from_iter(["output.pdf".to_string(), "temp.pdf".to_string()]);

    for element in exclude_files {
        if let Some(name) = element.file_name() {
            excludes.insert(name.to_string_lossy().to_ascii_lowercase());
        }
    }

    // Process empty input args
    let paths = if input_paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        input_paths
    };

    // Traverse pdf files
    let mut documents: Vec<Document> = Vec::new();

    for path in paths {
        for entry in WalkDir::new(&path)
            .min_depth(min_depth)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path().to_path_buf();

            // check if file
            if !entry.file_type().is_file() {
                continue;
            }

            match path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("pdf"))
            {
                Some(true) => {}
                _ => continue,
            }

            let fname = path
                .file_name()
                .and_then(|f| f.to_str())
                .map(|s| s.to_ascii_lowercase())
                .unwrap_or_default();

            if excludes.contains(&fname) {
                continue;
            }

            // load pdf
            match Document::load(&path) {
                Ok(doc) => documents.push(doc),
                Err(e) => eprintln!("❌ Failed load PDF: {path:?} - {e}"),
            }
        }
    }

    Ok(documents)
}
