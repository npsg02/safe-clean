use anyhow::Result;
use std::path::Path;
use crate::discovery::LargeFileFinder;
use crate::utils::{format_size, parse_size};

pub async fn run(path: Option<String>, size_str: String) -> Result<()> {
    let target_path = path.unwrap_or_else(|| ".".to_string());
    let path = Path::new(&target_path);
    let min_size = parse_size(&size_str)?;

    println!("Searching for files larger than {} in: {}", format_size(min_size), path.display());
    println!();

    let finder = LargeFileFinder::new();
    let results = finder.find_large_files(path, min_size).await?;

    if results.is_empty() {
        println!("No files found larger than {}", format_size(min_size));
        return Ok(());
    }

    println!("{:<60} {:>15}", "Path", "Size");
    println!("{:-<75}", "");

    for item in &results {
        println!(
            "{:<60} {:>15}",
            if item.path.to_string_lossy().len() > 57 {
                format!("...{}", &item.path.to_string_lossy()[item.path.to_string_lossy().len()-54..])
            } else {
                item.path.to_string_lossy().to_string()
            },
            format_size(item.size)
        );
    }

    println!("\nFound {} large files", results.len());

    Ok(())
}