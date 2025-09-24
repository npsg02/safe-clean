use anyhow::Result;
use std::path::Path;
use crate::discovery::DirAnalyzer;
use crate::utils::format_size;

pub async fn run(path: Option<String>, top: usize) -> Result<()> {
    let target_path = path.unwrap_or_else(|| ".".to_string());
    let path = Path::new(&target_path);

    println!("Analyzing directory: {}", path.display());
    println!("Finding top {} largest items...\n", top);

    let analyzer = DirAnalyzer::new();
    let results = analyzer.analyze_directory(path, true).await?;

    println!("{:<50} {:>15} {:>10}", "Path", "Size", "Items");
    println!("{:-<75}", "");

    for item in results.iter().take(top) {
        println!(
            "{:<50} {:>15} {:>10}",
            if item.path.to_string_lossy().len() > 47 {
                format!("...{}", &item.path.to_string_lossy()[item.path.to_string_lossy().len()-44..])
            } else {
                item.path.to_string_lossy().to_string()
            },
            format_size(item.size),
            item.item_count.unwrap_or(0)
        );
    }

    Ok(())
}