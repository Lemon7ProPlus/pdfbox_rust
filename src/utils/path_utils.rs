use std::{fs::{self, File}, io::{BufRead, BufReader, Write}, path::{Path, PathBuf}};

pub mod prelude {
    pub use super::read_titles_from_file;
    pub use super::write_titles_to_file;
    pub use super::normalize_output_path;
}

pub fn read_titles_from_file(path: PathBuf) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn write_titles_to_file(path: PathBuf, titles: &Vec<String>) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    for title in titles {
        writeln!(file, "{}", title)?;
    }
    Ok(())
}

pub fn normalize_output_path (
    output: &Path,
    default_name: &str,
    ext: &str,
    create_parent_dir: bool,
) -> Result<PathBuf, std::io::Error> {
    let final_path = if output.is_dir() || output.to_string_lossy().ends_with(std::path::MAIN_SEPARATOR) {
        output.join(format!("{}.{}", default_name, ext))
    } else {
        let mut path = output.to_path_buf();
        match path.extension().and_then(|s| s.to_str()) {
            None => {
                path.set_extension(ext);
            }
            Some(existing_ext) if existing_ext.to_ascii_lowercase() != ext.to_ascii_lowercase() => {
                path.set_extension(ext);
            }
            _ => {}
        }
        path
    };

    if create_parent_dir {
        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(final_path)
}