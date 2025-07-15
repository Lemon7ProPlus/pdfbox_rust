use std::path::PathBuf;
use pdfbox_rust::utils::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match parse_args() {
        CommandResult::Merge {
            input,
            exclude_files,
            toc_file,
            name,
            output
        } => {
            println!("------🛠️ merge PDFs------");
            
            let mut docs = load_pdfs(input, 1, 1, exclude_files)?;

            let titles:Vec<String> = if let Some(path) = toc_file {
                let titles = read_titles_from_file(path)?;
                titles
            } else {
                let titles = read_metadata(&mut docs, "Title")?;
                titles
            };

            let (mut doc, pagenums) = merge_pdfs(&mut docs)?;

            add_toc(&mut doc, titles, pagenums)?;

            modify_metada(&mut doc, "Title", name.unwrap_or("Merged PDF".to_string()))?;

            let output_path = match output {
                Some(p) => normalize_output_path(&p, "output", "pdf", true)?,
                None => PathBuf::from("output.pdf"),
            };

            doc.save(output_path)?;

        }

        CommandResult::ExtractTitle {
            input,
            toc_output
        } => {
            println!("------📖 extract-title------");

            let mut docs = load_pdfs(input, 1, 1, Vec::new())?;
            let titles = read_metadata(&mut docs, "Title")?;

            let toc_out_path = match toc_output {
                Some(p) => normalize_output_path(&p, "titles", "txt", true)?,
                None => PathBuf::from("titles.txt"),
            };

            write_titles_to_file(toc_out_path, &titles)?;

        }
    }
    Ok(())
}
