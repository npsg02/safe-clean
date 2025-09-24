use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use dialoguer::Confirm;
use crate::utils::format_size;

pub async fn cleanup(dry_run: bool) -> Result<()> {
    println!("🗂️  System Temporary Files Cleanup");
    println!("==================================");

    let temp_dirs = get_temp_directories();
    let mut total_size = 0u64;
    let mut total_files = 0usize;

    for temp_dir in &temp_dirs {
        if let Some((size, files)) = analyze_temp_dir(temp_dir).await? {
            println!("\n📁 {}", temp_dir.display());
            println!("   Size: {}", format_size(size));
            println!("   Files: {}", files);
            total_size += size;
            total_files += files;
        }
    }

    if total_size == 0 {
        println!("\n✅ No temporary files found to clean up.");
        return Ok(());
    }

    println!("\n📊 Summary:");
    println!("   Total size: {}", format_size(total_size));
    println!("   Total files: {}", total_files);

    if dry_run {
        println!("\n[DRY RUN] Would clean {} of temporary files", format_size(total_size));
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Clean up {} of temporary files?", format_size(total_size)))
        .interact()?
    {
        for temp_dir in &temp_dirs {
            cleanup_temp_dir(temp_dir).await?;
        }
        println!("\n✅ Temporary files cleanup completed!");
    }

    Ok(())
}

fn get_temp_directories() -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    
    // Common system temp directories
    if let Some(temp) = std::env::var_os("TMPDIR") {
        dirs.push(std::path::PathBuf::from(temp));
    }
    if let Some(temp) = std::env::var_os("TMP") {
        dirs.push(std::path::PathBuf::from(temp));
    }
    if let Some(temp) = std::env::var_os("TEMP") {
        dirs.push(std::path::PathBuf::from(temp));
    }
    
    // Standard locations
    dirs.push("/tmp".into());
    dirs.push("/var/tmp".into());
    
    // User-specific temp directories
    if let Some(home) = std::env::var_os("HOME") {
        let home_path = std::path::PathBuf::from(home);
        dirs.push(home_path.join(".cache"));
        dirs.push(home_path.join("Library/Caches")); // macOS
    }

    // Windows temp directories
    if cfg!(windows) {
        dirs.push("C:\\Windows\\Temp".into());
        if let Some(userprofile) = std::env::var_os("USERPROFILE") {
            let user_path = std::path::PathBuf::from(userprofile);
            dirs.push(user_path.join("AppData\\Local\\Temp"));
        }
    }

    // Filter to only existing directories
    dirs.into_iter()
        .filter(|dir| dir.exists() && dir.is_dir())
        .collect()
}

async fn analyze_temp_dir(path: &Path) -> Result<Option<(u64, usize)>> {
    let path = path.to_owned();
    
    tokio::task::spawn_blocking(move || {
        if !path.exists() || !path.is_dir() {
            return Ok(None);
        }

        let mut total_size = 0u64;
        let mut file_count = 0usize;

        for entry in WalkDir::new(&path)
            .max_depth(2) // Limit depth for performance
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    // Only count files that are likely safe to delete
                    if is_safe_temp_file(entry.path()) {
                        total_size += metadata.len();
                        file_count += 1;
                    }
                }
            }
        }

        if file_count > 0 {
            Ok(Some((total_size, file_count)))
        } else {
            Ok(None)
        }
    }).await?
}

fn is_safe_temp_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            // Common temporary file patterns
            return name_str.starts_with("tmp") ||
                   name_str.starts_with("temp") ||
                   name_str.ends_with(".tmp") ||
                   name_str.ends_with(".temp") ||
                   name_str.ends_with(".cache") ||
                   name_str.starts_with(".#") ||
                   name_str.ends_with("~");
        }
    }
    false
}

async fn cleanup_temp_dir(path: &Path) -> Result<()> {
    let path = path.to_owned();
    
    tokio::task::spawn_blocking(move || {
        let mut cleaned_files = 0;
        let mut cleaned_size = 0u64;

        for entry in WalkDir::new(&path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() && is_safe_temp_file(entry.path()) {
                    match fs::remove_file(entry.path()) {
                        Ok(_) => {
                            cleaned_files += 1;
                            cleaned_size += metadata.len();
                        },
                        Err(e) => {
                            // Don't fail the entire operation for individual file errors
                            eprintln!("   Warning: Failed to remove {}: {}", entry.path().display(), e);
                        }
                    }
                }
            }
        }

        if cleaned_files > 0 {
            println!("   ✅ Cleaned {} files ({})", cleaned_files, format_size(cleaned_size));
        }

        Ok(())
    }).await?
}