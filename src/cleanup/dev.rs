use anyhow::Result;
use std::path::Path;
use std::fs;
use dialoguer::Confirm;
use crate::discovery::{DevArtifactFinder, FileItem};
use crate::utils::format_size;

pub async fn cleanup(path: Option<String>, dry_run: bool) -> Result<()> {
    let target_path = path.unwrap_or_else(|| ".".to_string());
    let path = Path::new(&target_path);

    println!("🛠️  Development Artifacts Cleanup");
    println!("=================================");
    println!("Searching in: {}", path.display());

    let finder = DevArtifactFinder::new();
    let artifacts = finder.find_artifacts(path).await?;

    if artifacts.is_empty() {
        println!("\n✅ No development artifacts found.");
        return Ok(());
    }

    let total_size: u64 = artifacts.iter().map(|a| a.size).sum();
    let total_items: usize = artifacts.iter().map(|a| a.item_count.unwrap_or(0)).sum();

    println!("\n📊 Found development artifacts:");
    println!("{:<60} {:>15} {:>10}", "Path", "Size", "Items");
    println!("{:-<85}", "");

    for artifact in &artifacts {
        println!(
            "{:<60} {:>15} {:>10}",
            if artifact.path.to_string_lossy().len() > 57 {
                format!("...{}", &artifact.path.to_string_lossy()[artifact.path.to_string_lossy().len()-54..])
            } else {
                artifact.path.to_string_lossy().to_string()
            },
            format_size(artifact.size),
            artifact.item_count.unwrap_or(0)
        );
    }

    println!("\n📈 Summary:");
    println!("   Total artifacts: {}", artifacts.len());
    println!("   Total size: {}", format_size(total_size));
    println!("   Total items: {}", total_items);

    if dry_run {
        println!("\n[DRY RUN] Would remove {} development artifacts ({})", 
                 artifacts.len(), format_size(total_size));
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Remove {} development artifacts ({})?", 
                              artifacts.len(), format_size(total_size)))
        .interact()?
    {
        remove_artifacts(artifacts).await?;
        println!("\n✅ Development artifacts cleanup completed!");
    }

    Ok(())
}

async fn remove_artifacts(artifacts: Vec<FileItem>) -> Result<()> {
    let artifacts_clone = artifacts.clone();
    
    tokio::task::spawn_blocking(move || {
        let mut removed_count = 0;
        let mut removed_size = 0u64;

        for artifact in artifacts_clone {
            match remove_dir_all_safe(&artifact.path) {
                Ok(_) => {
                    removed_count += 1;
                    removed_size += artifact.size;
                    println!("   ✅ Removed: {}", artifact.path.display());
                },
                Err(e) => {
                    eprintln!("   ❌ Failed to remove {}: {}", artifact.path.display(), e);
                }
            }
        }

        if removed_count > 0 {
            println!("\n📊 Cleanup Summary:");
            println!("   Removed {} artifacts", removed_count);
            println!("   Freed up {}", format_size(removed_size));
        }

        Ok(())
    }).await?
}

fn remove_dir_all_safe(path: &Path) -> Result<()> {
    // Additional safety checks before removal
    if !path.exists() {
        return Ok(());
    }

    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", path.display()));
    }

    // Check if it's actually a development artifact directory
    let dir_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let safe_dirs = [
        "node_modules", ".venv", "venv", "__pycache__", 
        ".tox", "target", "build", "dist"
    ];

    if !safe_dirs.contains(&dir_name) {
        return Err(anyhow::anyhow!("Directory name '{}' is not in the safe removal list", dir_name));
    }

    // Additional check: ensure we're not at filesystem root
    if path.parent().is_none() {
        return Err(anyhow::anyhow!("Refusing to remove directory at filesystem root"));
    }

    fs::remove_dir_all(path)?;
    Ok(())
}